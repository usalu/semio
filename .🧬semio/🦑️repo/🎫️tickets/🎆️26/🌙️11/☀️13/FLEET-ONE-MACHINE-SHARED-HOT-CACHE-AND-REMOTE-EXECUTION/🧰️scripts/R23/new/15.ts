import Ajv from"ajv";
import{expect,test}from"bun:test";
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};
import fixture from"./🔣️.json"with{type:"json"};
import{hostOfRootV1,localRecordV1,neutralRecordV1,type ObservationRowV1,type PortableRecordV1}from"../../🟦️.ts";

/** 🌍️ Contract on real data: the records of one project as a Windows checkout (C:\semio) and a macOS checkout produced them, stored in the production cache of the fleet, have one neutral form, and the neutral form of one is, spelled for the other machine, the record that machine produced itself. The record of a Windows machine is therefore a record every POSIX machine can take, and the other way round. */
const validate=new Ajv({strict:false}).compile({$ref:"#/definitions/RealFixture",definitions:schema.definitions});
type Side={root:string;record:PortableRecordV1};
const VOLATILE=new Set(["mtimeMs","ctimeMs","ino"]);
const stable=(value:unknown):unknown=>Array.isArray(value)?value.map(stable):value!==null&&typeof value==="object"?Object.fromEntries(Object.entries(value).filter(([key])=>!VOLATILE.has(key)).map(([key,item])=>[key,stable(item)])):value;
const rowsOf=(rows:readonly ObservationRowV1[],skip:(row:ObservationRowV1)=>boolean=()=>false)=>rows.filter(row=>!skip(row)).map(row=>JSON.stringify(stable(row))).sort();
const providersOf=(record:PortableRecordV1)=>[...new Set(record.observations.flatMap(row=>row.kind==="analysis"?[(row.fact as{provider:string}).provider]:[]))];
const generationOf=(record:PortableRecordV1)=>/\.cache[\\/]b[\\/]d[\\/]m[\\/]j[\\/]([0-9a-f]{16,64})/u.exec(JSON.stringify(record.observations).replaceAll("\\\\","\\"))?.[1];
const ancestors=(side:Side)=>(row:ObservationRowV1)=>row.kind==="lstat"&&row.fact==="directory"&&row.path!==side.root;

test("the fixture is valid against the schema",()=>{expect(validate(fixture)).toBe(true);expect(fixture.cases.length).toBeGreaterThanOrEqual(2);});

test("the real records of a Windows and a macOS machine have one neutral form",()=>{
 for(const row of fixture.cases){
  const windows=neutralRecordV1(row.windows.record as PortableRecordV1,hostOfRootV1(row.windows.root)).record,posix=neutralRecordV1(row.posix.record as PortableRecordV1,hostOfRootV1(row.posix.root)).record;
  expect(rowsOf(windows.observations,item=>item.kind==="lstat"&&item.fact==="directory"&&item.path!=="⟦semio-root⟧"),row.name).toEqual(rowsOf(posix.observations,item=>item.kind==="lstat"&&item.fact==="directory"&&item.path!=="⟦semio-root⟧"));
  expect(stable(JSON.parse(windows.image)),row.name).toEqual(stable(JSON.parse(posix.image)));
  expect(windows.digest).toBe(posix.digest);
  expect(JSON.stringify(windows)).not.toContain("C:");
 }
});

test("the neutral record of the Windows machine, spelled for the macOS machine, is the record that machine produced, and the other way round",()=>{
 for(const row of fixture.cases){
  const windows=row.windows as Side,posix=row.posix as Side,winHost=hostOfRootV1(windows.root),posHost=hostOfRootV1(posix.root);
  const toMac=localRecordV1(neutralRecordV1(windows.record,winHost).record,posHost,{providers:providersOf(posix.record),generation:generationOf(posix.record)}).record;
  expect(rowsOf(toMac.observations,ancestors(posix)),row.name).toEqual(rowsOf(posix.record.observations,ancestors(posix)));
  expect(stable(JSON.parse(toMac.image)),row.name).toEqual(stable(JSON.parse(posix.record.image)));
  const toWin=localRecordV1(neutralRecordV1(posix.record,posHost).record,winHost,{providers:providersOf(windows.record),generation:generationOf(windows.record)}).record;
  expect(rowsOf(toWin.observations,ancestors(windows)),row.name).toEqual(rowsOf(windows.record.observations,ancestors(windows)));
  expect(stable(JSON.parse(toWin.image)),row.name).toEqual(stable(JSON.parse(windows.record.image)));
 }
});

test("a record published by one machine and taken by a machine without the dependency generation or the provider keeps rows that cannot hold",()=>{
 const row=fixture.cases[0]!,posix=row.posix as Side,taken=localRecordV1(neutralRecordV1(row.windows.record as PortableRecordV1,hostOfRootV1(row.windows.root)).record,hostOfRootV1(posix.root),{providers:[],generation:undefined}).record;
 expect(JSON.stringify(taken.observations)).toContain("generation-unavailable");
 expect(JSON.stringify(taken.observations)).toContain("⟦analysis-owner⟧");
});
