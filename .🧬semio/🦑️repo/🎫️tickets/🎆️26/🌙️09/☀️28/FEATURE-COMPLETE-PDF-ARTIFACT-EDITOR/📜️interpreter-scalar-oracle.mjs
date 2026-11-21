/** 🧮 Run the same authored scalar WebAssembly operations in the host WebAssembly engine. */
import fs from "node:fs";
import assert from "node:assert/strict";
const fixture = JSON.parse(fs.readFileSync(new URL("./🔣️interpreter-scalar-ledger.json",import.meta.url),"utf8"));
const types = {i32:0x7f,i64:0x7e,f32:0x7d,f64:0x7c};
function section(id, bytes){return [id, bytes.length, ...bytes];}
for(const row of fixture){
    const locals = row.args.flatMap((_,i)=>[0x20,i]);
    const instructions = [0,...locals,row.op,0x0b];
    const module = Buffer.from([
        0,97,115,109,1,0,0,0,
        ...section(1,[1,96,row.args.length,...row.args.map(type=>types[type]),1,types[row.result]]),
        ...section(3,[1,0]),...section(7,[1,1,102,0,0]),
        ...section(10,[1,instructions.length,...instructions])
    ]);
    const invoke = new WebAssembly.Instance(new WebAssembly.Module(module)).exports.f;
    const values = row.values.map((v,i)=>row.negative?.includes(i)?-0:Number(v));
    if(row.trap) assert.throws(()=>invoke(...values),WebAssembly.RuntimeError,row.label);
    else assert.ok(Object.is(invoke(...values),row.negativeResult?-0:row.expected),row.label);
}
console.log("[DEBUG] Host WebAssembly independently verified",fixture.length,"scalar cases");
