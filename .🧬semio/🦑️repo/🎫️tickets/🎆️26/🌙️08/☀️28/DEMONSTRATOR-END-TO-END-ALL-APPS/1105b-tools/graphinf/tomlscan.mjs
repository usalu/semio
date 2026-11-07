import {execFileSync} from "node:child_process";
import {readFileSync, existsSync} from "node:fs";
const root = process.argv[2];
const files = execFileSync("git", ["ls-files", "-z", "--", "*Cargo.toml"], {cwd: root, maxBuffer: 1 << 28}).toString().split("\0").filter(f => f && !f.includes(".🧬semio"));
let bad = 0;
for (const f of files) { if (!existsSync(`${root}/${f}`)) continue; try { Bun.TOML.parse(readFileSync(`${root}/${f}`, "utf8")); } catch (e) { bad++; console.log(f, String(e.message).slice(0, 80)); } }
console.log("scanned", files.length, "bad", bad);
