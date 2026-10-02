/// <reference path="./🗄️.d.ts" />
import sql from "./🗄️.sql" with {type:"text"};
import type {SequenceSnapshot} from "../🟦️.ts";
import {ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteInteger,artifactSqliteText,artifactSqliteTextBytes,artifactSqliteCheckpoint,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type {SqliteDatabase,SqliteRow} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
export const SEQUENCE_SQLITE_SCHEMA:string=sql;
function identity(row:SqliteRow,columns:number):void{if(row.values.length!==columns||row.rowid<=0n||artifactSqliteInteger(row,0)!==row.rowid)throw Error("Sequence requires exact fields and positive aliased identities")}
/** 📤️ Projects only persisted parent and child reference fields. */
export async function sequenceSnapshotToSqliteDatabase(snapshot:SequenceSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const out=await ArtifactSqliteProjection.create(sql,options);out.checkRowsAdditional(2);const child=snapshot.content,target=child.target;
 await out.insert("sequence_document",[snapshot.schema],1n);
 await out.insert("sequence_content",[1n,child.childId,target.artifactId,target.dialect.artifactKind,target.dialect.standard,target.dialect.subset]);
 return out.finish();
}
/** 📥️ Reconstructs the mandatory reference after exact singleton and structural parent validation. */
export async function sequenceSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<SequenceSnapshot>{
 const tables=await artifactSqliteTables(database,sql,options),documents=tables[0]!,children=tables[1]!;
 if(documents.length!==1||documents[0]!.rowid!==1n||children.length!==1)throw Error("Sequence requires one document and one content child");
 const document=documents[0]!,child=children[0]!;identity(document,2);identity(child,7);if(artifactSqliteInteger(child,1)!==1n)throw Error("Sequence child requires the document parent");
 const schema=artifactSqliteText(document,1),childId=artifactSqliteText(child,2),artifactId=artifactSqliteText(child,3),artifactKind=artifactSqliteText(child,4),standard=artifactSqliteText(child,5),subset=artifactSqliteText(child,6);
 for(const text of[schema,childId,artifactId,artifactKind,standard,subset])if(artifactSqliteTextBytes(text)>65536)await artifactSqliteCheckpoint(options,"reconstructSnapshot",1,0,false);
 await artifactSqliteCheckpoint(options,"reconstructSnapshot",2,2,false);return{schema,content:{childId,target:{artifactId,dialect:{artifactKind,standard,subset}}}};
}
