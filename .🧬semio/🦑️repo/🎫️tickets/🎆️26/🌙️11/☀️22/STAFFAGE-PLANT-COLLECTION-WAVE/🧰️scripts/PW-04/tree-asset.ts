/** 🏗️ PW-04 asset assembly: rig, masses, representations, symbols, volumes, thermal, structure, poses, clips and the manifest row of one street tree from its {@link Species} tables.
 *
 * @see ./tree-lib.ts
 * @see ../../📋️brief.md */
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

import { FEATURE, G, RHO_AIR, SCALES, T, VIEWS, add, barkMarks, boxOf, de, elevLocal, frames, inRing, len, marksIn, mul, pathOf, pathShape, primNear, primRing, primsOf, project, r, ringArea, shapeOfPrim, solidOf, sub, toLocal, union, type Frame, type Json, type P2, type Prim, type Scale, type Species, type Text, type V3, type View } from "./tree-lib.ts";

export const ROOT = "/Users/ueli/Documents/semio";
export const CATALOG = join(ROOT, "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation");
export const TICKET = join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-PLANT-COLLECTION-WAVE");

const LIMIT: Record<0 | 1 | 2 | 3 | 4, number> = { 0: 3, 1: 6, 2: 9, 3: 12, 4: 14 };
const deg = (a: number): number => (a * 180) / Math.PI;
const norm180 = (a: number): number => ((((a + 180) % 360) + 360) % 360) - 180;

/** ⚖️ Volume and centroid (local u, v) of a frustum capsule core. */
function frustum(f: Frame, a: V3, b: V3, r0: number, r1: number): { v: number; c: V3 } {
  const L = len(sub(b, a));
  const v = (Math.PI * L * (r0 * r0 + r0 * r1 + r1 * r1)) / 3;
  const t = (r0 * r0 + 2 * r0 * r1 + 3 * r1 * r1) / (4 * (r0 * r0 + r0 * r1 + r1 * r1) || 1);
  return { v, c: toLocal(f, add(a, mul(sub(b, a), t))) };
}

/** 🦴️ The rig with segment masses and sections, the total mass and the level of every bone. */
export function rigOf(sp: Species, fr: Map<string, Frame>): { bones: Json[]; massKg: number; level: Map<string, number>; leafKg: Map<string, number> } {
  const level = new Map<string, number>([["root-plate", 0]]);
  const prims50 = primsOf(sp, 50);
  const woodKg = new Map<string, { kg: number; c: V3 }>();
  const addKg = (host: string, kg: number, c: V3) => {
    const o = woodKg.get(host) ?? { kg: 0, c: [0, 0, 0] as V3 };
    const k = o.kg + kg;
    woodKg.set(host, { kg: k, c: k > 0 ? (o.c.map((x, i) => (x * o.kg + c[i]! * kg) / k) as V3) : o.c });
  };
  for (const p of prims50) {
    if (p.kind !== "capsule") continue;
    const fz = frustum(fr.get(p.host)!, p.a, p.b, p.r0, p.r1);
    addKg(p.host, fz.v * sp.densityKgM3, fz.c);
  }
  const flare = sp.rootFlare.profile;
  let flareV = 0;
  for (let i = 1; i < flare.length; i++) {
    const [h0, a0] = flare[i - 1]!;
    const [h1, a1] = flare[i]!;
    flareV += (Math.PI * (h1 - h0) * (a0 * a0 + a0 * a1 + a1 * a1)) / 3;
  }
  addKg("root-plate", flareV * sp.densityKgM3 * 0.85, [sp.rootFlare.lengthM / 3, 0, 0]);
  const leafKg = new Map<string, number>();
  const vol = (c: { rad: V3 }) => (4 / 3) * Math.PI * c.rad[0] * c.rad[1] * c.rad[2];
  const totals = { "leaf-sun": 0, "leaf-shade": 0 };
  for (const c of sp.clusters) totals[c.region] += vol(c);
  for (const c of sp.clusters) {
    const area = (vol(c) / totals[c.region]) * (c.region === "leaf-sun" ? sp.leafAreaM2.sun : sp.leafAreaM2.shade);
    const kg = area * sp.leafFreshKgM2 * 1.25;
    leafKg.set(c.host, (leafKg.get(c.host) ?? 0) + kg);
    const f = fr.get(c.host)!;
    const l = toLocal(f, c.c);
    const o = woodKg.get(`leaf:${c.host}`) ?? { kg: 0, c: [0, 0, 0] as V3 };
    const k = o.kg + kg;
    woodKg.set(`leaf:${c.host}`, { kg: k, c: o.c.map((x, i) => (x * o.kg + l[i]! * kg) / k) as V3 });
  }
  const bones: Json[] = [];
  const section = (d: number, title: Text) => ({ areaM2: r((Math.PI * d * d) / 4, 5), secondMomentM4: Number(((Math.PI * d ** 4) / 64).toPrecision(4)), extremeFibreM: r(d / 2), material: `${sp.id}-wood`, title });
  const mass = (kg: number, c: V3, uniform: boolean) => ({ kg: r(kg, 1), comX: r(c[0], 3), comY: r(c[1], 3), distribution: uniform ? "uniform" : "point", source: "authored-estimate", reference: "wood-handbook-2021" });
  const rootKg = woodKg.get("root-plate")!;
  const dRoot = 2 * sp.rootFlare.profile[sp.rootFlare.profile.length - 1]![1];
  bones.push({ id: "root-plate", parent: null, title: T("Root collar and flare", "Wurzelhals und Stammanlauf"), x: 0, y: 0, angleDeg: 90, lengthM: r(sp.rootFlare.lengthM), joint: { type: "fixed" }, mass: mass(rootKg.kg, rootKg.c, true), section: section(dRoot, T(`Root collar, d = ${dRoot.toFixed(2)} m`, `Wurzelhals, d = ${de(dRoot, 2)} m`)) });
  let total = rootKg.kg;
  for (const b of sp.wood) {
    const pf = fr.get(b.parent!)!;
    const f = fr.get(b.id)!;
    const l = toLocal(pf, b.a);
    level.set(b.id, b.level);
    const kg = woodKg.get(b.id) ?? { kg: 0, c: [0, 0, 0] as V3 };
    const leaf = woodKg.get(`leaf:${b.id}`);
    const allKg = kg.kg + (leaf?.kg ?? 0);
    total += allKg;
    const Lp = Math.hypot(b.b[0] - b.a[0], b.b[1] - b.a[1]);
    const d = 2 * b.r0;
    bones.push({ id: b.id, parent: b.parent, title: b.title, x: r(l[0]), y: r(l[1]), ...(Math.abs(l[2]) > 1e-4 ? { z: r(l[2]) } : {}), angleDeg: r(norm180(deg(f.angle - pf.angle)), 3), lengthM: r(Lp), joint: { type: "revolute", minDeg: -LIMIT[b.level], maxDeg: LIMIT[b.level] }, mass: mass(allKg, leaf ? leaf.c : kg.c, leaf === undefined), section: section(d, T(`${b.title.en}, d = ${d.toFixed(2)} m`, `${b.title.de}, d = ${de(d, 2)} m`)) });
  }
  for (const b of sp.foliage) {
    const pf = fr.get(b.parent)!;
    const f = fr.get(b.id)!;
    const l = toLocal(pf, b.a);
    level.set(b.id, 4);
    const leaf = woodKg.get(`leaf:${b.id}`) ?? { kg: 1, c: [0.2, 0, 0] as V3 };
    total += leaf.kg;
    bones.push({ id: b.id, parent: b.parent, title: b.title, x: r(l[0]), y: r(l[1]), ...(Math.abs(l[2]) > 1e-4 ? { z: r(l[2]) } : {}), angleDeg: r(norm180(deg(f.angle - pf.angle)), 3), lengthM: r(Math.hypot(b.b[0] - b.a[0], b.b[1] - b.a[1])), joint: { type: "revolute", minDeg: -LIMIT[4], maxDeg: LIMIT[4] }, mass: { ...mass(leaf.kg, leaf.c, false), reference: "jones-2014" } });
  }
  return { bones, massKg: r(total, 0), level, leafKg };
}

