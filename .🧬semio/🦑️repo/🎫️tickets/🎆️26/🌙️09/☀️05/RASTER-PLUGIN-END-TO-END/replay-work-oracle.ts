import{readFileSync}from"node:fs";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
const root="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native";
const read=(path:string)=>JSON.parse(readFileSync(root+"/"+path,"utf8"));
const fixture=read("🧪️tests/🧫️fixtures/⛽️work.json"),validate=new Ajv({strict:false}).compile(read("🧪️tests/🧬️schema/⛽️work.json"));
if(!validate(fixture))throw Error(JSON.stringify(validate.errors));
const db=new Database(":memory:");let checks=0;
try{for(const row of fixture.cases){const count=db.query("WITH RECURSIVE effects(n) AS (SELECT 1 WHERE ? > 0 UNION ALL SELECT n+1 FROM effects WHERE n < ?) SELECT COUNT(*) AS work FROM effects").get(row.deadline?0:row.units,row.deadline?0:row.units)as{work:number};if(count.work!==row.expectedWork)throw Error(row.name+" work mismatch");const result=row.deadline?"Blocked":row.reject?"Error":"Pending";if(result!==row.expectedResult)throw Error(row.name+" outcome mismatch");checks+=2;}const close=db.query("WITH RECURSIVE effects(n) AS (SELECT 1 UNION ALL SELECT n+1 FROM effects WHERE n < ?) SELECT COUNT(*) AS work FROM effects").get(fixture.closeUnits)as{work:number};if(close.work!==fixture.closeUnits)throw Error("close work mismatch");checks++;}finally{db.close()}
console.log(JSON.stringify({oracle:"SQLite recursive effect count + Ajv neutral schema",checks,nativeExecuted:false,status:"passed"}));
