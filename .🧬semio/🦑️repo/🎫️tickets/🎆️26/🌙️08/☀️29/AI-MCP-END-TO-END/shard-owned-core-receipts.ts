import assert from "node:assert/strict";
import { Database } from "bun:sqlite";
import fixture from "./shard-owned-core-receipts.json";

const db = new Database(":memory:");
db.exec("CREATE TABLE debit (currency INTEGER, amount INTEGER CHECK (amount >= 0)) STRICT");
const record = db.prepare("INSERT INTO debit VALUES (?, ?)");
const sum = db.prepare("SELECT COALESCE(SUM(amount), 0) AS total FROM debit WHERE currency = ?");
for (const scenario of fixture.cases) {
  db.exec("DELETE FROM debit");
  const remaining = [...fixture.stock];
  let last = 0;
  let input: number | undefined;
  let guest: { serial: number; phase: "held" | "frozen" | "received" } | undefined;
  let cleanup = false;
  let complete = false;
  let refusalAt: number | undefined;
  function debit(values: unknown, account: unknown) {
    assert(Array.isArray(values) && values.length === remaining.length);
    assert(account === "host" || account === "guest" || account === "vm");
    for (const [currency, amount] of values.entries()) {
      assert(Number.isSafeInteger(amount) && amount >= 0 && amount <= remaining[currency]);
      if (account === "vm") assert(currency === 5 || amount === 0);
      else assert(currency !== 5 || amount === 0);
    }
    for (const [currency, amount] of values.entries()) {
      remaining[currency] -= amount;
      record.run(currency, amount);
    }
  }
  for (const [index, [action, first, second, third]] of scenario.events.entries()) {
    try {
      if (action === "issue") {
        assert(typeof first === "number" && input === undefined && !complete && (last === 0 || first === last + 1));
        input = first;
        continue;
      }
      assert(input !== undefined);
      switch (action) {
        case "return":
          assert(first === input);
          last = input;
          input = undefined;
          break;
        case "deny":
          assert(first === fixture.source && second === fixture.wire && !guest && !cleanup && !complete);
          break;
        case "checkout":
          assert(first === fixture.source && second === fixture.wire && typeof third === "number" && !guest && !cleanup && !complete);
          guest = { serial: third, phase: "held" };
          break;
        case "spend":
          debit(second, first);
          break;
        case "capture":
          assert(guest?.phase === "held" && guest.serial === first);
          guest.phase = "frozen";
          break;
        case "receive":
          assert(guest?.phase === "frozen" && guest.serial === first);
          debit(second, "guest");
          guest.phase = "received";
          break;
        case "acknowledge":
          assert(guest?.phase === "received" && guest.serial === first);
          guest = undefined;
          break;
        case "cancel":
        case "fail":
          assert(!complete);
          cleanup = true;
          break;
        case "finish":
          assert(!guest && !complete && first === (cleanup ? "cleanup" : "output"));
          complete = true;
          break;
        default:
          assert.fail(`Unknown receipt action ${action}`);
      }
    } catch (error) {
      if (!(error instanceof assert.AssertionError)) throw error;
      refusalAt = index;
      break;
    }
  }
  const sqliteRemaining = fixture.stock.map((stock, currency) => stock - Number((sum.get(currency) as { total: number }).total));
  assert.deepEqual(remaining, scenario.remaining, scenario.name);
  assert.deepEqual(sqliteRemaining, scenario.remaining, scenario.name);
  assert.equal(refusalAt, scenario.refusalAt, scenario.name);
  if (refusalAt === undefined) assert(complete && input === undefined && guest === undefined, scenario.name);
}
console.log("[DEBUG] Validated shard Source receipt model and SQLite conservation", fixture.cases.length);
