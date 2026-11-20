/** 🩻️ Round 3 English oak: every structure solid with vertices outside the projected body of its scale, with the largest distance to a body vertex. Usage: bun r3-english-oak-inside.ts <scale> */
import { entryOf, scaleMeshes } from "./volumes-lib.ts";
const scale = Number(process.argv[2] ?? 50);
const { asset } = entryOf("vegetation/english-oak");
const sm = scaleMeshes(asset, scale, asset.volumes[String(scale)]) as any;
for (const view of ["elevation", "front"]) {
  const p = (mesh: any, k: number): [number, number] => [view === "front" ? mesh.world[3 * k + 2] : mesh.world[3 * k], mesh.world[3 * k + 1]];
  const tris: number[][] = [];
  sm.body.forEach((mesh: any, i: number) => {
    if (sm.solids.body[i].texture === true) return;
    const idx = mesh.indices ?? mesh.triangles;
    for (let k = 0; k < idx.length; k += 3) tris.push([...p(mesh, idx[k]), ...p(mesh, idx[k + 1]), ...p(mesh, idx[k + 2])]);
  });
  const inside = (x: number, y: number): boolean => tris.some(([ax, ay, bx, by, cx, cy]) => {
    const d1 = (x - bx!) * (ay! - by!) - (ax! - bx!) * (y - by!);
    const d2 = (x - cx!) * (by! - cy!) - (bx! - cx!) * (y - cy!);
    const d3 = (x - ax!) * (cy! - ay!) - (cx! - ax!) * (y - ay!);
    return !((d1 < 0 || d2 < 0 || d3 < 0) && (d1 > 0 || d2 > 0 || d3 > 0));
  });
  sm.structure.forEach((mesh: any, i: number) => {
    const s = sm.solids.structure[i];
    let [n, at] = [0, ""];
    for (let k = 0; k < mesh.world.length / 3; k++) {
      const [x, y] = p(mesh, k);
      if (!inside(x, y)) { n++; at = `${x.toFixed(2)},${y.toFixed(2)}`; }
    }
    if (n > 0) console.log(view, s.id, s.part, `${n} vertices outside, e.g. ${at}`);
  });
}
