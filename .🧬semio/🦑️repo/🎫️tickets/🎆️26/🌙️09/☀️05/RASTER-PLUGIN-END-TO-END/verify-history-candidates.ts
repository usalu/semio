import { readFileSync } from "node:fs";
import { Database } from "bun:sqlite";

const fixture = JSON.parse(readFileSync(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/RASTER-PLUGIN-END-TO-END/history-transition-candidates.json", "utf8")) as { cases: { bytes: number[]; candidate: string }[] };
const database = new Database(":memory:");
const classify = database.query("SELECT CASE WHEN json_array_length(?)=0 OR json_extract(?,'$[0]')>=128 THEN 'decode' WHEN json_extract(?,'$[0]')=0 THEN 'revert' WHEN json_extract(?,'$[0]')=6 THEN 'supersede' ELSE 'other' END AS candidate");
for (const row of fixture.cases) {
  const bytes=JSON.stringify(row.bytes);
  const expected=(classify.get(bytes,bytes,bytes,bytes) as {candidate:string}).candidate;
  if (row.candidate !== expected) throw new Error("transition candidate fixture disagrees with first varint octet");
}
database.close();
console.log(`Transition candidate fixture: ${fixture.cases.length} SQLite comparisons passed`);
