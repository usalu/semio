/** 🌿️ Shared helpers of PW-13 (boxwood ball, privet hedge, cherry laurel): leafy relief skins as closed polyhedra, their true outlines in the three views, leaf-mark texture, wind physics and the clip builders. The hornbeam-hedge kit (tubes, panels, flares, outlines) is imported from its place, never copied.
 * @see ../../📋️brief.md
 * @see ../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-hornbeam-hedge-kit.ts */
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { frames, simplifyRing, toLocal, type Frame, type Json, type P2, type V3 } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/volumes-lib.ts";
import { outline, pathOf, pointsOf, project, r4, r5, toDrawing, type Look, type View } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-hornbeam-hedge-kit.ts";

export * from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-hornbeam-hedge-kit.ts";
export { frames, toLocal, type Frame, type Json, type P2, type V3 };

export const HERE = dirname(fileURLToPath(import.meta.url));
export const TICKET = join(HERE, "../..");
export const GENERATED = join(TICKET, "🗑️generated/PW-13");
export const STAFFAGE = join(TICKET, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
export const VIEWS: readonly View[] = ["elevation", "front", "plan"];
export const SCALES = [50, 100, 200, 500] as const;
export const T = (en: string, de: string) => ({ en, de });
export const feature = (scale: number): number => (0.15 * scale) / 1000;

/** 🌬️ Mean wind speed at height z for a 10 m speed (power law, exponent 0.22, EN 1991-1-4 terrain II order of magnitude). */
export const windAt = (v10: number, z: number): number => v10 * Math.max(z, 0.2) ** 0.22 / 10 ** 0.22;
/** 💨️ Drag F = ½ ρ cd A v² with ρ = 1.25 kg/m³ (EN 1991-1-4). */
export const drag = (cd: number, area: number, v: number): number => 0.5 * 1.25 * cd * area * v * v;
export const f1 = (x: number): string => x.toFixed(1);
export const de1 = (x: number): string => x.toFixed(1).replace(".", ",");
export const de2 = (x: number): string => x.toFixed(2).replace(".", ",");

/** 📐️ The dimensional note of a true-extent plan symbol: the true extents of the asset and what outline and stem marks stand for (`body` completes the sentence in both languages). */
export const trueExtentNote = (d: { widthM: number; depthM: number; heightM: number }, body: { en: string; de: string }): { en: string; de: string } =>
  T(`A conventional simplification of the plant drawn at its true extent in model metres in all three views (${d.widthM.toFixed(2)} m wide, ${d.depthM.toFixed(2)} m deep and ${d.heightM.toFixed(2)} m high): ${body.en} It is a symbol, never used silently for a pictorial scale.`,
    `Eine konventionelle Vereinfachung der Pflanze in ihrer wahren Ausdehnung in Modellmetern in allen drei Ansichten (${de2(d.widthM)} m breit, ${de2(d.depthM)} m tief und ${de2(d.heightM)} m hoch): ${body.de} Es ist ein Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet.`);

/** 🎲️ A deterministic generator of reals in [0, 1). */
export function random(seed: number): () => number {
  let s = seed >>> 0 || 1;
  return () => ((s = (Math.imul(s ^ (s >>> 15), 2246822519) + 0x9e3779b9) >>> 0), (s >>> 8) / 16777216);
}

/** 🍃️ A leafy mass: a closed star-shaped skin about `c` with semi-axes `rx` (world x), `ryTop`/`ryBot` (up / down) and `rz` (world z), a relief of leaf clusters of amplitude `amp` (metres, radial) on 3D waves of wave numbers `waves`, tessellated on `nLat` x `nLon`. */
export type Leafy = { id: string; bone: string; look: Look; c: V3; rx: number; ryTop: number; ryBot: number; rz: number; amp: number; waves: [number, number]; nLat: number; nLon: number; seed: number; squash?: number; fit?: { min: V3; max: V3 } };

function reliefField(seed: number, waves: [number, number]): (d: V3) => number {
  const rnd = random(seed * 7919 + 13);
  const comps = Array.from({ length: 9 }, () => {
    const u = 2 * rnd() - 1, a = 2 * Math.PI * rnd(), s = Math.sqrt(1 - u * u);
    const k = waves[0] + (waves[1] - waves[0]) * rnd();
    return { k: [k * s * Math.cos(a), k * u, k * s * Math.sin(a)] as V3, p: 2 * Math.PI * rnd(), w: 0.6 + 0.4 * rnd() };
  });
  const total = comps.reduce((n, c) => n + c.w, 0);
  return (d) => {
    let v = 0;
    for (const c of comps) v += c.w * Math.sin(c.k[0] * d[0] + c.k[1] * d[1] + c.k[2] * d[2] + c.p);
    const x = (1.6 * v) / total;
    return Math.max(-1, Math.min(1, Math.sign(x) * Math.abs(x) ** 0.7));
  };
}

/** 📍️ The world vertices of a leafy mass (top pole, rings from top to bottom, bottom pole). */
export function leafyVertices(L: Leafy): V3[] {
  const f = reliefField(L.seed, L.waves);
  const point = (d: V3): V3 => {
    const ry = d[1] >= 0 ? L.ryTop : L.ryBot;
    const sq = L.squash ?? 0;
    const e: V3 = [L.rx * d[0], ry * d[1], L.rz * d[2]];
    const n = Math.hypot(d[0] / L.rx, d[1] / ry, d[2] / L.rz) || 1;
    const nn: V3 = [d[0] / L.rx / n, d[1] / ry / n, d[2] / L.rz / n];
    const flat = sq > 0 && d[1] < 0 ? 1 - sq * d[1] * d[1] : 1;
    const a = L.amp * f(d) * (d[1] < -0.6 ? 0.5 : 1);
    return [L.c[0] + e[0] * flat + nn[0] * a, L.c[1] + e[1] + nn[1] * a, L.c[2] + e[2] * flat + nn[2] * a];
  };
  const out: V3[] = [point([0, 1, 0])];
  for (let i = 1; i < L.nLat; i++) {
    const phi = (Math.PI * i) / L.nLat;
    for (let j = 0; j < L.nLon; j++) {
      const th = (2 * Math.PI * j) / L.nLon;
      out.push(point([Math.sin(phi) * Math.cos(th), Math.cos(phi), Math.sin(phi) * Math.sin(th)]));
    }
  }
  out.push(point([0, -1, 0]));
  if (!L.fit) return out;
  const e = extent(out);
  return out.map((p) => p.map((q, k) => {
    const c = L.c[k]!, hi = q >= c;
    const from = hi ? e.max[k]! - c : c - e.min[k]!, to = hi ? L.fit!.max[k]! - c : c - L.fit!.min[k]!;
    return c + (q - c) * (from > 1e-9 ? to / from : 1);
  }) as V3);
}

/** 🔺️ The triangles of a leafy skin, outward. */
export function leafyTriangles(L: Leafy, v: readonly V3[]): [number, number, number][] {
  const n = L.nLon, rows = L.nLat - 1, at = (i: number, j: number) => 1 + i * n + (((j % n) + n) % n), last = v.length - 1;
  const t: [number, number, number][] = [];
  for (let j = 0; j < n; j++) t.push([0, at(0, j + 1), at(0, j)]);
  for (let i = 0; i + 1 < rows; i++) for (let j = 0; j < n; j++) {
    const [a, b, c, d] = [at(i, j), at(i, j + 1), at(i + 1, j), at(i + 1, j + 1)];
    t.push([a, b, c], [b, d, c]);
  }
  for (let j = 0; j < n; j++) t.push([last, at(rows - 1, j), at(rows - 1, j + 1)]);
  let vol = 0;
  for (const [a, b, c] of t) {
    const [p, q, r] = [v[a]!, v[b]!, v[c]!];
    vol += p[0] * (q[1] * r[2] - q[2] * r[1]) - p[1] * (q[0] * r[2] - q[2] * r[0]) + p[2] * (q[0] * r[1] - q[1] * r[0]);
  }
  return vol > 0 ? t : t.map(([a, b, c]) => [a, c, b]);
}

/** 🧊️ The bone-local polyhedron of a leafy mass. */
export function leafySolid(L: Leafy, F: Record<string, Frame>): Json {
  const v = leafyVertices(L);
  const f = F[L.bone]!;
  return { id: L.id, kind: "polyhedron", bone: L.bone, material: L.look.material, role: L.look.role ?? "body", tone: L.look.tone, vertices: v.map((p) => toLocal(f, p).map(r5)), triangles: leafyTriangles(L, v), ...(L.look.flags ?? {}) };
}

/** ⭐️ The outline of a star-shaped point cloud about `c`: the farthest point of every angular bin, in angle order. */
export function starOutline(points: readonly P2[], c: P2, bins = 240): P2[] {
  const best: (P2 | undefined)[] = new Array(bins);
  const dist: number[] = new Array(bins).fill(-1);
  for (const p of points) {
    const a = Math.atan2(p.y - c.y, p.x - c.x);
    const k = Math.min(bins - 1, Math.floor(((a + Math.PI) / (2 * Math.PI)) * bins));
    const d = Math.hypot(p.x - c.x, p.y - c.y);
    if (d > dist[k]!) [dist[k], best[k]] = [d, p];
  }
  return best.filter((p): p is P2 => p !== undefined);
}

/** 👁️ The true outline of a leafy mass in a view: from its projected centre, the farthest crossing of every ray (one per `bins`) with the projected triangle edges, so the outline is exact at every ray for this star-shaped body. */
export function leafyPolygon(view: View, L: Leafy, bins = 256): P2[] {
  const v = leafyVertices(L).map((p) => project(view, p));
  const c = project(view, L.c);
  const edges = new Map<string, [number, number]>();
  for (const [a, b, d] of leafyTriangles(L, leafyVertices(L))) for (const [i, j] of [[a, b], [b, d], [d, a]] as [number, number][]) edges.set(i < j ? `${i},${j}` : `${j},${i}`, i < j ? [i, j] : [j, i]);
  const segs = [...edges.values()].map(([i, j]) => [v[i]!.x - c.x, v[i]!.y - c.y, v[j]!.x - c.x, v[j]!.y - c.y] as const);
  return Array.from({ length: bins }, (_, k) => {
    const a = (2 * Math.PI * k) / bins, dx = Math.cos(a), dy = Math.sin(a);
    let best = 0;
    for (const [x0, y0, x1, y1] of segs) {
      const ex = x1 - x0, ey = y1 - y0, den = dx * ey - dy * ex;
      if (Math.abs(den) < 1e-15) continue;
      const t = (x0 * ey - y0 * ex) / den, u = (x0 * dy - y0 * dx) / den;
      if (u >= 0 && u <= 1 && t > best) best = t;
    }
    return { x: c.x + best * dx, y: c.y + best * dy };
  });
}

/** 📦️ The world extent of a set of points. */
export function extent(points: readonly V3[]): { min: V3; max: V3 } {
  const min: V3 = [Infinity, Infinity, Infinity], max: V3 = [-Infinity, -Infinity, -Infinity];
  for (const p of points) for (let k = 0; k < 3; k++) [min[k], max[k]] = [Math.min(min[k]!, p[k]!), Math.max(max[k]!, p[k]!)];
  return { min, max };
}

/** ✏️ A filled or open drawing shape from world rings of a view: bone-local path in the elevation, world `poly`/`path` in front and plan. */
export function shapeOf(view: View, F: Record<string, Frame>, base: Json, rings: P2[][], closed = true): Json {
  const loc = toDrawing(view, F[base.bone]!, rings);
  if (view !== "elevation" && loc.length === 1) return { ...base, kind: "poly", points: pointsOf(loc[0]!), closed };
  return { ...base, kind: "path", d: pathOf(loc, closed) };
}

/** 🧹️ The union outline of polygons at a scale (simplified to a quarter feature, holes below a feature squared dropped). */
export const unite = (polys: P2[][], scale: number): P2[][] => outline(polys, feature(scale) / 4, feature(scale) ** 2);
export const simplify = (ring: P2[], scale: number): P2[] => simplifyRing(ring, feature(scale) / 6);

/** 🔍️ Whether a point lies inside a ring (even-odd). */
export function inside(p: P2, ring: readonly P2[]): boolean {
  let c = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const [a, b] = [ring[i]!, ring[j]!];
    if (a.y > p.y !== b.y > p.y && p.x < ((b.x - a.x) * (p.y - a.y)) / (b.y - a.y) + a.x) c = !c;
  }
  return c;
}

