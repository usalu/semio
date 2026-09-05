import { execFileSync } from "node:child_process";

const out = execFileSync("git", ["ls-files", "-z"], { cwd: process.cwd(), maxBuffer: 1024 * 1024 * 200 });
const buf = out;
const paths = [];
let start = 0;
for (let i = 0; i < buf.length; i++) {
  if (buf[i] === 0) {
    paths.push(buf.slice(start, i).toString("utf8"));
    start = i + 1;
  }
}
if (start < buf.length) paths.push(buf.slice(start).toString("utf8"));

// Windows MAX_PATH classic limit is 260 chars for the full path (drive+dir+file+NUL).
// Git checkout path length = length of UTF-16 code units of the relative path (since NTFS stores UTF-16).
function utf16Len(s) {
  let n = 0;
  for (const ch of s) n += ch.codePointAt(0) > 0xffff ? 2 : 1;
  return n;
}

const rows = paths.map(p => ({ p, len: utf16Len(p) })).sort((a, b) => b.len - a.len);
console.log("Top 15 longest tracked relative paths (UTF-16 code units):");
for (const r of rows.slice(0, 15)) console.log(r.len, r.p);

// assume a plausible windows checkout root, e.g. C:\Users\name\Documents\semio\ (~30 chars)
const ASSUMED_ROOT_PREFIX = 35;
const over260 = rows.filter(r => r.len + ASSUMED_ROOT_PREFIX > 260);
console.log(`\nPaths that would exceed 260 chars with a ~35-char Windows root prefix: ${over260.length}`);

// case-insensitive collisions among tracked paths
const map = new Map();
for (const p of paths) {
  const key = p.toLowerCase();
  if (!map.has(key)) map.set(key, []);
  map.get(key).push(p);
}
const collisions = [...map.values()].filter(v => v.length > 1 || (v.length === 1 && v[0] !== v[0]));
const realCollisions = [...map.entries()].filter(([k, v]) => new Set(v).size > 1);
console.log(`\nCase-insensitive path collisions: ${realCollisions.length}`);
for (const [k, v] of realCollisions.slice(0, 20)) console.log(v);
