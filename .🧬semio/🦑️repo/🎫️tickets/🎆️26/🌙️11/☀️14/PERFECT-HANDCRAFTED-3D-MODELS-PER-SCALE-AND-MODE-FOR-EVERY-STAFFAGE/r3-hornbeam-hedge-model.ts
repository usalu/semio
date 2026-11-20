/** 🌳️ Round-3 hornbeam hedge (Carpinus betulus, clipped formal hedge 3.14 x 0.70 x 1.87 m): every solid of every scale and list stated by hand in world metres, written as the volumes source; every 2D shape of every scale redrawn as the true view of that scale's body (ruling E1); the candidate catalog document with the root fixes.
 * Usage: bun r3-hornbeam-hedge-model.ts
 * @see ./round3-brief.md
 * @see ./r3-vegetation-hornbeam-hedge.md */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { entryOf, frames, GENERATED, sourcePath, type Frame, type Json, type P2, type V3 } from "./volumes-lib.ts";
import { blobPolygon, blobSolid, flarePolygon, flareSolid, outline, panelPolygons, panelSolid, pathOf, pointsOf, r4, r5, ridge, toDrawing, tube, tubePolygons, tubeSolid, type Blob, type Flare, type Look, type Panel, type Section, type Tube, type View } from "./r3-hornbeam-hedge-kit.ts";

const SPEC = "vegetation/hornbeam-hedge";
const { entry, asset: catalog } = entryOf(SPEC);
const asset: Json = structuredClone(catalog);
const F: Record<string, Frame> = frames(asset);
const K = [1, 2, 3, 4, 5, 6] as const;
const XK = (k: number): number => -1.25 + 0.5 * (k - 1);
const LEAF: Look = { material: "hornbeam-leaf", tone: 0.5 };
const BARK: Look = { material: "hornbeam-bark", tone: 0.35 };
const RIDGE: Look = { material: "hornbeam-bark", tone: 0.28, role: "texture", flags: { texture: true } };
const MULCH: Look = { material: "bark-mulch", tone: 0.22, role: "detail" };
const WOOD: Look = { material: "hornbeam-wood", tone: 0.4 };
const T = (en: string, de: string) => ({ en, de });

/** 📐️ The clipped cross-profile of the hedge at 1:50 (height y, half thickness across the row, rounding inset of the half length): rounded underside at 0.33 m, widest 0.70 m at 0.56 m, battered 3.5 degrees per face to 0.56 m at 1.68 m, shoulder radius about 7 cm, flat clipped top at 1.838 m. */
const PROFILE_50: [number, number, number][] = [
  [0.330, 0.268, 0.060], [0.350, 0.310, 0.030], [0.385, 0.336, 0.012], [0.450, 0.348, 0.003], [0.560, 0.350, 0], [0.800, 0.339, 0], [1.100, 0.320, 0],
  [1.400, 0.300, 0], [1.680, 0.281, 0], [1.760, 0.273, 0.004], [1.795, 0.259, 0.014], [1.818, 0.237, 0.030], [1.832, 0.207, 0.050], [1.838, 0.175, 0.068],
];
/** 🌿️ The six plants: centre and half length of each clipped panel along the row (each plant reaches a little differently into its neighbours), thickness factor and lateral drift of the face (all faces lie in the one plane the hedge trimmer cuts). */
const PLANTS_50: Record<number, { cx: number; hx: number; t: number; dz: [number, number, number, number] }> = {
  1: { cx: -1.255, hx: 0.315, t: 1, dz: [0, 0, 0, 0] },
  2: { cx: -0.75, hx: 0.4, t: 1, dz: [0, 0, 0, 0] },
  3: { cx: -0.248, hx: 0.395, t: 1, dz: [0, 0, 0, 0] },
  4: { cx: 0.252, hx: 0.405, t: 1, dz: [0, 0, 0, 0] },
  5: { cx: 0.75, hx: 0.398, t: 1, dz: [0, 0, 0, 0] },
  6: { cx: 1.255, hx: 0.315, t: 1, dz: [0, 0, 0, 0] },
};const drift = (dz: number[], y: number): number => {
  const ys = [0.45, 0.85, 1.25, 1.65];
  if (y <= ys[0]!) return dz[0]!;
  for (let i = 0; i + 1 < ys.length; i++) if (y <= ys[i + 1]!) return dz[i]! + ((dz[i + 1]! - dz[i]!) * (y - ys[i]!)) / (ys[i + 1]! - ys[i]!);
  return dz[3]!;
};
const panel = (id: string, bone: string, look: Look, rows: [number, number, number][], cx: number, hx: number, t: number, e: number, dz: number[] = [0, 0, 0, 0], lift = 0): Panel => ({
  id, bone, look, sections: rows.map(([y, hz, inset]): Section => ({ y: y > 1.7 ? r5(y + lift) : y, cx, cz: drift(dz, y), hx: hx - inset, hz: hz * t, e })),
});
/** ✂️ Neighbouring plants are clipped 4 mm apart in height, so their overlapping tops never share one plane. */
const LIFT = (k: number): number => (k % 2 === 0 ? -0.004 : 0);

/** 🪵️ The two stems of every plant at 1:50: base offset (x, z), offset at 0.64 m, radius at the collar and at 0.64 m, bow along the row; stations at 0, 0.06, 0.18, 0.33, 0.48, 0.64 m. */
const STEMS_50: Record<string, [number, number, number, number, number, number, number]> = {
  "1-a": [-0.018, -0.02, -0.06, -0.14, 0.026, 0.017, 0.012], "1-b": [0.02, 0.018, 0.08, 0.125, 0.024, 0.016, -0.01],
  "2-a": [-0.022, -0.016, -0.05, -0.15, 0.025, 0.016, -0.008], "2-b": [0.016, 0.022, 0.07, 0.13, 0.026, 0.017, 0.014],
  "3-a": [-0.014, -0.022, -0.075, -0.12, 0.027, 0.018, 0.01], "3-b": [0.022, 0.016, 0.055, 0.15, 0.023, 0.015, -0.012],
  "4-a": [-0.02, -0.018, -0.045, -0.145, 0.024, 0.016, -0.01], "4-b": [0.018, 0.02, 0.085, 0.12, 0.026, 0.017, 0.008],
  "5-a": [-0.016, -0.02, -0.07, -0.135, 0.026, 0.017, 0.014], "5-b": [0.02, 0.016, 0.06, 0.14, 0.025, 0.016, -0.008],
  "6-a": [-0.02, -0.016, -0.055, -0.13, 0.025, 0.016, -0.012], "6-b": [0.016, 0.02, 0.075, 0.145, 0.026, 0.017, 0.01],
};
const STATIONS_50 = [0, 0.06, 0.18, 0.33, 0.48, 0.64];
const stemTube = (k: number, s: "a" | "b", ys: number[], id = `stem-${k}-${s}`, bone = `stem-${k}`, look = BARK, shrink = 1): Tube => {
  const [bx, bz, tx, tz, r0, r1, bow] = STEMS_50[`${k}-${s}`]!;
  return tube(id, bone, look, ys.map((y) => {
    const f = (y / 0.64) ** 1.15;
    const r = y === 0 ? r0 * 1.12 : r0 + ((r1 - r0) * y) / 0.64;
    return [r4(XK(k) + bx + (tx - bx) * f + bow * Math.sin(Math.PI * f)), y, r4(bz + (tz - bz) * f), r5(r * shrink)];
  }));
};
const stemAt = (k: number, s: "a" | "b", y: number): V3 => {
  const t = stemTube(k, s, [y]);
  return t.points[0]!.p;
};
const flare = (k: number, profile: [number, number][], id = `flare-${k}`): Flare => ({ id, bone: `stem-${k}`, look: BARK, c: [XK(k), 0, 0], profile });

