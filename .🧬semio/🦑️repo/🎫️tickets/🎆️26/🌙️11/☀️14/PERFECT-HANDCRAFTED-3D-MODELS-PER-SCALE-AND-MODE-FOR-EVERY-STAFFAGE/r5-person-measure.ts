/** 📐 Measures a standing human figure of the catalog (volumes source merged) against the anthropometric shares of stature (Winter 2009, DIN 33402, Drillis and Contini 1966): joint heights from the rig in the default pose, bone lengths, and breadths, depths and lengths of the body solids sliced at the anatomical levels, each with the reference value and the deviation in percent.
 *
 * Usage: `bun r5-person-measure.ts <category/slug> [--scale N] [--stature M] [--sole M] [--arm upper-arm|arm] [--sex f|m] [--cloth M] [--hem M] [--calf M] [--json]`
 * @see ./r5-brief.md
 * @see ./r3-lib.ts */
import { cli, load, meshesIn, rigAt } from "./r3-lib.ts";
import { resolveList, type Json, type SolidMesh } from "./volumes-lib.ts";

const { spec, flag, has } = cli(["--scale", "--stature", "--sole", "--arm", "--sex", "--cloth", "--hem", "--calf"]);
const { asset } = load(spec, "bun r5-person-measure.ts <category/slug> [--scale N] [--stature M] [--sole M] [--arm upper-arm|arm]");
const scale = Number(flag("--scale") ?? 50);
const statureH = Number(flag("--stature") ?? 1.7);
const sole = Number(flag("--sole") ?? 0.02);
const armBone = flag("--arm") ?? "upper-arm";
const female = (flag("--sex") ?? "f") === "f";
const cloth = Number(flag("--cloth") ?? 0.02);
const rig = rigAt(asset, scale, { kind: "pose", pose: asset.presentation.defaultPose });
const F = rig.frames;
const prop = /^(umbrella|rib-|stretcher-|runner|cane|bag)/;
const solids = resolveList(asset.volumes[String(scale)], "body").filter((s: Json) => !prop.test(s.bone) && !/^bag/.test(s.id));
const meshes = meshesIn(asset, F, solids, scale);
const byBone = (re: RegExp): SolidMesh[] => meshes.filter((m) => re.test(solids.find((s: Json) => s.id === m.id)!.bone));
const verts = function* (ms: readonly SolidMesh[]): Generator<[number, number, number]> { for (const m of ms) for (let i = 0; i < m.world.length; i += 3) yield [m.world[i]!, m.world[i + 1]!, m.world[i + 2]!]; };
function slab(ms: readonly SolidMesh[], y0: number, y1: number): { x: [number, number]; z: [number, number] } {
  let [x0, x1, z0, z1] = [Infinity, -Infinity, Infinity, -Infinity];
  const take = (x: number, z: number): void => { x0 = Math.min(x0, x); x1 = Math.max(x1, x); z0 = Math.min(z0, z); z1 = Math.max(z1, z); };
  for (const y of [y0, (y0 + y1) / 2, y1]) for (const m of ms) {
    const w = m.world;
    for (let t = 0; t < m.indices.length; t += 3) {
      const ids = [m.indices[t]!, m.indices[t + 1]!, m.indices[t + 2]!];
      for (let e = 0; e < 3; e++) {
        const a = ids[e]! * 3, b = ids[(e + 1) % 3]! * 3;
        const [ya, yb] = [w[a + 1]!, w[b + 1]!];
        if ((ya - y) * (yb - y) > 0 || ya === yb) continue;
        const k = (y - ya) / (yb - ya);
        take(w[a]! + (w[b]! - w[a]!) * k, w[a + 2]! + (w[b + 2]! - w[a + 2]!) * k);
      }
    }
  }
  return { x: [x0, x1], z: [z0, z1] };
}
function extent(ms: readonly SolidMesh[], axis: [number, number, number], origin: [number, number, number]): [number, number] {
  let [a, b] = [Infinity, -Infinity];
  for (const p of verts(ms)) { const t = (p[0] - origin[0]) * axis[0] + (p[1] - origin[1]) * axis[1] + (p[2] - origin[2]) * axis[2]; a = Math.min(a, t); b = Math.max(b, t); }
  return [a, b];
}
const yOf = (bone: string): number => F[bone]!.o[1];
const len = (bone: string): number => (asset.rig.bones as Json[]).find((b) => b.id === bone)?.lengthM ?? NaN;
const all = [...verts(meshes)];
const top = Math.max(...all.map((p) => p[1]));
const H = statureH;
const rows: [string, number, number][] = [];
const row = (name: string, model: number, ref: number): void => { rows.push([name, model, ref]); };
row("stature, shod (top of skull/hair)", top, H + sole);
row("hip joint height", yOf("thigh-l"), 0.53 * H + sole);
row("knee joint height", yOf("shank-l"), 0.285 * H + sole);
row("ankle joint height", yOf("foot-l"), 0.039 * H + sole);
row("shoulder joint height", yOf(`${armBone}-l`), 0.818 * H + sole);
row("elbow height (hanging arm)", yOf("forearm-l"), 0.63 * H + sole);
row("wrist height (hanging arm)", yOf("hand-l"), 0.485 * H + sole);
row("thigh length (hip-knee)", len("thigh-l"), 0.245 * H);
row("shank length (knee-ankle)", len("shank-l"), 0.246 * H);
row("upper arm length", len(`${armBone}-l`), 0.186 * H);
row("forearm length", len("forearm-l"), 0.146 * H);
const head = byBone(/^head$/).filter((m) => !/hair|brow|lens|temple|glasses|collar/.test(m.id));
const headY = extent(head, [0, 1, 0], [0, 0, 0]);
const skull = byBone(/^head$/).filter((m) => m.id === "head");
const skullY = extent(skull, [0, 1, 0], [0, 0, 0]);
row("head height chin-vertex", skullY[1] - skullY[0], H / 7.5);
const hl = F["hand-l"]!;
const handL = byBone(/^hand-l$/);
const he = extent(handL, hl.u as never, hl.o as never);
row("hand reach of the relaxed hanging hand (wrist-fingertip, fingers flexed 10 to 30 degrees: 0.95 x 0.108 H)", he[1], 0.95 * 0.108 * H);
row("hand length of the rig (wrist to fingertip, 0.108 H)", len("hand-l"), 0.108 * H);
const sh = slab(meshes.filter((m) => !/hand|head|hair|neck|collar/.test(m.id)), yOf(`${armBone}-l`) - 0.08, yOf(`${armBone}-l`) - 0.02);
row("shoulder breadth over deltoids (bideltoid, clothed)", sh.z[1] - sh.z[0], (female ? 0.265 : 0.28) * H + 2 * cloth);
row("biacromial breadth (shoulder joint spacing + 2 x 25 mm)", Math.abs(F[`${armBone}-l`]!.o[2] - F[`${armBone}-r`]!.o[2]) + 0.05, 0.23 * H);
const hip = slab(meshes.filter((m) => !/hand|forearm|sleeve|cuff|wrist|arm/.test(m.id)), yOf("thigh-l") - 0.06, yOf("thigh-l") - 0.02);
row("hip breadth at buttocks (clothed)", hip.z[1] - hip.z[0], (female ? 0.215 : 0.2) * H + 2 * cloth);
const chest = slab(meshes.filter((m) => !/hand|forearm|sleeve|cuff|wrist|arm|strap|lapel|pocket|button/.test(m.id)), 0.72 * H + sole - 0.01, 0.72 * H + sole + 0.01);
row("chest depth (clothed)", chest.x[1] - chest.x[0], (female ? 0.15 : 0.14) * H + 2 * cloth);
const foot = byBone(/^(foot|toe)-l$/);
const fp = [...verts(foot)];
const fx = { x: [Math.min(...fp.map((q) => q[0])), Math.max(...fp.map((q) => q[0]))], z: [Math.min(...fp.map((q) => q[2])), Math.max(...fp.map((q) => q[2]))] };
row("shoe length (foot 0.152 H + 18 mm toe allowance and welt)", fx.x[1] - fx.x[0], 0.152 * H + 0.018);
row("shoe width", fx.z[1] - fx.z[0], 0.055 * H + 0.008);
const thighMid = yOf("thigh-l") - 0.5 * len("thigh-l");
const tm = slab(byBone(/^thigh-l$/), thighMid - 0.01, thighMid + 0.01);
row("thigh width at mid-thigh (clothed)", tm.z[1] - tm.z[0], 0.092 * H + 0.01);
const calfY = yOf("shank-l") - 0.3 * len("shank-l");
const cm = slab(byBone(/^shank-l$/), calfY - 0.01, calfY + 0.01);
row(flag("--calf") ? "trouser leg depth at the calf (straight cut)" : "calf depth at the calf (clothed)", cm.x[1] - cm.x[0], Number(flag("--calf") ?? 0.064 * H + 0.008));
const ankleY = yOf("foot-l") + 0.06;
const am = slab(byBone(/^shank-l$/), ankleY - 0.005, ankleY + 0.005);
row(flag("--hem") ? "leg width above the ankle (trouser hem or boot shaft)" : "leg width above the ankle (clothed)", am.z[1] - am.z[0], Number(flag("--hem") ?? 0.04 * H + 0.008));
const neckY = yOf(`${armBone}-l`) + 0.07;
const nm = slab(meshes.filter((m) => /^neck/.test(m.id)), neckY - 0.01, neckY + 0.01);
row("neck breadth", nm.z[1] - nm.z[0], 0.065 * H);
if (has("--json")) console.log(JSON.stringify(rows));
else {
  console.log(`| quantity | model m | reference m | deviation |`);
  console.log(`| --- | --- | --- | --- |`);
  for (const [n, m, r] of rows) console.log(`| ${n} | ${m.toFixed(3)} | ${r.toFixed(3)} | ${(((m - r) / r) * 100).toFixed(1)} % |`);
}
