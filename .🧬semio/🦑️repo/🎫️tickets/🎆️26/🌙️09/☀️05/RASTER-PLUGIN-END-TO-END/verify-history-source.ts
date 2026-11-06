import { readFileSync } from "node:fs";
import { Database } from "bun:sqlite";

const fixture = JSON.parse(readFileSync(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/RASTER-PLUGIN-END-TO-END/history-source-cases.json", "utf8")) as { cases: { entries: number[]; reverse: number[] }[] };
const database = new Database(":memory:");
const reverse = database.query("SELECT value FROM json_each(?) ORDER BY CAST(key AS INTEGER) DESC");
for (const row of fixture.cases) {
  const values = reverse.all(JSON.stringify(row.entries)).map((value) => (value as {value:number}).value);
  if (JSON.stringify(values) !== JSON.stringify(row.reverse)) throw new Error("history source fixture disagrees with SQLite reverse traversal");
}
database.close();
console.log(`History source fixture: ${fixture.cases.length} SQLite reverse traversal comparisons passed`);
