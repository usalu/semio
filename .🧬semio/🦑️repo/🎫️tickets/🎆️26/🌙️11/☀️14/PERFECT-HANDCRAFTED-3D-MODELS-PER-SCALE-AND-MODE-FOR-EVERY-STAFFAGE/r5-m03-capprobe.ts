/** 🔬 Round 5 M03: prints the corner normals of the triangles of one body solid that face +x, to see why a flat extrude cap shades in wedges. Usage: r5-m03-capprobe.ts <asset> <scale> <solid id> */
import { load, rigAt, listsIn, cornerNormals } from "./r3-lib.ts";
const [spec, scale, id] = process.argv.slice(2);
const { asset } = load(spec, "asset scale id");
const rig = rigAt(asset, Number(scale), { kind: "pose", pose: asset.presentation.defaultPose });
const lists = listsIn(asset, Number(scale), rig.frames);
const mesh = (lists.body as any[]).find((m) => m.id === id || m.solid?.id === id);
console.log(Object.keys(mesh ?? {}), (lists.body as any[]).map((m) => m.id ?? m.solid?.id).join(" "));
if (mesh) {
  const n = cornerNormals(mesh);
  const w = mesh.world;
  let shown = 0;
  for (let t = 0; t < mesh.indices.length / 3; t++) {
    const xs = [0, 1, 2].map((k) => w[mesh.indices[t * 3 + k] * 3]);
    if (Math.min(...xs) < 0.2 || shown > 30) continue;
    shown++;
    console.log(t, xs.map((x: number) => x.toFixed(4)).join(","), [0, 1, 2].map((k) => [0, 1, 2].map((q) => n[t * 9 + k * 3 + q].toFixed(3)).join("/")).join("  "));
  }
}
