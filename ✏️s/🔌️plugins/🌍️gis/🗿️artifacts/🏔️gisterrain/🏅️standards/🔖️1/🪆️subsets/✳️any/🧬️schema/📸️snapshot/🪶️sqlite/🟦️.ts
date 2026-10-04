/** 🏔️ Complete imported intrinsic trees and independent mesh identities. */
import type {GisTerrainArtifact,ImportedMap} from "../../🟦️.ts";
import type {IntrinsicValue} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";
import type {ArtifactDialect} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts";
import {sqliteOperation,type SqliteDatabase,type SqliteRow,type SqliteValue} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteCheckpoint,artifactSqliteInteger as integer,artifactSqliteText as text,artifactSqliteBoolean as boolean,artifactSqliteOrderedRowsControlled,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import {encodeIeee754Cells,readBinary64} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import sql from "./🗄️.sql" with {type:"text"};
export const GIS_TERRAIN_SQLITE_SCHEMA:string=sql;
const SCALAR=[{index:1,width:64}]as const;
const roles=["position","route","region"]as const;
/** 📤️ Each occurrence owns one intrinsic tree, with no encoded descriptor cell. */
export async function gisTerrainSnapshotToSqliteDatabase(snapshot:GisTerrainArtifact,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 options=sqliteOperation(options);const p=await ArtifactSqliteProjection.create(sql,options),put=(name:string,cells:readonly SqliteValue[],id?:bigint)=>p.insert("gis_terrain_"+name,cells,id);
 await put("document",[],1n);await put("parameters",encodeIeee754Cells([1n,snapshot.exaggeration],SCALAR,options.maxColumns).slice(1),1n);
 if(snapshot.mesh){const c=snapshot.mesh;await put("mesh_child",[c.childId,c.target.artifactId,c.target.dialect.artifactKind,c.target.dialect.standard,c.target.dialect.subset],1n);}
 if(snapshot.importedMap!==undefined){const map=snapshot.importedMap;await put("imported_map",[],1n);
  const pending:({value:IntrinsicValue;attach:(id:bigint)=>Promise<unknown>}|{close:IntrinsicValue})[]=[],active=new Set<object>();
  const enqueue=(value:IntrinsicValue,attach:(id:bigint)=>Promise<unknown>)=>{p.checkRowsAdditional(pending.length+1);pending.push({value,attach});};
  for(const role of roles){const values=map[(role+"s")as"positions"|"routes"|"regions"];p.checkRowsAdditional(values.length);for(let ordinal=values.length-1;ordinal>=0;ordinal--){const value=values[ordinal]!;if(value.kind!=="object")throw Error("imported feature object required");enqueue(value,id=>put(role,[1n,BigInt(ordinal),id]));}}
  for(let ordinal=map.properties.length-1;ordinal>=0;ordinal--){const member=map.properties[ordinal]!;if(["positions","routes","regions"].includes(member.name))throw Error("reserved imported map property");enqueue(member.value,id=>put("property",[1n,BigInt(ordinal),member.name,id]));}
  while(pending.length){await p.checkpoint();const task=pending.pop()!;if("close"in task){active.delete(task.close);continue;}const v=task.value;if(active.has(v))throw Error("cyclic imported value ownership");active.add(v);pending.push({close:v});const id=await put("value",[v.kind]);await task.attach(id);
   switch(v.kind){case"null":break;case"boolean":if(typeof v.value!=="boolean")throw Error("intrinsic boolean required");await put("boolean",[v.value?1n:0n],id);break;
    case"unsigned":if(typeof v.value!=="bigint"||v.value<0n||v.value>0xffffffffffffffffn)throw Error("intrinsic unsigned64 range");await put("unsigned",[v.value.toString()],id);break;
    case"signed":if(typeof v.value!=="bigint"||v.value<-(1n<<63n)||v.value>=(1n<<63n))throw Error("intrinsic signed64 range");await put("signed",[v.value],id);break;
    case"float":await put("float",encodeIeee754Cells([id,v.value],SCALAR,options.maxColumns).slice(1),id);break;
    case"text":await put("text",[v.value],id);break;case"bytes":if(!(v.value instanceof Uint8Array))throw Error("intrinsic octets required");await put("bytes",[v.value],id);break;
    case"array":p.checkRowsAdditional(v.items.length);for(let ordinal=v.items.length-1;ordinal>=0;ordinal--)enqueue(v.items[ordinal]!,child=>put("array",[id,BigInt(ordinal),child]));break;
    case"object":p.checkRowsAdditional(v.members.length);for(let ordinal=v.members.length-1;ordinal>=0;ordinal--){const m=v.members[ordinal]!;enqueue(m.value,child=>put("member",[id,BigInt(ordinal),m.name,child]));}break;
    default:throw Error("unknown intrinsic kind");
   }
  }
 }return p.finish();
}
/** 📥️ Consuming each row exactly once proves dense ordered ownership and variant exclusivity. */
export async function gisTerrainSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<GisTerrainArtifact>{
 const operation=sqliteOperation(options);options=operation;const rows=await artifactSqliteTables(database,sql,options),names=["document","parameters","mesh_child","imported_map","value","position","route","region","property","boolean","unsigned","signed","float","text","bytes","array","member"],widths=[1,4,6,1,2,4,4,4,5,2,2,2,4,2,2,4,5],tables=new Map<string,Map<bigint,SqliteRow>>();
 for(let i=0;i<names.length;i++){const map=new Map<bigint,SqliteRow>();for(let j=0;j<rows[i]!.length;j++){const r=rows[i]![j]!;if(r.values.length!==widths[i]||integer(r,0)!==r.rowid||map.has(r.rowid))throw Error("GIS terrain row identity differs");map.set(r.rowid,r);if((j+1)%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",j+1,rows[i]!.length);}tables.set(names[i]!,map);}
 const table=(name:string)=>tables.get(name)!,take=(name:string,id:bigint)=>{const row=table(name).get(id);if(!row)throw Error("dangling or multiply owned GIS terrain relation");table(name).delete(id);return row;};
 const grouped=new Map<string,Map<bigint,SqliteRow[]>>();
 const list=async(name:string,parent:bigint)=>{let groups=grouped.get(name);if(!groups){groups=new Map();let count=0;for(const row of table(name).values()){const id=integer(row,1),values=groups.get(id)??[];values.push(row);groups.set(id,values);if(++count%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",count,0);}grouped.set(name,groups);}const ordered=await artifactSqliteOrderedRowsControlled(groups.get(parent)??[],2,options);for(const row of ordered)take(name,row.rowid);return ordered;};
 if(table("document").size!==1||table("parameters").size!==1||table("mesh_child").size>1||table("imported_map").size>1)throw Error("GIS terrain entity ownership differs");const doc=table("document").values().next().value as SqliteRow;take("document",doc.rowid);const parameters=take("parameters",doc.rowid),result:GisTerrainArtifact={exaggeration:readBinary64(parameters,1,SCALAR)};
 if(table("mesh_child").size){const r=take("mesh_child",doc.rowid);result.mesh={childId:text(r,1),target:{artifactId:text(r,2),dialect:{artifactKind:text(r,3),standard:text(r,4),subset:text(r,5)}}};}
 if(table("imported_map").size){take("imported_map",doc.rowid);const map:ImportedMap={positions:[],routes:[],regions:[],properties:[]},pending:{id:bigint;attach:(v:IntrinsicValue)=>void}[]=[];
  for(const role of roles){const values=map[(role+"s")as"positions"|"routes"|"regions"];for(const row of await list(role,doc.rowid)){const ordinal=values.length;values.push({kind:"object",members:[]});pending.push({id:integer(row,3),attach:v=>{if(v.kind!=="object")throw Error("imported feature object required");values[ordinal]=v;}});}}
  for(const row of await list("property",doc.rowid)){const name=text(row,3);if(["positions","routes","regions"].includes(name))throw Error("reserved imported map property");const member={name,value:{kind:"null"}as IntrinsicValue};map.properties.push(member);pending.push({id:integer(row,4),attach:v=>{member.value=v;}});}
  let completed=0;while(pending.length){const task=pending.pop()!,row=take("value",task.id),kind=text(row,1);let value:IntrinsicValue;
   switch(kind){case"null":value={kind};break;case"boolean":value={kind,value:boolean(take(kind,task.id),1)};break;
    case"unsigned":{const decimal=text(take(kind,task.id),1);if(!/^(0|[1-9][0-9]*)$/.test(decimal)||decimal.length>20)throw Error("intrinsic unsigned64 canonical decimal");const number=BigInt(decimal);if(number>0xffffffffffffffffn)throw Error("intrinsic unsigned64 range");value={kind,value:number};break;}
    case"signed":value={kind,value:integer(take(kind,task.id),1)};break;case"float":value={kind,value:readBinary64(take(kind,task.id),1,SCALAR)};break;
    case"text":value={kind,value:text(take(kind,task.id),1)};break;case"bytes":{const bytes=take(kind,task.id).values[1];if(!(bytes instanceof Uint8Array))throw Error("intrinsic octets required");const owned=operation.allocateBytes(bytes.length);for(let offset=0;offset<bytes.length;offset+=65536){owned.set(bytes.subarray(offset,offset+65536),offset);await artifactSqliteCheckpoint(options,"reconstructSnapshot",offset,bytes.length);}value={kind,value:owned};break;}
    case"array":{const items:IntrinsicValue[]=[];for(const r of await list(kind,task.id)){const ordinal=items.length;items.push({kind:"null"});pending.push({id:integer(r,3),attach:v=>{items[ordinal]=v;}});}value={kind,items};break;}
    case"object":{const members:{name:string;value:IntrinsicValue}[]=[];for(const r of await list("member",task.id)){const member={name:text(r,3),value:{kind:"null"}as IntrinsicValue};members.push(member);pending.push({id:integer(r,4),attach:v=>{member.value=v;}});}value={kind,members};break;}
    default:throw Error("unknown intrinsic kind");
   }task.attach(value);if(++completed%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",completed,0);
  }result.importedMap=map;
 }for(const rows of tables.values())if(rows.size)throw Error("orphan GIS terrain entities");await artifactSqliteCheckpoint(options,"reconstructSnapshot",1,1);return result;
}
/** 🪪️ Compare exact intrinsic domains, IEEE words, member order and owned child identities. */
export async function validateGisTerrainSnapshotSqliteDialect(snapshot:GisTerrainArtifact,dialect:ArtifactDialect,database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<readonly never[]>{
 options=sqliteOperation(options);await artifactSqliteCheckpoint(options,"projectSnapshot",0,0,false);if(dialect.artifactKind!=="s.gis.gisterrain"||dialect.standard!=="1"||dialect.subset!=="*")throw Error("GIS terrain does not own this semantic subset");const restored=await gisTerrainSnapshotFromSqliteDatabase(database,options),pending:[unknown,unknown][]=[[snapshot,restored]];let completed=0;
 while(pending.length){const[a,b]=pending.pop()!;if(a instanceof Uint8Array&&b instanceof Uint8Array){if(a.length!==b.length||a.some((v,i)=>v!==b[i]))throw Error("GIS terrain owned identity differs");continue;}if(a&&b&&typeof a==="object"&&typeof b==="object"){const ak=Object.keys(a),bk=Object.keys(b);if(ak.length!==bk.length||ak.some(k=>!Object.hasOwn(b,k)))throw Error("GIS terrain owned identity differs");for(const k of ak)pending.push([(a as Record<string,unknown>)[k],(b as Record<string,unknown>)[k]]);}else if(a!==b)throw Error("GIS terrain owned identity differs");if(++completed%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",completed,0);}return[];
}
