import {createHash} from "node:crypto";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import policy from "./🔣️policy.json" with {type:"json"};

const protocol=schema.$defs.Protocol.const;

/** 🔌️ Names the closed neutral contract (the schema) every package manager adapter, request, key and receipt follows. */
export const packageManagerProtocolV1=protocol;
/** 🛠️ Closes the operations every adapter renders: produce the lock, install exactly the lock, check the lock is current. */
export type ManagerOperationV1="lock"|"sync"|"verify";
/** 📄️ One lock file a manager owns, with the neutral format id that selects its reader. */
export interface ManagerLockV1{readonly name:string;readonly role:"primary"|"secondary";readonly format:string}
/** 🔤️ Names the closed set of name transforms a layout may declare. */
export type TransformV1="identity"|"lowercase"|"go-escape"|"pep-escape";
/**
 * 🧬️ Declares everything one manager contributes; the schema (`Descriptor`) is the single definition and every language reads the same JSON.
 * @see ./🧬️schema/🔣️.json
 */
export interface ManagerDescriptorV1{
 readonly version:1;readonly id:string;readonly ecosystem:string;readonly toolchain:string;readonly executable:string;
 readonly runtime:Readonly<{hostVersionKey:string|null;probe:Readonly<{arguments:readonly string[];pattern:string}>}>;
 readonly manifests:readonly string[];readonly manifestIdentity:Readonly<{excludedTopLevelFields:readonly string[];bootstrapFields:readonly string[]}>;
 readonly locks:readonly ManagerLockV1[];readonly configurations:Readonly<Record<"scope"|"home"|"config",readonly string[]>>;
 readonly environment:Readonly<{set:Readonly<Record<string,string>>}>;readonly recipe:Readonly<Record<string,unknown>&{backends:Readonly<Record<string,string>>;clearEnvironmentPrefixes:readonly string[];clearEnvironmentKeys:readonly string[]}>;
 readonly backendKinds:Readonly<Record<string,"clone"|"copy"|"alias">>;
 readonly operations:Readonly<{lock:Readonly<{arguments:readonly string[]}>;sync:Readonly<{arguments:readonly string[]}>;verify?:Readonly<{arguments:readonly string[]}>}>;
 readonly installed:Readonly<{containers:readonly string[];entry:string;match:"exact"|"listed";nameTransform:TransformV1;nameEscape:Readonly<Record<string,string>>;fold:TransformV1;suffix:string|null;sourcePrefix:string|null}>;
 readonly resolution:Readonly<{profiles:readonly string[];strictPrefix:string|null}>;readonly generationDirectory:string;readonly graphComponent:string|null;
}
/** 📦️ One locked package in the neutral graph; `key` is the lock's own location so nested duplicates stay distinct. */
export interface LockPackageV1{readonly name:string;readonly version:string;readonly key:string;readonly source:string|null;readonly integrity:string|null}
/** 🏠️ One workspace member the lock lists; `path` is null when the lock does not say. */
export interface LockMemberV1{readonly name:string;readonly path:string|null}
/** 🕸️ The neutral lock graph: sorted by code unit so every language prints the same bytes. */
export interface LockGraphV1{readonly version:1;readonly format:string;readonly packages:readonly LockPackageV1[];readonly members:readonly LockMemberV1[]}
/** 📖️ Reads one lock text into unsorted rows; the core sorts and seals them. */
export type LockReaderV1=(text:string)=>Readonly<{packages:readonly LockPackageV1[];members:readonly LockMemberV1[]}>;
/** 🖥️ The only host facts keys and runtime identity may consume; injected so the core never touches `process`. */
export interface HostPortV1{readonly platform:string;readonly arch:string;readonly versions:Readonly<Record<string,string|undefined>>;probe?(executable:string,args:readonly string[]):string|undefined}
/** 📨️ Everything a rendering may vary by; unknown keys are refused. */
export interface ManagerRequestV1{readonly platform:string;readonly cache?:string;readonly directory?:string;readonly manifest?:string;readonly settings?:Readonly<Record<string,string>>}
/** 🚀️ What a runner executes: no shell, no ambient state. */
export interface ManagerInvocationV1{readonly executable:string;readonly arguments:readonly string[];readonly directory:string|null}
/**
 * 🧰️ The interface the rest of the repository sees; adapters differ only in their descriptor and lock reader.
 * @see {@link createPackageManagerV1}
 */
