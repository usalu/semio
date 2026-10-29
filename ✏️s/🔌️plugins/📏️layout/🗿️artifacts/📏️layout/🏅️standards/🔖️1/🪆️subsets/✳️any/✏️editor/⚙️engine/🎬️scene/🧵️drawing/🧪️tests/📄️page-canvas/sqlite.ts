/** 🧪️ Reference-only page members roundtrip through real SQLite and reject ambiguous ownership. */
import { deepStrictEqual, strictEqual, rejects } from "node:assert";
import fixtures from "./🔣️.json";
import graph from "../../../../../../🧪️tests/🧫️fixtures/🕸️graph/🔣️.json";
import { layoutArtifactFromNativeJson, layoutArtifactNativeJson } from "../../../../../../🚪️io/📝️text/📸️snapshot/🪪️native-json/🟦️.ts";
import { layoutSnapshotToSqliteDatabase, layoutSnapshotFromSqliteDatabase } from "../../../../../../🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
import { validateDocument } from "../../../../../../🧬️schema/✅️validation/🟦️.ts";
import type { LayoutDrawingMember } from "../../../../../../🧬️schema/🟦️.ts";
import type { SqliteDatabase, SqliteValue } from "@semio-tech/framework/io/sqlite-snapshot";
type MutableDatabase = { tables: { name: string; sql: string; rows: { rowid: bigint; values: SqliteValue[] }[] }[] };
export interface PageSqliteOracle { roundTrip(database: SqliteDatabase): SqliteDatabase }
export async function testPageDrawingSqlite(oracle: PageSqliteOracle): Promise<{ sqliteCases: number; sqliteMembers: number; sqliteRefusals: number }> {
  let count = 0;
  let original: SqliteDatabase | undefined;
  for (const fixture of fixtures.sqliteCases) {
    const doc = layoutArtifactFromNativeJson(graph.cases[0]!.before), page = structuredClone(doc.pages[0]!);
    page.id = "page-two"; page.frames = []; page.overrides = []; page.parentPageId = null;
    for (const layer of page.layers) layer.objectIds = [];
    doc.pages.push(page);
    doc.spreads.find(spread => spread.id === page.spreadId)!.pageIds.push(page.id);
    for (const entry of fixture.members) {
      const value: LayoutDrawingMember = { handle: { childId: entry.childId, target: { artifactId: entry.artifactId, dialect: { artifactKind: "s.draw.drawing", standard: "1", subset: "*" } } } };
      if (entry.owner === "document") doc.backgroundDrawing = value;
      else doc.pages[entry.page!]!.backgroundDrawing = value;
    }
    const saved = structuredClone(doc), database = await layoutSnapshotToSqliteDatabase(doc);
    const members = database.tables.find(table => table.name === "layout_drawing_member")!.rows;
    deepStrictEqual(members.map(row => row.rowid), fixture.members.map(entry => BigInt(entry.id)));
    strictEqual(database.tables.some(table => table.name.startsWith("semio_drawing_")), false);
    const restored = await layoutSnapshotFromSqliteDatabase(oracle.roundTrip(database));
    strictEqual(validateDocument(restored), null);
    deepStrictEqual(layoutArtifactNativeJson(restored), layoutArtifactNativeJson(doc), fixture.name);
    deepStrictEqual(doc, saved);
    count += fixture.members.length;
    original ??= database;
  }
  if (!original) throw Error("SQLite member fixture missing");
  const members = (db: MutableDatabase) => db.tables.find(table => table.name === "layout_drawing_member")!.rows;
  const mutations: ((db: MutableDatabase) => void)[] = [
    db => { const rows = members(db), row = structuredClone(rows[1]!); row.rowid = 4n; row.values[0] = 4n; rows.push(row); },
    db => { members(db)[1]!.values[2] = 999n; },
    db => { const rows = members(db); rows[2]!.values[3] = rows[1]!.values[3]!; },
    db => { const rows = members(db); rows[2]!.values[7] = rows[1]!.values[7]!; },
    db => { members(db)[0]!.values[4] = "s.stdio.semio"; },
    db => { members(db)[0]!.values[2] = 1n; },
    db => { members(db)[0]!.values[3] = ""; },
    db => { members(db)[0]!.values[7] = ""; },
  ];
  strictEqual(mutations.length, fixtures.sqliteRefusals.length);
  for (const [index, mutate] of mutations.entries()) {
    const bad = structuredClone(original) as MutableDatabase;
    mutate(bad);
    await rejects(() => layoutSnapshotFromSqliteDatabase(bad), () => true, fixtures.sqliteRefusals[index]);
    let refused = false;
    try { oracle.roundTrip(bad); } catch { refused = true; }
    strictEqual(refused, true, "independent SQLite member oracle: " + fixtures.sqliteRefusals[index]);
  }
  return { sqliteCases: fixtures.sqliteCases.length, sqliteMembers: count, sqliteRefusals: mutations.length };
}
