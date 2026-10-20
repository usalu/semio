import {isAbsolute} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

const policy=schema.definitions.Policy.const;

/** 🧭️ One neutral task as the Bazel batch sees it: its root-package label and the rule kind that executes it. */
export type BazelCommandTaskV1=Readonly<{label:string;kind:"task"|"hermetic"|"test"|"noop"|"launch"}>;

/** 🎛️ Everything the mapping depends on; `base` carries the cache, module and strictness flags owned elsewhere. */
export type BazelCommandInputV1=Readonly<{mode:"execute"|"plan";base:readonly string[];jobs:number;bail:boolean;verbose:boolean;skipCache:boolean;events?:string;tasks:readonly BazelCommandTaskV1[]}>;

/** 📋️ One Bazel invocation: its verb, the targets it names and the options after the base flags. */
export type BazelCommandStepV1=Readonly<{verb:"build"|"test";targets:readonly string[];options:readonly string[]}>;

/** 📑️ The ordered Bazel invocations of an operation; empty when Bazel has nothing to run. */
export type BazelCommandPlanV1=Readonly<{version:1;steps:readonly BazelCommandStepV1[]}>;

/** 🧾️ Names the compact execution log a caller asks for through the environment: one absolute path, so a conformance harness can read which task of the run executed and which was restored. */
export function bazelTaskReportOptionsV1(environment:Readonly<Record<string,string|undefined>>):string[]{
 const path=environment.SEMIO_BUILD_TASK_REPORT;
 if(path===undefined||path==="")return [];
 if(!isAbsolute(path)||/[\r\n]/u.test(path))throw Error("Absolute Bazel task report path required");
 return [`--execution_log_compact_file=${path}`];
}

/** 🗺️ Maps a selected batch to its one Bazel invocation: `test` when any member is a test (with `--build_tests_only` only when all are), `build` otherwise; planning modes and batches of services start nothing. */
export function bazelCommandPlanV1(input:BazelCommandInputV1):BazelCommandPlanV1{
 if(validateJsonSchemaSubset(schema.definitions.Input,input,schema).length)throw Error("Closed Bazel command input required");
 const batch=input.mode==="plan"?[]:input.tasks.filter(task=>task.kind!=="launch");
 if(!batch.length)return Object.freeze({version:1,steps:Object.freeze([])});
 const tests=batch.filter(task=>task.kind==="test").length,common=[`--jobs=${input.jobs}`,...input.bail?[]:["--keep_going"],...input.verbose?["--subcommands"]:[],...input.events?[`--build_event_json_file=${input.events}`]:[]];
 const options=tests?[...common,...tests===batch.length?["--build_tests_only"]:[],`--test_output=${input.verbose?policy.testOutput.verbose:policy.testOutput.default}`,input.skipCache?"--nocache_test_results":"--cache_test_results=auto",...input.bail?["--notest_keep_going"]:[]]:common;
 const plan={version:1 as const,steps:[{verb:tests?"test" as const:"build" as const,targets:batch.map(task=>task.label),options:[...input.base,...options]}]};
 if(validateJsonSchemaSubset(schema.definitions.Plan,plan,schema).length)throw Error("Closed Bazel command plan required");
 return Object.freeze({version:1,steps:Object.freeze(plan.steps.map(step=>Object.freeze(step)))});
}