type Drawn = { shape: Json; near: number };

/** 🖼️ The shapes of one scale and view in authored paint order. */
export function shapesOf(sp: Species, scale: Scale, view: View, fr: Map<string, Frame>): Json[] {
  const prims = primsOf(sp, scale);
  const drawn: Drawn[] = [];
  const meta = (p: Prim, role = "body"): Json => ({ id: p.id, bone: p.host, role, z: 0, filled: true, tone: p.tone, region: p.region, material: p.material, ...(p.region === "leaf-shade" && scale <= 100 ? { hatch: "shade-lines" } : {}) });
  if (scale === 500) {
    const stem = prims.filter((p) => p.host === "root-plate" || p.host === sp.wood[0]!.id);
    const crown = prims.filter((p) => !stem.includes(p));
    const stemRings = union(stem.map((p) => primRing(view, p)));
    const crownRings = union(crown.map((p) => primRing(view, p)), 0.25);
    const host = sp.wood[1]!.id;
    drawn.push({ shape: pathShape(view, stemRings, sp.wood[0]!.id, fr, { id: "stem", role: "body", z: 0, filled: true, tone: 0.28, region: "stem-bark", material: `${sp.id}-bark` }), near: -1e9 });
    drawn.push({ shape: pathShape(view, crownRings, host, fr, { id: "crown", role: "body", z: 0, filled: true, tone: 0.56, regions: ["leaf-sun", "leaf-shade", "limb-bark"], material: `${sp.id}-leaf-sun` }), near: 1e9 });
    return order(drawn);
  }
  const wood = prims.filter((p) => p.kind !== "ellipsoid");
  for (const p of wood) drawn.push({ shape: shapeOfPrim(view, p, fr, meta(p)), near: primNear(view, p) });
  const leaves = prims.filter((p): p is Extract<Prim, { kind: "ellipsoid" }> => p.kind === "ellipsoid");
  if (scale === 200) for (const p of leaves) drawn.push({ shape: shapeOfPrim(view, p, fr, meta(p)), near: primNear(view, p) });
  else
    for (const c of sp.clusters) {
      const lobes = leaves.filter((p) => p.cluster === c.id);
      const rings = union(lobes.map((p) => primRing(view, p)));
      const near = Math.max(...lobes.map((p) => primNear(view, p)));
      const m = { ...meta(lobes[0]!), id: c.id };
      drawn.push({ shape: pathShape(view, rings, c.host, fr, m), near });
      if (scale === 50 && c.region === "leaf-sun") {
        const marks = marksIn(sp, rings, c.seed * 13 + VIEWS.indexOf(view), 0.04);
        const f = fr.get(c.host)!;
        marks.forEach((strokes, i) => {
          const local = view === "elevation" ? strokes.map((s) => s.map((q) => elevLocal(f, q))) : strokes;
          drawn.push({ shape: { id: `${c.id}-mark-${i + 1}`, bone: c.host, role: "texture", z: 0, filled: false, tone: 0.5, kind: "path", d: local.map((s) => `M ${s.map((q) => `${r(q.x)} ${r(q.y)}`).join(" L ")}`).join(" ") }, near: near + 1e-6 * (i + 1) });
        });
      }
    }
  if (scale === 50 && view !== "plan")
    for (const p of wood) {
      if (p.kind !== "capsule" || !(p.host === sp.wood[0]!.id || p.host === sp.wood[1]!.id)) continue;
      const f = fr.get(p.host)!;
      const [a, b] = [project(view, p.a), project(view, p.b)];
      const lines = barkMarks(sp, a, b, p.r0, p.r1, 0.035, sp.bark.seed + (view === "front" ? 100 : 0) + p.id.length);
      lines.forEach((pts, i) => {
        const local = view === "elevation" ? pts.map((q) => elevLocal(f, q)) : pts;
        drawn.push({ shape: { id: `${p.id}-bark-${i + 1}`, bone: p.host, role: "texture", z: 0, filled: false, tone: 0.3, kind: "poly", points: local.map((q) => [r(q.x), r(q.y)]), closed: false }, near: primNear(view, p) + 1e-6 * (i + 1) });
      });
    }
  return order(drawn);
}

const order = (drawn: Drawn[]): Json[] => [...drawn].sort((a, b) => a.near - b.near).map((d, i) => ({ ...d.shape, z: r(1 + i * 0.01, 3) }));

/** 📦️ The extents of a view of one scale from its filled shapes (world metres of the paper: elevation x, y; front z, y; plan x, z). */
export function extentsOf(sp: Species, scale: Scale, view: View): { minX: number; maxX: number; minY: number; maxY: number } {
  const prims = primsOf(sp, scale);
  return boxOf(prims.flatMap((p) => primRing(view, p)));
}

/** 📍️ The support anchor and the transpiration anchors (tops of the sun clusters, sides of the shade clusters). */
export function anchorsOf(sp: Species, fr: Map<string, Frame>): { anchors: Json[]; sun: string[]; shade: string[] } {
  const d = 2 * sp.rootFlare.profile[0]![1];
  const anchors: Json[] = [{ id: "base-support", bone: "root-plate", x: 0, y: 0, kind: "support", title: T("Root plate anchorage", "Verankerung des Wurzeltellers"), support: "fixed", normalDeg: 90, footprintLengthM: r(d, 2), footprintWidthM: r(d, 2), frictionCoefficient: 0.6 }];
  const pick = (region: string, n: number): { id: string; host: string; p: V3 }[] => {
    const cs = sp.clusters.filter((c) => c.region === region).sort((a, b) => Math.atan2(a.c[2], a.c[0]) - Math.atan2(b.c[2], b.c[0]));
    const out: { id: string; host: string; p: V3 }[] = [];
    for (let i = 0; out.length < n; i++) {
      const c = cs[i % cs.length]!;
      const round = Math.floor(i / cs.length);
      const p: V3 = round === 0 ? [c.c[0], c.c[1] + 0.72 * c.rad[1], c.c[2]] : [c.c[0] + 0.55 * c.rad[0] * (round % 2 ? 1 : -1), c.c[1] + 0.3 * c.rad[1], c.c[2] + 0.55 * c.rad[2] * (round % 2 ? -1 : 1)];
      out.push({ id: `${region === "leaf-sun" ? "tp-sun" : "tp-shade"}-${out.length + 1}`, host: c.host, p });
    }
    return out;
  };
  const sun = pick("leaf-sun", Math.min(9, Math.max(5, sp.clusters.filter((c) => c.region === "leaf-sun").length)));
  const shade = pick("leaf-shade", Math.min(7, Math.max(5, sp.clusters.filter((c) => c.region === "leaf-shade").length)));
  for (const [list, en, deText] of [[sun, "sun leaves", "Sonnenblätter"], [shade, "shade leaves", "Schattenblätter"]] as const)
    list.forEach((a, i) => {
      const l = toLocal(fr.get(a.host)!, a.p);
      anchors.push({ id: a.id, bone: a.host, x: r(l[0]), y: r(l[1]), z: r(l[2]), kind: "emitter", title: T(`Transpiration of the ${en}, anchor ${i + 1}`, `Transpiration der ${deText}, Anker ${i + 1}`) });
    });
  return { anchors, sun: sun.map((a) => a.id), shade: shade.map((a) => a.id) };
}

