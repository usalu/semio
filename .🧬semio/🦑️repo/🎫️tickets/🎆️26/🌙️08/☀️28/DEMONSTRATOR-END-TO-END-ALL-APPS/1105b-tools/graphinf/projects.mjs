import {execFileSync} from "node:child_process";
import {readFileSync, existsSync} from "node:fs";
const root = process.argv[2], want = process.argv.slice(3);
const files = execFileSync("git", ["ls-files", "-z", "--", "*project.json"], {cwd: root, maxBuffer: 1 << 28}).toString().split("\0").filter(f => f && !f.includes(".🧬semio") && f.endsWith("📋️project.json"));
const byName = new Map();
for (const f of files) { if (!existsSync(`${root}/${f}`)) continue; try { const j = JSON.parse(readFileSync(`${root}/${f}`, "utf8")); byName.set(j.name, {f, targets: Object.keys(j.targets ?? {})}); } catch {} }
for (const w of want) { const [p, t] = [w.slice(0, w.lastIndexOf(":")), w.slice(w.lastIndexOf(":") + 1)]; const e = byName.get(p); console.log(w, e ? (e.targets.includes(t) ? "project+target authored" : `project ok, target ${t} not authored (inferred?) [${e.targets.join(",")}]`) : "NO project.json by this name"); }
