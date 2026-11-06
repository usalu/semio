import fs from "node:fs";
import path from "node:path";
import assert from "node:assert/strict";
import {createRequire} from "node:module";
const require=createRequire(import.meta.url),a=require("@iarna/toml"),b=require("smol-toml");
function valid(ref,deps){const match=/^(?:dep:([^/]+)|([^/?]+)\??\/.+)$/.exec(ref);return !match||deps.includes(match[1]||match[2])}
const fixture=JSON.parse(fs.readFileSync(new URL("🧫️feature-refs.json",import.meta.url),"utf8"));for(const c of fixture.cases)assert.equal(valid(c.reference,c.dependencies),c.valid);
const file=path.resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️one-item-owners/📦️packages/🦀️rust/Cargo.toml"),text=fs.readFileSync(file,"utf8"),v=a.parse(text);assert.deepEqual(v,b.parse(text));
const deps=Object.keys({...v.dependencies,...v["dev-dependencies"],...v["build-dependencies"]}), errors=[];for(const [feature,refs] of Object.entries(v.features||{}))for(const ref of refs)if(!valid(ref,deps))errors.push({feature,ref});console.log(JSON.stringify({vectors:fixture.cases.length,parsers:2,errors}));if(errors.length)process.exitCode=1;