/** 🍃️ Summer flush: low cushions of new shoots on the clipped top (1:50: two per plant, 3.4 cm high). */
const FLUSH_50: Record<number, [number, number][]> = { 1: [[-1.35, 0.02], [-1.13, -0.018]], 2: [[-0.86, -0.022], [-0.64, 0.016]], 3: [[-0.36, 0.018], [-0.135, -0.02]], 4: [[0.14, -0.016], [0.365, 0.022]], 5: [[0.64, 0.02], [0.86, -0.015]], 6: [[1.135, -0.018], [1.35, 0.02]] };
const flush50 = (k: number): Blob[] => FLUSH_50[k]!.map(([x, z], i) => ({ id: `flush-${k}-${"ab"[i]}`, bone: `block-${k}`, look: LEAF, c: [x, 1.836, z], radii: [0.2, 0.033, 0.135] }));

/** 🌫️ The shaded core: the inner leaf mass about 13 cm inside the clipped faces. */
const CORE_50: [number, number, number][] = [[0.47, 0.15, 0.08], [0.56, 0.205, 0.03], [0.9, 0.2, 0], [1.3, 0.18, 0], [1.58, 0.165, 0.02], [1.66, 0.11, 0.07]];

const panels50 = K.map((k) => panel(`block-${k}`, `block-${k}`, LEAF, PROFILE_50, PLANTS_50[k]!.cx, PLANTS_50[k]!.hx, PLANTS_50[k]!.t, 6, PLANTS_50[k]!.dz, LIFT(k)));
const stems50 = K.flatMap((k) => (["a", "b"] as const).map((s) => stemTube(k, s, STATIONS_50)));
const FLARE_50: [number, number][] = [[0, 0.085], [0.015, 0.072], [0.035, 0.055], [0.06, 0.043], [0.09, 0.036], [0.11, 0.034]];
const flares50 = K.map((k) => flare(k, FLARE_50));
const ridges50 = stems50.flatMap((t, i) => [ridge(`${t.id}-ridge1`, t, RIDGE, 35 + 40 * (i % 3), 0.12, 0.72, 0.004, 0.0025, 30), ridge(`${t.id}-ridge2`, t, RIDGE, 190 + 30 * (i % 2), 0.12, 0.72, 0.004, 0.0025, -25)]);
const cores50 = K.map((k) => panel(`th-core-${k}`, `block-${k}`, { material: "hornbeam-leaf", tone: 0.4 }, CORE_50, PLANTS_50[k]!.cx, k === 1 || k === 6 ? 0.27 : 0.3, 1, 3));
const MULCH_PROFILE: [number, number][] = [[0, -0.3], [0.01, -0.27], [0.02, -0.2], [0.026, -0.1], [0.028, 0], [0.026, 0.1], [0.02, 0.2], [0.01, 0.27], [0, 0.3]];

const local = (bone: string, p: V3): V3 => {
  const f = F[bone]!;
  const d: V3 = [p[0] - f.o[0], p[1] - f.o[1], p[2] - f.o[2]];
  return [r5(d[0] * f.u[0] + d[1] * f.u[1] + d[2] * f.u[2]), r5(d[0] * f.v[0] + d[1] * f.v[1] + d[2] * f.v[2]), r5(d[0] * f.w[0] + d[1] * f.w[1] + d[2] * f.w[2])];
};
const ell = (id: string, bone: string, look: Look, c: V3, radii: V3, extra: Json = {}): Json => ({ ...blobSolid({ id, bone, look, c, radii }, F), ...extra });
const box = (id: string, bone: string, look: Look, c: V3, size: V3, extra: Json = {}): Json => ({ id, kind: "box", bone, material: look.material, role: look.role ?? "body", tone: look.tone, size: size.map(r5), transform: { translation: local(bone, c) }, ...extra });
const mulch = (profile: [number, number][], id = "mulch"): Json => ({ id, kind: "extrude", bone: "root-plate", material: MULCH.material, role: "detail", tone: MULCH.tone, profile: profile.map(([v, w]) => [r5(v), r5(w)]), plane: "yz", from: -0.06, to: 3.06 });
const ofRef = (id: string, of: string, region: string): Json => ({ id, of, region });
const member = (s: Json, m: string, part: string, extra: Json = {}): Json => ({ ...s, member: m, part, ...extra });

/** 💨️ Transpiration outlets (D2): a lens on the summer flush of every plant, the face outlet on the first cushion, the core outlet on the second; anchors sit on the same points. */
const OUTLET_Y = 1.866;
const outletX = (k: number, which: 0 | 1, scale: number): number => (scale === 50 ? FLUSH_50[k]![which]![0] : XK(k) + (which === 0 ? -0.12 : 0.12));
function outlets(scale: number, top: number): Json[] {
  const thick = Math.max(0.008, (0.15 * scale) / 2000);
  return K.flatMap((k) => [
    ell(`th-transpiration-face-${k}`, `block-${k}`, { material: "hornbeam-leaf", tone: 0.45, role: "detail" }, [outletX(k, 0, scale), top, 0], [0.12, thick, 0.11], { region: "leaf-face", tags: ["transpiration-face"] }),
    ell(`th-transpiration-core-${k}`, `block-${k}`, { material: "hornbeam-leaf", tone: 0.45, role: "detail" }, [outletX(k, 1, scale), top, 0], [0.09, thick, 0.06], { region: "leaf-core", tags: ["transpiration-core"] }),
  ]);
}

/** 🦴️ The woody framework of one plant at 1:50: each stem continues as a leader to 1.70 m with four laterals that run out and up to the clipped faces and the top (hand-stated: height of origin, target across the row, along the row, height of the tip). */
const LATERALS: [number, number, number, number][] = [[0.62, 0.29, 0.06, 0.86], [0.95, -0.25, -0.07, 1.22], [1.28, 0.25, 0.08, 1.5], [1.56, -0.08, -0.06, 1.79]];
const leaderTube = (k: number, s: "a" | "b", id: string, look: Look, ys: number[], rs: number[]): Tube => {
  const sign = s === "a" ? -1 : 1;
  const top = stemAt(k, s, 0.64);
  return tube(id, `block-${k}`, look, ys.map((y, i) => {
    if (y <= 0.64) {
      const p = stemAt(k, s, y);
      return [p[0], y, p[2], rs[i]!];
    }
    const f = (y - 0.64) / (1.7 - 0.64);
    return [r4(top[0] + sign * 0.02 * f), y, r4(top[2] - sign * 0.05 * f), rs[i]!];
  }));
};
const leaderAt = (k: number, s: "a" | "b", y: number): V3 => leaderTube(k, s, "x", WOOD, [y], [0.01]).points[0]!.p;
function laterals(k: number, s: "a" | "b", n: number, r: number): Json[] {
  const sign = s === "a" ? -1 : 1;
  const rows = n === 4 ? LATERALS : [LATERALS[0]!, LATERALS[2]!];
  return rows.map(([y, zt, dx, yt], i) => {
    const a = leaderAt(k, s, y);
    const b: V3 = [r4(a[0] + dx * (k % 2 === 0 ? -1 : 1)), yt, r4(sign * zt)];
    const m: V3 = [r4((a[0] + b[0]) / 2), r4(a[1] + (yt - a[1]) * 0.42), r4(a[2] + (b[2] - a[2]) * 0.62)];
    const t = tube(`lateral-${k}-${s}${i + 1}`, `block-${k}`, { ...WOOD, tone: 0.42, flags: { slender: true } }, [[...a, r], [...m, r], [...b, r * 0.85]]);
    return member(tubeSolid(t, F), `block-${k}`, "member");
  });
}

