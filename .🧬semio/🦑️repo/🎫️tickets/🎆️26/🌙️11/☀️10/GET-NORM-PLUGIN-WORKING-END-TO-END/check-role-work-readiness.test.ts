import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import fixture from "./role-work-readiness.json";
import {
  createSessionWorkLedgerV1,
  type SessionWorkKindV1,
} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts";

test("the role control tracks the source instance until both operation kinds settle", () => {
  const ledger = createSessionWorkLedgerV1();
  const changes: boolean[] = [];
  const release: Array<() => void> = [];
  const unsubscribe = ledger.subscribe(() => changes.push(ledger.pending("norm", 1) > 0));
  const database = new Database(":memory:");
  try {
    database.run("CREATE TABLE work (id INTEGER PRIMARY KEY, instance INTEGER NOT NULL)");
    for (const step of fixture.steps) {
      if (step.begin) {
        const [plugin, instance, kind] = step.begin as [string, number, SessionWorkKindV1];
        database.run("INSERT INTO work VALUES (?, ?)", [release.length, instance]);
        release.push(ledger.begin(plugin, instance, kind));
      } else {
        database.run("DELETE FROM work WHERE id = ?", [step.release!]);
        release[step.release!]!();
      }
      const counted = database.query("SELECT COUNT(*) AS count FROM work WHERE instance = 1").get();
      expect(counted).toEqual({ count: step.pending });
      expect(ledger.pending("norm", 1)).toBe(step.pending);
    }
    expect(changes).toEqual(fixture.observedBusy);
    unsubscribe();
    ledger.begin("norm", 1, "typed-operation")();
    expect(changes).toEqual(fixture.observedBusy);
  } finally {
    database.close();
  }
});
