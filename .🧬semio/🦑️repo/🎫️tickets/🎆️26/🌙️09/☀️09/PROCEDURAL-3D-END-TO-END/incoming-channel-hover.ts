import assert from "node:assert/strict";
import {Database} from "bun:sqlite";
import {readFile,readdir,mkdir,writeFile} from "node:fs/promises";
import {join,resolve} from "node:path";
import {Builder} from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts";

const fixturePath=join(import.meta.dirname,"incoming-channel-hover.json");
const fixture=JSON.parse(await readFile(fixturePath,"utf8"));
const store=new Database(":memory:");
const oracle=store.query(`
select json_extract(p.value,'$.id') as id
from json_each($candidates) p
where exists (
 select 1 from json_each($graph) h where h.value='h:'||json_extract(p.value,'$.id') or h.value='n:'||json_extract(p.value,'$.widget')
) or exists (
 select 1 from json_each($wires) w join json_each($graph) h
 where (h.value='e:'||json_extract(w.value,'$.id') and json_extract(w.value,'$.toId')=json_extract(p.value,'$.id'))
 or ((h.value='e:'||json_extract(w.value,'$.id') or h.value='h:'||json_extract(w.value,'$.toId'))
    and (json_extract(w.value,'$.fromId')=json_extract(p.value,'$.id')
      or (json_extract(w.value,'$.fromPort')='' and json_extract(w.value,'$.from')=json_extract(p.value,'$.widget') and json_extract(p.value,'$.input')=0)))
) or (json_array_length($graph)=0 and $component=json_extract(p.value,'$.id'))
order by p.key`);
for(const row of fixture.cases) {
 const actual=oracle.all({$candidates:JSON.stringify(fixture.candidates),$wires:JSON.stringify(fixture.wires),$graph:JSON.stringify(row.graph),$component:row.component??null}).map(value=>value.id);
 assert.deepEqual(actual,row.expected,row.name);
}
function style(value) {
 const generator=new Builder().style(value);
 for(let step=generator.next();;step=generator.next())if(step.done)return step.value;
}
assert.equal(style({highlighted:true}).highlighted,true);
assert.equal(style({}).highlighted,undefined);
assert.throws(()=>style({highlighted:"true"}));

const output=join(import.meta.dirname,"🗑️generated/incoming-channel-hover");
await mkdir(output,{recursive:true});
const s3=resolve("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any");
const [source,record,harness]=await Promise.all([
 readFile(join(s3,"🧵️preview-eval/🦀️.rs"),"utf8"),
 readFile(resolve("🧰️framework/🔨️modules/🖱️ui/🌳️retained/🌳️tree/🔀️reconcile/🧾️record-routing-and-actions/🦀️.rs"),"utf8"),
 readFile(join(import.meta.dirname,"incoming-channel-hover-native.rs"),"utf8")]);
const drill=(source,start,end)=>{const a=source.indexOf(start),b=source.indexOf(end,a+start.length);assert.ok(a>=0&&b>=0);return source.slice(a,b);};
const entry=join(output,"actual.rs"),binary=join(output,process.platform==="win32"?"proof.exe":"proof");
await writeFile(entry,[
 "#![allow(dead_code)]",
 "#[path="+JSON.stringify(join(s3,"🧬️schema/🔌️graph-ports/🦀️.rs"))+"] mod graph_codec;",
 drill(source,"/// 🎯️ Raw IDs decoded once","/// 🔎️ Projects registered nodes"),
 drill(record,"pub(super) fn record_presence","/// 🎬️ The record's binding").replace("pub(super) fn","fn"),
 harness
].join("\n"));
const cache=resolve(".cache/b/r/m/u/l/p1c4772/prototype/build");
const libs=(await Promise.all((await readdir(cache,{withFileTypes:true})).filter(x=>x.isDirectory()).map(async({name})=>(await readdir(join(cache,name),{withFileTypes:true})).filter(x=>x.isDirectory()).map(({name:id})=>join(cache,name,id,"out"))))).flat();
const pack=join(cache,"semio-framework-pack-json/5262b28961bbf9e7/out/libsemio_framework_pack_json-5262b28961bbf9e7");
const compile=Bun.spawn(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustc","--edition=2024","--extern","semio_framework_pack_json="+pack+".rlib","--extern","semio_framework_pack_json="+pack+".rmeta",...libs.flatMap(path=>["-L","dependency="+path]),entry,"-o",binary],{stdout:"inherit",stderr:"inherit"});
assert.equal(await compile.exited,0);
const witness=Bun.spawn([binary,fixturePath],{stdout:"pipe",stderr:"inherit"});
const projected=JSON.parse(await new Response(witness.stdout).text());
assert.equal(await witness.exited,0);
assert.deepEqual(projected,fixture.cases.map(row=>row.expected));
await writeFile(join(output,"native.json"),JSON.stringify(projected));
console.log("[DEBUG] Canonical native hover, independent direction oracle, and retained style hydration passed");
