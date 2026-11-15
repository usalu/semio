/** 🦔️ Author 41: handcrafted hedgehog bodies per scale (spine dome and four spine skirts as lofts from the drawings, belly, neck, head with nose, stub legs on their real bones). Usage: bun author-41-hedgehog.ts <50|100> [--write] */
import { Zoo, range, type Look } from "./author-41-lib.ts";
const scale = Number(process.argv[2]);
const write = process.argv.includes("--write");
const S = new Zoo("animals/hedgehog", scale);
const fine = scale === 50;
const SP = (tone: number): Look => ({ material: "spines", tone });
const FU = (tone: number): Look => ({ material: "belly-fur", tone });
const FA = (tone: number): Look => ({ material: "face-fur", tone });
const SK: Look = { material: "bare-skin", tone: 0.2 };
const k = (a: number, b: number) => (fine ? a : b);

S.hullX("dome", "trunk", { elev: ["dome"], plan: ["dome"], xs: range(-0.034, 0.157, k(12, 8)), look: SP(0.4), e: 2.2 });
if (fine) S.hullX("belly", "trunk", { elev: ["belly"], xs: range(0.026, 0.146, 7), look: FU(0.8), e: 2.2, zc: 0, half: () => 0.058, floor: 0.0075 });
for (const [bone, id, ids, xs] of [["coat-rn", "skirt-rear-near", "near", [-0.056, 0.002]], ["coat-rf", "skirt-rear-far", "far", [-0.056, 0.002]], ["coat-fn", "skirt-front-near", "near", [0.118, 0.177]], ["coat-ff", "skirt-front-far", "far", [0.118, 0.177]]] as const) {
  const eid = id.replace("far", "near");
  S.hullX(id, bone, { elev: [eid], plan: [id], xs: range(xs[0], xs[1], k(8, 5)), look: SP(0.3), e: 2.2 });
  void ids;
}
S.limb("neck", "neck", { elev: ["neck"], plan: ["neck"], n: k(5, 4), look: FA(0.7), e: 2.2 });
S.limb("head", "head", { elev: ["head"], plan: ["head"], n: k(7, 5), look: FA(0.7), e: 2.2 });
const hr = S.rings("elevation", ["head"])[0]!;
const tip = hr.reduce((a, b) => (b[0] > a[0] ? b : a));
S.ell("nose", "head", [tip[0] - 0.003, tip[1], 0], fine ? [0.0125, 0.0105, 0.0105] : [0.0125, 0.0125, 0.0125], SK);
const legs: [string, string, string, number][] = [["fore-n", "near-fore", "n", 0], ["forepaw-n", "near-forepaw", "n", 0], ["hind-n", "near-hind", "n", 0], ["hindpaw-n", "near-hindpaw", "n", 0], ["fore-f", "far-fore", "f", 0], ["forepaw-f", "far-forepaw", "f", 0], ["hind-f", "far-hind", "f", 0], ["hindpaw-f", "far-hindpaw", "f", 0]];
const rig: any[] = (S.c.m as any).rig.bones;
for (const [bone, pid, , ] of legs) {
  if (!fine && bone.includes("paw")) continue;
  const f = S.frame(bone);
  const L = rig.find((b) => b.id === bone).lengthM as number;
  const a: [number, number, number] = [f.o[0]!, f.o[1]!, 0];
  const e: [number, number, number] = [f.o[0]! + f.u[0]! * L, f.o[1]! + f.u[1]! * L, 0];
  const sp = S.span("plan", [pid], "x", (a[0] + e[0]) / 2) ?? S.span("plan", [pid], "x", a[0])!;
  const z = (sp[0] + sp[1]) / 2;
  const G = 0.0097;
  const XL = 0.034;
  a[0] = Math.max(a[0], XL);
  e[0] = Math.max(e[0], XL);
  const rmax = Math.min(0.016, (sp[1] - sp[0]) / 2);
  const ry = (y: number) => Math.max(0.55 * S.feature, Math.min(rmax, y - G));
  if (bone.includes("paw")) {
    const L2 = Math.hypot(e[0] - a[0], e[1] - a[1]) / 2;
    S.ell(bone, bone, [(a[0] + e[0]) / 2, G + 0.55 * S.feature, z], [L2 + 0.004, 0.55 * S.feature, Math.max(0.55 * S.feature, Math.min(rmax, 0.016))], SK, [[1, 0, 0], [0, 1, 0]]);
    continue;
  }
  a[1] = Math.max(a[1], G + 0.55 * S.feature);
  e[1] = Math.max(e[1], G + 0.55 * S.feature);
  S.cap(bone, bone, [a[0], a[1], z], [e[0], e[1], z], ry(a[1]), ry(e[1]), FU(0.6));
}
if (process.argv.includes("--extra") && scale === 100) {
  S.ell("t-belly", "trunk", [0.085, 0.02, 0], [0.055, 0.0078, 0.058], FU(0.8));
  const ids: string[] = [];
  for (const [id, bone, x, z] of [["t-paw-fn", "forepaw-n", 0.112, -0.035], ["t-paw-hn", "hindpaw-n", 0.05, -0.042], ["t-paw-ff", "forepaw-f", 0.09, 0.035], ["t-paw-hf", "hindpaw-f", 0.04, 0.042]] as const) { S.ell(id, bone, [x, 0.0175, z], [0.0155, 0.0078, 0.0155], SK); ids.push(id); }
  S.takeThermal({ "belly-fur": ["t-belly"], "paws-skin": ids });
  process.exit(0);
}
console.log(`hedgehog 1:${scale}: ${S.body.length} body solids`);
if (write) S.save();
