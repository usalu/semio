
const fs=require("node:fs"),path=require("node:path");
const root=process.argv[2];if(!root||!path.isAbsolute(root))throw Error("Explicit census root required");
const policy=JSON.parse(fs.readFileSync(path.join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json"),"utf8"));
const descriptor=JSON.parse(fs.readFileSync(path.join(root,"🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🧬️schema/🔣️.json"),"utf8")).definitions.RelativePreservationProtocol.const;
const retirement=JSON.parse(fs.readFileSync(path.join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧹️retirement/🧬️schema/🔣️.json"),"utf8")).definitions.Policy.const;
const prefixes=[descriptor.capsulePrefix,retirement.markerPrefix],generated=new Set(policy.generatedDirectories),groups=new Map(),largest=[],started=Date.now();let entries=0,files=0,bytes=0,nestedPrivate=0;
function excluded(relative){const components=relative.split("/");if(components.some(name=>generated.has(name)||prefixes.some(prefix=>name.startsWith(prefix)&&name.length===prefix.length+descriptor.capsuleDigestHex&&/^[a-f0-9]+$/.test(name.slice(prefix.length)))))return true;
 return relative===".🧬semio"||relative.startsWith(".🧬semio/")||relative==="assets/repo"||relative.startsWith("assets/repo/")||relative.includes("/asset/repo/")||["/dist/","/build/","/target/","/__pycache__/","/.next/","/coverage/"].some(part=>relative.includes(part))||path.basename(relative).endsWith(".Designer.cs")||relative.includes("/codegen/");
}
function walk(directory){for(const entry of fs.readdirSync(path.join(root,directory),{withFileTypes:true})){entries++;const relative=directory?directory+"/"+entry.name:entry.name;if(excluded(relative)||entry.isSymbolicLink())continue;
const key=relative.split("/").slice(0,2).join("/");let row=groups.get(key);if(!row){row={path:key,entries:0,files:0,bytes:0};groups.set(key,row)}row.entries++;
if(entry.isDirectory())walk(relative);else if(entry.isFile()){const size=fs.statSync(path.join(root,relative)).size;files++;bytes+=size;row.files++;row.bytes+=size;if(relative.includes("/.🧬semio/"))nestedPrivate++;largest.push({path:relative,bytes:size});largest.sort((a,b)=>b.bytes-a.bytes);if(largest.length>20)largest.pop();}}}
walk("");console.log("[DEBUG] native-source-census "+JSON.stringify({elapsedMs:Date.now()-started,entries,files,bytes,nestedPrivate,groups:[...groups.values()].sort((a,b)=>b.files-a.files).slice(0,30),largest}));
