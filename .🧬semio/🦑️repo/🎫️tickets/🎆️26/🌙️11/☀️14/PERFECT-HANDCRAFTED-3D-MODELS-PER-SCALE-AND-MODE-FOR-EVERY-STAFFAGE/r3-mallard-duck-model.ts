/** 🦆️ Round 3 mallard: writes the handcrafted `body`, `thermal` and `structure` of one scale into the authoring source. Usage: bun r3-mallard-duck-model.ts <50|100|200|500>
 * @see ./r3-mallard-duck-body.ts
 * @see ./r3-mallard-duck-skeleton.ts */
import { Bench, mul, type Json, type Station } from "./r3-mallard-duck-kit.ts";
import { body50 } from "./r3-mallard-duck-body.ts";
import { body100, body200, body500 } from "./r3-mallard-duck-coarse.ts";
import { skeleton } from "./r3-mallard-duck-skeleton.ts";
import { DECISIONS } from "./r3-mallard-duck-decisions.ts";

const scale = Number(process.argv[2] ?? 50) as 50 | 100 | 200 | 500;
const S = new Bench(scale);
if (scale === 50) body50(S);
else if (scale === 100) body100(S);
else if (scale === 200) body200(S);
else body500(S);

/** 🌡️ Region of every body solid, with the two feather layers of the plumage regions: [pattern, region, layer prefix, thickness of the contour layer in metres]. */
const REGION: readonly (readonly [RegExp, string, string?, number?])[] = [
  [/^(body|rump|breast|(near|far)-(thigh|drumstick))$/, "body-plumage", "body", 0.0108],
  [/^neck/, "neck-plumage", "neck", 0.0063],
  [/^(head|eye(-f)?)$/, "head-plumage", "head", 0.0036],
  [/^bill$/, "bill"],
  [/(wing|scapulars|speculum|primaries)$/, "wing-plumage", "wing", 0.0072],
  [/^tail/, "tail-plumage", "tail", 0.0045],
  [/(shank|tarsus|leg)$/, "legs-skin"],
  [/(web|web-inner|web-outer|toe-\w+|hallux|foot)$/, "web-feet"],
];
const solidOnly = /^(eye|eye-f|tail-curl|(near|far)-speculum)$/;
for (const s of S.body) {
  const hit = REGION.find(([re]) => re.test(s.id));
  if (hit === undefined) throw new Error(`no region for ${s.id}`);
  const [, region, layer, contour] = hit;
  S.thermal.push({ id: `th-${s.id}`, of: s.id, region, ...(layer === undefined ? {} : { layer: `${layer}-contour` }) });
  if (scale !== 50 || layer === undefined || contour === undefined || solidOnly.test(s.id) || s.kind !== "loft") continue;
  const inner: Json = JSON.parse(JSON.stringify(s));
  inner.id = `th-${s.id}-down`;
  inner.tone = 0.9;
  inner.sections = (s.sections as Json[]).map((q) => ({ ...q, rx: Math.max(q.rx - contour, 0.3 * q.rx), ry: Math.max(q.ry - contour, 0.3 * q.ry) }));
  S.thermal.push({ ...inner, region, layer: `${layer}-down` });
}

const headFrame = S.frame("head");
if (scale !== 50) {
  const from = S.body.length;
  const r = scale === 100 ? 0.0076 : scale === 200 ? 0.0083 : 0.0088;
  S.ell("th-breath-nares", "head", S.J("head", 0.064, scale === 100 ? 0.0044 : scale === 200 ? 0.0036 : 0.003), [r, r, r], { material: "bill-keratin", tone: 0.2 });
  S.toThermal(from, "bill", ["emitter", "breath-nares"]);
} else
  for (const sd of [-1, 1] as const) {
    const from = S.body.length;
    S.ell(sd < 0 ? "th-breath-nares" : "th-breath-nares-f", "head", S.J("head", 0.064, 0.0074, sd * 0.0045), [0.004, 0.0033, 0.0033], { material: "bill-keratin", tone: 0.2 }, [headFrame.u as [number, number, number], headFrame.v as [number, number, number]]);
    S.toThermal(from, "bill", ["emitter", "breath-nares"]);
  }
const sink = scale === 50 ? 0.0045 : scale === 100 ? 0.0076 : 0.0152;
const patch: readonly (readonly [number, number, number])[] = scale === 50
  ? [[0.012, 0.0322, 0.005], [0.025, 0.0336, 0.014], [0.045, 0.0353, 0.019], [0.07, 0.0372, 0.02], [0.09, 0.0385, 0.017], [0.104, 0.0394, 0.007]]
  : scale === 100
    ? [[0.012, 0.0322, 0.005], [0.035, 0.0345, 0.018], [0.075, 0.0375, 0.02], [0.104, 0.0394, 0.007]]
    : scale === 200
      ? [[0.012, 0.0322, 0.006], [0.06, 0.0365, 0.02], [0.104, 0.0394, 0.007]]
      : [[0.016, 0.0326, 0.007], [0.064, 0.0368, 0.021], [0.1, 0.0392, 0.008]];
const p0 = S.body.length;
S.loft("th-plume-back", "trunk", patch.map(([s, top, half]): Station => ({ c: S.T(s, top - sink + 0.001), a: [0, 0, half], b: mul(S.dorsal, sink), e: 2.4 })), { material: "plumage", tone: 0.8 });
S.toThermal(p0, "body-plumage", ["emitter", "plume-back"]);
skeleton(S, scale);
S.finish(DECISIONS[scale] as Json[]);
console.log(`1:${scale} body ${S.body.length} thermal ${S.thermal.length} structure ${S.structure.length}`);