function scale50(): Json {
  const body: Json[] = [
    ...panels50.map((p) => panelSolid(p, F)),
    ...K.flatMap((k) => flush50(k).map((b) => blobSolid(b, F))),
    ...stems50.map((t) => tubeSolid(t, F)),
    ...flares50.map((f) => flareSolid(f, F)),
    ...ridges50.map((t) => tubeSolid(t, F)),
    mulch(MULCH_PROFILE),
  ];
  const thermal: Json[] = [
    ...K.flatMap((k) => [ofRef(`th-block-${k}`, `block-${k}`, "leaf-face"), ofRef(`th-flush-${k}-a`, `flush-${k}-a`, "leaf-face"), ofRef(`th-flush-${k}-b`, `flush-${k}-b`, "leaf-face")]),
    ...cores50.map((p) => ({ ...panelSolid(p, F), region: "leaf-core" })),
    ...stems50.map((t) => ({ ...tubeSolid(clipTube(stemTube(Number(t.bone.slice(5)), t.id.endsWith("a") ? "a" : "b", [0, 0.06, 0.18, 0.33, 0.48, 0.5], `th-${t.id}`), 0.5), F), region: "stem-bark" })),
    ...flares50.map((f) => ofRef(`th-${f.id}`, f.id, "stem-bark")),
    ...outlets(50, OUTLET_Y),
  ];
  const structure: Json[] = [
    member(box("member-root-plate", "root-plate", WOOD, [0, 0.01, 0], [3.0, 0.016, 0.5], { cornerRadius: 0.006 }), "root-plate", "member"),
    member(box("support-base-support", "root-plate", { ...WOOD, tone: 0.25, role: "detail" }, [0, -0.01, 0], [3.0, 0.02, 0.6]), "root-plate", "support", { anchor: "base-support" }),
    member(ell("mass-root-plate", "root-plate", { ...WOOD, tone: 0.3, role: "detail" }, [0, 0.012, 0], [0.1, 0.012, 0.1]), "root-plate", "mass"),
    ...K.flatMap((k) => [
      member(ell(`joint-root-${k}`, `stem-${k}`, { ...WOOD, tone: 0.35, role: "detail" }, [XK(k), 0.03, 0], [0.055, 0.03, 0.05]), `stem-${k}`, "joint"),
      ...(["a", "b"] as const).flatMap((s) => [
        member(tubeSolid(stemTube(k, s, [0, 0.06, 0.18, 0.33, 0.37], `member-stem-${k}-${s}`, `stem-${k}`, WOOD, 0.9), F), `stem-${k}`, "member"),
        member(ell(`joint-block-${k}-${s}`, `block-${k}`, { ...WOOD, tone: 0.35, role: "detail" }, stemAt(k, s, 0.35), [0.026, 0.026, 0.026]), `block-${k}`, "joint"),
        member(tubeSolid(leaderTube(k, s, `member-block-${k}-${s}`, WOOD, [0.33, 0.48, 0.64, 0.95, 1.3, 1.7], [0.018, 0.016, 0.0145, 0.012, 0.0095, 0.0075]), F), `block-${k}`, "member"),
        ...laterals(k, s, 4, 0.0055),
      ]),
      member(ell(`mass-block-${k}`, `block-${k}`, { ...WOOD, tone: 0.3, role: "detail" }, [XK(k), 1.08, 0], [0.045, 0.045, 0.045]), `block-${k}`, "mass"),
    ]),
  ];
  return { status: "authored", denominator: 50, body, thermal, structure, decisions: DECISIONS[50] };
}

const PROFILE_100: [number, number, number][] = [[0.33, 0.272, 0.05], [0.375, 0.334, 0.012], [0.52, 0.35, 0], [1.1, 0.32, 0], [1.68, 0.281, 0], [1.79, 0.262, 0.012], [1.825, 0.225, 0.04], [1.838, 0.18, 0.065]];
const HX = (k: number): number => (k === 1 || k === 6 ? 0.315 : 0.38);
const CX = (k: number): number => (k === 1 ? -1.255 : k === 6 ? 1.255 : XK(k));
const panels100 = K.map((k) => panel(`block-${k}`, `block-${k}`, LEAF, PROFILE_100, CX(k), HX(k), 1, 6, [0, 0, 0, 0], LIFT(k)));
const flush100 = (k: number): Blob => ({ id: `flush-${k}`, bone: `block-${k}`, look: LEAF, c: [CX(k), 1.836, 0], radii: [k === 1 || k === 6 ? 0.27 : 0.3, 0.033, 0.15] });
const stems100 = K.flatMap((k) => (["a", "b"] as const).map((s) => stemTube(k, s, [0, 0.33, 0.62])));
const flares100 = K.map((k) => flare(k, [[0, 0.07], [0.04, 0.046], [0.09, 0.034]]));
const cores100 = K.map((k) => panel(`th-core-${k}`, `block-${k}`, { material: "hornbeam-leaf", tone: 0.4 }, [[0.48, 0.16, 0.07], [0.6, 0.205, 0.02], [1.3, 0.18, 0], [1.64, 0.12, 0.06]], CX(k), k === 1 || k === 6 ? 0.27 : 0.3, 1, 3));

function scale100(): Json {
  const body: Json[] = [
    ...panels100.map((p) => panelSolid(p, F)),
    ...K.map((k) => blobSolid(flush100(k), F)),
    ...stems100.map((t) => tubeSolid(t, F)),
    ...flares100.map((f) => flareSolid(f, F)),
    mulch([[0, -0.3], [0.018, -0.22], [0.03, 0], [0.018, 0.22], [0, 0.3]]),
  ];
  const thermal: Json[] = [
    ...K.flatMap((k) => [ofRef(`th-block-${k}`, `block-${k}`, "leaf-face"), ofRef(`th-flush-${k}`, `flush-${k}`, "leaf-face")]),
    ...cores100.map((p) => ({ ...panelSolid(p, F), region: "leaf-core" })),
    ...stems100.map((t) => ({ ...tubeSolid(clipTube({ ...t, id: `th-${t.id}` }, 0.5), F), region: "stem-bark" })),
    ...flares100.map((f) => ofRef(`th-${f.id}`, f.id, "stem-bark")),
    ...outlets(100, OUTLET_Y),
  ];
  const structure: Json[] = [
    member(box("member-root-plate", "root-plate", WOOD, [0, 0.012, 0], [3.0, 0.02, 0.5]), "root-plate", "member"),
    member(box("support-base-support", "root-plate", { ...WOOD, tone: 0.25, role: "detail" }, [0, -0.01, 0], [3.0, 0.02, 0.6]), "root-plate", "support", { anchor: "base-support" }),
    member(ell("mass-root-plate", "root-plate", { ...WOOD, tone: 0.3, role: "detail" }, [0, 0.012, 0], [0.1, 0.012, 0.1]), "root-plate", "mass"),
    ...K.flatMap((k) => [
      member(ell(`joint-root-${k}`, `stem-${k}`, { ...WOOD, tone: 0.35, role: "detail" }, [XK(k), 0.03, 0], [0.055, 0.03, 0.05]), `stem-${k}`, "joint"),
      ...(["a", "b"] as const).flatMap((s) => [
        member(tubeSolid(tube(`member-stem-${k}-${s}`, `stem-${k}`, WOOD, [[...stemAt(k, s, 0), 0.021], [...stemAt(k, s, 0.36), 0.019]]), F), `stem-${k}`, "member"),
        member(ell(`joint-block-${k}-${s}`, `block-${k}`, { ...WOOD, tone: 0.35, role: "detail" }, stemAt(k, s, 0.35), [0.025, 0.025, 0.025]), `block-${k}`, "joint"),
        member(tubeSolid(leaderTube(k, s, `member-block-${k}-${s}`, WOOD, [0.34, 0.62, 1.28, 1.66], [0.017, 0.015, 0.011, 0.009]), F), `block-${k}`, "member"),
        ...laterals(k, s, 2, 0.0065),
      ]),
      member(ell(`mass-block-${k}`, `block-${k}`, { ...WOOD, tone: 0.3, role: "detail" }, [XK(k), 1.08, 0], [0.045, 0.045, 0.045]), `block-${k}`, "mass"),
    ]),
  ];
  return { status: "authored", denominator: 100, body, thermal, structure, decisions: DECISIONS[100] };
}

