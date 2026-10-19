import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const { build } = require("esbuild");
const r = await build({ entryPoints: [process.argv[2]], bundle: true, write: false, platform: "node", format: "esm", logLevel: "silent", packages: "external", loader: { ".json": "json" } });
const code = r.outputFiles[0].text;
const { writeFileSync } = await import("node:fs");
writeFileSync(process.argv[3], code);
const { execFileSync } = await import("node:child_process");
try { execFileSync("node", ["--check", process.argv[3]], { stdio: "pipe" }); console.log("syntax ok"); } catch (e) { console.log(String(e.stderr).slice(0, 800)); }
