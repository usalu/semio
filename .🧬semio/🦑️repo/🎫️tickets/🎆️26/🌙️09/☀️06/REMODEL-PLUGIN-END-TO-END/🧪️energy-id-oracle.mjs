import {readFileSync} from "node:fs";
import Ajv2020 from "ajv/dist/2020.js";
import {Database} from "bun:sqlite";
const base="✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️tests/🧫️fixtures/";
const read=name=>JSON.parse(readFileSync(base+name,"utf8"));
const law=read("🆔️native.json"),validate=new Ajv2020({strict:true}).compile(read("🆔️native-schema.json"));
if(!validate(law))throw Error(JSON.stringify(validate.errors));
const db=new Database(":memory:");
try{for(const row of law.cases){const result=db.query("SELECT CASE WHEN ?1 BETWEEN 0 AND 4294967295 THEN ?1 ELSE NULL END AS value, ?2 + 1 AS work").get(row.wire,law.wrapperWork);if(result.value!==row.expected||result.work!==2)throw Error(`identifier fixture disagrees: ${row.wire}`)}console.log(`${law.cases.length} identifier vectors agree with SQLite`)}finally{db.close()}
