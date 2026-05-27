/** One-off: stamp `primitiveKinds` on every shipped typology JSON from core inference. */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = join(import.meta.dir, "../../../../../..");

function inferTypologyPrimitiveKinds(typologyId: string): string[] {
  const id = typologyId.toLowerCase();
  if (id.includes(".selection.") || id.includes(".command.")) return [];
  if (id.includes(".measure.") && id.includes("volume")) return [];
  if (id.includes(".entity.") || id.includes("create-anchor")) return ["anchor"];
  if (id.includes(".measure.")) return ["anchor"];
  if (id.includes(".curve.")) return ["edge", "wire"];
  if (id.includes(".surface.")) return ["face"];
  if (id.includes(".primitive.") || id.includes(".solid.")) return ["solid"];
  if (id.includes(".feature.extrude")) return ["solid"];
  if (id.includes(".feature.offset")) return ["face", "solid"];
  if (id.includes("energy.energy.") || id.includes("structure.structure.")) return ["solid"];
  if (id.includes("lineelement") || id.includes("surfaceelement") || id.includes("solidelement")) return ["solid"];
  if (id.includes(".transform.") || id.includes(".edit.")) return ["vertex", "edge", "wire", "face", "solid"];
  return ["solid"];
}

function walkTypologyJsonFiles(dir: string, out: string[]): void {
  for (const name of readdirSync(dir, { withFileTypes: true })) {
    const p = join(dir, name.name);
    if (name.isDirectory()) {
      if (name.name === "action" || name.name === "interaction") continue;
      walkTypologyJsonFiles(p, out);
      continue;
    }
    if (!name.name.endsWith(".json")) continue;
    if (name.name === "typology.json" || name.parentPath.endsWith(`${name.name === "typology.json" ? "" : "typology"}`)) {
      const parent = join(dir, "..");
      if (dir.endsWith("typology") || name.name === "typology.json") out.push(p);
    }
  }
}

function collectTypologyPaths(): string[] {
  const root = join(repoRoot, "spatial/assets/modelDefinition");
  const out: string[] = [];
  const walk = (dir: string): void => {
    for (const ent of readdirSync(dir, { withFileTypes: true })) {
      const p = join(dir, ent.name);
      if (ent.isDirectory()) {
        if (ent.name === "action" || ent.name === "interaction") continue;
        walk(p);
        continue;
      }
      if (!ent.name.endsWith(".json")) continue;
      if (ent.name === "typology.json") {
        out.push(p);
        continue;
      }
      if (dir.replace(/\\/g, "/").endsWith("/typology") || dir.replace(/\\/g, "/").endsWith("\\typology")) {
        out.push(p);
      }
    }
  };
  walk(root);
  return [...new Set(out)].sort();
}

let updated = 0;
for (const file of collectTypologyPaths()) {
  const raw = JSON.parse(readFileSync(file, "utf8")) as Record<string, unknown>;
  if (raw.schema !== "spatial.typology/v1" || typeof raw.id !== "string") continue;
  const kinds = inferTypologyPrimitiveKinds(raw.id);
  const cur = JSON.stringify(raw.primitiveKinds ?? null);
  const next = JSON.stringify(kinds);
  if (cur === next) continue;
  raw.primitiveKinds = kinds;
  writeFileSync(file, `${JSON.stringify(raw, null, 2)}\n`);
  updated++;
}
console.log(`[patch-typology-primitive-kinds] updated ${updated} typology files`);
