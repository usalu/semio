/** 🔁️ Snapshot of every catalog asset that has a volume and a before/after volume-check run over that frozen snapshot.
 * Usage: `bun kernel-maintenance-volumes.ts snapshot` then `bun kernel-maintenance-volumes.ts run <label>`; outputs under `🗑️generated/kernel-maintenance/`. */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { GENERATED, catalogEntries, loadAsset } from "./volume-lib.ts";
import { checkAsset } from "./volume-check.ts";

const dir = join(GENERATED, "kernel-maintenance");
mkdirSync(dir, { recursive: true });
const snapshot = join(dir, "volumes-snapshot.json");
const [mode, label = "run"] = process.argv.slice(2);
if (mode === "snapshot") {
  const out: Record<string, unknown> = {};
  for (const entry of catalogEntries()) {
    const asset = loadAsset(entry.path);
    if (asset.volume && Array.isArray(asset.volume.solids)) out[`${entry.category}/${entry.slug}`] = asset;
  }
  writeFileSync(snapshot, JSON.stringify(out));
  console.log(Object.keys(out).length, "assets with volume");
} else if (mode === "run") {
  if (!existsSync(snapshot)) throw new Error("snapshot first");
  const assets = JSON.parse(readFileSync(snapshot, "utf8")) as Record<string, any>;
  const out: Record<string, unknown> = {};
  for (const [key, asset] of Object.entries(assets)) {
    try {
      const r = checkAsset(asset, asset.volume, 50, 0.08, 0.8);
      const loftNoRotation = asset.volume.solids.filter((s: any) => s.kind === "loft" && s.sections.some((x: any) => !x.rotation)).length;
      out[key] = { solids: r.solids, loftNoRotation, bbox: r.bbox.map((b) => b.bboxM), iou: Object.fromEntries(r.views.map((v) => [v.view, [v.iou, v.recall, v.precision]])), ok: r.ok };
    } catch (error) {
      out[key] = { error: String(error) };
    }
  }
  writeFileSync(join(dir, `volumes-${label}.json`), JSON.stringify(out, null, 1));
  console.log(Object.keys(out).length, "checked");
}
