/** 🏗️ Builds the structure list of one scale from its body: member solids (B1) with real sections kept inside the body, joints, supports with `anchor` (C6, B3) and masses.
 *
 * Usage: `bun kit-structure.ts <category>/<slug> --scale N [--core 0.7] [--no-joints] [--write] [--source file]`
 * - members: for every `memberBones` bone (the 2D structural view's eligible bones plus bones of 2D `member` shapes) the core of that bone's primary body solid (a loft sampled at 4/3/2 sections for 1:50/100/200+, other kinds scaled) with factor `--core` (default 0.7; torso/pelvis 0.72, head 0.8, hands/feet 0.6), or, for bones without a body solid or with a declared bone `section`, the section from the bone (tube from area and second moment, else estimated from mass); 2D `member` shapes become bars (capsules) with their drawn section. Every member is then checked against the union of the body in all three views and shrunk by 15 % up to 6 times while more than 2 % of its projection lies outside.
 * - joints: ellipsoids at the rig joints at 1:50 and 1:100 only (merged into members at 1:200 and 1:500, which also makes the structure count drop as `volumes-monotonic` wants).
 * - supports: one solid per `structure.supports` anchor with `part: "support"` and `anchor: <id>`, shaped by `SupportType`: contact = flat pad of the anchor's footprint, pin = cylinder across w, roller = cylinder along w with a flat base, fixed = block; sized by at least the smallest feature, placed at the anchor.
 * - masses: compact ellipsoids at the centre of mass of bones holding at least 10 % of the total mass.
 * `--write` saves `structure` of this scale. Review the render sheet afterwards and hand-correct: sections, joints and masses here are a generated first pass.
 * @see ./kit.md */
import { memberBones, memberShapes, installCancellation } from "./volumes-lib.ts";
import { draftStructure } from "./volumes-draft.ts";
import { Fitter } from "./kit-fitter.ts";
import { VIEWS, boxOf, capsuleOf, clone, flag, has, loftOf, openCtx, positional, saveLists, scaledSolid, superellipseSection, writeOut, type Json } from "./kit-lib.ts";

const args = process.argv.slice(2);
const spec = positional(args, ["--scale", "--core", "--source"]);
const scale = Number(flag(args, "--scale"));
if (spec === undefined || ![50, 100, 200, 500].includes(scale)) {
  process.stderr.write("usage: kit-structure.ts <category>/<slug> --scale N [--core 0.7] [--write]\n");
  process.exit(2);
}
installCancellation();
const c = openCtx(spec, scale, flag(args, "--source"));
const body: Json[] = c.vol.body;
if (body.length === 0) {
  process.stderr.write(`1:${scale} has no body; run kit-fit.ts first\n`);
  process.exit(2);
}
const f = c.feature;
const draft = draftStructure(c.raw, scale);
const notes: string[] = draft.flags.filter((x) => x.code !== "support-type").map((x) => `${x.code}: ${x.subject} ${x.message}`);
const CORE: Record<string, number> = { torso: 0.72, pelvis: 0.72, head: 0.8, hand: 0.6, foot: 0.6, toe: 0.6 };
const coreK = (bone: string): number => (flag(args, "--core") === undefined ? Object.entries(CORE).find(([k]) => bone.startsWith(k))?.[1] ?? 0.7 : Number(flag(args, "--core")));
const bones: Json[] = c.m.rig.bones;
const boneOf = (id: string): Json => bones.find((b) => b.id === id)!;

function primary(bone: string): Json | undefined {
  const mine = body.filter((s) => s.bone === bone && (s.role ?? "body") === "body" && s.slender !== true);
  const lofts = mine.filter((s) => s.kind === "loft");
  if (lofts.length > 0) return lofts.reduce((a, b) => (span(b) > span(a) ? b : a));
  return mine[0];
}
const span = (s: Json): number => Math.hypot(...[0, 1, 2].map((k) => s.sections[s.sections.length - 1].at[k] - s.sections[0].at[k]));

function pick(sections: Json[], n: number): Json[] {
  if (sections.length <= n) return sections;
  return Array.from({ length: n }, (_, i) => sections[Math.round((i * (sections.length - 1)) / (n - 1))]!);
}

function member(bone: string): Json | null {
  const p = primary(bone);
  if (p === undefined) return null;
  const k = coreK(bone);
  const id = `member-${bone}`;
  let s: Json;
  if (p.kind === "loft") {
    s = clone(p);
    s.sections = pick(p.sections, scale === 50 ? 4 : scale === 100 ? 3 : 2).map((q) => ({ ...clone(q), rx: Math.max(q.rx * k, 0.55 * f), ry: Math.max(q.ry * k, 0.55 * f), exponent: Math.min(q.exponent ?? 2, 3) }));
    s.id = id;
  } else if (scale >= 200 && p.kind === "ellipsoid") s = boxOf({ id, bone, material: p.material }, p.transform?.translation ?? [0, 0, 0], p.radii.map((r: number) => Math.max(2 * r * k, 1.05 * f)) as never);
  else if (scale >= 200 && p.kind === "capsule") s = loftOf({ id, bone, material: p.material }, [superellipseSection(p.p0, Math.max(p.r0 * k, 0.55 * f), Math.max(p.r0 * k, 0.55 * f)), superellipseSection(p.p1, Math.max(p.r1 * k, 0.55 * f), Math.max(p.r1 * k, 0.55 * f))]);
  else s = scaledSolid(p, k, id, 0.55 * f);
  delete s.region;
  delete s.layer;
  delete s.tags;
  return { ...s, role: "body", tone: 0.5, member: bone, part: "member" };
}

