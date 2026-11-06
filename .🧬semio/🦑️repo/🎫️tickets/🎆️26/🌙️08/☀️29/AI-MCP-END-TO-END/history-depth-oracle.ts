import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import Ajv2020 from "ajv/dist/2020";
const root=import.meta.dirname, cases=JSON.parse(readFileSync(`${root}/history-depth-cases.json`,"utf8"));
const validate=new Ajv2020({strict:true}).compile(JSON.parse(readFileSync(`${root}/history-depth-cases.schema.json`,"utf8")));if(!validate(cases))throw Error(JSON.stringify(validate.errors));
const db=new Database(":memory:");const query=db.query("select case when ? >= ? then 'progress' when ? < ? then 'refusal' else 'pending' end as result");
for(const c of cases){const sql=(query.get(c.current,c.need,c.total,c.need) as {result:string}).result;const model=c.current>=c.need?"progress":c.total<c.need?"refusal":"pending";if(sql!==model||sql!==c.expected)throw Error(JSON.stringify({name:c.name,sql,model,expected:c.expected}))}db.close();
console.log(`SQLite issuer depth oracle passed ${cases.length} cases`);