/** 📏️ The distance from a point to the boundary of a ring. */
export function edgeDistance(p: P2, ring: readonly P2[]): number {
  let best = Infinity;
  for (let i = 0; i < ring.length; i++) {
    const [a, b] = [ring[i]!, ring[(i + 1) % ring.length]!];
    const [dx, dy] = [b.x - a.x, b.y - a.y];
    const t = Math.max(0, Math.min(1, ((p.x - a.x) * dx + (p.y - a.y) * dy) / (dx * dx + dy * dy || 1)));
    best = Math.min(best, Math.hypot(p.x - a.x - t * dx, p.y - a.y - t * dy));
  }
  return best;
}

/** 🌱️ A leaf mark: open polylines of one mark at (x, y) with angle `a` (radians) and length `s` (metres). */
export type Mark = (x: number, y: number, a: number, s: number) => P2[][];

const rot = (x: number, y: number, a: number, u: number, v: number): P2 => ({ x: x + u * Math.cos(a) - v * Math.sin(a), y: y + u * Math.sin(a) + v * Math.cos(a) });

/** 🌿️ Box: a short curved stroke of one clipped shoot with two tiny opposite leaves. */
export const boxMark: Mark = (x, y, a, s) => [
  [rot(x, y, a, -0.5 * s, 0), rot(x, y, a, -0.15 * s, 0.12 * s), rot(x, y, a, 0.2 * s, 0.14 * s), rot(x, y, a, 0.5 * s, 0.02 * s)],
];
/** 🌿️ Privet: a twig tick with an opposite pair of narrow leaves (open strokes that never close a sliver). */
export const privetMark: Mark = (x, y, a, s) => [
  [rot(x, y, a, -0.45 * s, -0.25 * s), rot(x, y, a, 0, 0), rot(x, y, a, 0.45 * s, -0.25 * s)],
  [rot(x, y, a, -0.1 * s, 0.22 * s), rot(x, y, a, 0.1 * s, 0.42 * s)],
];
/** 🌿️ Cherry laurel: the curved upper edge of one large leathery leaf and a short separate midrib tick. */
export const laurelMark: Mark = (x, y, a, s) => [
  [rot(x, y, a, -0.5 * s, 0), rot(x, y, a, -0.25 * s, 0.16 * s), rot(x, y, a, 0.05 * s, 0.2 * s), rot(x, y, a, 0.32 * s, 0.13 * s), rot(x, y, a, 0.5 * s, 0)],
  [rot(x, y, a, -0.3 * s, -0.02 * s), rot(x, y, a, 0.25 * s, 0.03 * s)],
];