const PROFILE_200: [number, number, number][] = [[0.33, 0.29, 0.04], [0.4, 0.345, 0.008], [0.55, 0.35, 0], [1.7, 0.28, 0], [1.81, 0.25, 0.02], [1.855, 0.18, 0.06]];
const panels200 = K.map((k) => panel(`block-${k}`, `block-${k}`, LEAF, PROFILE_200, CX(k), HX(k), 1, 5, [0, 0, 0, 0], LIFT(k)));
const cores200 = K.map((k) => panel(`th-core-${k}`, `block-${k}`, { material: "hornbeam-leaf", tone: 0.4 }, [[0.5, 0.17, 0.06], [1.0, 0.2, 0], [1.62, 0.15, 0.05]], CX(k), k === 1 || k === 6 ? 0.27 : 0.3, 1, 3));
const stems200 = K.map((k) => tube(`stem-${k}`, `stem-${k}`, BARK, [[XK(k), 0, 0, 0.045], [XK(k), 0.07, 0, 0.031], [XK(k), 0.55, 0, 0.027]]));

function scale200(): Json {
  const body: Json[] = [
    ...panels200.map((p) => panelSolid(p, F)),
    ...stems200.map((t) => tubeSolid(t, F)),
    box("mulch", "root-plate", MULCH, [0, 0.015, 0], [3.1, 0.03, 0.6]),
  ];
  const thermal: Json[] = [
    ...K.map((k) => ofRef(`th-block-${k}`, `block-${k}`, "leaf-face")),
    ...cores200.map((p) => ({ ...panelSolid(p, F), region: "leaf-core" })),
    ...K.flatMap((k) => (["a", "b"] as const).map((s) => ({ ...tubeSolid(tube(`th-stem-${k}-${s}`, `stem-${k}`, BARK, [[...stemAt(k, s, 0), 0.026], [...stemAt(k, s, 0.5), 0.02]]), F), region: "stem-bark" }))),
    ...outlets(200, 1.855),
  ];
  const structure: Json[] = [
    member(box("member-root-plate", "root-plate", WOOD, [0, 0.015, 0], [3.0, 0.03, 0.5]), "root-plate", "member"),
    member(box("support-base-support", "root-plate", { ...WOOD, tone: 0.25, role: "detail" }, [0, -0.015, 0], [3.0, 0.03, 0.6]), "root-plate", "support", { anchor: "base-support" }),
    member(ell("mass-root-plate", "root-plate", { ...WOOD, tone: 0.3, role: "detail" }, [0, 0.016, 0], [0.12, 0.016, 0.12]), "root-plate", "mass"),
    ...K.flatMap((k) => [
      member(tubeSolid(tube(`member-stem-${k}`, `stem-${k}`, WOOD, [[XK(k), 0, 0, 0.022], [XK(k), 0.36, 0, 0.02]]), F), `stem-${k}`, "member"),
      member(ell(`joint-block-${k}`, `block-${k}`, { ...WOOD, tone: 0.35, role: "detail" }, [XK(k), 0.35, 0], [0.03, 0.03, 0.03]), `block-${k}`, "joint"),
      member(tubeSolid(tube(`member-block-${k}`, `block-${k}`, WOOD, [[XK(k), 0.34, 0, 0.018], [XK(k), 1.66, 0, 0.016]]), F), `block-${k}`, "member"),
      member(ell(`mass-block-${k}`, `block-${k}`, { ...WOOD, tone: 0.3, role: "detail" }, [XK(k), 1.08, 0], [0.05, 0.05, 0.05]), `block-${k}`, "mass"),
    ]),
  ];
  return { status: "authored", denominator: 200, body, thermal, structure, decisions: DECISIONS[200] };
}

const PROFILE_500: [number, number, number][] = [[0.3, 0.255, 0.05], [0.34, 0.325, 0.015], [0.42, 0.348, 0], [0.56, 0.35, 0], [1.68, 0.281, 0], [1.78, 0.262, 0.01], [1.825, 0.225, 0.03], [1.85, 0.17, 0.06]];
const hedge500 = panel("hedge", "root-plate", LEAF, PROFILE_500, 0, 1.57, 1, 10);
const legs500 = K.map((k): Tube => tube(`stem-${k}`, `stem-${k}`, BARK, [[XK(k), 0, 0, 0.04], [XK(k), 0.36, 0, 0.04]]));

function scale500(): Json {
  const body: Json[] = [panelSolid(hedge500, F), ...legs500.map((t) => tubeSolid(t, F))];
  const thermal: Json[] = [
    ofRef("th-hedge", "hedge", "leaf-face"),
    box("th-core", "root-plate", { material: "hornbeam-leaf", tone: 0.4 }, [0, 1.07, 0], [2.9, 1.1, 0.38], { region: "leaf-core" }),
    ...K.map((k) => ({ ...tubeSolid(tube(`th-stem-${k}`, `stem-${k}`, BARK, [[XK(k), 0, 0, 0.04], [XK(k), 0.58, 0, 0.04]]), F), region: "stem-bark" })),
    ...outlets(500, 1.85),
  ];
  const structure: Json[] = [
    ...K.map((k) => member(box(`member-root-plate-${k}`, "root-plate", WOOD, [XK(k), 0.0375, 0], [0.08, 0.075, 0.08]), "root-plate", "member")),
    member(box("support-base-support", "root-plate", { ...WOOD, tone: 0.25, role: "detail" }, [0, -0.0375, 0], [3.0, 0.075, 0.6]), "root-plate", "support", { anchor: "base-support" }),
    ...K.flatMap((k) => [
      member(box(`member-stem-${k}`, `stem-${k}`, WOOD, [XK(k), 0.2, 0], [0.32, 0.075, 0.075]), `stem-${k}`, "member"),
      member(box(`member-block-${k}`, `block-${k}`, WOOD, [XK(k), 1.0, 0], [1.3, 0.075, 0.075]), `block-${k}`, "member"),
      member(ell(`mass-block-${k}`, `block-${k}`, { ...WOOD, tone: 0.3, role: "detail" }, [XK(k), 1.08, 0], [0.075, 0.075, 0.075]), `block-${k}`, "mass"),
    ]),
  ];
  return { status: "authored", denominator: 500, body, thermal, structure, decisions: DECISIONS[500] };
}

