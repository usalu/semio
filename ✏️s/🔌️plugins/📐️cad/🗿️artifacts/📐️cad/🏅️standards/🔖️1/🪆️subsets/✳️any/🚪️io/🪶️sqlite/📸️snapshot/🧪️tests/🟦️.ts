import artifactReferenceSchema from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🔣️.json";
import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import fixture from"../🧫️fixtures/🔣️.json";
import mediaPricing from"../🧫️fixtures/🎞️media-pricing.json";
import {applyPatch} from "fast-json-patch";

test("CAD retained media source pricing preserves exact UTF-8 bytes and ordered lifecycle nodes",async()=>{
 const source=mediaPricing.prefix.repeat(mediaPricing.repeat)+mediaPricing.suffix;
 const encoded=new TextEncoder().encode(source),db=new Database(":memory:");
 try{expect(db.query("SELECT length(CAST(? AS BLOB)) AS bytes").get(source)).toEqual({bytes:mediaPricing.sourceBytes});}finally{db.close();}
 expect(encoded.length).toBe(mediaPricing.sourceBytes);
 expect(encoded.length).toBeGreaterThan(mediaPricing.closeMaximumBytes);
 expect(Math.ceil(encoded.length/mediaPricing.maximumUnitBytes)).toBe(mediaPricing.maximumPricingSteps);
 let checksum=0n;for(const byte of encoded)checksum=BigInt.asUintN(64,checksum*BigInt(mediaPricing.checksumMultiplier)+BigInt(byte));
 expect(checksum.toString()).toBe(mediaPricing.sourceChecksum);
 const nodes=applyPatch<string[]>([],mediaPricing.orderedNodes.map(value=>({op:"add" as const,path:"/-",value})),true).newDocument;
 expect(nodes).toEqual(["pricing","conversion","publication","close"]);
 expect(new Ajv().compile({type:"integer",minimum:1,maximum:49152})(encoded.length)).toBe(true);
 expect(mediaPricing.refusedBytes).toBe(mediaPricing.maximumSourceBytes+1);
 const {OBJLoader}=await import("three/examples/jsm/loaders/OBJLoader.js");
 const {Mesh}=await import("three");
 const object=new OBJLoader().parse(source);
 const positions:number[]=[];
 object.traverse(node=>{if(node instanceof Mesh){const attribute=node.geometry.getAttribute("position");for(let index=0;index<attribute.count;index++)positions.push(attribute.getX(index),attribute.getY(index),attribute.getZ(index));}});
 expect(positions).toEqual([0,0,0,2,0,0,0,3,0]);
 console.log(`[DEBUG] CAD retained media TextEncoder/SQLite oracle bytes=${encoded.length} checksum=${checksum} nodes=${nodes.join(",")}`);
});