/** 🍂️ Leaf marks scattered on a jittered grid of pitch `pitch` inside `ring`, at least `margin` from its edge and outside every `avoid` ring (inflated by `margin`). */
export function marks(mark: Mark, ring: readonly P2[], pitch: number, size: number, margin: number, seed: number, avoid: readonly (readonly P2[])[] = [], angle = 0.5): P2[][] {
  const rnd = random(seed);
  const xs = ring.map((p) => p.x), ys = ring.map((p) => p.y);
  const out: P2[][] = [];
  let row = 0;
  for (let y = Math.min(...ys) + pitch / 2; y < Math.max(...ys); y += pitch * 0.87, row++) {
    for (let x = Math.min(...xs) + (row % 2 ? pitch / 2 : 0); x < Math.max(...xs); x += pitch) {
      const p = { x: x + (rnd() - 0.5) * pitch * 0.5, y: y + (rnd() - 0.5) * pitch * 0.4 };
      if (!inside(p, ring) || edgeDistance(p, ring) < margin + size / 2) continue;
      if (avoid.some((r) => inside(p, r) || edgeDistance(p, r) < margin + size / 2)) continue;
      out.push(...mark(p.x, p.y, (rnd() - 0.5) * 2 * angle + (rnd() < 0.5 ? 0 : Math.PI), size * (0.8 + 0.4 * rnd())));
    }
  }
  return out;
}

