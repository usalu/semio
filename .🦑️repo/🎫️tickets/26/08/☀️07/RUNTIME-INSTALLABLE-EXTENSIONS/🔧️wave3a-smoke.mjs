import fs from "fs";
import path from "path";
import { spawnSync } from "child_process";
const root = "/Users/ueli/Documents/semio";
const sDir = fs.readdirSync(root).find((n) => n.startsWith("✏"));
const fw = fs.readdirSync(root).find((n) => n.endsWith("framework"));
const flowFw = path.join(root, fw, "🛍️products", "💻️os", "🔨️modules", "🌊️flow");
const pluginExt = path.join(root, sDir, "🔌️plugins", "🌊️flow",
  fs.readdirSync(path.join(root, sDir, "🔌️plugins", "🌊️flow")).find((n) => n.includes("extensions")));
const SIX = ["core","math","text","logic","dictionary","list"];
const errs = [];
for (const id of SIX) {
  const dir = fs.readdirSync(pluginExt).find((n) => n.endsWith(id));
  if (!dir) errs.push("missing "+id);
  for (const f of ["🦀️component.rs","📦️packages/🦀️rust/Cargo.toml","📦️packages/🦀️rust/📦️glue.rs","📦️packages/🦀️rust/📜️script.ts","📦️packages/🦀️rust/📋️project.json"]) {
    if (!fs.existsSync(path.join(pluginExt, dir, f))) errs.push(id+" missing "+f);
  }
}
const fwExt = fs.readdirSync(path.join(flowFw, fs.readdirSync(flowFw).find((n)=>n.includes("extensions"))));
for (const id of SIX) if (fwExt.some(n=>n.endsWith(id))) errs.push("fw still "+id);
const glue = fs.readFileSync(path.join(flowFw,"📦️packages/🦀️rust/📦️glue.rs"),"utf8");
if (/extensions\/(core|math|text|logic|dictionary|list)/.test(glue)) errs.push("glue still light");
const core = fs.readFileSync(path.join(flowFw, fs.readdirSync(flowFw).find(n=>n.includes("core")&&!n.includes("extensions")), "🦀️component.rs"),"utf8");
const builtin = core.match(/pub fn install_builtin_flow_extensions[\s\S]*?\n\}/)?.[0]||"";
if (/flow_extension_(core|math|text|logic|dictionary|list)::/.test(builtin)) errs.push("builtin still light");
const pluginCargo = fs.readFileSync(path.join(root,sDir,"🔌️plugins/🌊️flow/📦️packages/🦀️rust/Cargo.toml"),"utf8");
if (!pluginCargo.includes('consumes = ["flow.extension"]')) errs.push("no consumes");
const meta = spawnSync("cargo",["metadata","--no-deps","--format-version","1"],{cwd:root,encoding:"utf8",maxBuffer:50e6});
if (meta.status!==0) errs.push("metadata fail "+meta.stderr.slice(0,200));
else {
  const names=new Set(JSON.parse(meta.stdout).packages.map(p=>p.name));
  for (const id of SIX) if (!names.has(`semio-s-plugin-flow-extension-${id}`)) errs.push("meta "+id);
}
console.log(errs.length? "FAIL\n"+errs.join("\n") : "SMOKE OK");
console.log("fwExt", fwExt);
console.log("builtin", builtin.replace(/\s+/g," ").trim());
process.exit(errs.length?1:0);
