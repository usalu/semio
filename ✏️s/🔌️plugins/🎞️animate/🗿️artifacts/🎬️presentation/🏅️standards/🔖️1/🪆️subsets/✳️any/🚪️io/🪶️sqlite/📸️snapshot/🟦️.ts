/// <reference path="./🗄️.d.ts" />
import sql from "./🗄️.sql" with {type:"text"};
import type {PresentationSnapshot} from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import {encodeIeee754Cells,readBinary64,ieee754IsNull,type Ieee754Column} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import {parseBinary64,type Binary64} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import {ArtifactSqliteProjection,artifactSqliteCheckpoint,artifactSqliteTables,artifactSqliteInteger,artifactSqliteText,artifactSqliteDocument,artifactSqliteOrderedRowsControlled,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type {SqliteDatabase,SqliteRow} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
/** 📐️ Full binary64 geometry words at the owned snapshot boundary. */
export interface PresentationSqliteFrame {x:Binary64;y:Binary64;width:Binary64;height:Binary64}
/** 🎬️ Presentation fields with explicit geometry identity independent of JavaScript NaN conversion. */
export type PresentationSqliteSnapshot=Omit<PresentationSnapshot,"source"|"tiles">&{source:{src:string;kind:string;frame:PresentationSqliteFrame;sourceAspect?:Binary64;pdfPage?:number};tiles:{id:string;name:string;crop:PresentationSqliteFrame}[]};
export const PRESENTATION_SQLITE_SCHEMA:string=sql;
const SOURCE:readonly Ieee754Column[]=[{index:4,width:64},{index:5,width:64},{index:6,width:64},{index:7,width:64},{index:8,width:64}];
const TILE:readonly Ieee754Column[]=[{index:5,width:64},{index:6,width:64},{index:7,width:64},{index:8,width:64}];
function record(value:unknown,required:readonly string[],optional:readonly string[]=[]):Record<string,unknown>{if(value===null||typeof value!=="object"||Array.isArray(value)||Object.keys(value).some(key=>!required.includes(key)&&!optional.includes(key))||required.some(key=>!Object.hasOwn(value,key)))throw Error("Presentation requires its exact native fields");return value as Record<string,unknown>}
function text(value:unknown):string{if(typeof value!=="string")throw Error("Presentation requires native TEXT");return value}
function word(value:unknown):Binary64{record(value,["bits"]);return parseBinary64(value)}
function frame(value:unknown):PresentationSqliteFrame{const r=record(value,["x","y","width","height"]);return{x:word(r.x),y:word(r.y),width:word(r.width),height:word(r.height)}}
function page(value:unknown):bigint{if(typeof value!=="number"||!Number.isSafeInteger(value)||value<0||value>4294967295)throw Error("Presentation page exceeds u32 width");return BigInt(value)}
function child(value:unknown):PresentationSnapshot["presentation"]{const r=record(value,["childId","target"]),target=record(r.target,["artifactId","dialect"]),dialect=record(target.dialect,["artifactKind","standard","subset"]);return{childId:text(r.childId),target:{artifactId:text(target.artifactId),dialect:{artifactKind:text(dialect.artifactKind),standard:text(dialect.standard),subset:text(dialect.subset)}}}}
function identity(row:SqliteRow,columns:number):void{if(row.values.length!==columns||row.rowid<=0n||artifactSqliteInteger(row,0)!==row.rowid||artifactSqliteInteger(row,1)!==1n)throw Error("Presentation requires complete positive entities attached to its document")}
function geometry(row:SqliteRow,start:number,columns:readonly Ieee754Column[]):PresentationSqliteFrame{return{x:readBinary64(row,start,columns),y:readBinary64(row,start+1,columns),width:readBinary64(row,start+2,columns),height:readBinary64(row,start+3,columns)}}
/** 📤️ Projects explicit source geometry, ordered crops and the two literal owned child slots. */
export async function presentationSnapshotToSqliteDatabase(snapshot:PresentationSqliteSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const out=await ArtifactSqliteProjection.create(sql,options);const root=record(snapshot,["schema","source","tiles","presentation","animation"]);if(!Array.isArray(root.tiles))throw Error("Presentation tiles require an ordered collection");out.checkRowsAdditional(4+root.tiles.length);
 const source=record(root.source,["src","kind","frame"],["sourceAspect","pdfPage"]),sourceFrame=frame(source.frame),aspect=source.sourceAspect===undefined?null:word(source.sourceAspect),pdfPage=source.pdfPage===undefined?null:page(source.pdfPage);
 await out.insert("presentation_document",[text(root.schema)],1n);
 await out.insert("presentation_source",encodeIeee754Cells([1n,1n,text(source.src),text(source.kind),sourceFrame.x,sourceFrame.y,sourceFrame.width,sourceFrame.height,aspect,pdfPage],SOURCE,options.maxColumns).slice(1),1n);
 for(let ordinal=0;ordinal<root.tiles.length;ordinal++){const tile=record(root.tiles[ordinal],["id","name","crop"]),crop=frame(tile.crop);await out.insert("presentation_tile",encodeIeee754Cells([1n,1n,BigInt(ordinal),text(tile.id),text(tile.name),crop.x,crop.y,crop.width,crop.height],TILE,options.maxColumns).slice(1));}
 for(const slot of["presentation","animation"]as const){const c=child(root[slot]),t=c.target;await out.insert("presentation_child",[1n,slot,c.childId,t.artifactId,t.dialect.artifactKind,t.dialect.standard,t.dialect.subset]);}return out.finish();
}
/** 📥️ Validates the exact ownership graph and restores raw geometry identity and optional presence. */
export async function presentationSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<PresentationSqliteSnapshot>{
 const tables=await artifactSqliteTables(database,sql,options),document=artifactSqliteDocument(tables[0]!);if(document.values.length!==2||tables[1]!.length!==1||tables[1]![0]!.rowid!==1n||tables[3]!.length!==2)throw Error("Presentation requires one document, one source and its two child slots");
 const row=tables[1]![0]!;identity(row,20);const source:PresentationSqliteSnapshot["source"]={src:artifactSqliteText(row,2),kind:artifactSqliteText(row,3),frame:geometry(row,4,SOURCE)};
 if(!ieee754IsNull(row,8,SOURCE))source.sourceAspect=readBinary64(row,8,SOURCE);if(row.values[9]!==null){const value=artifactSqliteInteger(row,9);if(value<0n||value>4294967295n)throw Error("Presentation page exceeds u32 width");source.pdfPage=Number(value)}
 const children=new Map<string,PresentationSnapshot["presentation"]>();for(const item of tables[3]!){identity(item,8);const slot=artifactSqliteText(item,2);if((slot!=="presentation"&&slot!=="animation")||children.has(slot))throw Error("Presentation requires exactly its two distinct child slots");children.set(slot,{childId:artifactSqliteText(item,3),target:{artifactId:artifactSqliteText(item,4),dialect:{artifactKind:artifactSqliteText(item,5),standard:artifactSqliteText(item,6),subset:artifactSqliteText(item,7)}}})}
 const ordered=await artifactSqliteOrderedRowsControlled(tables[2]!,2,options),tiles:PresentationSqliteSnapshot["tiles"]=[];for(let ordinal=0;ordinal<ordered.length;ordinal++){if(ordinal%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",ordinal,ordered.length);const item=ordered[ordinal]!;identity(item,17);tiles.push({id:artifactSqliteText(item,3),name:artifactSqliteText(item,4),crop:geometry(item,5,TILE)})}
 await artifactSqliteCheckpoint(options,"reconstructSnapshot",ordered.length+4,ordered.length+4,false);return{schema:artifactSqliteText(document,1),source,tiles,presentation:children.get("presentation")!,animation:children.get("animation")!};
}

