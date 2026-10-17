// 🗂️ Dumps package-name to manifest-path TSV for all crates (git-tracked and untracked).
import {execFileSync} from "node:child_process";
import {readFileSync, existsSync} from "node:fs";
const ROOT = "/Users/ueli/Documents/semio";
const git = (extra: string[]) => execFileSync("git", ["-c", "core.quotepath=false", "ls-files", "-z", ...extra], {cwd: ROOT, encoding: "utf8", maxBuffer: 1 << 28}).split("\0").filter(Boolean);
const all = [...new Set([...git(["--", "Cargo.toml", "*/Cargo.toml"]), ...git(["--others", "--exclude-standard", "--", "Cargo.toml", "*/Cargo.toml"])])];
const out: string[] = [];
for (const m of all) {
  const p = ROOT + "/" + m;
  if (!existsSync(p)) continue;
  const t = readFileSync(p, "utf8");
  const i = t.indexOf("[package]");
  if (i < 0) continue;
  const mm = /^\s*name\s*=\s*"([^"]+)"/m.exec(t.slice(i));
  if (mm) out.push(mm[1] + "\t" + m);
}
console.log(out.join("\n"));
