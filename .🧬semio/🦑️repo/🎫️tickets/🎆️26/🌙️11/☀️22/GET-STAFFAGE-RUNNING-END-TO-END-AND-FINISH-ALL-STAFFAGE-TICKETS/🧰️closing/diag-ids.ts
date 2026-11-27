/** 🔎️ Finds the node ids longer than 64 characters in the model JSON of one asset (closing diagnosis of `🧊️model-catalog`): `bun diag-ids.ts <asset>`. */
import { mkdtempSync, readFileSync, readdirSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { exportModels } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/📦️batch/🟦️.ts";
const out = mkdtempSync(join(tmpdir(), "staffage-ids-"));
try {
  await exportModels({ out, assets: [process.argv[2]!], views: ["light-solid"] });
  const walk = (dir: string): string[] => readdirSync(dir, { withFileTypes: true }).flatMap((e) => (e.isDirectory() ? walk(join(dir, e.name)) : [join(dir, e.name)]));
  for (const file of walk(out).filter((f) => f.endsWith(".json") && !f.endsWith("manifest.json")).slice(0, 3)) {
    const doc = JSON.parse(readFileSync(file, "utf8")) as { nodes?: { id: string }[] };
    const long = (doc.nodes ?? []).filter((n) => n.id.length > 64).slice(0, 6);
    console.log(file.split("/").pop(), (doc.nodes ?? []).length, "nodes", long.map((n) => `${n.id.length}:${n.id}`));
  }
} finally {
  rmSync(out, { recursive: true, force: true });
}
