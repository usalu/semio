/** 🎪️ Actual Playground persisted field and independent semantic SQL laws. */
import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import * as snapshot from "../../🟦️.ts";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import schema from "../../🔣️.json";
import { parsePlaygroundDiff } from "../../../🔺️diff/🟦️.ts";
import { parsePlaygroundTopology } from "../../../💡️inferences/🟦️.ts";
import { parsePlaygroundSnapshotText } from "../../📝️text/🟦️.ts";
import { parsePlaygroundDiffText } from "../../../🔺️diff/📝️text/🟦️.ts";
import { parsePlaygroundInferenceText } from "../../../💡️inferences/📝️text/🟦️.ts";
import { parsePlaygroundMutationsText } from "../../../🧬️mutations/📝️text/🟦️.ts";
import diffSchema from "../../../🔺️diff/🔣️.json";
import inferenceSchema from "../../../💡️inferences/🔣️.json";
import { exportSqliteDatabase, importSqliteDatabase } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import type { SqliteDatabase } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import type { ArtifactSqliteOptions } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";

const sql = readFileSync(new URL("../../🪶️sqlite/🗄️.sql", import.meta.url), "utf8");
const validate = new Ajv({ strict: false }).compile(schema);
type Capability = {
  playgroundSnapshotToSqliteDatabase(value: snapshot.PlaygroundSnapshot, options?: ArtifactSqliteOptions): Promise<SqliteDatabase>;
  playgroundSnapshotFromSqliteDatabase(value: SqliteDatabase, options?: ArtifactSqliteOptions): Promise<snapshot.PlaygroundSnapshot>;
};
const capability = snapshot as unknown as Capability;

for (const [name, parse] of [["Snapshot", parsePlaygroundSnapshotText], ["Diff", parsePlaygroundDiffText], ["Inference", parsePlaygroundInferenceText], ["Mutation", parsePlaygroundMutationsText]] as const) test("Playground " + name + " text parser owns literal strings", () => {
  const check = new Ajv().compile({ type: "string" });
  for (const value of fixture.textCases) { expect(check(value)).toBe(true); expect(parse(value)).toBe(value); }
  for (const value of fixture.invalidTexts) { expect(check(value)).toBe(false); expect(() => parse(value)).toThrow(); }
});

test("Playground diff binds its actual closed artifact fields", () => {
  const check = new Ajv({ strict: false }).compile(diffSchema);
  for (const value of fixture.validDiffs) { expect(check(value)).toBe(true); expect(parsePlaygroundDiff(value)).toEqual({ artifact: value.artifact, schema: value.schema }); }
  for (const value of fixture.invalidDiffs) { expect(check(value)).toBe(false); expect(() => parsePlaygroundDiff(value)).toThrow(); }
});

test("Playground inference depth values follow independent integer semantics", () => {
  const check = new Ajv({ strict: false }).compile(inferenceSchema);
  expect(check({ topology: fixture.validTopology })).toBe(true);
  expect(parsePlaygroundTopology(fixture.validTopology)).toEqual(fixture.validTopology);
  for (const depth of fixture.invalidDepths) { const value = { ...fixture.validTopology, depth }; expect(check({ topology: value })).toBe(false); expect(() => parsePlaygroundTopology(value)).toThrow(); }
});

test("Playground actual Snapshot owns both semantic SQLite directions", () => {
  expect(Object.hasOwn(snapshot, "playgroundSnapshotToSqliteDatabase")).toBe(true);
  expect(Object.hasOwn(snapshot, "playgroundSnapshotFromSqliteDatabase")).toBe(true);
});

for (const { name, ...value } of fixture.nativeCases) test("Playground literal persisted schema " + name, () => {
  expect(validate(value)).toBe(true);
  expect(snapshot.parsePlaygroundSnapshot(value)).toEqual(value);
});

