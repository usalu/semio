import { execFileSync } from "node:child_process";
import { cleanIsWindowsIllegalName } from "/Users/ueli/Documents/semio/📜️script.ts";

const out = execFileSync("git", ["ls-files", "-z"], { cwd: "/Users/ueli/Documents/semio", maxBuffer: 1024 * 1024 * 200 });
const paths = [];
let start = 0;
for (let i = 0; i < out.length; i++) {
  if (out[i] === 0) { paths.push(out.slice(start, i).toString("utf8")); start = i + 1; }
}
let count = 0;
for (const p of paths) {
  for (const part of p.split("/")) {
    if (cleanIsWindowsIllegalName(part)) { console.log(JSON.stringify(p)); count++; break; }
  }
}
console.error(`scanned=${paths.length} illegal(real-impl)=${count}`);