/** 🎞️ A deterministic looping sway about the windy lean: `n` keys over `durationS`, per bone the lean `base` (degrees), the amplitude and a phase (cycles); first key equals last key. */
export function swayKeys(durationS: number, n: number, bones: Record<string, { base: number; amp: number; phase: number }>, pose = "windy"): Json[] {
  return Array.from({ length: n + 1 }, (_, i) => {
    const t = (durationS * i) / n;
    const k = i === n ? 0 : i;
    return { t: r4(t), pose, ease: "smooth", bones: Object.fromEntries(Object.entries(bones).map(([b, s]) => [b, { angleDeg: r4(s.base + s.amp * Math.sin(2 * Math.PI * (k / n + s.phase))) }])) };
  });
}

/** 🌪️ The gust profile: load factors at the key times (EN 1991-1-4 peak pressure with Iv 0.25 gives 2.75 times the mean). */
export const GUST = { t: [0, 1.2, 2.5, 3.6, 5, 8], v: [0.05, 0.45, 2.75, 1.4, 0.5, 0.05], lean: [0.1, 0.43, 1, 0.64, 0.29, 0.1] };

/** ☀️ The diurnal course: sunlit factor and transpiration of the sun and shade foliage at hours. */
export const DIURNAL = {
  sun: [[0, 0], [6, 0], [8, 0.43], [10, 0.79], [12, 1], [14, 0.79], [16, 0.36], [18, 0], [24, 0]] as [number, number][],
  face: [[0, 0.03], [5, 0.05], [7, 0.3], [9, 0.7], [12, 1], [15, 0.9], [18, 0.35], [20, 0.08], [24, 0.03]] as [number, number][],
  core: [[0, 0.03], [5, 0.05], [7, 0.35], [9, 0.75], [12, 1], [15, 0.85], [18, 0.3], [20, 0.06], [24, 0.03]] as [number, number][],
};

