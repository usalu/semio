/** 🔎 Prints the lofts of `humans/walking-commuter` at 1:50 as figure-space stations (centre, half extents along the section axes) and the bone frames of the default pose, the ruler for the rework.
 *
 * @see ./rework/assets/humans--walking-commuter.md */
import { makeKit, openCatalog, type V3 } from "./rework-04-kit.ts";
import type { Json } from "./volumes-lib.ts";

const { asset } = openCatalog("humans/walking-commuter");
const kit = makeKit(asset);
const f3 = (v: readonly number[]): string => v.map((x) => x.toFixed(3)).join(",");
for (const [id, f] of Object.entries(kit.FRAMES)) console.log(`frame ${id.padEnd(10)} o=(${f3(f.o)}) u=(${f3(f.u)}) w=(${f3(f.w)})`);
const rot = (q: number[], v: V3): V3 => {
  const [x, y, z, w] = q as [number, number, number, number];
  const t: V3 = [2 * (y * v[2] - z * v[1]), 2 * (z * v[0] - x * v[2]), 2 * (x * v[1] - y * v[0])];
  return [v[0] + w * t[0] + (y * t[2] - z * t[1]), v[1] + w * t[1] + (z * t[0] - x * t[2]), v[2] + w * t[2] + (x * t[1] - y * t[0])];
};
const want = new Set(process.argv.slice(2));
const list = process.argv.includes("--structure") ? "structure" : "body";
for (const s of asset.volumes["50"][list] as Json[]) {
  if (want.size > 1 && !want.has(s.id) && !want.has(s.bone)) continue;
  if (s.kind === "loft") {
    console.log(`${s.id} @${s.bone} loft ${s.material} tone ${s.tone}`);
    for (const q of s.sections as Json[]) {
      const c = kit.W(s.bone, q.at);
      const ex = kit.W(s.bone, [q.at[0] + rot(q.rotation ?? [0, 0, 0, 1], [1, 0, 0])[0], q.at[1] + rot(q.rotation ?? [0, 0, 0, 1], [1, 0, 0])[1], q.at[2] + rot(q.rotation ?? [0, 0, 0, 1], [1, 0, 0])[2]]).map((x, i) => x - c[i]!);
      console.log(`   c=(${f3(c)}) rx=${q.rx} ry=${q.ry} e=${q.exponent ?? 2} ex=(${ex.map((x) => x.toFixed(2)).join(",")})`);
    }
  } else console.log(`${s.id} @${s.bone} ${s.kind} ${JSON.stringify({ ...s, id: undefined, bone: undefined, kind: undefined, title: undefined }).slice(0, 260)}`);
}
