/** 🔏 Installs the verified native WGPU Save source owner under guarded live hashes. */
import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,readFileSync,renameSync,statSync,writeFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
type Row = {file:string,source:string,before:string|null,after:string};
const generated=join(import.meta.dir,"🗑️generated");
const opening=process.argv.includes("--open");
const staged=join(generated,"native-wgpu-save",opening?"open":"bridge");
const label=opening?"Open":"Save";
const receipt=readFileSync(join(staged,"receipt.json"));
const rows=JSON.parse(receipt.toString()) as Row[];
const sha=(data:Buffer)=>createHash("sha256").update(data).digest("hex");
const observe=(file:string)=>existsSync(file)?sha(readFileSync(file)):null;
assert.equal(rows.length,opening?12:16);
assert.equal(new Set(rows.map(row=>row.file)).size,rows.length);
const candidates=rows.map(row=>{
    const body=readFileSync(join(staged,row.source));
    assert.ok(resolve(row.file).normalize("NFD").length<=256,row.file);
    assert.equal(sha(body),row.after,row.file);
    const actual=observe(row.file);
    assert.ok(actual===row.before||actual===row.after,`Source preimage moved: ${row.file}`);
    return {...row,body};
});
if(process.argv.includes("--verify")) {
    console.log(`[DEBUG] Verified ${rows.length} native WGPU ${label} production paths`);
    process.exit(0);
}
const scratch=join(generated,"native-wgpu-save","publish");
mkdirSync(scratch,{recursive:true});
for(const [i,row] of candidates.sort((a,b)=>Number(a.before!==null)-Number(b.before!==null)).entries()){
    if(observe(row.file)===row.after)continue;
    mkdirSync(dirname(row.file),{recursive:true});
    const temporary=join(scratch,`source-${process.pid}-${i}.rs`);
    const mode=existsSync(row.file)?statSync(row.file).mode&0o777:0o644;
    writeFileSync(temporary,row.body,{flag:"wx",mode});
    assert.equal(observe(row.file),row.before,`Source preimage moved: ${row.file}`);
    renameSync(temporary,row.file);
    assert.equal(observe(row.file),row.after,row.file);
}
writeFileSync(join(generated,opening?"native-wgpu-open-installed.json":"native-wgpu-save-installed.json"),JSON.stringify({receipt:sha(receipt),files:rows},null,2)+"\n");
console.log(`[DEBUG] Installed ${rows.length} verified native WGPU ${label} sources`);