/** 🕰️ The diurnal clip of a plant with the given sunlit fractions (noon) per region and the two transpiration emitters. */
export function diurnalClip(noon: Record<string, number>, emitters: [string, string], title = T("Day of transpiration", "Transpirationstag")): Json {
  return {
    id: "diurnal-transpiration",
    title,
    description: T(
      "A 24 h cycle compressed to 24 s (1 s = 1 h, 00:00 to 24:00), calm weather. The sunlit fraction of every region and the transpiration strength follow the sun, stomata nearly closed at night; leaf and bark temperatures are not authored but follow from the thermal solver.",
      "Ein 24-h-Zyklus auf 24 s gerafft (1 s = 1 h, 00:00 bis 24:00) bei ruhigem Wetter. Der besonnte Anteil jeder Region und die Transpirationsstärke folgen der Sonne, nachts sind die Stomata fast geschlossen; Blatt- und Rindentemperaturen werden nicht vorgegeben, sondern folgen aus dem thermischen Löser.",
    ),
    durationS: 24,
    fps: 10,
    loop: true,
    rootMotion: "none",
    keys: [{ t: 0, pose: "calm", ease: "smooth" }, { t: 12, pose: "calm", ease: "smooth" }, { t: 24, pose: "calm", ease: "smooth" }],
    contacts: [],
    phases: [
      { id: "night", fromS: 0, toS: 6, title: T("Night", "Nacht") },
      { id: "morning", fromS: 6, toS: 12, title: T("Morning to noon", "Morgen bis Mittag") },
      { id: "afternoon", fromS: 12, toS: 18, title: T("Afternoon", "Nachmittag") },
      { id: "evening", fromS: 18, toS: 24, title: T("Evening and night", "Abend und Nacht") },
    ],
    events: [
      { t: 6, kind: "thermal", title: T("Sunrise", "Sonnenaufgang") },
      { t: 12, kind: "thermal", title: T("Solar noon, peak transpiration", "Sonnenhöchststand, maximale Transpiration") },
      { t: 18, kind: "thermal", title: T("Sunset", "Sonnenuntergang") },
    ],
    tracks: [
      { target: `emitter:${emitters[0]}`, ease: "smooth", keys: DIURNAL.face.map(([t, v]) => ({ t, v })) },
      { target: `emitter:${emitters[1]}`, ease: "smooth", keys: DIURNAL.core.map(([t, v]) => ({ t, v })) },
      ...Object.entries(noon).map(([region, s]) => ({ target: `region:${region}:sunlitFraction`, ease: "smooth", keys: DIURNAL.sun.map(([t, v]) => ({ t, v: r4(v * s) })) })),
    ],
  };
}