export interface PackageManagerV1{
 readonly descriptor:ManagerDescriptorV1;readonly id:string;readonly ecosystem:string;
 primaryLockName():string;lockFileNames():readonly string[];manifestFileNames():readonly string[];configurationNames(role:"scope"|"home"|"config"):readonly string[];
 resolutionProfiles():readonly string[];recipe():ManagerDescriptorV1["recipe"];generationDirectory():string;
 backend(platform:string):string|undefined;render(operation:ManagerOperationV1,request:ManagerRequestV1):ManagerInvocationV1;
 environment(root:string,ambient:Readonly<Record<string,string|undefined>>,separator?:"/"|"\\"):Readonly<Record<string,string>>;
 runtimeIdentity(host:HostPortV1):string;parseLock(bytes:Uint8Array|string):LockGraphV1;
}

const compare=(left:string,right:string):number=>left<right?-1:left>right?1:0;
const sortKeys=(value:unknown):unknown=>Array.isArray(value)?value.map(sortKeys):value!==null&&typeof value==="object"?Object.fromEntries(Object.keys(value).sort(compare).map(key=>[key,sortKeys((value as Record<string,unknown>)[key])])):value;
/** 📐️ Serializes with code-unit sorted keys and no whitespace so any language reproduces the same bytes. */
export const canonicalJsonV1=(value:unknown):string=>JSON.stringify(sortKeys(value));
/** #️⃣ Digests bytes or text with the one hash every neutral implementation shares. */
export const sha256HexV1=(data:string|Uint8Array):string=>createHash("sha256").update(data).digest("hex");
/** 🔏️ Seals a body with the digest of its canonical form. */
export const sealV1=<Body extends object>(body:Body):Body&{readonly sha256:string}=>Object.freeze({...body,sha256:sha256HexV1(canonicalJsonV1(body))});
/** 🔓️ Opens a sealed value or refuses it. */
export function openSealedV1<Body extends object>(value:Body&{readonly sha256?:unknown}):Body{
 const {sha256,...body}=value;if(typeof sha256!=="string"||sha256HexV1(canonicalJsonV1(body))!==sha256)throw Error("Sealed package manager value required");return body as unknown as Body;
}

const placeholderV1=/\{([A-Za-z][A-Za-z0-9]*)\}/gu;
function expandV1(template:string,values:Readonly<Record<string,string|undefined>>):string{
 return template.replace(placeholderV1,(_,name:string)=>{const value=values[name];if(value===undefined)throw Error("Complete package manager request required: "+name);return value;});
}
const transformsV1:Readonly<Record<TransformV1,(name:string)=>string>>=Object.freeze({
 identity:name=>name,lowercase:name=>name.toLowerCase(),
 "go-escape":name=>name.replace(/[A-Z]/gu,letter=>"!"+letter.toLowerCase()),
 "pep-escape":name=>name.toLowerCase().replace(/[-_.]+/gu,"_"),
});
const roleV1=(recipe:ManagerDescriptorV1["recipe"],role:string):string=>{const value=recipe[role];if(typeof value!=="string")throw Error("Declared directory role required: "+role);return value;};
const scalarsV1=(recipe:ManagerDescriptorV1["recipe"]):Record<string,string>=>Object.fromEntries(Object.entries(recipe).filter((entry):entry is [string,string]=>typeof entry[1]==="string"));

/**
 * 🏭️ Builds one manager from its descriptor and lock reader; every behavior below is data driven.
 * @see ./🧪️tests/🐍️oracle.py for the independent implementation the vectors come from.
 */
