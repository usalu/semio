import {readdirSync, readFileSync, statSync} from "node:fs";
import {join} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const SCOPES = ["✏️s/🔌️plugins/🎥️shooting","✏️s/🧑‍💻dev/🎭️variants/🎥️shooting","✏️s/🧑‍💻dev/🧩️catalog","🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️serve"];
function walk(d, out=[]) { for (const e of readdirSync(d)) { if (["node_modules","target",".cache"].includes(e)) continue; const p = join(d,e); const st = statSync(p); if (st.isDirectory()) walk(p,out); else out.push(p); } return out; }
const files = SCOPES.flatMap(s => walk(join(ROOT,s)));
const issues = [];
let jsonCount = 0, pairs = 0;
function scan(v, file, path) {
  if (Array.isArray(v)) return v.forEach((x,i)=>scan(x,file,path+"["+i+"]"));
  if (v && typeof v === "object") {
    const keys = Object.keys(v);
    const hasEn = keys.includes("en"), hasDe = keys.includes("de");
    if (hasEn !== hasDe && keys.length <= 3 && keys.every(k=>["en","de"].includes(k))) issues.push({file: file.slice(ROOT.length+1), path, problem: "one-language-only", keys});
    if (hasEn && hasDe) pairs++;
    for (const k of keys) scan(v[k], file, path+"."+k);
  }
}
for (const f of files) {
  if (!f.endsWith(".json")) continue;
  jsonCount++;
  let j; try { j = JSON.parse(readFileSync(f,"utf8")); } catch (e) { issues.push({file: f.slice(ROOT.length+1), problem: "invalid-json", msg: String(e).slice(0,120)}); continue; }
  scan(j, f, "$");
}
console.log(JSON.stringify({jsonCount, bilingualObjects: pairs, issues}, null, 1));