import nativeSchema from"../../../../🧬️schema/📸️snapshot/🔣️.json";
import artifactSchema from"../../../../🧬️schema/🔣️.json";
import childSchema from"../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json";
import ioSchema from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json";
import * as owner from"../../../../🧬️schema/📸️snapshot/🟦️.ts";
import {binary64} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import{exportSqliteDatabase,importSqliteDatabase}from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import type{ArtifactSqliteOptions}from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import * as projection from "../🟦️.ts";
import type {CadSqliteReference as Reference,CadSqliteSnapshot as Snapshot} from "../🟦️.ts";
function snapshot():Snapshot{
 const value=structuredClone(fixture.snapshot);
 return{...owner.parseCadSnapshot(value),referencesByModelDefinitionId:Object.fromEntries(Object.entries(value.referencesByModelDefinitionId).map(([key,rows])=>[key,rows.map(row=>({...row,origin:row.origin.map(binary64)as Reference["origin"],orientation:row.orientation===null?null:row.orientation.map(binary64)as Reference["orientation"],scale:row.scale===null?null:binary64(row.scale),widthWorld:binary64(row.widthWorld),opacity:row.opacity===null?null:binary64(row.opacity)}))]))};
}
const bytes=async(value:Snapshot)=>exportSqliteDatabase(await projection.cadSnapshotToSqliteDatabase(value));
const restore=async(value:Uint8Array,options:ArtifactSqliteOptions={})=>projection.cadSnapshotFromSqliteDatabase(await importSqliteDatabase(value),options);
test("CAD independent neutral schema admits every persisted domain",()=>{const validator=new Ajv({strict:false,validateFormats:false}).addSchema(artifactReferenceSchema).addSchema([ioSchema,childSchema,artifactSchema,nativeSchema]);expect(validator.validate(nativeSchema.$id,fixture.snapshot)).toBe(true);expect(validator.validate(nativeSchema.$id,{...fixture.snapshot,unexpected:true})).toBe(false);expect(fixture["ownership"]["semanticBudget"]).toEqual("completeSqlScalarBytes");expect(fixture["ownership"]["backingBudget"]).toEqual("cumulativeSystemAllocatorRequests");expect(fixture["ownership"]["phases"]).toEqual(["projectSnapshot","reconstructSnapshot"]);expect(fixture["ownership"]["checkpointRows"]).toEqual(256);expect(fixture["ownership"]["refusedBackingBytes"]).toEqual(0);expect(fixture["ownership"]["callerOptions"]).toEqual("capturedOnce");expect(fixture["ownership"]["malformedRefusal"]).toEqual("invalidValue");expect(fixture["tableRows"]["cad_document"]).toEqual(1);expect(fixture["tableRows"]["cad_model_child"]).toEqual(4);expect(fixture["tableRows"]["cad_drawing_child"]).toEqual(3);expect(fixture["tableRows"]["cad_reference_group"]).toEqual(3);expect(fixture["tableRows"]["cad_reference"]).toEqual(4);});
test("CAD independent SQLite declares seven complete domain tables without implicit indices",async()=>{const db=new Database(":memory:");try{db.run(await Bun.file(new URL("../🗄️.sql",import.meta.url)).text());expect(db.query("SELECT count(*) AS n FROM sqlite_schema WHERE type='table'").get()).toEqual({n:7});expect(db.query("PRAGMA table_info(cad_reference)").all().length).toBe(39);expect(db.query("SELECT name FROM sqlite_schema WHERE type='index'").all()).toEqual([])}finally{db.close()}});
test("CAD document parser preserves independent local and target child identities",()=>{expect(owner.parseCadSnapshot(fixture.snapshot) as unknown).toEqual(fixture.snapshot)});
test("CAD public persisted facade exposes both semantic directions",()=>{expect(Object.hasOwn(projection,"cadSnapshotToSqliteDatabase")).toBe(true);expect(Object.hasOwn(projection,"cadSnapshotFromSqliteDatabase")).toBe(true)});
test("CAD complete independently queryable files preserve all twenty-one rows and literal duplicates",async()=>{const expected=snapshot(),data=await bytes(expected),db=Database.deserialize(data,{safeIntegers:true});try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);for(const[name,count]of Object.entries(fixture.tableRows))expect(db.query('SELECT COUNT(*) AS n FROM "'+name+'"').get()).toEqual({n:BigInt(count)});expect(db.query("SELECT child_id,artifact_id FROM cad_model_child WHERE slot='shapeModel'").get()).toEqual({child_id:"shape local\0",artifact_id:"target shape"});expect(db.query("SELECT node_id FROM cad_node ORDER BY ordinal").all()).toEqual([{node_id:""},{node_id:""},{node_id:"node"}]);expect(await restore(data)).toEqual(expected)}finally{db.close()}});
for(const hex of fixture.binary64Words)test("CAD all reference binary64 words "+hex,async()=>{const expected=snapshot(),word={bits:BigInt("0x"+hex)};for(const rows of Object.values(expected.referencesByModelDefinitionId))for(const row of rows){row.origin=[word,word,word];row.orientation=[word,word,word,word];row.scale=word;row.widthWorld=word;row.opacity=word}const data=await bytes(expected),db=Database.deserialize(data,{safeIntegers:true});try{const buffer=Buffer.alloc(8);buffer.writeBigUInt64BE(word.bits);const value=buffer.readDoubleBE(),kind=Number.isNaN(value)?"nan":value===Infinity?"positiveInfinity":value===-Infinity?"negativeInfinity":"finite";expect(db.query("SELECT origin_x_bits,origin_x_class,orientation_x_bits,scale_bits,opacity_bits FROM cad_reference ORDER BY id").all()).toEqual(Array.from({length:4},()=>({origin_x_bits:BigInt.asIntN(64,word.bits),origin_x_class:kind,orientation_x_bits:BigInt.asIntN(64,word.bits),scale_bits:BigInt.asIntN(64,word.bits),opacity_bits:BigInt.asIntN(64,word.bits)})));expect(await restore(new Uint8Array(db.serialize()))).toEqual(expected)}finally{db.close()}});
test("CAD empty children, empty literal map groups and absent optional numerics are distinct",async()=>{const expected=snapshot();delete expected.shapeModel;delete expected.buildingModel;delete expected.energyModel;delete expected.structureClassicModel;expected.drawings=[];expected.breps=[];expected.nodes=[];expected.referencesByModelDefinitionId=Object.fromEntries([["",[]],["__proto__",[]]]);expect(await restore(await bytes(expected))).toEqual(expected);expected.referencesByModelDefinitionId={};expect(await restore(await bytes(expected))).toEqual(expected)});
test("CAD independent surrogate renumbering and domain edits retain relationships",async()=>{const expected=snapshot(),db=Database.deserialize(await bytes(expected));try{for(const table of["model_child","drawing_child","brep_child","node"])db.run("UPDATE cad_"+table+" SET id=id+100");db.run("UPDATE cad_reference_group SET id=id+100");db.run("UPDATE cad_reference SET group_id=group_id+100,id=id+100,source_url='edited 世界'");for(const rows of Object.values(expected.referencesByModelDefinitionId))for(const row of rows)row.sourceUrl="edited 世界";expect(await restore(new Uint8Array(db.serialize()))).toEqual(expected)}finally{db.close()}});
for(const[index,sql]of fixture.malformedSql.entries())test("CAD independently malformed semantic edit refuses "+index,async()=>{const db=Database.deserialize(await bytes(snapshot()));try{db.run("PRAGMA ignore_check_constraints=ON");db.run(sql);await expect(restore(new Uint8Array(db.serialize()))).rejects.toThrow()}finally{db.close()}});
test("CAD exact twenty-one-row admission passes and one below is refused",async()=>{const expected=snapshot(),database=await projection.cadSnapshotToSqliteDatabase(expected);expect(database.tables.reduce((n,t)=>n+t.rows.length,0)).toBe(21);await expect(projection.cadSnapshotToSqliteDatabase(expected,{maxRows:21})).resolves.toBeDefined();await expect(projection.cadSnapshotToSqliteDatabase(expected,{maxRows:20})).rejects.toThrow();await expect(projection.cadSnapshotFromSqliteDatabase(database,{maxRows:20})).rejects.toThrow()});
for(const phase of["projectSnapshot","reconstructSnapshot"]as const)test("CAD interior ordered-reference cancellation in "+phase,async()=>{const expected=snapshot(),row=expected.referencesByModelDefinitionId[""]![0]!;expected.referencesByModelDefinitionId[""]=Array.from({length:fixture.control.collectionLength},()=>structuredClone(row));const database=await projection.cadSnapshotToSqliteDatabase(expected),controller=new AbortController();let reached=false;const options:ArtifactSqliteOptions={signal:controller.signal,onProgress:event=>{if(event.phase===phase&&event.completed>=fixture.control.cancelAt&&event.completed<event.total){reached=true;controller.abort()}}};await expect(phase==="projectSnapshot"?projection.cadSnapshotToSqliteDatabase(expected,options):projection.cadSnapshotFromSqliteDatabase(database,options)).rejects.toHaveProperty("kind","canceled");expect(reached).toBe(true);const canceled=new AbortController();canceled.abort();await expect(phase==="projectSnapshot"?projection.cadSnapshotToSqliteDatabase(snapshot(),{signal:canceled.signal}):projection.cadSnapshotFromSqliteDatabase(database,{signal:canceled.signal})).rejects.toHaveProperty("kind","canceled")});
test("CAD lookup workspace is admitted before reconstruction maps",async()=>{const expected=snapshot(),row=expected.referencesByModelDefinitionId[""]![0]!;expected.referencesByModelDefinitionId[""]=Array.from({length:fixture.control.collectionLength},()=>structuredClone(row));const database=await projection.cadSnapshotToSqliteDatabase(expected);expect(database.tables.reduce((n,t)=>n+t.rows.length,0)*512).toBeGreaterThan(fixture.control.maxOwnedBytes);await expect(projection.cadSnapshotFromSqliteDatabase(database,{maxValueBytes:fixture.control.maxOwnedBytes})).rejects.toThrow()});
test("CAD cumulative value and physical file bounds refuse oversized ownership",async()=>{const expected=snapshot(),database=await projection.cadSnapshotToSqliteDatabase(expected);await expect(exportSqliteDatabase(database,{maxFileBytes:fixture.control.tinyFileBytes})).rejects.toThrow();await expect(projection.cadSnapshotFromSqliteDatabase(database,{maxValueBytes:1})).rejects.toThrow();expected.referencesByModelDefinitionId[""]![0]!.sourceUrl="😀".repeat(fixture.control.largeCharacters);await expect(projection.cadSnapshotToSqliteDatabase(expected,{maxValueBytes:fixture.control.maxOwnedBytes})).rejects.toThrow()});
test("CAD independently measured semantic scalar grant passes exactly and refuses one below",async()=>{
 const expected=snapshot(),database=await projection.cadSnapshotToSqliteDatabase(expected),db=Database.deserialize(await exportSqliteDatabase(database),{safeIntegers:true});let semanticBytes=0;
 try{for(const table of database.tables){const columns=db.query('PRAGMA table_info("'+table.name+'")').all() as {name:string}[];for(const row of db.query('SELECT * FROM "'+table.name+'"').all() as Record<string,unknown>[]){for(const column of columns){const value=row[column.name];semanticBytes+=value===null?0:typeof value==="string"?Buffer.byteLength(value,"utf8"):8}}}}finally{db.close()}
 expect(semanticBytes).toBeGreaterThan(0);expect(semanticBytes).toBeLessThan(database.tables.reduce((sum,table)=>sum+table.rows.length,0)*512);
 await expect(projection.cadSnapshotToSqliteDatabase(expected,{maxValueBytes:semanticBytes})).resolves.toEqual(database);
 await expect(projection.cadSnapshotFromSqliteDatabase(database,{maxValueBytes:semanticBytes})).resolves.toEqual(expected);
 await expect(projection.cadSnapshotFromSqliteDatabase(database,{maxValueBytes:semanticBytes-1})).rejects.toHaveProperty("kind","ownershipLimit");
});

