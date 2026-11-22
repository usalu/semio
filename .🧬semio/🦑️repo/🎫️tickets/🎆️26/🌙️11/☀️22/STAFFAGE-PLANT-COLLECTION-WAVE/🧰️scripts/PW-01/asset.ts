/** 🏗️ Builds one complete `semio.staffage/v2` vegetation asset of the PW-01 maple wave from its species design: planar botanical rig with lateral offsets, masses and sections from the modelled wood, per-scale volumes (body, thermal, structure), per-scale drawings as true views of that scale's body with their own detail and texture, symbols, poses, clips, thermal and structural models, all texts in English and German.
 * @see ./model.ts
 * @see ./design.ts
 * @see ../../📋️brief.md */
import { D2R, dirOf, quaternion, r4, r5, solidOf, type Lobe, type Tube, type View } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-small-leaved-lime-kit.ts";
import { dirToLocal, frames, toLocal, type Frame, type Json, type P2, type V3 } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/volumes-base.ts";
import { areaOf, clearIn, contours, ellipseOf, fieldOf, near, outlineOf, painterOf, primOf, rng, topAt, type Painter } from "./geom.ts";
import { SCALES, featureOf, foliageOf, woodOf, type Scale, type Species, type Text } from "./model.ts";
import type { Design, PoseSpec } from "./design.ts";

const VIEWS: View[] = ["elevation", "front", "plan"];
const RHO = 1.25;
const G = 9.80665;
const MAT: Record<string, string> = { bark: "maple-bark", young: "maple-bark-young", sun: "maple-leaf-sun", shade: "maple-leaf-shade" };
const REGION: Record<string, string> = { root: "trunk-bark", stem: "trunk-bark", limb: "limb-bark", side: "limb-bark", sun: "leaf-sun", tuft: "leaf-sun", shade: "leaf-shade" };
const r3 = (x: number): number => Math.round(x * 1000) / 1000;
const r2 = (x: number): number => Math.round(x * 100) / 100;
const t = (en: string, de: string): Text => ({ en, de });
const deNum = (x: number, d = 1): string => x.toFixed(d).replace(".", ",");

type Bone = Json;
type Placed = { o: V3; th: number; L: number };

/** 🦴️ The planar rig: root plate, axes, limbs and foliage bones placed on the modelled wood (bone z carries the lateral position of each joint). */
function rigOf(d: Design, wood50: Tube[]): { bones: Bone[]; placed: Record<string, Placed>; foliageOrigins: Record<string, V3> } {
  const sp = d.tree;
  const placed: Record<string, Placed> = { "root-plate": { o: [0, 0, 0], th: 90, L: 0.3 } };
  const bones: Bone[] = [{ id: "root-plate", parent: null, title: t("Root plate and collar", "Wurzelteller und Wurzelhals"), x: 0, y: 0, angleDeg: 90, lengthM: 0.3, joint: { type: "fixed" } }];
  const place = (id: string, parent: string, o: V3, th: number, L: number, title: Text, joint: Json): void => {
    const p = placed[parent]!;
    const u = [Math.cos(p.th * D2R), Math.sin(p.th * D2R)], v = [-Math.sin(p.th * D2R), Math.cos(p.th * D2R)];
    const dx = o[0] - p.o[0], dy = o[1] - p.o[1];
    let a = th - p.th;
    while (a > 180) a -= 360;
    while (a < -180) a += 360;
    const z = r4(o[2] - p.o[2]);
    bones.push({ id, parent, title, x: r4(dx * u[0]! + dy * u[1]!), y: r4(dx * v[0]! + dy * v[1]!), ...(Math.abs(z) > 1e-4 ? { z } : {}), angleDeg: r3(a), lengthM: r3(L), joint });
    placed[id] = { o, th, L };
  };
  const tubeOf = (id: string): Tube => wood50.find((w) => w.id === id)!;
  const planarOf = (tb: Tube): { th: number; L: number } => {
    const [a, b] = [tb.points[0]!.p, tb.points[tb.points.length - 1]!.p];
    return { th: Math.atan2(b[1] - a[1], b[0] - a[0]) / D2R, L: Math.hypot(b[0] - a[0], b[1] - a[1]) };
  };
  for (const a of sp.axes) {
    const tb = tubeOf(a.id);
    const { th, L } = planarOf(tb);
    place(a.id, a.parent, tb.points[0]!.p, th, L, a.title, { type: "revolute", minDeg: -a.limit, maxDeg: a.limit });
  }
  for (const l of sp.limbs) {
    const tb = tubeOf(l.id);
    const { th, L } = planarOf(tb);
    place(l.id, l.parent, tb.points[0]!.p, th, L, l.title, { type: "revolute", minDeg: -l.limit, maxDeg: l.limit });
  }
  const H = sp.crown.top;
  const fl = r3(Math.min(0.045 * H, 0.9));
  const foliageOrigins: Record<string, V3> = {};
  for (const l of sp.limbs) {
    const tb = tubeOf(l.id);
    const tip = tb.points[tb.points.length - 1]!.p;
    place(`fol-${l.id}`, l.id, tip, placed[l.id]!.th, fl, t(`Foliage of ${l.title.en.toLowerCase()}`, `Laub: ${l.title.de}`), { type: "revolute", minDeg: -16, maxDeg: 16 });
    foliageOrigins[`fol-${l.id}`] = tip;
  }
  const top = tubeOf(d.core.top);
  const topTip = top.points[top.points.length - 1]!.p;
  place("fol-top", d.core.top, topTip, 90, fl, t("Foliage of the crown top", "Laub der Kronenspitze"), { type: "revolute", minDeg: -16, maxDeg: 16 });
  foliageOrigins["fol-top"] = topTip;
  for (const [key, title] of [["low", t("Shaded inner crown, lower", "Beschattete Innenkrone, unten")], ["high", t("Shaded inner crown, upper", "Beschattete Innenkrone, oben")]] as const) {
    const [axis, y] = d.core[key];
    const tb = tubeOf(axis);
    const pts = tb.points;
    let p = pts[0]!.p;
    for (let i = 0; i + 1 < pts.length; i++) if (y >= pts[i]!.p[1] && y <= pts[i + 1]!.p[1]) {
      const f = (y - pts[i]!.p[1]) / (pts[i + 1]!.p[1] - pts[i]!.p[1]);
      p = [pts[i]!.p[0] + f * (pts[i + 1]!.p[0] - pts[i]!.p[0]), y, pts[i]!.p[2] + f * (pts[i + 1]!.p[2] - pts[i]!.p[2])];
    }
    place(`fol-core-${key}`, axis, p, 90, fl, title, { type: "revolute", minDeg: -12, maxDeg: 12 });
    foliageOrigins[`fol-core-${key}`] = p;
  }
  return { bones, placed, foliageOrigins };
}

/** ⚖️ Volume and world centroid of a tube (frusta between stations). */
function tubeVolume(tb: Tube): { v: number; c: V3; area: number } {
  let v = 0, area = 0;
  const c: V3 = [0, 0, 0];
  for (let i = 0; i + 1 < tb.points.length; i++) {
    const [a, b] = [tb.points[i]!, tb.points[i + 1]!];
    const h = Math.hypot(b.p[0] - a.p[0], b.p[1] - a.p[1], b.p[2] - a.p[2]);
    const dv = (Math.PI / 3) * h * (a.r * a.r + a.r * b.r + b.r * b.r);
    area += Math.PI * (a.r + b.r) * Math.hypot(h, a.r - b.r);
    v += dv;
    for (let k = 0; k < 3; k++) c[k]! += (dv * (a.p[k]! + b.p[k]!)) / 2;
  }
  return { v, c: v > 0 ? (c.map((x) => x / v) as V3) : tb.points[0]!.p, area };
}

