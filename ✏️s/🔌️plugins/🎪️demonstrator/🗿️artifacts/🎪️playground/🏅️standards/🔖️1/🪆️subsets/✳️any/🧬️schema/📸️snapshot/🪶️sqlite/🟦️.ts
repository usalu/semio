/** 🎪️ Handwritten semantic ownership of the actual Playground marker. */
import { parsePlaygroundSnapshot, type PlaygroundSnapshot } from "../🟦️.ts";
import type { ArtifactDialect } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts";
import type { SqliteDatabase } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import { ArtifactSqliteProjection, artifactSqliteTables, artifactSqliteInteger, artifactSqliteText, artifactSqliteCheckpoint, type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";

export const PLAYGROUND_SQLITE_SCHEMA = String.raw`CREATE TABLE playground_document (
 id INTEGER PRIMARY KEY,
 schema TEXT NOT NULL
);
`;

/** 🎯️ The actual owner declares one wildcard coordinate. */
export function validatePlaygroundSnapshotSqliteDialect(dialect: ArtifactDialect): void {
  if (dialect.artifactKind !== "s.demonstrator.playground" || dialect.standard !== "1" || dialect.subset !== "*") throw new Error("Playground SQLite dialect mismatch");
}

/** 📤️ Projects one explicitly owned string after authored schema and row admission. */
export async function playgroundSnapshotToSqliteDatabase(snapshot: PlaygroundSnapshot, options: ArtifactSqliteOptions = {}): Promise<SqliteDatabase> {
  const out = await ArtifactSqliteProjection.create(PLAYGROUND_SQLITE_SCHEMA, options);
  out.checkRowsAdditional(1);
  const value = parsePlaygroundSnapshot(snapshot);
  await out.insert("playground_document", [value.schema]);
  return out.finish();
}

/** 📥️ Consumes the exact positive singleton while preserving independent relational IDs. */
export async function playgroundSnapshotFromSqliteDatabase(database: SqliteDatabase, options: ArtifactSqliteOptions = {}): Promise<PlaygroundSnapshot> {
  const tables = await artifactSqliteTables(database, PLAYGROUND_SQLITE_SCHEMA, options), rows = tables[0]!;
  if (rows.length !== 1) throw new Error("Playground SQLite requires one persisted root");
  const row = rows[0]!;
  if (row.values.length !== 2 || row.rowid <= 0n || artifactSqliteInteger(row, 0) !== row.rowid) throw new Error("Playground SQLite root identity or width differs");
  const value = parsePlaygroundSnapshot({ schema: artifactSqliteText(row, 1) });
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", 1, 1);
  return value;
}
