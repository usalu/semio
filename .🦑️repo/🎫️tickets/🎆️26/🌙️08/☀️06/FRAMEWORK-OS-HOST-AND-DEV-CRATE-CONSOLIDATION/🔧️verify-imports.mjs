
import fs from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";

const ROOT = "/Users/ueli/Documents/semio";
const TICK = process.argv[2];
const targets = JSON.parse(fs.readFileSync(path.join(TICK, "🧪demonstrator-path-targets.json"), "utf8"));
const abs = path.join(ROOT, targets.osDevScript.replace(/^\.\//, ""));
const mod = await import(pathToFileURL(abs).href);
for (const s of ["buildEngineWasm", "buildPlugins", "ensurePluginRegistry", "semioBackboneVitePlugin", "semioBlobVitePlugin"]) {
  console.log(s, typeof mod[s]);
}
const demScript = fs.readFileSync(path.join(ROOT, "♻️mit-bestand/🧺️demonstrator/📜️script.ts"), "utf8");
const m = demScript.match(/from "([^"]+script\.ts)"/);
console.log("dem import", m[1], fs.existsSync(path.join(ROOT, "♻️mit-bestand/🧺️demonstrator", m[1])));
const vite = fs.readFileSync(path.join(ROOT, "♻️mit-bestand/🧺️demonstrator/⚙️vite.config.ts"), "utf8");
console.log("vite implementations left", (vite.match(/⚡️implementations/g) || []).length);
console.log("brand from", fs.readFileSync(path.join(ROOT, "♻️mit-bestand/🧺️demonstrator/🟦️brand.ts"), "utf8").match(/from "([^"]+)"/)?.[1]);
