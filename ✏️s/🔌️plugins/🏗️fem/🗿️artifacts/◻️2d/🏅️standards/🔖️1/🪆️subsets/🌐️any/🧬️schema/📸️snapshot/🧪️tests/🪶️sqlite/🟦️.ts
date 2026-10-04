import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import * as snapshot from "../../🟦️.ts";
import * as artifact from "../../../🟦️.ts";
import * as diff from "../../../🔺️diff/🟦️.ts";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import {fem2dSnapshotToSqliteDatabase,fem2dSnapshotFromSqliteDatabase} from "../../🪶️sqlite/🟦️.ts";
import type {FemDof} from "../../🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase,type SqliteValue} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
const bits=(word:string)=>({bits:BigInt("0x"+word)});
function dof(value:string):FemDof{switch(value){case"Tx":case"Ty":case"Tz":case"Rx":case"Ry":case"Rz":return value;default:throw Error("fixture DOF")}}
function state():snapshot.Fem2dSnapshot{
 const s=fixture.document;
 return{
  nodes:s.nodes.map(v=>({id:v.id,x:bits(v.xBits),y:bits(v.yBits)})),
  elements:s.elements.map(v=>{if(v.kind!=="bar"&&v.kind!=="beam")throw Error("fixture element");return{kind:v.kind,id:v.id,start:v.start,end:v.end,materialId:v.materialId,sectionId:v.sectionId}}),
  regions:s.regions.map(v=>({id:v.id,name:v.name,outline:v.outlineBits.map(p=>[bits(p[0]!),bits(p[1]!)]),holes:v.holesBits.map(h=>h.map(p=>[bits(p[0]!),bits(p[1]!)])),thickness:bits(v.thicknessBits),materialId:v.materialId,meshSize:bits(v.meshSizeBits)})),
  materials:s.materials.map(v=>({id:v.id,name:v.name,e:bits(v.eBits),nu:bits(v.nuBits),rho:bits(v.rhoBits)})),
  sections:s.sections.map(v=>({id:v.id,name:v.name,area:bits(v.areaBits),iy:bits(v.iyBits)})),
  supports:s.supports.map(v=>({id:v.id,nodeId:v.nodeId,fixed:v.fixed.map(dof)})),
  loadCases:s.loadCases.map(v=>({id:v.id,name:v.name,selfWeight:v.selfWeight,loads:v.loads.map(load=>{switch(load.kind){case"nodal":return{kind:"nodal",id:load.id,nodeId:load.nodeId!,dof:dof(load.dof!),value:bits(load.valueBits!)};case"memberUdl":return{kind:"memberUdl",id:load.id,elementId:load.elementId!,wx:bits(load.wxBits!),wy:bits(load.wyBits!)};case"area":return{kind:"area",id:load.id,regionId:load.regionId!,pressure:bits(load.pressureBits!)};default:throw Error("fixture load")}})})),
  combinations:s.combinations.map(v=>({id:v.id,name:v.name,terms:v.terms.map(t=>({caseId:t.caseId,factor:bits(t.factorBits)}))})),
  analysis:{modalCount:BigInt(s.analysis.modalCount),bucklingCount:BigInt(s.analysis.bucklingCount),deformationScale:bits(s.analysis.deformationScaleBits)}
 }
}
function setWords(s:snapshot.Fem2dSnapshot,word:string):void{
 s.analysis.deformationScale=bits(word);
 for(const n of s.nodes){n.x=bits(word);n.y=bits(word)}
 for(const m of s.materials){m.e=bits(word);m.nu=bits(word);m.rho=bits(word)}
 for(const section of s.sections){section.area=bits(word);section.iy=bits(word)}
 for(const r of s.regions){r.thickness=bits(word);r.meshSize=bits(word);for(const p of r.outline){p[0]=bits(word);p[1]=bits(word)}for(const hole of r.holes)for(const p of hole){p[0]=bits(word);p[1]=bits(word)}}
 for(const c of s.loadCases)for(const load of c.loads)switch(load.kind){case"nodal":load.value=bits(word);break;case"memberUdl":load.wx=bits(word);load.wy=bits(word);break;case"area":load.pressure=bits(word)}
 for(const combination of s.combinations)for(const t of combination.terms)t.factor=bits(word)
}
function copy(d:SqliteDatabase):{tables:{name:string;sql:string;rows:{rowid:bigint;values:SqliteValue[]}[]}[]}{return{tables:d.tables.map(t=>({...t,rows:t.rows.map(r=>({...r,values:[...r.values]}))}))}}

