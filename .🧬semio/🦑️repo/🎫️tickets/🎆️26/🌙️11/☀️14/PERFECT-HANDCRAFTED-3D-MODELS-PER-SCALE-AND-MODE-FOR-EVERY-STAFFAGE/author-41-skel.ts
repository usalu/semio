/** 🦴️ Author 41: replaces generated structure members and joints of an animal by bone shafts and ball joints placed on the real bones of the default pose (lateral offset from the plan drawing), sized by hand from a spec.
 * Usage: bun author-41-skel.ts <asset> <scale> <spec.json> [--write]
 * spec = { "<scale>": { "members": { "<bone>": [r0, r1] }, "joints": { "<bone>": r }, "keep": ["<member bone>"...] } } — members not listed and not kept are dropped, joints not listed are dropped.
 * @see ./author-41.md */
import { readFileSync } from "node:fs";
import { Zoo } from "./author-41-lib.ts";
import { drawn, saveLists, VIEWS, type Json } from "./kit-lib.ts";
import { Fitter } from "./kit-fitter.ts";

const [asset, scaleS, specFile] = process.argv.slice(2);
const scale = Number(scaleS);
const spec = JSON.parse(readFileSync(specFile!, "utf8"))[scaleS!];
const S = new Zoo(asset!, scale);
const bones: Json[] = (S.c.m as any).rig.bones;
const old: Json[] = S.c.vol.structure as Json[];
const keep = new Set<string>(spec.keep ?? []);
const out: Json[] = old.filter((s) => !(s.part === "member" && !keep.has(s.member)) && s.part !== "joint");
const look = { material: "bone", tone: 0.85, role: "body" };
const laterals = (bone: string, xa: number, xb: number): number => {
  const ids = drawn(S.c, "plan", bone).map((s) => s.id);
  if (ids.length === 0) return 0;
  const sp = S.span("plan", ids, "x", (xa + xb) / 2);
  return sp === null ? 0 : (sp[0] + sp[1]) / 2;
};
const joinEnds: Record<string, [number, number, number][]> = {};
const mat = (s: Json): Json => ({ ...s, material: spec.material ?? "short-fur", tone: s.tone });
for (const [bone, [r0, r1, t0 = 0, t1 = 1]] of Object.entries(spec.members as Record<string, number[]>)) {
  const b = bones.find((q) => q.id === bone)!;
  const f = S.frame(bone);
  const L = b.lengthM as number;
  const z = laterals(bone, f.o[0]!, f.o[0]! + f.u[0]! * L);
  const toWorld = (q: number[]): [number, number, number] => [f.o[0]! + q[0]! * f.u[0]! + q[1]! * f.v[0]! + q[2]! * f.w[0]!, f.o[1]! + q[0]! * f.u[1]! + q[1]! * f.v[1]! + q[2]! * f.w[1]!, f.o[2]! + q[0]! * f.u[2]! + q[1]! * f.v[2]! + q[2]! * f.w[2]!];
  const bs = (S.c.vol.body as Json[]).find((q) => q.id === bone);
  const fol: [number, number, number][] | undefined = (spec.follow ?? []).includes(bone) && bs !== undefined ? (bs.kind === "capsule" ? [toWorld(bs.p0), toWorld(bs.p1)] : bs.kind === "ellipsoid" ? (() => { const c = toWorld(bs.transform.translation); return [[c[0] - bs.radii[0] * 0.7, c[1], c[2]], [c[0] + bs.radii[0] * 0.7, c[1], c[2]]] as [number, number, number][]; })() : undefined) : undefined;
  const ov = (spec.ends?.[bone] as [number, number, number][] | undefined) ?? fol;
  if (ov !== undefined) joinEnds[bone] = ov;
  const a: [number, number, number] = ov?.[0] ?? [f.o[0]! + f.u[0]! * L * t0, f.o[1]! + f.u[1]! * L * t0, z];
  const e: [number, number, number] = ov?.[1] ?? [f.o[0]! + f.u[0]! * L * t1, f.o[1]! + f.u[1]! * L * t1, z];
  S.body.length = 0;
  const c = (spec.box ?? []).includes(bone) ? S.add({ id: `member-${bone}`, kind: "box", bone, material: look.material, role: "body", tone: look.tone, size: [2 * r0, 2 * r0, Math.hypot(e[0] - a[0], e[1] - a[1], e[2] - a[2])], transform: { translation: S.local(bone, [(a[0] + e[0]) / 2, (a[1] + e[1]) / 2, (a[2] + e[2]) / 2]) } }) : spec.loft === true ? S.tube(`member-${bone}`, bone, a, e, r0, r1, look) : S.cap(`member-${bone}`, bone, a, e, r0, r1, look);
  out.push({ ...mat(c), member: bone, part: "member" });
}
for (const [bone, r] of Object.entries(spec.joints as Record<string, number>)) {
  const f = S.frame(bone);
  const z = laterals(bone, f.o[0]!, f.o[0]! + 0.01);
  S.body.length = 0;
  const jo = ((spec.ends?.[bone] as [number, number, number][] | undefined) ?? joinEnds[bone])?.[0];
  const j = S.ell(`joint-${bone}`, bone, jo ?? [f.o[0]!, f.o[1]!, z], [r, r, r], { material: spec.material ?? "short-fur", tone: 0.35, role: "detail" });
  out.push({ ...j, member: bone, part: "joint" });
}
console.log(`structure 1:${scale}: ${out.length} solids (${out.filter((s) => s.part === "member").length} members, ${out.filter((s) => s.part === "joint").length} joints, ${out.filter((s) => s.part === "mass").length} masses)`);
const fit = new Fitter(S.c, 900);
for (const b of S.c.vol.body as Json[]) if (b.slender !== true && b.texture !== true) fit.add(fit.cellsOf(fit.mesh(b)), 1);
for (const q of out) {
  const cells = fit.cellsOf(fit.mesh(q));
  let n = 0;
  let o = 0;
  for (const v of VIEWS) { const g = fit.views.get(v)!; for (const i of cells.get(v)!) { n++; if (g.count[i]! === 0) o++; } }
  if (n > 0 && o / n > 0.002) console.log(`  outside ${q.id}: ${((o / n) * 100).toFixed(1)} %`);
}
if (process.argv.includes("--write")) saveLists(S.c, { structure: out });
