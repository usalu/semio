import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
const fixture=JSON.parse(readFileSync("🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️tests/🧭️drive/primitive.json","utf8"));
const db=new Database(":memory:");
for(const row of fixture.cases){const actual=db.query("SELECT CASE WHEN ? >= 1 THEN 1 ELSE 0 END AS work, CASE WHEN ? >= 1 THEN 1 ELSE 0 END AS effects, CASE WHEN ? >= 1 AND ? = 0 THEN 1 ELSE 0 END AS accepted").get(row.fuel,row.fuel,row.fuel,Number(row.fail))as{work:number,effects:number,accepted:number};if(actual.work!==row.work||actual.effects!==row.effects||Boolean(actual.accepted)!==row.accepted)throw Error("Independent SQLite primitive policy differs");}
const scalar=JSON.parse(readFileSync("🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/🧪️tests/scalar.json","utf8"));
for(const row of scalar.cases){const actual=db.query("SELECT CASE WHEN ? >= 1 THEN 1 ELSE 0 END AS work, CASE WHEN ? >= 1 AND ? BETWEEN 0 AND 255 THEN 1 ELSE 0 END AS copy").get(row.fuel,row.fuel,row.value)as{work:number,copy:number};if(actual.work!==row.work||actual.copy!==row.copy||Boolean(actual.copy)!==row.valid)throw Error("Independent SQLite scalar validation differs");}
db.close();console.log(JSON.stringify({cases:fixture.cases.length+scalar.cases.length,oracle:"SQLite CASE loan admission, failure and scalar range relations",result:"pass"}));
