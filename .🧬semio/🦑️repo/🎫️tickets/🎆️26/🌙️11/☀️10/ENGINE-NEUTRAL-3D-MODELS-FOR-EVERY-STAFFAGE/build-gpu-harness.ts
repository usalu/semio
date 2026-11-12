import { build } from "esbuild";
import { copyFileSync, mkdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const root = process.cwd();
const out = resolve(root, process.argv[2] ?? ".");
mkdirSync(out, { recursive: true });
await build({
  entryPoints: [resolve(root, "🧰️framework/🔨️modules/🧊️3d/🗿️model/🖥️viewer/🧪️tests/🧰️support/🖥️harness/🟦️.ts")],
  bundle: true, format: "iife", target: "es2022", outfile: resolve(out, "harness.js"), logLevel: "error",
});
copyFileSync(resolve(root, "🧰️framework/🔨️modules/🧊️3d/🗿️model/🖥️viewer/🧫️fixtures/🧱️rig/🔣️.json"), resolve(out, "rig.json"));
writeFileSync(resolve(out, "index.html"), `<!doctype html><meta charset="utf-8"><title>gpu</title><body style="font:12px monospace"><pre id="out">running</pre><script src="harness.js"></script><script>fetch("rig.json").then(r=>r.json()).then(d=>window.semioGpu.run(d.document)).then(r=>{window.__result=r;document.getElementById("out").textContent=JSON.stringify(r,null,1)}).catch(e=>{window.__result={error:String(e&&e.stack||e)};document.getElementById("out").textContent=String(e&&e.stack||e)})</script>`);
