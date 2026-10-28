X = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs"
XD = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.d.mts"
EDITS = [
(X, ''' return {projects:shareNxProjectInputsV1(projects),externalNodes:graph.externalNodes??{}};''', ''' return {projects:shareNxProjectInputsV1(liftNxNamedInputReferencesV1(projects)),externalNodes:graph.externalNodes??{}};
}

/** 🪜️ Nx refuses a named input that refers to another project, while the build graph names a provider closure as a `{input,projects}` row inside a command source group (`🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures`): every such row moves out of its group into each target that uses the group, directly or through other groups of its project (a target without inputs uses `default`), so Nx hashes the same inputs. A `^` input of a group that holds such rows is refused: the dependency's rows could not follow it. */
export function liftNxNamedInputReferencesV1(projects){
 const reference=item=>item!==null&&typeof item==="object"&&!Array.isArray(item)&&typeof item.input==="string"&&item.projects!==undefined,holding=new Set();
 for(const project of Object.values(projects))for(const[name,list]of Object.entries(project.namedInputs??{}))if(Array.isArray(list)&&list.some(reference))holding.add(name);
 if(!holding.size)return projects;
 return Object.fromEntries(Object.entries(projects).map(([key,project])=>{
  const named=project.namedInputs??{},reach=(name,seen)=>{if(seen.has(name)||!Array.isArray(named[name]))return[];seen.add(name);return named[name].flatMap(item=>reference(item)?[item]:typeof item==="string"&&!item.startsWith("^")&&Object.hasOwn(named,item)?reach(item,seen):[]);};
  const targets=Object.fromEntries(Object.entries(project.targets??{}).map(([name,target])=>{
   const inputs=target.inputs??["default","^default"];
   if(inputs.some(item=>typeof item==="string"&&item.startsWith("^")&&holding.has(item.slice(1))))throw Error(`A dependency input of a group holding project references cannot be projected for Nx: ${key}:${name}`);
   const seen=new Set(),known=new Set(inputs.map(item=>JSON.stringify(item))),lifted=inputs.flatMap(item=>typeof item==="string"&&!item.startsWith("^")&&Object.hasOwn(named,item)?reach(item,seen):[]).filter(item=>{const text=JSON.stringify(item);if(known.has(text))return false;known.add(text);return true;});
   return[name,lifted.length?{...target,inputs:[...inputs,...lifted]}:target];
  }));
  return[key,{...project,namedInputs:Object.fromEntries(Object.entries(named).map(([name,list])=>[name,Array.isArray(list)?list.filter(item=>!reference(item)):list])),targets}];
 }));'''),
(X, '''export function shareNxProjectInputsV1(projects){
 const owners=new Map(),''', '''export function shareNxProjectInputsV1(projects){
 const external=new Set();
 for(const project of Object.values(projects))for(const target of Object.values(project.targets??{}))for(const item of target.inputs??[])if(item!==null&&typeof item==="object"&&typeof item.input==="string"&&item.projects!==undefined)for(const owner of typeof item.projects==="string"?[item.projects]:item.projects)external.add(owner+"\\0"+item.input);
 const owners=new Map(),'''),
(X, '''   if(input===engineNamedInput||input===engineNativeNamedInput||referenced.has(input)||!Array.isArray(list)||!list.every(independent))return[[input,list]];''', '''   if(input===engineNamedInput||input===engineNativeNamedInput||referenced.has(input)||external.has(name+"\\0"+input)||!Array.isArray(list)||!list.every(independent))return[[input,list]];'''),
(XD, '''export function shareNxProjectInputsV1''', '''export function liftNxNamedInputReferencesV1<T extends Record<string,{namedInputs?:Record<string,unknown[]>;targets?:Record<string,{inputs?:unknown[]}>}>>(projects:T):T;
export function shareNxProjectInputsV1'''),
]
