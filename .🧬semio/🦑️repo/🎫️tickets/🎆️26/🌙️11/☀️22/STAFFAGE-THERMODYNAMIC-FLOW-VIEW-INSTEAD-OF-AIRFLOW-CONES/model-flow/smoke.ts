/** 🔬️ Smoke probe of the flow geometry of a model: builds the model of a fixture asset, validates it, tessellates every scene at its scale's quality and prints nodes, solids, materials and triangles of the flow per thermal scene. Run with bun from the repo root; the fixture name is the first argument. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { validateModel } from "../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { modelOfAsset } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🟦️.ts";
import { qualityOf } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts";
import { resolveScenes } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🗿️model/🧩️scenes/🟦️.ts";
import { writeGlbDetailed } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts";

const S = join(import.meta.dir, "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const name = process.argv[2] ?? "👤️sample-figure";
const asset = JSON.parse(readFileSync(join(S, "🧫️fixtures", name, "🔣️.json"), "utf8")).asset;
const environment = process.argv[3] === "cold" ? { ...asset.thermal.environment, airTempC: -5, meanRadiantTempC: -5, relativeHumidity: 0.8 } : undefined;
let t = performance.now();
let reports = 0;
const document = await modelOfAsset(asset, { at: { kind: "pose", pose: "stand" }, poses: ["stand"], clips: [], ...(environment ? { environment } : {}) }, { report: () => void reports++, cancelled: () => false });
console.log("model ms", Math.round(performance.now() - t), "reports", reports);
console.log("issues", JSON.stringify(validateModel(document).slice(0, 8)));
const resolved = await resolveScenes(document, (scale) => qualityOf(asset, scale), { report: () => {}, cancelled: () => false });
for (const scene of resolved.scenes!) {
  const meshes = new Map(resolved.meshes.map((m) => [m.id, m]));
  let all = 0, flow = 0;
  for (const id of scene.nodes) {
    const node = resolved.nodes.find((n) => n.id === id)!;
    if (node.mesh === undefined) continue;
    const tris = meshes.get(node.mesh)!.primitives.reduce((n, p) => n + (p.mode === "triangles" ? p.indices.length / 3 : 0), 0);
    all += tris;
    if (node.tags?.includes("flow")) flow += tris;
  }
  const solids = document.solids.filter((s) => s.id.startsWith(`${scene.id}-flow`));
  console.log(scene.id, "triangles", all, "flow", flow, "flow solids", solids.length, "lofts", solids.filter((s) => s.kind === "loft").length, "cones", solids.filter((s) => s.kind === "revolve").length, "puffs", solids.filter((s) => s.kind === "polyhedron").length, "sections", solids.reduce((n, s) => n + (s.kind === "loft" ? s.sections.length : 0), 0));
}
console.log("flow materials", document.materials.filter((m) => m.id.includes("flow")).map((m) => `${m.id}:${m.baseColor.map((c) => c.toFixed(2)).join(",")}`).slice(0, 40).join(" | "));
t = performance.now();
const glb = await writeGlbDetailed(asset, { pose: "stand", ...(environment ? { environment } : {}) });
console.log("glb bytes", glb.bytes.length, "triangles", glb.triangles, "ms", Math.round(performance.now() - t));
