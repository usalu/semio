import{readFileSync,writeFileSync}from"node:fs";import{resolve,dirname,relative,isAbsolute}from"node:path";import{pathToFileURL}from"node:url";import{parse as oracleParse}from"smol-toml";
const root=process.cwd(),ticket=resolve(root,".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END"),start="🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/Cargo.toml";
const repo=await import(pathToFileURL(resolve(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts")).href);
const stop=AbortSignal.timeout(120000),slash=(path:string)=>path.replaceAll("\\","/"),local=(path:string)=>slash(relative(root,path));
const documents=new Map<string,any>(),scopes=new Map<string,any>(),members=new Map<string,Set<string>>(),rows:any[]=[],issues:any[]=[],pending=[{manifest:start,via:"root"},{manifest:"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/📦️packages/🦀️rust/Cargo.toml",via:"explicit NativePoll authority audit"}],seen=new Set<string>();
function document(path:string):any{let value=documents.get(path);if(!value){value=oracleParse(readFileSync(resolve(root,path),"utf8"));documents.set(path,value);}return value;}
function scope(manifest:string,current:any):any{
 let key:string|undefined;if(typeof current.package?.workspace==="string")key=local(resolve(root,dirname(manifest),current.package.workspace,"Cargo.toml"));
 let owner=key?scopes.get(key):undefined;if(!owner){owner=repo.cargoWorkspaceForManifest(root,manifest);if(key&&owner.manifest!==key)throw Error("independent owner resolution differs: "+manifest);scopes.set(owner.manifest,owner);}return owner;
}
while(pending.length){
 stop.throwIfAborted();const item=pending.shift()!;if(seen.has(item.manifest))continue;seen.add(item.manifest);
 try{
  const current=document(item.manifest),owner=scope(item.manifest,current);let inventory=members.get(owner.manifest);
  if(!inventory){inventory=new Set(repo.cargoWorkspaceMembers(root,owner).map((row:any)=>row.manifest));members.set(owner.manifest,inventory);}
  const admitted=inventory.has(item.manifest),row={manifest:item.manifest,name:current.package?.name,owner:owner.manifest,workspace:current.package?.workspace,admitted,via:item.via};rows.push(row);if(!admitted)issues.push({...row,kind:"missing-authored-member"});
  const authority=document(owner.manifest).workspace,groups=[current,...Object.values(current.target??{})]as any[];
  for(const group of groups)for(const kind of item.manifest===start?["dependencies","dev-dependencies","build-dependencies"]:["dependencies","build-dependencies"])for(const[alias,value]of Object.entries(group[kind]??{})){
   if(!value||typeof value!=="object")continue;const entry=value as any,dependency=entry.workspace?authority.dependencies?.[alias]:entry;
   if(!dependency){issues.push({manifest:item.manifest,kind:"missing-inherited-dependency",alias,owner:owner.manifest});continue}
   if(typeof dependency.path!=="string")continue;const path=local(resolve(root,entry.workspace?owner.directory:dirname(item.manifest),dependency.path,"Cargo.toml"));
   if(path.startsWith("../")||isAbsolute(path)){issues.push({manifest:item.manifest,kind:"escaping-path",alias,path});continue}pending.push({manifest:path,via:item.manifest+" :: "+alias});
  }
 }catch(error){issues.push({manifest:item.manifest,via:item.via,kind:"read-or-authority-refusal",message:error instanceof Error?error.message:String(error)});}
 if(seen.size%100===0)console.error("[DEBUG] MCP member census "+seen.size+" manifests");
}
const result={start,packages:rows.length,workspaces:[...members.keys()],issues,rows};writeFileSync(resolve(ticket,"🗑️generated/mcp-membership-census.json"),JSON.stringify(result,null,2));
const note=["# MCP Reachable Cargo Membership — 2026-11-06","","Read-only census follows the same dependency groups as Cargo preparation: MCP root development dependencies and every normal/build/target path dependency recursively, including inherited workspace paths. NativePoll is also audited explicitly because its package is not currently reachable from MCP declarations. Actual repository scopes and member inventories are selected through the existing first-party Cargo interface; smol-toml independently parses package workspace/dependency fields. No preparation recipe, compiler, implicit member or authority bypass runs.",""];note.push("Packages: "+rows.length+"; workspaces: "+members.size+"; issues: "+issues.length+".","",...issues.map(row=>"- "+JSON.stringify(row)),"");writeFileSync(resolve(ticket,"mcp-membership-census-2026-11-06.md"),note.join("\n"));
console.log(JSON.stringify({packages:rows.length,workspaces:[...members.keys()],issues}));
