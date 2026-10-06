import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {Database} from "bun:sqlite";
import assert from "node:assert/strict";
const root="/Users/ueli/Documents/semio",base=join(root,"✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot");
const owner=await import(join(base,"🟦️.ts")),sqlite=await import(join(root,"🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts"));
const original=JSON.parse(readFileSync(join(base,"🧫️fixtures/🪶️sqlite/🔣️.json"),"utf8")).snapshot;
const fields=["schema","childId","artifactId","artifactKind","standard","subset"],cases=[{name:"complete-owner",field:null,text:null},{name:"empty-all",field:"all",text:""},{name:"unicode-all",field:"all",text:"世界\u0000😀; id!kind@standard/subset%"},...fields.map(field=>({name:"long-"+field,field,text:"😀界\u0000".repeat(17)}))];
const results=[];
for(const c of cases){const s=structuredClone(original);const target=(field:string)=>field==="schema"?s:field==="childId"?s.content:field==="artifactId"?s.content.target:s.content.target.dialect;if(c.field!==null)for(const field of c.field==="all"?fields:[c.field])target(field)[field]=c.text;
 const d=await owner.sequenceSnapshotToSqliteDatabase(s),db=Database.deserialize(await sqlite.exportSqliteDatabase(d));let bytes=0,rows=0;const tables=[];
 try{assert.deepEqual(db.query("PRAGMA integrity_check").get(),{integrity_check:"ok"});assert.deepEqual(db.query("PRAGMA foreign_key_check").all(),[]);for(const t of d.tables){let cost=0;const actual=db.query('SELECT * FROM "'+t.name+'"').all();rows+=actual.length;for(const row of actual)for(const value of Object.values(row))cost+=value===null?0:typeof value==="string"?Buffer.byteLength(value,"utf8"):value instanceof Uint8Array?value.length:8;bytes+=cost;tables.push({name:t.name,rows:actual.length,bytes:cost});}const texts=fields.map(field=>target(field)[field]);assert.equal(bytes,24+texts.reduce((n:number,text:string)=>n+Buffer.byteLength(text,"utf8"),0));results.push({...c,bytes,rows,tables});console.log("[DEBUG] Sequence independent complete SQL cells "+c.name+" bytes="+bytes+" rows="+rows+" tables="+tables.length);}finally{db.close()}
}
writeFileSync(join(import.meta.dir,"../../🗑️generated/sequence-independent-semantic-cell-extents.json"),JSON.stringify(results,null,2)+"\n");