/** 🌫️ Exposed lateral bark area of a set of tubes: the surface of each tube sampled and counted where it lies outside every other wood tube. */
function exposedBark(tubes: Tube[], of: Tube[]): number {
  const inside = (p: V3, self: Tube): boolean => tubes.some((o) => {
    if (o === self) return false;
    for (let i = 0; i + 1 < o.points.length; i++) {
      const [a, b] = [o.points[i]!, o.points[i + 1]!];
      const e = [b.p[0] - a.p[0], b.p[1] - a.p[1], b.p[2] - a.p[2]];
      const l2 = e[0]! ** 2 + e[1]! ** 2 + e[2]! ** 2;
      const s = Math.max(0, Math.min(1, ((p[0] - a.p[0]) * e[0]! + (p[1] - a.p[1]) * e[1]! + (p[2] - a.p[2]) * e[2]!) / (l2 || 1)));
      const d = Math.hypot(p[0] - a.p[0] - e[0]! * s, p[1] - a.p[1] - e[1]! * s, p[2] - a.p[2] - e[2]! * s);
      if (d < a.r + (b.r - a.r) * s - 1e-4) return true;
    }
    return false;
  });
  let total = 0;
  for (const tb of of) for (let i = 0; i + 1 < tb.points.length; i++) {
    const [a, b] = [tb.points[i]!, tb.points[i + 1]!];
    const axis = [b.p[0] - a.p[0], b.p[1] - a.p[1], b.p[2] - a.p[2]] as V3;
    const h = Math.hypot(...axis);
    if (h < 1e-6) continue;
    const z = axis.map((x) => x / h) as V3;
    const ref: V3 = Math.abs(z[1]) < 0.9 ? [0, 1, 0] : [1, 0, 0];
    const xd = [ref[1] * z[2] - ref[2] * z[1], ref[2] * z[0] - ref[0] * z[2], ref[0] * z[1] - ref[1] * z[0]] as V3;
    const xl = Math.hypot(...xd);
    const xx = xd.map((q) => q / xl) as V3;
    const yy: V3 = [z[1] * xx[2] - z[2] * xx[1], z[2] * xx[0] - z[0] * xx[2], z[0] * xx[1] - z[1] * xx[0]];
    const [na, nl] = [16, 6];
    const full = Math.PI * (a.r + b.r) * Math.hypot(h, a.r - b.r);
    let hit = 0;
    for (let j = 0; j < nl; j++) for (let k = 0; k < na; k++) {
      const s = (j + 0.5) / nl, ang = (2 * Math.PI * (k + 0.5)) / na;
      const r = a.r + (b.r - a.r) * s;
      const p: V3 = [0, 1, 2].map((q) => a.p[q]! + axis[q]! * s + r * (Math.cos(ang) * xx[q]! + Math.sin(ang) * yy[q]!)) as V3;
      if (p[1] < 0) continue;
      if (!inside(p, tb)) hit++;
    }
    total += (full * hit) / (na * nl);
  }
  return total;
}

/** ✏️ A shape from world rings of a view: elevation shapes are bone-local, front and plan shapes world rest coordinates. */
function shapeOf(base: Json, view: View, f: Frame, rings: P2[][], closed: boolean): Json | null {
  if (rings.length === 0 || rings[0]!.length < 2) return null;
  const local = view === "elevation" ? rings.map((ring) => ring.map((p) => {
    const l = toLocal(f, [p.x, p.y, f.o[2]]);
    return { x: l[0], y: l[1] };
  })) : rings;
  const d = local.map((ring) => ring.map((p, i) => `${i === 0 ? "M" : "L"} ${r4(p.x)} ${r4(p.y)}`).join(" ") + (closed ? " Z" : "")).join(" ");
  return { ...base, kind: "path", d };
}

const projectP = (view: View, p: V3): P2 => (view === "elevation" ? { x: p[0], y: p[1] } : view === "front" ? { x: p[2], y: p[1] } : { x: p[0], y: p[2] });

/** 🍁️ A palmate maple leaf outline of `size` (tip to tip) at c, turned by deg: `lobes` main lobes, `sharp` 0 (round) to 1 (acuminate), `sinus` depth of the sinuses (share of the radius). */
function leafRing(c: P2, size: number, deg: number, lobes: number, sharp: number, sinus: number): P2[] {
  const pts: P2[] = [];
  const spread = lobes === 3 ? 200 : lobes === 5 ? 260 : 290;
  const R = size / 2;
  const n = lobes * 6;
  for (let i = 0; i <= n; i++) {
    const s = i / n;
    const ang = -spread / 2 + spread * s;
    const phase = (s * lobes) % 1;
    const crest = 1 - sinus * (1 - Math.abs(Math.cos(Math.PI * phase)) ** (1 + 2 * sharp));
    const k = (0.55 + 0.45 * Math.cos((ang * Math.PI) / 360)) * crest;
    const a = (deg + 90 + ang) * D2R;
    pts.push({ x: c.x + R * k * Math.cos(a), y: c.y + R * k * Math.sin(a) });
  }
  const a0 = (deg - 90) * D2R;
  pts.push({ x: c.x + R * 0.12 * Math.cos(a0), y: c.y + R * 0.12 * Math.sin(a0) });
  return pts;
}

/** 🪽️ A pair of samaras as one open stroke: wing tip, nutlets, wing tip, with the species' angle between the wings. */
function samaraStroke(c: P2, span: number, angleDeg: number, hangDeg: number): P2[] {
  const half = (angleDeg / 2) * D2R, hang = (hangDeg - 90) * D2R;
  const L = span / 2 / Math.max(0.35, Math.sin(half));
  const wing = (sgn: number): P2[] => [0.35, 0.7, 1].map((f) => {
    const a = hang + sgn * half + sgn * 0.18 * (1 - f);
    return { x: c.x + L * f * Math.cos(a), y: c.y + L * f * Math.sin(a) };
  });
  return [...wing(-1).reverse(), c, ...wing(1)];
}

type Group = { id: string; bone: string; parts: (Lobe | Tube)[]; region?: string; regions?: string[]; material: string; tone: number; kind: "wood" | "foliage"; hatch?: string };

