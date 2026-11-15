import { loftOf, openCtx, saveLists, superellipseSection, featureM, type Json, type Section } from "./kit-lib.ts";
import { vsub, vlen, vnorm, vdot, type V } from "./author-54-lib.ts";

const c = openCtx("vegetation/agave", 500);
const F = featureM(500);
const body: Json[] = c.vol.body;
const blades = body.filter((s) => /^blade-/.test(s.id));
const frames = c.state.frames;
const old: Json[] = c.vol.structure;
const toWorld = (p: number[]): V => [-p[1]!, p[0]!, p[2]!];
const structure: Json[] = [];
const tipDir = (b: Json): V => vnorm(vsub(toWorld(b.sections[b.sections.length - 1].at), toWorld(b.sections[0].at)));
const k = 0.5;
const core = (b: Json, i0: number, i1: number, id: string, member: string): Json => {
  const secs: Section[] = [b.sections[i0], b.sections[i1]].map((s: Json) => ({ ...superellipseSection(s.at, Math.max(s.rx * k, F * 0.55), Math.max(s.ry * k, F * 0.55), s.exponent ?? 2, s.rotation) }));
  return { ...loftOf({ id, bone: "caudex", material: "agave-leaf", tone: 0.4 }, secs), member, part: "member" };
};
for (const bone of c.raw.rig.bones) {
  if (bone.id === "root-plate") continue;
  if (bone.id === "caudex") { const cm = old.find((s) => s.id === "member-caudex")!; structure.push({ ...cm, sections: [cm.sections[0], cm.sections[cm.sections.length - 1]] }); continue; }
  if (bone.id === "leaf-s") { const sp = body.find((s) => s.id === "leaf-s")!; structure.push({ ...loftOf({ id: "member-leaf-s", bone: "leaf-s", material: "agave-leaf", tone: 0.4 }, [0, 6].map((i) => { const s = sp.sections[i]; return superellipseSection(s.at, s.rx * 0.5, s.ry * 0.5, 2.1); })), member: "leaf-s", part: "member" }); continue; }
  const f = frames[bone.id]!;
  const dir = vnorm([f.u[0], f.u[1], f.u[2]] as V);
  const host = blades.reduce((a, b) => (vdot(tipDir(b), dir) > vdot(tipDir(a), dir) ? b : a));
  const n = host.sections.length;
  const isA = /-a$/.test(bone.id);
  const isB = /-b$/.test(bone.id);
  const [i0, i1] = isA ? [0, Math.max(1, Math.floor((n - 1) / 2))] : isB ? [Math.max(0, Math.floor((n - 1) / 2)), n - 1] : [0, n - 1];
  structure.push(core(host, i0, i1, `member-${bone.id}`, bone.id));
}
const rp = old.find((s) => s.id === "member-root-plate")!;
structure.push({ ...rp, sections: [rp.sections[0], rp.sections[rp.sections.length - 1]] });
structure.push(...old.filter((s) => s.part === "support"));
void vlen;
saveLists(c, { structure });
console.log(structure.length, "structure solids");