for(const phase of fixture.ownership.phases)test("CAD captures caller controls before complete "+phase,async()=>{
 const expected=snapshot(),database=await projection.cadSnapshotToSqliteDatabase(expected);let changed=false;
 const options={maxColumns:1024,maxValueBytes:1_000_000,onProgress:()=>{if(!changed){changed=true;options.maxColumns=1;options.maxValueBytes=0}}};
 await expect(phase==="projectSnapshot"?projection.cadSnapshotToSqliteDatabase(expected,options):projection.cadSnapshotFromSqliteDatabase(database,options)).resolves.toEqual(phase==="projectSnapshot"?database:expected);expect(changed).toBe(true);
});
for(const phase of fixture.ownership.phases)test("CAD neutral typed interior cancellation retains phase "+phase,async()=>{
 const expected=snapshot(),row=expected.referencesByModelDefinitionId[""]![0]!;expected.referencesByModelDefinitionId[""]=Array.from({length:fixture.control.collectionLength},()=>structuredClone(row));const database=await projection.cadSnapshotToSqliteDatabase(expected),controller=new AbortController();let reached=false;
 const options:ArtifactSqliteOptions={signal:controller.signal,onProgress:event=>{if(event.phase===phase&&event.completed>=fixture.ownership.checkpointRows&&event.completed<event.total){reached=true;controller.abort()}}};
 await expect(phase==="projectSnapshot"?projection.cadSnapshotToSqliteDatabase(expected,options):projection.cadSnapshotFromSqliteDatabase(database,options)).rejects.toHaveProperty("kind","canceled");expect(reached).toBe(true);
});

