const fs=require("node:fs"),path=require("node:path"),crypto=require("node:crypto");
const ticket=__dirname,roster=JSON.parse(fs.readFileSync(path.join(ticket,"🗑️generated/schema-bim-registration-inputs.json"),"utf8")),library="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",cache=library+"/⚡️caching";
if(roster.accepted?.owner?.tests!==68||roster.source.length!==27||roster.data.length!==7)throw Error("Accepted complete BIM roster required");
for(const row of roster.accepted.sources){const before=fs.lstatSync(row.path,{bigint:true});if(!before.isFile()||before.isSymbolicLink()||path.resolve(row.path).length>256)throw Error("Physical bounded BIM Source required");const hash=crypto.createHash("sha256").update(fs.readFileSync(row.path)).digest("hex"),after=fs.lstatSync(row.path,{bigint:true});if(hash!==row.sha256||["dev","ino","size","mtimeNs","ctimeNs"].some(key=>before[key]!==after[key]))throw Error("BIM Source checkpoint changed: "+row.path);}
for(const file of[roster.law,...roster.data])if(!fs.lstatSync(file).isFile())throw Error("Actual BIM input required");
const suiteFile=cache+"/🗂️suites/🔣️.json",projectFile=cache+"/📋️project.json",table=JSON.parse(fs.readFileSync(suiteFile,"utf8")),project=JSON.parse(fs.readFileSync(projectFile,"utf8")),suite=table.suites.find(row=>row.id==="law-coverage"),target=project.targets["test-law-coverage"],append=(list,values)=>{for(const value of values)if(!list.includes(value))list.push(value);};
append(suite.files,[path.relative(cache,roster.law).split(path.sep).join("/")]);
append(target.inputs,[roster.law,...roster.source,...roster.freshness].map(file=>"{workspaceRoot}/"+file));
const owner=path.posix.dirname(path.posix.dirname(path.posix.dirname(roster.law))),moduleInputs=new Set(roster.source.map(file=>"{workspaceRoot}/"+file));
target.metadata.semio.runtimeSourceEntries=target.metadata.semio.runtimeSourceEntries.filter(file=>!moduleInputs.has(file));
append(target.metadata.semio.runtimeSourceEntries,["{workspaceRoot}/"+roster.law]);
target.metadata.semio.closure.modules??=[];
append(target.metadata.semio.closure.modules,["{workspaceRoot}/"+owner+"/**/*.ts","!{workspaceRoot}/"+owner+"/**/🧪️tests/**"]);
append(target.metadata.semio.closure.resources,roster.data.map(file=>"{workspaceRoot}/"+file));
fs.writeFileSync(suiteFile,JSON.stringify(table,null,2)+"\n");fs.writeFileSync(projectFile,JSON.stringify(project,null,2)+"\n");
fs.appendFileSync(path.join(ticket,"schema-bim-law-registration.md"),"\nRoot registered accepted actual BIM law in existing law-coverage row: "+suite.files.length+" files;27Source entries plus law,7deduplicated DATA/config authorities, owner freshness and exact Source hashes checked. Canonical target identities are unchanged. Whole Project and aggregate validation follows.\n");
console.log(JSON.stringify({lawFiles:suite.files.length,source:roster.source.length,data:roster.data.length}));
