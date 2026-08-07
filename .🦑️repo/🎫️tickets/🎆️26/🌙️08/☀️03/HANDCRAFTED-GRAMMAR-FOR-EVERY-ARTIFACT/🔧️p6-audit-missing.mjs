#!/usr/bin/env bun
import { readFileSync, writeFileSync, readdirSync, statSync } from "fs";
import { join, relative, dirname } from "path";
import { fileURLToPath } from "url";
const ticket = dirname(fileURLToPath(import.meta.url));
const repo = "/Users/ueli/Documents/semio";
function walk(dir, out=[]) {
  for (const n of readdirSync(dir)) {
    if (n==="target"||n==="node_modules") continue;
    const p=join(dir,n); const st=statSync(p);
    if (st.isDirectory()) walk(p,out); else if (n.endsWith(".rs")) out.push(p);
  }
  return out;
}
function codeLines(content) {
  return content.split(/\n/).map(line => {
    const t=line.trimStart();
    if (t.startsWith("//")) return "";
    return line;
  }).join("\n");
}
const missing=[];
const roots=[join(repo,"✏️s/🔌️plugins"), join(repo,"� combos")];
// find store+dsl
for (const ent of readdirSync(repo)) {
  const dsl=join(repo,ent,"🛍️products","💻️os","🔨️modules","🗣️dsl","🦀️component.rs");
  const store=join(repo,ent,"🛍️products","💻️os","🔨️modules","🏪️store","🦀️component.rs");
  try { if (statSync(dsl).isFile()) roots.push(dsl); } catch {}
  try { if (statSync(store).isFile()) roots.push(store); } catch {}
}
const files=[];
for (const r of roots) {
  try {
    if (statSync(r).isDirectory()) files.push(...walk(r).filter(f=>f.includes("/🗿️artifacts/")||f.endsWith("🏪️store/🦀️component.rs")||f.endsWith("🗣️dsl/🦀️component.rs")));
    else files.push(r);
  } catch {}
}
for (const f of files) {
  const c=readFileSync(f,"utf8");
  const code=codeLines(c);
  const lines=c.split(/\n/);
  for (let i=0;i<lines.length;i++) {
    const t=lines[i].trimStart();
    if (t.startsWith("//")) continue;
    const m=t.match(/^#\[derive\s*\(([^)]*)\)\]/);
    if (!m) continue;
    const hasDoc=/\bDslDocument\b/.test(m[1]);
    const hasOps=/\bDslOps\b/.test(m[1]);
    if (!hasDoc&&!hasOps) continue;
    let name=null;
    for (let j=i+1;j<Math.min(lines.length,i+12);j++) {
      const tt=lines[j].trim();
      if (tt.startsWith("#[")||tt.startsWith("//")||tt==="") continue;
      const tm=tt.match(/^(?:pub(?:\([^)]*\))?\s+)?(struct|enum)\s+([A-Za-z0-9_]+)/);
      if (tm){name=tm[2];break;}
      break;
    }
    if (!name) continue;
    if (hasDoc && !new RegExp(`DocumentDsl\\s+for\\s+${name}\\b`).test(code)) missing.push({f:relative(repo,f),name,kind:"DocumentDsl"});
    if (hasOps && !new RegExp(`OpText\\s+for\\s+${name}\\b`).test(code)) missing.push({f:relative(repo,f),name,kind:"OpText"});
    if (hasOps && !new RegExp(`OpBinary\\s+for\\s+${name}\\b`).test(code)) missing.push({f:relative(repo,f),name,kind:"OpBinary"});
    if (hasDoc && !new RegExp(`DocumentPack\\s+for\\s+${name}\\b`).test(code)) missing.push({f:relative(repo,f),name,kind:"DocumentPack"});
  }
}
writeFileSync(join(ticket,"🧪p6-missing-traits.json"), JSON.stringify(missing,null,2));
console.log("missing", missing.length);
console.log(JSON.stringify(missing.slice(0,50),null,2));
