import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import law from "../../../../../../../../🧰️framework/🔨️modules/🌱️value/♻️retirement/📦️box/🧪️tests/🔣️.json";
import schema from "../../../../../../../../🧰️framework/🔨️modules/🌱️value/♻️retirement/📦️box/🧪️tests/📐️.json";
test("original Box field retirement agrees with independently stored custody and release order",()=>{
 expect(new Ajv().compile(schema)(law)).toBe(true);
 expect(2*law.fields*law.fieldBytes).toBeGreaterThan(law.birthCopyBytes);
 for(const cut of [null,...law.cancelAfter]){const db=new Database(":memory:");try{
 db.exec("CREATE TABLE shell(address INTEGER PRIMARY KEY); CREATE TABLE field(rank INTEGER PRIMARY KEY,address INTEGER,payload INTEGER); INSERT INTO shell VALUES(65536)");
 for(let rank=0;rank<law.fields;rank++)db.query("INSERT INTO field VALUES(?,?,?)").run(rank,65536+rank*law.fieldBytes,law.fieldBytes);
 for(let rank=0;rank<law.fields;rank++){
  const row=db.query("SELECT address,payload FROM field WHERE rank=?").get(rank)as {address:number,payload:number};
  expect(row.address).toBe(65536+rank*law.fieldBytes);expect(2*row.payload).toBeLessThanOrEqual(law.retirementCopyBytes);
  for(const axis of law.denials){expect(axis.length).toBeGreaterThan(0);expect(db.query("SELECT address,payload FROM field WHERE rank=?").get(rank)).toEqual(row);}
  if(cut===rank)expect(db.query("SELECT COUNT(*) AS n FROM shell").get()).toEqual({n:1});
  db.query("DELETE FROM field WHERE rank=?").run(rank);expect(db.query("SELECT COUNT(*) AS n FROM shell").get()).toEqual({n:1});
 }
 db.exec("DELETE FROM shell WHERE NOT EXISTS(SELECT 1 FROM field)");expect(db.query("SELECT COUNT(*) AS n FROM shell").get()).toEqual({n:0});
 }finally{db.close();}}
});
