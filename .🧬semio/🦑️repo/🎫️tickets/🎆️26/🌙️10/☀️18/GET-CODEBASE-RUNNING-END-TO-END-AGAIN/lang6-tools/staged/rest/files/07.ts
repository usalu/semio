/** 🧫️ Laws of test store isolation: the rules agree with their schema, the scanner agrees with an independent Python model on the fixtures, no test file of the repository reaches the checkout's live store, and the test preload fences that store at runtime. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {dirname,join,resolve,sep} from "node:path";
import {fileURLToPath} from "node:url";
import {runToolSyncV1,ToolErrorV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import {requireMaterializedPathV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {repositoryCacheBudgetV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts";
import {repositoryLeaseDirectoryV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import {repositoryStoreDirectoriesV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🚧️fence/🟦️.ts";
import {privateRepositoryWorkspaceV1,requirePrivateRepositoryWorkspaceV1,testCheckoutRootV1} from "../../../🧪️testing/🧫️private-repository/🟦️.ts";
import {readTestStoreRulesV1,scanTestStoreReachesV1,testStoreIsolationReportV1,TEST_STORE_RULES_V1} from "../🟦️.ts";
import rulesDocument from "../🔣️.json" with {type:"json"};
import rulesSchema from "../🧬️schema/🔣️.json" with {type:"json"};
import policy from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import fixtures from "./🧫️fixtures/🔣️.json" with {type:"json"};

const here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"🐍️oracle","🐍️.py"),checkout=testCheckoutRootV1();
/** 🐍️ Runs the independent Python model, or reports that no Python is installed. */
const python=(args:string[]):string|undefined=>{try{return runToolSyncV1("python",[oracle,...args],{encoding:"utf8",check:true,maxBuffer:16*1024*1024}).stdout;}catch(error){if(error instanceof ToolErrorV1&&error.code==="tool-missing")return undefined;throw error;}};

test("test store isolation rules agree with their schema and refuse a broken document",()=>{
 expect(new Ajv({strict:false,allErrors:true}).validate(rulesSchema,rulesDocument)).toBe(true);
 expect(readTestStoreRulesV1(rulesDocument)).toEqual(TEST_STORE_RULES_V1);
 expect(()=>readTestStoreRulesV1({...rulesDocument,storeCalls:[]})).toThrow("test-store-isolation-rules-invalid");
 expect(()=>readTestStoreRulesV1({...rulesDocument,storeLiterals:["("]})).toThrow();
});

test("the scanner finds exactly the fixture reaches and the independent Python model agrees",()=>{
 for(const fixture of fixtures.cases)expect({name:fixture.name,findings:scanTestStoreReachesV1(fixture.source,undefined,(fixture as {path?:string}).path).map(({line,rule})=>({line,rule}))}).toEqual({name:fixture.name,findings:fixture.findings as any});
 const directory=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??resolve(here),"tsi-"));
 try{const rules=join(directory,"rules.json"),cases=join(directory,"fixtures.json");writeFileSync(rules,JSON.stringify(rulesDocument));writeFileSync(cases,JSON.stringify(fixtures));const output=python(["fixtures",rules,cases]);if(output!==undefined)expect(JSON.parse(output)).toEqual(fixtures.cases.map(fixture=>({name:fixture.name,findings:fixture.findings})));}
 finally{rmSync(directory,{recursive:true,force:true});}
});

test("no test file of the repository reaches the checkout's live store",()=>{
 const report=testStoreIsolationReportV1(checkout);
 expect(report.scanned).toBeGreaterThan(1000);
 expect(report.findings.map(finding=>finding.path+":"+finding.line+" "+finding.rule+" "+finding.text)).toEqual([]);
 expect(report.stale).toEqual([]);
},120000);

test("the test preload fences the checkout's live store and leaves a private repository open",async()=>{
 const fenced=repositoryStoreDirectoriesV1(checkout),output=python(["fence",fileURLToPath(new URL("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json",import.meta.url)),checkout]);
 expect(fenced).toHaveLength(Object.keys(policy.partitions).length+1);
 if(output!==undefined)expect(fenced).toEqual(JSON.parse(output));
 for(const directory of fenced){expect(()=>requireMaterializedPathV1(directory)).toThrow("live repository store");expect(()=>requireMaterializedPathV1(join(directory,"m","data"))).toThrow("aktiven Repository-Speicher");}
 expect(()=>repositoryLeaseDirectoryV1(checkout)).toThrow("live repository store");
 expect(()=>repositoryCacheBudgetV1(checkout)).toThrow("live repository store");
 expect(requireMaterializedPathV1(join(checkout,".cache","b-neighbour"))).toBe(join(checkout,".cache","b-neighbour"));
 const repository=await privateRepositoryWorkspaceV1();
 expect(await repository.requireIsolated()).toBe(repository.root);
 expect(repositoryLeaseDirectoryV1(repository.root).startsWith(repository.root+sep)).toBe(true);
 expect(()=>requirePrivateRepositoryWorkspaceV1(checkout)).toThrow("private-repository-workspace-required");
 expect(()=>requirePrivateRepositoryWorkspaceV1(dirname(repository.root))).toThrow("private-repository-workspace-required");
});
