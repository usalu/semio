import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "../../../../../../../🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🧪️tests/parts.json";
import schema from "../../../../../../../🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🧪️tests/parts.schema.json";
test("prepaid parent retains all independent causes beside the original child",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const db=new Database(":memory:");
 try{
  db.exec("CREATE TABLE causes (node INTEGER, ordinal INTEGER, text TEXT)");
  for(const [node,value] of [fixture.leaf,fixture.parent].entries())for(const [ordinal,text] of [value.primary,value.work,value.receipt].entries())db.query("INSERT INTO causes VALUES (?, ?, ?)").run(node,ordinal,text);
  const independent=db.query("SELECT text FROM causes ORDER BY node,ordinal").all().map(row=>(row as {text:string}).text);
  expect(independent).toEqual([fixture.leaf.primary,fixture.leaf.work,fixture.leaf.receipt,fixture.parent.primary,fixture.parent.work,fixture.parent.receipt]);
  expect([fixture.attachAllocatedBytes,fixture.attachReleasedBytes]).toEqual([0,0]);
 }finally{db.close();}
});