/** 🎨️ The drawings of one scale: one filled outline per wood bone and per foliage group, painted in depth order, plus lobe arcs (detail) and leaf, samara and bark marks (texture) placed only where their group is visible. */
function drawScale(d: Design, scale: Scale, wood: Tube[], lobes: Lobe[], F: Record<string, Frame>): { geometry: Json; counts: Record<string, number> } {
  const feature = featureOf(scale);
  const cell = feature / 3;
  const geometry: Json = {};
  const counts: Record<string, number> = { wood: 0, foliage: 0, arcs: 0, leaves: 0, samaras: 0, bark: 0 };
  const groups: Group[] = [];
  const woodBones = [...new Set(wood.map((w) => w.bone))];
  for (const b of woodBones) {
    const parts = wood.filter((w) => w.bone === b);
    const region = REGION[parts[0]!.group]!;
    groups.push({ id: b === "root-plate" ? "root-collar" : `wood-${b}`, bone: b, parts, region, material: MAT[parts[0]!.material]!, tone: parts[0]!.tone, kind: "wood" });
  }
  const crownBones = [...new Set(lobes.map((l) => l.bone))];
  for (const b of crownBones) {
    const mine = lobes.filter((l) => l.bone === b);
    if (scale === 500) groups.push({ id: "crown", bone: b, parts: mine, regions: ["leaf-sun", "leaf-shade"], material: MAT.sun!, tone: d.tree.tone.sun, kind: "foliage" });
    else {
      const shade = mine.every((l) => l.group === "shade");
      groups.push({ id: b, bone: b, parts: mine, region: shade ? "leaf-shade" : "leaf-sun", material: shade ? MAT.shade! : MAT.sun!, tone: shade ? d.tree.tone.shade : d.tree.tone.sun, kind: "foliage", ...(shade && scale <= 100 ? { hatch: "shade-lines" } : {}) });
    }
  }
  const pick = rng(d.tree.seed * 131 + scale);
  for (const view of VIEWS) {
    const centroid = (g: Group): number => {
      let s = 0, w = 0;
      for (const p of g.parts) {
        const c = p.kind === "lobe" ? p.c : p.points[Math.floor(p.points.length / 2)]!.p;
        const m = p.kind === "lobe" ? p.radii[0] * p.radii[1] * p.radii[2] : p.points[0]!.r ** 2;
        s += near(view, c) * m;
        w += m;
      }
      return s / (w || 1);
    };
    const rank = (g: Group): number => (g.bone === "root-plate" ? 0 : g.kind === "wood" ? (d.tree.axes.some((a) => a.id === g.bone) ? 1 : 2) : 3);
    const order = [...groups].sort((a, b) => rank(a) - rank(b) || centroid(a) - centroid(b));
    const shapes: Json[] = [];
    order.forEach((g, i) => {
      const rings = outlineOf(view, g.parts, cell, feature / 6, (1.5 * feature) ** 2, (3 * feature) ** 2);
      const base = { id: g.id, bone: g.bone, role: "body", z: r4(1 + i * 0.01), filled: true, tone: g.tone, ...(g.region ? { region: g.region } : { regions: g.regions }), material: g.material, ...(g.hatch ? { hatch: g.hatch } : {}) };
      const s = shapeOf(base, view, F[g.bone]!, rings, true);
      if (s) {
        shapes.push(s);
        counts[g.kind]!++;
      }
    });
    const painter = painterOf(view, order.map((g) => g.parts), Math.max(cell, feature / 2));
    const index = new Map(order.map((g, i) => [g, i]));
    const paper = (mm: number): number => (mm * scale) / 1000;
    const add = (base: Json, rings: P2[][], closed: boolean): void => {
      const s = shapeOf(base, view, F[base.bone]!, rings, closed);
      if (s) shapes.push(s);
    };
    if (scale !== 500) {
      const most = scale === 50 ? 5 : scale === 100 ? 3 : 2;
      for (const g of order.filter((x) => x.kind === "foliage")) {
        const gi = index.get(g)!;
        const big = (g.parts as Lobe[]).filter((l) => l.group !== "tuft").sort((a, b) => b.radii[0] * b.radii[2] - a.radii[0] * a.radii[2]);
        let n = 0;
        for (const l of big) {
          if (n >= most) break;
          const e = ellipseOf(view, l);
          if (Math.min(e.ra, e.rb) < paper(2.5)) continue;
          const arc: P2[] = [];
          for (let k = 0; k <= 9; k++) {
            const a = (95 + (k * 80) / 9) * D2R + (view === "plan" ? Math.atan2(e.cy, e.cx) - Math.PI / 2 : 0);
            const lx = 0.74 * e.ra * Math.cos(a - e.th), ly = 0.74 * e.rb * Math.sin(a - e.th);
            arc.push({ x: e.cx + lx * Math.cos(e.th) - ly * Math.sin(e.th), y: e.cy + lx * Math.sin(e.th) + ly * Math.cos(e.th) });
          }
          const clear = paper(0.3);
          if (!arc.every((p) => topAt(painter, p.x, p.y) === gi) || !clearIn(painter, gi, arc[0]!.x, arc[0]!.y, clear) || !clearIn(painter, gi, arc[9]!.x, arc[9]!.y, clear) || !clearIn(painter, gi, arc[4]!.x, arc[4]!.y, clear)) continue;
          add({ id: `${g.id}-arc${n + 1}`, bone: g.bone, role: "detail", z: r4(4 + gi * 0.01 + n * 0.0001), filled: false, tone: 0.3 }, [arc], false);
          n++;
          counts.arcs!++;
        }
      }
    }
    if (scale === 50 || scale === 100) {
      const trunkGroups = order.filter((g) => g.kind === "wood" && (g.bone === d.tree.axes[0]!.id || g.bone === "root-plate"));
      for (const g of trunkGroups) {
        const gi = index.get(g)!;
        const tb = g.parts.find((p) => p.kind === "tube" && p.id === g.bone) as Tube | undefined;
        if (!tb) continue;
        const marks = d.bark.marks(view, tb, scale, pick);
        let n = 0;
        for (const ring of marks) {
          const clear = paper(0.25);
          if (!ring.every((p) => topAt(painter, p.x, p.y) === gi) || !ring.filter((_, k) => k % 2 === 0).every((p) => clearIn(painter, gi, p.x, p.y, clear))) continue;
          add({ id: `${g.id}-bark${n + 1}`, bone: g.bone, role: "texture", z: r4(6 + gi * 0.01 + n * 0.0001), filled: false, tone: 0.2 }, [ring], d.bark.closed);
          n++;
          counts.bark!++;
        }
      }
    }
    if (scale === 50) {
      const L = d.leaf;
      for (const g of order.filter((x) => x.kind === "foliage" && x.region === "leaf-sun")) {
        const gi = index.get(g)!;
        let [nl, ns] = [0, 0];
        const box = (g.parts as Lobe[]).map((l) => ellipseOf(view, l)).reduce((b, e) => ({ x0: Math.min(b.x0, e.cx - e.ra), x1: Math.max(b.x1, e.cx + e.ra), y0: Math.min(b.y0, e.cy - e.ra), y1: Math.max(b.y1, e.cy + e.ra) }), { x0: Infinity, x1: -Infinity, y0: Infinity, y1: -Infinity });
        const step = paper(L.spacingMm);
        const cand: [number, number][] = [];
        for (let y = box.y1 - step / 2; y > box.y0; y -= step) for (let x = box.x0 + ((Math.round(y / step) % 2) * step) / 2; x < box.x1; x += step) cand.push([x + (pick() - 0.5) * step * 0.6, y + (pick() - 0.5) * step * 0.6]);
        for (let i = cand.length - 1; i > 0; i--) {
          const j = Math.floor(pick() * (i + 1));
          [cand[i], cand[j]] = [cand[j]!, cand[i]!];
        }
        const placedAt: [number, number][] = [];
        for (const [px, py] of cand) {
          if (nl >= L.most && (d.samara === undefined || ns >= d.samara.most)) break;
          const samara = d.samara !== undefined && pick() < d.samara.share;
          const size = samara ? d.samara!.span : L.size * (0.85 + pick() * 0.3);
          if (placedAt.some(([qx, qy]) => Math.hypot(qx - px, qy - py) < 1.6 * size + paper(0.3))) continue;
          if (!clearIn(painter, gi, px, py, size * 0.6 + paper(0.25))) continue;
          placedAt.push([px, py]);
          if (samara && ns < d.samara!.most) {
            add({ id: `${g.id}-samara${ns + 1}`, bone: g.bone, role: "texture", z: r4(7 + gi * 0.01 + ns * 0.0001), filled: false, tone: 0.25 }, [samaraStroke({ x: px, y: py }, size, d.samara!.angleDeg, (pick() - 0.5) * 30)], false);
            ns++;
            counts.samaras!++;
          } else if (!samara && nl < L.most) {
            add({ id: `${g.id}-leaf${nl + 1}`, bone: g.bone, role: "texture", z: r4(7 + gi * 0.01 + (L.most + nl) * 0.0001), filled: false, tone: 0.25 }, [leafRing({ x: px, y: py }, size, (pick() - 0.5) * 120 + (view === "plan" ? pick() * 360 : 0), L.lobes, L.sharp, L.sinus)], true);
            nl++;
            counts.leaves!++;
          }
        }
      }
    }
    geometry[view] = shapes;
  }
  return { geometry, counts };
}

/** 🧭️ The world-space extent of the body of one scale. */
function extentOf(wood: Tube[], lobes: Lobe[]): { min: V3; max: V3 } {
  const min: V3 = [Infinity, Infinity, Infinity], max: V3 = [-Infinity, -Infinity, -Infinity];
  for (const tb of wood) for (const q of tb.points) for (let k = 0; k < 3; k++) {
    min[k] = Math.min(min[k]!, q.p[k]! - q.r);
    max[k] = Math.max(max[k]!, q.p[k]! + q.r);
  }
  for (const l of lobes) for (let k = 0; k < 3; k++) {
    const e = Math.hypot(...l.axes.map((a, i) => a[k]! * l.radii[i]!));
    min[k] = Math.min(min[k]!, l.c[k]! - e);
    max[k] = Math.max(max[k]!, l.c[k]! + e);
  }
  min[1] = 0;
  return { min, max };
}

/** 🧭️ Renormalises the rotation of a solid after rounding, so it stays a unit quaternion within the validator's tolerance. */
function unitRotation(solid: Json): void {
  const q = solid.transform?.rotation as number[] | undefined;
  if (q === undefined) return;
  const n = Math.hypot(...q);
  solid.transform.rotation = q.map((x) => Math.round((x / n) * 1e6) / 1e6);
}

export type Built = { document: Json; manifest: Json; report: string[] };