test("CAD independently edited owned domain corpus preserves typed refusal authority",async()=>{
 const original=await bytes(snapshot());
 for(const sql of fixture.malformedSql){const db=Database.deserialize(original);try{db.run("PRAGMA ignore_check_constraints=ON");db.run(sql);await expect(restore(new Uint8Array(db.serialize()))).rejects.toHaveProperty("kind",fixture.ownership.malformedRefusal)}finally{db.close()}}
});

test("CAD literal reference-index contract is closed and independently orders UTF-8 keys",()=>{
 
 expect(fixture.referenceIndex.nativeMapShape).toBe("mapOfOrderedReferences");
 expect(fixture.referenceIndex.keyComparison).toBe("literalUtf8Bytes");
 expect(fixture.referenceIndex.duplicateKeyRefusal).toBe("invalidValue");
 expect(fixture.referenceIndex.aliasRole).toBe("localRelationshipOnly");
 const db=new Database(":memory:");try{db.run("CREATE TABLE literal_key(value TEXT NOT NULL)");for(const key of fixture.referenceIndex.keys)db.run("INSERT INTO literal_key VALUES(?)",[key]);expect(db.query("SELECT value FROM literal_key ORDER BY CAST(value AS BLOB)").all()).toEqual(fixture.referenceIndex.orderedKeys.map(value=>({value})))}finally{db.close()}
});
test("CAD actual parent preserves numeric-looking NUL astral and prototype literal groups",async()=>{
 const expected=snapshot(),reference=expected.referencesByModelDefinitionId[""]![0]!;
 expected.referencesByModelDefinitionId=Object.fromEntries(fixture.referenceIndex.keys.map(key=>[key,key==="__proto__"?[]:[{...structuredClone(reference),id:key,sourceUrl:key}]]));
 const data=await bytes(expected),db=Database.deserialize(data);try{
  expect(db.query("SELECT model_definition_id AS value FROM cad_reference_group ORDER BY CAST(model_definition_id AS BLOB)").all()).toEqual(fixture.referenceIndex.orderedKeys.map(value=>({value})));
  expect(await restore(new Uint8Array(db.serialize()))).toEqual(expected);
  db.run("UPDATE cad_reference_group SET id=id+200");db.run("UPDATE cad_reference SET group_id=group_id+200,id=id+200");
  expect(await restore(new Uint8Array(db.serialize()))).toEqual(expected);
 }finally{db.close()}
});

