import {readFile} from "node:fs/promises";
import {resolve} from "node:path";
import {isDeepStrictEqual} from "node:util";

const root = resolve("✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any");
const schemaPath = resolve(root,"🧬️schema");
const {parseEn1998Mutation} = await import(resolve(schemaPath,"🧬️mutations/🟦️.ts"));
const {parseEn1998Diff} = await import(resolve(schemaPath,"🔺️diff/🟦️.ts"));
const {parseEn1998Snapshot} = await import(resolve(schemaPath,"📸️snapshot/🟦️.ts"));
const {semioSchemaAjvV1} = await import("@semio-tech/framework/schema/oracles/validator");
const getJson = async path => JSON.parse(await readFile(path,"utf8"));
const ajv = semioSchemaAjvV1({strict:true});
const snapshotSchema = await getJson(resolve(schemaPath,"📸️snapshot/🔣️.json"));
const diffSchema = await getJson(resolve(schemaPath,"🔺️diff/🔣️.json"));
ajv.addSchema(snapshotSchema);
const cases = await getJson(resolve(root,"🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/📋️design-cases.json"));
const archive = await getJson(resolve(root,"🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json"));
const assert = (condition, description) => { if (!condition) throw Error(description); };
assert(parseEn1998Snapshot({...archive,cases}).cases.length===2,"Snapshot reader must return both authored case rows");
const operations = [
  ["➕️insert-case",{mutation:"insertCase",index:0,case:cases[0]}],
  ["➖️remove-case",{mutation:"removeCase",id:cases[0].id}],
  ["🔢️change-case-number",{mutation:"changeCaseNumber",caseId:cases[0].id,key:"pipe_radius",value:0.45}],
  ["🔤️change-case-text",{mutation:"changeCaseText",caseId:cases[0].id,key:"pipe_wave_basis",value:"unconfirmed"}],
  ["🏷️change-case-label-en",{mutation:"changeCaseLabelEn",caseId:cases[0].id,newValue:"Updated pipe"}],
  ["🏷️change-case-label-de",{mutation:"changeCaseLabelDe",caseId:cases[0].id,newValue:"Geändertes Rohr"}],
  ["📇️change-case-entry-id",{mutation:"changeCaseEntryId",caseId:cases[0].id,newValue:"C98-11"}],
];
for (const [folder,operation] of operations) {
  const schema = await getJson(resolve(schemaPath,"🧬️mutations",folder,"🧬️schema/🔣️.json"));
  const check = ajv.compile(schema);
  assert(check(operation),JSON.stringify(check.errors));
  assert(isDeepStrictEqual(parseEn1998Mutation(operation),operation),operation.mutation);
  assert(!check({...operation,mutation:"unregistered"}),operation.mutation+" accepts an incorrect tag");
}
const emptyDelta = () => ({removed:[],inserted:[],moved:[],modified:[]});
const empty = Object.fromEntries(Object.entries(diffSchema.properties)
  .filter(([key])=>!["annex","site"].includes(key)).map(([key])=>[key,emptyDelta()]));
const caseDiff = {...empty,cases:{...emptyDelta(),inserted:[{index:0,row:cases[0]}],modified:[{
  id:cases[1].id,
  patch:{numbers:{...emptyDelta(),modified:[{id:"spatial_reference_distance",patch:{value:180}}]}},
}]}};
const checkDiff = ajv.compile(diffSchema);
assert(checkDiff(caseDiff),JSON.stringify(checkDiff.errors));
assert(parseEn1998Diff(caseDiff).cases.inserted[0].row.id===cases[0].id,"Diff reader dropped the inserted case");
assert(parseEn1998Diff(caseDiff).cases.modified[0].patch.numbers.modified[0].patch.value===180,"Diff reader dropped a nested number edit");
console.log("[DEBUG] EN 1998 snapshot, seven case mutations and nested case diff passed TypeScript and independent Ajv.");
