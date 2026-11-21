/** 📥 Install the verified Original constructor source and its recovery law. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, renameSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const generated=join(import.meta.dir,"🗑️generated");
const evidence=JSON.parse(readFileSync(join(generated,"pdf-open-construction-stage.json"),"utf8")) as {
    data:[string,string|null,string][]; after:string;
    checks:{os:string,test:boolean,admitted:boolean}[];
};
const sha=(body:Buffer|string)=>createHash("sha256").update(body).digest("hex");
const current=(file:string)=>existsSync(file)?sha(readFileSync(file)):null;
assert.equal(evidence.data.length,3);
assert.equal(sha(evidence.data[0][2]),evidence.after);
assert.equal(new Set(evidence.checks.map(row=>row.os+row.test)).size,8);
assert.ok(evidence.checks.every(row=>row.admitted));

const rows=evidence.data.map(([file,before,body])=>({file,before,body,after:sha(body)}));
for(const row of rows){
    assert.ok(resolve(row.file).normalize("NFD").length<=256,row.file);
    assert.ok(current(row.file)===row.before||current(row.file)===row.after,row.file);
}
if(process.argv.includes("--apply")){
    const scratch=join(generated,"constructor-recovery-install");
    mkdirSync(scratch,{recursive:true});
    rows.sort((left,right)=>Number(left.before!==null)-Number(right.before!==null));
    for(const [index,row] of rows.entries()){
        if(current(row.file)===row.after)continue;
        const staged=join(scratch,`source-${process.pid}-${index}`);
        const mode=existsSync(row.file)?statSync(row.file).mode&0o777:0o644;
        writeFileSync(staged,row.body,{flag:"wx",mode});
        assert.equal(current(row.file),row.before,row.file);
        mkdirSync(dirname(row.file),{recursive:true});
        renameSync(staged,row.file);
        assert.equal(current(row.file),row.after,row.file);
    }
    const files=rows.map(({file,before,after})=>({file,before,after}));
    writeFileSync(join(generated,"pdf-open-construction-installed.json"),JSON.stringify({files,checks:evidence.checks},null,2)+"\n");
}
console.log("[DEBUG] Constructor recovery source receipt",evidence.after);
