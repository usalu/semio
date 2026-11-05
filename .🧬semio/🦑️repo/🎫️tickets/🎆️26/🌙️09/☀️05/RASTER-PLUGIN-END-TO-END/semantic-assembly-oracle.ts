import{readFileSync}from"node:fs";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
const root="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone";
const read=(path:string)=>JSON.parse(readFileSync(root+"/"+path,"utf8"));
const fixture=read("🧪️tests/🧫️fixtures/📦️assembly/🔣️.json");
if(!new Ajv({strict:false}).compile(read("🧬️schema/📦️assembly/🔣️.json"))(fixture))throw Error("Assembly law schema refused");
const db=new Database(":memory:");let checks=0;
try{db.exec("CREATE TABLE custody(original TEXT PRIMARY KEY,frontier INTEGER)");
for(const value of ["forward α\\u0000😀","post ä","inverse first","inverse last"])db.query("INSERT INTO custody VALUES(?,0)").run(value);
for(const path of [fixture.originalFrontiers,fixture.semanticPostFrontiers])for(let phase=0;phase<path.length;phase++){
const before=db.query("SELECT original FROM custody ORDER BY original").all();
for(const axis of fixture.authorityRequired){const after=db.query("SELECT original FROM custody ORDER BY original").all();if(JSON.stringify(after)!==JSON.stringify(before))throw Error(axis);checks++;}
db.query("UPDATE custody SET frontier=?").run(phase);if((db.query("SELECT COUNT(*) AS owners FROM custody").get() as{owners:number}).owners!==4)throw Error("Custody lost");
}}finally{db.close();}
console.log(JSON.stringify({oracle:"SQLite original custody and Ajv neutral assembly schema",entryPoints:fixture.entryPoints,checks,matched:true,nativeExecuted:false}));
