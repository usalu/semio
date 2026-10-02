/// <reference path="./🗄️.d.ts" />
import sql from "./🗄️.sql" with {type:"text"};
import type {VcsSnapshot} from "../🟦️.ts";
import {ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteInteger,artifactSqliteText,artifactSqliteTextBytes,artifactSqliteCheckpoint,artifactSqliteOrderedRowsControlled,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type {SqliteDatabase,SqliteRow} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
/** 🌿️ Complete native document fields with an exact signed64 counter. */
export type VcsSqliteSnapshot=Omit<VcsSnapshot,"counter">&{counter:bigint};
export const VCS_SQLITE_SCHEMA:string=sql;
function counter(value:bigint):bigint{if(typeof value!=="bigint"||value< -9223372036854775808n||value>9223372036854775807n)throw Error("VCS counter requires signed64 INTEGER");return value}
function identity(row:SqliteRow,columns:number):void{if(row.values.length!==columns||row.rowid<=0n||artifactSqliteInteger(row,0)!==row.rowid)throw Error("VCS requires exact fields and positive aliased identities")}
/** 📤️ Projects scalar document fields and independently ordered textual tags. */
export async function vcsSnapshotToSqliteDatabase(snapshot:VcsSqliteSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const out=await ArtifactSqliteProjection.create(sql,options);out.checkRowsAdditional(snapshot.tags.length+1);
 await out.insert("vcs_document",[snapshot.schema,snapshot.title,counter(snapshot.counter),snapshot.notes,snapshot.status],1n);
 for(let ordinal=0;ordinal<snapshot.tags.length;ordinal++)await out.insert("vcs_tag",[1n,BigInt(ordinal),snapshot.tags[ordinal]!]);
 return out.finish();
}
/** 📥️ Reconstructs the complete document after validating structural parents and dense ordinals. */
export async function vcsSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<VcsSqliteSnapshot>{
 const tables=await artifactSqliteTables(database,sql,options),documents=tables[0]!,rows=tables[1]!;
 if(documents.length!==1||documents[0]!.rowid!==1n)throw Error("VCS requires one document identity");const document=documents[0]!;identity(document,6);
 const schema=artifactSqliteText(document,1),title=artifactSqliteText(document,2),count=counter(artifactSqliteInteger(document,3)),notes=artifactSqliteText(document,4),status=artifactSqliteText(document,5);
 for(const text of [schema,title,notes,status])if(artifactSqliteTextBytes(text)>65536)await artifactSqliteCheckpoint(options,"reconstructSnapshot",1,0,false);
 const identities=new Set<bigint>();for(let index=0;index<rows.length;index++){const row=rows[index]!;identity(row,4);if(artifactSqliteInteger(row,1)!==1n||identities.has(row.rowid))throw Error("VCS tag requires one document parent and a unique identity");artifactSqliteText(row,3);identities.add(row.rowid);if(index%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",index,rows.length,index>0)}
 const ordered=await artifactSqliteOrderedRowsControlled(rows,2,options),tags:string[]=[];
 for(let index=0;index<ordered.length;index++){const text=artifactSqliteText(ordered[index]!,3);if(artifactSqliteTextBytes(text)>65536)await artifactSqliteCheckpoint(options,"reconstructSnapshot",index+1,0,false);tags.push(text);if((index+1)%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",index+1,ordered.length)}
 await artifactSqliteCheckpoint(options,"reconstructSnapshot",ordered.length+1,ordered.length+1,false);return{schema,title,counter:count,notes,status,tags};
}

