import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";

test("scalar close-start work matches the independent SQLite state machine", () => {
 const law=JSON.parse(readFileSync("🧰️framework/🔨️modules/🌱️value/📽️projection/🎟️work/🧪️tests/🧫️fixtures/🔣️.json","utf8"));
 const db=new Database(":memory:");
 const query=db.query("select case when ?1=0 then 1 else 0 end as work");
 expect((query.get(0) as {work:number}).work).toBe(law.expected.closeStartCpu);
 expect((query.get(1) as {work:number}).work).toBe(law.expected.repeatedCloseStartCpu);
 db.close();
});
