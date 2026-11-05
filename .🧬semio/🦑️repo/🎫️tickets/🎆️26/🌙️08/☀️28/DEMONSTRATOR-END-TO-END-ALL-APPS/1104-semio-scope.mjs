import {readFileSync,writeFileSync,existsSync} from "node:fs";
import {resolve,dirname,join,relative} from "node:path";
import TOML from "@iarna/toml";
const checkout=process.cwd(),cache=new Map();
function read(path){if(!cache.has(path))cache.set(path,TOML.parse(readFileSync(path,"utf8")));return cache.get(path)}
function workspace(path){const value=read(path);if(value.workspace)return path;if(typeof value.package?.workspace==="string")return resolve(dirname(path),value.package.workspace,"Cargo.toml");for(let parent=dirname(path);;){const candidate=join(parent,"Cargo.toml");if(existsSync(candidate)&&read(candidate).workspace)return candidate;const next=dirname(parent);if(next===parent)throw Error("workspace unavailable "+path);parent=next}}
function dependencies(value,dev){return [value,...Object.values(value.target??{})].flatMap(row=>[row.dependencies,row["build-dependencies"],...(dev?[row["dev-dependencies"]]:[])]).flatMap(row=>Object.entries(row??{}))}
const root=resolve(process.argv[2]??"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/📦️packages/🦀️rust/Cargo.toml"),pending=[root],parents=new Map([[root,null]]),unresolved=[];
for(let index=0;index<pending.length;index++){const path=pending[index];for(const[name,entry]of dependencies(read(path),path===root)){let spec=entry,base=dirname(path);if(spec&&typeof spec==="object"&&spec.workspace){const owner=workspace(path);base=dirname(owner);spec=read(owner).workspace.dependencies?.[name];if(spec===undefined){unresolved.push({path:relative(checkout,path),name});continue}}if(!spec||typeof spec!=="object"||typeof spec.path!=="string")continue;const next=resolve(base,spec.path,"Cargo.toml");if(!existsSync(next))throw Error("missing "+next);if(!parents.has(next)){parents.set(next,path);pending.push(next)}}}
const packages=pending.map(path=>({name:read(path).package?.name,path:relative(checkout,path),parent:parents.get(path)?relative(checkout,parents.get(path)):null})),targets=packages.filter(row=>/os-flow-host|os-kernel-neural-engine|semio-s-artifact-flow-flow-editor/.test(row.name));
const label=process.argv[3]??"semio";if(!/^[a-z0-9-]+$/.test(label))throw Error("invalid scope label");
const result={root:relative(checkout,root),conservative:true,includesRootDev:true,includesAllOptionalAndTargetDependencies:true,includesBuildDependencies:true,packages,unresolved,targets};
writeFileSync(new URL("./🗑️generated/1104-"+label+"-scope.json",import.meta.url),JSON.stringify(result,null,2)+String.fromCharCode(10));
console.log(JSON.stringify({packages:packages.length,unresolved,targets}));
