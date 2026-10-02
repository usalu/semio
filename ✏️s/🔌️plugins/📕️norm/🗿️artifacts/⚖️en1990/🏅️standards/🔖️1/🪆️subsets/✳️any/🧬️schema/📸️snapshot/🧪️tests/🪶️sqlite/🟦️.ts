import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import {Buffer} from "node:buffer";
import Ajv from "ajv";
import schema from "../../🔣️.json";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import controls from "../../🧫️fixtures/🪶️sqlite/🎛️control.json";
import words from "../../🧫️fixtures/🪶️sqlite/🔢️binary64.json";
import widths from "../../🧫️fixtures/🪶️sqlite/🛂️invalid-u8.json";
import * as snapshot from "../../🟦️.ts";
import {EN1990_SQLITE_SCHEMA,en1990SnapshotToSqliteDatabase,en1990SnapshotFromSqliteDatabase,type En1990SqliteSnapshot} from "../../🟦️.ts";
import {binary64,type Binary64} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

function ownedFixture():En1990SqliteSnapshot{
 const f=snapshot.parseEn1990Snapshot(fixture);return{...f,annex:"En",altitudeM:binary64(f.altitudeM),designWorkingLifeYears:binary64(f.designWorkingLifeYears),referencePeriodYears:binary64(f.referencePeriodYears),kFiDeclared:binary64(f.kFiDeclared),betaComputed:binary64(f.betaComputed),permanents:f.permanents.map(a=>({...a,gk:binary64(a.gk)})),variables:f.variables.map(a=>({...a,qk:binary64(a.qk)})),accidentals:f.accidentals.map(a=>({...a,ad:binary64(a.ad)})),seismics:f.seismics.map((a,index)=>({...a,aEk:binary64(a.aEk),importanceClass:(["I","II","III","IV"]as const)[index]!})),members:f.members.map(m=>({...m,rdStr:binary64(m.rdStr),rdGeo:binary64(m.rdGeo),rdEquStab:binary64(m.rdEquStab),rdEquDestab:binary64(m.rdEquDestab),rdFat:binary64(m.rdFat),span:binary64(m.span),deflectionW:binary64(m.deflectionW),deflectionLimitRatio:binary64(m.deflectionLimitRatio),vibrationFrequency:binary64(m.vibrationFrequency),vibrationFrequencyMin:binary64(m.vibrationFrequencyMin)})),bridgeSls:f.bridgeSls.map(b=>({...b,deckAcceleration:binary64(b.deckAcceleration),deckAccelerationLimit:binary64(b.deckAccelerationLimit),deckTwist:binary64(b.deckTwist),deckTwistLimit:binary64(b.deckTwistLimit),bridgeDeflection:binary64(b.bridgeDeflection),bridgeDeflectionLimit:binary64(b.bridgeDeflectionLimit)})),effects:f.effects.map(e=>({...e,influence:binary64(e.influence)}))};
}
function setEveryWord(s:En1990SqliteSnapshot,value:Binary64):void{
 s.altitudeM=value;s.designWorkingLifeYears=value;s.referencePeriodYears=value;s.kFiDeclared=value;s.betaComputed=value;
 for(const a of s.permanents)a.gk=value;for(const a of s.variables)a.qk=value;for(const a of s.accidentals)a.ad=value;for(const a of s.seismics)a.aEk=value;
 for(const m of s.members){m.rdStr=value;m.rdGeo=value;m.rdEquStab=value;m.rdEquDestab=value;m.rdFat=value;m.span=value;m.deflectionW=value;m.deflectionLimitRatio=value;m.vibrationFrequency=value;m.vibrationFrequencyMin=value}
 for(const b of s.bridgeSls){b.deckAcceleration=value;b.deckAccelerationLimit=value;b.deckTwist=value;b.deckTwistLimit=value;b.bridgeDeflection=value;b.bridgeDeflectionLimit=value}
 for(const e of s.effects)e.influence=value;
}
function nativeWord(value:number):bigint{const buffer=Buffer.alloc(8);buffer.writeDoubleBE(value);return buffer.readBigUInt64BE()}

