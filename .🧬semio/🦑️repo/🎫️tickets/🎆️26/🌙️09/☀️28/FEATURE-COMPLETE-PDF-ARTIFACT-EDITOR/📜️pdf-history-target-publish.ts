/** 🔏 Installs guarded PDF history codec, conflict targets and semantic owner registrations. */
import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {existsSync,readFileSync,mkdirSync,renameSync,statSync,writeFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
const generated=join(import.meta.dir,"🗑️generated");
const source=readFileSync(join(generated,"pdf-history-target-overlay.json"));
const rows=JSON.parse(source.toString()) as [string,string|null,string][];
const bounds=JSON.parse(readFileSync(join(generated,"pdf-history-target-bounds.json"),"utf8")) as {checks:{os:string,admitted:boolean}[],stage:string};
const sha=(data:Buffer|string)=>createHash("sha256").update(data).digest("hex");
const current=(file:string)=>existsSync(file)?sha(readFileSync(file)):null;
assert.equal(bounds.stage,sha(source));
assert.equal(new Set(bounds.checks.map(row=>row.os)).size,4);
assert.ok(bounds.checks.every(row=>row.admitted));
const prepared=rows.map(([file,before,body])=>({file,before,body,after:sha(body)}));
for(const row of prepared){
    assert.ok(resolve(row.file).normalize("NFD").length<=256,row.file);
    assert.ok(current(row.file)===row.before||current(row.file)===row.after,row.file);
}
const areas=join(generated,"history-install");
mkdirSync(areas,{recursive:true});
prepared.sort((a,b)=>Number(a.before!==null)-Number(b.before!==null));
for(const [i,row]of prepared.entries()){
    if(current(row.file)===row.after)continue;
    mkdirSync(dirname(row.file),{recursive:true});
    const next=join(areas,`source-${process.pid}-${i}.rs`);
    writeFileSync(next,row.body,{flag:"wx",mode:existsSync(row.file)?statSync(row.file).mode&0o777:0o644});
    assert.equal(current(row.file),row.before,row.file);
    renameSync(next,row.file);
    assert.equal(current(row.file),row.after);
}
const files=prepared.map(({file,before,after})=>({file,before,after}));
writeFileSync(join(generated,"pdf-history-installed.json"),JSON.stringify({overlay:sha(source),files},null,2)+"\n");
console.log("[DEBUG] Installed",files.length,"exact PDF history source paths");
