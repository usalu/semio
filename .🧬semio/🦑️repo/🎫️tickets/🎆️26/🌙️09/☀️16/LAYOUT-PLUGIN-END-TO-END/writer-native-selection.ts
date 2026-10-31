/** 🔎️ Count actual cfg-selected Writer native laws for one original_ batch. */
import {readFile,writeFile} from "node:fs/promises";
import {resolve,dirname,relative} from "node:path";
import assert from "node:assert/strict";
import {readRustSourceClosureV1,selectRustSourceV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const prefix="✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/";
const roots=[prefix+"✏️editor/⌨️typing/🔗️fold",prefix+"🧬️schema/⚙️operations/🧪️tests/🔬️unit"];
const cfg=["target_arch=aarch64","target_os=macos","target_family=unix","unix","test"];
const rows=[];
for(const root of roots){const manifest=resolve(root,"📦️packages/🦀️rust/Cargo.toml"),doc=Bun.TOML.parse(await readFile(manifest,"utf8")) as any;const inputs=await readRustSourceClosureV1([relative(process.cwd(),resolve(dirname(manifest),doc.lib.path))],process.cwd(),cfg,async path=>await readFile(path).catch(error=>error.code==="ENOENT"?null:Promise.reject(error)));const tests=[];for(const input of inputs){if(input.kind!=="source")continue;const text=selectRustSourceV1(await readFile(input.path,"utf8"),cfg);for(const match of text.matchAll(/((?:#\s*\[[^\]]*\]\s*)+)(?:pub\s+)?(?:async\s+)?fn\s+([A-Za-z_][A-Za-z_0-9]*)/g)){if(!/#\s*\[\s*(?:[A-Za-z_][A-Za-z_0-9]*::)*test\b/.test(match[1]!)||!match[2]!.includes("original_"))continue;tests.push({path:input.path,name:match[2]});}}rows.push({package:doc.package.name,sourceFiles:inputs.length,sourceBytes:inputs.reduce((n,input)=>n+input.bytes,0),matches:tests});}
assert.equal(rows[0]!.matches.length,2);assert.equal(rows[1]!.matches.length,1);assert.equal(rows[1]!.matches[0]!.name,"writer_editor_forwards_its_canonical_original_typing_factory");
await writeFile(resolve(import.meta.dirname,"writer-native-selection.md"),"# Combined Writer Native Selection\n\nCfg-selected actual mounted source laws matching original_; no native request launched.\n\n```json\n"+JSON.stringify(rows,null,2)+"\n```\n");
console.log(JSON.stringify(rows));
