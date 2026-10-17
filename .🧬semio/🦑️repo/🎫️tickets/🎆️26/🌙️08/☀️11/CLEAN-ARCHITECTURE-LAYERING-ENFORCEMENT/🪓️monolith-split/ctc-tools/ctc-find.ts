// 📍️ Prints the manifest path of crates by package name.
import {execFileSync} from "node:child_process";
import {readFileSync, existsSync} from "node:fs";
const ROOT = "/Users/ueli/Documents/semio";
const names = new Set(process.argv.slice(2));
const git = (extra: string[]) => execFileSync("git", ["-c", "core.quotepath=false", "ls-files", "-z", ...extra], {cwd: ROOT, encoding: "utf8", maxBuffer: 1 << 28}).split("\0").filter(Boolean);
const all = [...new Set([...git(["--", "Cargo.toml", "*/Cargo.toml"]), ...git(["--others", "--exclude-standard", "--", "Cargo.toml", "*/Cargo.toml"])])];
for (const m of all) {
  const p = ROOT + "/" + m;
  if (!existsSync(p)) continue;
  const t = readFileSync(p, "utf8");
  const mm = /^\s*name\s*=\s*"([^"]+)"/m.exec(t.slice(t.indexOf("[package]") >= 0 ? t.indexOf("[package]") : 0));
  if (mm && names.has(mm[1]!)) console.log(mm[1] + "\t" + m);
}