/** 🌡️ The thermal model: environment, the four regions, two transpiration emitters. */
export function thermalOf(sp: Species, fr: Map<string, Frame>, an: { sun: string[]; shade: string[] }, anchors: Json[]): Json {
  const up = (id: string): number => {
    const a = anchors.find((x) => x.id === id)!;
    return r(90 - deg(fr.get(a.bone)!.angle), 2);
  };
  return {
    environment: sp.environment,
    passengers: [],
    operating: [],
    regions: [sp.regions.stem, sp.regions.limb, sp.regions.sun, sp.regions.shade],
    emitters: [
      { id: "transpiration-sun", kind: "transpiration", title: T("Transpiration of the sun leaves", "Transpiration der Sonnenblätter"), anchor: an.sun[0], anchors: an.sun, directionDeg: up(an.sun[0]!), diameterM: 1.1, region: "leaf-sun", source: "authored-estimate", reference: "monteith-unsworth-2013" },
      { id: "transpiration-shade", kind: "transpiration", title: T("Transpiration of the shade leaves", "Transpiration der Schattenblätter"), anchor: an.shade[0], anchors: an.shade, directionDeg: up(an.shade[0]!), diameterM: 0.8, region: "leaf-shade", source: "authored-estimate", reference: "monteith-unsworth-2013" },
    ],
    assumptions: sp.assumptions,
  };
}

/** 🧊️ The authored volume of one scale: body solids from the primitives, the thermal list by reference plus emitter outlets, the structure list of members, joints, masses and the root support. */
export function volumeOf(sp: Species, scale: Scale, fr: Map<string, Frame>, rig: Json[], anchors: Json[], words: { body: Text; thermal: Text; structure: Text }): Json {
  const f = FEATURE[scale];
  const prims = primsOf(sp, scale);
  const body = prims.map((p) => solidOf(p, fr));
  const thermal: Json[] = prims.map((p) => ({ id: `th-${p.id}`, of: p.id, region: p.region }));
  for (const a of anchors.filter((x) => x.kind === "emitter")) {
    const e = a.id.startsWith("tp-sun") ? "transpiration-sun" : "transpiration-shade";
    const region = e === "transpiration-sun" ? "leaf-sun" : "leaf-shade";
    const rr = 0.14 + 2 * f;
    thermal.push({ id: `th-${a.id}`, kind: "ellipsoid", bone: a.bone, material: `${sp.id}-${region}`, role: "detail", tone: 0.5, radii: [r(rr), r(rr * 0.6), r(rr)], transform: { translation: [a.x, a.y, a.z ?? 0] }, region, tags: ["emitter", e] });
  }
  const structure: Json[] = [];
  const woodOf = new Map(sp.wood.map((b) => [b.id, b] as const));
  const rootTop = sp.rootFlare.profile[sp.rootFlare.profile.length - 1]![1];
  const share = { 50: 0.55, 100: 0.58, 200: 0.62, 500: 0.66 }[scale];
  const memberR = (rad: number): number => Math.max(share * rad, f / 2 + 1e-4);
  const rootM = memberR(rootTop * 1.1);
  structure.push({ id: "member-root-plate", kind: "capsule", bone: "root-plate", material: `${sp.id}-wood`, role: "body", tone: 0.4, p0: [r(rootM), 0, 0], p1: [r(sp.rootFlare.lengthM), 0, 0], r0: r(rootM), r1: r(memberR(rootTop)), member: "root-plate", part: "member" });
  for (const b of sp.wood) {
    const fb = fr.get(b.id)!;
    const p0 = toLocal(fb, b.a);
    const p1 = toLocal(fb, b.b);
    const L = len(sub(b.b, b.a));
    const [m0, m1] = [memberR(b.r0), memberR(b.r1)];
    const slender = L >= 5 * 2 * Math.max(m0, m1) && L >= 2 * f;
    structure.push({ id: `member-${b.id}`, kind: "capsule", bone: b.id, material: `${sp.id}-wood`, role: "body", tone: 0.4, p0: p0.map((x) => r(x)), p1: p1.map((x) => r(x)), r0: r(m0), r1: r(m1), ...(slender ? { slender: true } : {}), member: b.id, part: "member" });
    const jr = Math.max((scale === 50 ? 0.8 : scale === 100 ? 0.85 : 0.9) * m0, f / 2 + 2e-4);
    if (scale !== 50 && b.level === 3) continue;
    if (scale === 500 && b.level === 1 && b.r1 < 0.05) continue;
    structure.push({ id: `joint-${b.id}`, kind: "ellipsoid", bone: b.id, material: `${sp.id}-wood`, role: "detail", tone: 0.3, radii: [r(jr), r(jr), r(jr)], member: b.id, part: "joint" });
  }
  const heavy = [...rig].filter((b) => woodOf.has(b.id)).sort((a, b) => b.mass.kg - a.mass.kg).slice(0, scale <= 100 ? 3 : scale === 200 ? 2 : 1);
  for (const b of heavy) {
    const w = woodOf.get(b.id)!;
    const mid = toLocal(fr.get(b.id)!, add(w.a, mul(sub(w.b, w.a), 0.45)));
    const mr = Math.max(0.6 * memberR(w.r0) * 1.6, f / 2 + 3e-4);
    structure.push({ id: `mass-${b.id}`, kind: "ellipsoid", bone: b.id, material: `${sp.id}-wood`, role: "detail", tone: 0.3, radii: [r(mr), r(mr), r(mr)], transform: { translation: mid.map((x) => r(x)) }, member: b.id, part: "mass" });
  }
  const rs = sp.rootFlare.profile[0]![1] * 1.05;
  const t = Math.max(0.06, f * 1.1);
  structure.push({ id: "support-base", kind: "revolve", bone: "root-plate", material: `${sp.id}-wood`, role: "ground", tone: 0.3, profile: [[0, r(rs)], [r(t), r(rs)]], axis: [1, 0, 0], origin: [0, 0, 0], member: "root-plate", part: "support", anchor: "base-support" });
  return { status: "authored", denominator: scale, body, thermal, structure, decisions: [words.body, words.thermal, words.structure] };
}

/** 🌬️ Wind speed at height `y` for a 10 m reference speed (power law, exponent 0.22). */
export const windAt = (v10: number, y: number): number => v10 * Math.pow(Math.max(1, y) / 10, 0.22);

