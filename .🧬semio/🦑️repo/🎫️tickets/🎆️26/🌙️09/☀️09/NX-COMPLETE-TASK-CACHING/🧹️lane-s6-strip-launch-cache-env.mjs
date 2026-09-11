import { readFileSync, writeFileSync } from "node:fs";

const path = process.argv[2];
const keysToRemove = ["CARGO_TARGET_DIR", "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER"];
const keyPattern = new RegExp(`^\\s*"(${keysToRemove.join("|")})":\\s*"[^"]*"\\s*,?\\s*$`);
const hasTrailingComma = (line) => /,\s*$/.test(line);

const raw = readFileSync(path, "utf8");
const lines = raw.split("\n");
const keep = new Array(lines.length).fill(true);
let removedCount = { CARGO_TARGET_DIR: 0, RUSTC_WRAPPER: 0, RUSTC_WORKSPACE_WRAPPER: 0 };

for (let i = 0; i < lines.length; i++) {
  const line = lines[i];
  const m = line.match(keyPattern);
  if (m) {
    keep[i] = false;
    removedCount[m[1]]++;
  }
}

// Fix trailing commas: for each removed line that had NO trailing comma (was last
// property in its object), find nearest preceding kept line and strip its trailing comma.
for (let i = 0; i < lines.length; i++) {
  if (keep[i]) continue;
  if (hasTrailingComma(lines[i])) continue; // had comma, removal is safe as-is
  // find nearest preceding kept line
  let j = i - 1;
  while (j >= 0 && !keep[j]) j--;
  if (j >= 0 && hasTrailingComma(lines[j])) {
    lines[j] = lines[j].replace(/,(\s*)$/, "$1");
  }
}

const out = lines.filter((_, i) => keep[i]).join("\n");
writeFileSync(path, out);
console.log(JSON.stringify(removedCount));
console.log("original lines:", lines.length, "output lines:", out.split("\n").length);
