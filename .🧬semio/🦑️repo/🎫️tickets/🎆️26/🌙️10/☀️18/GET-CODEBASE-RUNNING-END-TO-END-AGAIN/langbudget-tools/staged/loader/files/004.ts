/** 🏷️ Contract: when the loader or the snapshot a captured command proves differs from the one this process loaded (a peer saved the loader while the front door waited for capacity), the command ends as a stale implementation (the outermost command starts again, exit 75), while a malformed proof stays a plain refusal. */
import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {requireCapturedBuildCommandV1,type CapturedBuildCommandProofV1} from "../../🟦️.ts";
import {buildSourceClosureLoaderV1,buildSourceClosureStreamLoaderV1} from "../../../🟦️.ts";
import {spawnSync} from "node:child_process";
import {sealedPolicyV1} from "../../../../../🏛️sealed/🟦️.ts";
import {closureRefusalMessagesV1} from "../../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🟦️.ts";
import {STALE_IMPLEMENTATION_CODES_V1,concurrentEditOfV1} from "../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";

const sha=(text:string)=>createHash("sha256").update(text).digest("hex"),loader=sha(buildSourceClosureStreamLoaderV1),entry=sha("entry");
const proof=(change:Partial<{root:string;sha256:string;loaderSha256:string}>={}):CapturedBuildCommandProofV1=>({version:1,root:process.cwd(),snapshot:null,sha256:entry,loaderSha256:loader,recheck:()=>{},...change});
const failure=(run:()=>void):Error&{code?:string}=>{try{run();}catch(error){return error as Error;}throw Error("A refusal was expected");};

test("a proof that matches the loaded loader and snapshot is accepted",()=>{expect(()=>requireCapturedBuildCommandV1(proof(),entry,loader)).not.toThrow();});

test("a loader or snapshot that differs from the loaded one is a stale implementation, never a plain refusal",()=>{
 for(const[name,run]of[["loader identity",()=>requireCapturedBuildCommandV1(proof(),entry,sha("other loader"))],["snapshot identity",()=>requireCapturedBuildCommandV1(proof(),sha("other entry"),loader)],["loaded constant",()=>requireCapturedBuildCommandV1(proof({loaderSha256:sha("peer saved")}),entry,sha("peer saved"))]] as const){
  const error=failure(run);expect(STALE_IMPLEMENTATION_CODES_V1.has(error.code!),name).toBe(true);expect(error.message,name).toContain("restart required");expect(concurrentEditOfV1(error),name).toBeUndefined();
 }
});

test("a malformed proof stays a plain refusal",()=>{
 for(const bad of[proof({root:"relative/root"}),{...proof(),recheck:undefined as never},proof({sha256:"short"})]){const error=failure(()=>requireCapturedBuildCommandV1(bad,entry,loader));expect(error.message).toBe("invalid build.closure.command-entry-authentication-required {}");expect(error.code).toBe("invalid");}
});

const commandLineLengthV1=(argv:readonly string[])=>Number(spawnSync("python3",["-I","-c","import json,subprocess,sys;print(len(subprocess.list2cmdline(json.load(sys.stdin)))+1)"],{input:JSON.stringify(argv),encoding:"utf8"}).stdout);
const headroomCharactersV1=4500;
/** 🧮️ Contract: the `--eval` loader of a captured command is part of its command line, which Windows bounds at 32767 characters, so the loader stays below that bound minus the room an executable path and the arguments need (measured against Python's independent command-line quoting), and carries no refusal text: the catalog may grow without limit because the loader renders from the authenticated snapshot instead of inlining it. */
test("the captured command line stays within the operating system limit with room for arguments, whatever the refusal catalog holds",()=>{
 const limit=sealedPolicyV1.maximumCommandLineCharacters,executable="/"+"x".repeat(259),digest="0".repeat(64),worst=["run","project:target","--configuration=release","--parallelism=8","--","a\"".repeat(1500)],argv=[executable,"--eval",buildSourceClosureStreamLoaderV1,"--",digest,...worst];
 expect(commandLineLengthV1(argv),"command line of the longest realistic invocation").toBeLessThanOrEqual(limit);
 expect(commandLineLengthV1([executable,"--eval",buildSourceClosureStreamLoaderV1,"--",digest]),"command line without arguments").toBeLessThanOrEqual(limit-headroomCharactersV1+executable.length);
 expect(buildSourceClosureStreamLoaderV1.length,"stream loader characters").toBeLessThanOrEqual(limit-headroomCharactersV1);
 for(const[name,loader]of[["stream",buildSourceClosureStreamLoaderV1],["file",buildSourceClosureLoaderV1]] as const)for(const[code,row]of Object.entries(closureRefusalMessagesV1))for(const language of Object.values(row))expect(loader.includes(language.refusal.split("{")[0]!),name+" loader inlines the "+code+" refusal text").toBe(false);
});
