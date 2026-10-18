/** 🔍️ Bundles every tracked TS module of the scope with packages external, so each unresolved relative import or alias surfaces; prints one line per failing file. */
import { build } from "esbuild";
import { readFileSync } from "node:fs";
const files = readFileSync(process.argv[2], "utf8").split("\n").filter(Boolean);
const R = "/Users/ueli/Documents/semio/";
let bad = 0;
for (const f of files) {
  try {
    const r = await build({ entryPoints: [R + f], absWorkingDir: R, bundle: true, packages: "external", platform: "node", format: "esm", write: false, logLevel: "silent", loader: { ".json": "json" } });
    void r;
  } catch (e) {
    bad++;
    const msgs = (e.errors ?? []).slice(0, 3).map((m) => m.text).join(" | ");
    console.log("FAIL", f.slice(-110), "::", msgs.slice(0, 400));
  }
}
console.log("checked", files.length, "failed", bad);