test("fem2d handwritten semantic SQLite capability is owned by its snapshot", () => {
 expect(Object.hasOwn(snapshot, "fem2dSnapshotToSqliteDatabase")).toBe(true);
 expect(Object.hasOwn(snapshot, "fem2dSnapshotFromSqliteDatabase")).toBe(true);
});

test("fem2d complete literal state and empty holes through independent SQLite edits",async()=>{
 const source=state(),database=await fem2dSnapshotToSqliteDatabase(source),bytes=await exportSqliteDatabase(database),native=Database.deserialize(bytes);
 try{
  expect(await fem2dSnapshotFromSqliteDatabase(await importSqliteDatabase(bytes))).toEqual(source);
  for(const[name,count]of Object.entries(fixture.tableRowCounts))expect(native.query('SELECT COUNT(*) AS count FROM "'+name+'"').get()).toEqual({count});
  expect(native.query("SELECT authored_id FROM fem2d_node ORDER BY ordinal").all()).toEqual([{authored_id:""},{authored_id:""}]);
  expect(native.query("SELECT dof FROM fem2d_support_dof ORDER BY ordinal").all()).toEqual(fixture.document.supports[0]!.fixed.map(dof=>({dof})));
  expect(native.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(native.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
  native.query("UPDATE fem2d_material SET name=? WHERE id=1").run("edited 日本\u0000");
  const edited=await fem2dSnapshotFromSqliteDatabase(await importSqliteDatabase(native.serialize()));expect(edited.materials[0]!.name).toBe("edited 日本\u0000");expect(edited.regions[0]!.holes[0]).toEqual([]);
 }finally{native.close()}
});
test("fem2d every native IEEE word and complete unsigned count widths",async()=>{
 for(const word of fixture.ieee754Words){const source=state();setWords(source,word);const bytes=await exportSqliteDatabase(await fem2dSnapshotToSqliteDatabase(source)),oracle=Database.deserialize(bytes,{safeIntegers:true});try{expect(oracle.query("SELECT deformation_scale_ieee754_bits FROM fem2d_document").get()).toEqual({deformation_scale_ieee754_bits:BigInt.asIntN(64,BigInt("0x"+word))});expect(await fem2dSnapshotFromSqliteDatabase(await importSqliteDatabase(bytes))).toEqual(source);}finally{oracle.close()}}
 for(const word of fixture.countWords){const source=state();source.analysis.modalCount=BigInt(word);source.analysis.bucklingCount=BigInt(word);expect(await fem2dSnapshotFromSqliteDatabase(await importSqliteDatabase(await exportSqliteDatabase(await fem2dSnapshotToSqliteDatabase(source))))).toEqual(source)}
 const source=state();source.analysis.modalCount=18446744073709551616n;await expect(fem2dSnapshotToSqliteDatabase(source)).rejects.toThrow();
});
test("fem2d malformed relationship variants, storage types, words and schema are refused",async()=>{
 const source=await fem2dSnapshotToSqliteDatabase(state());
 for(let n=0;n<10;n++){const d=copy(source),t=(name:string)=>d.tables.find(t=>t.name===name)!;switch(n){case 0:t("fem2d_node").rows[1]!.values[2]=5n;break;case 1:t("fem2d_hole_vertex").rows[0]!.values[1]=999n;break;case 2:t("fem2d_nodal_load").rows=[];break;case 3:t("fem2d_nodal_load").rows[0]!.rowid=2n;t("fem2d_nodal_load").rows[0]!.values[0]=2n;break;case 4:t("fem2d_support_dof").rows[0]!.values[3]="bad";break;case 5:t("fem2d_document").rows[0]!.values[1]="18446744073709551616";break;case 6:t("fem2d_document").rows[0]!.values[2]="01";break;case 7:t("fem2d_node").rows[0]!.values[6]=0x3ff0000000000000n;break;case 8:t("fem2d_node").rows[0]!.values[3]=17n;break;default:t("fem2d_element").sql=t("fem2d_element").sql.replace("TEXT NOT NULL",'TEXT "NOT" "NULL"')}
  await expect(fem2dSnapshotFromSqliteDatabase(d)).rejects.toThrow()
 }
});
test("fem2d genuine borrowed UTF frontiers and cumulative domain bounds",async()=>{
 const source=state(),a=fixture.controlledAdmission;source.materials[0]!.name=a.largeText.repeat(a.repeat);
 for(const phase of["projectSnapshot","reconstructSnapshot"]){const cancel=new AbortController();let interior=false;const options={signal:cancel.signal,onProgress:(p:{phase:string;completed:number;total:number})=>{if(p.phase===phase&&p.total>a.cancelBytes&&p.completed>=a.cancelBytes&&p.completed<p.total){interior=true;cancel.abort()}}};const database=phase==="reconstructSnapshot"?await fem2dSnapshotToSqliteDatabase(source):undefined;await expect(database?fem2dSnapshotFromSqliteDatabase(database,options):fem2dSnapshotToSqliteDatabase(source,options)).rejects.toThrow();expect(interior).toBe(true)}
 await expect(fem2dSnapshotToSqliteDatabase(source,{maxValueBytes:a.smallOwnedBytes})).rejects.toThrow();
 await expect(fem2dSnapshotToSqliteDatabase(state(),{maxRows:1})).rejects.toThrow();
 const cancel=new AbortController();cancel.abort();await expect(fem2dSnapshotToSqliteDatabase(state(),{signal:cancel.signal})).rejects.toThrow();
});

test("fem2d independent SQLite DDL owns relationships and exact scalar columns", async () => {
 const sql = await Bun.file(new URL("../../🪶️sqlite/🗄️.sql", import.meta.url)).text();
 const database = new Database(":memory:");
 try {
  database.exec(sql);
  expect(database.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all()).toEqual(Object.keys(fixture.tableRowCounts).sort().map(name => ({name})));
  database.exec("PRAGMA foreign_keys=ON");
  database.query("INSERT INTO fem2d_document VALUES(1,?,?,50,?,?)").run(fixture.document.analysis.modalCount,fixture.document.analysis.bucklingCount,0x4049000000000000n,"finite");
  database.query("INSERT INTO fem2d_region VALUES(1,1,0,?,?,0.1,?,0.25,?,?,?,?)").run(fixture.document.regions[0]!.id,fixture.document.regions[0]!.name,fixture.document.regions[0]!.materialId,0x3fb999999999999an,"finite",0x3fd0000000000000n,"finite");
  database.query("INSERT INTO fem2d_hole VALUES(1,1,0)").run();
  database.query("INSERT INTO fem2d_hole VALUES(2,1,1)").run();
  expect(database.query("SELECT ordinal FROM fem2d_hole ORDER BY ordinal").all()).toEqual([{ordinal:0},{ordinal:1}]);
  expect(database.query("SELECT buckling_count FROM fem2d_document").get()).toEqual({buckling_count:fixture.document.analysis.bucklingCount});
  expect(database.query("SELECT material_id FROM fem2d_region").get()).toEqual({material_id:"unresolved"});
  expect(database.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(database.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
 } finally {database.close();}
});

for(const word of fixture.ieee754Words)test(`fem2d canonical snapshot artifact and diff exact word ${word}`,async()=>{
  const source=state();setWords(source,word);
  for(const count of fixture.countWords){source.analysis.modalCount=BigInt(count);source.analysis.bucklingCount=BigInt(count);
   expect(snapshot.parseFemAnalysisSettings(source.analysis)).toEqual(source.analysis);
   expect(artifact.parseFemAnalysisSettings(source.analysis)).toEqual(source.analysis);
   expect(artifact.parseFem2dArtifact(source)).toEqual(source);
   const delta=diff.parseFem2dNodesDelta({added:source.nodes,removed:[],patched:[]});
   expect(delta.added).toEqual(source.nodes);
   expect(diff.parseFem2dNodesDelta({added:[],removed:[],patched:source.nodes.map(item=>({id:item.id,item}))}).patched).toEqual(source.nodes.map(item=>({id:item.id,item})));
   expect(await fem2dSnapshotFromSqliteDatabase(await fem2dSnapshotToSqliteDatabase(artifact.parseFem2dArtifact(source)))).toEqual(source);
  }
});
test("fem2d canonical primitives reject numeric coercion",()=>{
 expect(()=>snapshot.parseFemAnalysisSettings({modalCount:1,bucklingCount:1,deformationScale:1})).toThrow();
 expect(()=>artifact.parseFemNode({id:"",x:{bits:0n},y:1})).toThrow();
});
