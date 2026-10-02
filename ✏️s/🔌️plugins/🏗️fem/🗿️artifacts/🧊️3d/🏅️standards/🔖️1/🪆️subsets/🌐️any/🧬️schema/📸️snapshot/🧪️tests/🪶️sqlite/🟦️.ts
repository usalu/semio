import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import * as snapshot from "../../🟦️.ts";
import * as artifact from "../../../🟦️.ts";
import * as diff from "../../../🔺️diff/🟦️.ts";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import {fem3dSnapshotToSqliteDatabase,fem3dSnapshotFromSqliteDatabase,type Fem3dSqliteSnapshot} from "../../🪶️sqlite/🟦️.ts";
import type {FemDof} from "../../🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase,type SqliteValue} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
const bits=(word:string)=>({bits:BigInt("0x"+word)});
function dof(value:string):FemDof{switch(value){case"Tx":case"Ty":case"Tz":case"Rx":case"Ry":case"Rz":return value;default:throw Error("fixture DOF")}}
function state():Fem3dSqliteSnapshot{
 const s=fixture.document;
 return{
  nodes:s.nodes.map(v=>({id:v.id,x:bits(v.xBits),y:bits(v.yBits),z:bits(v.zBits)})),
  elements:s.elements.map(v=>{const base={id:v.id,start:v.start,end:v.end,materialId:v.materialId,sectionId:v.sectionId};if(v.kind==="bar")return{kind:"bar",...base};if(v.kind==="frame"&&v.rollBits!==undefined)return{kind:"frame",...base,roll:bits(v.rollBits)};throw Error("fixture element")}),
  materials:s.materials.map(v=>({id:v.id,name:v.name,e:bits(v.eBits),g:bits(v.gBits),nu:bits(v.nuBits),rho:bits(v.rhoBits)})),
  sections:s.sections.map(v=>({id:v.id,name:v.name,area:bits(v.areaBits),iy:bits(v.iyBits),iz:bits(v.izBits),j:bits(v.jBits)})),
  solids:s.solids.map(v=>{if(v.axis!=="x"&&v.axis!=="y"&&v.axis!=="z")throw Error("fixture axis");return{id:v.id,name:v.name,outline:v.outlineBits.map(p=>[bits(p[0]!),bits(p[1]!)]),holes:v.holesBits.map(h=>h.map(p=>[bits(p[0]!),bits(p[1]!)])),baseZ:bits(v.baseZBits),height:bits(v.heightBits),layers:BigInt(v.layers),meshSize:bits(v.meshSizeBits),materialId:v.materialId,axis:v.axis}}),
  supports:s.supports.map(v=>({id:v.id,nodeId:v.nodeId,fixed:v.fixed.map(dof)})),
  loadCases:s.loadCases.map(v=>({id:v.id,name:v.name,selfWeight:v.selfWeight,loads:v.loads.map(x=>{if(x.kind==="nodal"&&x.nodeId!==undefined&&x.dof!==undefined&&x.valueBits!==undefined)return{kind:"nodal",id:x.id,nodeId:x.nodeId,dof:dof(x.dof),value:bits(x.valueBits)};if(x.kind==="memberUdl"&&x.elementId!==undefined&&x.wxBits!==undefined&&x.wyBits!==undefined&&x.wzBits!==undefined)return{kind:"memberUdl",id:x.id,elementId:x.elementId,wx:bits(x.wxBits),wy:bits(x.wyBits),wz:bits(x.wzBits)};if(x.kind==="area"&&x.solidId!==undefined&&x.pressureBits!==undefined)return{kind:"area",id:x.id,solidId:x.solidId,pressure:bits(x.pressureBits)};throw Error("fixture load")})})),
  combinations:s.combinations.map(v=>({id:v.id,name:v.name,terms:new Map(v.terms.map(x=>[x.caseId,bits(x.factorBits)]))})),
  analysis:{modalCount:BigInt(s.analysis.modalCount),bucklingCount:BigInt(s.analysis.bucklingCount),deformationScale:bits(s.analysis.deformationScaleBits)}
 }
}
function everyWord(s:Fem3dSqliteSnapshot,word:string):void{
 const v=bits(word);s.analysis.deformationScale=v;for(const n of s.nodes){n.x=v;n.y=v;n.z=v}for(const e of s.elements)if(e.kind==="frame")e.roll=v;
 for(const m of s.materials){m.e=v;m.g=v;m.nu=v;m.rho=v}for(const x of s.sections){x.area=v;x.iy=v;x.iz=v;x.j=v}
 for(const x of s.solids){x.baseZ=v;x.height=v;x.meshSize=v;for(const p of x.outline){p[0]=v;p[1]=v}for(const h of x.holes)for(const p of h){p[0]=v;p[1]=v}}
 for(const c of s.loadCases)for(const x of c.loads){switch(x.kind){case"nodal":x.value=v;break;case"memberUdl":x.wx=v;x.wy=v;x.wz=v;break;case"area":x.pressure=v;break}}
 for(const c of s.combinations)for(const key of c.terms.keys())c.terms.set(key,v)
}
function copy(d:SqliteDatabase):{tables:{name:string;sql:string;rows:{rowid:bigint;values:SqliteValue[]}[]}[]}{return{tables:d.tables.map(t=>({name:t.name,sql:t.sql,rows:t.rows.map(r=>({rowid:r.rowid,values:[...r.values]}))}))}}