const fit = new Fitter(c, 900);
for (const s of body) if (s.slender !== true && s.texture !== true) fit.add(fit.cellsOf(fit.mesh(s)), 1);
function outsideShare(s: Json): number {
  const cells = fit.cellsOf(fit.mesh(s));
  let out = 0;
  let all = 0;
  for (const v of VIEWS) {
    const g = fit.views.get(v)!;
    for (const i of cells.get(v)!) {
      all++;
      if (g.count[i]! === 0) out++;
    }
  }
  return all === 0 ? 0 : out / all;
}
function inside(s: Json): Json {
  let cur = s;
  for (let i = 0; i < 10 && outsideShare(cur) > 0.003; i++) cur = scaledSolid(cur, 0.88, s.id, 0.55 * f);
  return cur;
}

const out: Json[] = [];
const have = new Set<string>();
for (const b of memberBones(c.m, scale)) {
  const bone = boneOf(b);
  let s = bone?.section === undefined ? member(b) : null;
  if (s === null) s = draft.solids.find((x) => x.id === `member-${b}`) ?? null;
  if (s === null) continue;
  out.push(inside(s));
  have.add(b);
}
for (const s of draft.solids.filter((x) => x.id.startsWith("bar-"))) out.push(inside(s));
void memberShapes;
if (scale <= 100 && !has(args, "--no-joints"))
  for (const s of draft.solids.filter((x) => x.part === "joint" && have.has(x.member))) out.push(inside(s));
const total = bones.reduce((n, b) => n + (b.mass?.kg ?? 0), 0);
for (const b of bones) {
  const kg = b.mass?.kg ?? 0;
  if (total === 0 || kg / total < 0.1) continue;
  const m = draft.solids.find((x) => x.id === `mass-${b.id}`);
  if (m !== undefined) out.push(inside(scale >= 200 ? { ...boxOf({ id: m.id, bone: m.bone, material: m.material, tone: m.tone }, m.transform?.translation ?? [0, 0, 0], m.radii.map((r: number) => 2 * r) as never), role: "detail", member: m.member, part: "mass" } : m));
}
for (const id of c.m.structure?.supports ?? []) {
  const a = c.m.anchors.find((x: Json) => x.id === id);
  if (a === undefined) continue;
  const L = Math.max(f, a.footprintLengthM ?? 0, 0.02);
  const W = Math.max(f, a.footprintWidthM ?? 0, 0.02);
  const t = Math.max(f, 0.02);
  const common = { id: `support-${id}`, bone: a.bone, material: boneOf(a.bone)?.section?.material ?? c.m.materials[0].id, tone: 0.25 };
  const at: [number, number, number] = [a.x, a.y, a.z ?? 0];
  let s: Json;
  if (a.support === "pin") s = { ...capsuleOf(common, [at[0], at[1], at[2] - W / 2], [at[0], at[1], at[2] + W / 2], Math.max(0.55 * f, Math.min(L, W, 0.04) / 2)) };
  else if (a.support === "roller") s = { ...capsuleOf(common, [at[0], at[1] + 0.55 * f, at[2] - W / 2], [at[0], at[1] + 0.55 * f, at[2] + W / 2], Math.max(0.55 * f, 0.5 * t)) };
  else if (a.support === "fixed") s = boxOf(common, [at[0], at[1], at[2]], [Math.max(L, 1.5 * f), Math.max(t, 1.5 * f), Math.max(W, 1.5 * f)]);
  else s = boxOf(common, [at[0], at[1] + t / 2, at[2]], [L, t, W]);
  out.push({ ...s, role: "detail", member: a.bone, part: "support", anchor: id });
}
const lines = [`kit-structure 1:${scale} ${spec}: ${out.length} structure solids (${out.filter((s) => s.part === "member").length} members, ${out.filter((s) => s.part === "joint").length} joints, ${out.filter((s) => s.part === "support").length} supports, ${out.filter((s) => s.part === "mass").length} masses)`];
const missing = memberBones(c.m, scale).filter((b) => !have.has(b));
if (missing.length > 0) lines.push(`member bones with no member solid: ${missing.join(", ")}`);
for (const s of out.filter((x) => x.part !== "support")) {
  const sh = outsideShare(s);
  if (sh > 0.02) lines.push(`  ${s.id}: ${(sh * 100).toFixed(1)} % of its projection lies outside the body: move or shrink it by hand`);
}
lines.push(...notes.slice(0, 12).map((n) => `note: ${n}`));
process.stdout.write(lines.join("\n") + "\n");
writeOut(`structure/${spec.replace("/", "-")}.s${scale}.json`, JSON.stringify(out, null, 1));
if (has(args, "--write")) {
  saveLists(c, { structure: out });
  process.stdout.write(`written structure of 1:${scale} to ${c.file}\n`);
}
