import { readFileSync, readdirSync, statSync } from "fs";
import { join } from "path";
function walk(d, pred, acc=[]){
  for (const n of readdirSync(d)){
    if(n==="node_modules"||n==="target"||n===".git") continue;
    const p=join(d,n);
    try {
      const s=statSync(p);
      if(s.isDirectory()) walk(p, pred, acc);
      else if(pred(p)) acc.push(p);
    } catch {}
  }
  return acc;
}
const kind = process.argv[2];
const start = Number(process.argv[3]);
const end = Number(process.argv[4]);
const pick = (pred) => walk(".", pred)[0];
let p;
if (kind==="plugin") p = pick(x=>x.endsWith("🔌️plugin/🦀️component.rs")&&x.includes("🛍️products"));
else if (kind==="glue") p = pick(x=>x.includes("🔌️plugin/📦️packages/🦀️rust/📦️glue.rs")&&x.includes("🛍️products")&&!x.includes("🖥️host"));
else if (kind==="host") p = pick(x=>x.includes("🔌️plugin/🖥️host/🦀️component.rs"));
else if (kind==="wit") p = pick(x=>x.includes("🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️world.wit"));
else if (kind==="channel") p = pick(x=>x.includes("📡️spr/🌵️channel/🦀️component.rs"));
else p = kind;
console.error("file", p);
const lines = readFileSync(p, "utf8").split("\n");
for (let i = start - 1; i < end && i < lines.length; i++) console.log(`${i+1}|${lines[i]}`);
