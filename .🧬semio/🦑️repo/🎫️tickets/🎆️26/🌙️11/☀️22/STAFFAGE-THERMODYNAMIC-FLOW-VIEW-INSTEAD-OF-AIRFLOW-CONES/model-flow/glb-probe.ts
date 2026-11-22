/** 🔎️ Prints how the flow nodes, scenes and materials of a GLB of the sample figure look in the glTF JSON. Run with bun from the repo root. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { writeGlb } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts";

const S = join(import.meta.dir, "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const asset = JSON.parse(readFileSync(join(S, "🧫️fixtures/👤️sample-figure/🔣️.json"), "utf8")).asset;
const bytes = await writeGlb(asset, { pose: "stand", scales: [200] });
const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
const json = JSON.parse(new TextDecoder().decode(bytes.subarray(20, 20 + view.getUint32(12, true))));
console.log("scenes", JSON.stringify(json.scenes.map((s: any) => ({ name: s.name, nodes: s.nodes.length, extras: s.extras }))));
const flow = json.nodes.map((n: any, i: number) => ({ i, ...n })).filter((n: any) => n.extras?.tags?.includes("flow"));
console.log("flow nodes", flow.length, JSON.stringify(flow.slice(0, 4)));
console.log("mesh", JSON.stringify(json.meshes[flow.find((n: any) => n.mesh !== undefined).mesh]).slice(0, 900));
console.log("materials", JSON.stringify(json.materials.filter((m: any) => m.name.includes("flow")).slice(0, 3)));