test("fem3d handwritten semantic SQLite capability is owned by its snapshot", () => {
 expect(Object.hasOwn(snapshot, "fem3dSnapshotToSqliteDatabase")).toBe(true);
 expect(Object.hasOwn(snapshot, "fem3dSnapshotFromSqliteDatabase")).toBe(true);
});

test("fem3d complete independent file edits preserve literal references, empty holes and UTF-8 map order",async()=>{
 const source=state(),d=await fem3dSnapshotToSqliteDatabase(source),bytes=await exportSqliteDatabase(d);
 expect(await fem3dSnapshotFromSqliteDatabase(await importSqliteDatabase(bytes))).toEqual(source);
 const native=Database.deserialize(bytes);try{
  for(const [table,count] of Object.entries(fixture.tableRowCounts))expect(native.query(`SELECT COUNT(*) AS count FROM ${table}`).get()).toEqual({count});
  expect(native.query("SELECT case_id FROM fem3d_combination_term ORDER BY ordinal").all()).toEqual([{case_id:"\ue000"},{case_id:"🧬"}]);
  expect(native.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(native.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
  native.query("UPDATE fem3d_solid SET name=? WHERE id=1").run("edited 日本\u0000");const restored=await fem3dSnapshotFromSqliteDatabase(await importSqliteDatabase(native.serialize()));expect(restored.solids[0]!.name).toBe("edited 日本\u0000");expect(restored.solids[0]!.holes[0]).toEqual([])
 }finally{native.close()}
});
for(const word of fixture.ieee754Words)test(`fem3d complete scalar word ${word}`,async()=>{
 const source=state();everyWord(source,word);const d=await fem3dSnapshotToSqliteDatabase(source),bytes=await exportSqliteDatabase(d),oracle=Database.deserialize(bytes,{safeIntegers:true});try{expect(oracle.query("SELECT roll_ieee754_bits FROM fem3d_frame").get()).toEqual({roll_ieee754_bits:BigInt.asIntN(64,BigInt("0x"+word))});expect(await fem3dSnapshotFromSqliteDatabase(await importSqliteDatabase(bytes))).toEqual(source)}finally{oracle.close()}
});
for(const word of fixture.countWords)test(`fem3d complete unsigned count ${word}`,async()=>{
 const source=state();source.analysis.modalCount=BigInt(word);source.analysis.bucklingCount=BigInt(word);source.solids[0]!.layers=BigInt(word);expect(await fem3dSnapshotFromSqliteDatabase(await importSqliteDatabase(await exportSqliteDatabase(await fem3dSnapshotToSqliteDatabase(source))))).toEqual(source)
});
for(const axis of fixture.axisWords)test(`fem3d literal extrusion axis ${axis}`,async()=>{
 if(axis!=="x"&&axis!=="y"&&axis!=="z")throw Error("fixture axis");const source=state();source.solids[0]!.axis=axis;expect(await fem3dSnapshotFromSqliteDatabase(await fem3dSnapshotToSqliteDatabase(source))).toEqual(source)
});
test("fem3d unsigned layer count refuses overflow",async()=>{const source=state();source.solids[0]!.layers=18446744073709551616n;await expect(fem3dSnapshotToSqliteDatabase(source)).rejects.toThrow()});
test("fem3d malformed variant cardinality, ordinals, map keys, widths and schemas refuse",async()=>{
 const source=await fem3dSnapshotToSqliteDatabase(state());for(let n=0;n<12;n++){const d=copy(source),t=(name:string)=>d.tables.find(t=>t.name===name)!;switch(n){case 0:t("fem3d_node").rows[1]!.values[2]=7n;break;case 1:t("fem3d_hole_vertex").rows[0]!.values[1]=999n;break;case 2:t("fem3d_frame").rows=[];break;case 3:t("fem3d_frame").rows[0]!.rowid=1n;t("fem3d_frame").rows[0]!.values[0]=1n;break;case 4:t("fem3d_nodal_load").rows=[];break;case 5:t("fem3d_support_dof").rows[0]!.values[3]="bad";break;case 6:t("fem3d_solid").rows[0]!.values[7]="18446744073709551616";break;case 7:t("fem3d_solid").rows[0]!.values[10]="bad";break;case 8:t("fem3d_combination_term").rows[1]!.values[3]="\ue000";break;case 9:t("fem3d_node").rows[0]!.values[7]=0x3ff0000000000000n;break;case 10:t("fem3d_document").rows[0]!.values[1]="01";break;default:t("fem3d_element").sql=t("fem3d_element").sql.replace("TEXT NOT NULL",'TEXT "NOT" "NULL"')}await expect(fem3dSnapshotFromSqliteDatabase(d)).rejects.toThrow()}
});
test("fem3d real UTF frontiers, collection cancellation and cumulative limits",async()=>{
 const source=state();source.materials[0]!.name=fixture.controlledAdmission.largeText.repeat(fixture.controlledAdmission.repeat);const d=await fem3dSnapshotToSqliteDatabase(source),stages:string[]=[];
 for(const phase of["projectSnapshot","reconstructSnapshot"]){const cancel=new AbortController();const options={signal:cancel.signal,onProgress(p:{phase:string;completed:number;total:number}){if(p.phase===phase&&p.completed>=65536&&p.completed<p.total){stages.push(phase);cancel.abort()}return true}};if(phase==="projectSnapshot")await expect(fem3dSnapshotToSqliteDatabase(source,options)).rejects.toThrow();else await expect(fem3dSnapshotFromSqliteDatabase(d,options)).rejects.toThrow()}
 expect(stages).toEqual(["projectSnapshot","reconstructSnapshot"]);await expect(fem3dSnapshotToSqliteDatabase(source,{maxValueBytes:1024})).rejects.toThrow();await expect(fem3dSnapshotFromSqliteDatabase(d,{maxValueBytes:1024})).rejects.toThrow();await expect(fem3dSnapshotToSqliteDatabase(state(),{maxRows:2})).rejects.toThrow();const initial=new AbortController();initial.abort();await expect(fem3dSnapshotFromSqliteDatabase(d,{signal:initial.signal})).rejects.toThrow();
 const collection=state();collection.nodes=Array.from({length:fixture.controlledAdmission.collectionItems},()=>({id:"",x:bits("0"),y:bits("0"),z:bits("0")}));const interior=new AbortController();await expect(fem3dSnapshotToSqliteDatabase(collection,{signal:interior.signal,onProgress:p=>{if(p.phase==="projectSnapshot"&&p.completed>=fixture.controlledAdmission.cancelAfter&&p.completed<p.total&&p.total<65536)interior.abort()}})).rejects.toThrow()
});

test("fem3d independent SQLite DDL owns the full dimensional scalar and relationship shape", async () => {
 const sql = await Bun.file(new URL("../../🪶️sqlite/🗄️.sql", import.meta.url)).text();
 const database = new Database(":memory:");
 try {
  database.exec(sql);
  expect(database.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all()).toEqual(Object.keys(fixture.tableRowCounts).sort().map(name => ({name})));
  database.exec("PRAGMA foreign_keys=ON");
  database.query("INSERT INTO fem3d_document VALUES(1,?,?,50,?,?)").run(fixture.document.analysis.modalCount,fixture.document.analysis.bucklingCount,0x4049000000000000n,"finite");
  database.query("INSERT INTO fem3d_solid VALUES(1,1,0,?,?,-0,2,?,0.25,?,?,?,'finite',?,'finite',?,'finite')").run(fixture.document.solids[0]!.id,fixture.document.solids[0]!.name,fixture.document.solids[0]!.layers,fixture.document.solids[0]!.materialId,fixture.document.solids[0]!.axis,-9223372036854775808n,0x4000000000000000n,0x3fd0000000000000n);
  database.query("INSERT INTO fem3d_hole VALUES(1,1,0)").run();
  database.query("INSERT INTO fem3d_hole VALUES(2,1,1)").run();
  expect(database.query("SELECT ordinal FROM fem3d_hole ORDER BY ordinal").all()).toEqual([{ordinal:0},{ordinal:1}]);
  expect(database.query("SELECT layers,material_id,axis FROM fem3d_solid").get()).toEqual({layers:fixture.document.solids[0]!.layers,material_id:"unresolved",axis:"x"});
  expect(database.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(database.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
 } finally {database.close();}
});

for(const word of fixture.ieee754Words)test(`fem3d canonical snapshot artifact and diff exact word ${word}`,async()=>{
  const source=state();everyWord(source,word);
  for(const count of fixture.countWords){source.analysis.modalCount=BigInt(count);source.analysis.bucklingCount=BigInt(count);
   expect(snapshot.parseFemAnalysisSettings(source.analysis)).toEqual(source.analysis);
   expect(artifact.parseFemAnalysisSettings(source.analysis)).toEqual(source.analysis);
   expect(artifact.parseFem3dArtifact(source)).toEqual(source);
   const delta=diff.parseFem3dNodesDelta({added:source.nodes,removed:[],patched:[]});
   expect(delta.added).toEqual(source.nodes);
   expect(await fem3dSnapshotFromSqliteDatabase(await fem3dSnapshotToSqliteDatabase(artifact.parseFem3dArtifact(source)))).toEqual(source);
  }
});
test("fem3d canonical primitives reject numeric coercion",()=>{
 expect(()=>snapshot.parseFemAnalysisSettings({modalCount:1,bucklingCount:1,deformationScale:1})).toThrow();
 expect(()=>artifact.parseFemNode({id:"",x:{bits:0n},y:1,z:{bits:0n}})).toThrow();
});
