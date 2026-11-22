import { readFileSync, writeFileSync } from "node:fs";

const path = process.argv[2]!;
const lines = readFileSync(path, "utf8").split("\n");
const start = lines.findIndex(line => line.startsWith('test("Rewriting inline manifest owns all seven declared value type families"'));
if (start < 0) { console.log("already replaced"); process.exit(0); }
const end = lines.findIndex((line, index) => index > start && line === "});");
const replacement = [
  'test("Rewriting working graph references its manifest by id and owns no inline manifest",async()=>{',
  ' const vector=await Bun.file(new URL("./🧫️fixtures/🌳️typed/🔣️.json",import.meta.url)).json(),typed=owner.parseRewritingJsonValue(vector.snapshot);',
  ' expect(Object.hasOwn(typed.workingGraph,"manifest")).toBe(false);',
  ' const db=Database.deserialize(await exportSqliteDatabase(await project(typed)));',
  ' try{',
  '  expect(db.query("SELECT manifest_id FROM jack_document").get()).toEqual({manifest_id:typed.workingGraph.manifestId});',
  '  expect(db.query("SELECT name FROM sqlite_schema WHERE type=\'table\' AND name LIKE \'jack_%\' ORDER BY name").all()).toEqual([{name:"jack_camera"},{name:"jack_content_child"},{name:"jack_document"}]);',
  '  expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);',
  '  expect(await restore(await importSqliteDatabase(db.serialize()))).toEqual(typed);',
  ' }finally{db.close();}',
  '});',
];
lines.splice(start, end - start + 1, ...replacement);
writeFileSync(path, lines.join("\n"));
console.log(`replaced ${start + 1}-${end + 1}`);
