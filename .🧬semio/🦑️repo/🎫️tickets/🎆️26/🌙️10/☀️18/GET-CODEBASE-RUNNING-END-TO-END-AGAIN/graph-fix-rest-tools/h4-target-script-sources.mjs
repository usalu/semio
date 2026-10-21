import { readFileSync, existsSync } from "node:fs";
const ws = "/Users/ueli/Documents/semio";
const [, , root, mode] = process.argv;
const B = ws + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const inf = await import(B + "/🕸️graph/📥️inference/🟨️.mjs");
const { createScriptInputCache } = await import(B + "/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
const ci = inf.cacheInternals;
await ci.playgroundSessionTargets([], ws, []);
const json = JSON.parse(readFileSync(`${ws}/${root}/📋️project.json`, "utf8"));
const scripts = createScriptInputCache({});
const ownsScript = existsSync(`${ws}/${root}/📜️script.ts`);
let bad = 0, n = 0;
for (const [name, t0] of Object.entries(json.targets)) {
  for (const variant of ["raw", "defaulted"]) {
    const t = variant === "raw" ? t0 : ci.targetWithDefaults(t0, root, ownsScript);
    n++;
    try { await ci.targetScriptSources(t, ws, scripts, root); } catch (e) { bad++; console.log(variant, name, "|", e.message.slice(0, 160)); }
  }
}
console.log("checked", n, "bad", bad);
