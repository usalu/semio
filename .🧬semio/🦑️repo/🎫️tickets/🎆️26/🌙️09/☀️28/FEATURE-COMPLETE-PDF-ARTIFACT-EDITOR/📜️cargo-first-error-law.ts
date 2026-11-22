/** 🧾 Verifies and stages retention of Cargo's first hidden compiler error. */
import assert from "node:assert/strict";
import { writeSync } from "node:fs";
import { join } from "node:path";
import { Database } from "bun:sqlite";
import Ajv from "ajv";

const ticket = import.meta.dir;
const mode = process.argv[2];
assert(mode === "red" || mode === "green" || mode === "install");
const schema = await Bun.file(join(ticket,"🔣️cargo-first-error.schema.json")).json();
const fixture = await Bun.file(join(ticket,"🔣️cargo-first-error.json")).json();
const fixtureValid = new Ajv({strict:true,allErrors:true}).compile(schema);
assert(fixtureValid(fixture), JSON.stringify(fixtureValid.errors));

const source = "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts";
const before = await Bun.file(source).text();
const replace = (text: string, expected: string, update: string) => {
  assert.equal(text.split(expected).length, 2, "Compiler diagnostic source changed before staging");
  return text.replace(expected, update);
};
const stage = (text: string) => {
  const keep = "  private readonly newest: string[] = [];";
  const summary = replace(text, "replays the newest lines.", "replays the first hidden error and the newest lines.");
  let revised = replace(summary, keep, keep + "\n  private firstErrorLine = 0;\n  private readonly firstError: string[] = [];");
  const record = "    this.suppressed++;\n    this.newest.push(text);";
  revised = replace(revised, record, String.raw`    this.suppressed++;
    if (!this.firstErrorLine && /^error(?:\[[A-Z][A-Z0-9]*\])?:/u.test(text.replace(/\x1b\[[0-9;]*m/gu, ""))) {
      this.firstErrorLine = this.lines;
    }
    if (this.firstErrorLine && this.lines - this.firstErrorLine === this.firstError.length
      && this.firstError.length < this.newestLines && text.trim()) this.firstError.push(text);
    this.newest.push(text);`);
  return replace(revised, "    if (failed) rows.push(...this.newest);", `    if (failed) {
      const beforeNewest = Math.max(0, this.lines - this.newest.length - this.firstErrorLine + 1);
      rows.push(...this.firstError.slice(0, beforeNewest), ...this.newest);
    }`);
};
const live = before.includes("private firstErrorLine = 0;");
const candidate = mode === "red" || live ? before : stage(before);
const start = candidate.indexOf("export class BoundedDiagnosticsV1 {");
const end = candidate.indexOf("\n/** 🗂️", start);
assert(start >= 0 && end > start);
const classSource = candidate.slice(start, end).replace("export class", "class");
const runtime = new Bun.Transpiler({loader:"ts"}).transformSync(classSource);
const Constructor = new Function("writeSync", "nativeBuildDiagnosticLinesV1", runtime + "\nreturn BoundedDiagnosticsV1;")(
  writeSync, (id: string) => [id],
);
const db = new Database(":memory:");
try {
  const oracle = db.query(`WITH lines(n,text) AS (SELECT CAST(key AS INT), value FROM json_each(?1)),
   clean(n,text) AS (SELECT n,replace(replace(text,char(27)||'[1;31m',''),char(27)||'[0m','') FROM lines),
   first_error(n) AS (SELECT MIN(n) FROM clean WHERE n>=?2 AND (text LIKE 'error:%' OR text GLOB 'error[[]*]:*')),
   first_blank(n) AS (SELECT MIN(clean.n) FROM clean,first_error WHERE clean.n>first_error.n AND trim(clean.text)=''),
   last_line(n) AS (SELECT MAX(n) FROM lines)
   SELECT lines.text FROM lines,first_error,first_blank,last_line
   WHERE lines.n>last_line.n-?3 OR (lines.n>=first_error.n AND lines.n<first_error.n+?3 AND lines.n<COALESCE(first_blank.n,last_line.n+1))
   ORDER BY lines.n`);
  for (const test of fixture.cases) {
    assert.deepEqual(oracle.all(JSON.stringify(test.lines),fixture.live,fixture.tail).map((row: any)=>row.text),
      test.failureReplay, test.id + " independently expected replay");
    const shown: string[] = [];
    const diagnostics = new Constructor(null,(line: string)=>shown.push(line),fixture.live,fixture.tail);
    for (const line of test.lines) diagnostics.line(line);
    assert.deepEqual(shown,test.lines.slice(0,fixture.live),test.id + " live line budget");
    assert.deepEqual(diagnostics.finish(false,null),["diagnosticsHidden"],test.id + " completed output");
    assert.deepEqual(diagnostics.finish(true,null),["diagnosticsHidden",...test.failureReplay],test.id + " failure output");
  }
} finally {
  db.close();
}
if (mode === "install" && !live) {
  assert.equal(await Bun.file(source).text(),before,"Compiler source changed before install");
  await Bun.write(source,candidate);
}
console.log("[DEBUG] Three compiler diagnostic replay laws passed against SQLite");
