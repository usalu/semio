import {GRPC_CODES_V1} from "../../📡️wire/🚪️grpc/🟦️.ts";
import type {OffloadReasonV1} from "../📐️policy/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🩺️ The prefix with which a worker authors the refusal of an action by its own policy: the protocol's marker for the one `FAILED_PRECONDITION` that no other machine will answer differently. The worker writes it and the client reads it from this one definition. */
export const OFFLOAD_REFUSAL_PREFIX_V1="Action refused by the worker policy";

/** 🩺️ What a non-OK status of the execution service means for the requester: the reason the unit compiles here and whether the same action may be submitted again (only a lost transport). Classified by the gRPC code, never by compiler text. */
export function classifyOffloadStatusV1(status:Readonly<{code:number;message?:string}>):Readonly<{reason:OffloadReasonV1;transient:boolean}>{
 if(status.code===GRPC_CODES_V1.RESOURCE_EXHAUSTED)return {reason:"no-idle-machine",transient:false};
 if(status.code===GRPC_CODES_V1.FAILED_PRECONDITION)return {reason:(status.message??"").startsWith(OFFLOAD_REFUSAL_PREFIX_V1)?"worker-policy":"no-idle-machine",transient:false};
 return {reason:"infrastructure",transient:status.code===GRPC_CODES_V1.UNAVAILABLE};
}

const FAILURE_V1=schema.definitions.Failure.const,INCOMPLETE_INPUT_CODES=new Set<string>(FAILURE_V1.codes),CLASSES_V1=FAILURE_V1.classes.map(row=>({name:row.name,codes:new Set<string>(row.codes),patterns:row.patterns.map(pattern=>new RegExp(pattern,"u"))}));

/** 🩺️ What a non-zero compiler exit means for the requester: `compile-result` (ordinary diagnostics about the crate's own code: the build's verdict, handed over byte for byte and never compiled twice), `incomplete-inputs` (a diagnostic code that names a missing input), `environmental` (the failure looks like something the other machine lacked: no error diagnostic at all, a missing-crate or unreadable-file error, a linker error, a crash, a full disk; `class` names it, see `./🧬️schema/🔣️.json`) or `infrastructure` (the shell could not run the compiler, exit 126 or 127, or the compiler was killed: exit above 128 or negative). Only environmental and incomplete failures are compiled once more here. */
export function classifyOffloadExitV1(input:Readonly<{exitCode:number;stderr:string}>):Readonly<{kind:"compile-result"|"incomplete-inputs"|"infrastructure"}|{kind:"environmental";class:string}>{
 if(FAILURE_V1.notRunnable.includes(input.exitCode)||input.exitCode>FAILURE_V1.signalAbove||input.exitCode<0)return {kind:"infrastructure"};
 const codes:string[]=[];let diagnosed=false;
 for(const line of input.stderr.split("\n")){
  if(!line.startsWith("{"))continue;
  try{const row=JSON.parse(line) as {level?:unknown;code?:{code?:unknown}|null};if(typeof row.level==="string"&&row.level.startsWith("error"))diagnosed=true;const code=row.code?.code;if(typeof code==="string")codes.push(code);}catch{}
 }
 if(codes.some(code=>INCOMPLETE_INPUT_CODES.has(code)))return {kind:"incomplete-inputs"};
 for(const row of CLASSES_V1)if(codes.some(code=>row.codes.has(code))||row.patterns.some(pattern=>pattern.test(input.stderr)))return {kind:"environmental",class:row.name};
 return diagnosed?{kind:"compile-result"}:{kind:"environmental",class:FAILURE_V1.noDiagnostic};
}

/** 🩺️ What a compiler said when an input of the action was missing, for the event that explains the fall-back: the `message` of the first structured diagnostic (with the first source file it names), else its first line; at most `headCharacters`. */
export function offloadIncompleteDetailV1(stderr:string):string{
 for(const line of stderr.split("\n")){
  if(!line.startsWith("{"))continue;
  try{const row=JSON.parse(line) as {message?:unknown;spans?:{file_name?:unknown}[]};if(typeof row.message==="string"){const file=row.spans?.find(span=>typeof span.file_name==="string")?.file_name;return(row.message+(typeof file==="string"?` (${file})`:"")).slice(0,240);}}catch{}
 }
 return(stderr.split("\n")[0]??"").slice(0,240);
}
