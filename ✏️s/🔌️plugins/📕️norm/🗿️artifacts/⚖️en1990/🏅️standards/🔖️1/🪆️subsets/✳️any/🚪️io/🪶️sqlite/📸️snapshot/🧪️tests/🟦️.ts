import * as sqliteOwner from "../🟦️.ts";
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import {Buffer} from "node:buffer";
import Ajv from "ajv";
import schema from "../../../../🧬️schema/📸️snapshot/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import controls from "./🧫️fixtures/🎛️control.json";
import words from "./🧫️fixtures/🔢️binary64.json";
import widths from "./🧫️fixtures/🛂️invalid-u8.json";
import * as snapshot from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { EN1990_SQLITE_SCHEMA, en1990SnapshotToSqliteDatabase, en1990SnapshotFromSqliteDatabase, type En1990SqliteSnapshot } from "../🟦️.ts";
import {binary64,type Binary64} from "@semio-tech/framework/value/ieee754";
import {exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework/io/sqlite-snapshot";

function ownedFixture():En1990SqliteSnapshot{
 const f=snapshot.parseEn1990Snapshot(fixture);return{...f,annex:"En",cases:f.cases.map(k=>({...k,numbers:k.numbers.map(n=>({...n,value:binary64(n.value)}))})),altitudeM:binary64(f.altitudeM),designWorkingLifeYears:binary64(f.designWorkingLifeYears),referencePeriodYears:binary64(f.referencePeriodYears),betaComputed:binary64(f.betaComputed),permanents:f.permanents.map(a=>({...a,gk:binary64(a.gk)})),variables:f.variables.map(a=>({...a,qk:binary64(a.qk)})),accidentals:f.accidentals.map(a=>({...a,ad:binary64(a.ad)})),seismics:f.seismics.map((a,index)=>({...a,aEk:binary64(a.aEk),importanceClass:(["I","II","III","IV"]as const)[index]!})),members:f.members.map(m=>({...m,rdStr:binary64(m.rdStr),rdGeo:binary64(m.rdGeo),rdEquStab:binary64(m.rdEquStab),rdEquDestab:binary64(m.rdEquDestab),rdFat:binary64(m.rdFat),span:binary64(m.span),precamber:binary64(m.precamber),deflectionW1:binary64(m.deflectionW1),deflectionW2:binary64(m.deflectionW2),deflectionW3:binary64(m.deflectionW3),deflectionLimitRatio:binary64(m.deflectionLimitRatio),deflectionLimitRatioW3:binary64(m.deflectionLimitRatioW3),slsLimitChar:binary64(m.slsLimitChar),slsLimitFreq:binary64(m.slsLimitFreq),slsLimitQp:binary64(m.slsLimitQp),vibrationFrequency:binary64(m.vibrationFrequency),vibrationFrequencyMin:binary64(m.vibrationFrequencyMin),frameHeight:binary64(m.frameHeight),frameDrift:binary64(m.frameDrift),storeyHeight:binary64(m.storeyHeight),storeyDrift:binary64(m.storeyDrift),materialStrengthK:binary64(m.materialStrengthK),materialConversionFactor:binary64(m.materialConversionFactor),materialPartialFactor:binary64(m.materialPartialFactor),materialDesignStrength:binary64(m.materialDesignStrength),nominalDimension:binary64(m.nominalDimension),dimensionTolerance:binary64(m.dimensionTolerance),designDimension:binary64(m.designDimension),memberAlphaCr:binary64(m.memberAlphaCr),secondOrderAnalysis:binary64(m.secondOrderAnalysis)})),bridgeSls:f.bridgeSls.map(b=>({...b,deckAcceleration:binary64(b.deckAcceleration),deckAccelerationLimit:binary64(b.deckAccelerationLimit),deckTwist:binary64(b.deckTwist),deckTwistLimit:binary64(b.deckTwistLimit),bridgeDeflection:binary64(b.bridgeDeflection),bridgeDeflectionLimit:binary64(b.bridgeDeflectionLimit),trackGauge:binary64(b.trackGauge),lineSpeed:binary64(b.lineSpeed),ballastDepth:binary64(b.ballastDepth),deckMass:binary64(b.deckMass),naturalFrequency:binary64(b.naturalFrequency),endRotation:binary64(b.endRotation),basicWindSpeed:binary64(b.basicWindSpeed),dampingRatio:binary64(b.dampingRatio)})),effects:f.effects.map(e=>({...e,influence:binary64(e.influence)})),fatigueSpectra:f.fatigueSpectra.map(x=>({...x,detailCategory:binary64(x.detailCategory),stressRange:binary64(x.stressRange),cycles:binary64(x.cycles)})),reliabilityAnalyses:f.reliabilityAnalyses.map(x=>({...x,resistanceMean:binary64(x.resistanceMean),resistanceSd:binary64(x.resistanceSd),actionMean:binary64(x.actionMean),actionSd:binary64(x.actionSd)})),testSeries:f.testSeries.map(x=>({...x,mean:binary64(x.mean),stdDev:binary64(x.stdDev),variationKnown:binary64(x.variationKnown),conversionFactor:binary64(x.conversionFactor),required:binary64(x.required),theoryVariation:binary64(x.theoryVariation)})),placements:f.placements.map(x=>({...x,x:binary64(x.x),y:binary64(x.y),z:binary64(x.z),yaw:binary64(x.yaw)})),factorOverrides:f.factorOverrides.map(x=>({...x,value:binary64(x.value)}))};
}
function setEveryWord(s:En1990SqliteSnapshot,value:Binary64):void{
 s.altitudeM=value;s.designWorkingLifeYears=value;s.referencePeriodYears=value;s.betaComputed=value;
 for(const a of s.permanents)a.gk=value;for(const a of s.variables)a.qk=value;for(const a of s.accidentals)a.ad=value;for(const a of s.seismics)a.aEk=value;
 for(const m of s.members){m.rdStr=value;m.rdGeo=value;m.rdEquStab=value;m.rdEquDestab=value;m.rdFat=value;m.span=value;m.precamber=value;m.deflectionW1=value;m.deflectionW2=value;m.deflectionW3=value;m.deflectionLimitRatio=value;m.deflectionLimitRatioW3=value;m.slsLimitChar=value;m.slsLimitFreq=value;m.slsLimitQp=value;m.vibrationFrequency=value;m.vibrationFrequencyMin=value}
 for(const b of s.bridgeSls){b.deckAcceleration=value;b.deckAccelerationLimit=value;b.deckTwist=value;b.deckTwistLimit=value;b.bridgeDeflection=value;b.bridgeDeflectionLimit=value}
 for(const e of s.effects)e.influence=value;
 for(const x of s.fatigueSpectra){x.detailCategory=value;x.stressRange=value;x.cycles=value}
 for(const x of s.reliabilityAnalyses){x.resistanceMean=value;x.resistanceSd=value;x.actionMean=value;x.actionSd=value}
 for(const x of s.testSeries){x.mean=value;x.stdDev=value;x.variationKnown=value;x.conversionFactor=value;x.required=value;x.theoryVariation=value}
 for(const x of s.placements){x.x=value;x.y=value;x.z=value;x.yaw=value}
 for(const x of s.factorOverrides)x.value=value
}
function nativeWord(value:number):bigint{const buffer=Buffer.alloc(8);buffer.writeDoubleBE(value);return buffer.readBigUInt64BE()}

test("EN1990 neutral full input agrees with the independent schema oracle",()=>{
 const ajv=new Ajv({strict:false});expect(ajv.validate(schema,fixture)).toBe(true);expect(fixture).toEqual(snapshot.parseEn1990Snapshot(fixture));
 for(const field of widths.fields){for(const value of widths.invalidValues){const invalid={...fixture,[field]:value};expect(ajv.validate(schema,invalid)).toBe(false);expect(()=>snapshot.parseEn1990Snapshot(invalid)).toThrow()}for(const value of widths.validValues){const valid={...fixture,[field]:value};expect(ajv.validate(schema,valid)).toBe(true);expect(valid).toEqual(snapshot.parseEn1990Snapshot(valid))}}
 expect(ownedFixture().altitudeM.bits).toBe(nativeWord(fixture.altitudeM));
});
test("EN1990 snapshot publicly exposes both owned relational directions",()=>{expect(Object.hasOwn(sqliteOwner,"en1990SnapshotToSqliteDatabase")).toBe(true);expect(Object.hasOwn(sqliteOwner,"en1990SnapshotFromSqliteDatabase")).toBe(true)});

test("EN1990 independent SQLite joins preserve occurrence identities and allow domain edits",async()=>{
 expect(EN1990_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql",import.meta.url)).text());
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

test("EN1990 all empty collections retain twelve independently visible domain tables",async()=>{
 const expected:En1990SqliteSnapshot={...ownedFixture(),annex:"De",projectId:"",permanents:[],variables:[],accidentals:[],seismics:[],members:[],bridgeSls:[],effects:[],fatigueSpectra:[],reliabilityAnalyses:[],testSeries:[],placements:[],cases:[],factorOverrides:[],trafficGroups:[]};
 const database=await en1990SnapshotToSqliteDatabase(expected);expect(database.tables.length).toBe(18);expect(await en1990SnapshotFromSqliteDatabase(await importSqliteDatabase(await exportSqliteDatabase(database)))).toEqual(expected);
});

test("EN1990 refuses malformed choice coverage ordinals relationships IEEE classes and u8 widths",async()=>{
 const database=await en1990SnapshotToSqliteDatabase(ownedFixture());
 for(const sql of["UPDATE en1990_document SET consequence_class=256","UPDATE en1990_document SET annex='unknown'","UPDATE en1990_action SET kind='unknown' WHERE id=1","UPDATE en1990_action SET document_id=99 WHERE id=1","UPDATE en1990_action SET ordinal=99 WHERE id=1","UPDATE en1990_action SET ordinal=0 WHERE id=2","DELETE FROM en1990_seismic_action WHERE id=5","UPDATE en1990_seismic_action SET importance_class='V' WHERE id=5","UPDATE en1990_member SET ordinal=99 WHERE id=1","UPDATE en1990_bridge_sls SET resolved_member_id=2 WHERE id=1","UPDATE en1990_effect SET resolved_action_id=1 WHERE id=2","UPDATE en1990_effect SET resolved_member_id=NULL WHERE id=1","UPDATE en1990_member SET rd_str_ieee754_class='nan' WHERE id=1","UPDATE en1990_fatigue_spectrum SET resolved_member_id=2 WHERE id=1","UPDATE en1990_fatigue_spectrum SET ordinal=99 WHERE id=1","UPDATE en1990_fatigue_spectrum SET detail_category_ieee754_class='nan' WHERE id=1","UPDATE en1990_reliability_analysis SET ordinal=99 WHERE id=1","UPDATE en1990_test_series SET ordinal=99 WHERE id=1","UPDATE en1990_test_series SET sample_size=-1 WHERE id=1","DELETE FROM en1990_document"]){
 const db=Database.deserialize(await exportSqliteDatabase(database));try{db.run("PRAGMA ignore_check_constraints=ON");db.run(sql);await expect(en1990SnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow()}finally{db.close()}}
 await expect(en1990SnapshotToSqliteDatabase(ownedFixture(),{maxRows:1})).rejects.toThrow();await expect(en1990SnapshotFromSqliteDatabase(database,{maxValueBytes:0})).rejects.toThrow();
});

test("EN1990 initial cancellation and large ordered member work are bounded in both directions",async()=>{
 const expected=ownedFixture();const cancelled=new AbortController();cancelled.abort();await expect(en1990SnapshotToSqliteDatabase(expected,{signal:cancelled.signal})).rejects.toHaveProperty("kind","canceled");
 const database=await en1990SnapshotToSqliteDatabase(expected);await expect(en1990SnapshotFromSqliteDatabase(database,{signal:cancelled.signal})).rejects.toHaveProperty("kind","canceled");
 expected.members=Array.from({length:controls.entityCount},()=>({...expected.members[0]!}));
 const large=await en1990SnapshotToSqliteDatabase(expected);
 for(const phase of["projectSnapshot","reconstructSnapshot"]as const){const controller=new AbortController();let reached=false;const options={signal:controller.signal,onProgress:(event:{phase:string;completed:number})=>{if(event.phase===phase&&event.completed>=controls.cancelAt){reached=true;controller.abort()}}};await expect(phase==="projectSnapshot"?en1990SnapshotToSqliteDatabase(expected,options):en1990SnapshotFromSqliteDatabase(large,options)).rejects.toHaveProperty("kind","canceled");expect(reached).toBe(true)}
 expected.projectId="x".repeat(controls.largeTextBytes);await expect(en1990SnapshotToSqliteDatabase(expected,{maxValueBytes:controls.largeTextBytes-1})).rejects.toThrow();
 const controller=new AbortController();let reached=false;await expect(en1990SnapshotToSqliteDatabase(expected,{signal:controller.signal,onProgress:event=>{if(event.phase==="projectSnapshot"&&event.completed>0&&event.total===0){reached=true;controller.abort()}}})).rejects.toHaveProperty("kind","canceled");expect(reached).toBe(true);
});

import NormSemanticAjv from "ajv/dist/2020.js";
import normSemanticContract from "./🧫️fixtures/🎛️semantic.json";

import {en1990SnapshotToSqliteDatabase as normSemanticProject,en1990SnapshotFromSqliteDatabase as normSemanticRestore} from "../🟦️.ts";
import {exportSqliteDatabase as normSemanticExport,importSqliteDatabase as normSemanticImport} from "@semio-tech/framework/io/sqlite-snapshot";
import {independentSqliteExtent as normIndependentExtent} from "@semio-tech/framework/io/sqlite-snapshot/testing/semantic-extent";
test("EN1990 complete independent cells preserve copied limits and required empty metadata",async()=>{
 expect(normSemanticContract["schemaBytes"]).toEqual(12697);expect(normSemanticContract["tableWidths"]).toEqual({"en1990_accidental_action":4,"en1990_action":5,"en1990_bridge_sls":48,"en1990_case":7,"en1990_case_number":7,"en1990_case_text":5,"en1990_document":21,"en1990_effect":10,"en1990_factor_override":10,"en1990_fatigue_spectrum":15,"en1990_member":96,"en1990_permanent_action":5,"en1990_placement":16,"en1990_reliability_analysis":19,"en1990_seismic_action":5,"en1990_test_series":26,"en1990_traffic_group":6,"en1990_variable_action":6});expect(normSemanticContract["cases"]).toEqual([{"id":"full","rows":31,"valueBytes":4401},{"id":"emptyCollections","rows":1,"valueBytes":146}]);
 for(const item of normSemanticContract.cases){const source=ownedFixture();if(item.id==="emptyCollections"){source.permanents=[];source.variables=[];source.accidentals=[];source.seismics=[];source.members=[];source.bridgeSls=[];source.effects=[];source.fatigueSpectra=[];source.reliabilityAnalyses=[];source.testSeries=[];source.placements=[];source.factorOverrides=[];source.trafficGroups=[];}const database=await normSemanticProject(source),bytes=await normSemanticExport(database);expect(normIndependentExtent(bytes)).toEqual({rows:item.rows,valueBytes:item.valueBytes,schemaBytes:normSemanticContract.schemaBytes,tableWidths:normSemanticContract.tableWidths});const limits={maxRows:item.rows,maxValueBytes:item.valueBytes,maxSchemaBytes:normSemanticContract.schemaBytes,maxTables:18,maxColumns:96};expect(await normSemanticProject(source,limits)).toEqual(database);expect(await normSemanticRestore(await normSemanticImport(bytes),limits)).toEqual(source);
  for(const short of[{...limits,maxRows:item.rows-1},{...limits,maxValueBytes:item.valueBytes-1},{...limits,maxSchemaBytes:limits.maxSchemaBytes-1},{...limits,maxTables:14},{...limits,maxColumns:56}]){await expect(normSemanticProject(source,short)).rejects.toThrow();await expect(normSemanticRestore(database,short)).rejects.toThrow();}
 }
});

test("EN1990 annex tables keep their rows, exclusive group and member joins and allow domain edits",async()=>{
 const expected=ownedFixture();const database=await en1990SnapshotToSqliteDatabase(expected);
 const db=Database.deserialize(await exportSqliteDatabase(database));try{
 expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
 expect(db.query("SELECT (SELECT count(*) FROM en1990_fatigue_spectrum) AS f,(SELECT count(*) FROM en1990_reliability_analysis) AS r,(SELECT count(*) FROM en1990_test_series) AS t").get()).toEqual({f:3,r:1,t:2});
 expect(db.query("SELECT m.label_de,s.stress_range FROM en1990_fatigue_spectrum s JOIN en1990_member m ON m.id=s.resolved_member_id").all()).toEqual([{label_de:"Träger",stress_range:62500000.25}]);
 expect(db.query("SELECT count(*) AS n FROM en1990_fatigue_spectrum WHERE resolved_member_id IS NULL").get()).toEqual({n:2});
 expect(db.query("SELECT exclusive_group FROM en1990_variable_action").get()).toEqual({exclusive_group:"gr1a 世界"});
 expect(db.query("SELECT action_distribution,resistance_distribution FROM en1990_reliability_analysis").get()).toEqual({action_distribution:"gumbel 世界",resistance_distribution:"lognormal"});
 expect(db.query("SELECT sample_size FROM en1990_test_series ORDER BY ordinal").all()).toEqual([{sample_size:6},{sample_size:4294967295}]);
 db.run("UPDATE en1990_test_series SET required=310.5,required_ieee754_bits=?,required_ieee754_class='finite' WHERE id=1",[BigInt.asIntN(64,nativeWord(310.5))]);
 db.run("UPDATE en1990_reliability_analysis SET resistance_mean=640000,resistance_mean_ieee754_bits=?,resistance_mean_ieee754_class='finite' WHERE id=1",[BigInt.asIntN(64,nativeWord(640000))]);
 expected.testSeries[0]!.required={bits:nativeWord(310.5)};expected.reliabilityAnalyses[0]!.resistanceMean={bits:nativeWord(640000)};
 expect(await en1990SnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(expected);
 }finally{db.close()}
});
