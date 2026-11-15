import { readFileSync } from "node:fs";
import { loftOf, openCtx, saveLists, superellipseSection, featureM, type Json } from "./kit-lib.ts";

const scale = Number(process.argv[2]);
const c = openCtx("vegetation/bamboo-clump", scale);
const F = featureM(scale);
const body: Json[] = c.vol.body;
const tubes500: Json[] = scale === 500 ? JSON.parse(readFileSync("🗑️generated/author-54/bamboo.s200.culms.json", "utf8")) : [];
const culms = scale === 500 ? tubes500 : body.filter((s) => /^culm-/.test(s.id) || s.id === "stems");
const lobes = scale === 500 ? body : body.filter((s) => !culms.includes(s));
const oldT: Json[] = c.vol.thermal;
const oldS: Json[] = c.vol.structure;
const emitters = oldT.filter((s) => s.tags?.includes("emitter"));
const supports = oldS.filter((s) => s.part === "support");
const frames = c.state.frames;

const thermal: Json[] = [];
for (const s of culms) {
  const own = JSON.parse(JSON.stringify(s));
  if (s.profile && scale <= 100) {
    const L = Math.max(...s.profile.map((p: number[]) => p[0]));
    const rs = s.profile.filter((p: number[]) => p[0] < 1e-6).map((p: number[]) => p[1]);
    const re = s.profile.filter((p: number[]) => p[0] > L - 1e-6).map((p: number[]) => p[1]);
    own.profile = [[0, 0], [0, Math.max(...rs)], [L, Math.max(...re)], [L, 0]];
  }
  thermal.push({ ...own, id: `th-${s.id}`, region: "culms", slender: s.slender ?? true });
}
const ys = lobes.map((l) => l.transform?.translation?.[1] ?? 0).sort((a, b) => a - b);
const cut = ys[Math.floor(ys.length * 0.45)] ?? 0;
lobes.forEach((l, i) => thermal.push({ id: `th-${l.id}`, of: l.id, region: (l.transform?.translation?.[1] ?? 0) >= cut && i % 3 !== 2 ? "leaves-sun" : "leaves-shade" }));
if (!thermal.some((t) => t.region === "leaves-sun")) thermal[thermal.length - 1].region = "leaves-sun";
if (!thermal.some((t) => t.region === "leaves-shade")) thermal[thermal.length - 2].region = "leaves-shade";
thermal.push(...emitters);

const structure: Json[] = [];
if (scale === 500) {
  const tubes: Json[] = JSON.parse(readFileSync("🗑️generated/author-54/bamboo.s200.culms.json", "utf8"));
  for (const t of tubes) {
    const L = Math.max(...t.profile.map((p: number[]) => p[0]));
    const re = Math.max(...t.profile.filter((p: number[]) => p[0] > L - 1e-6).map((p: number[]) => p[1]));
    const rs = Math.max(...t.profile.filter((p: number[]) => p[0] < 1e-6).map((p: number[]) => p[1]));
    const z0 = t.transform?.translation?.[2] ?? 0;
    const qy = t.transform?.rotation?.[1] ?? 0;
    const qw = t.transform?.rotation?.[3] ?? 1;
    const phi = 2 * Math.atan2(qy, qw);
    const rm = (rs + re) / 2;
    structure.push({ id: `member-${t.id}`, kind: "box", bone: t.bone, material: "bamboo-culm", role: "body", tone: 0.5, slender: true, size: [L, 2 * rm, 2 * rm], transform: { translation: [(L / 2) * Math.cos(phi), 0, z0 - (L / 2) * Math.sin(phi)].map((x) => Math.round(x * 1e4) / 1e4), rotation: [0, qy, 0, qw] }, member: t.id, part: "member" });
  }
} else for (const s of culms) structure.push({ id: `member-${s.id}`, of: s.id, member: s.id, part: "member" });
const r = Math.max(F * 0.55, 0.0045);
const r2 = Math.max(F * 0.52, r);
const zc = culms.find((x) => x.id === "culm-3-a")?.transform?.translation?.[2] ?? 0;
for (const fb of c.raw.rig.bones.filter((b: Json) => /^fol-/.test(b.id))) {
  const L = Math.min(fb.lengthM * 0.6, 0.6);
  structure.push({ ...loftOf({ id: `member-${fb.id}`, bone: fb.id, material: "bamboo-leaf", tone: 0.5 }, [superellipseSection([0.02, 0, 0], r, r, 2), superellipseSection([L, 0, 0], r, r, 2)]), member: fb.id, part: "member" });
}
structure.push({ ...loftOf({ id: "member-root-plate", bone: "root-plate", material: "bamboo-culm", tone: 0.5 }, [superellipseSection([0.02 - Math.max(0.008, F * 0.6), 0.05, zc], r2, r2, 2), superellipseSection([0.02 + Math.max(0.008, F * 0.6), 0.05, zc], r2, r2, 2)]), member: "root-plate", part: "member" });
structure.push(...supports);
void frames;
saveLists(c, { thermal, structure });
console.log(thermal.length, "thermal", structure.length, "structure");
