import {access,stat} from "node:fs/promises";
import {constants} from "node:fs";
import {delimiter,dirname,join,resolve} from "node:path";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import policy from "./🔣️policy.json" with { type: "json" };
import {buildPreflightContainerV1,buildPreflightRemediesV1} from "./💊️remedies/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {nativeDirectoryHostAvailableBytesV1,probeNativeDirectoryCaseCapabilityV1,probeNativeDirectoryExchangeV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🟦️.ts";
import {checkScriptInvocation,requireIssuedScriptInvocationV1,advanceScriptInvocation,type ScriptInvocation} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🩺️ Declares the finite per-platform requirements of the whole workspace before any store is provisioned. */
export type BuildPreflightPlatformV1 = Readonly<{architectures:readonly("x64"|"arm64")[];minimumAvailableBytes:number;tools:readonly string[];exchange:boolean}>;
/** 🔬️ Captures read-only facts from the actual invoking process and the filesystem that will hold the stores. */
export type BuildPreflightFactsV1 = Readonly<{platform:string;architecture:string;availableBytes:number;tools:readonly string[];exchange:"supported"|"unsupported"|"unprobed";caseSensitive:"yes"|"no"|"unprobed"}>;

/** 🎯️ Accepts only the bare command; preflight has no scope, bypass or provisioning options. */
export function readBuildPreflightArgumentsV1(args:readonly string[]):void {
 if(args.length!==1||args[0]!=="preflight")throw Error("Expected repo preflight without arguments");
}
/** 📋️ Resolves the authored closed requirement metadata without invoking discovery or opening stores. */
export function readBuildPreflightPolicyV1():{version:1;platforms:Record<"linux"|"darwin"|"win32",BuildPreflightPlatformV1>} {
 if(validateJsonSchemaSubset(schema.definitions.Policy,policy,schema).length)throw Error("Closed build preflight policy required");return policy as never;
}
/** ⚖️ Reports every unmet authored requirement without pretending that observations grant compiler authority. */
export function buildPreflightProblemsV1(requirements:unknown,facts:unknown):string[] {
 if(validateJsonSchemaSubset(schema.definitions.Platform,requirements,schema).length)throw Error("Closed build preflight requirements required");if(validateJsonSchemaSubset(schema.definitions.Facts,facts,schema).length)throw Error("Closed build preflight facts required");
 const required=requirements as BuildPreflightPlatformV1,actual=facts as BuildPreflightFactsV1,problems:string[]=[];
 if(!(required.architectures as readonly string[]).includes(actual.architecture))problems.push(`architecture ${actual.architecture} is not one of ${required.architectures.join(", ")}`);
 if(actual.availableBytes<required.minimumAvailableBytes)problems.push(`available bytes ${actual.availableBytes} are below required ${required.minimumAvailableBytes}`);
 if(required.exchange&&actual.exchange!=="supported")problems.push("atomic or journaled directory swap is unsupported by the host or the workspace filesystem");
 if(required.exchange&&actual.caseSensitive!=="yes")problems.push("a case-sensitive workspace filesystem is required for directory-backed partitions");
 for(const tool of required.tools)if(!actual.tools.includes(tool))problems.push(`missing executable: ${tool}`);return problems;
}
/** 🧭️ Checks the actual host and its finite capacity before any managed allocation is attempted. */
export async function runBuildPreflightV1(root:string,args:readonly string[],invocation:ScriptInvocation):Promise<void> {
 requireIssuedScriptInvocationV1(invocation);checkScriptInvocation(invocation);readBuildPreflightArgumentsV1(args);
 const platform=process.platform,rows=readBuildPreflightPolicyV1().platforms as Record<string,BuildPreflightPlatformV1>,required=rows[platform];
 if(!required)throw Error(`Repository preflight refused before store allocation:\nplatform ${platform} is not supported`);
 await advanceScriptInvocation(invocation,"preflight","running");
 const tools:string[]=[],extensions=platform==="win32"?(process.env.PATHEXT??".EXE").split(";"):[""];
 for(const tool of required.tools){search:for(const directory of(process.env.PATH??"").split(delimiter)){if(!directory)continue;for(const extension of extensions){try{const path=join(directory,tool+extension);if(!(await stat(path)).isFile())continue;await access(path,constants.X_OK);tools.push(tool);break search;}catch{}}}checkScriptInvocation(invocation);}
 const cache=resolve(root,".cache");let host=cache;for(;;){try{await stat(host);break;}catch{const parent=dirname(host);if(parent===host)break;host=parent;}}
 const availableBytes=nativeDirectoryHostAvailableBytesV1(host);let exchange:BuildPreflightFactsV1["exchange"]="unprobed",caseSensitive:BuildPreflightFactsV1["caseSensitive"]="unprobed";
 const details:string[]=[];
 if(required.exchange){try{probeNativeDirectoryExchangeV1(host);exchange="supported";}catch(error){exchange="unsupported";details.push(`exchange probe beneath ${host}: ${error instanceof Error?error.message:String(error)}`);}checkScriptInvocation(invocation);try{caseSensitive=probeNativeDirectoryCaseCapabilityV1(host)?"yes":"no";}catch(error){caseSensitive="no";details.push(`case probe beneath ${host}: ${error instanceof Error?error.message:String(error)}`);}}
 checkScriptInvocation(invocation);const facts={platform,architecture:process.arch,availableBytes,tools,exchange,caseSensitive},problems=buildPreflightProblemsV1(required,facts);
 if(problems.length)throw Error(`Repository preflight refused before store allocation:\n${[...problems,...details,...buildPreflightRemediesV1(facts,required.tools,required.exchange,{container:buildPreflightContainerV1()})].join("\n")}`);
 await advanceScriptInvocation(invocation,"preflight","complete");
}
