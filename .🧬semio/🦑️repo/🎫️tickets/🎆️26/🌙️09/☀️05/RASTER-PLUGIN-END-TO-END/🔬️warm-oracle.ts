import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import manifest from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🔣️.json" with {type:"json"};
import schema from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🧬️schema/🔥️warm.json" with {type:"json"};
import fixture from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🧪️tests/🔥️warm/🔣️.json" with {type:"json"};
if(!new Ajv({strict:true}).validate(schema,manifest))throw Error("Warm manifest schema refused");const db=new Database(":memory:");for(const row of fixture.cases){const result=db.query("SELECT CASE WHEN ?1 AND ?2 THEN 1 ELSE 0 END AS ready").get(row.includeWarm?1:0,row.pinMatches?1:0) as {ready:number};if(Boolean(result.ready)!==row.ready)throw Error("Independent warm fixture differs");}db.close();console.log(JSON.stringify({cases:fixture.cases.length,manifestModules:manifest.modules.length,oracle:"Ajv and SQLite",status:"passed"}));