/** 🧲️ The loads of a pose: drag on the foliage of every host bone, line drag on stem and leader, wet snow on the foliage plan area. */
export function loadsOf(sp: Species, fr: Map<string, Frame>, windMS: number, snowKgM2: number, cd: number): Json[] {
  const loads: Json[] = [];
  const hosts = [...new Set(sp.clusters.map((c) => c.host))];
  const titleOf = (id: string): Text => [...sp.wood, ...sp.foliage].find((b) => b.id === id)?.title ?? T(id, id);
  for (const h of hosts) {
    const cs = sp.clusters.filter((c) => c.host === h);
    const f = fr.get(h)!;
    const area = cs.reduce((s, c) => s + Math.PI * c.rad[1] * c.rad[2] * 0.55, 0);
    const yc = cs.reduce((s, c) => s + c.c[1], 0) / cs.length;
    const v = windAt(windMS, yc);
    const F = 0.5 * RHO_AIR * cd * area * v * v;
    const centre: V3 = mul(cs.reduce((s, c) => add(s, c.c), [0, 0, 0] as V3), 1 / cs.length);
    const l = toLocal(f, centre);
    const t = titleOf(h);
    loads.push({ id: `wind-${h}`, kind: "wind", title: T(`Drag on the foliage: ${t.en}`, `Windwiderstand am Laub: ${t.de}`), bone: h, x: r(l[0], 3), y: r(l[1], 3), fxN: r(F, 1), fyN: 0, source: "literature", reference: "en-1991-1-4" });
    if (snowKgM2 > 0) {
      const plan = cs.reduce((s, c) => s + Math.PI * c.rad[0] * c.rad[2], 0) * 0.8;
      loads.push({ id: `snow-${h}`, kind: "snow", title: T(`Wet snow on the foliage: ${t.en}`, `Nassschnee auf dem Laub: ${t.de}`), bone: h, x: r(l[0], 3), y: r(l[1], 3), fxN: 0, fyN: r(-snowKgM2 * plan * G, 1), source: "authored-estimate", reference: "en-1991-1-3" });
    }
  }
  for (const b of sp.wood.filter((w) => w.level <= 1)) {
    const yc = (b.a[1] + b.b[1]) / 2;
    const v = windAt(windMS, yc);
    const w = 0.5 * RHO_AIR * sp.cd.stem * (b.r0 + b.r1) * v * v;
    loads.push({ id: `wind-${b.id}-stem`, kind: "wind", title: T(`Drag on the wood: ${b.title.en}`, `Widerstand am Holz: ${b.title.de}`), fxN: 0, fyN: 0, distributed: { bone: b.id, fromM: 0, toM: r(Math.hypot(b.b[0] - b.a[0], b.b[1] - b.a[1]), 3), wxNPerM: r(w, 2), wyNPerM: 0 }, source: "literature", reference: "en-1991-1-4" });
  }
  return loads;
}

/** 🎚️ The lean of every bone for a wind factor and a snow factor, in degrees (clockwise is downwind for upward bones). */
export function leanOf(sp: Species, fr: Map<string, Frame>, level: Map<string, number>, windFactor: number, snowFactor: number, phase?: (lv: number) => number): Json {
  const out: Json = {};
  for (const [id, lv] of level) {
    if (id === "root-plate") continue;
    const k = phase === undefined ? 1 : phase(lv);
    const wind = -sp.sway.lean[lv as 0 | 1 | 2 | 3 | 4] * windFactor * k;
    const ux = Math.cos(fr.get(id)!.angle);
    const snow = lv >= 2 ? -Math.sign(ux) * Math.min(1, Math.abs(ux) * 2) * snowFactor * (lv === 4 ? 2.2 : lv === 3 ? 1.4 : 0.8) : 0;
    const a = r(wind + snow, 2);
    if (a !== 0) out[id] = { angleDeg: a };
  }
  return out;
}

/** 🧍️ The named poses: the species pose table with its wind leans, loads and thermal bindings. */
export function posesOf(sp: Species, fr: Map<string, Frame>, level: Map<string, number>): Json[] {
  return sp.poses.map((ps) => {
    const windFactor = ps.leanFactor;
    const snowFactor = ps.snowKgM2 === undefined ? 0 : ps.snowKgM2 / 10;
    const cd = ps.windMS > 12 ? sp.cd.gale : sp.cd.breeze;
    const regions: Json = {};
    for (const id of ["stem-bark", "limb-bark", "leaf-sun", "leaf-shade"]) regions[id] = { exposedFraction: 1, sunlitFraction: ps.sun[id], ...(ps.wet === undefined ? {} : { wettedness: ps.wet }) };
    return { id: ps.id, title: ps.title, description: ps.description, bones: windFactor === 0 && snowFactor === 0 ? {} : leanOf(sp, fr, level, windFactor, snowFactor), ground: "authored", contacts: [], thermal: { regions, emitters: { "transpiration-sun": ps.emitters.sun, "transpiration-shade": ps.emitters.shade } }, loads: loadsOf(sp, fr, ps.windMS, ps.snowKgM2 ?? 0, cd) };
  });
}

