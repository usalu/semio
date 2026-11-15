import {pythonEnvironmentOfProjectV1, pythonEnvironmentInterpreterV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🧰️toolchains/🟦️.ts";
const plan = pythonEnvironmentOfProjectV1("/Users/ueli/Documents/semio", "workspace");
console.log(JSON.stringify({vars: plan.variables, groups: plan.groups, sync: plan.syncArguments, interpreter: pythonEnvironmentInterpreterV1(plan)}, null, 1));
