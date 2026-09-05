// 🪟️ Shorten tracked paths whose UTF-16 length runs too close to Windows MAX_PATH (260),
// leaving headroom for a real clone-location prefix. Two cases:
//  1. Long "🧪️tests/<description>/..." case directories -> rename the whole directory once.
//  2. Stray long filenames elsewhere -> rename just the file.
import { execSync } from "node:child_process";
import { createHash } from "node:crypto";

const SAFE_MAX = 190; // target ceiling per full relative path (UTF-16 code units)
const MARKER = "🧪️tests/";

const tracked = execSync("git ls-files -z", { maxBuffer: 1024 * 1024 * 64 })
  .toString("utf8")
  .split("\0")
  .filter(Boolean);

function shortHash(s: string): string {
  return createHash("sha1").update(s).digest("hex").slice(0, 6);
}

function truncateWithHash(name: string, targetLen: number, salt: string): string {
  const dotIdx = name.lastIndexOf(".");
  const ext = dotIdx > 0 ? name.slice(dotIdx) : "";
  const base = ext ? name.slice(0, -ext.length) : name;
  const hash = shortHash(salt);
  const keepLen = Math.max(20, targetLen - ext.length - 1 - hash.length);
  let cut = base.slice(0, keepLen);
  const lastDash = cut.lastIndexOf("-");
  if (lastDash > keepLen * 0.5) cut = cut.slice(0, lastDash);
  return `${cut}-${hash}${ext}`;
}

// case directories
const dirMaxLen = new Map<string, number>();
const strayFiles: string[] = [];

for (const p of tracked) {
  const idx = p.indexOf(MARKER);
  if (idx !== -1) {
    const afterMarker = p.slice(idx + MARKER.length);
    const nextSlash = afterMarker.indexOf("/");
    if (nextSlash !== -1) {
      const dirPath = p.slice(0, idx + MARKER.length + nextSlash);
      dirMaxLen.set(dirPath, Math.max(dirMaxLen.get(dirPath) ?? 0, p.length));
      continue;
    }
  }
  if (p.length > SAFE_MAX) strayFiles.push(p);
}

const dirRenames: [string, string][] = [];
const usedSiblingNames = new Map<string, Set<string>>();

for (const [dirPath, maxLen] of dirMaxLen) {
  if (maxLen <= SAFE_MAX) continue;
  const parent = dirPath.slice(0, dirPath.lastIndexOf("/"));
  const caseName = dirPath.slice(dirPath.lastIndexOf("/") + 1);
  const overBy = maxLen - SAFE_MAX;
  const targetLen = Math.max(20, caseName.length - overBy);
  let newName = truncateWithHash(caseName, targetLen, dirPath);
  const siblings = usedSiblingNames.get(parent) ?? new Set<string>();
  let attempt = 0;
  while (siblings.has(newName)) {
    attempt++;
    newName = truncateWithHash(caseName, targetLen - 2, dirPath + "#" + attempt);
  }
  siblings.add(newName);
  usedSiblingNames.set(parent, siblings);
  dirRenames.push([dirPath, `${parent}/${newName}`]);
}

const fileRenames: [string, string][] = [];
const usedFileSiblings = new Map<string, Set<string>>();

for (const p of strayFiles) {
  const parent = p.slice(0, p.lastIndexOf("/"));
  const fileName = p.slice(p.lastIndexOf("/") + 1);
  const overBy = p.length - SAFE_MAX;
  const targetLen = Math.max(20, fileName.length - overBy);
  let newName = truncateWithHash(fileName, targetLen, p);
  const siblings = usedFileSiblings.get(parent) ?? new Set<string>();
  let attempt = 0;
  while (siblings.has(newName)) {
    attempt++;
    newName = truncateWithHash(fileName, targetLen - 2, p + "#" + attempt);
  }
  siblings.add(newName);
  usedFileSiblings.set(parent, siblings);
  fileRenames.push([p, `${parent}/${newName}`]);
}

console.log(`planned directory renames: ${dirRenames.length}`);
console.log(`planned file renames: ${fileRenames.length}`);
const logPath = process.argv[2];
if (logPath) {
  const fs = require("node:fs");
  const lines = [...dirRenames, ...fileRenames].map(([a, b]) => `${a}\t${b}`);
  fs.writeFileSync(logPath, lines.join("\n") + "\n");
}

if (process.argv.includes("--apply")) {
  const fs = require("node:fs");
  const path = require("node:path");
  let done = 0;
  const all = [...dirRenames, ...fileRenames];
  for (const [from, to] of all) {
    if (!fs.existsSync(from)) continue; // already moved by an earlier rename in this run
    fs.mkdirSync(path.dirname(to), { recursive: true });
    fs.renameSync(from, to);
    done++;
  }
  console.log(`renamed ${done}/${all.length} on disk; staging with git add -A`);
  for (let attempt = 0; ; attempt++) {
    try {
      execSync("git add -A", { maxBuffer: 1024 * 1024 * 64 });
      break;
    } catch (err) {
      if (attempt >= 20) throw err;
      execSync("sleep 1");
    }
  }
  console.log("staged");
}
