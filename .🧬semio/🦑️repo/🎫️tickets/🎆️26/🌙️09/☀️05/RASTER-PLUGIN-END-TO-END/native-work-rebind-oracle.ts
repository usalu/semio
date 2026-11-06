import Ajv from "ajv";
import { Database } from "bun:sqlite";
const root="🧰️framework/🔨️modules/🧵️job/⏱️context/🛬️decode/🎟️work/🧪️tests";
const law=await Bun.file(`${root}/case.json`).json();
const schema=await Bun.file(`${root}/case.schema.json`).json();
const valid=new Ajv({strict:true}).compile(schema);if(!valid(law))throw new Error(JSON.stringify(valid.errors));
const db=new Database(":memory:");try{db.run("create table owner(reserved integer not null, actual integer not null)");db.run("insert into owner values (?,0)",law.rebindWork);const pending=db.query("select reserved>0 as refused from owner").get() as {refused:number};if(pending.refused!==1)throw new Error("pending close accepted");db.run("update owner set actual=actual+reserved,reserved=0");const settled=db.query("select reserved,actual from owner").get() as {reserved:number,actual:number};if(settled.reserved!==0||settled.actual!==law.rebindWork||settled.actual!==law.unwindPaidWork)throw new Error("native work rebind mismatch");}finally{db.close()}
console.log("native work rebind oracle: 1 case passed");