const DECISIONS: Record<number, Json[]> = {
  50: [
    T("Body: six clipped plants as superelliptic panel lofts on 14 hand-stated sections each (rounded underside at 0.33 m, widest 0.70 m at 0.56 m, faces battered 3.5 degrees to 0.56 m at 1.68 m, 7 cm shoulder, flat top at 1.84 m); the panels overlap by 13 to 31 cm and lie in one clipped plane, so the face reads as one hedge with the seams of the six plants; two low cushions of summer flush per plant reach 1.87 m; the clipped faces stay a smooth leaf skin (single leaves of 5 to 10 cm are drawn as texture in 2D, a repeated relief of them read as studs and was dropped); two fluted stems per plant of 4.5 to 5.5 cm at the collar with a root flare, bark ridges as texture, on a bark-mulch planting strip.", "Körper: sechs geschnittene Pflanzen als superelliptische Tafel-Lofts mit je 14 von Hand angegebenen Schnitten (gerundete Unterseite auf 0,33 m, größte Breite 0,70 m auf 0,56 m, Flanken 3,5 Grad geböscht bis 0,56 m auf 1,68 m, 7 cm Schulter, ebene Oberseite auf 1,84 m); die Tafeln überlappen 13 bis 31 cm und liegen in einer geschnittenen Ebene, sodass die Fläche als eine Hecke mit den Fugen der sechs Pflanzen liest; zwei flache Polster des Sommeraustriebs je Pflanze reichen bis 1,87 m; die geschnittenen Flanken bleiben eine glatte Blatthaut (Einzelblätter von 5 bis 10 cm sind in 2D als Textur gezeichnet, ein wiederholtes Relief daraus las sich als Noppen und entfiel); zwei gekehlte Stämme je Pflanze von 4,5 bis 5,5 cm am Wurzelhals mit Stammanlauf, Rindenrippen als Textur, auf einem Pflanzstreifen aus Rindenmulch."),
    T("Thermal: leaf-face is the clipped panels and the flush cushions by reference; leaf-core (hidden) is one inner loft per plant about 13 cm inside the faces; stem-bark is the twelve stems up to 0.5 m (the bark of the open foot and the first hand-breadth inside the foliage) and the six flares; the transpiration outlets are lenses on the flush cushions, one per anchor.", "Thermisch: leaf-face sind die geschnittenen Tafeln und die Austriebspolster per Verweis; leaf-core (verborgen) ist je Pflanze ein innerer Loft etwa 13 cm hinter den Flanken; stem-bark sind die zwölf Stämme bis 0,5 m (die Rinde des offenen Fußes und die erste Handbreite im Laub) und die sechs Stammanläufe; die Transpirationsauslässe sind Linsen auf den Austriebspolstern, eine je Anker."),
    T("Structure: the real load path of a clipped hedge: the root mat and the fixed planting-strip support, twelve stems with their collar joints, each stem continuing as a leader to 1.70 m with four laterals that run out to the clipped faces and the top, a joint where every stem enters its foliage block, and the point masses of the blocks and the root plate.", "Tragwerk: der wirkliche Lastpfad einer Schnitthecke: Wurzelteller und eingespanntes Auflager im Pflanzstreifen, zwölf Stämme mit ihren Wurzelhalsgelenken, jeder Stamm setzt sich als Leittrieb bis 1,70 m fort, mit vier Seitenästen bis an die geschnittenen Flanken und die Oberseite, ein Gelenk, wo jeder Stamm in seinen Laubblock eintritt, und die Punktmassen der Blöcke und des Wurzeltellers."),
  ],
  100: [
    T("Body modelled anew for 1:100: the same six clipped plants with eight sections each instead of fourteen, one flush cushion per plant, the two stems per plant as three-station lofts with a simpler flare, the mulch strip as a five-point dome, no bark ridges.", "Körper für 1:100 neu modelliert: dieselben sechs geschnittenen Pflanzen mit je acht statt vierzehn Schnitten, ein Austriebspolster je Pflanze, die zwei Stämme je Pflanze als Lofts mit drei Stationen und einfacherem Anlauf, der Mulchstreifen als Wölbung aus fünf Punkten, keine Rindenrippen."),
    T("Thermal: leaf-face by reference to panels and cushions, leaf-core as one four-section inner loft per plant, stem-bark the twelve stems up to 0.5 m and the flares, transpiration outlets as lenses on the cushions.", "Thermisch: leaf-face per Verweis auf Tafeln und Polster, leaf-core als ein innerer Loft mit vier Schnitten je Pflanze, stem-bark die zwölf Stämme bis 0,5 m und die Anläufe, Transpirationsauslässe als Linsen auf den Polstern."),
    T("Structure: every stem one straight tapered member with collar and block joints, each leader one tapered member to 1.66 m with two laterals, root mat, support and point masses.", "Tragwerk: jeder Stamm ein gerades, verjüngtes Glied mit Wurzelhals- und Blockgelenk, jeder Leittrieb ein verjüngtes Glied bis 1,66 m mit zwei Seitenästen, Wurzelteller, Auflager und Punktmassen."),
  ],
  200: [
    T("Body modelled anew for 1:200: six panels with six sections each (the 3.4 cm flush and the shoulder merge into a flat top at 1.855 m with a rounded edge), one stem per plant (the pair of 5 cm stems merged into one 5.4 cm stem with a flared foot), the mulch strip as a 3 cm slab.", "Körper für 1:200 neu modelliert: sechs Tafeln mit je sechs Schnitten (der Austrieb von 3,4 cm und die Schulter verschmelzen zu einer ebenen Oberseite auf 1,855 m mit gerundeter Kante), ein Stamm je Pflanze (das Paar 5-cm-Stämme zu einem 5,4-cm-Stamm mit Anlauf zusammengefasst), der Mulchstreifen als 3-cm-Platte."),
    T("Thermal: leaf-face by reference, leaf-core one three-section inner loft per plant, stem-bark as the real two stems per plant up to 0.5 m (the bark area does not merge with the drawn stems), transpiration outlets as lenses on the top.", "Thermisch: leaf-face per Verweis, leaf-core je Pflanze ein innerer Loft mit drei Schnitten, stem-bark als die wirklichen zwei Stämme je Pflanze bis 0,5 m (die Rindenfläche verschmilzt nicht mit den gezeichneten Stämmen), Transpirationsauslässe als Linsen auf der Oberseite."),
    T("Structure: one stem and one leader member per plant with the block joint, root mat, support and point masses; laterals and collar joints fall below the 30 mm feature.", "Tragwerk: je Pflanze ein Stamm- und ein Leittriebglied mit dem Blockgelenk, Wurzelteller, Auflager und Punktmassen; Seitenäste und Wurzelhalsgelenke liegen unter dem Merkmal von 30 mm."),
  ],
  500: [
    T("Body modelled anew for 1:500: the whole row is one clipped mass, a loft on eight horizontal sections with the battered faces, the flat top at 1.85 m, vertical ends with small rounded arrises; the six plants keep one leg each at the 75 mm feature, so the open foot of the hedge still reads.", "Körper für 1:500 neu modelliert: die ganze Reihe ist eine geschnittene Masse, ein Loft auf acht waagrechten Schnitten mit geböschten Flanken, ebener Oberseite auf 1,85 m und senkrechten Enden mit kleinen gerundeten Kanten; die sechs Pflanzen behalten je ein Bein in der Merkmalsgröße von 75 mm, sodass der offene Fuß der Hecke lesbar bleibt."),
    T("Thermal: leaf-face is the hedge mass by reference, leaf-core one inner box, stem-bark six stems of 80 mm up to 0.58 m (one per plant, carrying the bark area of the pair), transpiration outlets as lenses on the top.", "Thermisch: leaf-face ist die Heckenmasse per Verweis, leaf-core eine innere Box, stem-bark sechs Stämme von 80 mm bis 0,58 m (einer je Pflanze, mit der Rindenfläche des Paares), Transpirationsauslässe als Linsen auf der Oberseite."),
    T("Structure: square bars of 75 mm (the smallest feature) for the six stems and the six leaders, the root plate as six root balls under the stems, the support slab, point masses of the blocks.", "Tragwerk: Vierkantstäbe von 75 mm (das kleinste Merkmal) für die sechs Stämme und die sechs Leittriebe, der Wurzelteller als sechs Wurzelballen unter den Stämmen, die Auflagerplatte, Punktmassen der Blöcke."),
  ],
};

