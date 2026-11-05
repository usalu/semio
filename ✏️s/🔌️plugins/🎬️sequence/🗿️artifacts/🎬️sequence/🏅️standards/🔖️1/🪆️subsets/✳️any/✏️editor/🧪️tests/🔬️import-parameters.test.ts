import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/import-parameters.json";
import schema from "./🧬️schema/import-parameters.json";
test("Sequence parameter vectors satisfy their neutral schema",()=>expect(new Ajv().compile(schema)(fixture)).toBe(true));
for(const row of fixture.cases)test(row.id,()=>{
 const value=JSON.parse(row.wire),actual=value!==null&&typeof value==="object"&&!Array.isArray(value)?value:{value};
 expect(actual).toEqual(row.expected);
 const db=new Database(":memory:");
 try{
  const result=db.query("SELECT CASE WHEN json_type(?1)='object' THEN json(?1) ELSE json_object('value',json(?1)) END AS value").get(row.wire) as {value:string};
  expect(JSON.parse(result.value)).toEqual(row.expected);
 }finally{db.close();}
});
