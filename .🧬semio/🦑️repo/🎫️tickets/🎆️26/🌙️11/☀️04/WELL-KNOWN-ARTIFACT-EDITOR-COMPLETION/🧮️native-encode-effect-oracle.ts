import {Database} from "bun:sqlite";
import Ajv from "ajv";
import schema from "./📐️native-encode-effect-schema.json";
import cases from "./📐️native-encode-effect-fixture.json";
const validate=new Ajv().compile(schema);if(!validate(cases))throw new Error(JSON.stringify(validate.errors));const db=new Database(":memory:");const q=db.query("select case when ? then ? else 0 end work, case when ? then ?+?+?+? else 1 end causes");for(const c of cases){let r=q.get(+c.granted,+c.performed,+c.granted,+c.operation,+c.workRefusal,+c.physicalRefusal,+c.allocationRefusal) as {work:number,causes:number};if(r.work!==c.expectedWork||r.causes!==c.expectedCauses)throw new Error(c.name)}console.log("Ajv + SQLite: "+cases.length+" native effect cases passed");
