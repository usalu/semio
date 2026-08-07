import fs from "fs";
import path from "path";
const root = "/Users/ueli/Documents/semio";
function walk(dir, depth=0, acc=[]) {
  if (depth>12) return acc;
  let ents; try{ents=fs.readdirSync(dir,{withFileTypes:true})}catch{return acc}
  for (const ent of ents) {
    if (["target","node_modules","pkg",".git"].includes(ent.name)) continue;
    const p=path.join(dir,ent.name);
    if (ent.isDirectory()) walk(p,depth+1,acc);
    else if (ent.name.endsWith(".rs")||ent.name.endsWith(".tsx")||ent.name.endsWith(".ts")) acc.push(p);
  }
  return acc;
}
const files = [
  ...walk(path.join(root, fs.readdirSync(root).find(n=>n.endsWith("framework")), "🛍️products","💻️os","🔨️modules",
    fs.readdirSync(path.join(root, fs.readdirSync(root).find(n=>n.endsWith("framework")), "🛍️products","💻️os","🔨️modules")).find(n=>n.includes("flow")))),
  ...walk(path.join(root, fs.readdirSync(root).find(n=>n.startsWith("✏")), "🔌️plugins",
    fs.readdirSync(path.join(root, fs.readdirSync(root).find(n=>n.startsWith("✏")), "🔌️plugins")).find(n=>n.includes("procedural")))),
  ...walk(path.join(root, fs.readdirSync(root).find(n=>n.startsWith("✏")), "🔌️plugins",
    fs.readdirSync(path.join(root, fs.readdirSync(root).find(n=>n.startsWith("✏")), "🔌️plugins")).find(n=>n.includes("flow")))),
];
for (const p of files) {
  const t = fs.readFileSync(p,"utf8");
  if (!t.includes("FlowExtension") && !t.includes("sync_host_flow") && !t.includes("install_builtin_flow") && !t.includes("contributions_json")) continue;
  const lines = t.split(/\n/);
  lines.forEach((l,i)=>{
    if (/FlowExtension|sync_host_flow|install_builtin|app_id|procedural3d-play|flow-play|install_flow_extension_manifest|math\.add|operator_info\("math/.test(l)) {
      if (/app_id|FlowExtension|install_builtin|sync_host|manifest|math\.add|flow-play|procedural3d/.test(l))
        console.log(`${p}:${i+1}:${l.trim().slice(0,180)}`);
    }
  });
}
// read BIM guest section current
const bim = files.find(p=>p.includes("bim") && p.endsWith("component.rs") && p.includes("extensions"));
console.log("\n=== BIM TAIL ===");
console.log(fs.readFileSync(bim,"utf8").split(/\n/).slice(-120).join("\n"));
// read text extension fully as template for wrapping
const text = files.find(p=>p.includes("/text/") && p.includes("extensions") && p.includes("framework") && p.endsWith("component.rs"));
console.log("\nTEXT PATH", text);
console.log(fs.readFileSync(text,"utf8"));