/** 🎞️ The clips: the looping wind sway, the gust and the 24 h transpiration cycle. */
export function clipsOf(sp: Species, fr: Map<string, Frame>, level: Map<string, number>, windy: Json, calm: Json): Json[] {
  const P = sp.sway.periodS;
  const sway: Json[] = [];
  const breezeFactor = sp.poses.find((p) => p.id === windy.id)!.leanFactor;
  const n = 8;
  for (let i = 0; i <= n; i++) {
    const t = (P * i) / n;
    const lag = (lv: number) => 1 + 0.38 * Math.sin((2 * Math.PI * t) / P - lv * 0.45);
    sway.push({ t: r(t, 3), pose: windy.id, ease: "smooth", bones: leanOf(sp, fr, level, breezeFactor, 0, lag) });
  }
  const gustT = [0, 1.2, 2.5, 3.6, 5, 8];
  const gustV = [1, 1.28, sp.sway.gustPeak, sp.sway.gustPeak * 0.8, 1.33, 1];
  const g = (t: number): number => {
    if (t <= 0) return 1;
    if (t >= 8) return 1;
    let i = 0;
    while (gustT[i + 1]! < t) i++;
    const u = (t - gustT[i]!) / (gustT[i + 1]! - gustT[i]!);
    const s = u * u * (3 - 2 * u);
    return gustV[i]! + (gustV[i + 1]! - gustV[i]!) * s;
  };
  const gustKeys: Json[] = [];
  for (let i = 0; i <= 16; i++) {
    const t = i * 0.5;
    gustKeys.push({ t, pose: windy.id, ease: "smooth", bones: leanOf(sp, fr, level, breezeFactor, 0, (lv) => Math.pow(g(Math.max(0, t - (4 - lv) * 0.2)), 0.85)) });
  }
  gustKeys[16] = { ...gustKeys[16], bones: leanOf(sp, fr, level, breezeFactor, 0) };
  gustKeys[0] = { ...gustKeys[0], bones: leanOf(sp, fr, level, breezeFactor, 0) };
  const loadTracks = windy.loads.filter((l: Json) => l.kind === "wind").map((l: Json) => ({ target: `load:${l.id}`, ease: "smooth", keys: gustT.map((t, i) => ({ t, v: r(gustV[i]!, 3) })) }));
  const e = windy.thermal.emitters;
  const sunTrack = [[0, 0.03], [5, 0.05], [7, 0.35], [9, 0.85], [11.5, 1], [14, 0.72], [16, 0.78], [18, 0.32], [20, 0.08], [24, 0.03]];
  const sunlit = [[0, 0], [5, 0], [6, 0.05], [9, 0.6], [12, 1], [15, 0.7], [18, 0.15], [19, 0], [24, 0]];
  const calmRegions = calm.thermal.regions;
  return [
    {
      id: "wind-sway",
      title: T("Wind sway", "Windschwingen"),
      description: T(`A deterministic ${P.toFixed(1)} s sway cycle about the breeze lean (${sp.sway.breezeMS} m/s at 10 m), close to the first bending mode of a free-standing ${sp.title.en.toLowerCase()} of this size (about ${(1 / P).toFixed(2)} Hz); each level of the crown lags the one below it, the foliage swings most.`, `Ein deterministischer Schwingungszyklus von ${de(P)} s um die Brisenneigung (${de(sp.sway.breezeMS, 1)} m/s in 10 m Höhe), nahe der ersten Biegeeigenform eines frei stehenden Baums dieser Größe (etwa ${de(1 / P, 2)} Hz); jede Kronenebene läuft der darunter nach, das Laub schwingt am stärksten.`),
      durationS: r(P, 3),
      fps: 30,
      loop: true,
      rootMotion: "none",
      keys: sway,
      contacts: [],
      phases: [{ id: "sway", fromS: 0, toS: r(P, 3), title: T("Sway cycle", "Schwingungszyklus") }],
      events: [{ t: 0, kind: "motion", title: T("Cycle start at the mean breeze lean", "Zyklusbeginn in der mittleren Brisenneigung") }],
      tracks: [
        { target: "emitter:transpiration-sun", ease: "smooth", keys: [{ t: 0, v: e["transpiration-sun"] }, { t: r(P / 2, 3), v: r(e["transpiration-sun"] * 0.94, 3) }, { t: r(P, 3), v: e["transpiration-sun"] }] },
        { target: "emitter:transpiration-shade", ease: "smooth", keys: [{ t: 0, v: e["transpiration-shade"] }, { t: r(P / 2, 3), v: r(e["transpiration-shade"] * 0.94, 3) }, { t: r(P, 3), v: e["transpiration-shade"] }] },
      ],
    },
    {
      id: "gust",
      title: T("Gust", "Böe"),
      description: T(`A single gust over 8 s from ${sp.sway.breezeMS} m/s to about ${(sp.sway.breezeMS * Math.sqrt(sp.sway.gustPeak)).toFixed(0)} m/s at 10 m: the wind-load tracks follow the velocity pressure (peak factor ${sp.sway.gustPeak}, EN 1991-1-4 with a turbulence intensity of about 0.25) and the crown leans with a level-wise lag, foliage first, the stem last.`, `Eine einzelne Böe über 8 s von ${de(sp.sway.breezeMS, 1)} m/s auf etwa ${(sp.sway.breezeMS * Math.sqrt(sp.sway.gustPeak)).toFixed(0)} m/s in 10 m Höhe: die Windlastspuren folgen dem Geschwindigkeitsdruck (Spitzenfaktor ${de(sp.sway.gustPeak, 2)}, EN 1991-1-4 mit einer Turbulenzintensität von etwa 0,25), die Krone neigt sich ebenenweise verzögert, zuerst das Laub, zuletzt der Stamm.`),
      durationS: 8,
      fps: 30,
      loop: false,
      rootMotion: "none",
      keys: gustKeys,
      contacts: [],
      phases: [{ id: "rise", fromS: 0, toS: 2.5, title: T("Gust front", "Böenfront") }, { id: "peak", fromS: 2.5, toS: 3.6, title: T("Peak", "Spitze") }, { id: "decay", fromS: 3.6, toS: 8, title: T("Decay", "Abklingen") }],
      events: [{ t: 2.5, kind: "load-on", title: T("Peak gust load", "Böenspitze") }, { t: 5, kind: "load-off", title: T("Load back near the mean", "Last wieder nahe dem Mittelwert") }],
      tracks: loadTracks,
    },
    {
      id: "diurnal-transpiration",
      title: T("Diurnal transpiration", "Tagesgang der Transpiration"),
      description: T("A 24 h cycle compressed to 24 s (1 s = 1 h, 00:00 to 24:00), calm weather. The sunlit fraction of every region and the transpiration strength follow the sun, with stomatal closure at night and a mid-afternoon depression; leaf and bark temperatures are not authored but follow from the thermal solver.", "Ein 24-h-Zyklus auf 24 s verdichtet (1 s = 1 h, 00:00 bis 24:00), windstill. Der besonnte Anteil jeder Region und die Transpirationsstärke folgen der Sonne, mit Spaltöffnungsschluss in der Nacht und einer Mittagsdepression am Nachmittag; Blatt- und Rindentemperaturen sind nicht vorgegeben, sondern folgen aus dem thermischen Löser."),
      durationS: 24,
      fps: 10,
      loop: true,
      rootMotion: "none",
      keys: [{ t: 0, pose: calm.id, ease: "smooth" }, { t: 12, pose: calm.id, ease: "smooth" }, { t: 24, pose: calm.id, ease: "smooth" }],
      contacts: [],
      phases: [{ id: "night", fromS: 0, toS: 6, title: T("Night", "Nacht") }, { id: "morning", fromS: 6, toS: 12, title: T("Morning to noon", "Morgen bis Mittag") }, { id: "afternoon", fromS: 12, toS: 19, title: T("Afternoon", "Nachmittag") }, { id: "evening", fromS: 19, toS: 24, title: T("Evening and night", "Abend und Nacht") }],
      events: [{ t: 6, kind: "thermal", title: T("Sunrise", "Sonnenaufgang") }, { t: 12, kind: "thermal", title: T("Solar noon", "Sonnenhöchststand") }, { t: 19, kind: "thermal", title: T("Sunset", "Sonnenuntergang") }],
      tracks: [
        { target: "emitter:transpiration-sun", ease: "smooth", keys: sunTrack.map(([t, v]) => ({ t, v })) },
        { target: "emitter:transpiration-shade", ease: "smooth", keys: sunTrack.map(([t, v]) => ({ t, v: r(v! * 0.7, 3) })) },
        ...["stem-bark", "limb-bark", "leaf-sun", "leaf-shade"].map((id) => ({ target: `region:${id}:sunlitFraction`, ease: "smooth", keys: sunlit.map(([t, v]) => ({ t, v: r(v! * calmRegions[id].sunlitFraction, 3) })) })),
      ],
    },
  ];
}

const circlePath = (cx: number, cy: number, rr: number): string => pathOf([Array.from({ length: 24 }, (_, i) => ({ x: cx + rr * Math.cos((Math.PI * i) / 12), y: cy + rr * Math.sin((Math.PI * i) / 12) }))]);

