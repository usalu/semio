import {readFileSync} from "node:fs";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
const t=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/";
const cases=JSON.parse(readFileSync(t+"1105-scratch-transfer.cases.json","utf8"));
const validate=new Ajv({allErrors:true}).compile(JSON.parse(readFileSync(t+"1105-scratch-transfer.schema.json","utf8")));
if(!validate(cases))throw Error(JSON.stringify(validate.errors));
const db=new Database(":memory:");db.run("create table owner(location text primary key, identity text)");
for(const row of cases){
 db.run("delete from owner");
 if(row.source)db.run("insert into owner values ('source','original')");
 if(row.target)db.run("insert into owner values ('target','incoming')");
 const from=row.op==="take"?"source":"target",to=row.op==="take"?"target":"source";
 const changes=row.admitted?db.query("update owner set location=? where location=? and not exists(select 1 from owner where location=?)").run(to,from,to).changes:0;
 const actual={changed:changes===1,afterSource:!!db.query("select 1 from owner where location='source'").get(),afterTarget:!!db.query("select 1 from owner where location='target'").get()};
 for(const key of Object.keys(actual))if(actual[key]!==row[key])throw Error(row.id+":"+key);
 if(changes&&db.query("select identity from owner where location=?").get(to).identity!==(row.op==="take"?"original":"incoming"))throw Error("replaced identity");
}
db.close();console.log("Ajv and independent SQLite original ownership transfer passed 6 cases.");
