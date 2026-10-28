import {test,expect} from "bun:test";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import law from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/🔁️follow/🧪️tests/🔣️.json";
import schema from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/🔁️follow/🧪️tests/📐️.json";
test("original complete-prefix visibility agrees with SQLite transaction isolation",()=>{
 expect(new Ajv().compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");
 try{
  db.exec("CREATE TABLE committed(value REAL); CREATE TEMP TABLE private_prefix(ordinal INTEGER,value REAL)");
  db.query("INSERT INTO committed VALUES(?)").run(law.committed);
  for(const [ordinal,value] of law.operations.entries()){
   if(ordinal===law.typingRun.start)db.exec("SAVEPOINT typing_run");
   const outcome=db.query("SELECT ?>0 AS accepted").get(value)as{accepted:number};expect(!!outcome.accepted).toBe(law.accepted[ordinal]);
   db.query("INSERT INTO private_prefix SELECT ?,? WHERE ?>0").run(ordinal,value,value);
   if(ordinal===law.typingRun.end){db.exec("ROLLBACK TO typing_run; RELEASE typing_run");expect((db.query("SELECT value FROM private_prefix ORDER BY ordinal DESC LIMIT 1").get()as{value:number}).value).toBe(law.typingRun.restored);}
   expect((db.query("SELECT value FROM committed").get()as{value:number}).value).toBe(law.visibleBeforePublish);
  }
  db.exec("BEGIN; UPDATE committed SET value=(SELECT value FROM private_prefix ORDER BY ordinal DESC LIMIT 1); COMMIT");
  expect((db.query("SELECT value FROM committed").get()as{value:number}).value).toBe(law.visibleAfterPublish);
 }finally{db.close()}
});