test("CAD intrinsic geometry media preserves the declared file payload through independent OBJ parsing",async()=>{
 const policy=(fixture as unknown as {intrinsicGeometry?:unknown}).intrinsicGeometry;
 expect(policy).toEqual({accepted:["structuredText","intrinsicText","intrinsicBytes"],refused:["intrinsicNull","addressedBinary"],childPublication:"insertElement",childSlot:"shapeModel",geometryOwner:"persistedArtifact",geometrySlot:"breps",geometrySubset:"brep",parentOperation:"createBrep",atomicChildren:2,positionCount:6,analyticVertexCount:6,triangleCount:2,bounds:[[0,0,0],[3,3,2]]});
 const {OBJLoader}=await import("three/examples/jsm/loaders/OBJLoader.js");
 const text=await Bun.file(new URL("../../../../../../../../../../../🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧫️fixtures/📦️set-object-applied/⬅️before.obj",import.meta.url)).text();
 const bytes=Buffer.from(text,"utf8");
 expect(new TextDecoder("utf-8",{fatal:true}).decode(bytes)).toBe(text);
 const positions:number[]=[];
 const parsed=new OBJLoader().parse(text);
 parsed.traverse(node=>{const geometry=(node as unknown as {geometry?:{getAttribute(name:string):{array:ArrayLike<number>}}}).geometry;if(geometry)positions.push(...Array.from(geometry.getAttribute("position").array));});
 expect(positions.length/3).toBe(6);
 expect(new Set(Array.from({length:positions.length/3},(_,index)=>positions.slice(index*3,index*3+3).join(","))).size).toBe(6);
 expect(positions.length/9).toBe(2);
 expect([0,1,2].map(axis=>[Math.min(...positions.filter((_,index)=>index%3===axis)),Math.max(...positions.filter((_,index)=>index%3===axis))])).toEqual([[0,3],[0,3],[0,2]]);
});