/** 🔣️ The two plan symbols: the true crown extent with stem and scaffold directions, and the nominal circle-and-dot tree sign. */
export function symbolsOf(sp: Species, dims: Json): Json[] {
  const prims = primsOf(sp, 200);
  const crownOf = (view: View): P2[] => {
    const rings = union(prims.filter((p) => p.host !== "root-plate" && p.host !== sp.wood[0]!.id).map((p) => primRing(view, p)));
    return rings.filter((q) => ringArea(q) > 0).sort((a, b) => ringArea(b) - ringArea(a))[0]!;
  };
  const stemOf = (view: View): P2[] => union(prims.filter((p) => p.host === "root-plate" || p.host === sp.wood[0]!.id).map((p) => primRing(view, p))).sort((a, b) => ringArea(b) - ringArea(a))[0]!;
  const poly = (id: string, pts: P2[], z: number, filled: boolean, tone: number, role = "body", closed = true): Json => ({ id, bone: "root-plate", role, z, filled, tone, kind: "poly", points: pts.map((q) => [r(q.x), r(q.y)]), closed });
  const geometry: Json = {};
  for (const view of VIEWS) {
    const list: Json[] = [poly("crown", crownOf(view), 2, false, 0.5), poly("stem-1", stemOf(view), 3, true, 0.3)];
    if (view === "plan") {
      const tips = sp.wood.filter((b) => b.level === 2).slice(0, 6);
      tips.forEach((b, i) => list.push(poly(`scaffold-${i + 1}`, [project("plan", b.a), project("plan", b.b)], 3, false, 0.35, "detail", false)));
    }
    geometry[view] = list;
  }
  const H = dims.heightM;
  const W = dims.widthM;
  const D = dims.depthM;
  const sign: Json = {};
  for (const view of VIEWS) {
    const cy = view === "plan" ? 0 : 3.42;
    sign[view] = [
      { id: "sign-circle", bone: "root-plate", role: "body", z: 1, filled: false, tone: 0.5, kind: "path", d: circlePath(0, cy, 1.8) },
      { id: "sign-dot", bone: "root-plate", role: "body", z: 2, filled: true, tone: 0.3, kind: "path", d: circlePath(0, cy, 0.35) },
      ...(view === "plan" ? [] : [{ id: "sign-stem", bone: "root-plate", role: "body", z: 1, filled: false, tone: 0.3, kind: "path", d: "M 0 0 L 0 1.62" }]),
    ];
  }
  return [
    { id: `${sp.id}-symbol`, title: T("Plan symbol with true crown extent", "Grundrisssymbol mit wahrer Kronenausdehnung"), description: T("A conventional sign for site and landscape plans: the true outline of the crown, the scaffold limb directions in plan and the stem section. It is a separate symbol and is never used silently for a pictorial scale.", "Ein konventionelles Zeichen für Lage- und Freiraumpläne: der wahre Umriss der Krone, im Grundriss die Richtungen der Gerüstäste und der Stammquerschnitt. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet."), semantics: "true-extent", units: "model-m", denominators: [500, 200, 100], pens: { body: 0.2, detail: 0.13, texture: 0.1, glazing: null, ground: 0.3 }, hatches: [], geometry, dimensionalNote: T(`A conventional simplification of the tree drawn at its true extent in model metres in all three views (${W.toFixed(2)} m wide, ${D.toFixed(2)} m deep and ${H.toFixed(2)} m high): the outline is the crown, the stem is drawn at its real position and thickness. It is a symbol, never used silently for a pictorial scale.`, `Eine konventionelle Vereinfachung des Baums in seiner wahren Ausdehnung in Modellmetern in allen drei Ansichten (${de(W, 2)} m breit, ${de(D, 2)} m tief und ${de(H, 2)} m hoch): der Umriss ist die Krone, der Stamm ist in wirklicher Lage und Stärke gezeichnet. Es ist ein Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet.`) },
    { id: "tree-sign", title: T("Nominal tree sign with trunk centre", "Nominelles Baumzeichen mit Stammmitte"), description: T("The conventional circle-and-dot tree sign of small-scale site plans: the dot marks the trunk centre, the circle is a fixed paper size.", "Das übliche Kreis-und-Punkt-Baumzeichen kleinmaßstäblicher Lagepläne: Der Punkt markiert die Stammmitte, der Kreis hat eine feste Papiergröße."), semantics: "nominal", units: "paper-mm", denominators: [500, 200], pens: { body: 0.15, detail: null, texture: null, glazing: null, ground: 0.25 }, hatches: [], geometry: sign, dimensionalNote: T("A conventional symbol in paper millimetres, not to scale: a fixed 3.6 mm tree sign, drawn as plan circle, side lollipop and front lollipop. Its size carries no dimension.", "Ein konventionelles Symbol in Papiermillimetern, nicht maßstäblich: ein festes Baumzeichen von 3,6 mm, gezeichnet als Grundrisskreis sowie als Lutscher in Seiten- und Vorderansicht. Seine Größe trägt keine Abmessung.") },
  ];
}

