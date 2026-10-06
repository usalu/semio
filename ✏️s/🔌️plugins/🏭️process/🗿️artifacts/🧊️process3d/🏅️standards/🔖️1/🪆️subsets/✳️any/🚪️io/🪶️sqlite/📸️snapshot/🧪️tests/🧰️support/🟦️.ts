/** 🏭️ Authored complete Process3d fixture and independent SQLite input factory. */
import { Database } from "bun:sqlite";
import type { Process3dSnapshot, Process3dWorkingSolid, Process3dMeasureRecipe, StockQuantity } from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { process3dSnapshotToSqliteDatabase } from "../../🟦️.ts";
import corpus from "../../🧫️fixtures/🔣️.json";
import cohort from "../../🧫️fixtures/🚦️public/🔣️.json";
import sql from "../../🗄️.sql" with { type: "text" };
import metadata from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧬️schema/🗄️.sql" with { type: "text" };

/** 🧫️ Creates the actual Source owner without a native carrier or inferred model. */
export function process3dSqliteFixture(hex: string): Process3dSnapshot {
  const f = { bits: BigInt("0x" + hex) };
  const pose = { position: [f, f, f] as [typeof f, typeof f, typeof f], axis: [f, f, f] as [typeof f, typeof f, typeof f], angle: f };
  const literal = corpus.literal;
  const recipes: Process3dMeasureRecipe[] = [{ recipe: "discCut", diameter: literal, kerf: "" }, { recipe: "bladeCut", kerf: "", length: literal, depth: "" }, { recipe: "pocketCut", diameter: literal, depth: "" }, { recipe: "boreDrill", radius: "", depth: literal }, { recipe: "cylinderAttach", radius: literal, length: "" }, { recipe: "boxAttach", width: "", depth: literal, height: "" }];
  const solids: Process3dWorkingSolid[] = [{ kind: "box", width: f, depth: f, height: f }, { kind: "cylinder", radius: f, height: f }, { kind: "sphere", radius: f }, { kind: "importedMesh", meshUrl: literal }, { kind: "importedSolid", solidHandle: literal }, { kind: "reference", referenceId: literal }];
  const child = (n: number) => { const c = corpus.childAddresses[n]!; return { childId: c.childId, target: { artifactId: c.artifactId, dialect: { artifactKind: c.artifactKind, standard: c.standard, subset: c.subset } } }; };
  return {
    workshop: { machines: [{ id: "", label: literal, iconId: "", catalogId: "", capabilities: recipes.map(recipe => ({ id: "", label: literal, iconId: "", recipe, parameters: [{ id: literal, label: "", value: f }], rules: (corpus.stockQuantities as StockQuantity[]).flatMap(quantity => [{ kind: "min" as const, quantity, parameter: literal, margin: f }, { kind: "max" as const, quantity, parameter: "", margin: f }]) })) }, { id: "", label: "", iconId: literal, capabilities: [] }] },
    stockId: corpus.rootStock.id, stockLabel: corpus.rootStock.label, stockPose: pose,
    stockPayload: { id: corpus.payloadStock.id, label: corpus.payloadStock.label, solid: solids[0]!, pose },
    stockSolid: child(0), steps: child(1),
    stepPayloads: [...solids.flatMap(solid => [{ id: "", label: literal, enabled: false, origin: { machineId: "", capabilityId: literal }, measure: { measure: "cut" as const, tool: solid, pose } }, { id: "", label: literal, enabled: true, measure: { measure: "attach" as const, component: solid, pose } }]), { id: literal, label: "", enabled: true, measure: { measure: "drill", radius: f, depth: f, pose } }],
    toolSolids: [child(0), child(0)],
  };
}

/** 🪶️ Builds genuine SQLite bytes with independent BunSQLite from explicit domain rows. */
export async function independentProcess3dSqliteFile(hex: string, encoding: "binary" | "text", edited = false): Promise<Uint8Array> {
  const domain = await process3dSnapshotToSqliteDatabase(process3dSqliteFixture(hex));
  const database = new Database(":memory:", { safeIntegers: true });
  try {
    database.exec(sql + metadata);
    for (const table of domain.tables) {
      const insert = database.query("INSERT INTO " + table.name + " VALUES(" + Array(table.rows[0]?.values.length ?? corpus.tableWidths[table.name as keyof typeof corpus.tableWidths]).fill("?").join(",") + ")");
      for (const row of table.rows) insert.run(...row.values);
    }
    database.query("INSERT INTO semio_snapshot VALUES(1,?,?,?,1,?)").run(cohort.dialect.artifactKind, cohort.dialect.standard, cohort.dialect.subset, encoding);
    database.exec("PRAGMA application_id=" + cohort.applicationId + ";PRAGMA user_version=" + cohort.userVersion);
    if (edited) database.query("UPDATE " + cohort.independentEdit.table + " SET " + cohort.independentEdit.column + "=?").run(cohort.independentEdit.value);
    if ((database.query("PRAGMA integrity_check").get() as { integrity_check: string }).integrity_check !== "ok" || database.query("PRAGMA foreign_key_check").all().length) throw Error("Independent Process3d SQLite integrity");
    return database.serialize();
  } finally { database.close(); }
}
