import { readFileSync } from "node:fs";
import { spawnSync } from "node:child_process";

const root = "C:/git/semio";
const scopes = ["🧰️framework/🔨️modules/🖱️ui", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard"];
const skipped: string[] = [];
const status = spawnSync("git", ["-c", "core.quotepath=false", "status", "--short", "-uall", "--", ...scopes], { cwd: root, encoding: "utf8", maxBuffer: 1 << 28 });
const files = status.stdout.split(/\r?\n/).filter((line) => line.length > 3 && !line.startsWith(" D") && !line.startsWith("D ")).map((line) => line.slice(3).replace(/^"|"$/g, "")).filter((path) => path.endsWith(".rs") && !skipped.some((name) => path.includes(`/${name}/`)));
const docLine = /^\s*(\/\/\/|\/\/!)(?:\s+(.*))?$/;
const startsWithEmoji = (token: string) => /[⃣️]/.test(token) || ((token.codePointAt(0) ?? 0) > 0x2000 && !/^\p{L}/u.test(token));
let problems = 0;
for (const path of files) {
  const lines = readFileSync(`${root}/${path}`, "utf8").split(/\r?\n/);
  const seen = new Map<string, number[]>();
  let previousIsDoc = false;
  lines.forEach((line, index) => {
    const doc = docLine.exec(line);
    if (!doc) return void (previousIsDoc = false);
    if (previousIsDoc) return;
    previousIsDoc = true;
    const token = (doc[2] ?? "").split(/\s+/)[0] ?? "";
    if (!startsWithEmoji(token)) {
      console.log(`MISSING ${path}:${index + 1}: ${line.trim().slice(0, 90)}`);
      problems++;
      return;
    }
    const key = token.replaceAll("\uFE0F", "");
    seen.set(key, [...(seen.get(key) ?? []), index + 1]);
  });
  for (const [emoji, at] of seen) {
    if (at.length < 2) continue;
    problems++;
    console.log(`DUPLICATE ${path} ${emoji} x${at.length}: ${at.join(",")}`);
    for (const number of at) console.log(`    ${number}: ${lines[number - 1].trim().slice(0, 100)}`);
  }
  lines.forEach((line, index) => {
    if (/^\s+\/\/($|[^\/!])/.test(line) && !/wgpu/.test(path)) console.log(`COMMENT ${path}:${index + 1}: ${line.trim().slice(0, 80)}`);
    if (/\[DEBUG\]/.test(line)) console.log(`DEBUG ${path}:${index + 1}`);
  });
}
console.log(`files=${files.length} problems=${problems}`);
