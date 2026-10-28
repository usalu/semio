import{readFileSync}from"node:fs";
import{resolve}from"node:path";
import{validateJsonSchemaSubset}from"@semio-tech/schema-validator-ts";
import{refuseProcessV1}from"../../../📖️texts/🟦️.ts";
import type{ProcessTestSuiteV1,ProcessTestTableV1}from"../🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import policyDocument from"./🔣️.json"with{type:"json"};

/** 🧱️ Names a host-bound fact a test file may not use unconditionally if its suite is to run on Windows. */
export type ProcessPlatformCauseV1="posix-program"|"posix-shell-script"|"python3-command"|"symlink"|"permission-bits"|"posix-process-control";
/** 📋️ The closed evidence rules after schema admission. */
export type ProcessPlatformPolicyV1=Readonly<{version:1;header:string;gatedModifier:string;lineGate:string;condition:string;window:Readonly<{linesBeforeWithoutHeader:number;linesBeforeMarker:number}>;causes:readonly Readonly<{id:ProcessPlatformCauseV1;lineGate:boolean;markers:readonly string[]}>[]}>;
/** 🔎️ What one test file says: the causes it uses unconditionally, the ones inside a test block that names the platform, and the ones inside a skipped test. */
export type ProcessPlatformEvidenceV1=Readonly<{unconditioned:readonly ProcessPlatformCauseV1[];conditioned:readonly ProcessPlatformCauseV1[];gated:readonly ProcessPlatformCauseV1[]}>;
/** 🧾️ What a route table gets wrong about its platform verdicts. */
export type ProcessPlatformFindingsV1=Readonly<{unjudged:readonly string[];unexplained:readonly string[];overexplained:readonly string[]}>;

/** 📖️ Reads the evidence rules, refusing documents that break the closed schema, repeat a cause or hold a pattern that is no regular expression. @see ./🧬️schema/🔣️.json */
export function readProcessPlatformPolicyV1(value:unknown=policyDocument):ProcessPlatformPolicyV1{
 if(validateJsonSchemaSubset({$ref:"#/definitions/Policy"},value,schema).length)throw refuseProcessV1("invalid","testing.platform-policy-invalid");
 const row=value as ProcessPlatformPolicyV1;
 if(new Set(row.causes.map(cause=>cause.id)).size!==row.causes.length)throw refuseProcessV1("invalid","testing.platform-policy-invalid");
 try{for(const pattern of[row.header,row.gatedModifier,row.lineGate,row.condition,...row.causes.flatMap(cause=>cause.markers)])new RegExp(pattern);}catch(cause){throw refuseProcessV1("invalid","testing.platform-policy-invalid",{},{cause});}
 return row;
}

/** 🔎️ Reads one test file against the evidence rules: a marker counts as unconditioned unless its nearest test header is skipped, its line names the platform (for the causes that allow that) or its test block names the platform. */
export function processPlatformEvidenceV1(text:string,policy:ProcessPlatformPolicyV1=readProcessPlatformPolicyV1()):ProcessPlatformEvidenceV1{
 const lines=text.split("\n"),header=new RegExp(policy.header,"g"),gatedModifier=new RegExp(policy.gatedModifier),lineGate=new RegExp(policy.lineGate),condition=new RegExp(policy.condition);
 const headers:{line:number;gated:boolean}[]=[];
 lines.forEach((line,index)=>{const found=[...line.matchAll(header)];if(found.length)headers.push({line:index,gated:gatedModifier.test(found[found.length-1]![1]??"")});});
 const markers=policy.causes.map(cause=>({id:cause.id,lineGate:cause.lineGate,patterns:cause.markers.map(pattern=>new RegExp(pattern))})),unconditioned=new Set<ProcessPlatformCauseV1>(),conditioned=new Set<ProcessPlatformCauseV1>(),gated=new Set<ProcessPlatformCauseV1>();
 lines.forEach((line,index)=>{
  for(const cause of markers){
   if(!cause.patterns.some(pattern=>pattern.test(line)))continue;
   if(cause.lineGate&&lineGate.test(line))continue;
   let at=-1;for(let position=0;position<headers.length&&headers[position]!.line<=index;position++)at=position;
   if(at>=0&&headers[at]!.gated){gated.add(cause.id);continue;}
   const low=at>=0?headers[at]!.line:Math.max(0,index-policy.window.linesBeforeWithoutHeader),high=at+1<headers.length?headers[at+1]!.line:lines.length;
   const block=lines.slice(low,high).join("\n")+"\n"+lines.slice(Math.max(0,index-policy.window.linesBeforeMarker),index+1).join("\n");
   (condition.test(block)?conditioned:unconditioned).add(cause.id);
  }
 });
 const order=policy.causes.map(cause=>cause.id),sorted=(set:Set<ProcessPlatformCauseV1>)=>order.filter(id=>set.has(id));
 return Object.freeze({unconditioned:sorted(unconditioned),conditioned:sorted(conditioned),gated:sorted(gated)});
}

/** 🧮️ Unions the unconditioned causes of every test file of a suite: the files it runs below the module root plus the module of its function. */
export function processPlatformCausesOfSuiteV1(suite:ProcessTestSuiteV1,root:string,files:readonly string[],policy:ProcessPlatformPolicyV1=readProcessPlatformPolicyV1(),read:(path:string)=>string=path=>readFileSync(path,"utf8")):readonly ProcessPlatformCauseV1[]{
 const found=new Set<ProcessPlatformCauseV1>(),every=[...files,...(suite.function?[suite.function.module]:[])];
 for(const file of every)for(const cause of processPlatformEvidenceV1(read(resolve(root,file)),policy).unconditioned)found.add(cause);
 return policy.causes.map(cause=>cause.id).filter(id=>found.has(id));
}

/** ⚖️ Derives the verdict of a suite from its causes: every host when nothing host-bound is used unconditionally (or when the CI policy requires Windows), otherwise the POSIX hosts with the causes that keep Windows out. */
export function processPlatformVerdictV1(causes:readonly ProcessPlatformCauseV1[],windowsRequired:boolean):Readonly<{platforms:readonly("darwin"|"linux"|"win32")[];platformCauses:readonly ProcessPlatformCauseV1[]}>{
 return causes.length===0||windowsRequired?{platforms:["darwin","linux","win32"],platformCauses:[]}:{platforms:["darwin","linux"],platformCauses:causes};
}

/** 🚩️ Lists the suites of a route table whose platform verdict is missing (no `platforms`), unexplained (a host is missing and no cause names why) or overexplained (causes although every host is named). */
export function processPlatformFindingsV1(declared:ProcessTestTableV1):ProcessPlatformFindingsV1{
 const hosts=["darwin","linux","win32"],unjudged:string[]=[],unexplained:string[]=[],overexplained:string[]=[];
 for(const suite of declared.suites){
  if(suite.platforms===undefined){unjudged.push(suite.id);continue;}
  const everywhere=hosts.every(host=>(suite.platforms as readonly string[]).includes(host)),causes=suite.platformCauses??[];
  if(!everywhere&&causes.length===0)unexplained.push(suite.id);
  if(everywhere&&causes.length>0)overexplained.push(suite.id);
 }
 return Object.freeze({unjudged,unexplained,overexplained});
}
