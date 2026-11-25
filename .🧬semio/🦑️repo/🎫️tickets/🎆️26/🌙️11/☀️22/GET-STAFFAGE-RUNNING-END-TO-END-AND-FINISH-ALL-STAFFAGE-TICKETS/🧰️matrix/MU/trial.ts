/** 🧫️ One trial of the chain of an asset in one process, without touching the catalog: `bun trial.ts <id> [--base <file.json>] [--scales 50,100] [--poses] [--no-gate] [--no-measure] [--pose <id>] [--spec <file.json>]` applies the chain (`fix.ts` with `ops/<id>.json`) to the catalog document (or `--base`) in memory, writes the result to `$GEN/try/<id>.json` (`--spec` tries another spec file than `ops/<id>.json` and adds its name to the result file), and prints: whether the chain changed the document; the hard findings of the authors' gate of the suite (`T/🔎️scan.ts`, every pose, with the paper and derived-view checks); the gate with its reasons over poses, clip instants and transitions (`why.ts`); the outlines that moved in the default pose (and in `--pose`); whether every authored scale and range end prints; and the lines of the silhouette guard, `validateAsset` and the three-views sweep that differ between the base and the trial.
 * @see ./fix.ts — the chain
 * @see ./why.ts — the gate */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { chain } from "./fix.ts";
import { S, catalogFile } from "./files.ts";
import { gate } from "./why.ts";

const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const { DEFAULT_PRINT_PROFILE, resolveRepresentation } = await import(join(S, "🔨️modules", "🖨️representation", "🟦️.ts"));
const { measureAsset, VIEWPOINTS } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { featureOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"));
const { validateAsset } = await import(join(S, "📦️packages/🟦️typescript/🟦️.ts"));
const { authoredScales, consistency, sweep } = await import(join(S, "🧪️tests/🧊️three-views/🧰️.ts"));

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.includes(`--${name}`) ? args[args.indexOf(`--${name}`) + 1] : undefined);
const slug = args[0]!;
const text = readFileSync(flag("base") ?? catalogFile(slug), "utf8");
const base = JSON.parse(text).asset;
const doc = JSON.parse(text);
const log = chain(doc.asset, slug, flag("spec"));
const second = text.split("\n", 2)[1] ?? "";
const next = JSON.stringify(doc, null, second.length - second.trimStart().length) + (text.endsWith("\n") ? "\n" : "");
const out = join(process.env.GEN ?? "/tmp", "try");
mkdirSync(out, { recursive: true });
writeFileSync(join(out, `${slug}${flag("spec") === undefined ? "" : `.${flag("spec")!.split("/").at(-1)!.replace(/\.json$/u, "")}`}.json`), next);
for (const line of log) console.log(`spec ${line}`);
console.log(`${slug}: the chain ${next === text ? "changes nothing (the document holds every repair)" : "changes the document"}`);
const scales = flag("scales")?.split(",");

const { scanAsset, groupsOf } = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🔎️scan.ts"));
const row = scanAsset(doc.asset);
for (const g of groupsOf(row.findings).filter((group: any) => group.tier === "hard")) console.log(`scan ${g.code} ${g.views.join("/")} bone=${g.bone ?? ""} anchor=${g.anchor ?? ""} n=${g.count} scales=${g.scales.join(",")} poses=${g.poses.join(",")} worst ${g.worst.deltaMm ?? ""}@${g.worst.scale}/${g.worst.pose}: ${String(g.worst.message).slice(0, 170)}`);
console.log(`scan (authors' gate, poses): ${row.summary.hard} hard findings, ${row.summary.reported} reported`);
if (!args.includes("--no-gate")) console.log(`hard findings: ${gate(doc.asset, { scales, poses: args.includes("--poses"), quiet: true })}`);

