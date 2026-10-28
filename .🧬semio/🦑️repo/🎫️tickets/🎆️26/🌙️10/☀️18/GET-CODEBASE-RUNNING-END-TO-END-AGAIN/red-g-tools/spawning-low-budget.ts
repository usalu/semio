import{readFileSync}from"node:fs";import{join}from"node:path";
const root="/Users/ueli/Documents/semio/",P=root+"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🗂️suites/🟦️.ts",D=root+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🗂️suites/";
const{discoverProcessTestFilesV1,expandProcessTestSuiteV1}=await import(P),{cachingTestRootV1,readCachingTestTableV1}=await import(D+"🟦️.ts"),fixture=JSON.parse(readFileSync(D+"🧪️tests/🧫️fixtures/🔣️.json","utf8"));
const declared=readCachingTestTableV1(),found=discoverProcessTestFilesV1(cachingTestRootV1);
for(const suite of declared.suites){if(suite.budgetMs>=fixture.levels.quick)continue;const files=expandProcessTestSuiteV1(suite,found,declared).filter((f:string)=>!f.startsWith(".."));for(const f of files)if(fixture.spawnTokens.some((t:string)=>readFileSync(join(cachingTestRootV1,f),"utf8").includes(t)))console.log(suite.id,suite.budgetMs,f);}
