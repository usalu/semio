/** 🌡️ Thermal list at 1:50: every garment as nested layer shells under its outer surface (innermost first, real thickness), the uncovered skin as its own patches (the face as a mask loft cut from the head sections, its back a section face through the head that continues under the hair), the stroller regions as references, and the five outlets flush with the surface they leave from.
 * @see ./r3-stroller-s50-figure.ts */
import { E, dense, inset, th, type Json, type Row } from "./r3-stroller-kit.ts";
import { HEAD_ROWS } from "./r3-stroller-s50-figure.ts";

export const LAYERS: Record<string, [string, number][]> = {
  "torso-clothing": [["tj-air", 0.004], ["tj-shirt", 0.0012], ["tj-sweater", 0.003], ["tj-jacket", 0.012]],
  "arm-clothing": [["aj-air", 0.004], ["aj-shirt", 0.0012], ["aj-sweater", 0.003], ["aj-jacket", 0.012]],
  "leg-clothing": [["lj-air", 0.004], ["lj-briefs", 0.0008], ["lj-leggings", 0.002], ["lj-jeans", 0.0012]],
  shoes: [["sh-sock", 0.002], ["sh-air", 0.004], ["sh-upper", 0.004]],
  hair: [["hair-pelt", 0.015]],
  "child-clothing": [["cs-snowsuit", 0.012]],
};
const NESTED = /^(jacket-body|jacket-skirt|sleeve-(upper|lower)-[lr]|jeans-(seat|thigh-[lr]|shank-[lr])|shoe-[lr])$/;
const BRIEFS = /^jeans-seat$/;

/** 🎭️ The face as a loft from the coronal plane `v0` forward: each section is the cut of the stated head sections with a plane of constant v (half width = widest lateral reach, half height = half the cut length along u). */
export function faceMask(id: string, rows: Row[], vs: number[], region: string): Json {
  const d = dense(rows, 4);
  const sections = vs.map((v) => {
    let lo = Infinity, hi = -Infinity, wide = 0;
    for (const r of d) { const t = Math.abs(v - r[3]!) / r[2]!; if (t >= 1) continue; const e = r[5] ?? 2, w = r[1]! * Math.pow(1 - Math.pow(t, e), 1 / e); lo = Math.min(lo, r[0]!); hi = Math.max(hi, r[0]!); wide = Math.max(wide, w); }
    return { at: [Math.round(((lo + hi) / 2) * 1e5) / 1e5, v, 0], rx: Math.round(wide * 1e5) / 1e5, ry: Math.round(((hi - lo) / 2) * 1e5) / 1e5, exponent: 2.2, rotation: [-0.5, -0.5, -0.5, 0.5] };
  });
  return { id, kind: "loft", bone: "head", material: "skin", role: "body", tone: 0.84, sections, capStart: true, capEnd: true, region, sectionFaces: ["+v"] };
}

/** ✂️ The outer surface a garment solid really shows: a sleeve without its shoulder cap (the armhole is open to the jacket, a section face), a trouser leg without its hip cap (it continues into the seat) and of the trouser seat only the crotch below the jacket hem (above it the jeans lie under the jacket skirt, which is the outer surface of the pelvis); `null` for every other solid. */
export function outerPart(s: Json, id: string, region: string, layer?: string): Json | null {
  const cut = /^(sleeve-upper|jeans-thigh)-[lr]$/.test(s.id) ? { keep: (u: number) => u >= 0, face: "-u" } : s.id === "jeans-seat" ? { keep: (u: number) => u <= -0.068, face: "+u" } : null;
  if (cut === null) return null;
  const { _r, ...solid } = s;
  return { ...solid, id, sections: solid.sections.filter((q: Json) => cut.keep(q.at[0])), region, ...(layer ? { layer } : {}), sectionFaces: [cut.face] };
}

export function thermal50(tagged: Json[]): Json[] {
  const out: Json[] = [];
  for (const s of tagged) {
    const region = s._r as string, layers = LAYERS[region];
    if (s.id === "head" || s.id === "child-head" || s.id === "child-face") continue;
    if (layers === undefined) { out.push(th(`th-${s.id}`, s.id, region)); continue; }
    const outer = layers[layers.length - 1]!;
    out.push(outerPart(s, `th-${s.id}`, region, outer[0]) ?? th(`th-${s.id}`, s.id, region, outer[0]));
    if (!NESTED.test(s.id)) continue;
    let depth = 0;
    for (let i = layers.length - 2; i >= 0; i--) {
      depth += layers[i + 1]![1];
      if (layers[i]![0] === "lj-briefs" && !BRIEFS.test(s.id)) continue;
      const { _r, ...solid } = s;
      out.push({ ...inset(solid, depth, `th-${s.id}-${layers[i]![0]}`), region, layer: layers[i]![0] });
    }
  }
  out.push(faceMask("th-face", HEAD_ROWS, [-0.012, -0.03, -0.05, -0.068, -0.08, -0.088, -0.092], "head-skin"));
  out.push({ id: "th-child-face", kind: "loft", bone: "child", material: "skin", role: "body", tone: 0.88, sections: [[0.002, 0.322, 0.056, 0.062], [0.02, 0.32, 0.05, 0.055], [0.035, 0.318, 0.04, 0.044], [0.046, 0.316, 0.026, 0.03], [0.052, 0.316, 0.012, 0.014]].map(([u, v, rw, rv]) => ({ at: [u, v, 0.1925], rx: rw, ry: rv, exponent: 2.1, rotation: [0, 0.70711, 0, 0.70711] })), capStart: true, capEnd: true, region: "child-skin", sectionFaces: ["-u"] });
  const outlet = (id: string, bone: string, at: [number, number, number], radii: [number, number, number], region: string): Json => ({ ...E(`th-${id}`, bone, "still-air", 0.9, at, radii, { role: "detail" }), region, tags: [id, "emitter"] });
  out.push(outlet("breath-nose", "head", [0.107, -0.1, 0], [0.004, 0.004, 0.011], "head-skin"));
  out.push(outlet("breath-mouth", "head", [0.083, -0.092, 0], [0.004, 0.004, 0.0062], "head-skin"));
  out.push(outlet("plume", "head", [0.234, 0.013, 0], [0.046, 0.07, 0.06], "torso-clothing"));
  out.push(outlet("child-nose-breath", "child", [0.056, 0.313, 0.1925], [0.004, 0.004, 0.006], "child-skin"));
  out.push(outlet("child-mouth-breath", "child", [0.045, 0.288, 0.1925], [0.004, 0.004, 0.006], "child-skin"));
  return out;
}
