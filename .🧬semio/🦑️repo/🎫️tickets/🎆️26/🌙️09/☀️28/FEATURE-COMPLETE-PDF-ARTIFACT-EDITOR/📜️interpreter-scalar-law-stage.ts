/** 🧪 Materializes the candidate Rust files in ticket scratch for standalone scalar linkage. */
import fs from "node:fs";
import assert from "node:assert/strict";
import {dirname,join} from "node:path";
const output=join(import.meta.dir,"🗑️generated","scalar-law");
const files=JSON.parse(fs.readFileSync(join(import.meta.dir,"🗑️generated","interpreter-scalar-overlay.json"),"utf8"));
for(const [name,,body] of files){
    if(!name.includes("🧮️numeric-execution")&&!name.includes("🔢️numeric-semantics"))continue;
    if(!name.endsWith(".rs"))continue;
    const inner=name.split("/🧠️interpreter/")[1];
    assert.ok(inner);
    const p=join(output,inner);
    fs.mkdirSync(dirname(p),{recursive:true});
    fs.writeFileSync(p,body);
}
