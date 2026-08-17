import fs from "fs";
import path from "path";
const OS = fs.readFileSync("/tmp/os-path.txt","utf8").trim();
const TICKET = fs.readFileSync("/tmp/os-ticket-path.txt","utf8").trim();
const REPO = "/Users/ueli/Documents/semio";
function walk(dir, out=[]) {
  for (const ent of fs.readdirSync(dir,{withFileTypes:true})) {
    const p = path.join(dir, ent.name);
    if (!ent.isDirectory()) continue;
    if (ent.name === "⚡️implementations") out.push(p);
    else if (ent.name !== "target" && ent.name !== "node_modules") walk(p, out);
  }
  return out;
}
function findLib(implDir) {
  const rust = path.join(implDir, "🦀️rust");
  const cands = [path.join(rust,"📦️lib.rs"), path.join(implDir,"📦️lib.rs")];
  for (const c of cands) if (fs.existsSync(c)) return c;
  if (!fs.existsSync(rust)) return null;
  for (const ent of fs.readdirSync(rust)) {
    const nested = path.join(rust, ent, "📦️lib.rs");
    if (fs.existsSync(nested)) return nested;
  }
  return null;
}
const impls = walk(OS);
let n=0, skip=0;
const log=[];
for (const impl of impls) {
  const parent = path.dirname(impl);
  const dest = path.join(parent, "🦀️component.rs");
  if (fs.existsSync(dest)) continue;
  if (parent.includes("/📡️protocol")) { skip++; log.push("SKIP_PROTOCOL "+path.relative(REPO,parent)); continue; }
  const lib = findLib(impl);
  if (!lib) { skip++; log.push("SKIP_NO_LIB "+path.relative(REPO,parent)); continue; }
  fs.writeFileSync(dest, fs.readFileSync(lib));
  n++;
  log.push("PROMOTED "+path.relative(REPO,dest));
}
fs.writeFileSync(path.join(TICKET,"🧪promote-log.txt"), log.join("\n")+"\n");
fs.copyFileSync("/tmp/os-promote.mjs", path.join(TICKET, "🔧️promote.mjs"));
console.log("promoted="+n+" skipped="+skip+" impls="+impls.length);