const boxes = (asset: any, key: string, pose: string): Map<string, number[]> => {
  const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: Number(key) } });
  const found = new Map<string, number[]>();
  for (const view of ["elevation", "front", "plan"] as const)
    for (const item of state.shapes[view]) {
      const points = item.rings.flat();
      if (points.length > 0) found.set(`${view} ${item.shape.bone} ${item.shape.id}`, [Math.min(...points.map((p: any) => p.x)), Math.max(...points.map((p: any) => p.x)), Math.min(...points.map((p: any) => p.y)), Math.max(...points.map((p: any) => p.y))]);
    }
  return found;
};
for (const pose of [...new Set([base.presentation.defaultPose, ...(flag("pose") === undefined ? [] : [flag("pose")!])])]) {
  let count = 0;
  for (const key of Object.keys(doc.asset.representations)) {
    if (doc.asset.representations[key].geometry === undefined) continue;
    const [x, y] = [boxes(base, key, pose), boxes(doc.asset, key, pose)];
    for (const name of new Set([...x.keys(), ...y.keys()])) {
      const [p, q] = [x.get(name), y.get(name)];
      const moved = p === undefined || q === undefined ? Number.POSITIVE_INFINITY : Math.max(...p.map((v, i) => Math.abs(v - q[i]!)));
      if (moved <= 0.0002) continue;
      count++;
      if (count <= 60) console.log(`moved 1:${key} ${pose} ${name}: ${p === undefined ? "added" : q === undefined ? "removed" : `${(moved * 1000).toFixed(1)} mm`} ${p?.map((v) => v.toFixed(4)).join(" ") ?? ""} -> ${q?.map((v) => v.toFixed(4)).join(" ") ?? ""}`);
    }
  }
  console.log(`${slug} pose ${pose}: ${count} outlines differ`);
}

const printOf = (asset: any): string[] => {
  const lines: string[] = [];
  for (const [key, representation] of Object.entries(asset.representations) as [string, any][]) {
    if (representation.geometry === undefined) continue;
    const range = representation.validRange;
    for (const denominator of [...new Set([Number(key), range.minDenominator, range.maxDenominator])]) {
      const resolution = resolveRepresentation(asset, { denominator }, DEFAULT_PRINT_PROFILE);
      if (resolution.kind !== "authored") lines.push(`print 1:${denominator} (band ${key}) ${resolution.kind}: ${JSON.stringify(resolution.suitability?.violations ?? resolution).slice(0, 500)}`);
    }
  }
  return lines;
};
const measureOf = (asset: any): string[] => {
  const lines: string[] = [];
  const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  for (const scale of [50, 100, 200, 500].filter((d) => asset.volumes?.[String(d)]?.status === "authored" && asset.representations[String(d)]?.status === "authored")) {
    const feature = featureOf(scale);
    const m = measureAsset(asset, scale, undefined, undefined, feature);
    lines.push(`silhouette 1:${scale} bbox ${m.bbox.map((b: number, k: number) => `${b.toFixed(4)}/${dims[k]}${Math.abs(b - dims[k]!) > feature ? "!" : ""}`).join(" ")}  ${VIEWPOINTS.map((v: string) => `${v} ${m.views[v].iou.toFixed(4)}${m.views[v].iou < 0.92 ? "!" : ""}/${(m.views[v].hausdorffM * 1000).toFixed(1)}mm${m.views[v].hausdorffM > 2 * feature ? "!" : ""}`).join("  ")}`);
  }
  const issues = validateAsset(asset) as { code: string; path?: string; message: { en: string } }[];
  lines.push(`validate ${issues.length} issues`, ...issues.slice(0, 10).map((issue) => `validate ${issue.code} ${issue.path ?? ""}: ${issue.message.en}`.slice(0, 300)));
  const findings = authoredScales(asset).flatMap((scale: number) => [...sweep(asset, scale), ...consistency(asset, scale)]);
  lines.push(`three-views sweep ${findings.length} findings`, ...findings.slice(0, 6).map((f: any) => `sweep [${f.check}] ${f.message}`.slice(0, 260)));
  return lines;
};
const compare = (what: string, before: string[], after: string[]): void => {
  const same = JSON.stringify(before) === JSON.stringify(after);
  console.log(`${what}: ${same ? "identical" : "DIFFERS"} (${after.length} lines)`);
  for (const line of same ? after.filter((l) => /silhouette|issues|findings|print/u.test(l)) : [...before.filter((l) => !after.includes(l)).map((l) => `  before: ${l}`), ...after.filter((l) => !before.includes(l)).map((l) => `  after:  ${l}`)]) console.log(same ? `  ${line}` : line);
};
compare("print", printOf(base), printOf(doc.asset));
if (!args.includes("--no-measure")) compare("measure", measureOf(base), measureOf(doc.asset));
