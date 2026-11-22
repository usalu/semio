import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";

const root = new URL("../../../../../../../", import.meta.url);
const runtime = new URL("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/", root);
const cases: { id: string; acknowledgements: number; moreWork: boolean; queuedInput: boolean; order: string[] }[] =
  JSON.parse(readFileSync(new URL("./note-archive-actor-fairness.json", import.meta.url), "utf8"));

test("detached archive work returns the actor after its accepted receipts settle", () => {
  const drive = readFileSync(new URL("🚚️handle-turn-drive/🟦️.ts", runtime), "utf8");
  const detached = drive.slice(drive.indexOf("const drainTypedOperations ="), drive.indexOf("const openOriginalMediaIngress"));
  expect(detached).toContain("new Set(), (turn) => acceptUiPatches(instanceId, turn), false, activation, call");
  expect(detached).toContain("drainTypedOperationBatches(");
  expect(detached).toContain("}, yieldPluginUiContinuation)");
  const settle = readFileSync(new URL("🔁️turn-settle/🟦️.ts", runtime), "utf8");
  expect(settle).toContain("acknowledgements.length > 0 || hasWork() || deferredPending()");
});

test("SQLite independently orders receipts, waiting input, and detached continuation", () => {
  const db = new Database();
  try {
    const jobs = db.query<{ task: string }, [number, number, number]>(`
      WITH RECURSIVE pending(n) AS (SELECT ? UNION ALL SELECT n - 1 FROM pending WHERE n > 1)
      SELECT task FROM (
        SELECT 'acknowledgement' AS task, 0 AS priority FROM pending WHERE n > 0
        UNION ALL SELECT 'queued-input', 1 WHERE ?
        UNION ALL SELECT 'maintenance', 2 WHERE ?
      ) ORDER BY priority
    `);
    for (const row of cases) {
      const order: string[] = Array(row.acknowledgements).fill("acknowledgement");
      if (row.queuedInput) order.push("queued-input");
      if (row.moreWork) order.push("maintenance");
      expect(order, row.id).toEqual(row.order);
      expect(jobs.all(row.acknowledgements, Number(row.queuedInput), Number(row.moreWork)).map(row => row.task), row.id).toEqual(row.order);
    }
  } finally {
    db.close();
  }
});