export const volumes = { "50": scale50(), "100": scale100(), "200": scale200(), "500": scale500() };

/** 🔪️ A ring clipped to the vertical strip lo <= x <= hi (Sutherland-Hodgman): each plant is drawn up to 2 cm past its seam with the neighbour, so the drawing shows one hedge with the plant seams. */
function clipStrip(ring: P2[], lo: number, hi: number): P2[] {
  const cut = (pts: P2[], inside: (p: P2) => boolean, at: (a: P2, b: P2) => P2): P2[] => pts.flatMap((p, i) => {
    const q = pts[(i + 1) % pts.length]!;
    if (inside(p)) return inside(q) ? [p] : [p, at(p, q)];
    return inside(q) ? [at(p, q)] : [];
  });
  const atX = (x: number) => (a: P2, b: P2): P2 => ({ x, y: a.y + ((b.y - a.y) * (x - a.x)) / (b.x - a.x) });
  return cut(cut(ring, (p) => p.x >= lo, atX(lo)), (p) => p.x <= hi, atX(hi));
}

/** ✂️ A tube cut at height `ymax` (the drawn part below the foliage). */
function clipTube(t: Tube, ymax: number): Tube {
  const pts: Tube["points"] = [];
  for (let i = 0; i < t.points.length; i++) {
    const a = t.points[i]!;
    if (a.p[1] <= ymax) pts.push(a);
    else {
      const b = t.points[i - 1]!;
      const f = (ymax - b.p[1]) / (a.p[1] - b.p[1]);
      pts.push({ p: [b.p[0] + (a.p[0] - b.p[0]) * f, ymax, b.p[2] + (a.p[2] - b.p[2]) * f], r: b.r + (a.r - b.r) * f });
      break;
    }
  }
  return { ...t, points: pts };
}

type Draw = { panels: Record<number, Panel>; flush: Record<number, Blob[]>; cores: Record<number, Panel>; stems: Record<number, Tube[]>; flares: Record<number, Flare | null>; mulch: { top: number; half: number } | null; hedge?: Panel };
const DRAW: Record<number, Draw> = {
  50: { panels: Object.fromEntries(K.map((k) => [k, panels50[k - 1]!])), flush: Object.fromEntries(K.map((k) => [k, flush50(k)])), cores: Object.fromEntries(K.map((k) => [k, cores50[k - 1]!])), stems: Object.fromEntries(K.map((k) => [k, stems50.filter((t) => t.bone === `stem-${k}`)])), flares: Object.fromEntries(K.map((k) => [k, flares50[k - 1]!])), mulch: { top: 0.028, half: 0.3 } },
  100: { panels: Object.fromEntries(K.map((k) => [k, panels100[k - 1]!])), flush: Object.fromEntries(K.map((k) => [k, [flush100(k)]])), cores: Object.fromEntries(K.map((k) => [k, cores100[k - 1]!])), stems: Object.fromEntries(K.map((k) => [k, stems100.filter((t) => t.bone === `stem-${k}`)])), flares: Object.fromEntries(K.map((k) => [k, flares100[k - 1]!])), mulch: { top: 0.03, half: 0.3 } },
  200: { panels: Object.fromEntries(K.map((k) => [k, panels200[k - 1]!])), flush: Object.fromEntries(K.map((k) => [k, []])), cores: Object.fromEntries(K.map((k) => [k, cores200[k - 1]!])), stems: Object.fromEntries(K.map((k) => [k, [stems200[k - 1]!]])), flares: Object.fromEntries(K.map((k) => [k, null])), mulch: { top: 0.03, half: 0.3 } },
  500: { panels: {}, flush: {}, cores: {}, stems: Object.fromEntries(K.map((k) => [k, [legs500[k - 1]!]])), flares: {}, mulch: null, hedge: hedge500 },
};

const STEM_STYLE = { role: "body", filled: true, tone: 0.35, region: "stem-bark", material: "hornbeam-bark" };
const MASS_STYLE = { role: "detail", filled: true, tone: 0.3 };

