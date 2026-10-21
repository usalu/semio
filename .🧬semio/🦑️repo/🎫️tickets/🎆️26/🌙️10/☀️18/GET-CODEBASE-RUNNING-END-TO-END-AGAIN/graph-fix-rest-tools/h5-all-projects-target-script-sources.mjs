import { readFileSync, existsSync } from "node:fs";
import { execFileSync } from "node:child_process";
const ws = "/Users/ueli/Documents/semio";
const B = ws + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const inf = await import(B + "/🕸️graph/📥️inference/🟨️.mjs");
const { createScriptInputCache } = await import(B + "/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
const ci = inf.cacheInternals;
await ci.playgroundSessionTargets([], ws, []);
const files = execFileSync("git", ["ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", "*📋️project.json"], { cwd: ws, maxBuffer: 1 << 28 }).toString().split("\0").filter(f => f && !f.includes(".🧬semio") && !f.includes("node_modules"));
console.log("projects", files.length);
const scripts = createScriptInputCache({});
const stat = { raw: 0, def: 0, n: 0 }, kinds = new Map();
for (const f of files) {
  const root = f.replace(/\/?📋️project\.json$/, "") || ".";
  let json; try { json = JSON.parse(readFileSync(`${ws}/${f}`, "utf8")); } catch { continue; }
  const own = existsSync(`${ws}/${root}/📜️script.ts`);
  for (const [name, t0] of Object.entries(json.targets ?? {})) {
    stat.n++;
    for (const v of ["raw", "def"]) {
      const t = v === "raw" ? t0 : ci.targetWithDefaults(t0, root, own);
      try { await ci.targetScriptSources(t, ws, scripts, root); } catch (e) {
        stat[v]++;
        const m = e.message.replace(/\/Users\/ueli\/Documents\/semio/g, "").slice(0, 110);
        if (v === "def" || /escapes|ENOENT/.test(m)) console.log(v, json.name, name, "|", m);
      }
    }
  }
}
console.log("DONE", JSON.stringify(stat));
