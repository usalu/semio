import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./📐️source-unwind-failure-schema.json";
import cases from "./📐️source-unwind-failure-fixture.json";
const validate=new Ajv().compile(schema);if(!validate(cases))throw new Error(JSON.stringify(validate.errors));
const db=new Database(":memory:");const query=db.query("select case when ? then 0 else ? + ? end retained, case when ? then ? + ? else 0 end transferred");
for(const c of cases){const got=query.get(+c.capacity,+c.receipt,+c.provenance,+c.capacity,+c.receipt,+c.provenance) as {retained:number,transferred:number};if(got.retained!==c.expectedRetained||got.transferred!==c.expectedTransferred)throw new Error(c.name);}
console.log("Ajv + SQLite: "+cases.length+" Source unwind ownership cases passed");
