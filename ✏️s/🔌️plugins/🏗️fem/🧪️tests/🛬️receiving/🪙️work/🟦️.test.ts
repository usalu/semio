import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "./🧫️fixtures/📋️schema.json";
test("both FEM dimensions preserve selected work, failed prefixes and every owning cause",()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 for(const row of fixture.cases){
  const db=new Database(":memory:");
  db.run("CREATE TABLE actions(sequence INTEGER PRIMARY KEY,units INTEGER,physical INTEGER,failed INTEGER)");
  db.run("CREATE TABLE causes(sequence INTEGER PRIMARY KEY,name TEXT)");
  let outcome=row.admitted?"ok":"denied";
  if(row.admitted)for(const action of row.actions){
   if(action.kind!=="row")expect(action.units).toBe(1);
   db.run("INSERT INTO actions(units,physical,failed) VALUES (?,?,?)",action.units,action.physicalItems,Number(action.fails));
   for(const cause of action.causes)db.run("INSERT INTO causes(name) VALUES (?)",cause);
   if(action.fails){outcome="failure";break;}
  }
  const totals=db.query("SELECT coalesce(sum(units),0) AS work,coalesce(sum(physical),0) AS physicalItems FROM actions").get() as {work:number,physicalItems:number};
  const causes=(db.query("SELECT name FROM causes ORDER BY sequence").all() as {name:string}[]).map(row=>row.name);
  expect({...totals,outcome,causes}).toEqual(row.expected);
  expect(totals.work).not.toBe(row.priorWork);
  db.close();
 }
});
