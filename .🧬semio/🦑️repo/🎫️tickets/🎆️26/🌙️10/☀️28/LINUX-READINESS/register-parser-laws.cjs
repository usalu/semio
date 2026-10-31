const fs=require("node:fs"),path=require("node:path");const library="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",cache=library+"/⚡️caching",ticket=__dirname;
const report=fs.readFileSync(path.join(ticket,"schema-parser-law-registration.md"),"utf8"),files=[library+"/🎛️commands/🧬️schema/🪞️parser-projection/🚪️guard-helpers/🧪️tests/🟦️.ts","✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🚪️parsers/🟦️.ts"],resources=[...report.matchAll(/^- `(\{workspaceRoot\}\/[^\n`]+\.json)`/gm)].map(row=>row[1]);
if(resources.length!==7)throw Error("Expected seven actual DATA inputs");
const owner=path.posix.dirname(path.posix.dirname(path.posix.dirname(files[1]))),sourceFiles=[];
function visit(directory){for(const entry of fs.readdirSync(directory,{withFileTypes:true})){if(["🧪️tests","📦️packages"].includes(entry.name))continue;const file=directory+"/"+entry.name;if(entry.isDirectory())visit(file);else if(entry.name.endsWith(".ts"))sourceFiles.push(file)}}
visit(owner);
const suitePath=cache+"/🗂️suites/🔣️.json",projectPath=cache+"/📋️project.json",suites=JSON.parse(fs.readFileSync(suitePath,"utf8")),project=JSON.parse(fs.readFileSync(projectPath,"utf8")),suite=suites.suites.find(row=>row.id==="law-coverage"),target=project.targets["test-law-coverage"],count=Object.keys(project.targets).length;
const append=(array,values)=>{for(const value of values)if(!array.includes(value))array.push(value)};
for(const file of [...files,...sourceFiles])if(!fs.statSync(file).isFile())throw Error("Missing actual Source "+file);
append(suite.files,files.map(file=>path.relative(cache,file).split(path.sep).join("/")));
append(target.inputs,[...files.map(file=>"{workspaceRoot}/"+file),...resources,"{workspaceRoot}/"+owner+"/**/*.ts"]);
append(target.metadata.semio.runtimeSourceEntries,[...files,...sourceFiles].map(file=>"{workspaceRoot}/"+file));
append(target.metadata.semio.closure.resources,resources);
if(Object.keys(project.targets).length!==count)throw Error("Target identity changed");
fs.writeFileSync(suitePath,JSON.stringify(suites,null,2)+"\n");fs.writeFileSync(projectPath,JSON.stringify(project,null,2)+"\n");
fs.appendFileSync(path.join(ticket,"schema-parser-law-registration.md"),"\nRoot registered both laws in the existing canonical law-coverage row: "+suite.files.length+" files. Seven exact DATA resources and the actual owner TypeScript freshness glob are included; "+sourceFiles.length+" physical compiled sources join runtimeSourceEntries as Source. Existing "+count+" target identities are unchanged. Combined canonical acceptance remains pending.\n");
console.log(JSON.stringify({lawFiles:suite.files.length,sourceFiles:sourceFiles.length,resources:resources.length,targets:count}));