test("Playground closed persisted shape follows the independent JSON schema", () => {
  for (const value of fixture.invalidSnapshots) {
    expect(validate(value)).toBe(false);
    expect(() => snapshot.parsePlaygroundSnapshot(value)).toThrow();
  }
  for (const units of fixture.invalidUtf16CodeUnits) expect(() => snapshot.parsePlaygroundSnapshot({ schema: String.fromCharCode(...units) })).toThrow();
});

test("Playground hand-authored SQL is independently queryable without a carrier", () => {
  const db = new Database(":memory:", { safeIntegers: true });
  try {
    db.exec(sql);
    expect(db.query("SELECT name FROM sqlite_schema WHERE type='table'").all()).toEqual([{ name: "playground_document" }]);
    expect(db.query("PRAGMA table_info(playground_document)").all().length).toBe(fixture.tableWidths.playground_document);
    for (const value of fixture.nativeCases) {
      db.query("INSERT OR REPLACE INTO playground_document VALUES(1,?)").run(value.schema);
      expect(db.query("SELECT schema FROM playground_document").get()).toEqual({ schema: value.schema });
      expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    }
  } finally { db.close(); }
});

test("Playground real SQLite bytes preserve an independently edited and renumbered marker", async () => {
  const value = snapshot.parsePlaygroundSnapshot({ schema: fixture.nativeCases[2]!.schema });
  const bytes = await exportSqliteDatabase(await capability.playgroundSnapshotToSqliteDatabase(value));
  const db = Database.deserialize(bytes, { safeIntegers: true });
  try {
    expect(db.query("SELECT schema FROM playground_document").get()).toEqual(value);
    db.query("UPDATE playground_document SET id=id+?,schema=?").run(fixture.identityOffset, fixture.nativeCases[3]!.schema);
    expect(await capability.playgroundSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual({ schema: fixture.nativeCases[3]!.schema });
  } finally { db.close(); }
});

test("Playground rejects malformed relational ownership and actual admission frontiers", async () => {
  const value = { schema: fixture.nativeCases[0]!.schema }, database = await capability.playgroundSnapshotToSqliteDatabase(value);
  for (const edit of ["DELETE FROM playground_document", "INSERT INTO playground_document VALUES(2,'second')", "UPDATE playground_document SET id=0", "UPDATE playground_document SET schema=CAST(X'00' AS BLOB)"]) {
    const db = Database.deserialize(await exportSqliteDatabase(database));
    try { db.run(edit); await expect(capability.playgroundSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).rejects.toThrow(); }
    finally { db.close(); }
  }
  await expect(capability.playgroundSnapshotToSqliteDatabase(value, { maxRows: 0 })).rejects.toThrow();
  await expect(capability.playgroundSnapshotToSqliteDatabase(value, { maxSchemaBytes: new TextEncoder().encode(sql).length - 1 })).rejects.toThrow();
  await expect(capability.playgroundSnapshotToSqliteDatabase(value, { maxValueBytes: 8 + new TextEncoder().encode(value.schema).length - 1 })).rejects.toThrow();
  const signal = new AbortController(); signal.abort();
  await expect(capability.playgroundSnapshotToSqliteDatabase(value, { signal: signal.signal })).rejects.toThrow();
});

test("Playground cancellation occurs inside actual long scalar work in both directions", async () => {
  const value = { schema: fixture.largeTextUnit.repeat(fixture.largeTextRepeats) };
  const database = await capability.playgroundSnapshotToSqliteDatabase(value);
  for (const phase of ["projectSnapshot", "reconstructSnapshot"] as const) {
    const controller = new AbortController(); let interior = false;
    const options: ArtifactSqliteOptions = { signal: controller.signal, onProgress: event => { if (event.phase === phase && event.total > 16384 && event.completed > 0 && event.completed < event.total) { interior = true; controller.abort(); } } };
    await expect(phase === "projectSnapshot" ? capability.playgroundSnapshotToSqliteDatabase(value, options) : capability.playgroundSnapshotFromSqliteDatabase(database, options)).rejects.toThrow();
    expect(interior).toBe(true);
  }
});
