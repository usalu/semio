/** 🕊️ Round 3 pigeon: writes the handcrafted `body`, `thermal` and `structure` of one scale into the authoring source. Usage: bun r3-pigeon-model.ts <50|100|200|500> */
import { Bench, mul, type Json, type Station, type V3 } from "./r3-pigeon-kit.ts";
import { body50, H } from "./r3-pigeon-body50.ts";
import { body100, body200, body500 } from "./r3-pigeon-coarse.ts";
import { skeleton } from "./r3-pigeon-skeleton.ts";
import { DECISIONS } from "./r3-pigeon-decisions.ts";

const scale = Number(process.argv[2] ?? 50) as 50 | 100 | 200 | 500;
const S = new Bench(scale);
if (scale === 50) body50(S);
else if (scale === 100) body100(S);
else if (scale === 200) body200(S);
else body500(S);

const REGION: readonly (readonly [RegExp, string])[] = [
  [/^(body|undertail|(near|far)-(thigh|drumstick))$/, "plumage-body"],
  [/^(neck(-\d)?|head)$/, "plumage-neck-head"],
  [/(coverts|primaries|shoulder|wing)$/, "plumage-wings"],
  [/tail$/, "plumage-tail"],
  [/(shank|tarsus|toes?|toe-inner|toe-outer|toe-spread|hallux|leg|foot)$/, "legs-feet"],
  [/^(beak|jaw)$/, "beak"],
  [/^cere$/, "cere"],
  [/^eye(-f)?$/, "eye"],
];
for (const s of S.body) {
  const region = REGION.find(([re]) => re.test(s.id))?.[1];
  if (region === undefined) throw new Error(`no region for ${s.id}`);
  S.thermal.push({ id: `th-${s.id}`, of: s.id, region, ...(region.startsWith("plumage") ? { layer: "plumage" } : {}) });
}
if (scale >= 200) for (const region of ["beak", "cere", "eye"]) S.thermal.push({ id: `th-${region}-head`, of: "head", region });
if (scale >= 200) {
  const from = S.body.length;
  S.ell("th-breath-nostril", "head", H(S, scale === 200 ? 0.024 : 0.022, 0.003), scale === 200 ? [0.0146, 0.0146, 0.0146] : [0.014, 0.014, 0.014], { material: "cere-wax", tone: 0.2 });
  S.toThermal(from, "plumage-neck-head", ["emitter", "breath-nostril"]);
} else
  for (const sd of [-1, 1] as const) {
    const from = S.body.length;
    S.ell(sd < 0 ? "th-breath-nostril" : "th-breath-nostril-f", "head", H(S, 0.037, 0.0036, sd * 0.0021), [0.0022, 0.0022, 0.0022], { material: "cere-wax", tone: 0.2 });
    S.toThermal(from, "cere", ["emitter", "breath-nostril"]);
  }
const sink = scale === 50 ? 0.0045 : scale === 100 ? 0.0076 : 0.0151;
const patch: readonly (readonly [number, number, number])[] = scale === 50
  ? [[0.03, 0.0095, 0.006], [0.04, 0.0098, 0.0125], [0.06, 0.0106, 0.0165], [0.085, 0.0113, 0.018], [0.105, 0.0114, 0.0165], [0.122, 0.0109, 0.0115], [0.13, 0.0104, 0.005]]
  : scale === 100
    ? [[0.03, 0.0095, 0.006], [0.05, 0.0102, 0.0155], [0.085, 0.0113, 0.018], [0.115, 0.0111, 0.014], [0.13, 0.0104, 0.005]]
    : scale === 200
      ? [[0.03, 0.0095, 0.006], [0.08, 0.0112, 0.018], [0.13, 0.0104, 0.005]]
      : [[0.034, 0.0096, 0.007], [0.085, 0.0113, 0.019], [0.126, 0.0105, 0.006]];
const p0 = S.body.length;
S.loft("th-plume-back", "trunk", patch.map(([s, top, half]): Station => ({ c: S.T(s, top - sink + 0.001), a: [0, 0, half], b: mul(S.dorsal, sink) })), { material: "plumage", tone: 0.8 });
S.toThermal(p0, "plumage-body", ["emitter", "plume-back"]);
skeleton(S, scale);
S.finish(DECISIONS[scale] as Json[]);
console.log(`1:${scale} body ${S.body.length} thermal ${S.thermal.length} structure ${S.structure.length}`);
