import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
const source = new URL("./history-forward-supersession.json", import.meta.url);
const corpus = JSON.parse(readFileSync(source, "utf8")) as { cases: { events: {target:string,value:string}[], expected:Record<string,string> }[] };
for (const test of corpus.cases) {
  const db = new Database(":memory:");
  db.run("create table events (position integer primary key, target text not null, value text not null)");
  const insert = db.prepare("insert into events values (?, ?, ?)");
  test.events.forEach((event, index) => insert.run(index, event.target, event.value));
  const rows = db.query("select e.target, e.value from events e where e.position = (select max(x.position) from events x where x.target = e.target) order by e.target").all() as {target:string,value:string}[];
  const actual = Object.fromEntries(rows.map(row => [row.target, row.value]));
  if (JSON.stringify(actual) !== JSON.stringify(Object.fromEntries(Object.entries(test.expected).sort()))) throw new Error("chronological supersession mismatch");
  db.close();
}
console.log(`Chronological supersession: ${corpus.cases.length} SQLite comparisons passed`);
