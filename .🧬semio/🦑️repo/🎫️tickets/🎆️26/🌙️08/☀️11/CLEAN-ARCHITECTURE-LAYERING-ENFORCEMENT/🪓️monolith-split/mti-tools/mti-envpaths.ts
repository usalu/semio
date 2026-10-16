// 🔭 Read-only: resolves every include_str!/include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "<rel>")) literal under a pathspec against one manifest directory and lists those whose target is missing.
import {existsSync, readFileSync} from "node:fs";
import {resolve} from "node:path";
import {execFileSync} from "node:child_process";

const ROOT = "/Users/ueli/Documents/semio";
const [manifestDir, ...scope] = process.argv.slice(2);
const files = execFileSync("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "ls-files", "-z", "--", ...scope], {cwd: ROOT, encoding: "utf8", maxBuffer: 1 << 28}).split("\0").filter(f => f.endsWith(".rs"));
const re = /(include_str|include_bytes|include)!\(\s*concat!\(\s*env!\("CARGO_MANIFEST_DIR"\)\s*,\s*"([^"]+)"\s*\)\s*\)/gu;
for (const file of files) {
  let text: string;
  try { text = readFileSync(resolve(ROOT, file), "utf8"); } catch { continue; }
  const lines = text.split("\n");
  lines.forEach((line, index) => {
    for (const match of line.matchAll(re)) {
      const target = resolve(ROOT, manifestDir!) + match[2];
      const normalized = resolve(target);
      if (!existsSync(normalized)) console.log(["MISSING", file, index + 1, match[2], normalized.replace(ROOT + "/", "")].join("\t"));
    }
  });
}
