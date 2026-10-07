/** 🔌️ Seventeen handwritten entities preserve Wires intrinsic trees and the independent graph handle. */
import type{WiresSnapshot}from"../../../🧬️schema/📸️snapshot/🟦️.ts";
import type{WiresValue}from"../../../🧬️schema/🌱️value/🟦️.ts";
import type{ ArtifactDialect } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🟦️.ts";
import type{SqliteDatabase,SqliteRow}from"../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import{ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteCheckpoint,artifactSqliteInteger as integer,artifactSqliteText as text,artifactSqliteValueBudget,type ArtifactSqliteOptions}from"../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import {encodeIeee754Cells,readBinary64,ieee754CellByteLength} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import {parseBinary64} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
export const WIRES_SQLITE_SCHEMA=String.raw`CREATE TABLE wires_document (id INTEGER PRIMARY KEY);
CREATE TABLE wires_value (id INTEGER PRIMARY KEY, kind TEXT NOT NULL CHECK(kind IN ('null','boolean','unsigned','signed','float','text','bytes','array','object')));
CREATE TABLE wires_snapshot_root (id INTEGER PRIMARY KEY REFERENCES wires_document(id), value_id INTEGER NOT NULL REFERENCES wires_value(id));
CREATE TABLE wires_meta_root (id INTEGER PRIMARY KEY REFERENCES wires_document(id), value_id INTEGER NOT NULL REFERENCES wires_value(id));
CREATE TABLE wires_content_child (id INTEGER PRIMARY KEY REFERENCES wires_document(id), child_id TEXT NOT NULL, artifact_id TEXT NOT NULL, artifact_kind TEXT NOT NULL, standard TEXT NOT NULL, subset TEXT NOT NULL);
CREATE TABLE wires_null (id INTEGER PRIMARY KEY REFERENCES wires_value(id));
CREATE TABLE wires_boolean (id INTEGER PRIMARY KEY REFERENCES wires_value(id), value INTEGER NOT NULL CHECK(value IN (0,1)));
CREATE TABLE wires_unsigned (id INTEGER PRIMARY KEY REFERENCES wires_value(id), high INTEGER NOT NULL CHECK(high BETWEEN 0 AND 4294967295), low INTEGER NOT NULL CHECK(low BETWEEN 0 AND 4294967295));
CREATE TABLE wires_signed (id INTEGER PRIMARY KEY REFERENCES wires_value(id), value INTEGER NOT NULL);
CREATE TABLE wires_float (id INTEGER PRIMARY KEY REFERENCES wires_value(id), value REAL, value_ieee754_bits INTEGER NOT NULL, value_numeric_class TEXT NOT NULL CHECK(value_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')));
CREATE TABLE wires_text (id INTEGER PRIMARY KEY REFERENCES wires_value(id), value TEXT NOT NULL);
CREATE TABLE wires_bytes (id INTEGER PRIMARY KEY REFERENCES wires_value(id));
CREATE TABLE wires_octet (id INTEGER PRIMARY KEY, bytes_id INTEGER NOT NULL REFERENCES wires_bytes(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), value INTEGER NOT NULL CHECK(value BETWEEN 0 AND 255));
CREATE TABLE wires_array (id INTEGER PRIMARY KEY REFERENCES wires_value(id));
CREATE TABLE wires_array_element (id INTEGER PRIMARY KEY, array_id INTEGER NOT NULL REFERENCES wires_array(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), value_id INTEGER NOT NULL REFERENCES wires_value(id));
CREATE TABLE wires_object (id INTEGER PRIMARY KEY REFERENCES wires_value(id));
CREATE TABLE wires_object_member (id INTEGER PRIMARY KEY, object_id INTEGER NOT NULL REFERENCES wires_object(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), name TEXT NOT NULL, value_id INTEGER NOT NULL REFERENCES wires_value(id));
`;
const FLOAT=[{index:1,width:64}]as const;
function variant(kind:string):number{switch(kind){case"null":return 5;case"boolean":return 6;case"unsigned":return 7;case"signed":return 8;case"float":return 9;case"text":return 10;case"bytes":return 11;case"array":return 13;case"object":return 15;default:throw Error("Wires intrinsic variant differs");}}
function words(value:bigint):[bigint,bigint]{if(typeof value!=="bigint"||value<0n||value>18446744073709551615n)throw Error("Wires unsigned value differs");return[value>>32n,value&4294967295n];}
async function textBytes(value:string,options:ArtifactSqliteOptions,phase:"projectSnapshot"|"reconstructSnapshot",before:number):Promise<number>{
 if(typeof value!=="string")throw Error("Wires text differs");let bytes=0,next=65536;
 for(let i=0;i<value.length;i++){const n=value.charCodeAt(i);if(n<128)bytes++;else if(n<2048)bytes+=2;else if(n>=0xd800&&n<=0xdbff){const low=value.charCodeAt(++i);if(!(low>=0xdc00&&low<=0xdfff))throw Error("Wires unpaired UTF16 surrogate");bytes+=4;}else if(n>=0xdc00&&n<=0xdfff)throw Error("Wires unpaired UTF16 surrogate");else bytes+=3;
  if(i+1>=next){artifactSqliteValueBudget(before+bytes,options);await artifactSqliteCheckpoint(options,phase,i+1,value.length,false);await new Promise<void>(resolve=>setTimeout(resolve,0));await artifactSqliteCheckpoint(options,phase,i+1,value.length,false);next+=65536;}
 }
 artifactSqliteValueBudget(before+bytes,options);return bytes;
}
/** 📤️ Admit literal rows, text and borrowed tree frontiers before relational allocation. */
export async function wiresSnapshotToSqliteDatabase(snapshot:WiresSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const p=await ArtifactSqliteProjection.create(WIRES_SQLITE_SCHEMA,options);let count=4,bytes=48,metadata=768;
 const strings=[snapshot.content.childId,snapshot.content.target.artifactId,snapshot.content.target.dialect.artifactKind,snapshot.content.target.dialect.standard,snapshot.content.target.dialect.subset];
 for(const value of strings)bytes+=await textBytes(value,options,"projectSnapshot",bytes+metadata);
 p.checkRowsAdditional(count+4);p.checkValueBytesAdditional(bytes+metadata);
 const nodes:WiresValue[]=[snapshot.wiresSnapshot,snapshot.meta],ids=new Map<WiresValue,bigint>();
 for(let index=0;index<nodes.length;index++){
  const node=nodes[index]!;if(node===null||typeof node!=="object"||ids.has(node))throw Error("Wires intrinsic ownership differs");variant(node.kind);ids.set(node,BigInt(index+1));count+=2;bytes+=16+node.kind.length;
  let extra=0;switch(node.kind){
   case"null":break;
   case"boolean":if(typeof node.value!=="boolean")throw Error("Wires boolean differs");bytes+=8;break;
   case"unsigned":words(node.value);bytes+=16;break;
   case"signed":if(typeof node.value!=="bigint"||node.value< -9223372036854775808n||node.value>9223372036854775807n)throw Error("Wires signed value differs");bytes+=8;break;
   case"float":parseBinary64(node.value);bytes+=ieee754CellByteLength(node.value,64);break;
   case"text":bytes+=await textBytes(node.value,options,"projectSnapshot",bytes+metadata);break;
   case"bytes":if(!(node.value instanceof Uint8Array))throw Error("Wires octets differ");count+=node.value.length;bytes+=32*node.value.length;break;
   case"array":if(!Array.isArray(node.items))throw Error("Wires array differs");extra=node.items.length;count+=extra;bytes+=32*extra;break;
   case"object":if(!Array.isArray(node.members))throw Error("Wires members differ");extra=node.members.length;count+=extra;bytes+=32*extra;p.checkRowsAdditional(count+2*(nodes.length-index-1+extra));p.checkValueBytesAdditional(bytes+metadata+96*extra);for(const[i,member]of node.members.entries()){bytes+=await textBytes(member.name,options,"projectSnapshot",bytes+metadata);if((i+1)%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",i+1,node.members.length);}break;
  }
  metadata+=96*extra;p.checkRowsAdditional(count+2*(nodes.length-index-1+extra));p.checkValueBytesAdditional(bytes+metadata);
  if(node.kind==="array")for(const[i,child]of node.items.entries()){nodes.push(child);if((i+1)%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",i+1,node.items.length);}else if(node.kind==="object")for(const[i,member]of node.members.entries()){nodes.push(member.value);if((i+1)%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",i+1,node.members.length);}
  if(index%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",index,nodes.length);
 }
 await p.insert("wires_document",[],1n);await p.insert("wires_snapshot_root",[ids.get(snapshot.wiresSnapshot)!],1n);await p.insert("wires_meta_root",[ids.get(snapshot.meta)!],1n);await p.insert("wires_content_child",strings,1n);
 for(const node of nodes){const id=ids.get(node)!;await p.insert("wires_value",[node.kind],id);switch(node.kind){
  case"null":await p.insert("wires_null",[],id);break;
  case"boolean":await p.insert("wires_boolean",[node.value?1n:0n],id);break;
  case"unsigned":await p.insert("wires_unsigned",words(node.value),id);break;
  case"signed":await p.insert("wires_signed",[node.value],id);break;
  case"float":await p.insert("wires_float",encodeIeee754Cells([id,node.value],FLOAT,options.maxColumns).slice(1),id);break;
  case"text":await p.insert("wires_text",[node.value],id);break;
  case"bytes":await p.insert("wires_bytes",[],id);for(const[index,value]of node.value.entries())await p.insert("wires_octet",[id,BigInt(index),BigInt(value)]);break;
  case"array":await p.insert("wires_array",[],id);for(const[index,value]of node.items.entries())await p.insert("wires_array_element",[id,BigInt(index),ids.get(value)!]);break;
  case"object":await p.insert("wires_object",[],id);for(const[index,member]of node.members.entries())await p.insert("wires_object_member",[id,BigInt(index),member.name,ids.get(member.value)!]);break;
 }}
 return p.finish();
}
/** 📥️ Consume exclusive ownership once and assemble deeply nested intrinsic entities in reverse order. */
export async function wiresSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<WiresSnapshot>{
 const tables=await artifactSqliteTables(database,WIRES_SQLITE_SCHEMA,options),widths=[1,2,2,2,6,1,2,3,2,4,2,1,4,1,4,1,5];
 const total=tables.reduce((n,rows)=>n+rows.length,0);let owned=0,work=0;
 const charge=(bytes:number)=>{owned+=bytes;artifactSqliteValueBudget(owned,options);};
 const ownText=async(row:SqliteRow,index:number)=>{const value=text(row,index);charge(await textBytes(value,options,"reconstructSnapshot",owned));return value;};
 charge(total*192);const maps:Map<bigint,SqliteRow>[]=[],used=new Set<SqliteRow>();
 for(const[index,rows]of tables.entries()){const map=new Map<bigint,SqliteRow>();for(const row of rows){if(row.values.length!==widths[index]||integer(row,0)!==row.rowid||map.has(row.rowid))throw Error("Wires row identity differs");map.set(row.rowid,row);if(++work%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",work,total);}maps.push(map);}
 const take=(table:number,id:bigint)=>{const row=maps[table]!.get(id);if(!row||used.has(row))throw Error("Wires entity has missing or repeated owner");used.add(row);return row;};
 if(tables[0]!.length!==1||tables[2]!.length!==1||tables[3]!.length!==1||tables[4]!.length!==1)throw Error("Wires document root ownership differs");
 const root=take(0,tables[0]![0]!.rowid).rowid,fixtureRoot=integer(take(2,root),1),metaRoot=integer(take(3,root),1),child=take(4,root);
 const content={childId:await ownText(child,1),target:{artifactId:await ownText(child,2),dialect:{artifactKind:await ownText(child,3),standard:await ownText(child,4),subset:await ownText(child,5)}}};
 charge((tables[12]!.length+tables[14]!.length+tables[16]!.length)*16);
 const groups=new Map<string,SqliteRow[]>();for(const table of[12,14,16])for(const row of tables[table]!){const key=table+":"+integer(row,1);let rows=groups.get(key);if(!rows){rows=[];groups.set(key,rows);}rows.push(row);if(++work%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",work,total*2);}
 const edges=async(table:number,id:bigint)=>{const rows=groups.get(table+":"+id)??[];charge(rows.length*8);const result:SqliteRow[]=new Array(rows.length);let count=0;for(const row of rows){const ordinal=integer(row,2);if(ordinal<0n||ordinal>=BigInt(rows.length)||result[Number(ordinal)])throw Error("Wires occurrence ordinal differs");result[Number(ordinal)]=row;if(++count%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",count,rows.length);}return result;};
 type Plan={id:bigint;kind:WiresValue["kind"];row:SqliteRow;relations:SqliteRow[];children:bigint[]};
 charge(16);const pending=[metaRoot,fixtureRoot],plans:Plan[]=[];
 while(pending.length){
  const id=pending.pop()!,header=take(1,id),kind=text(header,1)as WiresValue["kind"],row=take(variant(kind),id);charge(48);let relations:SqliteRow[]=[],children:bigint[]=[];
  if(kind==="bytes"||kind==="array"||kind==="object"){const table=kind==="bytes"?12:kind==="array"?14:16;relations=await edges(table,id);if(kind!=="bytes")charge(relations.length*16);let index=0;for(const edge of relations){take(table,edge.rowid);if(kind!=="bytes")children.push(integer(edge,kind==="array"?3:4));if(++index%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",index,relations.length);}}
  plans.push({id,kind,row,relations,children});for(let i=children.length-1;i>=0;i--)pending.push(children[i]!);if(plans.length%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",plans.length,tables[1]!.length);
 }
 if(used.size!==total)throw Error("Wires has unreachable intrinsic entities");const values=new Map<bigint,WiresValue>();
 const childValue=(id:bigint)=>{const value=values.get(id);if(!value)throw Error("Wires child topology differs");values.delete(id);return value;};
 for(let index=plans.length-1;index>=0;index--){
  const p=plans[index]!;let result:WiresValue;switch(p.kind){
   case"null":result={kind:"null"};break;
   case"boolean":{const n=integer(p.row,1);if(n!==0n&&n!==1n)throw Error("Wires boolean differs");result={kind:"boolean",value:n===1n};break;}
   case"unsigned":{const high=integer(p.row,1),low=integer(p.row,2);if(high<0n||high>4294967295n||low<0n||low>4294967295n)throw Error("Wires unsigned word differs");result={kind:"unsigned",value:(high<<32n)|low};break;}
   case"signed":result={kind:"signed",value:integer(p.row,1)};break;
   case"float":result={kind:"float",value:readBinary64(p.row,1,FLOAT)};break;
   case"text":result={kind:"text",value:await ownText(p.row,1)};break;
   case"bytes":{charge(p.relations.length);const bytes=new Uint8Array(p.relations.length);for(const[i,row]of p.relations.entries()){const n=integer(row,3);if(n<0n||n>255n)throw Error("Wires octet differs");bytes[i]=Number(n);if((i+1)%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i+1,p.relations.length);}result={kind:"bytes",value:bytes};break;}
   case"array":{charge(p.children.length*8);const items:WiresValue[]=[];for(const[i,id]of p.children.entries()){items.push(childValue(id));if((i+1)%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i+1,p.children.length);}result={kind:"array",items};break;}
   case"object":{charge(p.children.length*24);const members:{name:string;value:WiresValue}[]=[];for(const[i,id]of p.children.entries()){members.push({name:await ownText(p.relations[i]!,3),value:childValue(id)});if((i+1)%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i+1,p.children.length);}result={kind:"object",members};break;}
  }
  values.set(p.id,result);if(index%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",plans.length-index,plans.length);
 }
 const wiresSnapshot=childValue(fixtureRoot),meta=childValue(metaRoot);if(values.size)throw Error("Wires roots differ");await artifactSqliteCheckpoint(options,"reconstructSnapshot",total,total);return{wiresSnapshot,content,meta};
}
/** 🧭️ Enforce complete typed equality while admitting independent consistent surrogate names. */
export async function validateWiresSnapshotSqliteDialect(snapshot:WiresSnapshot,dialect:ArtifactDialect,database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<readonly never[]>{
 await artifactSqliteCheckpoint(options,"projectSnapshot",0,0,false);if(dialect.artifactKind!=="s.reasoning.wires"||dialect.standard!=="1"||dialect.subset!=="*")throw Error("Wires does not own this snapshot coordinate");
 const restored=await wiresSnapshotFromSqliteDatabase(database,options),a=snapshot.content,b=restored.content;
 if(a.childId!==b.childId||a.target.artifactId!==b.target.artifactId||a.target.dialect.artifactKind!==b.target.dialect.artifactKind||a.target.dialect.standard!==b.target.dialect.standard||a.target.dialect.subset!==b.target.dialect.subset)throw Error("Wires child identity differs");
 artifactSqliteValueBudget(64,options);const pending:[WiresValue,WiresValue][]=[[snapshot.wiresSnapshot,restored.wiresSnapshot],[snapshot.meta,restored.meta]];let work=0,owned=64;
 while(pending.length){const[a,b]=pending.pop()!;if(a.kind!==b.kind)throw Error("Wires intrinsic identity differs");
  switch(a.kind){case"null":break;case"boolean":case"unsigned":case"signed":case"text":if(!("value"in b)||a.value!==b.value)throw Error("Wires scalar identity differs");break;
   case"float":if(b.kind!=="float"||a.value.bits!==b.value.bits)throw Error("Wires IEEE identity differs");break;
   case"bytes":if(b.kind!=="bytes"||a.value.length!==b.value.length)throw Error("Wires octet identity differs");for(let i=0;i<a.value.length;i++){if(a.value[i]!==b.value[i])throw Error("Wires octet identity differs");if(++work%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",work,0);}break;
   case"array":if(b.kind!=="array"||a.items.length!==b.items.length)throw Error("Wires array identity differs");owned+=a.items.length*16;artifactSqliteValueBudget(owned,options);for(let i=0;i<a.items.length;i++){pending.push([a.items[i]!,b.items[i]!]);if(++work%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",work,0);}break;
   case"object":if(b.kind!=="object"||a.members.length!==b.members.length)throw Error("Wires member identity differs");owned+=a.members.length*16;artifactSqliteValueBudget(owned,options);for(let i=0;i<a.members.length;i++){if(a.members[i]!.name!==b.members[i]!.name)throw Error("Wires member identity differs");pending.push([a.members[i]!.value,b.members[i]!.value]);if(++work%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",work,0);}break;
  }
  if(++work%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",work,0);
 }
 return[];
}
