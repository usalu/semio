/** 🕸️ Full managed topology and intrinsic surface values have independent relational owners. */
import type{LowpolyMeshState}from"../../../../🧬️schema/🕸️mesh/🟦️.ts";
import type{IntrinsicValue}from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🌳️intrinsic/🟦️.ts";
import{encodeIeee754Cells,readBinary32,readBinary64}from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import{ArtifactSqliteRowIndex,artifactSqliteInteger as integer,artifactSqliteText as text,artifactSqliteBoolean as boolean,artifactSqliteCheckpoint}from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type{SqliteRow,SqliteTable,SqliteValue,SqliteOperation}from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
type Put=(table:string,cells:readonly SqliteValue[],identity?:bigint)=>Promise<bigint>;
const VERTEX=[{index:3,width:32},{index:4,width:32},{index:5,width:32},{index:6,width:32},{index:7,width:32},{index:8,width:32}]as const;
const EDGE=[{index:7,width:32},{index:8,width:32}]as const;
const FLOAT=[{index:1,width:64}]as const;
function u32(value:bigint):number{if(value<0n||value>0xffffffffn)throw Error("Lowpoly mesh index exceeds UInt32");return Number(value);}
function optional(row:SqliteRow,index:number):number|null{return row.values[index]===null?null:u32(integer(row,index));}
function named(rows:readonly{name:string}[]):void{const names=new Set<string>();for(const row of rows){if(names.has(row.name))throw Error("Lowpoly managed namespace repeats a name");names.add(row.name);}}
/** 📤️ Traverse borrowed members without making a JSON document or narrowing logical references. */
export async function projectLowpolyMesh(mesh:LowpolyMeshState,owner:bigint,put:Put):Promise<void>{
 await put("lowpoly_mesh_state",[],owner);
 const pending:{value:IntrinsicValue;attach:(id:bigint)=>Promise<unknown>}[]=[];
 for(const[ordinal,vertex]of mesh.vertices.entries())await put("lowpoly_mesh_vertex",encodeIeee754Cells([0n,owner,BigInt(ordinal),...vertex.position,...(vertex.normal??[null,null,null]),vertex.halfedge===null?null:BigInt(vertex.halfedge)],VERTEX).slice(1));
 for(const[ordinal,edge]of mesh.halfedges.entries())await put("lowpoly_mesh_halfedge",encodeIeee754Cells([0n,owner,BigInt(ordinal),BigInt(edge.vertex),edge.twin===null?null:BigInt(edge.twin),BigInt(edge.next),edge.face===null?null:BigInt(edge.face),...edge.uv],EDGE).slice(1));
 for(const[ordinal,face]of mesh.faces.entries())await put("lowpoly_mesh_face",[owner,BigInt(ordinal),BigInt(face.halfedge),face.smooth?1n:0n,face.flipped?1n:0n]);
 for(const[ordinal,seam]of mesh.uvSeams.entries())await put("lowpoly_mesh_seam",[owner,BigInt(ordinal),BigInt(seam)]);
 for(const[ordinal,attribute]of mesh.attributes.entries()){
  const id=await put("lowpoly_mesh_attribute",[owner,BigInt(ordinal),attribute.name,attribute.domain,attribute.semantic,attribute.interpolation,attribute.indices===null?0n:1n]);
  for(const[index,value]of(attribute.indices??[]).entries())await put("lowpoly_mesh_attribute_index",[id,BigInt(index),BigInt(value)]);
  for(let index=attribute.values.length-1;index>=0;index--)pending.push({value:attribute.values[index]!,attach:value=>put("lowpoly_mesh_attribute_sample",[id,BigInt(index),value])});
 }
 for(let ordinal=mesh.materials.length-1;ordinal>=0;ordinal--){const material=mesh.materials[ordinal]!;pending.push({value:material.value,attach:value=>put("lowpoly_mesh_material",[owner,BigInt(ordinal),material.name,value])});}
 for(const[ordinal,texture]of mesh.textures.entries()){const id=await put("lowpoly_mesh_texture",[owner,BigInt(ordinal),texture.name,texture.mime]);for(const[index,value]of texture.bytes.entries())await put("lowpoly_mesh_texture_octet",[id,BigInt(index),BigInt(value)]);}
 while(pending.length){const task=pending.pop()!,value=task.value,id=await put("lowpoly_mesh_value",[value.kind]);await task.attach(id);
  switch(value.kind){
   case"null":break;
   case"boolean":await put("lowpoly_mesh_boolean",[value.value?1n:0n],id);break;
   case"unsigned":await put("lowpoly_mesh_unsigned",[value.value.toString()],id);break;
   case"signed":await put("lowpoly_mesh_signed",[value.value],id);break;
   case"float":await put("lowpoly_mesh_float",encodeIeee754Cells([id,value.value],FLOAT).slice(1),id);break;
   case"text":await put("lowpoly_mesh_text",[value.value],id);break;
   case"bytes":await put("lowpoly_mesh_bytes",[],id);for(const[ordinal,byte]of value.value.entries())await put("lowpoly_mesh_bytes_octet",[id,BigInt(ordinal),BigInt(byte)]);break;
   case"array":for(let ordinal=value.items.length-1;ordinal>=0;ordinal--)pending.push({value:value.items[ordinal]!,attach:child=>put("lowpoly_mesh_array_member",[id,BigInt(ordinal),child])});break;
   case"object":for(let ordinal=value.members.length-1;ordinal>=0;ordinal--){const member=value.members[ordinal]!;pending.push({value:member.value,attach:child=>put("lowpoly_mesh_object_member",[id,BigInt(ordinal),member.name,child])});}break;
  }
 }
}
/** 🧮️ Count actual authored entities including each independent intrinsic scalar owner. */
export function lowpolyMeshRows(mesh:LowpolyMeshState):number{
 let rows=1+mesh.vertices.length+mesh.halfedges.length+mesh.faces.length+mesh.uvSeams.length+mesh.attributes.length+mesh.materials.length+mesh.textures.length;
 const values:IntrinsicValue[]=[];for(const attribute of mesh.attributes){rows+=attribute.values.length+(attribute.indices?.length??0);for(const value of attribute.values)values.push(value);}for(const material of mesh.materials)values.push(material.value);for(const texture of mesh.textures)rows+=texture.bytes.length;
 while(values.length){const value=values.pop()!;rows++;switch(value.kind){case"null":break;case"array":rows+=value.items.length;for(const item of value.items)values.push(item);break;case"object":rows+=value.members.length;for(const member of value.members)values.push(member.value);break;case"bytes":rows+=1+value.value.length;break;default:rows++;}}
 if(!Number.isSafeInteger(rows))throw Error("Lowpoly mesh workload exceeds safe row count");return rows;
}
/** 📥️ Consume every concrete relation once and refuse shared, cyclic or orphan ownership. */
export class LowpolyMeshRows{
 private constructor(private readonly index:ArtifactSqliteRowIndex,private readonly tables:Map<string,Map<bigint,SqliteRow>>,private readonly operation:SqliteOperation){}
 static async create(tables:readonly SqliteTable[],operation:SqliteOperation):Promise<LowpolyMeshRows>{const maps=new Map<string,Map<bigint,SqliteRow>>();for(const table of tables){const rows=new Map<bigint,SqliteRow>();for(const row of table.rows){if(integer(row,0)!==row.rowid||rows.has(row.rowid))throw Error("Lowpoly managed entity identity");rows.set(row.rowid,row);}maps.set(table.name,rows);}return new LowpolyMeshRows(await ArtifactSqliteRowIndex.create(tables,operation),maps,operation);}
 private async take(name:string,id:bigint):Promise<SqliteRow>{const row=this.tables.get(name)?.get(id);if(!row)throw Error("Lowpoly managed relation is missing");await this.index.take(row);return row;}
 private async list(name:string,parent:bigint):Promise<SqliteRow[]>{const rows=await this.index.grouped(name,parent);for(const row of rows)await this.index.take(row);return rows;}
 private async octets(name:string,parent:bigint):Promise<Uint8Array>{const rows=await this.list(name,parent),bytes=this.operation.allocateBytes(rows.length);for(let ordinal=0;ordinal<rows.length;ordinal++){const value=integer(rows[ordinal]!,3);if(value<0n||value>255n)throw Error("Lowpoly managed octet width");bytes[ordinal]=Number(value);if((ordinal+1)%256===0)await artifactSqliteCheckpoint(this.operation,"reconstructSnapshot",ordinal+1,rows.length);}return bytes;}
 private async value(id:bigint):Promise<IntrinsicValue>{
  let result:IntrinsicValue={kind:"null"};const pending:{id:bigint;attach:(value:IntrinsicValue)=>void}[]=[{id,attach:value=>{result=value}}];
  while(pending.length){const task=pending.pop()!,row=await this.take("lowpoly_mesh_value",task.id),kind=text(row,1);let value:IntrinsicValue;
   switch(kind){
    case"null":value={kind};break;
    case"boolean":value={kind,value:boolean(await this.take("lowpoly_mesh_boolean",task.id),1)};break;
    case"unsigned":{const magnitude=text(await this.take("lowpoly_mesh_unsigned",task.id),1);if(!/^(0|[1-9][0-9]{0,19})$/.test(magnitude)||BigInt(magnitude)>0xffffffffffffffffn)throw Error("Lowpoly mesh UInt64 magnitude");value={kind,value:BigInt(magnitude)};break;}
    case"signed":{const value=integer(await this.take("lowpoly_mesh_signed",task.id),1);if(value<-(1n<<63n)||value>=(1n<<63n))throw Error("Lowpoly mesh Int64 width");task.attach({kind,value});continue;}
    case"float":value={kind,value:readBinary64(await this.take("lowpoly_mesh_float",task.id),1,FLOAT)};break;
    case"text":value={kind,value:text(await this.take("lowpoly_mesh_text",task.id),1)};break;
    case"bytes":await this.take("lowpoly_mesh_bytes",task.id);value={kind,value:await this.octets("lowpoly_mesh_bytes_octet",task.id)};break;
    case"array":{const items:IntrinsicValue[]=[];for(const row of await this.list("lowpoly_mesh_array_member",task.id)){const ordinal=items.length;items.push({kind:"null"});pending.push({id:integer(row,3),attach:value=>{items[ordinal]=value}});}value={kind,items};break;}
    case"object":{const members:{name:string;value:IntrinsicValue}[]=[];for(const row of await this.list("lowpoly_mesh_object_member",task.id)){const member={name:text(row,3),value:{kind:"null"}as IntrinsicValue};members.push(member);pending.push({id:integer(row,4),attach:value=>{member.value=value}});}value={kind,members};break;}
    default:throw Error("Lowpoly managed intrinsic kind");
   }task.attach(value);
  }return result;
 }
 async mesh(owner:bigint):Promise<LowpolyMeshState|null>{
  if(!this.tables.get("lowpoly_mesh_state")?.has(owner))return null;await this.take("lowpoly_mesh_state",owner);const mesh:LowpolyMeshState={vertices:[],halfedges:[],faces:[],uvSeams:[],attributes:[],materials:[],textures:[]};
  for(const row of await this.list("lowpoly_mesh_vertex",owner)){const normals=[row.values[16],row.values[18],row.values[20]],present=normals.every(value=>value!==null);if(!present&&normals.some(value=>value!==null))throw Error("Lowpoly mesh optional normal components differ");if(!present)for(const at of[6,7,8,17,19,21])if(row.values[at]!==null)throw Error("Lowpoly absent normal carries scalar state");mesh.vertices.push({position:[readBinary32(row,3,VERTEX),readBinary32(row,4,VERTEX),readBinary32(row,5,VERTEX)],normal:present?[readBinary32(row,6,VERTEX),readBinary32(row,7,VERTEX),readBinary32(row,8,VERTEX)]:null,halfedge:optional(row,9)});}
  for(const row of await this.list("lowpoly_mesh_halfedge",owner))mesh.halfedges.push({vertex:u32(integer(row,3)),twin:optional(row,4),next:u32(integer(row,5)),face:optional(row,6),uv:[readBinary32(row,7,EDGE),readBinary32(row,8,EDGE)]});
  for(const row of await this.list("lowpoly_mesh_face",owner))mesh.faces.push({halfedge:u32(integer(row,3)),smooth:boolean(row,4),flipped:boolean(row,5)});
  for(const row of await this.list("lowpoly_mesh_seam",owner))mesh.uvSeams.push(u32(integer(row,3)));
  for(const row of await this.list("lowpoly_mesh_attribute",owner)){const indices=await this.list("lowpoly_mesh_attribute_index",row.rowid),present=boolean(row,7);if(!present&&indices.length)throw Error("Lowpoly absent attribute indices own entries");const values:IntrinsicValue[]=[];for(const sample of await this.list("lowpoly_mesh_attribute_sample",row.rowid))values.push(await this.value(integer(sample,3)));mesh.attributes.push({name:text(row,3),domain:text(row,4)as LowpolyMeshState["attributes"][number]["domain"],semantic:text(row,5)as LowpolyMeshState["attributes"][number]["semantic"],interpolation:text(row,6)as LowpolyMeshState["attributes"][number]["interpolation"],values,indices:present?indices.map(row=>u32(integer(row,3))):null});}
  for(const row of await this.list("lowpoly_mesh_material",owner))mesh.materials.push({name:text(row,3),value:await this.value(integer(row,4))});
  for(const row of await this.list("lowpoly_mesh_texture",owner))mesh.textures.push({name:text(row,3),mime:text(row,4),bytes:await this.octets("lowpoly_mesh_texture_octet",row.rowid)});
  named(mesh.attributes);named(mesh.materials);named(mesh.textures);if(new Set(mesh.uvSeams).size!==mesh.uvSeams.length)throw Error("Lowpoly managed seam identity repeats");
  for(const attribute of mesh.attributes)if(!["vertex","corner","face","edge"].includes(attribute.domain)||!["normal","uv","color","material","custom"].includes(attribute.semantic)||!["linear","nearest","constant"].includes(attribute.interpolation))throw Error("Lowpoly managed channel enum");
  return mesh;
 }
 async finish():Promise<void>{await this.index.finish();}
}