export function build(d: Design): Built {
  const sp: Species = d.tree;
  const report: string[] = [];
  const wood50 = woodOf(sp, 50);
  const { bones, placed, foliageOrigins } = rigOf(d, wood50);
  const rigAsset = { rig: { kind: "botanical", root: "root-plate", bones } };
  const F = frames(rigAsset);
  for (const [id, p] of Object.entries(placed)) {
    const f = F[id]!;
    const err = Math.hypot(f.o[0] - p.o[0], f.o[1] - p.o[1], f.o[2] - p.o[2]);
    if (err > 2e-3) report.push(`[DEBUG] frame of ${id} off by ${err.toFixed(4)} m`);
  }
  const models = Object.fromEntries(SCALES.map((s) => [s, { wood: woodOf(sp, s), lobes: foliageOf(sp, s, foliageOrigins) }])) as Record<Scale, { wood: Tube[]; lobes: Lobe[] }>;
  const ext = extentOf(models[50].wood, models[50].lobes);
  for (const s of SCALES.slice(1)) for (const [k, sign] of [[0, 1], [0, -1], [2, 1], [2, -1]] as const) {
    const centre = k === 0 ? sp.crown.cx : 0;
    const target = sign > 0 ? ext.max[k]! : ext.min[k]!;
    for (let it = 0; it < 4; it++) {
      const side = models[s].lobes.filter((l) => sign * (l.c[k]! - centre) > 0);
      const reach = (l: Lobe): number => l.c[k]! + sign * Math.hypot(...l.axes.map((a, i) => a[k]! * l.radii[i]!));
      const far = side.reduce((best, l) => (sign * reach(l) > sign * reach(best) ? l : best), side[0]!);
      const e = reach(far) - far.c[k]!;
      const f = (target - e - centre) / (far.c[k]! - centre);
      if (!(f > 0.8 && f < 1.25)) break;
      for (const l of side) l.c[k] = centre + (l.c[k]! - centre) * f;
    }
  }
  const dims = { heightM: r3(ext.max[1]), widthM: r3(ext.max[0] - ext.min[0]), depthM: r3(ext.max[2] - ext.min[2]) };
  const crownTop = Math.max(...models[50].lobes.map((l) => l.c[1] + l.radii[1]));
  report.push(`1:50 extent x ${ext.min[0].toFixed(2)}..${ext.max[0].toFixed(2)} y 0..${ext.max[1].toFixed(2)} z ${ext.min[2].toFixed(2)}..${ext.max[2].toFixed(2)} (crown top ${crownTop.toFixed(2)})`);
  for (const s of SCALES.slice(1)) {
    const e = extentOf(models[s].wood, models[s].lobes);
    report.push(`1:${s} extent dx ${(e.max[0] - e.min[0] - dims.widthM).toFixed(3)} dy ${(e.max[1] - dims.heightM).toFixed(3)} dz ${(e.max[2] - e.min[2] - dims.depthM).toFixed(3)}`);
  }

  const density = d.wood.densityKgM3;
  const totalWood: Record<string, { v: number; c: V3 }> = {};
  for (const tb of models[50].wood) {
    const { v, c } = tubeVolume(tb);
    const e = totalWood[tb.bone] ?? { v: 0, c: [0, 0, 0] as V3 };
    const nv = e.v + v;
    totalWood[tb.bone] = { v: nv, c: [0, 1, 2].map((k) => (e.c[k]! * e.v + c[k]! * v) / nv) as V3 };
  }
  const planField = fieldOf(models[50].lobes.map((l) => primOf("plan", l)), 0.05);
  const crownPlanM2 = Array.from(planField.f).filter((x) => x < 0).length * 0.05 * 0.05;
  const leafAreaM2 = d.leaf.lai * crownPlanM2;
  const sunLobes = models[50].lobes.filter((l) => l.group !== "shade");
  const lobeArea = (l: Lobe): number => l.radii[0] * l.radii[2] + l.radii[1] * l.radii[2] + l.radii[0] * l.radii[1];
  const foliageShare: Record<string, number> = {};
  const sumSun = sunLobes.reduce((s, l) => s + lobeArea(l), 0);
  const shadeLobes = models[50].lobes.filter((l) => l.group === "shade");
  const sumShade = shadeLobes.reduce((s, l) => s + lobeArea(l), 0);
  for (const l of sunLobes) foliageShare[l.bone] = (foliageShare[l.bone] ?? 0) + (d.leaf.sunShare * lobeArea(l)) / sumSun;
  for (const l of shadeLobes) foliageShare[l.bone] = (foliageShare[l.bone] ?? 0) + ((1 - d.leaf.sunShare) * lobeArea(l)) / sumShade;
  const foliageCentroid: Record<string, V3> = {};
  for (const b of Object.keys(foliageShare)) {
    const mine = models[50].lobes.filter((l) => l.bone === b);
    const w = mine.reduce((s, l) => s + lobeArea(l), 0);
    foliageCentroid[b] = [0, 1, 2].map((k) => mine.reduce((s, l) => s + l.c[k]! * lobeArea(l), 0) / w) as V3;
  }
  let massKg = 0;
  for (const b of bones) {
    const f = F[b.id]!;
    if (totalWood[b.id]) {
      const { v, c } = totalWood[b.id]!;
      const kg = v * density;
      const loc = toLocal(f, c);
      b.mass = { kg: r2(kg), comX: r3(loc[0]), comY: r3(loc[1]), distribution: "point", source: "authored-estimate", reference: "wood-handbook-2021" };
      massKg += r2(kg);
      const tb = models[50].wood.find((w) => w.id === b.id) ?? models[50].wood.find((w) => w.bone === b.id)!;
      const r0 = b.id === "root-plate" ? tb.points.find((q) => q.p[1] >= 0.15)!.r : tb.points[0]!.r;
      b.section = { areaM2: r5(Math.PI * r0 * r0), secondMomentM4: Number(((Math.PI * r0 ** 4) / 4).toPrecision(4)), extremeFibreM: r4(r0), material: "maple-wood", title: t(`${b.title.en}, d = ${(2 * r0).toFixed(2)} m`, `${b.title.de}, d = ${deNum(2 * r0, 2)} m`) };
    } else if (foliageShare[b.id] !== undefined) {
      const kg = foliageShare[b.id]! * leafAreaM2 * d.leaf.massPerAreaKgM2;
      const loc = toLocal(f, foliageCentroid[b.id]!);
      b.mass = { kg: r2(kg), comX: r3(loc[0]), comY: r3(loc[1]), distribution: "point", source: "authored-estimate", reference: "jones-2014" };
      massKg += r2(kg);
    }
  }
  massKg = Math.round(massKg);
  report.push(`mass ${massKg} kg (wood ${Math.round(Object.values(totalWood).reduce((s, x) => s + x.v, 0) * density)} kg), crown plan ${crownPlanM2.toFixed(1)} m2, leaf area ${leafAreaM2.toFixed(0)} m2`);

  const trunkTubes = models[50].wood.filter((w) => REGION[w.group] === "trunk-bark");
  const limbTubes = models[50].wood.filter((w) => REGION[w.group] === "limb-bark");
  const trunkArea = exposedBark(models[50].wood, trunkTubes);
  const limbArea = exposedBark(models[50].wood, limbTubes);
  report.push(`bark trunk ${trunkArea.toFixed(2)} m2, limbs ${limbArea.toFixed(2)} m2`);

  const anchors: Json[] = [{ id: "base-support", bone: "root-plate", x: 0, y: 0, kind: "support", title: t("Root plate anchorage", "Verankerung des Wurzeltellers"), support: "fixed", normalDeg: 90, footprintLengthM: r2(d.rootPlateM), footprintWidthM: r2(d.rootPlateM), frictionCoefficient: 0.6 }];
  const outlets: { emitter: string; anchor: string; group: string; p: V3; bone: string }[] = [];
  const surfaceOf = (l: Lobe, dir: V3): V3 => {
    const q = Math.hypot(...l.axes.map((a, k) => (a[0] * dir[0] + a[1] * dir[1] + a[2] * dir[2]) / l.radii[k]!));
    return [l.c[0] + dir[0] / q, l.c[1] + dir[1] / q, l.c[2] + dir[2] / q];
  };
  const sunBones = Object.keys(foliageShare).filter((b) => !b.startsWith("fol-core"));
  const chosenSun = sunBones.filter((_, i) => i % Math.max(1, Math.floor(sunBones.length / 7)) === 0).slice(0, 7);
  while (chosenSun.length < 5) chosenSun.push(sunBones[chosenSun.length]!);
  chosenSun.forEach((b, i) => {
    const mine = models[50].lobes.filter((l) => l.bone === b && l.group === "sun").sort((p, q) => p.c[1] - q.c[1]);
    const l = mine[Math.floor(mine.length * 0.6)]!;
    const az = Math.atan2(l.c[2], l.c[0] - sp.crown.cx) / D2R;
    const dir = dirOf(az, 40);
    const p = surfaceOf(l, dir);
    const at: V3 = [p[0] + 0.04 * dir[0], p[1] + 0.04 * dir[1], p[2] + 0.04 * dir[2]];
    const loc = toLocal(F[b]!, at);
    anchors.push({ id: `tp-sun-${i + 1}`, bone: b, x: r4(loc[0]), y: r4(loc[1]), z: r4(loc[2]), kind: "emitter", title: t(`Transpiration of the sun leaves, anchor ${i + 1}`, `Transpiration der Sonnenblätter, Anker ${i + 1}`) });
    outlets.push({ emitter: "transpiration-sun", anchor: `tp-sun-${i + 1}`, group: "sun", p, bone: b });
  });
  shadeLobes.filter((_, i) => i % Math.max(1, Math.floor(shadeLobes.length / 5)) === 0).slice(0, 5).forEach((l, i) => {
    const az = Math.atan2(l.c[2], l.c[0] - sp.crown.cx) / D2R;
    const dir = dirOf(az, 20);
    const p = surfaceOf(l, dir);
    const at: V3 = [p[0] + 0.04 * dir[0], p[1] + 0.04 * dir[1], p[2] + 0.04 * dir[2]];
    const loc = toLocal(F[l.bone]!, at);
    anchors.push({ id: `tp-shade-${i + 1}`, bone: l.bone, x: r4(loc[0]), y: r4(loc[1]), z: r4(loc[2]), kind: "emitter", title: t(`Transpiration of the shade leaves, anchor ${i + 1}`, `Transpiration der Schattenblätter, Anker ${i + 1}`) });
    outlets.push({ emitter: "transpiration-shade", anchor: `tp-shade-${i + 1}`, group: "shade", p, bone: l.bone });
  });

  const volumes: Json = {};
  const reps: Json = {};
  const drawnCounts: Record<number, Record<string, number>> = {};
  for (const s of SCALES) {
    const m = models[s];
    const feature = featureOf(s);
    const solid = (p: Tube | Lobe): Json => ({ ...solidOf({ ...p, material: MAT[p.material]! } as Tube | Lobe, F) });
    const body: Json[] = [...m.wood.map(solid), ...m.lobes.map(solid)];
    const thermal: Json[] = [...m.wood, ...m.lobes].map((p) => ({ id: `th-${p.id}`, of: p.id, region: REGION[p.group]! }));
    const keepOutlets = s === 50 || s === 100 ? outlets : s === 200 ? outlets.filter((_, i) => i % 2 === 0) : [outlets[0]!, outlets.find((o) => o.group === "shade")!];
    for (const o of keepOutlets) {
      const cands = m.lobes.filter((l) => (o.group === "shade" ? l.group === "shade" : l.group !== "shade"));
      const l = cands.sort((a, b) => Math.hypot(a.c[0] - o.p[0], a.c[1] - o.p[1], a.c[2] - o.p[2]) - Math.hypot(b.c[0] - o.p[0], b.c[1] - o.p[1], b.c[2] - o.p[2]))[0]!;
      const dv: V3 = [o.p[0] - l.c[0], o.p[1] - l.c[1], o.p[2] - l.c[2]];
      const dl = Math.hypot(...dv);
      const dir = dv.map((x) => x / dl) as V3;
      const p = surfaceOf(l, dir);
      const q = [p[0] - l.c[0], p[1] - l.c[1], p[2] - l.c[2]];
      const nrm0 = l.axes.reduce((acc, ax, k) => [0, 1, 2].map((j) => acc[j]! + (ax[j]! * (ax[0] * q[0]! + ax[1] * q[1]! + ax[2] * q[2]!)) / l.radii[k]! ** 2) as V3, [0, 0, 0] as V3);
      const nl = Math.hypot(...nrm0);
      const n = nrm0.map((x) => x / nl) as V3;
      const side: V3 = Math.abs(n[1]) > 0.98 ? [1, 0, 0] : [n[2], 0, -n[0]].map((x) => x / Math.hypot(n[2], n[0])) as V3;
      const third: V3 = [side[1] * n[2] - side[2] * n[1], side[2] * n[0] - side[0] * n[2], side[0] * n[1] - side[1] * n[0]];
      const rho = Math.max(feature * 1.2, Math.min(l.radii[0], l.radii[2]) * (s === 500 ? 0.4 : 0.5));
      const h = Math.max(feature * 1.1, 0.06);
      const c: V3 = [p[0] - n[0] * (h + 0.003), p[1] - n[1] * (h + 0.003), p[2] - n[2] * (h + 0.003)];
      const lens: Lobe = { kind: "lobe", id: `th-${o.anchor}`, bone: l.bone, material: l.material, tone: l.tone, group: l.group, c, axes: [side, n, third], radii: [rho, h, rho] };
      thermal.push({ ...solid(lens), role: "detail", region: REGION[l.group]!, tags: [o.emitter] });
    }
    const structure: Json[] = [];
    const bark = (r: number): number => 0.004 + 0.05 * r;
    for (const tb of m.wood.filter((w) => w.id === w.bone || (w.bone === "root-plate" && w.id === "root-plate"))) {
      const pts = tb.points.map((q) => ({ p: q.p, r: Math.max(feature * 0.56, q.r - bark(q.r)) }));
      structure.push({ ...solid({ ...tb, id: `member-${tb.bone}`, material: "wood", tone: 0.4, points: pts } as Tube), material: "maple-wood", member: tb.bone, part: "member" });
    }
    for (const b of bones) {
      if (b.parent === null || !b.section) continue;
      const tb = m.wood.find((w) => w.id === b.id);
      if (!tb) continue;
      const r = Math.min(Math.max(feature * 0.6, (tb.points[0]!.r - bark(tb.points[0]!.r)) * 0.95), Math.max(feature * 0.6, F[b.id]!.o[1] - 0.02));
      if (s === 500 && !sp.axes.some((a) => a.id === b.id)) continue;
      structure.push({ id: `joint-${b.id}`, kind: "ellipsoid", bone: b.id, material: "maple-wood", role: "detail", tone: 0.35, radii: [r5(r), r5(r), r5(r)], transform: { translation: [0, 0, 0] }, member: b.id, part: "joint" });
    }
    const plate = Math.max(0.05, feature);
    structure.push({ id: "support-base-support", kind: "revolve", bone: "root-plate", material: "maple-wood", role: "detail", tone: 0.25, profile: [[r5(-plate), r3(d.rootPlateM / 2)], [0, r3(d.rootPlateM / 2)]], axis: [1, 0, 0], origin: [0, 0, 0], member: "root-plate", part: "support", anchor: "base-support" });
    const totalMass = bones.reduce((acc, b) => acc + (b.mass?.kg ?? 0), 0);
    for (const b of bones) {
      if (!b.mass) continue;
      const isWood = totalWood[b.id] !== undefined;
      if (isWood && b.mass.kg < 0.02 * totalMass) continue;
      if (!isWood && s === 500) continue;
      const f = F[b.id]!;
      const c = isWood ? totalWood[b.id]!.c : foliageCentroid[b.id]!;
      const loc = toLocal(f, c);
      let radii: V3;
      if (isWood) {
        const tb = m.wood.find((w) => w.id === b.id) ?? m.wood.find((w) => w.bone === b.id)!;
        const rr = Math.max(feature * 0.6, 0.55 * tb.points[Math.floor(tb.points.length / 2)]!.r);
        radii = sp.axes.some((a) => a.id === b.id) || b.id === "root-plate" ? [Math.max(rr, Math.min(0.25 * b.lengthM, 3 * rr)), rr, rr] : [rr, rr, rr];
      } else {
        const rr = Math.max(feature * 0.6, Math.min(0.12, Math.cbrt((3 * b.mass.kg) / (4 * Math.PI * 600))));
        radii = [rr, rr, rr];
      }
      structure.push({ id: `mass-${b.id}`, kind: "ellipsoid", bone: b.id, material: isWood ? "maple-wood" : MAT.sun, role: "detail", tone: 0.3, radii: radii.map(r5), transform: { translation: loc.map(r5) }, member: isWood ? b.id : b.parent, part: "mass" });
    }
    volumes[String(s)] = { status: "authored", denominator: s, body, thermal, structure, decisions: d.texts.volume(s, { wood: m.wood.length, lobes: m.lobes.length, tufts: m.lobes.filter((l) => l.group === "tuft").length, sides: m.wood.filter((w) => w.group === "side").length, outlets: keepOutlets.length }) };
    const { geometry, counts } = drawScale(d, s, m.wood, m.lobes, F);
    drawnCounts[s] = counts;
    reps[String(s)] = {
      status: "authored", denominator: s,
      validRange: { 500: { minDenominator: 350, maxDenominator: 1000 }, 200: { minDenominator: 150, maxDenominator: 350 }, 100: { minDenominator: 75, maxDenominator: 150 }, 50: { minDenominator: 20, maxDenominator: 75 } }[s],
      pens: { 500: { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 }, 200: { body: 0.2, detail: 0.13, texture: null, glazing: null, ground: 0.25 }, 100: { body: 0.25, detail: 0.13, texture: 0.1, glazing: null, ground: 0.3 }, 50: { body: 0.3, detail: 0.18, texture: 0.13, glazing: null, ground: 0.35 } }[s],
      hatches: s <= 100 ? [{ id: "shade-lines", title: t("Shade line hatch", "Schattenschraffur"), pattern: "lines", angleDeg: 45, spacingMm: s === 50 ? 0.8 : 0.7, penMm: 0.1 }] : [],
      geometry,
      decisions: d.texts.drawing(s, counts),
    };
    report.push(`1:${s} body ${body.length} thermal ${thermal.length} structure ${structure.length} | drawn ${JSON.stringify(counts)}`);
  }

  const crown500 = models[500].lobes;
  const symbolGeometry: Json = {};
  for (const view of VIEWS) {
    const rings = outlineOf(view, crown500, 0.05, 0.03, 1, 4);
    const crownRing = rings.filter((r) => areaOf(r) > 0).sort((a, b) => areaOf(b) - areaOf(a))[0]!;
    const shapes: Json[] = [{ id: "crown", bone: "root-plate", role: "body", z: 2, filled: false, tone: 0.5, kind: "poly", points: crownRing.map((p) => [r4(p.x), r4(p.y)]), closed: true }];
    if (view === "plan") {
      const r = models[50].wood.find((w) => w.id === sp.axes[0]!.id)!.points[0]!.r;
      shapes.push({ id: "stem", bone: "root-plate", role: "body", z: 4, filled: true, tone: 0.25, kind: "poly", points: Array.from({ length: 24 }, (_, i) => [r4(r * Math.cos((i * Math.PI) / 12)), r4(r * Math.sin((i * Math.PI) / 12))]), closed: true });
      sp.limbs.filter((l) => l.parent === sp.axes[1]?.id || l.parent === sp.axes[0]!.id).slice(0, 6).forEach((l, i) => {
        const tip = models[200].wood.find((w) => w.id === l.id)!.points.at(-1)!.p;
        const base = models[200].wood.find((w) => w.id === l.id)!.points[0]!.p;
        shapes.push({ id: `scaffold-${i + 1}`, bone: "root-plate", role: "detail", z: 3, filled: false, tone: 0.35, kind: "poly", points: [[r4(base[0]), r4(base[2])], [r4(tip[0]), r4(tip[2])]], closed: false });
      });
    } else {
      const stem = outlineOf(view, models[500].wood.filter((w) => w.id === "root-plate" || w.id === sp.axes[0]!.id), 0.025, 0.012, 0.01, 1)[0]!;
      shapes.push({ id: "stem", bone: "root-plate", role: "body", z: 3, filled: true, tone: 0.3, kind: "poly", points: stem.map((p) => [r4(p.x), r4(p.y)]), closed: true });
    }
    symbolGeometry[view] = shapes;
  }
  const H = dims.heightM;
  const signY = r3(H * 0.62);
  const circle = (cx: number, cy: number, r: number): string => {
    const k = 0.5523 * r;
    return `M${r4(cx + r)} ${r4(cy)}C${r4(cx + r)} ${r4(cy + k)} ${r4(cx + k)} ${r4(cy + r)} ${r4(cx)} ${r4(cy + r)}C${r4(cx - k)} ${r4(cy + r)} ${r4(cx - r)} ${r4(cy + k)} ${r4(cx - r)} ${r4(cy)}C${r4(cx - r)} ${r4(cy - k)} ${r4(cx - k)} ${r4(cy - r)} ${r4(cx)} ${r4(cy - r)}C${r4(cx + k)} ${r4(cy - r)} ${r4(cx + r)} ${r4(cy - k)} ${r4(cx + r)} ${r4(cy)}Z`;
  };
  const sign = (cy: number, stem: boolean): Json[] => [
    { id: "sign-circle", bone: "root-plate", role: "body", z: 1, filled: false, tone: 0.5, kind: "path", d: circle(0, cy, 1.6) },
    { id: "sign-dot", bone: "root-plate", role: "body", z: 2, filled: true, tone: 0.3, kind: "path", d: circle(0, cy, 0.35) },
    ...(stem ? [{ id: "sign-stem", bone: "root-plate", role: "body", z: 1, filled: false, tone: 0.3, kind: "path", d: `M0 0L0 ${r4(cy - 1.6)}` }] : []),
  ];
  void signY;
  const dimNote = t(`The crown outline and the stem are drawn at their true extent of ${dims.widthM.toFixed(2)} × ${dims.depthM.toFixed(2)} × ${dims.heightM.toFixed(2)} m in width, depth and height.`, `Kronenumriss und Stamm sind in ihrer tatsächlichen Ausdehnung von ${deNum(dims.widthM, 2)} × ${deNum(dims.depthM, 2)} × ${deNum(dims.heightM, 2)} m in Breite, Tiefe und Höhe gezeichnet.`);
  const symbols = [
    { id: `${sp.id}-plan-crown`, title: t("Plan symbol with true crown extent", "Grundrisssymbol mit wahrer Kronenausdehnung"), description: d.texts.symbol, semantics: "true-extent", units: "model-m", denominators: [500, 200, 100], pens: { body: 0.2, detail: 0.13, texture: null, glazing: null, ground: 0.3 }, hatches: [], geometry: symbolGeometry, dimensionalNote: dimNote },
    { id: "tree-sign", title: t("Nominal tree sign with stem centre", "Nominelles Baumzeichen mit Stammmitte"), description: t("The conventional circle-and-dot tree sign of small-scale plans: the dot marks the stem, the circle is a fixed paper size.", "Das übliche Kreis-und-Punkt-Baumzeichen kleinmaßstäblicher Pläne: Der Punkt markiert den Stamm, der Kreis hat eine feste Papiergröße."), semantics: "nominal", units: "paper-mm", denominators: [500, 200], pens: { body: 0.15, detail: null, texture: null, glazing: null, ground: 0.25 }, hatches: [], geometry: { elevation: sign(3.2, true), front: sign(3.2, true), plan: sign(0, false) }, dimensionalNote: t("A fixed paper sign: the circle is 3.2 mm across on paper whatever the scale; it carries no dimension of the tree.", "Ein festes Papierzeichen: Der Kreis misst auf dem Papier 3,2 mm, unabhängig vom Maßstab; er trägt kein Maß des Baums.") },
  ];

  const frontField = (list: Lobe[]): { area: number; y: number } => {
    const fd = fieldOf(list.map((l) => primOf("front", l)), 0.05, 0);
    let n = 0, sy = 0;
    for (let j = 0; j < fd.ny; j++) for (let i = 0; i < fd.nx; i++) if (fd.f[j * fd.nx + i]! < 0) {
      n++;
      sy += fd.y0 + (j + 0.5) * fd.cell;
    }
    return { area: n * 0.0025, y: n ? sy / n : 0 };
  };
  const whole = frontField(models[50].lobes);
  const perBone: Record<string, { area: number; y: number }> = {};
  for (const b of Object.keys(foliageShare)) perBone[b] = frontField(models[50].lobes.filter((l) => l.bone === b));
  const sumA = Object.values(perBone).reduce((s, x) => s + x.area, 0);
  for (const x of Object.values(perBone)) x.area *= whole.area / sumA;
  report.push(`frontal crown area ${whole.area.toFixed(1)} m2`);
  const trunkTube = models[50].wood.find((w) => w.id === sp.axes[0]!.id)!;
  const trunkBone = bones.find((b) => b.id === sp.axes[0]!.id)!;
  const trunkD = 2 * trunkTube.points[Math.floor(trunkTube.points.length / 2)]!.r;
  const vAt = (v10: number, z: number): number => v10 * (Math.max(1, z) / 10) ** 0.22;
  const cdOf = (v: number): number => (v <= 8 ? d.wind.cd : d.wind.cd * (v / 8) ** -0.6);
  const loadsOf = (p: PoseSpec, factor = 1): Json[] => {
    const out: Json[] = [];
    for (const [b, a] of Object.entries(perBone)) {
      const loc = toLocal(F[b]!, foliageCentroid[b]!);
      const v = vAt(p.v10, a.y);
      const fx = 0.5 * RHO * cdOf(p.v10) * a.area * (1 - d.wind.porosity) * v * v * factor;
      const fy = p.extra ? -(p.extra.basis === "plan" ? foliageShare[b]! * crownPlanM2 : foliageShare[b]! * leafAreaM2) * p.extra.kgPerM2 * G : 0;
      const title = b.startsWith("fol-core") ? bones.find((x) => x.id === b)!.title : bones.find((x) => x.id === b)!.title;
      out.push({ id: `wind-${b}`, kind: "wind", title: t(`Drag on ${title.en.toLowerCase()}`, `Winddruck: ${title.de}`), bone: b, x: r3(loc[0]), y: r3(loc[1]), fxN: r2(fx), fyN: 0, source: "literature", reference: "en-1991-1-4" });
      if (p.extra) out.push({ id: `${p.extra.kind}-${b}`, kind: p.extra.load, title: t(`${p.extra.title.en} on ${title.en.toLowerCase()}`, `${p.extra.title.de}: ${title.de}`), bone: b, x: r3(loc[0]), y: r3(loc[1]), fxN: 0, fyN: r2(fy), source: p.extra.source, reference: p.extra.reference });
    }
    const vt = vAt(p.v10, trunkBone.lengthM / 2 + 0.3);
    out.push({ id: "wind-stem", kind: "wind", title: t("Drag on the stem", "Winddruck am Stamm"), fxN: 0, fyN: 0, distributed: { bone: trunkBone.id, fromM: 0, toM: trunkBone.lengthM, wxNPerM: r2(0.5 * RHO * 1.0 * trunkD * vt * vt * factor), wyNPerM: 0 }, source: "literature", reference: "en-1991-1-4" });
    return out;
  };
  const levelOf = (b: Json): number => (b.id.startsWith("fol-") ? 3 : sp.limbs.some((l) => l.id === b.id) ? 2 : sp.axes.some((a, i) => a.id === b.id && i > 0) ? 1 : b.id === sp.axes[0]!.id ? 0 : -1);
  const offsetsOf = (p: PoseSpec, scale = 1): Json => {
    const out: Json = {};
    for (const b of bones) {
      const lv = levelOf(b);
      if (lv < 0) continue;
      let a = p.lean[lv]! * scale;
      if (p.droop && (lv === 2 || lv === 3)) {
        const th = (b.id.startsWith("fol-") ? placed[b.id]!.th : placed[b.id]!.th) * D2R;
        const outward = Math.cos(th);
        a += -Math.sign(outward || 1) * p.droop[lv - 2]! * Math.min(1, Math.abs(outward) * 2 + 0.2);
      }
      if (Math.abs(a) >= 0.005) out[b.id] = { angleDeg: r2(a) };
    }
    return out;
  };
  const regionsOf = (p: PoseSpec): Json => ({
    "trunk-bark": { exposedFraction: 1, sunlitFraction: p.sun[0], ...(p.wet ? { wettedness: p.wet } : {}) },
    "limb-bark": { exposedFraction: 1, sunlitFraction: p.sun[1], ...(p.wet ? { wettedness: p.wet } : {}) },
    "leaf-sun": { exposedFraction: 1, sunlitFraction: p.sun[2], ...(p.wet ? { wettedness: p.wet } : {}) },
    "leaf-shade": { exposedFraction: 1, sunlitFraction: p.sun[3], ...(p.wet ? { wettedness: p.wet } : {}) },
  });
  const poses = d.poses.map((p) => ({ id: p.id, title: p.title, description: p.description, bones: offsetsOf(p), ground: "authored", contacts: [], thermal: { regions: regionsOf(p), emitters: { "transpiration-sun": p.transp[0], "transpiration-shade": p.transp[1] } }, loads: loadsOf(p) }));
  for (const p of poses) for (const [b, o] of Object.entries(p.bones) as [string, Json][]) {
    const j = bones.find((x) => x.id === b)!.joint;
    if (Math.abs(o.angleDeg) > j.maxDeg) report.push(`[DEBUG] pose ${p.id} bone ${b} ${o.angleDeg} beyond ${j.maxDeg}`);
  }

  const breeze = d.poses.find((p) => p.id === "summer-breeze")!;
  const T = d.wind.periodS;
  const amp = [0.55, 0.65, 0.8, 1.0], lag = [0, 0.35, 0.7, 1.05];
  const swayKeys = Array.from({ length: 9 }, (_, k) => {
    const tk = (T * k) / 8;
    const bonesK: Json = {};
    for (const b of bones) {
      const lv = levelOf(b);
      if (lv < 0) continue;
      const base = breeze.lean[lv]!;
      const a = base * (1 + amp[lv]! * Math.sin((2 * Math.PI * k) / 8 - lag[lv]!));
      bonesK[b.id] = { angleDeg: r2(a) };
    }
    return { t: r3(tk), pose: "summer-breeze", ease: "smooth", bones: bonesK };
  });
  swayKeys[8]!.bones = swayKeys[0]!.bones;
  const gustV = d.wind.gustPeak;
  const gf = (v: number): number => ((v / breeze.v10) ** 2 * cdOf(v)) / cdOf(breeze.v10);
  const gustProfile: [number, number][] = [[0, breeze.v10], [1.2, breeze.v10 * 1.12], [2.6, gustV], [3.8, breeze.v10 + 0.72 * (gustV - breeze.v10)], [5.4, breeze.v10 * 1.15], [8, breeze.v10]];
  const lagS = [0.45, 0.3, 0.15, 0];
  const gustKeys = gustProfile.map(([tk]) => {
    const bonesK: Json = {};
    for (const b of bones) {
      const lv = levelOf(b);
      if (lv < 0) continue;
      const tt = Math.max(0, tk - lagS[lv]!);
      let v = breeze.v10;
      for (let i = 0; i + 1 < gustProfile.length; i++) if (tt >= gustProfile[i]![0] && tt <= gustProfile[i + 1]![0]) v = gustProfile[i]![1] + ((gustProfile[i + 1]![1] - gustProfile[i]![1]) * (tt - gustProfile[i]![0])) / (gustProfile[i + 1]![0] - gustProfile[i]![0]);
      bonesK[b.id] = { angleDeg: r2(breeze.lean[lv]! * gf(v)) };
    }
    return { t: tk, pose: "summer-breeze", ease: "smooth", bones: bonesK };
  });
  const breezeLoads = loadsOf(breeze);
  const gustTracks = breezeLoads.map((l) => ({ target: `load:${l.id}`, ease: "smooth", keys: gustProfile.map(([tk, v]) => ({ t: tk, v: r3(l.id === "wind-stem" ? (v / breeze.v10) ** 2 : gf(v)) })) }));
  const calm = d.poses.find((p) => p.id === "summer-calm")!;
  const sunCurve: [number, number][] = [[0, 0], [5.5, 0], [7, 0.25], [9, 0.65], [12, 1], [15, 0.7], [17.5, 0.2], [19, 0], [24, 0]];
  const transCurve: [number, number][] = [[0, 0.03], [5, 0.05], [7, 0.35], [9, 0.85], [11.5, 1], [14, 0.72], [16, 0.76], [18, 0.3], [20, 0.08], [24, 0.03]];
  const clips = [
    {
      id: "wind-sway", title: t("Wind sway", "Schwanken im Wind"), description: d.texts.sway, durationS: T, fps: 30, loop: true, rootMotion: "none", keys: swayKeys, contacts: [],
      phases: [{ id: "downwind", fromS: 0, toS: r3(T / 2), title: t("Downwind swing", "Schwung in Windrichtung") }, { id: "upwind", fromS: r3(T / 2), toS: T, title: t("Return swing", "Rückschwung") }], events: [], tracks: [],
    },
    {
      id: "gust", title: t("Gust", "Windböe"), description: d.texts.gust, durationS: 8, fps: 30, loop: false, rootMotion: "none", keys: gustKeys, contacts: [],
      phases: [{ id: "rise", fromS: 0, toS: 2.6, title: t("Gust front", "Böenfront") }, { id: "peak", fromS: 2.6, toS: 3.8, title: t("Peak", "Spitze") }, { id: "decay", fromS: 3.8, toS: 8, title: t("Decay and recovery", "Abklingen und Erholung") }],
      events: [{ t: 2.6, kind: "load-on", title: t("Peak gust load", "Böenspitze") }, { t: 5.4, kind: "load-off", title: t("Load back near the mean", "Last wieder nahe dem Mittelwert") }], tracks: gustTracks,
    },
    {
      id: "diurnal-transpiration", title: t("Day of transpiration", "Transpirationstag"), description: d.texts.diurnal, durationS: 24, fps: 10, loop: true, rootMotion: "none",
      keys: [{ t: 0, pose: "summer-calm", ease: "smooth" }, { t: 12, pose: "summer-calm", ease: "smooth" }, { t: 24, pose: "summer-calm", ease: "smooth" }], contacts: [],
      phases: [{ id: "night", fromS: 0, toS: 6, title: t("Night", "Nacht") }, { id: "morning", fromS: 6, toS: 12, title: t("Morning to noon", "Morgen bis Mittag") }, { id: "afternoon", fromS: 12, toS: 18, title: t("Afternoon", "Nachmittag") }, { id: "evening", fromS: 18, toS: 24, title: t("Evening and night", "Abend und Nacht") }],
      events: [{ t: 6, kind: "thermal", title: t("Sunrise", "Sonnenaufgang") }, { t: 12, kind: "thermal", title: t("Solar noon", "Sonnenhöchststand") }, { t: 18, kind: "thermal", title: t("Sunset", "Sonnenuntergang") }],
      tracks: [
        { target: "emitter:transpiration-sun", ease: "smooth", keys: transCurve.map(([tk, v]) => ({ t: tk, v: r3(v) })) },
        { target: "emitter:transpiration-shade", ease: "smooth", keys: transCurve.map(([tk, v]) => ({ t: tk, v: r3(v * 0.7) })) },
        ...(["trunk-bark", "limb-bark", "leaf-sun", "leaf-shade"] as const).map((r, i) => ({ target: `region:${r}:sunlitFraction`, ease: "smooth", keys: sunCurve.map(([tk, v]) => ({ t: tk, v: r3(v * calm.sun[i]!) })) })),
      ],
    },
  ];

  const thermal = {
    environment: d.environment,
    regions: [
      { id: "trunk-bark", title: d.texts.regions.trunk, kind: "bark", model: "passive", material: "maple-bark", areaM2: r2(trunkArea * 1.12), characteristicLengthM: r3(trunkD), geometry: "cylinder", skyViewFactor: 0.5, layers: [], heatCapacityJM2K: d.wood.trunkHeatCapacity, source: "authored-estimate", reference: "wood-handbook-2021" },
      { id: "limb-bark", title: d.texts.regions.limb, kind: "bark", model: "passive", material: "maple-bark-young", areaM2: r2(limbArea), characteristicLengthM: r3(d.wood.limbLengthM), geometry: "cylinder", skyViewFactor: 0.45, layers: [], heatCapacityJM2K: d.wood.limbHeatCapacity, source: "authored-estimate", reference: "wood-handbook-2021" },
      { id: "leaf-sun", title: d.texts.regions.sun, kind: "foliage", model: "leaf", material: "maple-leaf-sun", areaM2: r2(leafAreaM2 * d.leaf.sunShare), areaBasis: "exchange", characteristicLengthM: d.leaf.lengthM, geometry: "horizontal-plate-up", skyViewFactor: 0.75, layers: [], heatCapacityJM2K: 800, leaf: { stomatalConductanceMaxMS: d.leaf.gsSun, leafAreaIndex: r2(d.leaf.lai * d.leaf.sunShare), photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
      { id: "leaf-shade", title: d.texts.regions.shade, kind: "foliage", model: "leaf", material: "maple-leaf-shade", areaM2: r2(leafAreaM2 * (1 - d.leaf.sunShare)), areaBasis: "exchange", characteristicLengthM: r3(d.leaf.lengthM * 1.15), geometry: "horizontal-plate-up", skyViewFactor: 0.25, layers: [], heatCapacityJM2K: 600, leaf: { stomatalConductanceMaxMS: d.leaf.gsShade, leafAreaIndex: r2(d.leaf.lai * (1 - d.leaf.sunShare)), photosynthesis: "c3" }, source: "authored-estimate", reference: "jones-2014" },
    ],
    emitters: [
      { id: "transpiration-sun", kind: "transpiration", title: t("Transpiration of the sun leaves", "Transpiration der Sonnenblätter"), anchor: "tp-sun-1", anchors: anchors.filter((a) => a.id.startsWith("tp-sun")).map((a) => a.id), directionDeg: r3(90 - placed[anchors.find((a) => a.id === "tp-sun-1")!.bone]!.th), diameterM: r2(Math.max(1, sp.crown.rx * 0.22)), region: "leaf-sun", source: "authored-estimate", reference: "monteith-unsworth-2013" },
      { id: "transpiration-shade", kind: "transpiration", title: t("Transpiration of the shade leaves", "Transpiration der Schattenblätter"), anchor: "tp-shade-1", anchors: anchors.filter((a) => a.id.startsWith("tp-shade")).map((a) => a.id), directionDeg: r3(90 - placed[anchors.find((a) => a.id === "tp-shade-1")!.bone]!.th), diameterM: r2(Math.max(0.8, sp.crown.rx * 0.16)), region: "leaf-shade", source: "authored-estimate", reference: "monteith-unsworth-2013" },
    ],
    passengers: [],
    assumptions: d.texts.thermalAssumptions,
    operating: [],
  };

  const asset = {
    id: sp.id, category: "vegetation", title: sp.title, summary: sp.summary, tags: sp.tags,
    provenance: { author: "Semio staffage plant collection worker PW-01", method: "handcrafted", created: "2026-11-22", references: d.references, notes: sp.notes },
    dimensions: { ...dims, massKg },
    materials: d.materials,
    rig: { kind: "botanical", root: "root-plate", bones },
    anchors,
    representations: reps,
    symbols,
    poses,
    clips,
    thermal,
    structure: { gravityMS2: G, dynamics: "quasi-static", supports: ["base-support"], assumptions: d.texts.structureAssumptions, clinical: false },
    volumes,
    presentation: { defaultPose: "summer-calm", defaultClip: "wind-sway", defaultScale: 200 },
    exports: ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    completion: "authored",
  };
  for (const volume of Object.values(volumes) as Json[]) for (const list of ["body", "thermal", "structure"]) for (const solid of volume[list] as Json[]) unitRotation(solid);
  const document = { $schema: "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", version: "semio.staffage/v2", asset };
  const manifest = {
    id: sp.id, category: "vegetation", title: sp.title, heightM: dims.heightM, widthM: dims.widthM, powerW: 0,
    poses: poses.map((p) => p.id), clips: clips.map((c) => c.id),
    representations: { 50: "authored", 100: "authored", 200: "authored", 500: "authored" },
    symbols: symbols.map((s) => s.id), analytics: ["thermal", "transport", "structural"], exports: asset.exports, distinction: sp.distinction, completion: "authored",
  };
  void quaternion; void dirToLocal; void contours; void painterOf;
  return { document, manifest, report };
}

export type { Painter };