/** 📝️ The authored decisions of one representation. */
export function decisionsOf(sp: Species, scale: Scale, counts: { wood: number; clusters: number; marks: number }): Json {
  const t = sp.texts;
  const mm = 1000 / scale;
  const head = T(`At 1:${scale} (1 m is ${mm} mm)`, `Bei 1:${scale} (1 m sind ${de(mm, mm < 10 ? 0 : 0)} mm)`);
  if (scale === 500)
    return {
      silhouette: T(`${head.en} the ${sp.title.en} is one composed crown silhouette over its stem: ${t.crown.en}`, `${head.de} ist ${sp.title.de} eine einzige gefügte Kronensilhouette über dem Stamm: ${t.crown.de}`),
      retained: T("Retained: the stem with its root flare as one outline and the crown as one outline that is the union of the projected crown masses of this scale's model.", "Beibehalten: der Stamm mit seinem Anlauf als ein Umriss und die Krone als ein Umriss, die Vereinigung der projizierten Kronenmassen des Modells dieses Maßstabs."),
      merged: T(`The ${sp.clusters.length} foliage clusters are merged into ${sp.masses500.length} crown masses and those into one crown outline; limbs inside the crown are part of it.`, `Die ${sp.clusters.length} Laubballen sind zu ${sp.masses500.length} Kronenmassen und diese zu einem Kronenumriss verschmolzen; Äste innerhalb der Krone gehören dazu.`),
      omitted: t.omitted500,
      thinElements: T("No element thinner than 0.2 m stays separate (the print minimum at 1:1000); thinner limbs are inside the crown outline.", "Kein Element dünner als 0,2 m bleibt eigenständig (das Druckminimum bei 1:1000); dünnere Äste liegen im Kronenumriss."),
      hatching: T("None: the tone of the crown carries the mass.", "Keine: der Ton der Krone trägt die Masse."),
      styles: T("Line views draw two outlines with the 0.15 mm pen; solid views fill the stem dark and the crown mid-grey.", "Linienansichten zeichnen zwei Umrisse mit der 0,15-mm-Feder; Flächenansichten füllen den Stamm dunkel und die Krone mittelgrau."),
      overlays: T("The crown outline merges the sun leaves, the shade leaves and the limb bark (area-weighted); the stem shows the stem bark. Structural views colour the two outlines by the stem and leader members.", "Der Kronenumriss fasst Sonnenblätter, Schattenblätter und Astrinde zusammen (flächengewichtet); der Stamm zeigt die Stammrinde. Strukturansichten färben die zwei Umrisse nach den Stäben von Stamm und Stammverlängerung."),
    };
  if (scale === 200)
    return {
      silhouette: T(`${head.en} the ${sp.title.en} shows its main masses: ${t.stem.en} ${t.limbs.en} and ${counts.clusters} foliage clusters as ellipses.`, `${head.de} zeigt ${sp.title.de} die Hauptmassen: ${t.stem.de} ${t.limbs.de} und ${counts.clusters} Laubballen als Ellipsen.`),
      retained: T(`Retained: ${counts.wood} woody outlines (stem, leader and scaffold limbs, each the projection of its tapered model capsule) and one ellipse per foliage cluster, each painted in the depth order of the view.`, `Beibehalten: ${counts.wood} Holzumrisse (Stamm, Stammverlängerung und Gerüstäste, jeder die Projektion seiner verjüngten Modellkapsel) und je Laubballen eine Ellipse, in der Tiefenfolge der Ansicht gemalt.`),
      merged: T("Each foliage cluster is one ellipsoid of the model and one ellipse in each view; side branches are merged into the limbs that carry them.", "Jeder Laubballen ist ein Ellipsoid des Modells und in jeder Ansicht eine Ellipse; Seitenzweige sind in die tragenden Äste einbezogen."),
      omitted: t.omitted200,
      thinElements: T("Wood thinner than 70 mm is drawn at 70 mm only where it is part of a scaffold limb; thinner side branches are omitted.", "Holz dünner als 70 mm wird nur als Teil eines Gerüstasts mit 70 mm gezeichnet; dünnere Seitenzweige entfallen."),
      hatching: T("None at this scale.", "Keine bei diesem Maßstab."),
      styles: T("Line views draw every outline with the 0.18 mm pen; solid views separate dark bark, mid-grey shade clusters and light sun clusters.", "Linienansichten zeichnen jeden Umriss mit der 0,18-mm-Feder; Flächenansichten trennen dunkle Rinde, mittelgraue Schattenballen und helle Sonnenballen."),
      overlays: T("Every outline carries one region: stem bark, limb bark, sun leaves or shade leaves; members follow the rig bones.", "Jeder Umriss trägt eine Region: Stammrinde, Astrinde, Sonnen- oder Schattenblätter; die Stäbe folgen den Rigknochen."),
    };
  return {
    silhouette: T(`${head.en} the ${sp.title.en} is drawn from its model of the same scale: ${t.crown.en}`, `${head.de} ist ${sp.title.de} aus dem Modell desselben Maßstabs gezeichnet: ${t.crown.de}`),
    retained: T(`Retained: ${counts.wood} woody outlines including the side branches and ${counts.clusters} lobed foliage outlines, each the union of the projected lobes of one cluster${counts.marks > 0 ? `, plus ${counts.marks} texture strokes of leaves and bark` : ""}.`, `Beibehalten: ${counts.wood} Holzumrisse samt Seitenzweigen und ${counts.clusters} gelappte Laubumrisse, je die Vereinigung der projizierten Lappen eines Ballens${counts.marks > 0 ? `, dazu ${counts.marks} Texturstriche von Blättern und Rinde` : ""}.`),
    merged: T(`The ${sp.lobes[scale as 50 | 100].n} lobes and the core of one cluster are one outline; single leaves are carried by ${scale === 50 ? "the leaf marks" : "the lobed edge"}.`, `Die ${sp.lobes[scale as 50 | 100].n} Lappen und der Kern eines Ballens sind ein Umriss; Einzelblätter trägt ${scale === 50 ? "die Blattzeichnung" : "der gelappte Rand"}.`),
    omitted: T(scale === 50 ? `Omitted: single twigs and single leaves (${t.leaf.en}).` : "Omitted: twigs thinner than 30 mm, single leaves and bark texture.", scale === 50 ? `Weggelassen: einzelne Triebe und Einzelblätter (${t.leaf.de}).` : "Weggelassen: Triebe dünner als 30 mm, Einzelblätter und Rindentextur."),
    thinElements: T(scale === 50 ? "Side branches keep their true diameter down to 15 mm (0.2 mm at 1:75)." : "Side branches keep their true diameter down to 30 mm (0.2 mm at 1:150).", scale === 50 ? "Seitenzweige behalten ihren wahren Durchmesser bis 15 mm (0,2 mm bei 1:75)." : "Seitenzweige behalten ihren wahren Durchmesser bis 30 mm (0,2 mm bei 1:150)."),
    hatching: T(`The shade clusters of the inner crown carry the 45 degree shade hatch; ${scale === 50 ? `the bark carries ${t.bark.en}.` : "no texture."}`, `Die Schattenballen der Innenkrone tragen die 45-Grad-Schattenschraffur; ${scale === 50 ? `die Rinde trägt ${t.bark.de}.` : "keine Textur."}`),
    styles: T(`Line views draw bodies with the ${scale === 50 ? "0.25" : "0.2"} mm pen${scale === 50 ? ", leaf and bark marks with the 0.13 mm texture pen" : ""}; solid views separate dark bark, hatched shade clusters and light sun clusters.`, `Linienansichten zeichnen Körper mit der ${scale === 50 ? "0,25" : "0,2"}-mm-Feder${scale === 50 ? ", Blatt- und Rindenzeichen mit der 0,13-mm-Texturfeder" : ""}; Flächenansichten trennen dunkle Rinde, schraffierte Schattenballen und helle Sonnenballen.`),
    overlays: T("Every outline carries one region; the thermal view paints sun and shade leaves apart, the structural views colour stem, leader and limbs by their members.", "Jeder Umriss trägt eine Region; die thermische Ansicht malt Sonnen- und Schattenblätter getrennt, die Strukturansichten färben Stamm, Stammverlängerung und Äste nach ihren Stäben."),
  };
}

const PENS: Record<Scale, Json> = {
  500: { body: 0.15, detail: null, texture: null, glazing: null, ground: 0.25 },
  200: { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 },
  100: { body: 0.2, detail: 0.13, texture: 0.1, glazing: null, ground: 0.3 },
  50: { body: 0.25, detail: 0.18, texture: 0.13, glazing: null, ground: 0.35 },
};
const RANGE: Record<Scale, [number, number]> = { 500: [350, 1000], 200: [150, 350], 100: [75, 150], 50: [20, 75] };

