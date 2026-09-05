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

const forbidden = /[<>:"|?*\x00-\x1f]/;
const reserved = /^(con|prn|aux|nul|com[0-9]|lpt[0-9])(\..*)?$/i;

function isIllegal(name) {
  if (name.length === 0) return true;
  if (name.trim().length === 0) return true;
  if (name.endsWith(" ") || name.endsWith(".") || name.startsWith(" ")) return true;
  if (forbidden.test(name)) return true;
  if (reserved.test(name)) return true;
  return false;
}

let count = 0;
for (const p of paths) {
  const parts = p.split("/");
  for (const part of parts) {
    if (isIllegal(part)) {
      console.log(JSON.stringify(p));
      count++;
      break;
    }
  }
}
console.error(`total scanned: ${paths.length}, illegal: ${count}`);
