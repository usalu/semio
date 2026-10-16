import {readFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
const cmp=(a:string,b:string)=>Buffer.compare(Buffer.from(a),Buffer.from(b));
function problems(text: string): Set<string> {
  const parts = text.split(/\n(?=\[\[package\]\]\n)/u);
  if (!parts[0]!.startsWith("[[package]]")) parts.shift();
  const pk = parts.map(b=>b.trimEnd()).map(b=>({name:/name = "([^"]+)"/u.exec(b)![1]!, version:/version = "([^"]+)"/u.exec(b)![1]!, deps:[...(/\ndependencies = \[([\s\S]*?)\n\]/u.exec(b)?.[1]??"").matchAll(/"((?:[^"\\]|\\.)*)"/gu)].map(m=>m[1]!)}));
  const byName = new Map<string, any[]>(); for (const p of pk) byName.set(p.name,[...(byName.get(p.name)??[]),p]);
  const out = new Set<string>();
  for (let i=1;i<pk.length;i++) if (cmp(pk[i-1]!.name, pk[i]!.name) > 0) out.add(`unsorted package ${pk[i-1]!.name} > ${pk[i]!.name}`);
  for (const p of pk) {
    for (let i=1;i<p.deps.length;i++) if (cmp(p.deps[i-1]!, p.deps[i]!) > 0) { out.add(`unsorted deps of ${p.name}`); break; }
    for (const d of p.deps) { const [n,v] = d.split(" "); const c = byName.get(n!); if (!c) out.add(`${p.name} -> ${d} missing package`); else if (c.length>1 && !v) out.add(`${p.name} -> ${d} ambiguous`); else if (v && !c.some((x:any)=>x.version===v)) out.add(`${p.name} -> ${d} version missing`); }
  }
  return out;
}
const touched = readFileSync(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/sf3-tools/touched-locks.txt","utf8").split("\n").filter(l=>l.endsWith("Cargo.lock"));
let regress = 0;
for (const lock of touched) {
  const now = problems(readFileSync(lock,"utf8"));
  let before = new Set<string>();
  try { before = problems(execFileSync("git", ["-c","core.fsmonitor=false","-c","core.quotepath=false","show", `HEAD:${lock}`], {encoding:"utf8", maxBuffer: 1<<28})); } catch {}
  const fresh = [...now].filter(p => !before.has(p));
  if (fresh.length) { regress++; console.log("NEW problems", lock.slice(-70), fresh.slice(0,3)); }
}
console.log("locks", touched.length, "with new problems vs HEAD", regress);
