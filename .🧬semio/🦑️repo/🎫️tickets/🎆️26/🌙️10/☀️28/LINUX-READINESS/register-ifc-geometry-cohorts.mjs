import fs from "node:fs";import path from "node:path";
const ticket=import.meta.dirname,cache="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching",suiteFile=cache+"/🗂️suites/🔣️.json",projectFile=cache+"/📋️project.json",roster=JSON.parse(fs.readFileSync(path.join(ticket,"🗑️generated/native-ifc-geometry-cohort-inputs.json"),"utf8")),table=JSON.parse(fs.readFileSync(suiteFile,"utf8")),project=JSON.parse(fs.readFileSync(projectFile,"utf8")),base="laws-ifc-native-components",template=project.targets["test-"+base],resource=file=>"{workspaceRoot}/"+file,prior=[...project.targets.test.dependsOn];
if(roster.cohorts.length!==2||!template)throw Error("Two actual IFC cohorts and prior owner required");
for(const row of roster.cohorts){
 const id=row.id==="production"?"laws-ifc-geometry-ownership":row.id==="units"?"laws-ifc-geometry-units":null;if(!id)throw Error("Closed cohort required");
 if(table.suites.some(x=>x.id===id)||project.targets["test-"+id])throw Error("Existing cohort requires review");
 if(row.resources.length>64||row.runtimeSource.length>64||!row.allExist)throw Error("Complete bounded roster required");
 for(const file of [...row.resources,...row.runtimeSource,...row.freshness])if(!fs.existsSync(file))throw Error("Actual cohort input missing: "+file);
 table.suites.push({id,budgetMs:300000,typecheck:"none",files:[path.relative(cache,row.law).split(path.sep).join("/")],platforms:["darwin","linux","win32"]});
 project.targets["test-"+id]={executor:"repo:script",cache:true,dependsOn:[],outputs:[],inputs:["default","^default","testEnv","reads",...new Set(row.freshness.map(resource))],options:{...template.options,command:template.options.command.slice(0,-base.length)+id},metadata:{semio:{execution:{version:1,kind:"source"},runtimeSourceEntries:row.runtimeSource.map(resource),closure:{resources:row.resources.map(resource)}}}};
}
fs.writeFileSync(suiteFile,JSON.stringify(table,null,2)+"\n");
const {deriveCanonicalTestAggregateV1,requireCanonicalTestAggregateV1,canonicalTestAggregatePolicyV1:policy}=await import("./../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🗂️suites/🧮️aggregate/🟦️.ts");
project.targets[policy.target]=deriveCanonicalTestAggregateV1({...project,root:policy.root});if(prior.some(x=>!project.targets[policy.target].dependsOn.includes(x)))throw Error("Prior canonical leaf lost");requireCanonicalTestAggregateV1(project,policy.root);fs.writeFileSync(projectFile,JSON.stringify(project,null,2)+"\n");
console.log(JSON.stringify({prior:prior.length,dependencies:project.targets[policy.target].dependsOn.length,targets:Object.keys(project.targets).length,cohorts:roster.cohorts.map(x=>({id:x.id,resources:x.resources.length,runtime:x.runtimeSource.length,tests:x.tests}))}));
