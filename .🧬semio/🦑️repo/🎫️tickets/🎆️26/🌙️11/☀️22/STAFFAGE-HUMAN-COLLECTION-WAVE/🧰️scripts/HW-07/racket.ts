/** 🎾️ The tennis racket of `tennis-player`: one handcrafted design per drawing scale and per volume scale, in the frame of the `racket` bone (u along
 * the shaft from the centre of the grip towards the head, v across the string bed, w the string-bed normal; rest pose: face in the sagittal plane).
 * Design input: ITF Rules of Tennis, Appendix II (overall length at most 73.7 cm, strung surface at most 39.4 x 29.2 cm); a typical 27 in (68.6 cm)
 * midplus frame of 645 cm2 (100 sq in) head, 23 mm beam, 300 g strung, balance 33 cm from the butt.
 * @see https://www.itftennis.com/en/about-us/governance/rules-and-regulations/ — ITF Rules of Tennis, Appendix II */
import { r, tx, type Json } from "./lib.ts";

export const RACKET = {
  butt: -0.065, tip: 0.62, headCentre: 0.45, outer: [0.17, 0.13] as const, inner: [0.157, 0.117] as const,
  beamW: 0.023, stringW: 0.002, throatFrom: 0.125, throatTo: 0.3, gripTo: 0.11, gripHalf: 0.0155, gripDepth: 0.027,
  massKg: 0.3, balanceFromButt: 0.33,
};
const R = RACKET;
const ellipse = (cu: number, a: number, b: number, n = 48): number[][] => Array.from({ length: n }, (_, i) => [r(cu + a * Math.cos((2 * Math.PI * i) / n), 5), r(b * Math.sin((2 * Math.PI * i) / n), 5)]);
const rect = (u0: number, u1: number, h: number): number[][] => [[u0, -h], [u1, -h], [u1, h], [u0, h]];
/** 🛶️ The paddle outline of the coarse scales: head oval, throat and handle as one form, the handle at least `minHalf` wide. */
const paddle = (minHalf: number): number[][] => {
  const head = Array.from({ length: 33 }, (_, i) => { const t = -Math.PI * 0.78 + (i / 32) * Math.PI * 1.56; return [r(R.headCentre + R.outer[0] * Math.cos(t), 5), r(R.outer[1] * Math.sin(t), 5)]; });
  return [[R.butt, -minHalf], ...head, [R.butt, minHalf]];
};

type Part = { id: string; points: number[][]; half: number; tone: number; material: string; role?: string; hatch?: string; z: number; filled?: boolean };
/** 🖊️ The racket parts drawn at one scale (shared by elevation, front and plan; every part is an extruded solid so it turns with the racket). */
export function racketParts(scale: string): Part[] {
  const frame = { id: "racket-frame", points: ellipse(R.headCentre, R.outer[0], R.outer[1]), half: R.beamW / 2, tone: 0.18, material: "racket-composite", z: 9 };
  const strings = { id: "racket-strings", points: ellipse(R.headCentre, R.inner[0], R.inner[1]), half: R.stringW / 2, tone: 0.93, material: "string-polyamide", z: 9.1 };
  if (scale === "50") return [
    frame, { ...strings, hatch: "strings" },
    { id: "racket-throat-a", points: [[0.118, 0.006], [0.333, 0.098], [0.337, 0.079], [0.16, 0.0]], half: 0.01, tone: 0.18, material: "racket-composite", z: 8.9 },
    { id: "racket-throat-b", points: [[0.118, -0.006], [0.16, 0.0], [0.337, -0.079], [0.333, -0.098]], half: 0.01, tone: 0.18, material: "racket-composite", z: 8.9 },
    { id: "racket-shaft", points: rect(R.gripTo - 0.005, 0.14, 0.0125), half: 0.011, tone: 0.18, material: "racket-composite", z: 8.8 },
    { id: "racket-grip", points: rect(R.butt + 0.007, R.gripTo, R.gripHalf), half: R.gripDepth / 2, tone: 0.06, material: "grip-tape", z: 8.7 },
    { id: "racket-butt", points: rect(R.butt, R.butt + 0.008, R.gripHalf + 0.0015), half: R.gripDepth / 2 + 0.0015, tone: 0.18, material: "racket-composite", z: 8.75 },
    { id: "racket-grip-seam", points: [[R.butt + 0.03, -R.gripHalf], [R.butt + 0.05, R.gripHalf]], half: R.gripDepth / 2, tone: 0.3, material: "grip-tape", role: "texture", z: 8.71, filled: false },
  ];
  if (scale === "100") return [
    { ...frame, id: "racket-head", points: ellipse(R.headCentre, R.outer[0], R.outer[1]), tone: 0.88, material: "string-polyamide", hatch: "strings" },
    { id: "racket-throat", points: [[0.118, -0.009], [0.336, -0.095], [0.336, 0.095], [0.118, 0.009]], half: 0.01, tone: 0.18, material: "racket-composite", z: 8.9 },
    { id: "racket-grip", points: rect(R.butt, R.gripTo + 0.01, R.gripHalf), half: R.gripDepth / 2, tone: 0.06, material: "grip-tape", z: 8.7 },
  ];
  const half = scale === "200" ? 0.021 : 0.062;
  return [{ id: "racket", points: paddle(half), half: scale === "200" ? 0.005 : 0.016, tone: scale === "200" ? 0.5 : 0, material: "racket-composite", z: 8.9 }];
}

