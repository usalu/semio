/** 🧩 Checks that each explicit target's named inputs resolve in its project or the root nx.json, and reports env-bearing named inputs. */
import { readFileSync } from "node:fs";
import { dirname } from "node:path";
const WS = "/Users/ueli/Documents/semio";
const C = process.argv[2];
const list = readFileSync(`${C}/cache-audit-tools/pj-list.txt`, "utf8").split("\n").filter(Boolean);
const nx = JSON.parse(readFileSync(`${WS}/nx.json`, "utf8")).namedInputs ?? {};
const globalNames = new Set(Object.keys(nx));
const envBearing: Record<string, string[]> = {};
const undefinedRefs: Record<string, number> = {};
const missing: string[] = [];
const samples: string[] = [];
for (const f of list) {
  const j = JSON.parse(readFileSync(`${WS}/${f}`, "utf8"));
  const named = j.namedInputs ?? {};
  for (const [k, v] of Object.entries<any>(named)) {
    const s = JSON.stringify(v);
    if (/"env"|externalDependencies|"runtime"/.test(s)) envBearing[k] = [...(envBearing[k] ?? []), f].slice(0, 3);
  }
  for (const [n, t] of Object.entries<any>(j.targets ?? {})) {
    for (const i of t.inputs ?? []) {
      if (typeof i !== "string") continue;
      const ref = i.replace(/^\^/, "").replace(/^!/, "");
      if (/[{}\/*]/.test(ref) || ["default", "production"].includes(ref) && false) continue;
      if (["default", "production", "sharedGlobals", "schemaSources", "cargo", "go", "uv", "dotnet", "nativeSources", "nativeTestSources"].includes(ref)) continue;
      if (!named[ref] && !globalNames.has(ref)) { undefinedRefs[ref] = (undefinedRefs[ref] ?? 0) + 1; if (samples.length < 15) samples.push(`${f} :: ${n} :: ${i}`); }
    }
  }
}
console.log(JSON.stringify({ envBearingNamedInputs: envBearing, undefinedRefCounts: undefinedRefs, samples }, null, 1));
