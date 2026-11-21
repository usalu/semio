/** 🚪 Install the independently verified reserved wire settlement changes. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, renameSync, statSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const generated=join(import.meta.dir,"🗑️generated");
const overlay=readFileSync(join(generated,"pdf-open-wire-admission-overlay.json"));
const {data}=JSON.parse(overlay.toString()) as {data:[string,string,string][]};
const checks=JSON.parse(readFileSync(join(generated,"pdf-open-wire-admission-bounds.json"),"utf8")) as {admitted:boolean}[];
const sha=(source:Buffer|string)=>createHash("sha256").update(source).digest("hex");
assert.equal(data.length,2);
assert.equal(checks.length,16);
assert.ok(checks.every(row=>row.admitted));
const files=data.map(([file,before,source])=>({file,before,source,after:sha(source)}));
for(const row of files){
    assert.ok(resolve(row.file).normalize("NFD").length<=256,row.file);
    const actual=sha(readFileSync(row.file));
    assert.ok(actual===row.before||actual===row.after,row.file);
}
if(process.argv.includes("--apply")){
    const scratch=join(generated,"reserved-wire-install");
    mkdirSync(scratch,{recursive:true});
    for(const [index,row] of files.entries()){
        if(sha(readFileSync(row.file))===row.after)continue;
        const staged=join(scratch,`source-${process.pid}-${index}`);
        writeFileSync(staged,row.source,{flag:"wx",mode:statSync(row.file).mode&0o777});
        assert.equal(sha(readFileSync(row.file)),row.before,row.file);
        renameSync(staged,row.file);
        assert.equal(sha(readFileSync(row.file)),row.after,row.file);
    }
    writeFileSync(join(generated,"pdf-open-wire-admission-installed.json"),
        JSON.stringify({overlay:sha(overlay),files:files.map(({file,before,after})=>({file,before,after})),checks},null,2)+"\n");
}
console.log("[DEBUG] Reserved wire source receipt",sha(overlay));
