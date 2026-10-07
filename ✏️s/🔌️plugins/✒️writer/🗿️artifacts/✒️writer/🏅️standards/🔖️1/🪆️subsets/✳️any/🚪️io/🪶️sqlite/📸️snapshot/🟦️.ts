/** ✒️ Writer authored strings and persisted composed child identity. */
import type { WriterSnapshot } from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import type { ArtifactDialect } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🟦️.ts";
import type { SqliteDatabase } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import { ArtifactSqliteProjection, artifactSqliteCheckpoint, artifactSqliteTables, artifactSqliteInteger, artifactSqliteText, type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
/** 🏛️ The two handcrafted Writer tables mirror the adjacent SQL asset. */
export const WRITER_SQLITE_SCHEMA=String.raw`CREATE TABLE writer_document (rowid INTEGER PRIMARY KEY, schema TEXT NOT NULL, id TEXT NOT NULL, language_id TEXT NOT NULL, uri TEXT NOT NULL, text TEXT NOT NULL);
CREATE TABLE writer_document_child (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES writer_document(rowid), child_id TEXT NOT NULL, target_artifact_id TEXT NOT NULL, target_artifact_kind TEXT NOT NULL, target_standard TEXT NOT NULL, target_subset TEXT NOT NULL);
`;
/** 📤️ Project only the Writer persisted fields, preserving each child reference component. */
export async function writerSnapshotToSqliteDatabase(snapshot:WriterSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const p=await ArtifactSqliteProjection.create(WRITER_SQLITE_SCHEMA,options);p.checkRowsAdditional(2);
 await p.insert("writer_document",[snapshot.schema,snapshot.id,snapshot.languageId,snapshot.uri,snapshot.text],1n);
 const child=snapshot.document;await p.insert("writer_document_child",[1n,child.childId,child.target.artifactId,child.target.dialect.artifactKind,child.target.dialect.standard,child.target.dialect.subset],1n);return p.finish();
}
/** 📥️ Reconstruct the exact Writer-owned text and child handle without materializing child content. */
export async function writerSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<WriterSnapshot>{
 await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,0);
 const tables=await artifactSqliteTables(database,WRITER_SQLITE_SCHEMA,options);
 if(tables[0]!.length!==1||tables[1]!.length!==1)throw Error("Writer requires exactly its document and fixed child");
 const document=tables[0]![0]!,child=tables[1]![0]!;
 if(artifactSqliteInteger(document,0)!==document.rowid||document.values.length!==6||artifactSqliteInteger(child,0)!==child.rowid||artifactSqliteInteger(child,1)!==document.rowid||child.values.length!==7)throw Error("Writer fixed ownership differs");
 const result={schema:artifactSqliteText(document,1),id:artifactSqliteText(document,2),languageId:artifactSqliteText(document,3),uri:artifactSqliteText(document,4),text:artifactSqliteText(document,5),document:{childId:artifactSqliteText(child,2),target:{artifactId:artifactSqliteText(child,3),dialect:{artifactKind:artifactSqliteText(child,4),standard:artifactSqliteText(child,5),subset:artifactSqliteText(child,6)}}}};
 await artifactSqliteCheckpoint(options,"reconstructSnapshot",2,2);return result;
}
/** 🧭️ Writer owns only its declared standard and wildcard snapshot subset. */
export async function validateWriterSnapshotSqliteDialect(snapshot:WriterSnapshot,dialect:ArtifactDialect,database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<readonly never[]>{
 await artifactSqliteCheckpoint(options,"projectSnapshot",0,0,false);
 if(dialect.artifactKind!=="s.writer.writer"||dialect.standard!=="1"||dialect.subset!=="*")throw Error("Writer does not own this semantic subset");
 const restored=await writerSnapshotFromSqliteDatabase(database,options),a=snapshot.document,b=restored.document;
 if(snapshot.schema!==restored.schema||snapshot.id!==restored.id||snapshot.languageId!==restored.languageId||snapshot.uri!==restored.uri||snapshot.text!==restored.text||a.childId!==b.childId||a.target.artifactId!==b.target.artifactId||a.target.dialect.artifactKind!==b.target.dialect.artifactKind||a.target.dialect.standard!==b.target.dialect.standard||a.target.dialect.subset!==b.target.dialect.subset)throw Error("Writer document identity disagrees with its snapshot");
 return[];
}