/** 📚️ The references every plant of this worker cites. */
export const REFERENCES: Json[] = [
  { id: "wood-handbook-2021", citation: "Forest Products Laboratory (2021). Wood Handbook: Wood as an Engineering Material. General Technical Report FPL-GTR-282, USDA Forest Service. Chapter 5 (green density, modulus of elasticity and rupture; closest listed species used as proxy)." },
  { id: "en-1991-1-4", citation: "EN 1991-1-4:2005 Eurocode 1, Actions on structures, Part 1-4: Wind actions. Air density 1.25 kg/m3, F = 0.5 rho cd A v^2, peak velocity pressure (1 + 7 Iv) q_m." },
  { id: "en-1991-1-3", citation: "EN 1991-1-3:2003 Eurocode 1, Actions on structures, Part 1-3: Snow loads. Used for the order of magnitude of snow on plant surfaces." },
  { id: "vogel-1989", citation: "Vogel, S. (1989). Drag and reconfiguration of broad leaves in high winds. Journal of Experimental Botany 40(8), 941-948." },
  { id: "jones-2014", citation: "Jones, H. G. (2014). Plants and Microclimate, 3rd ed., Cambridge University Press. Leaf energy balance, leaf area index, boundary-layer conductance." },
  { id: "monteith-unsworth-2013", citation: "Monteith, J. L. and Unsworth, M. H. (2013). Principles of Environmental Physics, 4th ed., Academic Press. Penman-Monteith leaf transpiration." },
  { id: "stace-2019", citation: "Stace, C. A. (2019). New Flora of the British Isles, 4th ed., C & M Floristics. Species descriptions (habit, leaf size and shape)." },
  { id: "roloff-baertels-2018", citation: "Roloff, A. and Bärtels, A. (2018). Flora der Gehölze, 5th ed., Eugen Ulmer, Stuttgart. Habit, height, growth and leaf characters of woody plants." },
];

/** 🗒️ The assumptions of the thermal model shared by the three plants. */
export const THERMAL_ASSUMPTIONS: Json[] = [
  T(
    "Leaf regions use one-sided leaf area on an exchange basis; the leaf area index is that area over the crown ground projection. The maximum stomatal conductance is an authored estimate for the species type, guided by Jones (2014). Photosynthesis type: C3. Aperture and night closure are the engine's business, not authored here.",
    "Blattregionen verwenden die einseitige Blattfläche als Austauschfläche; der Blattflächenindex ist diese Fläche bezogen auf die Kronenprojektion am Boden. Die maximale Spaltöffnungsleitfähigkeit ist eine Autorenschätzung für den Artentyp, angelehnt an Jones (2014). Photosynthesetyp: C3. Öffnung und nächtliches Schließen sind Sache der Engine und werden hier nicht vorgegeben.",
  ),
  T(
    "Bark and stems are passive. Their areal heat capacity is the volumetric heat capacity of green wood (about 2.7 MJ per m3 and K) times an effective depth of a quarter of the stem diameter, an authored estimate guided by the Wood Handbook. No surface temperature is authored.",
    "Rinde und Stämme sind passiv. Ihre flächenbezogene Wärmekapazität ist die volumetrische Wärmekapazität von frischem Holz (etwa 2,7 MJ je m3 und K) mal einer wirksamen Tiefe von einem Viertel des Stammdurchmessers, eine Autorenschätzung angelehnt an das Wood Handbook. Es werden keine Oberflächentemperaturen vorgegeben.",
  ),
  T(
    "Transpiration emitters release the latent flux of their foliage region as vapour, spread over at least five anchors on the crown surface; the direction is the vertical, since the release is diffuse. Sun and shade foliage are separate regions: the outer leaf skin and the hidden inner leaf mass.",
    "Transpirationsemitter geben den latenten Strom ihrer Laubregion als Dampf ab, verteilt auf mindestens fünf Anker an der Kronenoberfläche; die Richtung ist die Senkrechte, da die Abgabe diffus ist. Sonnen- und Schattenlaub sind getrennte Regionen: die äußere Blatthaut und die verborgene innere Blattmasse.",
  ),
];

export const ENVIRONMENT = { airTempC: 22, meanRadiantTempC: 25, relativeHumidity: 0.55, windMS: 2, windDirectionDeg: 0, solarWM2: 700, pressurePa: 101325, diffuseFraction: 0.15 };
export const EXPORTS = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"];

/** 📏️ Pens, valid ranges and hatches per preset scale (role pens as the existing vegetation). */
export const PENS: Record<number, Json> = {
  50: { body: 0.25, detail: 0.18, texture: 0.13, glazing: null, ground: 0.35 },
  100: { body: 0.2, detail: 0.13, texture: 0.1, glazing: null, ground: 0.3 },
  200: { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 },
  500: { body: 0.15, detail: null, texture: null, glazing: null, ground: 0.25 },
};
export const RANGES: Record<number, Json> = { 50: { minDenominator: 20, maxDenominator: 75 }, 100: { minDenominator: 75, maxDenominator: 150 }, 200: { minDenominator: 150, maxDenominator: 350 }, 500: { minDenominator: 350, maxDenominator: 1000 } };
export const SHADE: Record<number, Json[]> = {
  50: [{ id: "shade-lines", title: T("Shade line hatch", "Schattenschraffur"), pattern: "lines", angleDeg: 45, spacingMm: 0.8, penMm: 0.1 }],
  100: [{ id: "shade-lines", title: T("Shade line hatch", "Schattenschraffur"), pattern: "lines", angleDeg: 45, spacingMm: 0.6, penMm: 0.1 }],
  200: [],
  500: [],
};

