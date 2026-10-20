import {describe,expect,test} from "bun:test";
import {forwardRepoCommandV1,serviceEligibleV1,serviceModeV1} from "../client/repo.mjs";

describe("which commands go to the service",()=>{
 test("plain task commands qualify; flags that can change a task, separators and empty selections do not",()=>{
  for(const args of[["run","p:build"],["run","p:build","--output=stream"],["test","suite","--service=auto","--lang=de"],["run-many","a","b","--no-color"]])expect(serviceEligibleV1(args)).toBe(true);
  for(const args of[["run"],["run","p:build","--skip-cache"],["run","p:build","--configuration=prod"],["run","p:build","--","--watch"],["graph"],["watch","p:build"],["dependencies","sync"],["run","p:build","--lang","de"],["run","p:build","--tool=bazel"],[]])expect(serviceEligibleV1(args as string[])).toBe(false);
 });
 test("service mode: flag beats environment, default auto, unknown values are refused, nothing after -- counts",()=>{
  expect(serviceModeV1(["run","x"],{})).toBe("auto");
  expect(serviceModeV1(["run","x"],{SEMIO_BUILD_SERVICE:"off"})).toBe("off");
  expect(serviceModeV1(["run","x","--service=require"],{SEMIO_BUILD_SERVICE:"off"})).toBe("require");
  expect(serviceModeV1(["run","x","--service","off"],{})).toBe("off");
  expect(serviceModeV1(["run","x","--","--service=off"],{})).toBe("auto");
  expect(()=>serviceModeV1(["run","--service=sometimes"],{})).toThrow(/auto\|off\|require/u);
 });
});

describe("forwarding decision",()=>{
 const sink=()=>{const parts:string[]=[];return{parts,write:(t:string)=>{parts.push(String(t));return true;}};};
 const base={root:"/w",environment:{SEMIO_BUILD_TOOL:"bazel",UNRELATED:"x"} as Record<string,string>,languageOf:()=>"de"};
 test("an answered request returns the exit code and sends ids, bound environment and the preferred language order",async()=>{
  const calls:any[]=[];
  const code=await forwardRepoCommandV1({...base,args:["run","a:build","--output=stream"],forward:async o=>{calls.push(o);return{handled:true,exit:7,timing:{}};}});
  expect(code).toBe(7);expect(calls[0]).toMatchObject({command:"run",args:["a:build"],languages:["de","en"],env:{SEMIO_BUILD_TOOL:"bazel"}});expect(calls[0].env.UNRELATED).toBeUndefined();
 });
 test("off, ineligible, unanswered and unresolved language fall back with undefined; require fails instead",async()=>{
  const never=async()=>{throw new Error("must not be called");};
  expect(await forwardRepoCommandV1({...base,args:["run","a:build","--service=off"],forward:never as any})).toBeUndefined();
  expect(await forwardRepoCommandV1({...base,args:["run","a:build","--skip-cache"],forward:never as any})).toBeUndefined();
  expect(await forwardRepoCommandV1({...base,args:["run","a:build"],forward:async()=>({handled:false,reason:"down",timing:{}})})).toBeUndefined();
  expect(await forwardRepoCommandV1({...base,args:["run","a:build"],languageOf:()=>{throw new Error("unresolved");},forward:never as any})).toBeUndefined();
  const err=sink();
  expect(await forwardRepoCommandV1({...base,args:["run","a:build","--service=require"],stderr:err,forward:async()=>({handled:false,reason:"down",timing:{}})})).toBe(1);expect(err.parts.join("")).toContain("down");
  expect(await forwardRepoCommandV1({...base,args:["run","a:build","--service=require"],languageOf:()=>{throw new Error("unresolved");},forward:never as any})).toBe(1);
 });
});
