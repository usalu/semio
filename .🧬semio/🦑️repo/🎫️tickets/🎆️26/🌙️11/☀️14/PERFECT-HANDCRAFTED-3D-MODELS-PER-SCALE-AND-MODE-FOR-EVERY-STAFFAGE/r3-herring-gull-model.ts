/** 🕊️ Round 3 herring gull: writes the handcrafted `body`, `thermal` and `structure` of one scale into the authoring source. Usage: bun r3-herring-gull-model.ts <50|100|200|500>
 * @see ./r3-herring-gull-body.ts
 * @see ./r3-herring-gull-coarse.ts
 * @see ./r3-herring-gull-skeleton.ts */
import { Bench, mul, type Json, type Station, type V3 } from "./r3-herring-gull-kit.ts";
import { body50, topOf } from "./r3-herring-gull-body.ts";
import { body100, body200 } from "./r3-herring-gull-coarse.ts";
import { skeleton } from "./r3-herring-gull-skeleton.ts";
import { DECISIONS } from "./r3-herring-gull-decisions.ts";
import { body500, skeleton500 } from "./r4-herring-gull-500.ts";

const scale = Number(process.argv[2] ?? 50) as 50 | 100 | 200 | 500;
const S = new Bench(scale);
if (scale === 50) body50(S);
else if (scale === 100) body100(S);
else if (scale === 200) body200(S);
else body500(S);

/** 🌡️ Region of every body solid, with the two feather layers of the plumage regions: [pattern, region, layer prefix, thickness of the contour layer in metres]. */
const REGION: readonly (readonly [RegExp, string, string?, number?])[] = [
  [/^(body|mantle|(near|far)-(thigh|drumstick))$/, "body-plumage", "body", 0.0108],
  [/^neck/, "neck-plumage", "neck", 0.0063],
  [/^(head|eye(-f)?)$/, "head-plumage", "head", 0.0036],
  [/^bill/, "bill"],
  [/(coverts|tertials|secondaries|skirt|primaries|mirror|propatagium|arm-web|hand-web)$/, "wing-plumage", "wing", 0.0072],
  [/^tail/, "tail-plumage", "tail", 0.0045],
  [/(shank|tarsus|leg)$/, "legs-skin"],
  [/(web|web-inner|web-outer|toe-\w+|foot)$/, "web-feet"],
];
const solidOnly = /^(eye|eye-f|mantle|(near|far)-(mirror|skirt|tertials|propatagium|arm-web|hand-web))$/;
for (const s of S.body) {
  const hit = REGION.find(([re]) => re.test(s.id));
  if (hit === undefined) throw new Error(`no region for ${s.id}`);
  const [, region, layer, contour] = hit;
  S.thermal.push({ id: `th-${s.id}`, of: s.id, region, ...(layer === undefined ? {} : { layer: `${layer}-contour` }) });
  if (scale !== 50 || layer === undefined || contour === undefined || solidOnly.test(s.id) || s.kind !== "loft") continue;
  const inner: Json = JSON.parse(JSON.stringify(s));
  inner.id = `th-${s.id}-down`;
  inner.tone = 0.9;
  delete inner.texture;
  inner.sections = (s.sections as Json[]).map((q) => ({ ...q, rx: Math.max(q.rx - contour, 0.3 * q.rx), ry: Math.max(q.ry - contour, 0.3 * q.ry) }));
  S.thermal.push({ ...inner, region, layer: `${layer}-down` });
}

const headFrame = S.frame("head");
if (scale !== 50) {
  const from = S.body.length;
  const r = scale === 100 ? 0.0076 : scale === 200 ? 0.0152 : 0.019;
  S.ell("th-breath-nares", "head", S.J("head", 0.087, scale === 100 ? -0.0002 : -0.002), [r, r * 0.7, r * 0.7], { material: "bill-keratin", tone: 0.2 }, [headFrame.u as V3, headFrame.v as V3]);
  S.toThermal(from, "bill", ["emitter", "breath-nares"]);
} else
  for (const sd of [-1, 1] as const) {
    const from = S.body.length;
    S.ell(sd < 0 ? "th-breath-nares" : "th-breath-nares-f", "head", S.J("head", 0.087, 0.0005, sd * 0.0043), [0.0045, 0.0031, 0.0031], { material: "bill-keratin", tone: 0.2 }, [headFrame.u as V3, headFrame.v as V3]);
    S.toThermal(from, "bill", ["emitter", "breath-nares"]);
  }
const sink = scale === 50 ? 0.0045 : scale === 100 ? 0.0076 : scale === 200 ? 0.0152 : 0.019;
const patch: readonly (readonly [number, number])[] = scale === 50
  ? [[0.0, 0.006], [0.02, 0.014], [0.045, 0.018], [0.07, 0.016], [0.09, 0.006]]
  : scale === 100
    ? [[0.0, 0.006], [0.045, 0.018], [0.09, 0.006]]
    : scale === 200
      ? [[0.0, 0.008], [0.045, 0.02], [0.09, 0.008]]
      : [[0.002, 0.009], [0.046, 0.022], [0.088, 0.009]];
const p0 = S.body.length;
S.loft("th-plume-back", "trunk", patch.map(([s, half]): Station => ({ c: S.T(s, topOf(S, s) - sink + 0.0016), a: [0, 0, half], b: mul(S.dorsal, sink), e: 2.4 })), { material: "plumage", tone: 0.8 });
S.toThermal(p0, "body-plumage", ["emitter", "plume-back"]);
if (scale === 500) skeleton500(S);
else skeleton(S, scale);
S.finish(DECISIONS[scale] as Json[]);
console.log(`1:${scale} body ${S.body.length} thermal ${S.thermal.length} structure ${S.structure.length}`);