for (const row of (await import("../🧫️fixtures/🧵️paged-text.json")).default.cases) test("CAD paged UTF-8 census SQLite oracle " + row.id, () => {
 const text = row.segments.join("").repeat(row.repeat), database = new Database(":memory:");
 try { const result = database.query("SELECT length(CAST(? AS BLOB)) AS bytes").get(text) as {bytes:number}; expect(result.bytes).toBe(row.textBytes); expect(new TextEncoder().encode(text).length).toBe(result.bytes); console.log(`[DEBUG] CAD paged UTF-8 SQLite oracle ${row.id} bytes=${result.bytes}`); }
 finally { database.close(); }
});


test("CAD ordered brep children preserve complete topology identities through SQLite",async()=>{
 const expected=snapshot(),data=await bytes(expected),db=Database.deserialize(data,{safeIntegers:true});
 try{expect(db.query("SELECT ordinal,child_id,artifact_id,artifact_kind,standard,subset FROM cad_brep_child ORDER BY ordinal").all()).toEqual(fixture.snapshot.breps.map((child,ordinal)=>({ordinal:BigInt(ordinal),child_id:child.childId,artifact_id:child.target.artifactId,artifact_kind:child.target.dialect.artifactKind,standard:child.target.dialect.standard,subset:child.target.dialect.subset})));expect((await restore(data)).breps).toEqual(expected.breps);console.log("[DEBUG] CAD ordered brep SQLite oracle rows="+expected.breps.length);}
 finally{db.close();}
});
test("CAD brep snapshot and sparse diff admit only declared topology dialects",async()=>{
 const {parseCadDiff}=await import("../../../../🧬️schema/🔺️diff/🟦️.ts");
 expect<unknown>(owner.parseCadSnapshot(fixture.snapshot).breps).toEqual(fixture.snapshot.breps);
 const diff={breps:{values:fixture.snapshot.breps}};expect<unknown>(parseCadDiff(diff)).toEqual(diff);
 for(const subset of ["model","drawing"]){const bad=structuredClone(fixture.snapshot);bad.breps[0]!.target.dialect.subset=subset;expect(()=>owner.parseCadSnapshot(bad)).toThrow();expect(()=>parseCadDiff({breps:{values:bad.breps}})).toThrow();}
 const missing=structuredClone(fixture.snapshot) as Record<string,unknown>;delete missing.breps;expect(()=>owner.parseCadSnapshot(missing)).toThrow();
});
for(const phase of ["projectSnapshot","reconstructSnapshot"] as const)test("CAD interior ordered-brep cancellation in "+phase,async()=>{
 const expected=snapshot();expected.breps=Array.from({length:fixture.control.collectionLength},()=>structuredClone(expected.breps[0]!));
 const database=await projection.cadSnapshotToSqliteDatabase(expected),controller=new AbortController();let reached=false;
 const options:ArtifactSqliteOptions={signal:controller.signal,onProgress:event=>{if(event.phase===phase&&event.completed>=fixture.control.cancelAt&&event.completed<event.total){reached=true;controller.abort()}}};
 await expect(phase==="projectSnapshot"?projection.cadSnapshotToSqliteDatabase(expected,options):projection.cadSnapshotFromSqliteDatabase(database,options)).rejects.toHaveProperty("kind","canceled");expect(reached).toBe(true);
});


