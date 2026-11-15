/** 📏️ Contract: the wire shape of the retirement plan holds every role roster the producer may legally hand over. The producer reads the store root with the bound `maximumRoles` of the plan protocol (24) and the closed image has 18 declared names; the wire shape allowed 16, so a fully funded image ended `repo tools setup all` with `The dependency store returned no complete retirement plan (malformed, 227696 bytes, first violation: /value/roles/names must contain at most 16 item(s))` (g1-m02 step 0 of rounds 20 and 24, after the bazel role of R6c had landed).
 * @see ../../🧬️schema/🔣️.json */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import fixture from "../🔣️.json" with {type:"json"};
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import {javascriptInstallSchemaV1} from "../../../../../../../../../🚀️bootstrap/📦️dependencies/💾️storage/🧱️schemas/🟦️.ts";
import {materializeSchemaV1,schemaViewV1} from "../../../../../../../../../🚀️bootstrap/📦️dependencies/🔌️managers/🧩️materialize/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

const selected=materializeSchemaV1(schema,schemaViewV1(javascriptInstallSchemaV1.$defs.ManagerProtocol.manager.descriptor)) as typeof schema;
const wire={...selected,$defs:{...selected.$defs,SelectedRetirementSnapshot:javascriptInstallSchemaV1.$defs.SelectedRetirementSnapshot,SelectedRetirementResult:javascriptInstallSchemaV1.$defs.SelectedRetirementResult}};
const protocol=javascriptInstallSchemaV1.$defs.SelectedPlanProtocol.const;
const declared=[...protocol.directoryRoles,...protocol.fileRoles,protocol.service,...protocol.partitionRoles.map((role:{name:string})=>role.name)] as string[];
const planWith=(names:readonly string[],candidateRoles:readonly string[]=[])=>{const plan:any=structuredClone(fixture.plan);plan.roles={stamp:plan.roles?.stamp??"stamp",names:[...names]};if(candidateRoles.length)plan.candidates[0].roles=[...candidateRoles];return{acquired:true,value:plan};};
const names=(count:number)=>Array.from({length:count},(_,at)=>"r"+String(at).padStart(2,"0"));
const independent=(count:number)=>count<=protocol.maximumRoles;

test("the wire bound of the role rosters is the producer's bound, declared once in the plan protocol",()=>{
 const plan=wire.$defs.Plan as any;
 expect(plan.properties.roles.properties.names.maxItems).toBe(protocol.maximumRoles);
 expect(plan.properties.candidates.items.properties.roles.maxItems).toBe(protocol.maximumRoles);
 expect(declared.length).toBeLessThanOrEqual(protocol.maximumRoles);
 expect(declared.length).toBeGreaterThan(16);
});

for(const count of [0,1,16,17,declared.length,24,25])test("a plan whose root roster has "+count+" names is "+(independent(count)?"accepted":"refused")+" by the wire shape, as the independent bound says",()=>{
 const ajv=new Ajv({strict:true}).compile({...wire,$ref:"#/$defs/AcquiredPlan"});
 const plan=planWith(names(count));
 expect(ajv(plan),JSON.stringify(ajv.errors)).toBe(independent(count));
 expect(validateJsonSchemaSubset((wire as any).$defs.AcquiredPlan,plan,wire as any).length===0).toBe(independent(count));
});

test("the declared roles of a fully funded image, all present, pass the wire shape",()=>{
 const ajv=new Ajv({strict:true}).compile({...wire,$ref:"#/$defs/AcquiredPlan"}),plan=planWith(declared,declared);
 expect(ajv(plan),JSON.stringify(ajv.errors)).toBe(true);
});
