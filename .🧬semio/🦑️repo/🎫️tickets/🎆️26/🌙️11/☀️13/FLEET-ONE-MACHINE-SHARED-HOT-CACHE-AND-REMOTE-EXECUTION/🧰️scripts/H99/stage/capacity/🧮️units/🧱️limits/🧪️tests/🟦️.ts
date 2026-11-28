/** 🧱️ A plan warns before the build when a unit's measured time or peak memory is above its declared limit; a unit declared as carving debt warns with its carving action. The oracle is a Python model of the same rules (`🐍️.py`); the rules are language-agnostic fixtures. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixtures from "./🔣️.json" with {type:"json"};
import {UNIT_LIMITS_POLICY_V1,planUnitLimitsV1,unitLimitStateV1,unitLimitsV1} from "../🟦️.ts";

test("the policy validates against its own schema and every carving budget is the measured cost with the declared margin", () => {
 const ajv=new Ajv({strict:false}),validate=ajv.compile({...schema.definitions.Policy,definitions:schema.definitions});
 expect(validate(UNIT_LIMITS_POLICY_V1),JSON.stringify(validate.errors)).toBe(true);
 for(const row of UNIT_LIMITS_POLICY_V1.carving){
  expect(row.budgetMilliseconds).toBe(Math.ceil(row.measuredMilliseconds*UNIT_LIMITS_POLICY_V1.defaults.budgetBasisPoints/10000));
  expect(row.budgetMegabytes).toBe(Math.ceil(row.measuredMegabytes*UNIT_LIMITS_POLICY_V1.defaults.budgetBasisPoints/10000));
 }
});

test("every fixture unit gets its state", () => {
 for(const row of fixtures.states)expect(unitLimitStateV1(row as never),row.name).toBe(row.state as never);
 expect(unitLimitsV1("a_unit","host")).toEqual({maximumMilliseconds:1200000,maximumMegabytes:1536});
});

test("the plan names every unit above a limit in both languages, with the carving action of the declared ones", () => {
 const plan=planUnitLimitsV1(fixtures.states.filter(row=>"milliseconds" in row||"megabytes" in row) as never);
 expect(plan.over).toBe(4);
 const tiles=plan.warnings.find(row=>row.crate.endsWith("tile_coordinates")&&row.state==="carving")!;
 expect(tiles.en).toContain("trim-unused-dev-dependencies");expect(tiles.de).toContain("trim-unused-dev-dependencies");
 expect(tiles.en).not.toBe(tiles.de);
 expect(plan.warnings.every(row=>row.en.length>0&&row.de.length>0)).toBe(true);
});

test("the Python model gives the same states (oracle)", () => {
 const run=spawnSync("python3",[join(import.meta.dir,"🐍️.py")],{input:JSON.stringify({policy:UNIT_LIMITS_POLICY_V1,states:fixtures.states}),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 expect(JSON.parse(run.stdout)).toEqual(fixtures.states.map(row=>row.state));
});
