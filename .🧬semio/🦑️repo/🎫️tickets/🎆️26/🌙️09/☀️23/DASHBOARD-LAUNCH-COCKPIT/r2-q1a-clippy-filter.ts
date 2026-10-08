import { readFileSync } from "node:fs";

const file = process.argv[2];
const excluded = ["🔡️ansi", "🔌️backend", "🪟️windows", "🏃️host", "🚇️pty", "backend-native", "ansi-unit"];
const seen = new Set<string>();
for (const raw of readFileSync(file, "utf8").split(/\r?\n/)) {
  if (!/: (warning|error)/.test(raw) || /generated \d+ warn/.test(raw)) continue;
  const line = raw.replaceAll("\\", "/");
  if (!line.includes("⌨️tui")) continue;
  if (excluded.some((name) => line.includes(name))) continue;
  const normalised = line.replace(/^.*\/🖱️ui\/(📦️packages\/🦀️rust\/\.\.\/\.\.\/🎯️targets\/⌨️tui\/\.\.\/\.\.\/)?/, "").replace(/(\.\.\/)+/g, "");
  seen.add(normalised.slice(0, 230));
}
console.log([...seen].sort().join("\n"));
console.log(`total ${seen.size}`);