test("EN1990 neutral full input agrees with the independent schema oracle",()=>{
 const ajv=new Ajv({strict:false});expect(ajv.validate(schema,fixture)).toBe(true);expect(fixture).toEqual(snapshot.parseEn1990Snapshot(fixture));
 for(const field of widths.fields){for(const value of widths.invalidValues){const invalid={...fixture,[field]:value};expect(ajv.validate(schema,invalid)).toBe(false);expect(()=>snapshot.parseEn1990Snapshot(invalid)).toThrow()}for(const value of widths.validValues){const valid={...fixture,[field]:value};expect(ajv.validate(schema,valid)).toBe(true);expect(valid).toEqual(snapshot.parseEn1990Snapshot(valid))}}
 expect(ownedFixture().altitudeM.bits).toBe(nativeWord(fixture.altitudeM));
});
test("EN1990 snapshot publicly exposes both owned relational directions",()=>{expect(Object.hasOwn(snapshot,"en1990SnapshotToSqliteDatabase")).toBe(true);expect(Object.hasOwn(snapshot,"en1990SnapshotFromSqliteDatabase")).toBe(true)});

test("EN1990 independent SQLite joins preserve occurrence identities and allow domain edits",async()=>{
 expect(EN1990_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql",import.meta.url)).text());
 const expected=ownedFixture();const database=await en1990SnapshotToSqliteDatabase(expected);expect(await en1990SnapshotFromSqliteDatabase(database)).toEqual(expected);
 const db=Database.deserialize(await exportSqliteDatabase(database));try{
 expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
 expect(db.query("SELECT m.label_de,a.logical_id,v.category,e.influence FROM en1990_effect e JOIN en1990_member m ON m.id=e.resolved_member_id JOIN en1990_action a ON a.id=e.resolved_action_id JOIN en1990_variable_action v ON v.id=a.id").get()).toEqual({label_de:"Träger",logical_id:"Q",category:"office",influence:1.25});
 expect(db.query("SELECT count(*) AS n FROM en1990_effect WHERE resolved_member_id IS NULL AND resolved_action_id IS NULL").get()).toEqual({n:2});
 db.run("UPDATE en1990_variable_action SET qk=42.5,qk_ieee754_bits=?,qk_ieee754_class='finite' WHERE id=3",[BigInt.asIntN(64,nativeWord(42.5))]);
 db.run("UPDATE en1990_member SET label_de='Geändert 世界' WHERE id=1");
 db.run("UPDATE en1990_bridge_sls SET deck_twist=-7.25,deck_twist_ieee754_bits=?,deck_twist_ieee754_class='finite' WHERE id=1",[BigInt.asIntN(64,nativeWord(-7.25))]);
 expected.variables[0]!.qk={bits:nativeWord(42.5)};expected.members[0]!.labelDe="Geändert 世界";expected.bridgeSls[0]!.deckTwist={bits:nativeWord(-7.25)};
 expect(await en1990SnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(expected);
 }finally{db.close()}
});
for(const text of words.ieee754Binary64Bits)test(`EN1990 every native numeric domain retains word ${text}`,async()=>{
 const expected=ownedFixture();const value={bits:BigInt(text)};setEveryWord(expected,value);
 const db=Database.deserialize(await exportSqliteDatabase(await en1990SnapshotToSqliteDatabase(expected)),{safeIntegers:true});try{
 const buffer=Buffer.alloc(8);buffer.writeBigUInt64BE(value.bits);const scalar=buffer.readDoubleBE();
 const result=db.query("SELECT altitude_m,altitude_m_ieee754_bits,altitude_m_ieee754_class FROM en1990_document").get()as{altitude_m:number|null;altitude_m_ieee754_bits:number|bigint;altitude_m_ieee754_class:string};
 const exact=db.query("SELECT altitude_m_ieee754_bits FROM en1990_document").get()as{altitude_m_ieee754_bits:bigint};expect(BigInt.asUintN(64,exact.altitude_m_ieee754_bits)).toBe(value.bits);expect(result.altitude_m).toBe(Number.isNaN(scalar)?null:scalar);
 expect(await en1990SnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(expected);
 }finally{db.close()}
});

test("EN1990 all empty collections retain nine independently visible domain tables",async()=>{
 const expected:En1990SqliteSnapshot={...ownedFixture(),annex:"De",projectId:"",permanents:[],variables:[],accidentals:[],seismics:[],members:[],bridgeSls:[],effects:[]};
 const database=await en1990SnapshotToSqliteDatabase(expected);expect(database.tables.length).toBe(9);expect(await en1990SnapshotFromSqliteDatabase(await importSqliteDatabase(await exportSqliteDatabase(database)))).toEqual(expected);
});

test("EN1990 refuses malformed choice coverage ordinals relationships IEEE classes and u8 widths",async()=>{
 const database=await en1990SnapshotToSqliteDatabase(ownedFixture());
 for(const sql of["UPDATE en1990_document SET consequence_class=256","UPDATE en1990_document SET annex='unknown'","UPDATE en1990_action SET kind='unknown' WHERE id=1","UPDATE en1990_action SET document_id=99 WHERE id=1","UPDATE en1990_action SET ordinal=99 WHERE id=1","UPDATE en1990_action SET ordinal=0 WHERE id=2","DELETE FROM en1990_seismic_action WHERE id=5","UPDATE en1990_seismic_action SET importance_class='V' WHERE id=5","UPDATE en1990_member SET ordinal=99 WHERE id=1","UPDATE en1990_bridge_sls SET resolved_member_id=2 WHERE id=1","UPDATE en1990_effect SET resolved_action_id=1 WHERE id=2","UPDATE en1990_effect SET resolved_member_id=NULL WHERE id=1","UPDATE en1990_member SET rd_str_ieee754_class='nan' WHERE id=1","DELETE FROM en1990_document"]){
 const db=Database.deserialize(await exportSqliteDatabase(database));try{db.run("PRAGMA ignore_check_constraints=ON");db.run(sql);await expect(en1990SnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow()}finally{db.close()}}
 await expect(en1990SnapshotToSqliteDatabase(ownedFixture(),{maxRows:1})).rejects.toThrow();await expect(en1990SnapshotFromSqliteDatabase(database,{maxValueBytes:0})).rejects.toThrow();
});

test("EN1990 initial cancellation and large ordered member work are bounded in both directions",async()=>{
 const expected=ownedFixture();const cancelled=new AbortController();cancelled.abort();await expect(en1990SnapshotToSqliteDatabase(expected,{signal:cancelled.signal})).rejects.toHaveProperty("name","AbortError");
 const database=await en1990SnapshotToSqliteDatabase(expected);await expect(en1990SnapshotFromSqliteDatabase(database,{signal:cancelled.signal})).rejects.toHaveProperty("name","AbortError");
 expected.members=Array.from({length:controls.entityCount},()=>({...expected.members[0]!}));
 const large=await en1990SnapshotToSqliteDatabase(expected);
 for(const phase of["projectSnapshot","reconstructSnapshot"]as const){const controller=new AbortController();let reached=false;const options={signal:controller.signal,onProgress:(event:{phase:string;completed:number})=>{if(event.phase===phase&&event.completed>=controls.cancelAt){reached=true;controller.abort()}}};await expect(phase==="projectSnapshot"?en1990SnapshotToSqliteDatabase(expected,options):en1990SnapshotFromSqliteDatabase(large,options)).rejects.toHaveProperty("name","AbortError");expect(reached).toBe(true)}
 expected.projectId="x".repeat(controls.largeTextBytes);await expect(en1990SnapshotToSqliteDatabase(expected,{maxValueBytes:controls.largeTextBytes-1})).rejects.toThrow();
 const controller=new AbortController();let reached=false;await expect(en1990SnapshotToSqliteDatabase(expected,{signal:controller.signal,onProgress:event=>{if(event.phase==="projectSnapshot"&&event.completed>0&&event.total===0){reached=true;controller.abort()}}})).rejects.toHaveProperty("name","AbortError");expect(reached).toBe(true);
});