/** ✏️ The racket shapes of one scale (one list for each of the three views) with solids swept across the string-bed normal around the rest lateral position `restZ` of the racket bone. */
export function racketShapes(scale: string, restZ: number): Json[] {
  return racketParts(scale).map((p) => ({
    id: p.id, bone: "racket", role: p.role ?? "body", z: p.z, filled: p.filled ?? true, tone: p.tone, region: "racket", material: p.material, ...(p.hatch ? { hatch: p.hatch } : {}),
    kind: "poly", points: p.filled === false ? ribbonOf(p.points) : p.points, closed: true,
    solid: { kind: "extruded", sweep: "w", z0: r(restZ - p.half, 5), z1: r(restZ + p.half, 5) },
  }));
}
const ribbonOf = (pts: number[][]): number[][] => { const [a, b] = pts as [number[], number[]]; const dx = b[0]! - a[0]!, dy = b[1]! - a[1]!, l = Math.hypot(dx, dy), nx = (-dy / l) * 0.001, ny = (dx / l) * 0.001; return [[a[0]! + nx, a[1]! + ny], [b[0]! + nx, b[1]! + ny], [b[0]! - nx, b[1]! - ny], [a[0]! - nx, a[1]! - ny]].map((p) => [r(p[0]!, 5), r(p[1]!, 5)]); };

/** 🧊️ The racket solids of one volume scale: body solids and their thermal references (region `racket`) and the structural member. */
export function racketVolume(scale: string): { body: Json[]; thermal: Json[]; structure: Json[] } {
  const base = { bone: "racket", role: "body" };
  const ell = (a: number, b: number, n = 40) => ellipse(R.headCentre, a, b, n);
  const ext = (id: string, profile: number[][], half: number, material: string, tone: number, extra: Json = {}) => ({ id, kind: "extrude", ...base, material, tone, profile, plane: "xy", from: -half, to: half, ...extra });
  let body: Json[];
  if (scale === "50" || scale === "100") {
    body = [
      ext("racket-frame", ell(R.outer[0], R.outer[1]), R.beamW / 2, "racket-composite", 0.18, { holes: [ell(R.inner[0], R.inner[1])] }),
      ext("racket-strings", ell(R.inner[0] + 0.001, R.inner[1] + 0.001), R.stringW / 2, "string-polyamide", 0.93, { sheet: true }),
      ...(scale === "50"
        ? [ext("racket-throat-a", [[0.118, 0.006], [0.16, 0.0], [0.337, 0.079], [0.333, 0.098]], 0.01, "racket-composite", 0.18), ext("racket-throat-b", [[0.118, -0.006], [0.333, -0.098], [0.337, -0.079], [0.16, 0.0]], 0.01, "racket-composite", 0.18), { id: "racket-shaft", kind: "box", ...base, material: "racket-composite", tone: 0.18, size: [0.04, 0.025, 0.022], transform: { translation: [0.125, 0, 0] } }]
        : [ext("racket-throat", [[0.118, -0.009], [0.336, -0.095], [0.336, 0.095], [0.118, 0.009]], 0.01, "racket-composite", 0.18)]),
      { id: "racket-grip", kind: "box", ...base, material: "grip-tape", tone: 0.06, size: [R.gripTo - R.butt, 2 * R.gripHalf, R.gripDepth], cornerRadius: 0.006, transform: { translation: [r((R.gripTo + R.butt) / 2, 5), 0, 0] } },
    ];
  } else {
    const half = scale === "200" ? 0.021 : 0.062;
    body = [ext("racket", paddle(half).map((p) => [p[0]!, p[1]!]).reverse(), 0.005, "racket-composite", scale === "200" ? 0.5 : 0, { sheet: true })];
  }
  const thermal = body.map((s) => ({ id: `th-${s.id}`, of: s.id, region: "racket" }));
  const structure = [{ id: "st-racket", kind: "capsule", ...base, material: "racket-composite", tone: 0.85, p0: [R.butt + 0.01, 0, 0], p1: [R.tip - 0.01, 0, 0], r0: 0.012, r1: 0.012, slender: true, member: "racket", part: "member" }];
  return { body, thermal, structure };
}

export const RACKET_MATERIALS = [
  { id: "racket-composite", title: tx("Graphite-epoxy racket frame", "Rahmen aus Graphit-Epoxid"), emissivity: 0.9, solarAbsorptivity: 0.85, densityKgM3: 1550, source: "authored-estimate" },
  { id: "string-polyamide", title: tx("Polyamide (synthetic gut) strings", "Saiten aus Polyamid (synthetischer Darm)"), emissivity: 0.9, solarAbsorptivity: 0.3, densityKgM3: 1140, source: "authored-estimate" },
  { id: "grip-tape", title: tx("Polyurethane grip tape", "Griffband aus Polyurethan"), emissivity: 0.92, solarAbsorptivity: 0.9, source: "authored-estimate" },
];
