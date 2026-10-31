async function census(){
const fs=require("node:fs"),path=require("node:path");
const root=process.cwd(),ticket=__dirname,caching="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/";
const read=p=>JSON.parse(fs.readFileSync(path.join(root,p),"utf8"));
const excluded=new Set(read(caching+"🔣️policy.json").generatedDirectories);
const protocol=read("🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🧬️schema/🔣️.json").definitions.RelativePreservationProtocol.const;
const retirement=read(caching+"🚀️bootstrap/📦️dependencies/💾️storage/🧹️retirement/🧬️schema/🔣️.json").definitions.Policy.const;
const result={at:new Date().toISOString(),directories:0,symlinks:0,excluded:0,capsules:[],journals:[],otherP:[],errors:[],policy:[...excluded].sort()};
const reserved=(name,prefix)=>name.startsWith(prefix)&&name.length===prefix.length+protocol.capsuleDigestHex&&/^[a-f0-9]+$/u.test(name.slice(prefix.length));
let cancelled=false;process.once("SIGINT",()=>{cancelled=true});
const pending=[""];
while(pending.length&&!cancelled){
 const relative=pending.pop(),absolute=path.join(root,relative);let entries;
 try{const stat=fs.lstatSync(absolute);if(stat.isSymbolicLink()){result.symlinks++;continue}if(!stat.isDirectory())continue;entries=fs.readdirSync(absolute,{withFileTypes:true});result.directories++}catch(e){result.errors.push({path:relative,code:e.code});continue}
 for(const entry of entries){const p=relative?relative+"/"+entry.name:entry.name;if(entry.isSymbolicLink()){result.symlinks++;if(reserved(entry.name,protocol.capsulePrefix))result.capsules.push(p);else if(reserved(entry.name,retirement.markerPrefix))result.journals.push(p);else if(entry.name.startsWith(".p"))result.otherP.push({path:p,kind:"symlink"});continue}if(reserved(entry.name,protocol.capsulePrefix)){result.capsules.push(p);continue}if(reserved(entry.name,retirement.markerPrefix)){result.journals.push(p);continue}if(entry.name.startsWith(".p"))result.otherP.push({path:p,kind:entry.isDirectory()?"directory":"file"});if(entry.isDirectory()){if(excluded.has(entry.name)){result.excluded++;continue}pending.push(p)}}
 if(result.directories%50000===0)process.stderr.write("[DEBUG] Source census directories="+result.directories+"\n");
 if(result.directories%256===0)await new Promise(setImmediate);
}
result.cancelled=cancelled;
for(const key of ["capsules","journals"])result[key].sort();
result.otherP.sort((a,b)=>a.path.localeCompare(b.path));
const generated=path.join(ticket,"🗑️generated");fs.mkdirSync(generated,{recursive:true});
const output=cancelled?"capsule-interrupted-census.json":"capsule-current-census.json";
fs.writeFileSync(path.join(generated,output),JSON.stringify(result,null,2)+"\n");
const report="\n## Fresh Source Census "+result.at+"\n\nObserved "+result.directories+" directories, skipped "+result.symlinks+" symbolic-link endpoints and "+result.excluded+" protected/generated directory endpoints. Capsules: **"+result.capsules.length+"**; retirement journals: **"+result.journals.length+"**; filesystem errors: "+result.errors.length+"; cancelled: "+cancelled+". Exact reserved names and exclusions were read from current authored schemas and caching policy. The scan skipped observed symlinks and did not inspect protected cache interiors or delete anything. Concurrent path replacement is outside this read-only census guarantee.\n\nOther .p-prefixed Source entries:\n\n"+result.otherP.map(x=>"- "+x.path+" ("+x.kind+")").join("\n")+"\n\nRaw census: 🗑️generated/"+output+".\n";
fs.appendFileSync(path.join(ticket,"capsule-current-verification.md"),report);
process.stdout.write(JSON.stringify({...result,policy:undefined})+"\n");
process.exitCode=cancelled?130:result.errors.length||result.capsules.length||result.journals.length?1:0;
}
census().catch(error=>{console.error("[DEBUG] Source census failed",error);process.exitCode=1;});
