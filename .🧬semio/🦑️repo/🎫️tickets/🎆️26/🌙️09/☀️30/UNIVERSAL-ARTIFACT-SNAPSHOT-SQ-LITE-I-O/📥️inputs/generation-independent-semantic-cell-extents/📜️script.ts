import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import assert from "node:assert/strict";
import {Database} from "bun:sqlite";
const root="/Users/ueli/Documents/semio";
const dimension=process.argv[2]==="2d"?"2d":"3d";
const family=join(root,"✏️s/🔌️plugins/🌀️procedural"),snapshot=join(family,dimension==="2d"?"🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot":"🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot");
const {proceduralSnapshotFixture}=await import(join(family,"🫀️core/🧬️generation/🪶️sqlite/🧫️fixtures/🟦️.ts"));
const owner=await import(join(snapshot,"🪶️sqlite/🟦️.ts"));
const project=dimension==="2d"?owner.generation2dSnapshotToSqliteDatabase:owner.generation3dSnapshotToSqliteDatabase;
const {exportSqliteDatabase}=await import(join(root,"🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts"));
const corpus=JSON.parse(readFileSync(join(snapshot,"🧫️fixtures/🪶️sqlite/🔣️.json"),"utf8"));
const cases=[{name:"complete-owner",word:null},{name:"camera-positive-zero",word:"0000000000000000"},{name:"camera-negative-zero",word:"8000000000000000"},{name:"camera-minimum-subnormal",word:"0000000000000001"},{name:"camera-minimum-normal",word:"0010000000000000"},{name:"camera-maximum-finite",word:"7fefffffffffffff"},{name:"camera-positive-infinity",word:"7ff0000000000000"},{name:"camera-negative-infinity",word:"fff0000000000000"},{name:"camera-quiet-nan",word:"7ff8000000000001"},{name:"camera-signaling-nan",word:"7ff0000000000001"}];
const result=[];
for(const c of cases){const source=proceduralSnapshotFixture(corpus);if(c.word!==null)source.hostSnapshot.camera.x={bits:BigInt("0x"+c.word)};const owned=await project(source),oracle=Database.deserialize(await exportSqliteDatabase(owned));try{assert.deepEqual(oracle.query("PRAGMA integrity_check").get(),{integrity_check:"ok"});assert.deepEqual(oracle.query("PRAGMA foreign_key_check").all(),[]);let bytes=0,rows=0;const tables=[];for(const table of owned.tables){const actual=oracle.query('SELECT * FROM "'+table.name+'"').all();rows+=actual.length;let tableBytes=0;for(const row of actual)for(const value of Object.values(row))tableBytes+=value===null?0:typeof value==="string"?Buffer.byteLength(value,"utf8"):value instanceof Uint8Array?value.length:8;bytes+=tableBytes;tables.push({name:table.name,rows:actual.length,bytes:tableBytes,columns:oracle.query('PRAGMA table_info("'+table.name+'")').all().map((column:any)=>column.name)})}result.push({...c,bytes,rows,tables});console.log("[DEBUG] Generation"+dimension+" independent complete SQLite cells case="+c.name+" bytes="+bytes+" rows="+rows+" tables="+tables.length)}finally{oracle.close()}}
writeFileSync(join(import.meta.dir,"../../🗑️generated/generation"+(dimension==="2d"?"2d":"")+"-independent-semantic-cell-extents.json"),JSON.stringify(result,null,2)+"\n");