export function createPackageManagerV1(descriptor:ManagerDescriptorV1,reader:LockReaderV1):PackageManagerV1{
 const recipe=descriptor.recipe,primary=descriptor.locks.find(lock=>lock.role==="primary");
 if(!primary)throw Error("Primary lock required: "+descriptor.id);
 const backend=(platform:string)=>recipe.backends[platform];
 const requireBackend=(platform:string)=>{
  const name=backend(platform);
  if(name===undefined)return undefined;
  const kind=descriptor.backendKinds[name];
  if(kind===undefined||protocol.refusedBackendKinds.includes(kind))throw Error("Supported non-aliasing install backend required: "+descriptor.id+" "+name);
  return name;
 };
 const manager:PackageManagerV1={
  descriptor,id:descriptor.id,ecosystem:descriptor.ecosystem,
  primaryLockName:()=>primary.name,lockFileNames:()=>descriptor.locks.map(lock=>lock.name),manifestFileNames:()=>descriptor.manifests,
  configurationNames:role=>descriptor.configurations[role],resolutionProfiles:()=>descriptor.resolution.profiles,recipe:()=>recipe,generationDirectory:()=>descriptor.generationDirectory,
  backend:requireBackend,
  render(operation,request){
   const known=new Set(["platform","cache","directory","manifest","settings"]);
   for(const key of Object.keys(request))if(!known.has(key))throw Error("Closed package manager request required: "+key);
   const spec=descriptor.operations[operation];if(!spec)throw Error("Declared package manager operation required: "+descriptor.id+" "+operation);
   const values={...scalarsV1(recipe),...request.settings,backend:requireBackend(request.platform),cache:request.cache,directory:request.directory,manifest:request.manifest};
   return Object.freeze({executable:descriptor.executable,arguments:Object.freeze(spec.arguments.map(template=>expandV1(template,values))),directory:request.directory??null});
  },
  environment(root,ambient,separator="/"){
   const output:Record<string,string>={},upperPrefixes=recipe.clearEnvironmentPrefixes,keys=recipe.clearEnvironmentKeys;
   for(const [name,value] of Object.entries(ambient)){
    if(value===undefined)continue;
    const upper=name.toUpperCase();
    if(upperPrefixes.some(prefix=>upper.startsWith(prefix.toUpperCase()))||keys.includes(upper))continue;
    output[name]=value;
   }
   const scalars=scalarsV1(recipe);
   for(const [name,spec] of Object.entries(descriptor.environment.set)){
    const [kind,rest]=[spec.slice(0,spec.indexOf(":")),spec.slice(spec.indexOf(":")+1)];
    output[name]=kind==="role"?root+separator+roleV1(recipe,rest):expandV1(rest,scalars);
   }
   return Object.freeze(output);
  },
  runtimeIdentity(host){
   const key=descriptor.runtime.hostVersionKey,embedded=key===null?undefined:host.versions[key];
   if(embedded!==undefined)return embedded;
   const text=host.probe?.(descriptor.executable,descriptor.runtime.probe.arguments),match=text===undefined?null:new RegExp(descriptor.runtime.probe.pattern,"u").exec(text.trim());
   if(!match?.[1])throw Error("Resolvable package manager runtime required: "+descriptor.id);
   return match[1];
  },
  parseLock(bytes){
   if(bytes.length>protocol.maximumLockBytes)throw Error("Bounded lock required: "+descriptor.id);
   const text=typeof bytes==="string"?bytes:new TextDecoder("utf-8",{fatal:true}).decode(bytes),read=reader(text),seen=new Set<string>(),packages:LockPackageV1[]=[];
   for(const row of read.packages){const id=canonicalJsonV1(row);if(!seen.has(id)){seen.add(id);packages.push(Object.freeze({...row}));}}
   packages.sort((left,right)=>compare(left.name,right.name)||compare(left.version,right.version)||compare(left.key,right.key));
   const members=[...read.members].sort((left,right)=>compare(left.name,right.name)||compare(left.path??"",right.path??"")).map(row=>Object.freeze({...row}));
   return Object.freeze({version:1 as const,format:primary.format,packages:Object.freeze(packages),members:Object.freeze(members)});
  },
 };
 return Object.freeze(manager);
}

