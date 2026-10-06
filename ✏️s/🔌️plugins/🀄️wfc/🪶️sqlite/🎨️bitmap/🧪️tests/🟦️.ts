/** 🎨️ Closed neutral pixel ownership checked by independent SQLite and binary codecs. */
import {expect} from "bun:test";
import {Database} from "bun:sqlite";
import fixture from "../🧫️fixtures/🔣️.json";
import {exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase} from "../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
type Case=typeof fixture.cases[number];
type Bitmap={width:number;height:number;palette:readonly unknown[];pixels:string};
function gridPrefix(text:string):number[]{
 const alphabet="ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
 const clean=Array.from(new TextEncoder().encode(text)).filter(b=>b!==61&&![9,10,11,12,13,32].includes(b)),out:number[]=[];
 for(let offset=0;offset<clean.length;offset+=4){
  const chunk=clean.slice(offset,offset+4),values=chunk.map(b=>alphabet.indexOf(String.fromCharCode(b)));
  if(values.some(v=>v<0))break;
  let word=0;for(const value of values)word=word*64+value;word*=2**((4-values.length)*6);
  out.push((word>>>16)&255);if(values.length>2)out.push((word>>>8)&255);if(values.length>3)out.push(word&255);
 }return out;
}
/** 🔎️ Tests the full owner without imposing renderer extents or palette bounds as snapshot invariants. */
export async function verifyBitmapSqlite<S>(prefix:string,make:(value:Case)=>S,bitmap:(value:S)=>Bitmap,project:(value:S)=>Promise<SqliteDatabase>,reconstruct:(value:SqliteDatabase)=>Promise<S>):Promise<void>{
 expect(fixture.cases.length).toBe(14);
 expect(fixture.edit).toEqual({ordinal:1,paletteIndex:255});
 expect(new Set(fixture.cases.map(c=>c.id)).size).toBe(fixture.cases.length);
 for(const value of fixture.cases){
  const decoded=Array.from(Buffer.from(value.text,"base64"));
  const canonical=Buffer.from(decoded).toString("base64")===value.text;
  expect(canonical).toBe(value.mode==="indices");expect(gridPrefix(value.text)).toEqual(value.gridPrefix);
  if(canonical)expect(decoded).toEqual(value.indices);
  const source=make(value),native=bitmap(source),db=Database.deserialize(await exportSqliteDatabase(await project(source)));
  try{
   expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
   expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
   expect(db.query(`SELECT storage_kind FROM ${prefix}_bitmap`).get()).toEqual({storage_kind:value.mode});
   expect(db.query(`SELECT ordinal,palette_index FROM ${prefix}_bitmap_pixel ORDER BY ordinal`).all()).toEqual(value.indices.map((palette_index,ordinal)=>({ordinal,palette_index})));
   expect(db.query(`SELECT ordinal,scalar_text FROM ${prefix}_bitmap_literal_scalar ORDER BY ordinal`).all()).toEqual(value.mode==="literal"?Array.from(value.text).map((scalar_text,ordinal)=>({ordinal,scalar_text})):[]);
   expect(await reconstruct(await importSqliteDatabase(db.serialize()))).toEqual(source);
   if(value.id==="three-unknown-index"){
    db.query(`UPDATE ${prefix}_bitmap_pixel SET palette_index=? WHERE ordinal=?`).run(fixture.edit.paletteIndex,fixture.edit.ordinal);
    const restored=await reconstruct(await importSqliteDatabase(db.serialize())),edited=bitmap(restored),expected=[...value.indices];expected[fixture.edit.ordinal]=fixture.edit.paletteIndex;
    expect(edited.pixels).toBe(Buffer.from(expected).toString("base64"));expect(Array.from(Buffer.from(edited.pixels,"base64"))).toEqual(expected);
    expect(edited.width).toBe(native.width);expect(edited.height).toBe(native.height);expect(edited.palette).toEqual(native.palette);
    const independent=db.query(`SELECT palette_index FROM ${prefix}_bitmap_pixel ORDER BY ordinal`).all() as {palette_index:number}[];expect(Buffer.from(independent.map(r=>r.palette_index)).toString("base64")).toBe(edited.pixels);
   }
  }finally{db.close();}
 }
 const valid=await project(make(fixture.cases.find(c=>c.id==="three-unknown-index")!));
 for(const violation of ["orphan","duplicate","branch"]){
  const changed={tables:valid.tables.map(t=>({...t,rows:t.rows.map(r=>({...r,values:[...r.values]}))}))};
  const pixels=changed.tables.find(t=>t.name===prefix+"_bitmap_pixel")!;
  if(violation==="orphan")pixels.rows[0]!.values[1]=999n;
  else if(violation==="duplicate")pixels.rows[1]!.values[2]=0n;
  else changed.tables.find(t=>t.name===prefix+"_bitmap_literal_scalar")!.rows.push({rowid:1n,values:[1n,pixels.rows[0]!.values[1]!,0n,"文"]});
  await expect(reconstruct(changed)).rejects.toThrow();
 }
}
