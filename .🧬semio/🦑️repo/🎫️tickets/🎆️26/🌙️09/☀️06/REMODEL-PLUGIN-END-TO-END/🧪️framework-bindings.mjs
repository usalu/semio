import fs from "node:fs";
import path from "node:path";
import assert from "node:assert/strict";
import {createRequire} from "node:module";
const require=createRequire(import.meta.url), a=require("@iarna/toml"), b=require("smol-toml");
function read(f){const t=fs.readFileSync(f,"utf8"),v=a.parse(t);assert.deepEqual(v,b.parse(t),f);return v}
const dir=path.resolve("🧰️framework/📦️packages/🦀️rust"), manifest=read(path.join(dir,"Cargo.toml")), source=fs.readFileSync(path.join(dir,"🦀️.rs"),"utf8"), issues=[];
let checked=0,aliases=0;
for(const group of ["dependencies","dev-dependencies"])for(const [key,dep] of Object.entries(manifest[group]||{})){if(!dep||typeof dep!=="object"||!dep.path)continue;const target=path.resolve(dir,dep.path,"Cargo.toml"),owner=read(target).package.name,expected=dep.package||key;checked++;if(dep.package&&dep.package!==key)aliases++;if(owner!==expected)issues.push({group,key,expected,owner,target,canonicalSource:source.includes(owner.replaceAll("-","_"))})}
assert(source.includes("semio_ui_declaration::"));assert(source.includes("semio_ui_locale::"));
console.log(JSON.stringify({checked,aliases,issues},null,2));if(issues.length)process.exitCode=1;
