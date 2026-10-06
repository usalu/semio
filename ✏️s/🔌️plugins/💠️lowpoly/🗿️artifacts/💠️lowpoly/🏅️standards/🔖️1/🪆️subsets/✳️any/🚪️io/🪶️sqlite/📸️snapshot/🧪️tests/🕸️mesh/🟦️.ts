/** 🕸️ Managed half-edge topology and rich mesh channels remain individually editable SQL state. */
import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import fixture from"../../🧫️fixtures/🕸️mesh/🔣️.json";

import{parseLowpolyArtifact}from"../../../../../🧬️schema/🟦️.ts";
import{lowpolySnapshotToSqliteDatabase,lowpolySnapshotFromSqliteDatabase}from"../../🟦️.ts";
import{exportSqliteDatabase,importSqliteDatabase}from"../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import valueSchema from"../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🔣️.json";
import meshSchema from"../../../../../🧬️schema/🕸️mesh/🔣️.json";
import intrinsicSchema from"../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🌳️intrinsic/🔣️.json";
import artifactSchema from"../../../../../🧬️schema/🔣️.json";
import ioSchema from"../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json";
import childSchema from"../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json";
import{decodeLowpolyJsonSnapshot}from"../../../../📝️text/📸️snapshot/🔣️json/🟦️.ts";
type Neutral={kind:string;value?:unknown;items?:Neutral[];members?:{name:string;value:Neutral}[]};
function intrinsic(value:Neutral):unknown{switch(value.kind){case"null":return{kind:value.kind};case"unsigned":case"signed":return{kind:value.kind,value:BigInt(value.value as string)};case"float":return{kind:value.kind,value:{bits:BigInt(`0x${value.value}`)}};case"bytes":return{kind:value.kind,value:Uint8Array.from(value.value as number[])};case"array":return{kind:value.kind,items:value.items!.map(intrinsic)};case"object":return{kind:value.kind,members:value.members!.map(member=>({name:member.name,value:intrinsic(member.value)}))};default:return{kind:value.kind,value:value.value};}}
import{parseLowpolyMeshState}from"../../../../../🧬️schema/🕸️mesh/🟦️.ts";
const word=(value:string)=>({bits:parseInt(value,16)});
function mesh(){return parseLowpolyMeshState({vertices:fixture.mesh.vertices.map(vertex=>({...vertex,position:vertex.position.map(word),normal:vertex.normal?.map(word)??null})),halfedges:fixture.mesh.halfedges.map(edge=>({...edge,uv:edge.uv.map(word)})),faces:fixture.mesh.faces,uvSeams:fixture.mesh.uvSeams,attributes:fixture.mesh.attributes.map(attribute=>({...attribute,values:attribute.values.map(intrinsic)})),materials:fixture.mesh.materials.map(member=>({...member,value:intrinsic(member.value)})),textures:fixture.mesh.textures.map(texture=>({...texture,bytes:Uint8Array.from(texture.bytes)}))});}
test("Lowpoly closed managed-mesh corpus matches independent topology, words and intrinsic octet oracles",()=>{
 expect(fixture["version"]).toEqual(1);
 const extra=structuredClone(fixture)as typeof fixture&{unknown?:boolean};extra.unknown=true;
 const wrong=structuredClone(fixture);wrong.mesh.halfedges[0]!.vertex=4294967296;
 const bad=structuredClone(fixture);bad.mesh.textures[0]!.bytes[0]=256;
 for(const[kind,value]of[["unsigned","18446744073709551616"],["unsigned","-1"],["signed","9223372036854775808"],["signed","-9223372036854775809"]]){const invalid=structuredClone(fixture);invalid.mesh.materials.find(member=>member.value.kind===kind)!.value.value=value;}
 const bytes=new ArrayBuffer(4),view=new DataView(bytes);for(const value of fixture.binary32Words){view.setUint32(0,parseInt(value,16),true);expect(Buffer.from(bytes).readUInt32LE()).toBe(parseInt(value,16));}
 expect(fixture.mesh.materials.map(value=>value.value.kind)).toEqual(["null","boolean","unsigned","signed","float","text","bytes","array","object"]);
 const oracle=new Database(":memory:");try{oracle.exec("CREATE TABLE edge(ordinal INTEGER PRIMARY KEY,vertex INTEGER,next INTEGER,face INTEGER);CREATE TABLE vertex(ordinal INTEGER PRIMARY KEY,x_bits INTEGER,y_bits INTEGER,z_bits INTEGER);");for(const[index,edge]of fixture.mesh.halfedges.entries())oracle.run("INSERT INTO edge VALUES(?,?,?,?)",[index,edge.vertex,edge.next,edge.face]);for(const[index,vertex]of fixture.mesh.vertices.entries())oracle.run("INSERT INTO vertex VALUES(?,?,?,?)",[index,...vertex.position.map(value=>parseInt(value,16))]);expect(oracle.query("SELECT e.ordinal,e.vertex,e.next,n.vertex AS next_vertex,e.face FROM edge e JOIN edge n ON n.ordinal=e.next ORDER BY e.ordinal").all()).toEqual([{ordinal:0,vertex:0,next:1,next_vertex:1,face:0},{ordinal:1,vertex:1,next:2,next_vertex:2,face:0},{ordinal:2,vertex:2,next:0,next_vertex:0,face:0},{ordinal:3,vertex:1,next:3,next_vertex:1,face:null}]);expect(oracle.query("SELECT x_bits,y_bits,z_bits FROM vertex ORDER BY ordinal").all()).toEqual(fixture.mesh.vertices.map(vertex=>({x_bits:parseInt(vertex.position[0]!,16),y_bits:parseInt(vertex.position[1]!,16),z_bits:parseInt(vertex.position[2]!,16)})));}finally{oracle.close();}
});
test("Lowpoly independent named mesh ownership is unique while intrinsic members retain duplicates",()=>{const oracle=new Database(":memory:");try{oracle.exec("CREATE TABLE named(owner INTEGER,name TEXT,value TEXT,UNIQUE(owner,name));CREATE TABLE member(owner INTEGER,ordinal INTEGER,name TEXT,PRIMARY KEY(owner,ordinal));");for(const[owner,rows]of[fixture.mesh.attributes,fixture.mesh.materials,fixture.mesh.textures].entries()){for(const row of rows)oracle.run("INSERT INTO named VALUES(?,?,?)",[owner,row.name,"first"]);expect(()=>oracle.run("INSERT INTO named VALUES(?,?,?)",[owner,rows[0]!.name,"different"])).toThrow();}oracle.exec("INSERT INTO member VALUES(1,0,'same'),(1,1,'same');");expect(oracle.query("SELECT name FROM member ORDER BY ordinal").all()).toEqual([{name:"same"},{name:"same"}]);}finally{oracle.close();}});
test("Lowpoly actual owner exposes complete managed mesh SQL independently of explicit source text",async()=>{
 const vector:[{bits:number},{bits:number},{bits:number}]=[{bits:0},{bits:0},{bits:0}],state=mesh(),expected={schema:"lowpoly.document",objects:[{id:"managed",name:"世界",transform:{position:vector,rotation:vector,scale:vector},smoothShading:false,mesh:null,meshContent:fixture.source,meshState:state,paintLayers:[]}]};
 const database=await lowpolySnapshotToSqliteDatabase(expected),oracle=Database.deserialize(await exportSqliteDatabase(database));try{
 expect(oracle.query("SELECT ordinal,vertex_index,next_index,face_index FROM lowpoly_mesh_halfedge ORDER BY ordinal").all()).toEqual(fixture.mesh.halfedges.map((edge,ordinal)=>({ordinal,vertex_index:edge.vertex,next_index:edge.next,face_index:edge.face})));
 expect(oracle.query("SELECT position_x_ieee754_bits AS x,position_y_ieee754_bits AS y,position_z_ieee754_bits AS z FROM lowpoly_mesh_vertex ORDER BY ordinal").all()).toEqual(fixture.mesh.vertices.map(vertex=>({x:parseInt(vertex.position[0]!,16),y:parseInt(vertex.position[1]!,16),z:parseInt(vertex.position[2]!,16)})));
 expect(oracle.query("SELECT halfedge_index,smooth,flipped FROM lowpoly_mesh_face ORDER BY ordinal").all()).toEqual([{halfedge_index:0,smooth:1,flipped:1}]);
 expect(oracle.query("SELECT halfedge_index FROM lowpoly_mesh_seam ORDER BY ordinal").all()).toEqual([{halfedge_index:0},{halfedge_index:2}]);
 expect(oracle.query("SELECT name,domain,semantic,interpolation,has_indices FROM lowpoly_mesh_attribute ORDER BY ordinal").all()).toEqual(fixture.mesh.attributes.map(value=>({name:value.name,domain:value.domain,semantic:value.semantic,interpolation:value.interpolation,has_indices:value.indices===null?0:1})));
 expect(oracle.query("SELECT value FROM lowpoly_mesh_unsigned").all()).toEqual([{value:"18446744073709551615"}]);
 expect(oracle.query("SELECT type FROM pragma_table_info('lowpoly_mesh_float') WHERE name='value_ieee754_bits'").get()).toEqual({type:"INTEGER"});
 const signed=Database.deserialize(oracle.serialize(),{safeIntegers:true});try{expect(signed.query("SELECT value FROM lowpoly_mesh_signed").all()).toEqual([{value:-9223372036854775808n}]);}finally{signed.close();}
 expect(oracle.query("SELECT name FROM lowpoly_mesh_object_member WHERE name='same' ORDER BY ordinal").all()).toEqual([{name:"same"},{name:"same"}]);
 expect(oracle.query("SELECT mime FROM lowpoly_mesh_texture ORDER BY ordinal").all()).toEqual(fixture.mesh.textures.map(value=>({mime:value.mime})));
 expect(oracle.query("SELECT value FROM lowpoly_mesh_texture_octet ORDER BY ordinal").all()).toEqual(fixture.mesh.textures[0]!.bytes.map(value=>({value})));
 expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(await lowpolySnapshotFromSqliteDatabase(await importSqliteDatabase(oracle.serialize()))).toEqual(expected);expect(parseLowpolyArtifact(expected)).toEqual(expected);
 oracle.exec("UPDATE lowpoly_mesh_halfedge SET next_index=0 WHERE ordinal=1;UPDATE lowpoly_mesh_texture_octet SET value=17 WHERE ordinal=0;");const edited=await lowpolySnapshotFromSqliteDatabase(await importSqliteDatabase(oracle.serialize()))as unknown as typeof expected;expect(edited.objects[0]!.meshState.halfedges[1]!.next).toBe(0);expect(edited.objects[0]!.meshState.textures[0]!.bytes[0]).toBe(17);expect(edited.objects[0]!.meshContent).toBe(fixture.source);
 }finally{oracle.close();}
 const empty={vertices:[],halfedges:[],faces:[],uvSeams:[],attributes:[],materials:[],textures:[]};for(const state of[null,empty]){const source={...expected,objects:[{...expected.objects[0]!,meshState:state}]},database=await lowpolySnapshotToSqliteDatabase(source),oracle=Database.deserialize(await exportSqliteDatabase(database));try{expect(oracle.query("SELECT COUNT(*) AS count FROM lowpoly_mesh_state").get()).toEqual({count:state===null?0:1});expect(await lowpolySnapshotFromSqliteDatabase(await importSqliteDatabase(oracle.serialize()))).toEqual(source);}finally{oracle.close();}}
 for(const field of["attributes","materials","textures"]as const){const invalid=structuredClone(expected);const rows=invalid.objects[0]!.meshState[field];rows.push({...rows[0]!}as never);expect(()=>parseLowpolyArtifact(invalid)).toThrow();}
});
test("Lowpoly public mesh JSON publishes canonical intrinsic words and optional full topology",()=>{
 const definitions=artifactSchema.$defs as unknown as Record<string,{properties?:Record<string,unknown>}>;
 expect(definitions.LowpolyObject!.properties!.meshState).toEqual({oneOf:[{type:"null"},{$ref:"https://json.schemas.assets.semio-tech.com/s/lowpoly/lowpoly/mesh/schema.json"}]});
 const validate=new Ajv({strict:false}).addSchema(valueSchema).addSchema(ioSchema).addSchema(childSchema).addSchema(intrinsicSchema).addSchema(meshSchema).compile(artifactSchema),vector=[0,0,0],wire={schema:"lowpoly.document",objects:[{id:"managed",name:"世界",transform:{position:vector,rotation:vector,scale:vector},smoothShading:false,mesh:null,meshContent:fixture.source,meshState:fixture.mesh,paintLayers:[]}]};
 expect(validate(wire)).toBe(true);const decoded=decodeLowpolyJsonSnapshot(wire);expect(decoded.objects[0]!.meshState).toEqual(mesh());
 const wrong=structuredClone(wire);wrong.objects[0]!.meshState.materials.find(member=>member.value.kind==="unsigned")!.value.value="18446744073709551616";expect(validate(wrong)).toBe(false);expect(()=>decodeLowpolyJsonSnapshot(wrong)).toThrow();
});

 test("Lowpoly managed mesh patches preserve untouched clear and full state through closed public domains",async()=>{
 const diff=(await import("../../../../../🧬️schema/🔺️diff/🔣️.json")).default,artifact=artifactSchema,intrinsic=intrinsicSchema,managed=meshSchema,child=childSchema,io=ioSchema,value=valueSchema;
 const ajv=new Ajv({strict:false}).addSchema(artifact).addSchema(intrinsic).addSchema(managed).addSchema(child).addSchema(io).addSchema(value).addSchema(diff),validate=ajv.compile({$ref:diff.$id+"#/$defs/LowpolyObjectPatch"}),base={name:null,smoothShading:null,transform:null,mesh:null,meshContent:null};
 for(const meshState of [null,{state:null},{state:fixture.mesh}])expect(validate({...base,meshState})).toBe(true);
 for(const meshState of [{},{state:null,extra:true},{state:{...fixture.mesh,extra:true}}])expect(validate({...base,meshState})).toBe(false);
 const {parseLowpolyObjectPatch}=await import("../../../../../🧬️schema/🔺️diff/🟦️.ts");
 expect(parseLowpolyObjectPatch({...base,meshState:{state:mesh()}}).meshState).toEqual({state:mesh()});
 expect(parseLowpolyObjectPatch({...base,meshState:{state:null}}).meshState).toEqual({state:null});
 expect(parseLowpolyObjectPatch({...base,meshState:null}).meshState).toBeNull();
 });
