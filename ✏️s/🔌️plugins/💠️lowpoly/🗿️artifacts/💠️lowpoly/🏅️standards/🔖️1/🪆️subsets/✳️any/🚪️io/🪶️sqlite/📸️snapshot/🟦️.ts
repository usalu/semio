/** 💠️ Literal Lowpoly entities preserve exact native binary32 fields and intrinsic paint octets. */
import type{LowpolySnapshot}from"../../../🧬️schema/📸️snapshot/🟦️.ts";
import type{LowpolyObject,LowpolyPaintLayer}from"../../../🧬️schema/🟦️.ts";
import{parseLowpolyMeshState}from"../../../🧬️schema/🕸️mesh/🟦️.ts";
import meshSql from"./🕸️mesh/🗄️.sql"with{type:"text"};
import{projectLowpolyMesh,lowpolyMeshRows,LowpolyMeshRows}from"./🕸️mesh/🟦️.ts";
import{sqliteOperation}from"../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import type{ArtifactDialect}from"../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts";
import type{SqliteDatabase,SqliteRow}from"../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import{ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteCheckpoint,artifactSqliteValueBudget,artifactSqliteInteger as integer,artifactSqliteText as text,artifactSqliteBoolean as boolean,artifactSqliteOrderedRowsControlled,type ArtifactSqliteOptions}from"../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import {encodeIeee754Cells,readBinary32,ieee754CellByteLength} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import {parseBinary32,type Binary32} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
export const LOWPOLY_SQLITE_SCHEMA=String.raw`CREATE TABLE lowpoly_document (id INTEGER PRIMARY KEY, schema TEXT NOT NULL);
CREATE TABLE lowpoly_object (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES lowpoly_document(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), object_id TEXT NOT NULL, name TEXT NOT NULL, smooth_shading INTEGER NOT NULL CHECK(smooth_shading IN (0,1)), mesh_content TEXT NOT NULL);
CREATE TABLE lowpoly_transform (id INTEGER PRIMARY KEY REFERENCES lowpoly_object(id), position_x REAL, position_y REAL, position_z REAL, rotation_x REAL, rotation_y REAL, rotation_z REAL, scale_x REAL, scale_y REAL, scale_z REAL, position_x_ieee754_bits INTEGER NOT NULL CHECK(position_x_ieee754_bits BETWEEN 0 AND 4294967295), position_x_numeric_class TEXT NOT NULL, position_y_ieee754_bits INTEGER NOT NULL CHECK(position_y_ieee754_bits BETWEEN 0 AND 4294967295), position_y_numeric_class TEXT NOT NULL, position_z_ieee754_bits INTEGER NOT NULL CHECK(position_z_ieee754_bits BETWEEN 0 AND 4294967295), position_z_numeric_class TEXT NOT NULL, rotation_x_ieee754_bits INTEGER NOT NULL CHECK(rotation_x_ieee754_bits BETWEEN 0 AND 4294967295), rotation_x_numeric_class TEXT NOT NULL, rotation_y_ieee754_bits INTEGER NOT NULL CHECK(rotation_y_ieee754_bits BETWEEN 0 AND 4294967295), rotation_y_numeric_class TEXT NOT NULL, rotation_z_ieee754_bits INTEGER NOT NULL CHECK(rotation_z_ieee754_bits BETWEEN 0 AND 4294967295), rotation_z_numeric_class TEXT NOT NULL, scale_x_ieee754_bits INTEGER NOT NULL CHECK(scale_x_ieee754_bits BETWEEN 0 AND 4294967295), scale_x_numeric_class TEXT NOT NULL, scale_y_ieee754_bits INTEGER NOT NULL CHECK(scale_y_ieee754_bits BETWEEN 0 AND 4294967295), scale_y_numeric_class TEXT NOT NULL, scale_z_ieee754_bits INTEGER NOT NULL CHECK(scale_z_ieee754_bits BETWEEN 0 AND 4294967295), scale_z_numeric_class TEXT NOT NULL);
CREATE TABLE lowpoly_mesh_child (id INTEGER PRIMARY KEY REFERENCES lowpoly_object(id), child_id TEXT NOT NULL, artifact_id TEXT NOT NULL, artifact_kind TEXT NOT NULL, standard TEXT NOT NULL, subset TEXT NOT NULL);
CREATE TABLE lowpoly_paint_layer (id INTEGER PRIMARY KEY, object_id INTEGER NOT NULL REFERENCES lowpoly_object(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), name TEXT NOT NULL, visible INTEGER NOT NULL CHECK(visible IN (0,1)), opacity REAL, blend_mode TEXT NOT NULL, opacity_ieee754_bits INTEGER NOT NULL CHECK(opacity_ieee754_bits BETWEEN 0 AND 4294967295), opacity_numeric_class TEXT NOT NULL);
CREATE TABLE lowpoly_paint_octet (id INTEGER PRIMARY KEY, layer_id INTEGER NOT NULL REFERENCES lowpoly_paint_layer(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), value INTEGER NOT NULL CHECK(value BETWEEN 0 AND 255));
`+meshSql;
const TRANSFORM=[{index:1,width:32},{index:2,width:32},{index:3,width:32},{index:4,width:32},{index:5,width:32},{index:6,width:32},{index:7,width:32},{index:8,width:32},{index:9,width:32}]as const;
const OPACITY=[{index:5,width:32}]as const;
type Phase="projectSnapshot"|"reconstructSnapshot";
async function unicode(value:string,options:ArtifactSqliteOptions,phase:Phase):Promise<number>{
 if(typeof value!=="string")throw Error("Lowpoly field requires TEXT");let bytes=0,nextCheckpoint=65536;for(let index=0;index<value.length;index++){const c=value.charCodeAt(index);if(c>=0xd800&&c<=0xdbff){const next=value.charCodeAt(++index);if(!(next>=0xdc00&&next<=0xdfff))throw Error("Lowpoly field is not Unicode scalar text");bytes+=4;}else if(c>=0xdc00&&c<=0xdfff)throw Error("Lowpoly field is not Unicode scalar text");else bytes+=c<128?1:c<2048?2:3;if(index+1>=nextCheckpoint){artifactSqliteValueBudget(bytes,options);await artifactSqliteCheckpoint(options,phase,index+1,value.length);nextCheckpoint=index+1+65536;}}artifactSqliteValueBudget(bytes,options);return bytes;
}
function scalar(value:Binary32):number{return ieee754CellByteLength(parseBinary32(value),32);}
async function admission(snapshot:LowpolySnapshot,options:ArtifactSqliteOptions):Promise<number>{
 let rows=1,bytes=await unicode(snapshot.schema,options,"projectSnapshot");if(!Array.isArray(snapshot.objects))throw Error("Lowpoly objects require ordered occurrences");
 for(const object of snapshot.objects){
  rows+=2;bytes+=await unicode(object.id,options,"projectSnapshot")+await unicode(object.name,options,"projectSnapshot")+await unicode(object.meshContent,options,"projectSnapshot");if(typeof object.smoothShading!=="boolean")throw Error("Lowpoly shading requires boolean");
  if(object.meshState!==null)rows+=lowpolyMeshRows(object.meshState);
  const t=object.transform;if(t.position.length!==3||t.rotation.length!==3||t.scale.length!==3)throw Error("Lowpoly transform requires three components");
  bytes+=scalar(t.position[0])+scalar(t.position[1])+scalar(t.position[2])+scalar(t.rotation[0])+scalar(t.rotation[1])+scalar(t.rotation[2])+scalar(t.scale[0])+scalar(t.scale[1])+scalar(t.scale[2]);
  if(object.mesh!==null){rows++;const child=object.mesh;bytes+=await unicode(child.childId,options,"projectSnapshot")+await unicode(child.target.artifactId,options,"projectSnapshot")+await unicode(child.target.dialect.artifactKind,options,"projectSnapshot")+await unicode(child.target.dialect.standard,options,"projectSnapshot")+await unicode(child.target.dialect.subset,options,"projectSnapshot");}
  if(!Array.isArray(object.paintLayers))throw Error("Lowpoly paint layers require ordered occurrences");
  for(const layer of object.paintLayers){if(!(layer.pixels instanceof Uint8Array)||typeof layer.visible!=="boolean")throw Error("Lowpoly paint requires octets and boolean visibility");rows+=1+layer.pixels.length;bytes+=await unicode(layer.name,options,"projectSnapshot")+await unicode(layer.blendMode,options,"projectSnapshot")+scalar(layer.opacity)+layer.pixels.length*32;}
  if(!Number.isSafeInteger(rows)||rows>(options.maxRows??1000000))throw Error("Lowpoly row limit");artifactSqliteValueBudget(bytes+rows*192,options);await artifactSqliteCheckpoint(options,"projectSnapshot",0,rows,false);
 }if(rows>(options.maxRows??1000000))throw Error("Lowpoly row limit");artifactSqliteValueBudget(bytes+rows*192,options);return rows;
}
/** 📤️ Admit complete borrowed fields before allocating authored entity rows. */
export async function lowpolySnapshotToSqliteDatabase(snapshot:LowpolySnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 snapshot={...snapshot,objects:snapshot.objects.map(object=>({...object,meshState:object.meshState===null?null:parseLowpolyMeshState(object.meshState)}))};options=sqliteOperation(options);
 await artifactSqliteCheckpoint(options,"projectSnapshot",0,0);const total=await admission(snapshot,options),p=await ArtifactSqliteProjection.create(LOWPOLY_SQLITE_SCHEMA,options);p.checkRowsAdditional(total);let completed=0;
 const step=async()=>{if(++completed%256===0||completed===total)await artifactSqliteCheckpoint(options,"projectSnapshot",completed,total);};
 await p.insert("lowpoly_document",[snapshot.schema],1n);await step();let objectId=0n,layerId=0n,octetId=0n;
 for(let ordinal=0;ordinal<snapshot.objects.length;ordinal++){const o=snapshot.objects[ordinal]!,id=++objectId;await p.insert("lowpoly_object",[1n,BigInt(ordinal),o.id,o.name,o.smoothShading?1n:0n,o.meshContent],id);await step();
 const t=o.transform,cells=encodeIeee754Cells([id,t.position[0],t.position[1],t.position[2],t.rotation[0],t.rotation[1],t.rotation[2],t.scale[0],t.scale[1],t.scale[2]],TRANSFORM,options.maxColumns);await p.insert("lowpoly_transform",cells.slice(1),id);await step();
 if(o.mesh!==null){const c=o.mesh;await p.insert("lowpoly_mesh_child",[c.childId,c.target.artifactId,c.target.dialect.artifactKind,c.target.dialect.standard,c.target.dialect.subset],id);await step();}
 if(o.meshState!==null)await projectLowpolyMesh(o.meshState,id,async(name,cells,key)=>{const result=await p.insert(name,cells,key);await step();return result;});
 for(let index=0;index<o.paintLayers.length;index++){const layer=o.paintLayers[index]!,key=++layerId;const encoded=encodeIeee754Cells([key,id,BigInt(index),layer.name,layer.visible?1n:0n,layer.opacity,layer.blendMode],OPACITY,options.maxColumns);await p.insert("lowpoly_paint_layer",encoded.slice(1),key);await step();
 for(let byte=0;byte<layer.pixels.length;byte++){await p.insert("lowpoly_paint_octet",[key,BigInt(byte),BigInt(layer.pixels[byte]!)],++octetId);await step();}}
 }return p.finish();
}
function keyed(rows:readonly SqliteRow[],width:number):Map<bigint,SqliteRow>{const result=new Map<bigint,SqliteRow>();for(const row of rows){if(row.values.length!==width||integer(row,0)!==row.rowid||result.has(row.rowid))throw Error("Lowpoly entity identity or width differs");result.set(row.rowid,row);}return result;}
function groups(rows:readonly SqliteRow[],column:number,parents:Map<bigint,SqliteRow>):Map<bigint,SqliteRow[]>{const result=new Map<bigint,SqliteRow[]>();for(const row of rows){const owner=integer(row,column);if(!parents.has(owner))throw Error("Lowpoly entity has unknown owner");let list=result.get(owner);if(!list){list=[];result.set(owner,list);}list.push(row);}return result;}
/** 📥️ Reconstruct every explicit occurrence with bounded lookup and exact word identity. */
export async function lowpolySnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<LowpolySnapshot>{
 database={...database,tables:database.tables.map(table=>({...table,rows:table.rows.map(row=>({rowid:row.rowid,values:[...row.values]}))}))};
 options=sqliteOperation(options);const rows=await artifactSqliteTables(database,LOWPOLY_SQLITE_SCHEMA,options),total=rows.reduce((n,r)=>n+r.length,0);artifactSqliteValueBudget(total*192,options);
 const managed=await LowpolyMeshRows.create(database.tables.filter(table=>table.name.startsWith("lowpoly_mesh_")&&table.name!=="lowpoly_mesh_child"),sqliteOperation(options));
 const documents=keyed(rows[0]!,2),objects=keyed(rows[1]!,7),transforms=keyed(rows[2]!,28),children=keyed(rows[3]!,6),layers=keyed(rows[4]!,9);keyed(rows[5]!,4);
 if(documents.size!==1||transforms.size!==objects.size)throw Error("Lowpoly requires one document and one transform per object");for(const id of transforms.keys())if(!objects.has(id))throw Error("Lowpoly transform owner differs");for(const id of children.keys())if(!objects.has(id))throw Error("Lowpoly child owner differs");
 const document=documents.values().next().value!;for(const o of objects.values())if(integer(o,1)!==document.rowid)throw Error("Lowpoly object document differs");
 const layerGroups=groups(rows[4]!,1,objects),octetGroups=groups(rows[5]!,1,layers);let completed=1,bytes=total*192;
 const copy=async(row:SqliteRow,index:number):Promise<string>=>{const value=text(row,index);bytes+=await unicode(value,options,"reconstructSnapshot");artifactSqliteValueBudget(bytes,options);return value;};
 const step=async()=>{if(++completed%256===0||completed===total)await artifactSqliteCheckpoint(options,"reconstructSnapshot",completed,total);};
 const result:LowpolySnapshot={schema:await copy(document,1),objects:[]};
 for(const row of await artifactSqliteOrderedRowsControlled(rows[1]!,2,options)){const t=transforms.get(row.rowid)!,object:LowpolyObject={id:await copy(row,3),name:await copy(row,4),smoothShading:boolean(row,5),meshContent:await copy(row,6),meshState:await managed.mesh(row.rowid),transform:{position:[readBinary32(t,1,TRANSFORM),readBinary32(t,2,TRANSFORM),readBinary32(t,3,TRANSFORM)],rotation:[readBinary32(t,4,TRANSFORM),readBinary32(t,5,TRANSFORM),readBinary32(t,6,TRANSFORM)],scale:[readBinary32(t,7,TRANSFORM),readBinary32(t,8,TRANSFORM),readBinary32(t,9,TRANSFORM)]},mesh:null,paintLayers:[]};await step();await step();
 const child=children.get(row.rowid);if(child){object.mesh={childId:await copy(child,1),target:{artifactId:await copy(child,2),dialect:{artifactKind:await copy(child,3),standard:await copy(child,4),subset:await copy(child,5)}}};await step();}
 for(const layer of await artifactSqliteOrderedRowsControlled(layerGroups.get(row.rowid)??[],2,options)){const octets=await artifactSqliteOrderedRowsControlled(octetGroups.get(layer.rowid)??[],2,options);bytes+=octets.length;artifactSqliteValueBudget(bytes,options);const value:LowpolyPaintLayer={name:await copy(layer,3),visible:boolean(layer,4),opacity:readBinary32(layer,5,OPACITY),blendMode:await copy(layer,6),pixels:new Uint8Array(octets.length)};await step();for(let index=0;index<octets.length;index++){const byte=integer(octets[index]!,3);if(byte<0n||byte>255n)throw Error("Lowpoly paint sample exceeds octet width");value.pixels[index]=Number(byte);await step();}object.paintLayers.push(value);}result.objects.push(object);
 }await managed.finish();completed=total;await artifactSqliteCheckpoint(options,"reconstructSnapshot",completed,total);return result;
}
/** 🧭️ Exact declaration and logical state remain independent of surrogate SQL identities. */
export async function validateLowpolySnapshotSqliteDialect(snapshot:LowpolySnapshot,dialect:ArtifactDialect,database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<void>{
 await artifactSqliteCheckpoint(options,"projectSnapshot",0,0,false);if(dialect.artifactKind!=="s.lowpoly.lowpoly"||dialect.standard!=="1"||dialect.subset!=="*")throw Error("Lowpoly does not own this snapshot dialect");
 const actual=await lowpolySnapshotFromSqliteDatabase(database,options),a=await lowpolySnapshotToSqliteDatabase(snapshot,options),b=await lowpolySnapshotToSqliteDatabase(actual,options);
 for(let i=0;i<a.tables.length;i++){const left=a.tables[i]!.rows,right=b.tables[i]!.rows;if(left.length!==right.length)throw Error("Lowpoly logical entity count differs");for(let j=0;j<left.length;j++){const l=left[j]!.values,r=right[j]!.values;if(l.length!==r.length||l.some((v,k)=>v instanceof Uint8Array?r[k] instanceof Uint8Array&&(v.length!==(r[k]as Uint8Array).length||v.some((byte,index)=>byte!==(r[k]as Uint8Array)[index]))||!(r[k] instanceof Uint8Array):v!==r[k]))throw Error("Lowpoly logical state differs");if(j%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",j,left.length);}}
}
