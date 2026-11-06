import {readFileSync} from "node:fs";
import {isDeepStrictEqual} from "node:util";
import Ajv2020 from "ajv/dist/2020.js";
import {Database} from "bun:sqlite";

const base="🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🪡️field/🟰️change/";
const read=(kind,name)=>JSON.parse(readFileSync(`${base}${kind}/🧪️tests/🧫️fixtures/${name}.json`,"utf8"));
const db=new Database(":memory:");
try{
 for(const kind of ["🎚️record","📋️indexed"]){
  const fixture=read(kind,"🔣️"),validate=new Ajv2020({strict:true}).compile(read(kind,"📐️"));
  if(!validate(fixture))throw Error(JSON.stringify(validate.errors));
  for(const row of fixture.cases){
   const original=JSON.stringify(row.original),authored=JSON.stringify(row.authored);
   let expected;
   if(kind==="🎚️record"){
    expected={};for(const key of ["title","count","body"]){const same=db.query("SELECT json_extract(?1, ?3) IS json_extract(?2, ?3) AS same").get(original,authored,`$.${key}`).same;if(!same)expected[key]=row.authored[key];}
    if(!isDeepStrictEqual(expected,row.patch))throw Error(`record fixture mismatch: ${row.id}`);
   }else{
    const same=db.query("SELECT json(?1) = json(?2) AS same").get(original,authored).same;
    expected=same?null:{removed:db.query("SELECT CAST(key AS INTEGER) AS value FROM json_each(?1) ORDER BY CAST(key AS INTEGER)").all(original).map(x=>x.value),added:db.query("SELECT CAST(key AS INTEGER) AS 'index', value FROM json_each(?1) ORDER BY CAST(key AS INTEGER)").all(authored)};
    if(!isDeepStrictEqual(expected,row.expected))throw Error(`indexed fixture mismatch: ${row.id}`);
   }
  }
  console.log(`${kind}: ${fixture.cases.length} fixture cases agree with SQLite`);
 }
 const birth=read("🎚️record","birth"),birthSchema=read("🎚️record","birth.schema"),birthValidate=new Ajv2020({strict:true}).compile(birthSchema);if(!birthValidate(birth))throw Error(JSON.stringify(birthValidate.errors));for(const row of birth){const expected=db.query("SELECT ? + 34 AS work").get(row.cursorWork).work;if(expected!==row.baseWork)throw Error("record field birth work mismatch");}console.log(birth.length+" record field birth vectors agree with SQLite");
}finally{db.close();}
