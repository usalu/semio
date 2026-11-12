import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { validateAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { STAFFAGE, frames, loadAsset, meshes, resolveAsset, QUALITY_FOR } from "./volume-lib.ts";
import { tessellateSolid } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🟦️.ts";

const asset = loadAsset(resolveAsset(process.argv[2]!).path);
const schema = JSON.parse(readFileSync(`${STAFFAGE}/🧬️schema/🔣️.json`, "utf8"));
const ajv = new Ajv({ strict: true, allErrors: true });
ajv.addSchema(schema);
const check = ajv.compile({ $ref: `${schema.$id}#/$defs/Asset` });
const ok = check(asset);
console.log("ajv", ok ? "ok" : JSON.stringify(check.errors?.slice(0, 5)));
const issues = validateAsset(asset);
const vol = issues.filter((i: any) => i.code.startsWith("volume-"));
console.log("validateAsset total", issues.length, "volume-*", vol.length);
for (const i of vol) console.log("  ", JSON.stringify(i));
for (const i of issues.filter((i: any) => !i.code.startsWith("volume-")).slice(0, 6)) console.log("   other:", i.code, JSON.stringify(i).slice(0, 160));

const rest = frames(asset);
const maxDim = Math.max(asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM);
const solids = asset.volume.solids;
const quality = QUALITY_FOR(maxDim);
const tess = solids.map((s: any) => ({ s, g: tessellateSolid({ ...s, node: s.bone } as never, quality) }));
type Box = { min: number[]; max: number[] };
function boxes(frameOf: (bone: string) => any): Box[] {
  return tess.map(({ s, g }: any) => {
    const f = frameOf(s.bone);
    const min = [1e9, 1e9, 1e9], max = [-1e9, -1e9, -1e9];
    for (let i = 0; i < g.positions.length; i += 3) {
      const loc = [g.positions[i], g.positions[i + 1], g.positions[i + 2]];
      const w = [f.o[0] + loc[0] * f.u[0] + loc[1] * f.v[0] + loc[2] * f.w[0], f.o[1] + loc[0] * f.u[1] + loc[1] * f.v[1] + loc[2] * f.w[1], f.o[2] + loc[0] * f.u[2] + loc[1] * f.v[2] + loc[2] * f.w[2]];
      for (let k = 0; k < 3; k++) { min[k] = Math.min(min[k]!, w[k]!); max[k] = Math.max(max[k]!, w[k]!); }
    }
    return { min, max };
  });
}
const tol = 0.004;
const touch = (a: Box, b: Box) => [0, 1, 2].every((k) => a.min[k]! <= b.max[k]! + tol && b.min[k]! <= a.max[k]! + tol);
function components(bs: Box[]): number[] {
  const parent = bs.map((_, i) => i);
  const find = (i: number): number => (parent[i] === i ? i : (parent[i] = find(parent[i]!)));
  for (let i = 0; i < bs.length; i++) for (let j = i + 1; j < bs.length; j++) if (touch(bs[i]!, bs[j]!)) parent[find(i)] = find(j);
  return bs.map((_, i) => find(i));
}
const restBoxes = boxes((b) => rest[b]);
const restComp = components(restBoxes);
const restEdges = new Set<string>();
for (let i = 0; i < restBoxes.length; i++) for (let j = i + 1; j < restBoxes.length; j++) if (touch(restBoxes[i]!, restBoxes[j]!)) restEdges.add(`${i},${j}`);
console.log("rest components", new Set(restComp).size, "solids", solids.length);
const loose = restComp.map((c, i) => ({ c, i })).filter(({ c }) => restComp.filter((x) => x === c).length === 1).map(({ i }) => solids[i].id);
if (loose.length) console.log("  isolated at rest:", loose.join(","));
const queries: { label: string; q: any }[] = [];
for (const p of asset.poses) queries.push({ label: `pose:${p.id}`, q: { kind: "pose", pose: p.id } });
for (const c of asset.clips ?? []) for (const f of [0.25, 0.5, 0.9]) queries.push({ label: `clip:${c.id}@${f}`, q: { kind: "clip", clip: c.id, timeS: c.durationS * f } });
for (const { label, q } of queries) {
  const state: any = evaluate(asset, q, { kind: "scale", scale: { denominator: 50 } } as any);
  const root = state.root;
  const arr = (q: any) => [q.x, q.y, q.z];
  const bs = boxes((b) => { const f = state.bones[b].frame; return { o: arr(f.o), u: arr(f.u), v: arr(f.v), w: arr(f.w) }; });
  const comp = components(bs);
  const lost: string[] = [];
  for (const e of restEdges) { const [i, j] = e.split(",").map(Number); if (!touch(bs[i!]!, bs[j!]!)) lost.push(`${solids[i!].id}|${solids[j!].id}`); }
  const mn = [1e9, 1e9, 1e9], mx = [-1e9, -1e9, -1e9];
  for (const b of bs) for (let k = 0; k < 3; k++) { mn[k] = Math.min(mn[k]!, b.min[k]!); mx[k] = Math.max(mx[k]!, b.max[k]!); }
  console.log(`${label} bbox ${mn.map((x) => x.toFixed(3))}..${mx.map((x) => x.toFixed(3))} components ${new Set(comp).size} lostContacts ${lost.length}${lost.length ? " " + lost.slice(0, 8).join(";") : ""}`);
}
