import { readFileSync } from "node:fs";
import { modelOf } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🟦️.ts";
const id = process.argv[2] ?? "standing-adult";
const file = process.argv[3] ?? `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/${id}/🔣️.json`;
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
const t0 = performance.now();
const doc = await modelOf(asset, { scale: 100, at: { kind: "pose", pose: asset.presentation.defaultPose }, poses: [asset.presentation.defaultPose], clips: [] });
console.log("[DEBUG] modelOf ms", Math.round(performance.now() - t0), "nodes", doc.nodes.length, "solids", doc.solids.length, "materials", doc.materials.length);
const glyph = doc.solids.filter((s) => s.tags?.includes("glyph"));
console.log("[DEBUG] glyph solids", glyph.length, glyph.slice(0, 3).map((s) => `${s.id}:${s.material}`));
for (const v of doc.variants.slice(0, 7)) {
  const g = glyph.slice(0, 2).map((s) => { const b = v.bindings[s.material]; const m = doc.materials.find((x) => x.id === (b ?? s.material)); return `${s.material}->${b ?? "(base)"} alpha=${m?.baseColor[3]} mode=${m?.alphaMode}`; });
  console.log("[DEBUG]", v.id, g.join(" | "));
}
import { EDGE_ANGLE_RAD } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🗿️model/🟦️.ts";
import { qualityOf } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts";
import { featureEdges, tessellate } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
const t1 = performance.now();
const resolved = await tessellate(doc, qualityOf(asset), { report: () => {}, cancelled: () => false });
console.log("[DEBUG] tessellate ms", Math.round(performance.now() - t1));
const t2 = performance.now();
const edged = await featureEdges(resolved, EDGE_ANGLE_RAD, { material: "edge" }, { report: () => {}, cancelled: () => false });
console.log("[DEBUG] edges ms", Math.round(performance.now() - t2));
const edge = edged.materials.find((m) => m.id === "edge");
console.log("[DEBUG] edge material", JSON.stringify(edge));
for (const v of edged.variants) console.log("[DEBUG] variant", v.id, "edge ->", v.bindings["edge"] ?? "(base)", JSON.stringify(edged.materials.find((m) => m.id === (v.bindings["edge"] ?? "edge"))?.baseColor));
const lines = edged.meshes.flatMap((m) => m.primitives).filter((p) => p.mode === "lines");
console.log("[DEBUG] line primitives", lines.length, "line vertices", lines.reduce((n, p) => n + p.positions.length / 3, 0));