/** ✏️ Ruling E1: every body, core and stem shape of every scale and view redrawn from that scale's body; texture and detail strokes stay where the form under them did not move; the plinth of 1:100 to 1:500 is replaced by the stems and the mulch strip it stood for; the per-shape pen overrides (up to 1.8 mm, nine times the scale's pens) are dropped so every stroke takes its role's pen. */
function draw(): string[] {
  const report: string[] = [];
  for (const scale of [50, 100, 200, 500]) {
    const d = DRAW[scale]!;
    const feature = (0.15 * scale) / 1000;
    const tol = feature / 4;
    const rep = asset.representations[String(scale)];
    for (const view of ["elevation", "front", "plan"] as View[]) {
      const old: Json[] = rep.geometry[view];
      const next: Json[] = [];
      const note = { redrawn: [] as string[], removed: [] as string[], added: [] as string[] };
      const put = (shape: Json, rings: P2[][], closed = true): Json => {
        const loc = toDrawing(view, F[shape.bone]!, rings);
        const { d: _d, points: _p, closed: _c, x: _x, y: _y, w: _w, h: _h, r: _r, ...rest } = shape;
        if (view === "elevation") return { ...rest, kind: "path", d: pathOf(loc, closed) };
        if (loc.length === 1) return { ...rest, kind: "poly", points: pointsOf(loc[0]!), closed };
        return { ...rest, kind: "path", d: pathOf(loc, closed) };
      };
      const stemRings = (k: number): P2[][] => {
        const polys = [...d.stems[k]!.flatMap((t) => tubePolygons(view, clipTube(t, 0.35))), ...(d.flares[k] ? [flarePolygon(view, d.flares[k]!)] : [])];
        return outline(polys, tol, feature ** 2);
      };
      const mulchRings = (): P2[][] => {
        const m = d.mulch!;
        if (view === "elevation") return [[{ x: -1.56, y: 0 }, { x: 1.56, y: 0 }, { x: 1.56, y: m.top }, { x: -1.56, y: m.top }]];
        if (view === "front") return [scale === 200 ? [{ x: -m.half, y: 0 }, { x: m.half, y: 0 }, { x: m.half, y: m.top }, { x: -m.half, y: m.top }] : (scale === 50 ? MULCH_PROFILE : [[0, -0.3], [0.018, -0.22], [0.03, 0], [0.018, 0.22], [0, 0.3]] as [number, number][]).map(([v, w]) => ({ x: w, y: v }))];
        return [[{ x: -1.56, y: -m.half }, { x: 1.56, y: -m.half }, { x: 1.56, y: m.half }, { x: -1.56, y: m.half }]];
      };
      const coreRings = (k: number): P2[][] => outline(panelPolygons(view, d.cores[k]!), tol, feature ** 2);
      for (const shape of old) {
        const id: string = shape.id;
        let m: RegExpMatchArray | null;
        if (id === "plinth") note.removed.push(id);
        else if ((m = id.match(/^block-(\d)$/)) && shape.role === "body") {
          const k = Number(m[1]);
          const polys = [...panelPolygons(view, d.panels[k]!), ...d.flush[k]!.map((b) => blobPolygon(view, b))];
          const rings = outline(polys, tol, feature ** 2);
          next.push(put(shape, view === "front" ? rings : rings.map((r) => clipStrip(r, k === 1 ? -9 : XK(k) - 0.27, k === 6 ? 9 : XK(k) + 0.27))));
          note.redrawn.push(id);
        } else if ((m = id.match(/^core-(\d)$/))) {
          const k = Number(m[1]);
          next.push(put(shape, view === "front" ? coreRings(k) : coreRings(k).map((r) => clipStrip(r, k === 1 ? -9 : XK(k) - 0.25, k === 6 ? 9 : XK(k) + 0.25))));
          note.redrawn.push(id);
        } else if (id === "core-strip") {
          const rings = K.flatMap((k) => coreRings(k));
          next.push(put(shape, outline(rings, tol, feature ** 2)));
          note.redrawn.push(id);
        } else if ((m = id.match(/^stem-(\d)-([ab])$/))) {
          const k = Number(m[1]);
          const t = d.stems[k]!.find((x) => x.id === id)!;
          const polys = [...tubePolygons(view, clipTube(t, 0.35)), ...(m[2] === "a" && d.flares[k] ? [flarePolygon(view, d.flares[k]!)] : [])];
          next.push(put({ ...shape, ...STEM_STYLE, z: shape.z }, outline(polys, tol, feature ** 2)));
          note.redrawn.push(id);
        } else if ((m = id.match(/^stem-(\d)-bark$/))) {
          const k = Number(m[1]);
          const lines = ridges50.filter((r) => r.id.startsWith(`stem-${k}-a-`)).map((r) => clipTube(r, 0.33).points.map((q) => ({ x: view === "elevation" ? q.p[0] : q.p[2], y: q.p[1] })));
          const loc = toDrawing(view, F[shape.bone]!, view === "elevation" ? lines : [lines[0]!]);
          const { d: _d, points: _p, ...rest } = shape;
          next.push(view === "elevation" ? { ...rest, kind: "path", d: pathOf(loc, false) } : { ...rest, kind: "poly", points: pointsOf(loc[0]!), closed: false });
          note.redrawn.push(id);
        } else if ((m = id.match(/^stem-(\d)$/))) {
          next.push(put({ ...shape, ...STEM_STYLE, z: 1.9 }, stemRings(Number(m[1]))));
          note.redrawn.push(id);
        } else if (id === "root-plate-mass") {
          next.push(put(shape, mulchRings()));
          note.redrawn.push(id);
        } else if (id === "strip") {
          const m2 = DRAW[scale]!.mulch!;
          const pts = view === "plan" ? [[-1.55, -m2.half], [1.55, -m2.half]] : view === "front" ? [[-m2.half + 0.01, 0.02], [m2.half - 0.01, 0.02]] : [[-0.05, 0.02], [3.05, 0.02]];
          next.push({ ...shape, points: pts });
          note.redrawn.push(id);
        } else if (id === "hedge") {
          next.push(put(shape, outline(panelPolygons(view, d.hedge!), tol, feature ** 2)));
          note.redrawn.push(id);
        } else next.push(shape);
      }
      const has = (id: string) => next.some((s) => s.id === id);
      if (d.mulch && view !== "plan" && !has("root-plate-mass")) {
        next.unshift(put({ id: "root-plate-mass", bone: "root-plate", ...MASS_STYLE, z: 1.95 }, mulchRings()));
        note.added.push("root-plate-mass");
      }
      if (view !== "plan" && scale >= 100) for (const k of K) if (!has(`stem-${k}`)) {
        next.push(put({ id: `stem-${k}`, bone: `stem-${k}`, ...STEM_STYLE, z: 1.9 + k / 100 }, stemRings(k)));
        note.added.push(`stem-${k}`);
      }
      for (const shape of next) delete shape.penMm;
      rep.geometry[view] = next;
      report.push(`1:${scale} ${view}: redrawn ${note.redrawn.length}, removed ${note.removed.join(" ") || "-"}, added ${note.added.join(" ") || "-"}`);
    }
  }
  return report;
}

