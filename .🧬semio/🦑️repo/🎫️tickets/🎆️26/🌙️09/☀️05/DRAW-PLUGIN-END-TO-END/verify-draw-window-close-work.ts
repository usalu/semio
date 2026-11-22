/** ⚖️ Compares the witnessed Chromium close effect with the native minimum-work contract. */
import assert from "node:assert/strict";
import { Database } from "bun:sqlite";

const [fixture, observation] = Bun.argv.slice(2);
const { minimumWorkWithPhysicalClose: minimum } = await Bun.file(fixture).json();
const { locals, words } = await Bun.file(observation).json();
const reported = [locals.$var9, locals.$var11, locals.$var10, locals.$var12];
const gross = [locals.$var3, locals.$var5, locals.$var21, locals.$var16];
const fresh = [locals.$var20, locals.$var19, locals.$var18, locals.$var17];
assert.deepEqual(reported, [1, 4, 0, 0]);
assert.deepEqual(gross, fresh);
assert.deepEqual(reported, fresh);
assert(fresh.every((amount, i) => amount <= words[i + 1]));
const db = new Database(":memory:");
try {
  const verdict = db.query<{ admitted: number }, [number, number, number, number, number, number]>(
    "SELECT (max(?1, ?2, ?3, ?4) = 0 OR ?5 >= ?6) AS admitted",
  );
  assert.equal(verdict.get(0, 0, 0, 0, 0, minimum)?.admitted, 1);
  assert.equal(verdict.get(...fresh as [number, number, number, number], 0, minimum)?.admitted, 0);
  assert.equal(verdict.get(...fresh as [number, number, number, number], 1, minimum)?.admitted, 1);
} finally {
  db.close();
}
console.log("[DEBUG] Three independent SQLite checks reproduced the exact missing-work refusal and the paid receipt law");
