import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🏃️execution/🧬️schema/🌐️remote-cache.json" with {type:"json"};
import buildPolicy from "../🔣️policy.json" with {type:"json"};

/** 🌐️ The resolved, secret-free contract of one enabled remote cache, shaped by `🌐️remote-cache.json`. */
export type RemoteCacheContractV1=Readonly<{version:1;endpoint:string;instance:string;access:"read-only"|"read-write";upload:boolean;compression:boolean;download:"minimal"|"toplevel"|"all";timeoutSeconds:number;retries:number;auth:Readonly<{tokenEnvironment:string}>}>;

/** 📜️ The authored remote cache section of the build policy. */
export type RemoteCachePolicyV1=Readonly<typeof buildPolicy.remoteCache>;

/** 🧾️ Names which tools render the contract and what the selected one could not honour. */
export type RemoteCacheRenderingV1=Readonly<{mechanism:"bazel-flags"|"nx-environment";applied:readonly string[];unsupported:readonly ("compression"|"download"|"timeoutSeconds"|"retries")[];uploadEnforcedBy:"client"|"server"|"none"}>;

const definitions=schema.definitions;

/** 🛡️ Validates one value against a named definition of the contract schema and names the first problem. */
export function requireRemoteCacheShapeV1(definition:keyof typeof definitions,value:unknown,message:string):void{
 const errors=validateJsonSchemaSubset({$ref:`#/definitions/${definition}`},value,schema);
 if(errors.length)throw Error(`${message}: ${errors[0]}`);
}

/** 🏛️ Reads the authored policy and refuses any drift from its schema. */
export function readRemoteCachePolicyV1(policy:unknown=buildPolicy.remoteCache):RemoteCachePolicyV1{
 requireRemoteCacheShapeV1("Policy",policy,"Closed remote cache policy required");return policy as RemoteCachePolicyV1;
}

const flag=(name:string,value:string):boolean=>{
 if(value==="true")return true;
 if(value==="false")return false;
 throw Error(`Closed remote cache contract required: ${name} must be true or false`);
};
const count=(name:string,value:string):number=>{
 if(!/^[0-9]{1,5}$/u.test(value))throw Error(`Closed remote cache contract required: ${name} must be a whole number`);
 return Number(value);
};

/** 🧮️ Resolves the contract from the committed defaults and the environment; `undefined` means the remote cache is off, which is the case whenever no endpoint is configured. */
export function resolveRemoteCacheContractV1(environment:Readonly<Record<string,string|undefined>>,policy:RemoteCachePolicyV1=readRemoteCachePolicyV1()):RemoteCacheContractV1|undefined{
 const names=policy.environment,defaults=policy.defaults,read=(field:keyof typeof names):string|undefined=>{const value=environment[names[field]];return value===undefined||value===""?undefined:value;};
 const endpoint=read("endpoint")??defaults.endpoint;
 if(endpoint===null||endpoint===undefined)return undefined;
 const contract:RemoteCacheContractV1={
  version:1,
  endpoint,
  instance:read("instance")??defaults.instance,
  access:(read("access")??defaults.access) as RemoteCacheContractV1["access"],
  upload:flag(names.upload,read("upload")??String(defaults.upload)),
  compression:flag(names.compression,read("compression")??String(defaults.compression)),
  download:(read("download")??defaults.download) as RemoteCacheContractV1["download"],
  timeoutSeconds:count(names.timeoutSeconds,read("timeoutSeconds")??String(defaults.timeoutSeconds)),
  retries:count(names.retries,read("retries")??String(defaults.retries)),
  auth:{tokenEnvironment:read("tokenEnvironment")??defaults.tokenEnvironment}
 };
 requireRemoteCacheShapeV1("Contract",contract,"Closed remote cache contract required");
 return Object.freeze(contract);
}

/** ⬆️ Tells whether this machine pushes local results: the switch must be on and the credentials must be able to write. */
export function remoteCacheEffectiveUploadV1(contract:RemoteCacheContractV1):boolean{return contract.upload&&contract.access==="read-write";}

/** 🔗️ Joins the origin and the namespace into the base URL both protocols append their routes to. */
export function remoteCacheBaseUrlV1(contract:RemoteCacheContractV1):string{return `${contract.endpoint}/${contract.instance}`;}

/** 🔑️ Reads the access token from the environment variable the contract names; `undefined` means anonymous access. The value is never stored. */
export function readRemoteCacheTokenV1(contract:RemoteCacheContractV1,environment:Readonly<Record<string,string|undefined>>):string|undefined{
 const value=environment[contract.auth.tokenEnvironment];
 if(value===undefined||value==="")return undefined;
 requireRemoteCacheShapeV1("Token",value,`Closed remote cache token required in ${contract.auth.tokenEnvironment}`);return value;
}

/** 🎭️ Replaces a token by a marker for every text that reaches a terminal or a report. */
export const REMOTE_CACHE_REDACTED_V1="<redacted>";
