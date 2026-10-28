import{readFileSync}from"node:fs";import{resolve,relative}from"node:path";
import{discoverProcessTestFilesV1,expandProcessTestSuiteV1,readProcessTestTableV1}from"../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🗂️suites/🟦️.ts";
const L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const tables=[["build",L+"🏗️build"],["caching",L+"⚡️caching"],["ci",L+"🏗️build/🧾️ci-report"]].slice(0,2);
const files=readFileSync(process.argv[2],"utf8").split("\n").filter(Boolean);
for(const[n,root]of tables){
 const abs=resolve(root),table=readProcessTestTableV1(JSON.parse(readFileSync(abs+"/🗂️suites/🔣️.json","utf8")));
 const found=discoverProcessTestFilesV1(abs);
 for(const s of table.suites){const exp=expandProcessTestSuiteV1(s,found,table);for(const f of exp){const full=relative(process.cwd(),resolve(abs,f));if(files.includes(full))console.log(n,s.id,s.budgetMs,JSON.stringify(s.capabilities??null),JSON.stringify(s.fileLaunchers??null),"|",full.split("/").slice(-4).join("/"));}}
}
