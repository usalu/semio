import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {Database} from "bun:sqlite";
import assert from "node:assert/strict";
const root="/Users/ueli/Documents/semio";
const path=join(root,"✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot");
const owner=await import(join(path,"🟦️.ts")),sqlite=await import(join(root,"🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts"));
const fixture=JSON.parse(readFileSync(join(path,"🧫️fixtures/🪶️sqlite/🔣️.json"),"utf8"));
const word=(value:number)=>{const b=Buffer.alloc(8);b.writeDoubleBE(value);return{bits:b.readBigUInt64BE()}};
const frame=(v:any)=>({x:word(v.x),y:word(v.y),width:word(v.width),height:word(v.height)});
const cases=[{name:"complete-owner",word:null,mode:"complete"},...fixture.binary64Words.map((word:string)=>({name:"geometry-"+word,word,mode:"complete"})),{name:"absent-options-and-empty-tiles",word:null,mode:"absent"},{name:"present-zero-options-and-empty-tiles",word:null,mode:"zero"}];
const results=[];
for(const c of cases){const original=structuredClone(fixture.snapshot);const value={...original,source:{...original.source,frame:frame(original.source.frame),sourceAspect:word(original.source.sourceAspect)},tiles:original.tiles.map((t:any)=>({...t,crop:frame(t.crop)}))};if(c.word!==null){const f={x:{bits:BigInt("0x"+c.word)},y:{bits:BigInt("0x"+c.word)},width:{bits:BigInt("0x"+c.word)},height:{bits:BigInt("0x"+c.word)}};value.source.frame=f;value.source.sourceAspect={bits:BigInt("0x"+c.word)};value.tiles[0].crop=f;}if(c.mode!=="complete"){value.tiles=[];if(c.mode==="absent"){delete value.source.sourceAspect;delete value.source.pdfPage;}else{value.source.sourceAspect=word(-0);value.source.pdfPage=0;}}
 const d=await owner.presentationSnapshotToSqliteDatabase(value),db=Database.deserialize(await sqlite.exportSqliteDatabase(d));let bytes=0,rows=0;const tables=[];
 try{assert.deepEqual(db.query("PRAGMA integrity_check").get(),{integrity_check:"ok"});assert.deepEqual(db.query("PRAGMA foreign_key_check").all(),[]);for(const t of d.tables){let cost=0;const actual=db.query('SELECT * FROM "'+t.name+'"').all();rows+=actual.length;for(const row of actual)for(const v of Object.values(row))cost+=v===null?0:typeof v==="string"?Buffer.byteLength(v,"utf8"):v instanceof Uint8Array?v.length:8;bytes+=cost;tables.push({name:t.name,rows:actual.length,bytes:cost});}results.push({...c,bytes,rows,tables});console.log("[DEBUG] Presentation independent complete SQL cells "+c.name+" bytes="+bytes+" rows="+rows+" tables="+tables.length);}finally{db.close()}
}
writeFileSync(join(import.meta.dir,"../../🗑️generated/presentation-independent-semantic-cell-extents.json"),JSON.stringify(results,null,2)+"\n");
