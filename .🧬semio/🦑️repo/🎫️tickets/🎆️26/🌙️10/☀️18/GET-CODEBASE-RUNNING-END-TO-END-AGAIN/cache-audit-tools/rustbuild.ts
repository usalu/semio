/** 🦀 Counts explicit Rust `build` targets that replace the cargo-generated declared outputs with an empty list. */
import { readFileSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
const WS = "/Users/ueli/Documents/semio";
const C = process.argv[2];
const list = readFileSync(`${C}/cache-audit-tools/pj-list.txt`, "utf8").split("\n").filter(Boolean);
let explicitBuild = 0, emptyOut = 0, cacheTrue = 0, hostOut = 0, intent = 0; const ex: string[] = [], hostEx: string[] = [];
for (const f of list) {
  const root = dirname(f); if (!existsSync(join(WS, root, "Cargo.toml"))) continue;
  const j = JSON.parse(readFileSync(join(WS, f), "utf8"));
  const b = j.targets?.build; if (!b) continue;
  explicitBuild++;
  if (Array.isArray(b.outputs) && b.outputs.length === 0) { emptyOut++; if (b.cache !== false) cacheTrue++; if (ex.length < 6) ex.push(f); }
  if (b.metadata?.semio?.hostPlatformOutputsV1) { hostOut++; if (hostEx.length < 3) hostEx.push(f); }
  if (b.metadata?.semio?.rustBuildIntent) intent++;
}
console.log(JSON.stringify({ rustProjectsWithExplicitBuild: explicitBuild, emptyOutputs: emptyOut, cacheTrueWithEmptyOutputs: cacheTrue, withHostPlatformOutputs: hostOut, withRustBuildIntent: intent, sampleEmpty: ex, sampleHost: hostEx }, null, 1));
