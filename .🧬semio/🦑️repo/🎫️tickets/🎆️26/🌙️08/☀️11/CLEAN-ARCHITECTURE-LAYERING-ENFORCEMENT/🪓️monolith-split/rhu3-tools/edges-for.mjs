import fs from "node:fs";
import { spawnSync } from "node:child_process";
import { imports } from "./clos.mjs";
const R = "/Users/ueli/Documents/semio/";
const [outFile, targetRe, scopeArg] = process.argv.slice(2);
const re = new RegExp(targetRe);
const RT = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/";
const scopes = (scopeArg ?? RT + "🎯️targets/⚛️react," + RT + "🧱️elements").split(",");
const git = args => spawnSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", ...args], { cwd: R, encoding: "utf8", maxBuffer: 1 << 28 }).stdout.split("\n").filter(Boolean).map(s => s.normalize("NFC"));
const files = new Set();
for (const s of scopes) for (const f of [...git(["ls-files", "--", s]), ...git(["ls-files", "--others", "--exclude-standard", "--", s])]) if (/\.(tsx?|mts)$/.test(f)) files.add(f);
const rows = [];
for (const f of files) {
  if (!fs.existsSync(R + f)) continue;
  for (const d of imports(R + f)) {
    if (!d.file || d.ext) continue;
    const to = d.file.replace(R, "").normalize("NFC");
    if (re.test(to)) rows.push({ from: f, to, spec: d.spec, types: [] });
  }
}
fs.writeFileSync(outFile, rows.map(r => JSON.stringify(r)).join("\n") + (rows.length ? "\n" : ""));
console.log("files", files.size, "rows", rows.length);
