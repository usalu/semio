import{readFileSync}from"node:fs";
import Ajv from"ajv/dist/2020.js";
import{Database}from"bun:sqlite";
const base="🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🎟️context/🧪️tests/🧫️fixtures/";
const law=JSON.parse(readFileSync(base+"🧭️source-drain.json","utf8"));
const check=new Ajv({strict:true}).compile(JSON.parse(readFileSync(base+"🧭️source-drain.schema.json","utf8")));
if(!check(law))throw Error(JSON.stringify(check.errors));
for(const row of law.closeWorkCases){
 const db=new Database(":memory:");db.run("CREATE TABLE primitive(work INTEGER,physical INTEGER,error INTEGER); INSERT INTO primitive VALUES(0,0,0),(1,1,0),(0,0,1),(1,0,1)");
 let fuel=law.work.fuel,spent=0;
 for(const raw of db.query("SELECT * FROM primitive").all()){const parent={work:raw.work,physical:raw.physical,error:raw.error};spent+=parent.work;fuel-=parent.work;if(parent.work!==raw.work||parent.physical!==raw.physical||parent.error!==raw.error)throw Error("raw child ownership changed")}
 const sql=db.query("SELECT SUM(work) AS work,SUM(physical) AS physical FROM primitive").get();
 if(spent!==sql.work||fuel+sql.work!==law.work.fuel||row.financialItems!==512||row.primitiveItems!==1)throw Error("original close work renewed or guessed");
 db.close();
}
console.log("PASS schema+Ajv2020+independent SQLite raw close/cancel/zero/error conservation cases="+law.closeWorkCases.length+"; native actual-child comparison NOTRUN");
