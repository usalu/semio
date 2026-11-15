/** 🦌️ Author 29: hand-sculpted details of the roe deer (ears, 1:500 legs, 1:500 body) on top of the fitted lists. Usage: bun author-29-roe.ts <50|100|200|500> [--write] */
import { Sculpt, type Look } from "./author-29-lib.ts";
const scale = Number(process.argv[2]);
const write = process.argv.includes("--write");
const S = new Sculpt("animals/roe-deer", scale);
const range = (a: number, b: number, n: number) => Array.from({ length: n }, (_, i) => a + ((b - a) * i) / (n - 1));
const COAT = (tone: number): Look => ({ material: "summer-coat", tone });
const norm = (v: [number, number, number]): [number, number, number] => { const l = Math.hypot(...v); return [v[0] / l, v[1] / l, v[2] / l]; };
const earLook = (tone: number) => COAT(tone);

if (scale <= 200) {
  S.keep(["far-ear", "near-ear"]);
  for (const [sd, id, z, tone] of [["f", "far-ear", 0.073, 0.5], ["n", "near-ear", -0.073, 0.6]] as const) {
    const s = Math.sign(z);
    const zz = scale === 200 ? z * 0.84 : z;
    const e = scale === 200 ? [0.9, 0.025] : [0.9, 0.025];
    void e;
    const dir = norm([0.6, 0.78, s * 0.2]);
    const w = norm([0.78, -0.6, 0]);
    S.ell(id, `ear-${sd}`, [1.2, 1.007, zz], scale === 200 ? [0.036, 0.097, 0.024] : [0.03, 0.09, 0.019], earLook(tone), [w, dir]);
  }
  console.log(`roe 1:${scale}: ears rebuilt, ${S.body.length} body solids`);
  if (write) S.save();
}

if (scale === 500) {
  const legs = [["far-hind-leg", "shank-hf", "cannon-hf", "hoof-hf", 0.06, 0.55], ["near-hind-leg", "shank-hn", "cannon-hn", "hoof-hn", -0.06, 0.72], ["far-fore-leg", "forearm-ff", "cannon-ff", "hoof-ff", 0.05, 0.55], ["near-fore-leg", "forearm-fn", "cannon-fn", "hoof-fn", -0.05, 0.72]] as const;
  S.keep([...legs.flatMap((l) => [l[0], `${l[0]}-cannon`, `${l[0]}-hoof`]), "ears"]);
  for (const [id, b0, b1, b2, z, tone] of legs) {
    const st = (S.c.state.elevation as any[]).find((s) => s.shape.id === id);
    const P = (st.rings[0] as { x: number; y: number }[]).map((p) => [p.x, p.y, z] as [number, number, number]);
    const r = 0.05;
    S.tube(id, b0, P[0]!, P[1]!, r, r * 0.95, COAT(tone));
    S.tube(`${id}-cannon`, b1, P[1]!, P[2]!, r * 0.95, r * 0.92, COAT(tone));
    S.tube(`${id}-hoof`, b2, P[2]!, P[3]!, r * 0.92, r * 0.9, { material: "hoof-horn", tone: 0.8 });
  }
  S.ell("ears", "ear-n", [1.211, 1.007, 0], [0.042, 0.07, 0.097], COAT(0.6), [norm([1, 0, 0]), norm([0.2, 1, 0])]);
  console.log(`roe 1:500: legs and ears rebuilt, ${S.body.length} body solids`);
  if (write) S.save();
}

if (process.argv.includes("--extra") && scale === 500) {
  S.body.length = 0;
  S.ell("t-nose", "head", [1.292, 0.793, 0], [0.04, 0.04, 0.04], { material: "bare-skin", tone: 0.08 });
  S.takeThermal({ "nose-skin": ["t-nose"] }, (t) => "tags" in t && t.tags.includes("breath-nose"));
}