/**
 * 🗃️ Holds the registered managers; registration is explicit so swapping is a data change, never an import edit in neutral code.
 * @see ./🔣️policy.json
 */
export class PackageManagerRegistryV1{
 readonly #managers=new Map<string,PackageManagerV1>();
 register(manager:PackageManagerV1):this{
  if(this.#managers.has(manager.id))throw Error("Unique package manager id required: "+manager.id);
  for(const other of this.#managers.values())if(other.ecosystem===manager.ecosystem&&(other.generationDirectory()!==manager.generationDirectory()||other.descriptor.toolchain!==manager.descriptor.toolchain))throw Error("Ecosystem managers must share toolchain and generation directory: "+manager.id);
  this.#managers.set(manager.id,manager);return this;
 }
 get(id:string):PackageManagerV1{const manager=this.#managers.get(id);if(!manager)throw Error("Registered package manager required: "+id);return manager;}
 all():readonly PackageManagerV1[]{return [...this.#managers.values()].sort((left,right)=>compare(left.id,right.id));}
 ecosystems():readonly string[]{return [...new Set(this.all().map(manager=>manager.ecosystem))].sort(compare);}
 forEcosystem(ecosystem:string):readonly PackageManagerV1[]{return this.all().filter(manager=>manager.ecosystem===ecosystem);}
 select(ecosystem:string,environment:Readonly<Record<string,string|undefined>>={},selection:Readonly<Record<string,string>>=policy.selection):PackageManagerV1{
  const variable=protocol.selectionEnvironment.replace("{ECOSYSTEM}",ecosystem.toUpperCase().replace(/-/gu,"_")),id=environment[variable]||selection[ecosystem];
  if(!id)throw Error("Selected package manager required: "+ecosystem);
  const manager=this.get(id);if(manager.ecosystem!==ecosystem)throw Error("Package manager of the selected ecosystem required: "+ecosystem+" "+id);
  return manager;
 }
 lockFileNames():readonly string[]{return [...new Set(this.all().flatMap(manager=>manager.lockFileNames()))].sort(compare);}
 manifestFileNames():readonly string[]{return [...new Set(this.all().flatMap(manager=>manager.manifestFileNames()))].sort(compare);}
}

/** 🔑️ Derives the generation key; the shape is fixed by the protocol and the manager contributes its recipe and runtime identity. */
export function generationKeyV1(manager:PackageManagerV1,input:Readonly<{root:string;host:HostPortV1;mode:string;members:readonly unknown[];material:readonly Readonly<{path:string;bytes:Uint8Array}>[]}>):string{
 const hash=createHash("sha256").update(JSON.stringify([manager.recipe(),input.root,manager.runtimeIdentity(input.host),input.host.platform,input.host.arch,input.mode,input.members]));
 for(const row of input.material)hash.update(JSON.stringify([row.path,row.bytes.length])).update(row.bytes);
 return hash.digest("hex");
}
/** 🥾️ Derives the bootstrap receipt key from the manager's declared manifest fields, its runtime identity and the lock bytes. */
export function bootstrapKeyV1(manager:PackageManagerV1,input:Readonly<{manifest:Readonly<Record<string,unknown>>;host:HostPortV1;lock:Uint8Array}>):string{
 const fields=manager.descriptor.manifestIdentity.bootstrapFields.map(name=>input.manifest[name]);
 return createHash("sha256").update(JSON.stringify([...fields,manager.runtimeIdentity(input.host),input.host.platform,input.host.arch])).update(input.lock).digest("hex");
}
/** 📍️ Names where one generation lives: the ecosystem's directory plus the shortened key. */
export const generationLocationV1=(manager:PackageManagerV1,key:string):string=>manager.generationDirectory()+"/"+key.slice(0,protocol.generationNameUnits);
/** 🔎️ Finds the version a lock pins at an install location (or, failing that, the only package of that name). */
export function lockedVersionV1(graph:LockGraphV1,name:string):string|undefined{
 const keyed=graph.packages.find(row=>row.key===name&&row.name===name);if(keyed)return keyed.version;
 const named=graph.packages.filter(row=>row.name===name);return named.length===1?named[0]!.version:undefined;
}

/** 🧾️ The sealed proof that one generation was installed and every locked provider found. */
export interface GenerationReceiptV1{readonly version:1;readonly purpose:"dependency-generation-receipt";readonly ecosystem:string;readonly manager:Readonly<{id:string;runtime:string}>;readonly scope:string;readonly generation:Readonly<{key:string;location:string}>;readonly lock:Readonly<{name:string;sha256:string}>;readonly providers:Readonly<{examined:number}>;readonly sha256:string}
/** 🧾️ Seals the receipt of one finished generation. */
export function sealReceiptV1(manager:PackageManagerV1,input:Readonly<{scope:string;key:string;runtime:string;lock:Readonly<{name:string;sha256:string}>;examined:number}>):GenerationReceiptV1{
 return sealV1({version:1 as const,purpose:protocol.receiptPurpose as "dependency-generation-receipt",ecosystem:manager.ecosystem,manager:{id:manager.id,runtime:input.runtime},scope:input.scope,generation:{key:input.key,location:generationLocationV1(manager,input.key)},lock:input.lock,providers:{examined:input.examined}});
}

/** 🧭️ One provider a lock pins, with the relative path the manager lays it out at. */
export interface ExpectedProviderV1{readonly name:string;readonly version:string;readonly entry:string}
/** 📍️ Computes the layout entry of every locked provider. */
export function expectedProvidersV1(manager:PackageManagerV1,graph:LockGraphV1):readonly ExpectedProviderV1[]{
 const layout=manager.descriptor.installed,transform=transformsV1[layout.nameTransform],escape=(name:string)=>Object.entries(layout.nameEscape).reduce((text,[from,to])=>text.split(from).join(to),transform(name));
 return graph.packages.filter(row=>row.version!==""&&(layout.sourcePrefix===null||row.source?.startsWith(layout.sourcePrefix)===true)).map(row=>({name:row.name,version:row.version,entry:layout.entry.replace(/\{name\}/gu,escape(row.name)).replace(/\{version\}/gu,row.version)}));
}
/** 🗂️ Reads the installed tree; paths are relative to the install root and the port owns every filesystem call. */
export interface ProviderPortV1{kind(path:string):Promise<"file"|"directory"|"link"|"missing"|"other">;list(path:string):Promise<readonly string[]>}
/** ⏱️ Carries cancellation and bounded progress into every long loop. */
export interface ManagerControlV1{readonly signal?:AbortSignal;advance?(completed:number):Promise<void>|void}
/** 🚶️ The verdict of a provider walk: complete, or the first missing providers. */
export type ProviderWalkV1=Readonly<{status:"match";examined:number}>|Readonly<{status:"mismatch";examined:number;missing:readonly ExpectedProviderV1[];truncated:boolean}>;
/** 🚶️ Verifies that every locked provider exists in the installed layout (existence and layout name, not content). */
export async function walkProvidersV1(manager:PackageManagerV1,graph:LockGraphV1,port:ProviderPortV1,control:ManagerControlV1={}):Promise<ProviderWalkV1>{
 const layout=manager.descriptor.installed,providers=expectedProvidersV1(manager,graph);
 if(providers.length>protocol.maximumProviders)throw Error("Bounded provider count required");
 let container:string|undefined;
 for(const pattern of layout.containers){
  let paths=[""];
  for(const part of pattern.split("/")){
   const next:string[]=[];
   for(const base of paths){
    if(part!=="*"){next.push(base===""?part:base+"/"+part);continue;}
    if(await port.kind(base===""?".":base)!=="directory")continue;
    for(const child of [...await port.list(base===""?".":base)].sort(compare)){const path=base===""?child:base+"/"+child;if(await port.kind(path)==="directory")next.push(path);}
   }
   paths=next;
  }
  for(const path of paths)if(await port.kind(path)==="directory"){container=path;break;}
  if(container!==undefined)break;
 }
 const fold=transformsV1[layout.fold],tail=layout.suffix===null?/^$/u:new RegExp(layout.suffix,"u"),missing:ExpectedProviderV1[]=[];
 let names:Set<string>|undefined,examined=0;
 if(container!==undefined&&layout.match==="listed"){
  names=new Set();
  for(const name of await port.list(container)){
   if(tail.test(""))names.add(fold(name));
   for(let at=name.indexOf("+");at>=0;at=name.indexOf("+",at+1))if(tail.test(name.slice(at)))names.add(fold(name.slice(0,at)));
  }
 }
 for(const provider of providers){
  control.signal?.throwIfAborted();
  const found=container===undefined?false:names?names.has(fold(provider.entry)):(await port.kind(container==="."?provider.entry:container+"/"+provider.entry))==="directory";
  if(!found)missing.push(provider);
  examined++;
  if(control.advance&&examined%protocol.progressInterval===0)await control.advance(examined);
 }
 if(!missing.length)return Object.freeze({status:"match" as const,examined});
 return Object.freeze({status:"mismatch" as const,examined,missing:missing.slice(0,protocol.maximumMissingReported),truncated:missing.length>protocol.maximumMissingReported});
}

/** 🏃️ Executes one invocation; the port owns process spawning, environment and cancellation. */
export interface RunnerPortV1{run(invocation:ManagerInvocationV1,environment:Readonly<Record<string,string>>,signal?:AbortSignal):Promise<Readonly<{code:number}>>}
/** 🗄️ Stores sealed receipts by name. */
export interface ReceiptPortV1{read(name:string):Promise<string|null>;write(name:string,text:string):Promise<void>}
/** 🟢️ Reports whether an ensure installed anything. */
export type EnsureOutcomeV1=Readonly<{status:"current"|"synced";key:string;receipt:GenerationReceiptV1}>;

function currentReceiptV1(text:string,key:string,lockSha:string):GenerationReceiptV1|undefined{
 try{const receipt=JSON.parse(text) as GenerationReceiptV1;openSealedV1(receipt);return receipt.generation.key===key&&receipt.lock.sha256===lockSha?receipt:undefined;}catch{return undefined;}
}
/** 🎯️ Brings one scope to the locked state through any manager: a current receipt ends it, otherwise sync, walk the providers, seal. */
export async function ensureGenerationV1(manager:PackageManagerV1,input:Readonly<{scope:string;root:string;host:HostPortV1;mode:string;members:readonly unknown[];material:readonly Readonly<{path:string;bytes:Uint8Array}>[];lock:Uint8Array;request:ManagerRequestV1;ambient:Readonly<Record<string,string|undefined>>}>,ports:Readonly<{runner:RunnerPortV1;receipts:ReceiptPortV1;providers:ProviderPortV1}>,control:ManagerControlV1={}):Promise<EnsureOutcomeV1>{
 const key=generationKeyV1(manager,input),name=generationLocationV1(manager,key).replace("/","-")+".json",existing=await ports.receipts.read(name);
 const lockSha=sha256HexV1(input.lock),receipt=existing===null?undefined:currentReceiptV1(existing,key,lockSha);
 if(receipt)return Object.freeze({status:"current" as const,key,receipt});
 control.signal?.throwIfAborted();
 const graph=manager.parseLock(input.lock),invocation=manager.render("sync",input.request),result=await ports.runner.run(invocation,manager.environment(input.root,input.ambient),control.signal);
 if(result.code!==0)throw Error("Successful package manager sync required: "+manager.id+" exit "+result.code);
 control.signal?.throwIfAborted();
 const walk=await walkProvidersV1(manager,graph,ports.providers,control);
 if(walk.status!=="match")throw Error("Complete locked providers required: "+walk.missing.map(row=>row.name+"@"+row.version).slice(0,8).join(", "));
 const sealed=sealReceiptV1(manager,{scope:input.scope,key,runtime:manager.runtimeIdentity(input.host),lock:{name:manager.primaryLockName(),sha256:lockSha},examined:walk.examined});
 await ports.receipts.write(name,JSON.stringify(sealed));
 return Object.freeze({status:"synced" as const,key,receipt:sealed});
}
