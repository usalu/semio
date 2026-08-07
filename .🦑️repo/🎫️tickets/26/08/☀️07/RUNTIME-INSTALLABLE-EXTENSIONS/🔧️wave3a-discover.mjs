import fs from "fs";
import path from "path";
const root = "/Users/ueli/Documents/semio";
const framework = fs.readdirSync(root).find((n) => n.endsWith("framework"));
const s = fs.readdirSync(root).find((n) => n.includes("s") && n.startsWith("✏"));
const flowPlugin = path.join(root, s, "🔌️plugins", fs.readdirSync(path.join(root, s, "🔌️plugins")).find(n => n.includes("flow")));
console.log("flowPlugin", flowPlugin);
// find app ids
function walk(dir, depth=0, acc=[]) {
  if (depth>8) return acc;
  for (const ent of fs.readdirSync(dir,{withFileTypes:true})) {
    if (["target","node_modules","pkg"].includes(ent.name)) continue;
    const p=path.join(dir,ent.name);
    if (ent.isDirectory()) walk(p,depth+1,acc);
    else if (/\.(rs|toml|json|ts)$/.test(ent.name)) acc.push(p);
  }
  return acc;
}
const files = walk(flowPlugin);
for (const p of files) {
  const t = fs.readFileSync(p,"utf8");
  if (/flow-play|app_id\s*=\s*"flow|AppId|"id":\s*"flow/.test(t) || t.includes("flow-play")) {
    const lines = t.split(/\n/);
    lines.forEach((l,i)=>{ if (/flow-play|app.?id|App::|"flow"/.test(l) && /play|app/i.test(l)) console.log(p+":"+(i+1)+":"+l.trim().slice(0,160)); });
  }
}
// summarize extension files
const flowFw = path.join(root, framework, "🛍️products","💻️os","🔨️modules");
const flowMod = path.join(flowFw, fs.readdirSync(flowFw).find(n=>n.includes("flow")));
const extRoot = path.join(flowMod, fs.readdirSync(flowMod).find(n=>n.includes("extensions")));
const six = ["core","math","text","logic","dictionary","list"];
for (const id of six) {
  const dir = fs.readdirSync(extRoot).find(n => n.endsWith(id) || n.includes(id));
  const comp = path.join(extRoot, dir, fs.readdirSync(path.join(extRoot,dir)).find(n=>n.includes("component")));
  const t = fs.readFileSync(comp,"utf8");
  const lines = t.split(/\n/).length;
  const hasRegister = /pub fn register\(/.test(t);
  const hasModuleRegistry = /fn module_registry|pub fn module_registry/.test(t);
  const hasWasm = /#\[cfg\(feature = "standalone-wasm"\)\]|wasm_bindgen/.test(t);
  const hasGuest = /extension_exports!|ExtensionBundle/.test(t);
  console.log(JSON.stringify({id, dir, lines, hasRegister, hasModuleRegistry, hasWasm, hasGuest, path:comp}));
}
// check glue aliases needed
const glue = path.join(flowMod,"📦️packages","🦀️rust","📦️glue.rs");
console.log("GLUE\n"+fs.readFileSync(glue,"utf8"));
// usages of flow_extension_{core,math,...} outside install_builtin
const names = ["flow_extension_core","flow_extension_math","flow_extension_text","flow_extension_logic","flow_extension_dictionary","flow_extension_list"];
function walkAll(dir, depth=0) {
  if (depth>14) return;
  let ents; try{ents=fs.readdirSync(dir,{withFileTypes:true});}catch{return;}
  for (const ent of ents) {
    if (["target","node_modules","pkg",".git",".repo-cache"].includes(ent.name)) continue;
    const p=path.join(dir,ent.name);
    if (ent.isDirectory()) walkAll(p,depth+1);
    else if (ent.name.endsWith(".rs")) {
      const t=fs.readFileSync(p,"utf8");
      for (const n of names) {
        if (t.includes(n+"::")) {
          const lines=t.split(/\n/);
          lines.forEach((l,i)=>{ if (l.includes(n+"::")) console.log(`USE ${n} ${p}:${i+1}:${l.trim().slice(0,120)}`); });
        }
      }
    }
  }
}
walkAll(root);
