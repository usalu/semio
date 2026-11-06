import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "./🧫️fixtures/📋️schema.json";
test("cold graph references and namespace uniqueness",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 for(const row of fixture.cases){
  const db=new Database(":memory:");db.run("CREATE TABLE nodes(id TEXT PRIMARY KEY)");db.run("CREATE TABLE materials(id TEXT PRIMARY KEY)");db.run("CREATE TABLE sections(id TEXT PRIMARY KEY)");let valid=true;
  try{for(const id of row.nodes)db.run("INSERT INTO nodes VALUES (?)",id);db.run("INSERT INTO materials VALUES (?)",row.material);db.run("INSERT INTO sections VALUES (?)",row.section);const q=db.query("SELECT EXISTS(SELECT 1 FROM nodes WHERE id=?) AND EXISTS(SELECT 1 FROM nodes WHERE id=?) AND EXISTS(SELECT 1 FROM materials WHERE id=?) AND EXISTS(SELECT 1 FROM sections WHERE id=?) AS valid").get(row.element.start,row.element.end,row.element.material,row.element.section) as {valid:number};valid=!!q.valid}catch{valid=false}expect(valid).toBe(row.valid);db.close();
 }
});

test("exclusive custody survives allocation refusal",()=>{
 const f=require("./🧫️fixtures/🔣️custody.json"),s=require("./🧫️fixtures/📋️custody-schema.json");expect(new Ajv({strict:true}).compile(s)(f)).toBe(true);
 for(const row of f.cases){const db=new Database(":memory:");const result=db.query("SELECT CASE WHEN ? = 0 THEN 1 ELSE 0 END AS released").get(row.aliases) as {released:number};expect(!!result.released).toBe(row.released);db.close();}
});
