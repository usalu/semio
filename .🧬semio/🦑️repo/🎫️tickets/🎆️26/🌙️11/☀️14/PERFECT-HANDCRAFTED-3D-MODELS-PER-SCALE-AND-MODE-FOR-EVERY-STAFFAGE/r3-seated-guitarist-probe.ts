/** 🔎️ Prints, for a pose of the candidate document of `humans/seated-guitarist`, the bone frames and where the shoulders, elbows, wrists and landmarks of the guitar lie (world metres and guitar coordinates).
 * Usage: `bun r3-seated-guitarist-probe.ts [pose]` */
import { rooted, framesOf, at } from "./r3-seated-guitarist-roots.ts";
import { GUITAR, onGuitar, type V3 } from "./r3-seated-guitarist-ref.ts";
const asset = rooted().asset;
const pose = asset.poses.find((p: any) => p.id === (process.argv[2] ?? "strumming"));
const f = framesOf(asset.rig, pose.bones);
const g = f.guitar!;
const inGuitar = (p: V3): string => {
  const d = [0, 1, 2].map((k) => p[k]! - g.o[k]!);
  const [u, v, w] = [g.u, g.v, g.w].map((ax) => d[0]! * ax[0] + d[1]! * ax[1] + d[2]! * ax[2]);
  return `a ${(u! + GUITAR.pivot[0]).toFixed(3)} b ${(-w! + GUITAR.pivot[1]).toFixed(3)} c ${(-v! + GUITAR.pivot[2]).toFixed(3)}`;
};
const show = (name: string, p: V3): void => console.log(name.padEnd(14), p.map((x) => x.toFixed(3)).join(" "), " | ", inGuitar(p));
for (const id of ["upper-arm-r", "forearm-r", "hand-r", "upper-arm-l", "forearm-l", "hand-l", "head", "thigh-r", "shank-r", "foot-r"]) show(id, f[id]!.o);
show("tail", at(g, onGuitar(0, 0, -0.06)));
show("lower-bout-top", at(g, onGuitar(0.14, 0.1985, 0)));
show("lower-bout-back", at(g, onGuitar(0.14, 0.1985, -0.12)));
show("waist-back", at(g, onGuitar(0.29, 0, -0.11)));
show("upper-back-top", at(g, onGuitar(0.43, 0.146, -0.1)));
show("nut", at(g, onGuitar(GUITAR.nut, 0, 0)));
show("head-end", at(g, onGuitar(GUITAR.length, 0, -0.04)));
for (const id of ["hand-l", "hand-r", "forearm-r", "forearm-l", "guitar", "upper-arm-r"]) console.log(id, "u", f[id]!.u.map((x) => x.toFixed(2)).join(" "), "v", f[id]!.v.map((x) => x.toFixed(2)).join(" "), "w", f[id]!.w.map((x) => x.toFixed(2)).join(" "), "wz(det)", f[id]!.w[2].toFixed(2));
const t = f.torso!;
for (const [name, p] of [["waist-back", onGuitar(0.29, 0, -0.11)], ["lower-bout-back", onGuitar(0.14, 0.1985, -0.12)], ["upper-back-top", onGuitar(0.43, 0.146, -0.1)], ["mid-back", onGuitar(0.2, 0.1, -0.118)]] as [string, V3][]) {
  const q = at(g, p);
  const d = [0, 1, 2].map((k) => q[k]! - t.o[k]!);
  console.log(name, "in torso frame: s", (d[0]! * t.u[0] + d[1]! * t.u[1]).toFixed(3), "v", (d[0]! * t.v[0] + d[1]! * t.v[1]).toFixed(3), "w", d[2]!.toFixed(3));
}
