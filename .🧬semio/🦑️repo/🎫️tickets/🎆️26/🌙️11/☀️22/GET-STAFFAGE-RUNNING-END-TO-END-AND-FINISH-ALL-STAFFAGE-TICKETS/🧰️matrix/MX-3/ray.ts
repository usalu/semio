/** 🔦️ The intervals of y at which the ray of the generator intake case hits the container solids: ray.ts <scales> */
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { Ray, Triangle, Vector3 } from "three";
import { tessellateSolid } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { modelOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🟦️.ts";
const root = process.env.S + "/🗂️catalog";
const file = readdirSync(root).map((c) => join(root, c, "diesel-generator", "🔣️.json")).find((f) => { try { readFileSync(f); return true; } catch { return false; } })!;
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
for (const scale of process.argv[2]!.split(",").map(Number)) for (const view of ["light-solid", "thermal", "forces"] as const) {
  const model = await modelOf(asset, { scale, view, poses: [], clips: [] });
  const faces: Triangle[] = [];
  for (const solid of model.solids.filter((p: any) => p.node === "container" && !p.tags?.includes("part-mass"))) {
    const mesh = tessellateSolid(solid, { chordM: 0.0001, minSegments: 24, maxSegments: 48 });
    for (let i = 0; i < mesh.indices.length; i += 3) { const q = [0, 1, 2].map((j) => new Vector3().fromArray(mesh.positions, 3 * mesh.indices[i + j]!)); faces.push(new Triangle(q[0], q[1], q[2])); }
  }
  const out: string[] = [];
  for (const x of [0.52, 3.12]) {
    const hits: boolean[] = []; const p = new Vector3();
    for (let i = 0; i < 560; i++) { const y = 0.1 + i * 0.005; hits.push(faces.some((t) => new Ray(new Vector3(x, y, 1.3), new Vector3(0, 0, -1)).intersectTriangle(t.a, t.b, t.c, false, p) !== null)); }
    const spans: string[] = []; let start = -1;
    hits.forEach((h, i) => { if (h && start < 0) start = i; if ((!h || i === hits.length - 1) && start >= 0) { spans.push(`${(0.1 + start * 0.005).toFixed(3)}..${(0.1 + (i - 1) * 0.005).toFixed(3)}`); start = -1; } });
    out.push(`x ${x}: ${spans.join(" ")}`);
  }
  console.log(`1:${scale} ${view}: ${out.join(" | ")}`);
}
