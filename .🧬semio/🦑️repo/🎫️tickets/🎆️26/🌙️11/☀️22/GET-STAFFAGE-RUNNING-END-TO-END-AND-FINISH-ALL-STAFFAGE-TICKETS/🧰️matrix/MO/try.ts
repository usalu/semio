/** 🧫️ Tries the repair ops of an asset in memory, without writing the catalog: the derived-view flags of every authored band, the hard scan findings of every pose (extent sync, per-bone sync, anchors), whether every authored scale and both ends of its range print under the default profile, the silhouette measure per band and view (`!` marks a failing one) and the `validateAsset` count.
 *  bun try.ts <id> [--no-ops] [--fast] */
import { join } from "node:path";
import { applyAll } from "./apply.ts";
import { S, catalogFile, opsFile } from "./files.ts";
const tests = `${S}/🧪️tests`;
const engine = await import(join(tests, "🔭️multiview-coverage", "🧰️.ts"));
const { derivedFlags } = await import(join(tests, "🔭️multiview-coverage", "🪞️derived.ts"));
const { anchorLandmarks, boneSync } = await import(join(tests, "🔭️multiview-coverage", "⚓️landmarks.ts"));
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { measureAsset, VIEWPOINTS } = await import(`${tests}/🧰️support/🌓️silhouette/🟦️.ts`);
const { featureOf } = await import(`${S}/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts`);
const { validateAsset } = await import(`${S}/📦️packages/🟦️typescript/🟦️.ts`);
const { DEFAULT_PRINT_PROFILE, resolveRepresentation } = await import(`${S}/🔨️modules/🖨️representation/🟦️.ts`);
const args = process.argv.slice(2);
const slug = args[0]!;
const asset = (await Bun.file(catalogFile(slug)).json()).asset;
if (!args.includes("--no-ops") && (await Bun.file(opsFile(slug)).exists())) applyAll(asset, JSON.parse(await Bun.file(opsFile(slug)).text()));
const authored = Object.keys(asset.representations).filter((key) => engine.authoredOf(asset, key) !== undefined);
const shapesAt: Record<string, any> = {};
for (const key of authored) shapesAt[key] = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, engine.scaleSelector(Number(key))).shapes;
const tolerance = engine.toleranceM(asset);
let hard = 0;
for (const key of authored) {
  const range = asset.representations[key].validRange;
  for (const denominator of [...new Set([Number(key), range.minDenominator, range.maxDenominator])]) {
    const resolution = resolveRepresentation(asset, { denominator }, DEFAULT_PRINT_PROFILE);
    if (resolution.kind === "authored") continue;
    if (denominator === Number(key)) hard++;
    console.log(`print 1:${denominator} (band ${key}) ${resolution.kind}: ${JSON.stringify(resolution.suitability?.violations ?? resolution).slice(0, 400)}`);
  }
}
for (const key of authored) {
  const others = Object.fromEntries(authored.filter((other) => other !== key).map((other) => [`1:${other}`, shapesAt[other]]));
  for (const flag of derivedFlags(shapesAt[key], others)) { hard++; console.log(`derived 1:${key} ${flag.view} = ${flag.against} (${flag.transform} ${flag.distance})`); }
  const seen = new Map<string, number>();
  const note = (code: string, text: string): void => { if (!seen.has(code)) console.log(`1:${key} ${code}: ${text}`.slice(0, 300)); seen.set(code, (seen.get(code) ?? 0) + 1); };
  for (const pose of asset.poses) {
    const state = evaluate(asset, { kind: "pose", pose: pose.id }, engine.scaleSelector(Number(key)));
    const e = engine.extentOf(state.shapes.elevation), f = engine.extentOf(state.shapes.front), p = engine.extentOf(state.shapes.plan);
    const agree = (code: string, a: [number, number], b: [number, number]): void => { if (!(Math.max(Math.abs(a[0] - b[0]), Math.abs(a[1] - b[1])) <= tolerance)) note(code, `${pose.id}: [${a.map((v) => v.toFixed(3))}] vs [${b.map((v) => v.toFixed(3))}] tol ${tolerance.toFixed(3)}`); };
    if (e && p) agree("sync-forward", [e.minX, e.maxX], [p.minX, p.maxX]);
    if (e && f) agree("sync-height", [e.minY, e.maxY], [f.minY, f.maxY]);
    if (f && p) agree("sync-lateral", [f.minX, f.maxX], [p.minY, p.maxY]);
    for (const finding of boneSync(state, tolerance)) note(finding.check, `${pose.id}: ${finding.message}`);
    for (const finding of anchorLandmarks(state, tolerance)) note("anchor-landmark", `${pose.id}: ${finding.message}`);
  }
  for (const [code, n] of seen) { if (!code.startsWith("group-drift")) hard += n; }
  if (seen.size > 0) console.log(`1:${key} counts ${[...seen].map(([c, n]) => `${c}×${n}`).join(", ")}`);
}
if (!args.includes("--fast")) {
  const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  for (const key of authored) {
    const scale = Number(key);
    if (asset.volumes[key]?.status !== "authored") continue;
    const feature = featureOf(scale);
    const m = measureAsset(asset, scale, undefined, undefined, feature);
    const bb = m.bbox.map((b: number, k: number) => `${b.toFixed(4)}/${dims[k]}${Math.abs(b - dims[k]!) > feature ? "!" : ""}`).join(" ");
    console.log(`silhouette 1:${key} f=${feature * 1000}mm bbox ${bb}  ${VIEWPOINTS.map((v: string) => { const x = m.views[v]; return `${v} ${x.iou.toFixed(3)}${x.iou < 0.92 ? "!" : ""}/${(x.hausdorffM * 1000).toFixed(1)}mm${x.hausdorffM > 2 * feature ? "!" : ""}`; }).join("  ")}`);
  }
  const issues = validateAsset(asset) as { code: string; path?: string; message: { en: string } }[];
  console.log(`validateAsset ${issues.length} issues`);
  for (const issue of issues.slice(0, 12)) console.log(`   ${issue.code} ${issue.path ?? ""}: ${issue.message.en}`.slice(0, 330));
}
console.log(`${slug}: ${hard} hard findings`);