/** 🏺️ The whole asset document and its manifest row. */
export function assetOf(sp: Species): { doc: Json; row: Json; report: Json } {
  const fr = frames(sp);
  const rig = rigOf(sp, fr);
  const { anchors, sun, shade } = anchorsOf(sp, fr);
  const ext = { elevation: extentsOf(sp, 200, "elevation"), front: extentsOf(sp, 200, "front"), plan: extentsOf(sp, 200, "plan") };
  const dims = { heightM: r(ext.elevation.maxY, 3), widthM: r(ext.elevation.maxX - ext.elevation.minX, 3), depthM: r(ext.front.maxX - ext.front.minX, 3), massKg: rig.massKg };
  const representations: Json = {};
  const volumes: Json = {};
  const report: Json = { extents: {}, shapes: {}, solids: {} };
  for (const scale of SCALES) {
    const geometry: Json = {};
    for (const view of VIEWS) geometry[view] = shapesOf(sp, scale, view, fr);
    const prims = primsOf(sp, scale);
    const counts = { wood: prims.filter((p) => p.kind !== "ellipsoid").length, clusters: scale === 500 ? sp.masses500.length : sp.clusters.length, marks: geometry.elevation.filter((s: Json) => s.role === "texture").length };
    representations[`${scale}`] = { status: "authored", denominator: scale, validRange: { minDenominator: RANGE[scale][0], maxDenominator: RANGE[scale][1] }, pens: PENS[scale], hatches: scale <= 100 ? [{ id: "shade-lines", title: T("Shade line hatch", "Schattenschraffur"), pattern: "lines", angleDeg: 45, spacingMm: scale === 50 ? 1 : 0.8, penMm: 0.1 }] : [], geometry, decisions: decisionsOf(sp, scale, counts) };
    const wordsBody = T(`Body at 1:${scale} (${prims.length} solids, smallest feature ${FEATURE[scale] * 1000} mm): ${scale === 500 ? `stem, root flare, the limbs at no less than 100 mm and ${sp.masses500.length} crown masses (one per limb system, split into ellipsoids of two neighbouring clusters each, turned along the line through them)` : scale === 200 ? `stem, leader and scaffold limbs as tapered capsules and one ellipsoid per foliage cluster (${sp.clusters.length})` : `stem, leader, limbs and side branches as tapered capsules and every cluster as a core with ${sp.lobes[scale as 50 | 100].n} lobes`}. Every limb starts on the axis of its parent, so sway never opens a joint, and every foliage mass is pierced by the wood that carries it.`, `Body bei 1:${scale} (${prims.length} Körper, kleinstes Merkmal ${FEATURE[scale] * 1000} mm): ${scale === 500 ? `Stamm, Stammanlauf, die Äste mit mindestens 100 mm und ${sp.masses500.length} Kronenmassen (eine je Astsystem, geteilt in Ellipsoide aus je zwei benachbarten Ballen, entlang deren Verbindungslinie gedreht)` : scale === 200 ? `Stamm, Stammverlängerung und Gerüstäste als verjüngte Kapseln und je Laubballen ein Ellipsoid (${sp.clusters.length})` : `Stamm, Stammverlängerung, Äste und Seitenzweige als verjüngte Kapseln und jeder Ballen als Kern mit ${sp.lobes[scale as 50 | 100].n} Lappen`}. Jeder Ast beginnt auf der Achse seines Mutterasts, so öffnet das Schwingen keine Fuge, und jede Laubmasse wird vom tragenden Holz durchstoßen.`);
    const wordsThermal = T(`Thermal at 1:${scale}: every bark and foliage surface is its region by reference to the body solid, so the thermal view is the same tree; the transpiration emitters are low outlet domes on the foliage that carries their anchors.`, `Thermik bei 1:${scale}: jede Rinden- und Laubfläche ist ihre Region als Verweis auf den Body-Körper, die thermische Ansicht zeigt also denselben Baum; die Transpirationsemitter sind flache Auslasskuppeln auf dem Laub, das ihre Anker trägt.`);
    const wordsStructure = T(`Structure at 1:${scale}: every stem, leader and limb bone is a round member along its true axis at ${{ 50: "0.55", 100: "0.58", 200: "0.62", 500: "0.66" }[scale]} of the bark radius (the share grows towards the coarse scales so the idealised member stays readable; never thinner than the smallest feature), connected inside its parent down to the root collar; joint spheres sit at the rig joints${scale === 50 ? "" : " (not at the joints inside a limb, which the scale merges)"}, the fixed support is the root plate disc on the ground and the ${scale <= 100 ? "three heaviest segments carry" : scale === 200 ? "two heaviest segments carry" : "heaviest segment carries"} a mass sphere.`, `Struktur bei 1:${scale}: jeder Knochen von Stamm, Stammverlängerung und Ästen ist ein runder Stab entlang seiner wahren Achse mit ${{ 50: "0,55", 100: "0,58", 200: "0,62", 500: "0,66" }[scale]} des Rindenradius (der Anteil wächst zu den groben Maßstäben, damit der idealisierte Stab lesbar bleibt; nie dünner als das kleinste Merkmal), im Mutterast bis zum Wurzelhals verbunden; Gelenkkugeln sitzen an den Rig-Gelenken${scale === 50 ? "" : " (nicht an den Gelenken innerhalb eines Asts, die der Maßstab zusammenfasst)"}, das feste Auflager ist die Wurzeltellerscheibe am Boden, ${scale <= 100 ? "die drei schwersten Segmente tragen" : scale === 200 ? "die zwei schwersten Segmente tragen" : "das schwerste Segment trägt"} eine Massenkugel.`);
    volumes[`${scale}`] = volumeOf(sp, scale, fr, rig.bones, anchors, { body: wordsBody, thermal: wordsThermal, structure: wordsStructure });
    report.extents[scale] = { elevation: extentsOf(sp, scale, "elevation"), front: extentsOf(sp, scale, "front"), plan: extentsOf(sp, scale, "plan") };
    report.shapes[scale] = Object.fromEntries(VIEWS.map((v) => [v, geometry[v].length]));
    report.solids[scale] = { body: volumes[`${scale}`].body.length, thermal: volumes[`${scale}`].thermal.length, structure: volumes[`${scale}`].structure.length };
  }
  const poses = posesOf(sp, fr, rig.level);
  const windy = poses.find((p) => p.id === "windy")!;
  const calm = poses.find((p) => p.id === "calm")!;
  const clips = clipsOf(sp, fr, rig.level, windy, calm);
  const asset = {
    id: sp.id,
    category: "vegetation",
    title: sp.title,
    summary: sp.summary,
    tags: sp.tags,
    provenance: sp.provenance,
    dimensions: dims,
    materials: sp.materials,
    rig: { kind: "botanical", root: "root-plate", bones: rig.bones },
    anchors,
    representations,
    symbols: symbolsOf(sp, dims),
    poses,
    clips,
    thermal: thermalOf(sp, fr, { sun, shade }, anchors),
    structure: { gravityMS2: G, dynamics: "quasi-static", supports: ["base-support"], assumptions: sp.structureAssumptions, clinical: false },
    volumes,
    presentation: { defaultPose: "calm", defaultClip: "wind-sway", defaultScale: 200 },
    exports: ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    completion: "authored",
  };
  const doc = { $schema: "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", version: "semio.staffage/v2", asset };
  const row = { id: sp.id, category: "vegetation", title: sp.title, heightM: dims.heightM, widthM: dims.widthM, powerW: 0, poses: poses.map((p) => p.id), clips: clips.map((c) => c.id), representations: { "50": "authored", "100": "authored", "200": "authored", "500": "authored" }, symbols: asset.symbols.map((s: Json) => s.id), analytics: ["thermal", "transport", "structural"], exports: asset.exports, distinction: sp.distinction, completion: "authored" };
  report.dimensions = dims;
  report.bones = rig.bones.length;
  report.leafKg = Object.fromEntries(rig.leafKg);
  return { doc, row, report };
}

/** 💾️ Writes the asset into the catalog folder, the manifest row and the generator report into the ticket. */
export function writeAll(sp: Species): Json {
  const { doc, row, report } = assetOf(sp);
  const file = join(CATALOG, sp.id, "🔣️.json");
  mkdirSync(dirname(file), { recursive: true });
  writeFileSync(file, `${JSON.stringify(doc)}\n`);
  const rowFile = join(TICKET, "📥️manifest-rows", `${sp.id}.json`);
  mkdirSync(dirname(rowFile), { recursive: true });
  writeFileSync(rowFile, `${JSON.stringify(row, null, 2)}\n`);
  const rep = join(TICKET, "🗑️generated", "PW-04", `${sp.id}.build.json`);
  mkdirSync(dirname(rep), { recursive: true });
  writeFileSync(rep, `${JSON.stringify(report, null, 1)}\n`);
  return report;
}
