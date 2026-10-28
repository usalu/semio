/** 🧾️ Independent schema validation checks declared IFC fixture loans against initiating stock. */
import Ajv from "ajv/dist/2020.js";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {strict as assert} from "node:assert";

type Grant = Record<"maximumItems" | "maximumCopyBytes" | "maximumCapacityBytes" | "maximumReleaseBytes" | "maximumDepth", number>;
interface OwnerPolicyOracle { valid(value:unknown): value is Grant }
function createOracle(schema:Record<string,unknown>):OwnerPolicyOracle {
 const ajv = new Ajv({strict:true,allErrors:true});
 ajv.addSchema(schema);
 const validate = ajv.getSchema(String(schema.$id) + "#/$defs/RetainedCloneGrantV1")!;
 return {valid:(value:unknown):value is Grant => Boolean(validate(value))};
}
const root = resolve(import.meta.dirname,"../../../../../../..");
const read = (path:string):any => JSON.parse(readFileSync(resolve(root,path),"utf8"));
const fixture = read("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests/source-owners.json");
const original = read("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json");
const oracle = createOracle(read("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏇️mounted-owner/🧬️schema/🔣️.json"));
assert(oracle.valid(original.grant));
for (const name of ["shortLoan","contiguousOwnerLoan"]) {
 assert(oracle.valid(fixture[name]));
 for (const key of Object.keys(original.grant) as (keyof Grant)[]) assert(fixture[name][key] <= original.grant[key]);
}
assert(fixture.shortLoan.maximumCapacityBytes < fixture.deniedExistingBody.capacityDemand);
assert(fixture.contiguousOwnerLoan.maximumCapacityBytes >= fixture.deniedExistingBody.capacityDemand);
assert(fixture.contiguousOwnerLoan.maximumReleaseBytes >= fixture.deniedExistingBody.textBytes);
assert(!oracle.valid({...fixture.contiguousOwnerLoan,maximumItems:-1}));
assert(!oracle.valid({...fixture.contiguousOwnerLoan,unfunded:true}));
console.log(JSON.stringify({oracle:"Ajv2020",loans:2,belowOriginalStock:true,shortContiguousCloneDenied:true,shapeRefusals:2,nativeAuthorityAndHeapExecution:"not executed"}));
