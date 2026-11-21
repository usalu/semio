import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { Database } from "bun:sqlite";

const fixture = JSON.parse(readFileSync("🧰️framework/🔨️modules/🧵️job/⏱️context/📦️return/🎟️place/sized-case.json", "utf8"));
const database = new Database(":memory:");
try {
  const query = database.query(`
    WITH source(requested, captured, granted, fresh) AS (VALUES (?, ?, ?, ?)),
         reserve AS (
           SELECT requested, captured, fresh, requested <= granted AS admitted FROM source
         ),
         receipt AS (
           SELECT CASE WHEN admitted AND captured THEN requested ELSE 0 END AS spent,
                  fresh FROM reserve
         )
    SELECT spent, (fresh + spent) - spent AS fresh_copy FROM receipt
  `);
  for (const row of fixture.cases) {
    assert.deepEqual(query.get(row.requested, Number(row.captured), fixture.grantBytes, 0),
      { spent: row.spent, fresh_copy: 0 });
    assert.deepEqual(query.get(row.requested, Number(row.captured), fixture.grantBytes, row.admitted ? 1 : 0),
      { spent: row.spent, fresh_copy: row.admitted ? 1 : 0 });
  }
  console.log(`Verified ${fixture.cases.length} held-return scenarios with SQLite`);
} finally {
  database.close();
}
