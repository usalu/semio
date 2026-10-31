import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import {compileJsonSchemaSubset} from "../../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import {schemaResolutionDiagnostics} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📐️schema-contracts/🔗️reference-diagnostics/🟦️.ts";
import rows from "./schema-bcf-octets-vectors.json";
test("actual BCF SetParts resolves canonical byte owner",()=>{
 const base="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🧬️schema/",schema=JSON.parse(readFileSync(base+"🧬️mutations/📎set-parts/🧬️schema/🔣️.json","utf8")),snapshot=JSON.parse(readFileSync(base+"📸️snapshot/🔣️.json","utf8")),bytes=JSON.parse(readFileSync("🧰️framework/🔨️modules/🌱️value/🧬️bytes/🧬️schema/🔣️.json","utf8"));
 const ajv=new Ajv({strict:false}).addSchema(bytes);
 expect(ajv.validateSchema(schema)).toBe(true);expect(bytes.$defs.Octets).toEqual(snapshot.definitions.octets);
 const oracle=ajv.compile(schema),owned=compileJsonSchemaSubset(schema,schema,[bytes]);
 for(const row of rows){expect(oracle(row.input),row.id).toBe(row.expected);expect(owned.validate(row.input).length===0,row.id).toBe(row.expected)}
 expect(schema.definitions.part.properties.data.$ref).toBe(bytes.$id+"#/$defs/Octets");
});

test("actual BCF leaf catalog declares its byte dependency",()=>{expect(schemaResolutionDiagnostics(process.cwd(),"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🧬️schema/🧬️mutations/📎set-parts/🧬️schema")).toEqual([])});
