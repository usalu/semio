/** 🏗️ PW-08 build: `bun build.ts <asset-id> [--publish]` assembles the asset document of one species file into `T/🗑️generated/PW-08/<id>/🔣️.json` (and with `--publish` into the catalog folder of the asset), checks the drawing frames against the product's `evaluate` and prints the hand-check numbers (envelope, masses, areas, loads).
 *
 * @see ./asset.ts */
import { existsSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { SCALES, VIEWS, bounds, frames, r1, r3, toLocal, type V3 } from "./lib.ts";
import { SCHEMA, dimensionsOf, representationsOf, rigOf, symbolOf, volumesOf, type Species } from "./asset.ts";
import { verticalIn } from "./common.ts";

const HERE = dirname(fileURLToPath(import.meta.url));
const TICKET = join(HERE, "../..");
const CATALOG = join(HERE, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation");
const id = process.argv[2]!;
const publish = process.argv.includes("--publish");
const mod = await import(`./${id}.ts`);
const sp: Species = mod.species();
const log = (s: string) => process.stderr.write(`[DEBUG] ${s}\n`);
log(`${id}: building`);
const { bones, masses } = rigOf(sp);
const total = (bones as { mass?: { kg: number } }[]).reduce((s, b) => s + (b.mass?.kg ?? 0), 0);
const dims = dimensionsOf(sp, total);
const phys = mod.physics(sp, { bones, masses });
const b50 = bounds(sp.scales[50].parts.map((p) => p.mesh));
const longest = Math.max(b50.max[0] - b50.min[0], b50.max[1] - b50.min[1], b50.max[2] - b50.min[2]);
log(`dims ${JSON.stringify(dims)}`);
for (const s of SCALES) {
  const b = bounds(sp.scales[s].parts.map((p) => p.mesh));
  log(`1:${s} body x ${r3(b.min[0])}..${r3(b.max[0])} y ${r3(b.min[1])}..${r3(b.max[1])} z ${r3(b.min[2])}..${r3(b.max[2])} parts ${sp.scales[s].parts.length} tris ${sp.scales[s].parts.reduce((n, p) => n + p.mesh.t.length, 0)}`);
}
const { reps, drawn } = representationsOf(sp, longest);
log("representations drawn");
const fr = frames(sp.bones);
const anchors: unknown[] = [{ id: "base-support", bone: sp.bones[0]!.id, x: 0, y: 0, kind: "support", title: { en: "Root plate anchorage", de: "Verankerung des Wurzeltellers" }, support: "fixed", normalDeg: 90, footprintLengthM: phys.supportFootprintM, footprintWidthM: phys.supportFootprintM, frictionCoefficient: 0.6 }];
const emitters = sp.emitters.map((e) => {
  const ids = e.anchors.map((a, k) => {
    const aid = `${e.id.replace("transpiration", "tp")}-${k + 1}`;
    const local = toLocal(fr[a.bone]!, a.at);
    anchors.push({ id: aid, bone: a.bone, x: r3(local[0]), y: r3(local[1]), ...(Math.abs(local[2]) > 1e-4 ? { z: r3(local[2]) } : {}), kind: "emitter", title: { en: `${e.title.en.split(" (")[0]}, anchor ${k + 1}`, de: `${e.title.de.split(" (")[0]}, Anker ${k + 1}` } });
    return aid;
  });
  const first = sp.bones.find((b) => b.id === e.anchors[0]!.bone)!;
  return { id: e.id, kind: "transpiration", title: e.title, anchor: ids[0], anchors: ids, directionDeg: verticalIn(first), diameterM: e.diameterM, region: e.region, source: e.source, reference: e.reference };
});
const plan500 = sp.scales[500];
const symbol = symbolOf(sp, drawn[500]!, { bone: sp.bones[0]!.id, d: sp.symbolStemD, topY: 0 });
const asset = {
  id: sp.id, category: "vegetation", title: sp.title, summary: sp.summary, tags: sp.tags,
  provenance: { author: "Semio staffage plant collection worker PW-08", method: "handcrafted", created: "2026-11-22", references: sp.references, notes: sp.notes },
  dimensions: dims, materials: sp.materials, rig: { kind: "botanical", root: sp.bones[0]!.id, bones }, volumes: volumesOf(sp), anchors,
  representations: reps, symbols: [symbol], poses: phys.poses, clips: phys.clips,
  thermal: { environment: sp.thermalEnvironment, regions: phys.regions, emitters, assumptions: phys.thermalAssumptions, operating: [], passengers: [] },
  structure: { gravityMS2: 9.80665, dynamics: "quasi-static", supports: ["base-support"], assumptions: phys.structureAssumptions, clinical: false },
  presentation: { defaultPose: "calm", defaultClip: "wind-sway", defaultScale: sp.defaultScale },
  exports: ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
  completion: "authored",
};
void plan500;
const doc = { $schema: SCHEMA, version: "semio.staffage/v2", asset };
const out = join(TICKET, "🗑️generated", "PW-08", id, "🔣️.json");
mkdirSync(dirname(out), { recursive: true });
writeFileSync(out, `${JSON.stringify(doc)}\n`);
log(`wrote ${out}`);
for (const s of SCALES) {
  const state = evaluate(asset as never, { kind: "pose", pose: "calm" } as never, { kind: "scale", scale: { denominator: s } } as never) as { shapes: Record<string, { shape: { id: string }; rings: { x: number; y: number }[][] }[]> };
  for (const view of VIEWS) {
    let worst = 0;
    for (const d of drawn[s]!.filter((x) => x.view === view)) {
      const placed = state.shapes[view]!.find((e) => e.shape.id === d.group.id);
      if (placed === undefined || d.rings.length === 0) continue;
      const ring = placed.rings[0]!;
      for (let k = 0; k < Math.min(ring.length, d.rings[0]!.length); k += 7) worst = Math.max(worst, Math.hypot(ring[k]!.x - d.rings[0]![k]![0], ring[k]!.y - d.rings[0]![k]![1]));
    }
    log(`frame check 1:${s} ${view}: largest placed-vs-drawn distance ${(worst * 1000).toFixed(2)} mm, shapes ${(reps[String(s)] as { geometry: Record<string, unknown[]> }).geometry[view]!.length}`);
  }
}
for (const p of phys.poses as { id: string; loads: { fxN: number; fyN: number }[]; bones: Record<string, { angleDeg: number }> }[]) log(`pose ${p.id}: loads fx ${r1(p.loads.reduce((s, l) => s + l.fxN, 0))} N fy ${r1(p.loads.reduce((s, l) => s + l.fyN, 0))} N, bones ${JSON.stringify(Object.fromEntries(Object.entries(p.bones).slice(0, 6)))}`);
log(`mass ${r1(total)} kg; regions ${JSON.stringify((phys.regions as { id: string; areaM2: number }[]).map((r) => [r.id, r.areaM2]))}`);
if (publish) {
  const row = { id: sp.id, category: "vegetation", title: sp.title, heightM: dims.heightM, widthM: dims.widthM, powerW: 0, poses: (phys.poses as { id: string }[]).map((p) => p.id), clips: (phys.clips as { id: string }[]).map((c) => c.id), representations: { "50": "authored", "100": "authored", "200": "authored", "500": "authored" }, symbols: [`${sp.id}-symbol`], analytics: ["thermal", "transport", "structural"], exports: asset.exports, distinction: sp.distinction, completion: "authored" };
  const rowPath = join(TICKET, "📥️manifest-rows", `${id}.json`);
  mkdirSync(dirname(rowPath), { recursive: true });
  writeFileSync(rowPath, `${JSON.stringify(row, null, 2)}\n`);
  const dest = join(CATALOG, id, "🔣️.json");
  mkdirSync(dirname(dest), { recursive: true });
  writeFileSync(dest, `${JSON.stringify(doc)}\n`);
  log(`published ${dest} (existed before: ${existsSync(dest)})`);
}
void (null as unknown as V3);
