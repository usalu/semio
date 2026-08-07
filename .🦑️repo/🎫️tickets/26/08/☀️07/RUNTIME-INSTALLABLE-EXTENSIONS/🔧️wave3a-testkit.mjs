import fs from "fs";
import path from "path";
const root = "/Users/ueli/Documents/semio";
const s = fs.readdirSync(root).find(n=>n.startsWith("✏"));
const flow = path.join(root,s,"🔌️plugins", fs.readdirSync(path.join(root,s,"🔌️plugins")).find(n=>n.includes("flow")));
function walk(dir,acc=[]) {
  for (const ent of fs.readdirSync(dir,{withFileTypes:true})) {
    if (["target","node_modules","pkg"].includes(ent.name)) continue;
    const p=path.join(dir,ent.name);
    if (ent.isDirectory()) walk(p,acc); else if (/\.rs$/.test(ent.name)) acc.push(p);
  }
  return acc;
}
for (const p of walk(flow)) {
  const t=fs.readFileSync(p,"utf8");
  if (/fn flow_app|testkit|install_flow_extension|install_builtin|contributions_json|FLOW_AUTOMATIONS/.test(t)) {
    const lines=t.split(/\n/);
    lines.forEach((l,i)=>{
      if (/fn flow_app|install_flow|install_builtin|contributions|testkit|struct.*Testkit|mod testkit/.test(l))
        console.log(p+":"+(i+1)+":"+l.trim().slice(0,160));
    });
  }
}
// read text extension
const fw=fs.readdirSync(root).find(n=>n.endsWith("framework"));
const textDir=path.join(root,fw,"🛍️products","💻️os","🔨️modules","🌊️flow","🧩️extensions");
const text=path.join(textDir, fs.readdirSync(textDir).find(n=>n.includes("text")), "🦀️component.rs");
console.log("\n=== TEXT ===\n"+fs.readFileSync(text,"utf8"));
