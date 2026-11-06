import {readFileSync} from "node:fs";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
const fixture=JSON.parse(readFileSync("🧰️framework/🔨️modules/🧵️job/⏱️context/🛫️encoding/🧪️tests/source-cancel.json","utf8"));
const schema={type:"object",required:["contract","prefix","tail","expectedCopiedBytes","expectedCapacityBytes","expectedActionWork","expectedFuel","expectedKind"],additionalProperties:false,properties:{contract:{const:"semio.job.source-encoder-post-effect-cancellation.v1"},prefix:{type:"array",minItems:1,items:{type:"integer",minimum:0,maximum:255}},tail:{type:"array",minItems:1,items:{type:"integer",minimum:0,maximum:255}},expectedCopiedBytes:{type:"integer",minimum:0},expectedCapacityBytes:{type:"integer",minimum:0},expectedActionWork:{type:"integer",minimum:0},expectedFuel:{type:"integer",minimum:0},expectedKind:{const:"canceled"}}};
const ajv=new Ajv();if(!ajv.validate(schema,fixture))throw Error(ajv.errorsText());
const db=new Database(":memory:");db.run("CREATE TABLE bytes (part TEXT NOT NULL, byte INTEGER NOT NULL)");const insert=db.prepare("INSERT INTO bytes VALUES (?,?)");for(const byte of fixture.prefix)insert.run("prefix",byte);for(const byte of fixture.tail)insert.run("tail",byte);
const row=db.query("SELECT SUM(part='prefix') AS copied, COUNT(*) AS capacity, CASE WHEN SUM(part='prefix')>0 THEN 1 ELSE 0 END AS actions, 2+CASE WHEN SUM(part='prefix')>0 THEN 1 ELSE 0 END AS fuel FROM bytes").get() as {copied:number;capacity:number;actions:number;fuel:number};
if(row.copied!==fixture.expectedCopiedBytes||row.capacity!==fixture.expectedCapacityBytes||row.actions!==fixture.expectedActionWork||row.fuel!==fixture.expectedFuel)throw Error(JSON.stringify(row));console.log(JSON.stringify(row));db.close();
