import { readdirSync, readFileSync, statSync } from "node:fs";

const root = "C:/git/semio";
const ui = `${root}/🧰️framework/🔨️modules/🖱️ui`;
const excludedModules = ["🔡️ansi", "🔌️backend", "🪟️windows", "🏃️host", "🚇️pty"];
const roots = process.argv.slice(2).filter((a) => !a.startsWith("--")).length ? process.argv.slice(2).filter((a) => !a.startsWith("--")) : [`${ui}/⌨️tui`, `${ui}/🧱️elements`];

function* walk(directory: string): Generator<string> {
  for (const name of readdirSync(directory)) {
    const path = `${directory}/${name}`;
    if (statSync(path).isDirectory()) yield* walk(path);
    else if (name.endsWith(".rs")) yield path;
  }
}

const inScope = (path: string) =>
  !excludedModules.some((name) => path.includes(`/${name}/`)) && (!path.includes("/🧱️elements/") || path.includes("/⌨️tui/"));
const docLine = /^\s*(\/\/\/|\/\/!)(?:\s+(.*))?$/;
const normalise = (emoji: string) => emoji.replaceAll("\uFE0F", "");
const startsWithEmoji = (token: string) => {
  const first = token.codePointAt(0) ?? 0;
  return first > 0x2000 && !/^\p{L}/u.test(token);
};
const showText = process.argv.includes("--text");

let problems = 0;
let files = 0;
for (const root of roots) {
  for (const path of walk(root)) {
    if (!inScope(path)) continue;
    files++;
    const lines = readFileSync(path, "utf8").split(/\r?\n/);
    const seen = new Map<string, number[]>();
    let previousIsDoc = false;
    lines.forEach((line, index) => {
      const doc = docLine.exec(line);
      if (!doc) {
        previousIsDoc = false;
        return;
      }
      if (previousIsDoc) return;
      previousIsDoc = true;
      const token = (doc[2] ?? "").split(/\s+/)[0] ?? "";
      if (!startsWithEmoji(token)) {
        console.log(`MISSING ${path.slice(root.length + 1)}:${index + 1}: ${line.trim().slice(0, 80)}`);
        problems++;
        return;
      }
      const key = normalise(token);
      seen.set(key, [...(seen.get(key) ?? []), index + 1]);
    });
    for (const [emoji, at] of seen) {
      if (at.length < 2) continue;
      problems++;
      console.log(`DUPLICATE ${path.slice(root.length + 1)} ${emoji} x${at.length}: ${at.join(",")}`);
      if (showText) for (const number of at) console.log(`    ${number}: ${lines[number - 1].trim().slice(0, 110)}`);
    }
  }
}
console.log(`files=${files} problems=${problems}`);
process.exit(problems ? 1 : 0);
