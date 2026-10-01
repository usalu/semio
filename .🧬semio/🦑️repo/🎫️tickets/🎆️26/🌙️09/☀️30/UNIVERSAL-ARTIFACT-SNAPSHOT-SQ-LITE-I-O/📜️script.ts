/** 📦️ Consumer-side verification of the public semantic relational SQLite API. */
import { strict as assert } from "node:assert";
import { Database } from "bun:sqlite";
import { exportSqliteDatabase, importSqliteDatabase, type SqliteDatabase, type SqliteDatabaseOptions, type SqliteDatabaseProgress } from "@semio-tech/framework";

const encode: (value: SqliteDatabase, options?: SqliteDatabaseOptions) => Promise<Uint8Array> = exportSqliteDatabase;
const decode: (bytes: Uint8Array, options?: SqliteDatabaseOptions) => Promise<SqliteDatabase> = importSqliteDatabase;
const input: SqliteDatabase = { tables: [
  { name: "entity", sql: "CREATE TABLE entity (id INTEGER PRIMARY KEY, name TEXT NOT NULL)", rows: [{ rowid: 1n, values: [1n, "semantic entity"] }] },
  { name: "relation", sql: "CREATE TABLE relation (id INTEGER PRIMARY KEY, target INTEGER NOT NULL REFERENCES entity(id))", rows: [{ rowid: 1n, values: [1n, 1n] }] },
] };
const progress: SqliteDatabaseProgress[] = [];
const options: SqliteDatabaseOptions = { maxValueBytes: 1024, maxSchemaBytes: 1024, maxRows: 4, maxColumns: 4, maxTables: 2, maxFileBytes: 65536, maxPages: 16, onProgress: (event) => progress.push(event) };
const bytes: Uint8Array = await encode(input, options);
const output: SqliteDatabase = await decode(bytes, options);
assert.deepEqual(output, input);
const db = Database.deserialize(bytes);
try {
  assert.deepEqual(db.query("PRAGMA integrity_check").get(), { integrity_check: "ok" });
  assert.deepEqual(db.query("SELECT entity.name FROM relation JOIN entity ON entity.id=relation.target").get(), { name: "semantic entity" });
} finally { db.close(); }
assert.equal(progress.at(-1)?.phase, "readPages");
assert.equal(progress.at(-1)?.completed, progress.at(-1)?.total);
console.log("[DEBUG] public relational SQLite API", JSON.stringify({ tables: output.tables.map((table) => table.name), rows: output.tables.map((table) => table.rows.length), bytes: bytes.length, phases: Array.from(new Set(progress.map((event) => event.phase))) }));