/** 💾️ Writes the asset document and the manifest row of the coordinator. */
export function save(asset: Json, row: Json): string {
  const folder = join(STAFFAGE, "🗂️catalog/🌳️vegetation", asset.id);
  mkdirSync(folder, { recursive: true });
  const path = join(folder, "🔣️.json");
  writeFileSync(path, JSON.stringify({ $schema: "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", version: "semio.staffage/v2", asset }, null, 2) + "\n");
  const rows = join(TICKET, "📥️manifest-rows");
  mkdirSync(rows, { recursive: true });
  writeFileSync(join(rows, `${asset.id}.json`), JSON.stringify(row, null, 2) + "\n");
  return path;
}

/** 🧾️ The manifest row of an asset, derived from the asset itself. */
export function manifestRow(asset: Json, distinction: Json): Json {
  return {
    id: asset.id,
    category: "vegetation",
    title: asset.title,
    heightM: asset.dimensions.heightM,
    widthM: asset.dimensions.widthM,
    powerW: 0,
    poses: asset.poses.filter((p: Json) => !p.key).map((p: Json) => p.id),
    clips: asset.clips.map((c: Json) => c.id),
    representations: Object.fromEntries(Object.entries(asset.representations).map(([k, v]) => [k, (v as Json).status])),
    symbols: asset.symbols.map((s: Json) => s.id),
    analytics: ["thermal", "transport", "structural"],
    exports: asset.exports,
    distinction,
    completion: asset.completion,
  };
}

/** ✂️ A tube cut at height `ymax` (the part below a crown). */
export function clipTube<Tb extends { points: { p: V3; r: number }[] }>(t: Tb, ymax: number): Tb {
  const pts: Tb["points"] = [];
  for (let i = 0; i < t.points.length; i++) {
    const a = t.points[i]!;
    if (a.p[1] <= ymax) pts.push(a);
    else if (pts.length && Math.abs(pts[pts.length - 1]!.p[1] - ymax) < 1e-9) break;
    else {
      const b = t.points[i - 1]!;
      const f = (ymax - b.p[1]) / (a.p[1] - b.p[1]);
      pts.push({ p: [b.p[0] + (a.p[0] - b.p[0]) * f, ymax, b.p[2] + (a.p[2] - b.p[2]) * f], r: b.r + (a.r - b.r) * f });
      break;
    }
  }
  return { ...t, points: pts };
}

/** 🥚️ The [height, radius] profile of a body of revolution with an elliptic upper half (semi-axes `r`, `top`) and lower half (`r`, `bottom`), closed through its axis. */
export function ovoid(r: number, top: number, bottom: number, n = 24): [number, number][] {
  return Array.from({ length: n + 1 }, (_, i) => {
    const a = -Math.PI / 2 + (Math.PI * i) / n;
    const s = Math.sin(a);
    return [r5(s * (s < 0 ? bottom : top)), r5(Math.max(0, r * Math.cos(a)))] as [number, number];
  }).map(([h, q], i, all) => (i === 0 || i === all.length - 1 ? [h, 0] : [h, q]));
}

/** 🔪️ A ring clipped to the vertical strip lo <= x <= hi (Sutherland-Hodgman): a clipped plant drawn up to its seams with the neighbours. */
export function clipStrip(ring: P2[], lo: number, hi: number): P2[] {
  const cut = (pts: P2[], keep: (p: P2) => boolean, at: (a: P2, b: P2) => P2): P2[] => pts.flatMap((p, i) => {
    const q = pts[(i + 1) % pts.length]!;
    if (keep(p)) return keep(q) ? [p] : [p, at(p, q)];
    return keep(q) ? [at(p, q)] : [];
  });
  const atX = (x: number) => (a: P2, b: P2): P2 => ({ x, y: a.y + ((b.y - a.y) * (x - a.x)) / (b.x - a.x) });
  return cut(cut(ring, (p) => p.x >= lo, atX(lo)), (p) => p.x <= hi, atX(hi));
}