/** 🗒️ The root members besides the drawings: anchors on the flush, the mulch material, summary, 2D decisions, plan symbol. */
function roots(): void {
  for (const a of asset.anchors) {
    const m = String(a.id).match(/^tp-(face|core)-(\d)$/);
    if (!m) continue;
    const k = Number(m[2]);
    const l = local(`block-${k}`, [outletX(k, m[1] === "face" ? 0 : 1, 50), OUTLET_Y + 0.004, 0]);
    a.x = r4(l[0]);
    a.y = r4(l[1]);
  }
  if (!asset.materials.some((m: Json) => m.id === "bark-mulch")) asset.materials.push({ id: "bark-mulch", title: T("Bark mulch on the planting strip", "Rindenmulch auf dem Pflanzstreifen"), emissivity: 0.95, solarAbsorptivity: 0.85, densityKgM3: 250, conductivityWmK: 0.07, source: "authored-estimate", reference: "jones-2014" });
  asset.summary = T(
    "A clipped hornbeam hedge segment (Carpinus betulus) 3.14 m long and 1.87 m high: six plants clipped into one dense, flat-topped mass, 0.70 m thick at the base and battered to 0.56 m at the shoulder so light reaches the lower foliage, with a low summer flush on the top; the lowest 0.35 m is open, so the grey, fluted stems and the mulched planting strip show at large scale.",
    "Ein Abschnitt einer geschnittenen Hainbuchenhecke (Carpinus betulus) von 3,14 m Länge und 1,87 m Höhe: sechs Pflanzen zu einer dichten, flach abgeschlossenen Masse geschnitten, am Fuß 0,70 m dick und bis zur Schulter auf 0,56 m geböscht, damit Licht an das untere Laub gelangt, mit einem flachen Sommeraustrieb auf der Oberseite; die untersten 0,35 m sind offen, sodass die grauen, gekehlten Stämme und der gemulchte Pflanzstreifen im großen Maßstab sichtbar sind.",
  );
  const R = asset.representations;
  R["50"].decisions.silhouette = T(
    "At 1:50 (1 m is 20 mm) the hornbeam hedge is drawn in full: six clipped plants with battered faces, a rounded shoulder and the low summer flush on the top, leaf texture, shaded core outlines, two fluted stems per plant with bark lines below 0.35 m and the mulched planting strip. Front and plan: front shows the battered section (0.70 m at the base, 0.56 m at the shoulder) over twelve stems; plan shows the six clipped tops with the shallow plant seams.",
    "Bei 1:50 (1 m sind 20 mm) wird die Hainbuchenhecke vollständig gezeichnet: sechs geschnittene Pflanzen mit geböschten Flanken, gerundeter Schulter und dem flachen Sommeraustrieb auf der Oberseite, Blatttextur, Kernumrisse, zwei gekehlte Stämme je Pflanze mit Rindenlinien unter 0,35 m und der gemulchte Pflanzstreifen. Vorder- und Draufsicht: die Vorderansicht zeigt den geböschten Querschnitt (0,70 m am Fuß, 0,56 m an der Schulter) über zwölf Stämmen; die Draufsicht zeigt die sechs geschnittenen Oberseiten mit den flachen Pflanzenfugen.",
  );
  R["100"].decisions.silhouette = T(
    "At 1:100 (1 m is 10 mm) the six clipped plants keep the battered section and one flush cushion each; three leaf marks per block; the two stems of every plant are drawn as one filled stem outline with its flare over the mulch strip. Front and plan: front adds the hatched core, leaf strokes and the stems; plan adds the hatched core of every block and leaf strokes.",
    "Bei 1:100 (1 m sind 10 mm) behalten die sechs geschnittenen Pflanzen den geböschten Querschnitt und je ein Austriebspolster; drei Blattzeichen je Block; die zwei Stämme jeder Pflanze werden als ein gefüllter Stammumriss mit Anlauf über dem Mulchstreifen gezeichnet. Vorder- und Draufsicht: die Vorderansicht ergänzt den schraffierten Kern, Blattstriche und die Stämme; die Draufsicht ergänzt den schraffierten Kern jedes Blocks und Blattstriche.",
  );
  R["100"].decisions.retained = T("Retained: six clipped plants with battered faces and summer flush, each with its inner shaded core outline; the stems of every plant as one outline over the mulch strip.", "Beibehalten: sechs geschnittene Pflanzen mit geböschten Flanken und Sommeraustrieb, jede mit dem Umriss ihres beschatteten Kerns; die Stämme jeder Pflanze als ein Umriss über dem Mulchstreifen.");
  R["200"].decisions.silhouette = T(
    "At 1:200 (1 m is 5 mm) the hornbeam hedge is drawn as six clipped plants with one flat top and the battered faces, each with its inner shaded core outline, on six single stems over the mulch strip. Front and plan: front shows the battered section over the stems; plan shows the six clipped tops.",
    "Bei 1:200 (1 m sind 5 mm) wird die Hainbuchenhecke als sechs geschnittene Pflanzen mit einer ebenen Oberseite und geböschten Flanken gezeichnet, jede mit dem Umriss ihres beschatteten Kerns, auf sechs einzelnen Stämmen über dem Mulchstreifen. Vorder- und Draufsicht: die Vorderansicht zeigt den geböschten Querschnitt über den Stämmen; die Draufsicht zeigt die sechs geschnittenen Oberseiten.",
  );
  R["200"].decisions.retained = T("Retained: the open foot with one stem per plant (the pair merged into one outline 5.4 cm wide) and the mulch strip.", "Beibehalten: der offene Fuß mit einem Stamm je Pflanze (das Paar zu einem 5,4 cm breiten Umriss zusammengefasst) und der Mulchstreifen.");
  R["200"].decisions.omitted = T("Omitted: the summer flush (3.4 cm, merged into the flat top), the second stem of every plant, bark lines and all texture strokes.", "Weggelassen: der Sommeraustrieb (3,4 cm, in die ebene Oberseite übernommen), der zweite Stamm jeder Pflanze, Rindenlinien und alle Texturstriche.");
  R["200"].decisions.overlays = T("Every thermal region keeps its own shape: stem-bark (the six stems); overlays follow the bone-bound masses, so sway and loads act on the drawn shapes.", "Jede thermische Region behält ihre eigene Form: stem-bark (die sechs Stämme); Überlagerungen folgen den knochengebundenen Massen, sodass Schwingung und Lasten auf die gezeichneten Formen wirken.");
  R["500"].decisions.silhouette = T(
    "The hornbeam hedge is composed as one clipped mass with a flat top and vertical ends, 3.14 m by 1.85 m, on six legs; 1 m is 2 mm, so only the overall form and the open foot are drawn, with a restrained closed outline and no internal structure. Front and plan: front shows the battered section on one leg; plan shows one clipped band, 0.70 m deep.",
    "Die Hainbuchenhecke ist als eine geschnittene Masse mit ebener Oberseite und senkrechten Enden, 3,14 m mal 1,85 m, auf sechs Beinen komponiert; 1 m sind 2 mm, daher werden nur die Gesamtform und der offene Fuß mit zurückhaltendem geschlossenem Umriss und ohne innere Struktur gezeichnet. Vorder- und Draufsicht: die Vorderansicht zeigt den geböschten Querschnitt auf einem Bein; die Draufsicht zeigt ein geschnittenes Band, 0,70 m tief.",
  );
  R["500"].decisions.retained = T("Retained: the open foot of the hedge, one leg per plant at the 75 mm minimum feature.", "Beibehalten: der offene Fuß der Hecke, ein Bein je Pflanze in der kleinsten Merkmalsgröße von 75 mm.");
  R["500"].decisions.omitted = T("Omitted: the plant seams, the flush, the second stems and the mulch strip; they are far below 0.1 mm line width at this scale.", "Weggelassen: die Pflanzenfugen, der Austrieb, die zweiten Stämme und der Mulchstreifen; sie liegen bei diesem Maßstab weit unter 0,1 mm Strichbreite.");
  R["500"].decisions.thinElements = T("The stems (5 cm) are thickened to the 75 mm minimum feature, one leg per plant; nothing else is thickened.", "Die Stämme (5 cm) werden auf das kleinste Merkmal von 75 mm verstärkt, ein Bein je Pflanze; sonst wird nichts verstärkt.");
  R["500"].decisions.styles = T("Light line: closed outline only. Dark solid: the hedge mass reads as one dark band, the legs lighter. Both stay readable at 36 mm printed height or less.", "Helle Linie: nur geschlossener Umriss. Dunkel flächig: die Heckenmasse liest als ein dunkles Band, die Beine heller. Beides bleibt bei 36 mm Druckhöhe oder weniger lesbar.");
  R["500"].decisions.overlays = T("Thermal and structural overlays map onto: stem-bark (the six legs) and, merged in the hedge path, leaf-face and leaf-core. Merged paths show the area-weighted value of their regions, labelled as merged.", "Thermische und statische Überlagerungen bilden ab auf: stem-bark (die sechs Beine) und, im Heckenpfad zusammengefasst, leaf-face und leaf-core. Zusammengefasste Pfade zeigen den flächengewichteten Wert ihrer Regionen, als zusammengefasst beschriftet.");
  const sym = asset.symbols.find((s: Json) => s.id === "hornbeam-hedge-symbol");
  if (sym) {
    const front = [[-0.3, 0], [0.3, 0], [0.33, 0.35], [0.35, 0.56], [0.281, 1.68], [0.262, 1.79], [0.2, 1.87], [-0.2, 1.87], [-0.262, 1.79], [-0.281, 1.68], [-0.35, 0.56], [-0.33, 0.35]];
    const elevation = [[-1.57, 0], [1.57, 0], [1.57, 1.8], [1.5, 1.87], [-1.5, 1.87], [-1.57, 1.8]];
    sym.geometry.front[0].points = front;
    sym.geometry.elevation[0].points = elevation;
  }
}

const report = draw();
roots();
const merged = { ...asset, volumes };
const source = { volumes };
writeFileSync(sourcePath(entry), JSON.stringify(source, null, 2) + "\n");
const out = join(GENERATED, "r3-hornbeam-hedge");
mkdirSync(out, { recursive: true });
const doc = { ...JSON.parse(readFileSync(entry.path, "utf8")), asset: merged };
writeFileSync(join(out, "🔣️.json"), JSON.stringify(doc, null, 2) + "\n");
const count = (s: Json) => `${s.body.length}/${s.thermal.length}/${s.structure.length}`;
console.log(`volumes source written; body/thermal/structure: 1:50 ${count(volumes["50"])}, 1:100 ${count(volumes["100"])}, 1:200 ${count(volumes["200"])}, 1:500 ${count(volumes["500"])}`);
for (const line of report) console.log(line);
