import{expect,test}from"bun:test";
import Ajv from"ajv";
import fixture from"../../🧫️fixtures/🪶️sqlite/🔣️.json";
import schema from"../../🔣️.json";
import*as snapshot from"../../🟦️.ts";

test("EN1991 complete neutral state agrees with independent schema admission",()=>{expect(new Ajv({strict:false}).validate(schema,fixture)).toBe(true)});
test("EN1991 publicly exposes both complete owned relational directions",()=>{expect(Object.hasOwn(snapshot,"en1991SnapshotToSqliteDatabase")).toBe(true);expect(Object.hasOwn(snapshot,"en1991SnapshotFromSqliteDatabase")).toBe(true)});
