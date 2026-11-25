/** 🧲️ The hard geometry findings of the multiview gate for the poses of one asset per authored scale (the catalog file as it is now, or a file given by path), grouped by check; with `--check <code>` the messages of that check.
 * usage: ROOT=<mirror> bun findings-probe.ts used-concert-harp [--check anchor-landmark] [--file <asset json>]
 */
import { statSync } from "node:fs";
import { assetFile, mod, readAsset } from "./lib.ts";

const engine = await mod("🧪️tests/🔭️multiview-coverage/🧰️.ts");
const { evaluate } = await mod("🔨️modules/🧮️evaluation/🟦️.ts");
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.includes(`--${name}`) ? args[args.indexOf(`--${name}`) + 1] : undefined);
const source = flag("file") ?? assetFile(args[0]!);
const asset = readAsset(source);
console.log(`${asset.id}: ${source.split("/").slice(-3).join("/")} modified ${statSync(source).mtime.toISOString()}`);
for (const key of Object.keys(asset.representations)) {
  if (asset.representations[key].status !== "authored") continue;
  const counts = new Map<string, number>();
  const lines: string[] = [];
  for (const pose of asset.poses) {
    const state = evaluate(asset, { kind: "pose", pose: pose.id }, engine.scaleSelector(Number(key)));
    for (const finding of engine.geometryFindings(asset, state, pose.id === asset.presentation.defaultPose)) {
      if (finding.tier !== "hard") continue;
      counts.set(finding.check, (counts.get(finding.check) ?? 0) + 1);
      if (finding.check === flag("check")) lines.push(`  ${pose.id}: ${finding.message.slice(0, 200)}`);
    }
  }
  console.log(`1:${key}: ${[...counts].map(([check, n]) => `${check} ${n}`).join(", ") || "no hard geometry finding"} (tolerance ${(engine.toleranceM(asset) * 1000).toFixed(0)} mm)`);
  for (const line of lines.slice(0, 8)) console.log(line);
}