test("CAD ordered brep event payload matches independent JSON schema admission",async()=>{
 const createSchema=(await import("../../../../🧬️schema/🧬️mutations/🧊️create-brep/🧬️schema/🔣️.json")).default;
 const deleteSchema=(await import("../../../../🧬️schema/🧬️mutations/🧹delete-brep/🧬️schema/🔣️.json")).default;
 const mutations=await import("../../../../🧬️schema/🧬️mutations/🟦️.ts");
 const ajv=new Ajv({strict:false,validateFormats:false}).addSchema(artifactReferenceSchema).addSchema([createSchema,deleteSchema]);
 const {mutation,...create}=fixture.brepMutation.create;expect(ajv.validate(createSchema.$id,fixture.brepMutation.create)).toBe(true);expect(mutations.parseCreateBrep(create)).toEqual(create);
 const {mutation:deleteName,...remove}=fixture.brepMutation.delete;expect(ajv.validate(deleteSchema.$id,fixture.brepMutation.delete)).toBe(true);expect(mutations.parseDeleteBrep(remove)).toEqual(remove);
 for(const index of fixture.brepMutation.validIndices){const valid={...create,index};expect(ajv.validate(createSchema.$id,{mutation,...valid})).toBe(true);expect(mutations.parseCreateBrep(valid)).toEqual(valid);}
 for(const index of fixture.brepMutation.invalidIndices){const bad={...create,index};expect(ajv.validate(createSchema.$id,{mutation,...bad})).toBe(false);expect(()=>mutations.parseCreateBrep(bad)).toThrow();}
 for(const subset of fixture.brepMutation.invalidSubsets){const bad=structuredClone(create);bad.target.dialect.subset=subset;expect(ajv.validate(createSchema.$id,{mutation,...bad})).toBe(false);expect(()=>mutations.parseCreateBrep(bad)).toThrow();}
 console.log("[DEBUG] CAD ordered brep payload independent AJV admission index="+create.index);
});


test("CAD GraphQL siblings declare neutral persisted topology and canonical event union",async()=>{
 const {parse,Kind,buildASTSchema,validateSchema}=await import("graphql");
 const refs=await Bun.file(new URL("../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🔗️.graphql",import.meta.url)).text(),children=await Bun.file(new URL("../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔗️.graphql",import.meta.url)).text(),artifact=await Bun.file(new URL("../../../../🧬️schema/🔗️.graphql",import.meta.url)).text();
 for(const [path,typeName] of [["../../../../🧬️schema/🔗️.graphql","CadArtifact"],["../../../../🧬️schema/📸️snapshot/🔗️.graphql","CadSnapshot"],["../../../../🧬️schema/🔺️diff/🔗️.graphql","CadDiff"]] as const){
  const document=parse(await Bun.file(new URL(path,import.meta.url)).text()),type=document.definitions.find(def=>def.kind===Kind.OBJECT_TYPE_DEFINITION&&def.name.value===typeName);
  if(!type||type.kind!==Kind.OBJECT_TYPE_DEFINITION)throw new Error("CAD GraphQL type missing "+typeName);
  expect(type.fields?.map(field=>field.name.value)).toEqual(typeName==="CadDiff"?["artifact",...fixture.publicSchemas.fields]:fixture.publicSchemas.fields);
  const source=await Bun.file(new URL(path,import.meta.url)).text(),schema=buildASTSchema(parse(fixture.publicSchemas.graphqlContext+"\n"+refs+"\n"+children+"\n"+(typeName==="CadDiff"?artifact+"\n":"")+source+"\ntype Query { value: "+typeName+" }"));
  expect(validateSchema(schema)).toEqual([]);

 }
 const document=parse(await Bun.file(new URL("../../../../🧬️schema/🧬️mutations/🔗️.graphql",import.meta.url)).text()),union=document.definitions.find(def=>def.kind===Kind.UNION_TYPE_DEFINITION&&def.name.value==="CadMutation");
 if(!union||union.kind!==Kind.UNION_TYPE_DEFINITION)throw new Error("CAD GraphQL mutation union missing");
 expect(union.types?.map(type=>type.name.value)).toEqual(fixture.publicSchemas.mutationTypes);
 const source=await Bun.file(new URL("../../../../🧬️schema/🧬️mutations/🔗️.graphql",import.meta.url)).text(),schema=buildASTSchema(parse(fixture.publicSchemas.graphqlContext+"\n"+refs+"\n"+children+"\n"+artifact+"\n"+source+"\ntype Query { value: CadMutation }"));
 expect(validateSchema(schema)).toEqual([]);
 for(const name of fixture.publicSchemas.mutationTypes){const type=document.definitions.find(def=>def.kind===Kind.OBJECT_TYPE_DEFINITION&&def.name.value===name);if(!type||type.kind!==Kind.OBJECT_TYPE_DEFINITION)throw new Error("CAD GraphQL event missing "+name);expect(type.fields?.map(field=>field.name.value)).toEqual(["mutation",...fixture.publicSchemas.mutationFields[name as keyof typeof fixture.publicSchemas.mutationFields]]);}

 console.log("[DEBUG] CAD GraphQL independent parser fields="+fixture.publicSchemas.fields.length+" mutationTypes="+fixture.publicSchemas.mutationTypes.length);
});


