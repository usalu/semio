import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import fixture from "./🧫️fixtures/🔣️.json";

test("no migrated call site asks the host disk about a partition root any more",()=>{
 for(const row of fixture.migrated){
  const text=readFileSync(resolve(import.meta.dir,"../../../../../../../../../../../",row.path),"utf8");
  expect(text.match(/(?<![A-Za-z0-9_.])statfs(?:Sync)?\(/gu)??[],row.path).toEqual([]);expect(text).toContain(row.uses);
 }
});
