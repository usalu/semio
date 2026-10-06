import {Database} from "bun:sqlite";
import {join} from "node:path";
import assert from "node:assert/strict";
const repo="/Users/ueli/Documents/semio";
const root=join(repo,"🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/📸️snapshot");
const io=await import(join(repo,"🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts"));
const ieee=await import(join(repo,"🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts"));
const owner=await import(join(root,"🟦️.ts"));
const fixture=await Bun.file(join(root,"🧫️fixtures/🪶️sqlite/🔣️.json")).json();
function snapshot(){const value=structuredClone(fixture.snapshot);for(const node of value.graph.nodes)for(const name of["x","y","width","height"])node[name]=ieee.binary64(node[name]);for(const parameter of value.parameters)if(parameter.type==="numeric")for(const name of["value","min","max","step"])parameter[name]=parameter[name]===null?null:ieee.binary64(parameter[name]);return value;}
async function extent(value:ReturnType<typeof snapshot>){
 const database=await owner.workflowSnapshotToSqliteDatabase(value);
 const db=Database.deserialize(await io.exportSqliteDatabase(database),{safeIntegers:true});
 try{
  assert.equal(db.query("PRAGMA integrity_check").get().integrity_check,"ok");
  assert.equal(db.query("PRAGMA foreign_key_check").all().length,0);
  const rows=db.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all() as {name:string}[];
  assert.equal(rows.length,16);
  return rows.map(({name})=>{const rows=db.query('SELECT * FROM "'+name+'" ORDER BY id').all();let bytes=0;for(const row of rows)for(const value of Object.values(row))bytes+=value===null?0:typeof value==="string"?Buffer.byteLength(value):8;return{table:name,rows:rows.length,bytes};});
 }finally{db.close();}
}
const cases=[{name:"base",snapshot:snapshot()},...fixture.binary64Words.map((word:string)=>{const value=snapshot();for(const node of value.graph.nodes)for(const name of["x","y","width","height"])node[name]={bits:BigInt("0x"+word)};for(const parameter of value.parameters)if(parameter.type==="numeric")for(const name of["value","min","max","step"])parameter[name]={bits:BigInt("0x"+word)};return{name:word,snapshot:value};})];
const report=[];
for(const value of cases){const tables=await extent(value.snapshot);report.push({name:value.name,tables,bytes:tables.reduce((sum,row)=>sum+row.bytes,0)});console.log("[DEBUG] independent Workflow complete SQL cell extent case="+value.name+" bytes="+report.at(-1)!.bytes+" tables=16");}
const ticket=join(repo,".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O");
await Bun.write(join(ticket,"🗑️generated/workflow-independent-semantic-cell-extents.json"),JSON.stringify(report,null,2)+"\n");
