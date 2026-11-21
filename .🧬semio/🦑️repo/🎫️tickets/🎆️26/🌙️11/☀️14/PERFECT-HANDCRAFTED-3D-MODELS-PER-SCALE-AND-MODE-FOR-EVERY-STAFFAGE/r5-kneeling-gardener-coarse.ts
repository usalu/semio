/** 🪴️ Round 5 figure masses of the kneeling gardener at 1:200 and 1:500, stated by hand from the 1:50 stations (`SHIRT50`): the shirt with the blousing back, bust and kyphosis in nine (1:200) or six (1:500) sections, the trouser seat with the gluteal fullness, sleeves of the real upper-arm girth and the right fist as one closed form around the trowel grip. Equipment, head, hat, legs and the left hand keep their round 4 forms; the ids stay, so the thermal references and the skeleton of these scales stay bound.
 * @see ./r5-kneeling-gardener-body.ts
 * @see ./r5-kneeling-gardener-figure.ts */
import { Kit, type Json, type Row } from "./r3-kneeling-gardener-kit.ts";
import { CANVAS, GLOVE, LINEN, SIDES, T, m } from "./r5-kneeling-gardener-body.ts";

/** 👕️ A shirt row from back and front reach: `[s, back, front, half width, exponent]`. */
const shirt = (rows: readonly (readonly [number, number, number, number, number])[]): Row[] => rows.map(([s, b, f, hw, e]) => [s, Math.round(((b - f) / 2) * 1e4) / 1e4, 0, hw, Math.round(((b + f) / 2) * 1e4) / 1e4, e] as Row);

const FIGURE: Record<200 | 500, (k: Kit) => void> = {
  200: (k) => {
    k.loft("seat", "pelvis", CANVAS, T.trousers, [[-0.1, 0.014, 0, 0.042, 0.036, 2.2], [-0.075, 0.024, 0, 0.12, 0.098, 2.3], [-0.04, 0.03, 0, 0.165, 0.122, 2.3], [0.01, 0.026, 0, 0.174, 0.122, 2.3], [0.07, 0.012, 0, 0.162, 0.108, 2.3], [0.128, 0, 0, 0.147, 0.099, 2.3]]);
    k.loft("shirt", "torso", LINEN, T.shirt, shirt([[-0.075, 0.112, 0.132, 0.17, 2.5], [-0.035, 0.121, 0.138, 0.17, 2.5], [0.06, 0.111, 0.141, 0.152, 2.4], [0.17, 0.118, 0.139, 0.154, 2.4], [0.265, 0.133, 0.151, 0.163, 2.3], [0.335, 0.137, 0.12, 0.167, 2.3], [0.4, 0.119, 0.084, 0.17, 2.2], [0.44, 0.095, 0.06, 0.124, 2.1], [0.47, 0.07, 0.05, 0.06, 2]]));
    for (const [sfx, g] of SIDES) k.loft(`sleeve-${sfx}`, `upper-arm-${sfx}`, LINEN, T.shirt, m([[-0.054, 0, -0.011, 0.018, 0.02], [-0.03, 0, -0.006, 0.04, 0.043], [0.01, 0.001, 0, 0.051, 0.054], [0.13, 0.003, 0.003, 0.047, 0.051], [0.228, 0.002, 0.002, 0.05, 0.054]], g));
    k.loft("glove-r", "hand-r", GLOVE, T.glove, [[-0.014, 0, -0.01, 0.02, 0.027, 2.2], [0.025, -0.003, -0.008, 0.03, 0.039, 2.3], [0.065, -0.004, -0.002, 0.037, 0.043, 2.3], [0.095, -0.004, 0.003, 0.032, 0.036, 2.2], [0.112, -0.003, 0.004, 0.017, 0.02, 2]]);
  },
  500: (k) => {
    k.loft("seat", "pelvis", CANVAS, T.trousers, [[-0.098, 0.015, 0, 0.056, 0.047, 2.2], [-0.05, 0.028, 0, 0.16, 0.12, 2.3], [0.01, 0.026, 0, 0.174, 0.122, 2.3], [0.124, 0, 0, 0.149, 0.1, 2.3]]);
    k.loft("shirt", "torso", LINEN, T.shirt, shirt([[-0.075, 0.112, 0.132, 0.17, 2.5], [0.06, 0.112, 0.141, 0.153, 2.4], [0.265, 0.133, 0.151, 0.163, 2.3], [0.37, 0.131, 0.101, 0.169, 2.2], [0.44, 0.095, 0.06, 0.124, 2.1], [0.472, 0.07, 0.05, 0.057, 2]]));
    for (const [sfx, g] of SIDES) k.loft(`upper-arm-${sfx}`, `upper-arm-${sfx}`, LINEN, T.shirt, m([[-0.054, 0, -0.011, 0.021, 0.023], [0.01, 0.001, 0, 0.052, 0.055], [0.228, 0.002, 0.002, 0.05, 0.054], [0.295, -0.003, 0, 0.042, 0.04], [0.326, -0.002, 0, 0.03, 0.03]], g));
    k.loft("glove-r", "hand-r", GLOVE, T.glove, [[-0.014, 0, -0.008, 0.024, 0.03, 2.2], [0.05, -0.004, -0.004, 0.038, 0.043, 2.3], [0.112, -0.003, 0.004, 0.02, 0.022, 2]]);
  },
};

/** 🔁️ The body list of 1:`scale` with the round 5 figure masses put in place of the solids of the same id. */
export function coarseBody(volume: Json, scale: 200 | 500): Json[] {
  const k = new Kit();
  FIGURE[scale](k);
  const by = new Map<string, Json>(k.solids.map((s) => [s.id, s]));
  const body: Json[] = (volume.body as Json[]).map((s) => by.get(s.id) ?? s);
  for (const s of k.solids) if (!body.some((b) => b.id === s.id)) throw new Error(`1:${scale} has no solid "${s.id}" to replace`);
  return body;
}
