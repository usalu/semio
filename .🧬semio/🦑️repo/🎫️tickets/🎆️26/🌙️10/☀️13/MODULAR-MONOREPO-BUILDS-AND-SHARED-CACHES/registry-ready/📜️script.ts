import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {cargoRegistryReadyResolutionV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟢️ready/🟦️.ts";
import {withRepositoryCargoRegistryReadyV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟢️ready/💾️storage/🟦️.ts";
import schema from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟢️ready/🧬️schema/🔣️.json" with {type:"json"};
import fixture from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟢️ready/🧪️tests/🧫️fixtures/🔣️.json" with {type:"json"};
test("closed workspace-only registry readiness agrees with independent SQLite",()=>{
 const validator=new Ajv({strict:true}).compile(schema.definitions.Resolution),database=new Database(":memory:");
 try{for(const row of fixture.cases){expect(validator(row.resolution),row.name).toBe(true);const source=JSON.stringify(row.resolution),oracle=database.query("SELECT NOT EXISTS(SELECT 1 FROM json_each(?, '$.packages') WHERE json_type(value,'$.source')!='null') AND NOT EXISTS(SELECT 1 FROM json_each(?, '$.members') WHERE value GLOB '*[*?[]*') AND NOT EXISTS(SELECT 1 FROM json_each(?, '$.resolved') AS r WHERE NOT EXISTS(SELECT 1 FROM json_each(?, '$.packages') AS p WHERE json_extract(p.value,'$.id')=r.value)) AND NOT EXISTS(SELECT 1 FROM json_each(?, '$.packages') WHERE json_extract(value,'$.manifest') LIKE '../%') AND instr(json_extract(?,'$.workspaceManifest'),char(92))=0 AND (SELECT count(DISTINCT json_extract(value,'$.manifest'))=count(*) FROM json_each(?, '$.packages')) AS ready").get(source,source,source,source,source,source,source) as {ready:number};expect(Boolean(oracle.ready),row.name).toBe(row.ready);expect(cargoRegistryReadyResolutionV1(row.resolution),row.name).toBe(row.ready);}}finally{database.close();}
 console.log("[DEBUG] Registry ready neutral resolution corpus agrees with actual SQLite source/path/membership predicates");
});
