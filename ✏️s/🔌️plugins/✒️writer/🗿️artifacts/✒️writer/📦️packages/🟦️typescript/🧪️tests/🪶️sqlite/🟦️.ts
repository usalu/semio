/** ✒️ Actual public Writer package and neutral independent SQLite boundary. */
import { writerSnapshotToSqliteDatabase,writerSnapshotFromSqliteDatabase,validateWriterSnapshotSqliteDialect,WRITER_SQLITE_SCHEMA,type WriterSnapshot } from "@semio-tech/writer-writer";
import { exportSqliteDatabase,importSqliteDatabase } from "@semio-tech/framework";
import { Database } from "bun:sqlite";
import { expect,test } from "bun:test";
import fixture from "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures/🔣️.json";
test("Writer public package exposes the complete typed snapshot and independently editable relationships",async()=>{
 const snapshot:WriterSnapshot=fixture.snapshot;
 const database=await writerSnapshotToSqliteDatabase(snapshot);
 expect(WRITER_SQLITE_SCHEMA).toContain("CREATE TABLE writer_document_child");
 const oracle=Database.deserialize(await exportSqliteDatabase(database));
 try{
  for(const sql of fixture.renumberSql)oracle.run(sql);
  expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
  expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);
  const renumbered=await importSqliteDatabase(oracle.serialize());
  expect(await writerSnapshotFromSqliteDatabase(renumbered)).toEqual(snapshot);
  expect(await validateWriterSnapshotSqliteDialect(snapshot,fixture.dialect,renumbered)).toEqual([]);
  oracle.query("UPDATE writer_document SET text=?").run(fixture.editedText);
  const edited=await importSqliteDatabase(oracle.serialize());
  expect(await writerSnapshotFromSqliteDatabase(edited)).toEqual({...snapshot,text:fixture.editedText});
  await expect(validateWriterSnapshotSqliteDialect(snapshot,fixture.dialect,edited)).rejects.toThrow("identity");
 }finally{oracle.close();}
});
