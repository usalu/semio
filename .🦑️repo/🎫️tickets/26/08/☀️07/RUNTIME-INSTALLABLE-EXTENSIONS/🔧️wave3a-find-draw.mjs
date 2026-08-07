import fs from "fs";
import path from "path";
const root = "/Users/ueli/Documents/semio";
const sDir = fs.readdirSync(root).find((n) => n.startsWith("✏"));
const fw = fs.readdirSync(root).find((n) => n.endsWith("framework"));
function walk(dir, depth=0, acc=[]) {
  if (depth>8) return acc;
  let ents; try{ents=fs.readdirSync(dir,{withFileTypes:true})}catch{return acc}
  for (const ent of ents) {
    if (["target","node_modules","pkg"].includes(ent.name)) continue;
    const p=path.join(dir,ent.name);
    if (ent.isDirectory()) {
      if (ent.name.includes("draw") && p.includes("flow")) acc.push(p);
      walk(p, depth+1, acc);
    }
  }
  return acc;
}
console.log("draw dirs", walk(path.join(root,sDir,"🔌️plugins","🌊️flow")));
console.log("fw flow ext", fs.readdirSync(path.join(root,fw,"🛍️products","💻️os","🔨️modules","🌊️flow",
  fs.readdirSync(path.join(root,fw,"🛍️products","💻️os","🔨️modules","🌊️flow")).find(n=>n.includes("extensions")))));
// search for flow_extension_draw register
function grep(dir, depth=0) {
  if (depth>10) return;
  let ents; try{ents=fs.readdirSync(dir,{withFileTypes:true})}catch{return}
  for (const ent of ents) {
    if (["target","node_modules","pkg",".git"].includes(ent.name)) continue;
    const p=path.join(dir,ent.name);
    if (ent.isDirectory()) grep(p, depth+1);
    else if (ent.name.endsWith(".rs") || ent.name==="Cargo.toml") {
      const t=fs.readFileSync(p,"utf8");
      if (t.includes("flow_extension_draw") || t.includes("flow-extension-draw") || (t.includes("render_scene_json") && p.includes("draw"))) {
        if (p.includes("flow") || p.includes("draw")) console.log(p);
      }
    }
  }
}
grep(path.join(root,sDir,"🔌️plugins","🌊️flow"));
grep(path.join(root,fw,"🛍️products","💻️os","🔨️modules","🌊️flow"));
