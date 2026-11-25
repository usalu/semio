/** ⚖️ Proves one asset script on a copy outside the catalog, through the modules of the repo: the copy of the catalog document gets the volumes (written, then applied again: unchanged; every other byte equal), then `authoredScales`, `volumeIssues`, `exportIssues`, `auditGeometry` of the model-catalog oracles (model, geometry, closed, budget; a finding that names only solids or scale pairs of scales the script does not write is reported apart as `catalogFindings`, because it is in the catalog before the script), the triangles per scale and mode (`falling`: no growth into or out of a scale the script writes; `fallingEverywhere`: over all scales), the extent of each new body scale in the default pose against `dimensions` by `extentMiss` of the oracles and by the limit of `volumes-bounds`, and, unless `--no-rules`, `checkVolumes` at each new scale as information. Prints one JSON line (`PROOF …`) per asset and exits 1 when a required proof fails.
 *
 * usage: prove.ts <scratch dir outside the catalog> <asset id> [--no-rules] [--keep] [--source <asset document>]   (the copy is removed after it is read unless --keep) */
import { copyFileSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { S, type Script, applyTo, assetFile, mod, spanOf } from "./lib.ts";

const args = process.argv.slice(2);
const noRules = args.includes("--no-rules");
const keep = args.includes("--keep");
const sourceAt = args.indexOf("--source");
const source = sourceAt < 0 ? undefined : args.splice(sourceAt, 2)[1];
const [scratch, id] = args.filter((arg) => !arg.startsWith("--")) as [string, string];
if (resolve(scratch).startsWith(S)) throw new Error("refused: the scratch directory lies inside the staffage library");
const script = ((await import(`./${id}.ts`)) as { default: Script }).default;
const scales = Object.keys(script.volumes).map(Number);
mkdirSync(join(scratch, id), { recursive: true });
const copy = join(scratch, id, "🔣️.json");
copyFileSync(source ?? assetFile(id), copy);
const before = readFileSync(copy, "utf8");
const first = applyTo(before, script);
writeFileSync(copy, first.text);
const second = applyTo(readFileSync(copy, "utf8"), script);
const strip = (text: string): string => {
  let out = text;
  for (const scale of [...scales].sort((a, b) => b - a)) {
    const span = spanOf(out, ["asset", "volumes", String(scale)])!;
    out = out.slice(0, span[0]) + out.slice(span[1]);
  }
  return out;
};
const untouched = strip(before) === strip(first.text);
const asset = JSON.parse(readFileSync(copy, "utf8")).asset;
if (!keep) rmSync(join(scratch, id), { recursive: true, force: true });

const { volumeIssues, exportIssues } = await mod("🔨️modules/🗂️catalog/🧊️volume/🟦️.ts");
const { authoredScales } = await mod("🔨️modules/🧊️volume/🟦️.ts");
const { auditGeometry, extentMiss } = await mod("🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts");
const { evaluate } = await mod("🔨️modules/🧮️evaluation/🟦️.ts");
const { worldSolids, extentOf } = await mod("🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts");
const { checkVolumes, VOLUME_LIMITS, featureOf } = await mod("🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts");

const issues = [...volumeIssues(asset), ...exportIssues(asset)].map((issue: any) => `${issue.code} ${issue.path}`);
const authored = authoredScales(asset.volumes);
const geometry = await auditGeometry(asset);
const elsewhere = (finding: any): boolean => {
  const at = /^s(\d+)-/u.exec(finding.item);
  if (at !== null) return !scales.includes(Number(at[1]));
  const pair = /from 1:(\d+) \(\d+\) to 1:(\d+)/u.exec(finding.message);
  return pair !== null && !scales.includes(Number(pair[1])) && !scales.includes(Number(pair[2]));
};
const text = (finding: any): string => `${finding.check} ${finding.item}: ${finding.message}`;
const findings = geometry.findings.filter((finding: any) => !elsewhere(finding)).map(text);
const catalogFindings = geometry.findings.filter(elsewhere).map(text);
const triangles: Record<string, number[]> = {};
for (const scene of [...geometry.facts.scenes].sort((a: any, b: any) => a.scale - b.scale)) (triangles[scene.mode] ??= []).push(scene.triangles);
const order = [...new Set(geometry.facts.scenes.map((scene: any) => scene.scale as number))].sort((a, b) => a - b);
const own = (at: number): boolean => scales.includes(order[at]!) || scales.includes(order[at - 1]!);
const falling = Object.values(triangles).every((row) => row.every((count, at) => at === 0 || !own(at) || count <= row[at - 1]!));
const fallingEverywhere = Object.values(triangles).every((row) => row.every((count, at) => at === 0 || count <= row[at - 1]!));
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose });
const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
const bounds = scales.map((scale) => {
  const extent = extentOf(worldSolids(asset, state.bones, scale, "body"));
  const size = [0, 1, 2].map((k) => extent.max[k] - extent.min[k]);
  const miss = extentMiss(asset, extent, scale);
  const rule = size.map((got, k) => Math.abs(got - dims[k]!) <= Math.max(VOLUME_LIMITS.boundsFeatures * featureOf(scale), VOLUME_LIMITS.boundsFraction * dims[k]!));
  return { scale, sizeMm: size.map((v) => Math.round(v * 1000)), dimsMm: dims.map((v) => Math.round(v * 1000)), minYmm: Math.round(extent.min[1] * 1000), oracleMiss: Math.round(miss.miss * 1000) / 1000, oracleOk: miss.miss <= 1, ruleOk: rule.every(Boolean) };
});
const rules: Record<string, string[]> = {};
let rulesError: string | undefined;
if (!noRules)
  for (const scale of scales) {
    try {
      for (const finding of checkVolumes(asset, { scale })) (rules[`${scale}:${finding.rule}`] ??= []).push(`${finding.path}: ${finding.message.en}`);
    } catch (error) {
      rulesError = `${scale}: ${(error as Error).message}`;
    }
  }
const ok = first.result === "written" && second.result === "unchanged" && untouched && issues.length === 0 && [50, 100, 200, 500].every((scale) => authored.includes(scale)) && findings.length === 0 && falling && bounds.every((b) => b.oracleOk && b.ruleOk);
console.log(`PROOF ${JSON.stringify({ id, ok, scales, apply: [first.result, second.result], untouched, authored, volumeIssues: issues, auditFindings: findings, catalogFindings, triangles, falling, fallingEverywhere, bounds, solids: Object.fromEntries(scales.map((scale) => [scale, ["body", "thermal", "structure"].map((list) => asset.volumes[scale][list].length)])), rules: noRules ? "skipped" : Object.fromEntries(Object.entries(rules).map(([key, list]) => [key, list.length])), rulesError, rssMb: Math.round(process.memoryUsage().rss / 1048576) })}`);
for (const [key, list] of Object.entries(rules)) for (const line of list) console.log(`RULE ${id} ${key} ${line}`);
process.exit(ok ? 0 : 1);
