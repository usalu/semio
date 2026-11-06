import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./📐️source-header-schema.json";
import cases from "./📐️source-header-fixture.json";
const validate=new Ajv().compile(schema);if(!validate(cases))throw new Error(JSON.stringify(validate.errors));
const db=new Database(":memory:");const query=db.query("select ? >= ? as born, case when ? >= ? then ? else 0 end as copied");
for(const c of cases){const got=query.get(c.copyGrant,c.header,c.copyGrant,c.header,c.header) as {born:number,copied:number};if(Boolean(got.born)!==c.expectedBorn||got.copied!==c.expectedCopy)throw new Error(c.name);}
console.log("Ajv + SQLite: "+cases.length+" Source header admission cases passed");
