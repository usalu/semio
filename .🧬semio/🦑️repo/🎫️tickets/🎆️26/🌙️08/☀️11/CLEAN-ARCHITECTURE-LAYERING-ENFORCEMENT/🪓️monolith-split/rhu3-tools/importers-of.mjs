import { spawnSync } from "node:child_process";
import { imports } from "./clos.mjs";
const R = "/Users/ueli/Documents/semio/";
const [target, ...scopes] = process.argv.slice(2);
const git = args => spawnSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", ...args], { cwd: R, encoding: "utf8", maxBuffer: 1 << 28 }).stdout.split("\n").filter(Boolean).map(s => s.normalize("NFC"));
const files = new Set();
for (const s of scopes) for (const f of [...git(["ls-files", "--", s]), ...git(["ls-files", "--others", "--exclude-standard", "--", s])]) if (/\.(tsx?|mts)$/.test(f)) files.add(f);
const T = R + target;
let n = 0;
for (const f of files) { try { for (const d of imports(R + f)) if (d.file === T) { console.log(f.slice(-110), "|", d.spec.slice(-50)); n++; } } catch {} }
console.log("importers", n);