test("CAD Protobuf siblings resolve canonical imports and exact neutral event tags",async()=>{
 const {Root}=await import("protobufjs"),{fileURLToPath}=await import("node:url"),{resolve}=await import("node:path");
 const workspace=fileURLToPath(new URL("../../../../../../../../../../../../../",import.meta.url)),root=new Root();
 root.resolvePath=(_origin,target)=>resolve(workspace,target);
 const paths=["../../../../🧬️schema/🛰️.proto","../../../../🧬️schema/📸️snapshot/🛰️.proto","../../../../🧬️schema/🔺️diff/🛰️.proto","../../../../🧬️schema/🧬️mutations/🛰️.proto"];
 await root.load(paths.map(path=>fileURLToPath(new URL(path,import.meta.url))));root.resolveAll();
 for(const [name,offset]of [["artifact.CadArtifact",0],["snapshot.CadSnapshot",0],["diff.CadDiff",1]] as const){const type=root.lookupType("semio.s.cad.cad."+name);expect(type.fieldsArray.map(field=>field.name)).toEqual(offset?["artifact",...fixture.publicSchemas.fields]:fixture.publicSchemas.fields);expect(type.fieldsArray.map(field=>field.id)).toEqual(Array.from({length:fixture.publicSchemas.fields.length+offset},(_,index)=>index+1));}
 const mutation=root.lookupType("semio.s.cad.cad.mutation.CadMutation");
 expect(mutation.oneofs.mutation!.oneof).toEqual(fixture.publicSchemas.mutationTypes.map(name=>name[0]!.toLowerCase()+name.slice(1)));
 for(const [index,name]of fixture.publicSchemas.mutationTypes.entries()){const field=mutation.fieldsArray[index]!;expect(field.id).toBe(index+1);expect(field.type).toBe(name);const payload=root.lookupType("semio.s.cad.cad.mutation."+name);expect(payload.fieldsArray.map(field=>field.name)).toEqual(fixture.publicSchemas.mutationFields[name as keyof typeof fixture.publicSchemas.mutationFields]);expect(payload.fieldsArray.map(field=>field.id)).toEqual(Array.from({length:payload.fieldsArray.length},(_,index)=>index+1));}
 expect(root.lookupType("semio.s.cad.cad.mutation.CreateBrep").fields.index!.type).toBe("uint32");
 console.log("[DEBUG] CAD Protobuf independent resolved oracle eventTypes="+mutation.fieldsArray.length+" indexMaximum="+fixture.publicSchemas.indexMaximum);
});
