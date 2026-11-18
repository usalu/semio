/** ❓️ Contract: a target that names an input no group states is refused as a typed refusal that names the owner and the input, rendered in English and German, never as a raw error that a service reports as unexpected. */
import{expect,test}from"bun:test";
import{textV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import{isRepoErrorV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";
import{BUILD_TEXTS_V1}from"../../../../📖️texts/🟦️.ts";
import{buildSourceContractGroupsV1}from"../../🟦️.ts";

test("the selected input walk refuses an unknown named input with the owner and the input in both languages",()=>{
 let caught:unknown;
 try{buildSourceContractGroupsV1({name:"@fixture/owner",targets:{build:{inputs:["default","scriptProvider:absent-port:scheduling"]}}},{default:["{projectRoot}/**/*"]});}catch(error){caught=error;}
 expect(isRepoErrorV1(caught)).toBe(true);
 const error=caught as {code:string;key:string;values:Record<string,string>};
 expect(error.key).toBe("build.graph.unknown-source-input");
 expect(error.values).toEqual({owner:"@fixture/owner",input:"scriptProvider:absent-port:scheduling"});
 const en=textV1(BUILD_TEXTS_V1.catalog,"graph.unknown-source-input","en",error.values),de=textV1(BUILD_TEXTS_V1.catalog,"graph.unknown-source-input","de",error.values);
 for(const text of[en,de]){expect(text).toContain("@fixture/owner");expect(text).toContain("scriptProvider:absent-port:scheduling");}
 expect(en).not.toBe(de);expect(en).toContain("Unknown");expect(de).toContain("Unbekannte");
});
