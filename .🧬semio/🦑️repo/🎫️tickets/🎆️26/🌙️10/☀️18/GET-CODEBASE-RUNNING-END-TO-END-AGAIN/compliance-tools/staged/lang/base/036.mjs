/** 🔎️ Discovers canonical owners, their native manifests and the selected executable Source closures. */
import { checkResourceLeaseControl } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { pinnedAuthorityReadV1 } from "../../../📥️inputs/🔏️revision/🟨️.mjs";
import { checkScriptInvocation, requireIssuedResourceControlInvocationV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { createHash } from "node:crypto";
import { parserTypescriptV1 } from "../../../🚀️bootstrap/📦️parsers/🟨️.mjs";
import { dirname, isAbsolute, join, relative, resolve, posix } from "node:path";
import { loadCatalogBuildProvidersV1, catalogBuildContributionForManifestV1 } from "../../../../../../../💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🟦️.ts";
import { observedSourceAnalysisV1, observedRequireResolve, observedTypeScriptHost, observedFs, recordRegistryOwnerV1 } from "../../../../🏗️build/👁️observation/🟨️.mjs";
import { pathToFileURL } from "node:url";
import * as loadedProjectInputs from "../../../../🏗️build/📥️inputs/🟨️.mjs";
import ownerAdmissionSchema from "../../../🧬️schema/🚪️admission.json" with { type: "json" };
import { buildRelativeImportProjectV1, buildInputSourceRoleV1, buildSourceDemandV1, buildNativeSourceObservationV1 } from "../../🚪️admission/🟦️.ts";
const { existsSync, lstatSync, readFileSync, readdirSync } = observedFs;

/** 🏷️ Names this component as its roster entry does. */
export const component = "owner-discovery";
let buildPath, POLICY, WORKSPACE_POLICY, exampleCollectionPath, yieldInferenceV1, readPhysicalSource, importRevision, PROJECT_BASENAME, nativeDependencyRoots, libraryBootstrap, readToml, createScriptInputCache, cargoTargets, componentBuildTargets, readTaxonomyProjectionV1, owned, playgroundSessionTargets, playgroundPreparationTargets, collectPlaygroundCatalog, playgroundSiteTargetsForAdmissionV1, targetWithDefaults, SCRIPT_BASENAME, loaded, targetScriptSources, projectWithDefaults, withNativePreparation, LIBRARY_ROOT, EXAMPLE_COLLECTIONS, WORKSPACE_OWNERSHIP, relativeScriptInputs, prepareSourceAnalysesV1, buildImportResourceV1;

/** 🔌️ Binds this component's dependencies from the revision registry once its needs are wired. */
export function wire(registry) {
  ({ buildPath, POLICY, WORKSPACE_POLICY, exampleCollectionPath, readPhysicalSource, PROJECT_BASENAME, owned, SCRIPT_BASENAME, LIBRARY_ROOT, EXAMPLE_COLLECTIONS, WORKSPACE_OWNERSHIP } = registry.coordinates);
  ({ yieldInferenceV1, createScriptInputCache, targetScriptSources, relativeScriptInputs, buildImportResourceV1 } = registry.scriptInputs);
  ({ importRevision, libraryBootstrap, loaded } = registry.bootstrap);
  ({ nativeDependencyRoots, readToml, withNativePreparation } = registry.nativeManifests);
  ({ cargoTargets, componentBuildTargets } = registry.componentTargets);
  ({ readTaxonomyProjectionV1, prepareSourceAnalysesV1 } = registry.sourceAnalyses);
  ({ playgroundSessionTargets, playgroundPreparationTargets, collectPlaygroundCatalog, playgroundSiteTargetsForAdmissionV1 } = registry.playgroundTargets);
  ({ targetWithDefaults } = registry.targetPolicy);
  ({ projectWithDefaults } = registry.projectAssembly);
}

/** 🦀️ Lists every workspace Cargo manifest through taxonomy-respecting discovery. */
export function walkCargoToml(workspaceRoot) {
  const found = [];
  const walk = (directory) => {
    for (const entry of readdirSync(join(workspaceRoot, directory), { withFileTypes: true })) {
      const path = buildPath(join(directory, entry.name));
      if (entry.isSymbolicLink() || path === "compose" || path === "temp/compose" || entry.name === ".🧬semio" || POLICY.generatedDirectories.includes(entry.name)) continue;
      if (entry.isDirectory()) walk(path);
      else if (entry.isFile() && entry.name === "Cargo.toml") found.push(path);
    }
  };
  walk("");
  return found.sort();
}

/** 🦀️ Discovers the same complete canonical manifest scope through the caller's bounded continuation. */
export async function walkCargoTomlControlledV1(workspaceRoot, control) {
  const found = [], directories = [""];let visited=0;
  while(directories.length){
    control.check();if(visited%128===0)await control.advance("manifest-discovery");
    const directory=directories.pop();
    for(const entry of readdirSync(join(workspaceRoot,directory),{withFileTypes:true})){
      const path=buildPath(join(directory,entry.name));
      if(entry.isSymbolicLink()||path==="compose"||path==="temp/compose"||entry.name===".🧬semio"||POLICY.generatedDirectories.includes(entry.name))continue;
      if(entry.isDirectory())directories.push(path);else if(entry.isFile()&&entry.name==="Cargo.toml")found.push(path);
    }
    visited++;
  }
  await control.advance(`manifest-discovery:${visited}`);
  return found.sort();
}

/**
 * @param {string[]} configFiles
 * @param {unknown} options
 * @param {{ workspaceRoot: string }} context
 */
/** 🛡️ Admits callback candidates through fresh raw physical workspace ancestry. */
export function inferenceCandidates(configFiles, workspaceRoot) {
  if (typeof workspaceRoot !== "string" || !workspaceRoot || workspaceRoot.includes("\0") || workspaceRoot.split(/[\\/]/u).some(part => part === "." || part === "..")) throw Error("Build inference requires raw real workspace ancestry");
  const root = resolve(workspaceRoot), ancestors = [];
  for (let path = root; ; path = dirname(path)) { ancestors.push(path); if (path === dirname(path)) break; }
  for (const path of ancestors.reverse()) { const stat = lstatSync(path); if (!stat.isDirectory() || stat.isSymbolicLink()) throw Error(`Build inference requires real workspace ancestry: ${path}`); }
  const directories = new Map([[root, true]]), files = new Set();
  for (const supplied of configFiles) {
    if (typeof supplied !== "string" || !supplied || supplied.includes("\0") || supplied.includes("\uFFFD")) continue;
    const path = buildPath(supplied), parts = path.split("/");
    if (exampleCollectionPath(path) || isAbsolute(path) || /^[A-Za-z]:/u.test(path) || parts.some(part => !part || part === "." || part === ".." || part === ".🧬semio" || POLICY.generatedDirectories.includes(part)) || path.startsWith("compose/") || path.startsWith("temp/compose/")) continue;
    let directory = root, admitted = true;
    for (const part of parts.slice(0, -1)) {
      directory = join(directory, part);
      if (!directories.has(directory)) {
        try { const value = lstatSync(directory); directories.set(directory, value.isDirectory() && !value.isSymbolicLink()); }
        catch (error) { if (!["ENOENT", "ENOTDIR"].includes(error.code)) throw error; directories.set(directory, false); }
      }
      if (!directories.get(directory)) { admitted = false; break; }
    }
    if (!admitted) continue;
    try { const value = lstatSync(join(root, path)); if (value.isFile() && !value.isSymbolicLink()) files.add(path); }
    catch (error) { if (!["ENOENT", "ENOTDIR"].includes(error.code)) throw error; }
  }
  return [...files].sort((left, right) => Buffer.compare(Buffer.from(left), Buffer.from(right)));
}

/** 🦉️ Projects native catalog manifest candidates from canonical owner identities. */
export async function catalogNativeManifestCandidatesV1(workspaceRoot,registry,accept,context){
 const manifest=root=>root==="."?"Cargo.toml":root+"/Cargo.toml",declared=new Set(registry.owners.flatMap(owner=>[...(owner.manifest.endsWith("Cargo.toml")?[owner.manifest]:[]),...(owner.nativeRoots??[]).map(manifest)])),candidates=[...new Set([...declared,...registry.owners.map(owner=>manifest(owner.root))])].filter(accept).sort(),result=[];let index=0;
 for(const path of candidates){await yieldInferenceV1(context,"build:catalog-manifests",++index);try{const entry=lstatSync(join(workspaceRoot,path));if(!entry.isFile()||entry.isSymbolicLink())throw Error("Catalog native manifest must be a real file: "+path);}catch(error){if(!declared.has(path)&&["ENOENT","ENOTDIR"].includes(error.code))continue;throw error;}result.push(path);}
 return result;
}

/** 🔌️ Loads only the caller-declared optional providers and verifies current producer authority. */
export async function catalogBuildContributionsV1(workspaceRoot,providers) {
  const catalogContributions=await loadCatalogBuildProvidersV1(providers,{
    readText(path){return readFileSync(join(workspaceRoot,path),"utf8");},
    kind(path){try{const stat=lstatSync(join(workspaceRoot,path));return stat.isSymbolicLink()?"symlink":stat.isDirectory()?"directory":stat.isFile()?"file":null;}catch(error){if(error.code==="ENOENT")return null;throw error;}},
    async load(path){const file=join(workspaceRoot,path),source=readPhysicalSource(file),revision=createHash("sha256").update(source).digest("hex"),module=await importRevision(pathToFileURL(file),revision);if(createHash("sha256").update(readPhysicalSource(file)).digest("hex")!==revision)throw Object.assign(Error("Catalog provider source changed during admission"),{code:"BUILD_SOURCE_AUTHORITY"});return module;},
  });
  for(const owner of catalogContributions){for(const input of owner.inputs)readFileSync(join(workspaceRoot,input));const project=JSON.parse(readFileSync(join(workspaceRoot,owner.projectRoot,PROJECT_BASENAME),"utf8"));if(project.name!==owner.project||!project.targets?.[owner.generateTarget])throw Error("Catalog build producer is absent from its owner's project");}
  return catalogContributions;
}

/** 🧭️ Reads the explicit catalog options declared on this workspace plugin. */
export async function workspaceCatalogBuildContributionsV1(workspaceRoot) {
  const config=JSON.parse(pinnedAuthorityReadV1(join(workspaceRoot,"📋️build-owners.json"),()=>readFileSync(join(workspaceRoot,"📋️build-owners.json"),"utf8")));
  return catalogBuildContributionsV1(workspaceRoot,config.catalogProviders.map(({ownerRoot,sourcePath})=>({ownerRoot,sourcePath})));
}

/** 🧩️ Invalidates project assembly only when its actual native owners or component pipeline change. */
export function projectAssemblyAuthorityV1(workspaceRoot,nativeRoot,projectsByRoot,componentRoots,nativeCache=new Map(),nativeClosures=new Map()){
 let nativeOwners;
 if(nativeRoot!==undefined){try{nativeOwners=nativeDependencyRoots(nativeRoot,workspaceRoot,true,nativeCache,nativeClosures).map(owner=>[owner,projectsByRoot.get(owner)??null]);}catch(error){if(error.code!=="NATIVE_INPUT_ADMISSION")throw error;nativeOwners=[error.manifest,error.message];}}
 return createHash("sha256").update(JSON.stringify([nativeOwners??null,componentRoots??null])).digest("hex");
}

/** 🪪️ Expands actual native owners through the same read-only phase and original caller controls. */
export async function discoverNativeBuildOwnersV1(configFiles,workspaceRoot,context){
  configFiles = inferenceCandidates([...configFiles,...configFiles.filter(file=>file.endsWith("Cargo.toml")).map(file=>buildPath(join(dirname(file),PROJECT_BASENAME)))], workspaceRoot);
  const nativeCache=new Map(),nativeClosures=new Map(),nativeCandidates=new Set(configFiles);let candidateIndex=0;
  for(const file of configFiles.filter(file=>file.endsWith("Cargo.toml"))){
    await yieldInferenceV1(context,"build:cargo-owners",++candidateIndex);
    try{for(const root of nativeDependencyRoots(buildPath(dirname(file)),workspaceRoot,true,nativeCache,nativeClosures)){nativeCandidates.add(`${root}/Cargo.toml`);nativeCandidates.add(`${root}/${PROJECT_BASENAME}`);}}
    catch(error){if(error.code!=="NATIVE_INPUT_ADMISSION")throw error;}
  }
  return {files:inferenceCandidates([...nativeCandidates],workspaceRoot),nativeCache,nativeClosures};
}

/** 🚧️ The characters a generated import path may not hold, compiled once. */
const UNSAFE_GENERATED_PATH_V1 = /[\\%*?#[\]{}]/u;

/** 🏭️ Resolves only one exact ignored output contract to its physical authored task owner. */
export function readBuildGeneratedCompileOutputV1(path,workspaceRoot,contracts,registry,context){
 const local=buildPath(relative(workspaceRoot,path)),safe=value=>typeof value==="string"&&value!=="."&&value!==""&&!isAbsolute(value)&&posix.normalize(value)===value&&!value.startsWith("../")&&!UNSAFE_GENERATED_PATH_V1.test(value);const refusal=()=>Error("Canonical generated import path required: "+JSON.stringify({path:local.slice(0,256),...(typeof context?.importer==="string"?{importer:context.importer.slice(0,256)}:{}),...(typeof context?.specifier==="string"?{specifier:context.specifier.slice(0,256)}:{})}));if(local==="."||local===""||isAbsolute(local)||posix.normalize(local)!==local||local.startsWith("../")||local.includes("\0"))throw refusal();const matches=[],declared=new Set();
 const resolution=ownerAdmissionSchema.$defs.generatedImportResolution.const,extensionless=posix.extname(local)==="";
 for(const contract of Object.values(contracts)){if(contract.ownership!=="owned")continue;for(const output of contract.outputRoots??[]){if(output.inclusion!=="ignored")continue;if(!safe(output.path))throw Error("Canonical generated output root required");declared.add(output.path);const exact=local===output.path||local.startsWith(output.path+"/"),inferred=extensionless&&resolution.files.some(suffix=>local+suffix===output.path);if(exact||inferred)matches.push({output,producer:output.producer??contract,inferred:!exact});}}
 if(!matches.length)return undefined;if(!safe(local))throw refusal();if(matches.length!==1)throw Error("Ambiguous generated compile output producer");const {output,producer,inferred}=matches[0];if(inferred&&resolution.probes.some(suffix=>{const candidate=local+suffix;if(candidate===output.path)return false;const absolute=join(workspaceRoot,candidate);return declared.has(candidate)||existsSync(absolute)&&!lstatSync(absolute).isDirectory();}))throw Error("Ambiguous generated compile output resolution");const separator=producer.target?.lastIndexOf(":"),project=producer.target?.slice(0,separator),target=producer.target?.slice(separator+1);if(!project||!target||!safe(producer.ownerPath)&&producer.ownerPath!==".")throw Error("Exact generated task owner required");const manifest=buildPath(join(producer.ownerPath,PROJECT_BASENAME)),owner=registry?.owners.find(owner=>owner.name===project);if(registry)recordRegistryOwnerV1(workspaceRoot,project,owner);if(registry&&(!owner||owner.root!==producer.ownerPath||owner.manifest!==manifest))throw Error("Canonical generated task owner absent");const declaration=JSON.parse(readPhysicalSource(join(workspaceRoot,manifest)).toString("utf8"));if(declaration.name!==project||!declaration.targets?.[target])throw Error("Authored generated producer task absent");return Object.freeze({project,target:producer.target,manifest,output:output.path});
}

/** 🧵️ Shares physical traversal while keeping each target's matching and exclusions separate. */
export async function readBuildSourceInputMatchesV1(scans,workspaceRoot,context){
 const matched=new Set(),compileMatched=new Set(),compileByTarget=new Map(),walked=new Map(),matchers=new Map(),matches=(path,pattern)=>{let matcher=matchers.get(pattern);if(!matcher){matcher=loadedProjectInputs.createBuildInputPathMatcherV1(pattern);matchers.set(pattern,matcher);}return matcher(path);};let visited=0;
  const walk=async root=>{if(walked.has(root))return walked.get(root);const pending=[root],found=[];while(pending.length){await yieldInferenceV1(context,"build:selected-source-inputs",++visited);const directory=pending.pop();const absolute=join(workspaceRoot,directory);if(!existsSync(absolute)||!lstatSync(absolute).isDirectory())continue;for(const entry of readdirSync(absolute,{withFileTypes:true})){await yieldInferenceV1(context,"build:selected-source-inputs",++visited);const path=buildPath(join(directory,entry.name));if(entry.isSymbolicLink())continue;if(entry.isDirectory()){if(![...POLICY.generatedDirectories,".git",".🧬semio",".nx",".cache","node_modules","dist","target","🗑️generated"].includes(entry.name)&&!(EXAMPLE_COLLECTIONS.includes(entry.name)&&directory.split("/").at(-1)!==WORKSPACE_OWNERSHIP.collections.moduleMember))pending.push(path);}else if(entry.isFile())found.push(path);}}walked.set(root,found);return found;};
 for(const scan of scans){const patterns=scan.patterns,exclusions=patterns.filter(value=>value.startsWith("!")).map(value=>value.slice(1));for(const pattern of patterns.filter(value=>!value.startsWith("!"))){await yieldInferenceV1(context,"build:selected-source-inputs",++visited);const index=pattern.search(/[?*[{(!]/u),prefix=index<0?pattern:pattern.slice(0,index),root=index<0?dirname(prefix):prefix.endsWith("/")?prefix.slice(0,-1):dirname(prefix),candidates=index<0?(existsSync(join(workspaceRoot,pattern))?[pattern]:[]):await walk(root||".");for(const path of candidates){await yieldInferenceV1(context,"build:selected-source-inputs",++visited);if(matches(path,pattern)&&!exclusions.some(pattern=>matches(path,pattern))){matched.add(path);if(scan.compile){compileMatched.add(path);const rows=compileByTarget.get(scan.target)??new Set();rows.add(path);compileByTarget.set(scan.target,rows);}}}}}
 return {matched,compileMatched,compileByTarget};
}

/** 🚪️ Observes one canonical owner and only its selected executable source closures. */
export async function discoverBuildOwnerAdmissionFactsV1(owner,targets,registry,context,inputNames=[]){
 await libraryBootstrap;const {workspaceRoot}=context,nativeCache=context.nativeCache??new Map(),nativeClosures=context.nativeClosures??new Map();
 if(context.invocation){requireIssuedResourceControlInvocationV1(context.phaseControl,context.invocation);checkResourceLeaseControl(context.phaseControl);checkScriptInvocation(context.invocation);}
 const stage=async phase=>{await yieldInferenceV1(context,"build:selected-"+phase,0,true);};
 await stage("declaration");
 const path=join(workspaceRoot,owner.manifest),source=readPhysicalSource(path);
 const declaration=owner.manifest.endsWith("Cargo.toml")?readToml(path):JSON.parse(source.toString("utf8"));
 const authoredTargets=owner.manifest.endsWith("Cargo.toml")?{}:declaration.targets??{},scripts=createScriptInputCache(context),nativeDeclarations=await cargoTargets(owner.root,workspaceRoot,undefined,scripts),json=owner.manifest.endsWith("Cargo.toml")?{name:declaration.package?.name,root:owner.root,targets:nativeDeclarations}:{...declaration,root:owner.root};
 if(json.name!==owner.name)throw Error("Admitted canonical build owner differs: "+owner.manifest);
 json.targets={...nativeDeclarations,...json.targets,...componentBuildTargets(owner.root,workspaceRoot)};
 const contractsPath=join(workspaceRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"),contracts=existsSync(contractsPath)?readTaxonomyProjectionV1(contractsPath).generatorContracts:{},providers=registry.catalogProviders.filter(provider=>targets.length&&provider.consumerRoots.some(root=>owned(owner.root,root))||inputNames.includes("{workspaceRoot}/"+provider.sourcePath)),catalogContributions=providers.length?await catalogBuildContributionsV1(workspaceRoot,providers.map(({ownerRoot,sourcePath})=>({ownerRoot,sourcePath}))):[];
 scripts.generatedOutput=(path,context)=>readBuildGeneratedCompileOutputV1(path,workspaceRoot,contracts,registry,context);
 const catalogFiles=catalogContributions.length?await catalogNativeManifestCandidatesV1(workspaceRoot,registry,path=>catalogBuildContributionForManifestV1(catalogContributions,path),context):[];
 await stage("catalog-targets");
 if(catalogContributions.length&&targets.length){
  if(json.name==="@semio-tech/plugin-registry")json.targets={...json.targets,...playgroundSessionTargets(catalogFiles,workspaceRoot,catalogContributions)};
  if(json.name==="@semio-tech/framework-os-dev")json.targets={...json.targets,...await playgroundPreparationTargets(catalogFiles,workspaceRoot,owner.root,contracts,catalogContributions,context)};
  const catalog=collectPlaygroundCatalog(catalogFiles,workspaceRoot,catalogContributions);json.targets={...json.targets,...playgroundSiteTargetsForAdmissionV1(owner.root,catalog,workspaceRoot,registry,targets),...authoredTargets};
 }
 const selected=Object.fromEntries(targets.map(name=>{if(!json.targets?.[name])throw Error("Unknown admitted build target: "+owner.name+":"+name);const target=targetWithDefaults(json.targets[name],owner.root,existsSync(join(workspaceRoot,owner.root,SCRIPT_BASENAME))),modes=loaded.targetBuildModes(name,target),configuration=(context.selection?.kind==="tasks"?context.selection.tasks.find(task=>task.project===owner.name&&task.target===name)?.configuration:undefined)??modes.defaultConfiguration,overrides=modes.configurations?.[configuration];if(!overrides)throw Error("Unknown selected build configuration: "+configuration);if(Object.keys(overrides).some(key=>["dependsOn","inputs","outputs","configurations","executor","metadata"].includes(key)))throw Error("Build configuration cannot change task identity");buildInputSourceRoleV1(true,false,[target.metadata?.semio?.sourceInputObservation]);return [name,target];})),files=new Set([owner.manifest]),dependencies=new Set(),dependencyInputs=new Map(),caretInputs=new Set(),inputRole=buildInputSourceRoleV1(targets.length>0,false,Object.values(selected).map(target=>target.metadata?.semio?.sourceInputObservation));
 const byPackage=new Map(registry.owners.flatMap(owner=>owner.packages.map(name=>[name,owner.name]))),byRoot=new Map(registry.owners.flatMap(owner=>[owner.root,...owner.nativeRoots??[]].map(root=>[root,owner.name]))),owners=registry.owners.flatMap(owner=>[owner.root,...owner.nativeRoots??[]].map(root=>({name:owner.name,root}))).sort((a,b)=>b.root.length-a.root.length);
 const addFile=path=>files.add(path),addDependency=path=>{const name=buildRelativeImportProjectV1(owners,path);if(name&&name!==json.name)dependencies.add(name);},addInput=(project,input)=>{if(!registry.owners.some(owner=>owner.name===project)||typeof input!=="string"||!input)throw Error("Canonical named input owner required");const selected=dependencyInputs.get(project)??new Set();selected.add(input);dependencyInputs.set(project,selected);};
 for(const provider of providers)addFile(provider.sourcePath);for(const file of catalogFiles){readPhysicalSource(join(workspaceRoot,file));addFile(file);}for(const contribution of catalogContributions){for(const input of contribution.inputs)addFile(input);addFile(contribution.projectRoot+"/"+PROJECT_BASENAME);if(contribution.project!==json.name)dependencies.add(contribution.project);}
 await stage("command-sources");
 const commandModules=new Map(),commandResources=new Map();
 for(const [name,target] of Object.entries(selected)){
  const sources=await targetScriptSources(target,workspaceRoot,scripts,owner.root,owner.manifest.endsWith("Cargo.toml")?owner.manifest:json.metadata?.nativeRoot?buildPath(join(json.metadata.nativeRoot,"Cargo.toml")):undefined);
  const compiled=scripts.compiledNames.get(sources.closure)??new Set(),absolute=path=>join(workspaceRoot,path.slice("{workspaceRoot}/".length));commandModules.set(name,new Set([...compiled].map(absolute)));commandResources.set(name,new Set((sources.closure??[]).filter(path=>typeof path==="string"&&!compiled.has(path)).map(absolute)));
  for(const input of sources.closure??[]){if(typeof input==="string"){if(input.startsWith("{workspaceRoot}/"))addFile(input.slice("{workspaceRoot}/".length));}else if(input!==null&&typeof input==="object"&&!Array.isArray(input)&&Object.keys(input).length===2&&typeof input.input==="string"&&Array.isArray(input.projects)&&input.projects.length>0)for(const project of input.projects)addInput(project,input.input);else throw Error("Canonical command closure reference required: "+JSON.stringify(input).slice(0,256));}

 }
 const packageFile=join(workspaceRoot,owner.root,"package.json");if(existsSync(packageFile)){const manifest=JSON.parse(readFileSync(packageFile,"utf8"));addFile(buildPath(relative(workspaceRoot,packageFile)));for(const name of inputRole==="compile"?Object.keys({...manifest.dependencies,...manifest.peerDependencies,...manifest.optionalDependencies}):[]){const owner=byPackage.get(name);if(owner&&owner!==json.name)dependencies.add(owner);}}
 const nativeRoot=json.metadata?.nativeRoot??owner.root,nativeManifest=join(workspaceRoot,nativeRoot,"Cargo.toml"),nativeTargets=Object.entries(selected).filter(([,target])=>target.metadata?.semio?.execution?.kind!=="source"),nativeObservation=()=>buildNativeSourceObservationV1({selectedSource:Object.values(selected).some(target=>target.metadata?.semio?.execution?.kind==="source"),selectedNative:nativeTargets.length>0,requestedInputs:inputNames.length>0,dependencyInputs:caretInputs.size>0,inputRole});let nativeObserved=false;
 const observeNative=()=>{if(nativeObserved)return;nativeObserved=true;if(existsSync(nativeManifest))for(const root of nativeDependencyRoots(nativeRoot,workspaceRoot,true,nativeCache,nativeClosures)){addFile(root+"/Cargo.toml");const dependency=byRoot.get(root);if(!dependency)throw Error("Canonical native dependency owner required: "+root);if(dependency!==json.name)dependencies.add(dependency);}};
 await stage("native-closure");if(nativeObservation()!=="none")observeNative();
 const declaredCompileEntries=new Map(),compileModules=new Map();
 if(targets.length||inputNames.length){
  await stage("normalization");const normalized=await projectWithDefaults({...json,targets:{...json.targets,...selected}},owner.root,join(workspaceRoot,owner.root),workspaceRoot,contracts,new Map(),undefined,scripts,undefined,"source",targets,inputNames);await stage("native-preparation");if(nativeTargets.length){const preparing={...normalized,targets:Object.fromEntries(nativeTargets.map(([name])=>[name,normalized.targets[name]]))};withNativePreparation(preparing,workspaceRoot,contracts,nativeCache,byRoot,nativeClosures);normalized.targets={...normalized.targets,...preparing.targets};if(preparing.metadata!==undefined)json.metadata=normalized.metadata=preparing.metadata;for(const[name,target]of Object.entries(preparing.targets))if(!Object.hasOwn(json.targets,name))json.targets[name]=target;}
  const named={...WORKSPACE_POLICY.namedInputs,...normalized.namedInputs},scans=[],active=new Set();
  const expand=(values,patterns,target)=>{for(const value of values){if(typeof value!=="string"){
   if(value===null||typeof value!=="object"||Array.isArray(value))throw Error("Closed selected source input required");
   const keys=Object.keys(value);
   if(typeof value.input==="string"&&keys.every(key=>["input","projects","dependencies"].includes(key))){if(value.projects!==undefined){if(!Array.isArray(value.projects)||value.dependencies!==undefined)throw Error("Canonical named input project list required");for(const project of value.projects)addInput(project,value.input);}else if(value.dependencies===true)caretInputs.add(value.input);else if(value.dependencies===undefined||value.dependencies===false)expand([value.input],patterns,target);else throw Error("Closed named input dependency required");}
   else if(typeof value.fileset==="string"&&keys.length===1)expand([value.fileset],patterns,target);
   else if(keys.length===1&&typeof(value.env??value.runtime)==="string"||keys.length===1&&Array.isArray(value.externalDependencies)&&value.externalDependencies.every(name=>typeof name==="string"))continue;
   else if(typeof value.dependentTasksOutputFiles==="string"&&keys.every(key=>["dependentTasksOutputFiles","transitive"].includes(key))&&(value.transitive===undefined||typeof value.transitive==="boolean"))continue;
   else throw Error("Unsupported selected source input object");
   continue;
  }if(value.startsWith("^")){caretInputs.add(value.slice(1));continue;}if(Object.hasOwn(named,value)){const contract=json.metadata?.sourceInputs?.[value];if(contract){const entries=declaredCompileEntries.get(target)??new Set();for(const entry of loaded.readSourceInputContract(join(workspaceRoot,contract)).entries)entries.add(join(workspaceRoot,entry));declaredCompileEntries.set(target,entries);}if(active.has(value))throw Error("Selected named input cycle: "+value);active.add(value);expand(named[value],patterns,target);active.delete(value);}else if(value.includes("{workspaceRoot}")||value.includes("{projectRoot}"))patterns.push(value.replaceAll("{workspaceRoot}/","").replaceAll("{projectRoot}",owner.root));else throw Error("Unknown selected named source input: "+value);}};
  for(const name of targets){json.targets[name]=normalized.targets[name];const patterns=[];expand(normalized.targets[name].inputs??["default"],patterns,name);scans.push({target:name,patterns,compile:buildInputSourceRoleV1(true,false,[json.targets[name].metadata?.semio?.sourceInputObservation])==="compile"});}
  const requested=[];expand(inputNames,requested);scans.push({patterns:requested,compile:false});
  if(nativeObservation()!=="none")observeNative();
  const {matched,compileByTarget}=await readBuildSourceInputMatchesV1(scans,workspaceRoot,context);
  for(const path of matched)addFile(path);
  await stage("source-closure");
  for(const name of new Set([...compileByTarget.keys(),...declaredCompileEntries.keys()])){const declared=declaredCompileEntries.get(name)??new Set(),resources=commandResources.get(name)??new Set(),compiled=commandModules.get(name)??new Set(),modules=new Set([...compileByTarget.get(name)??[]].filter(path=>/\.[cm]?[jt]sx?$/u.test(path)).map(path=>join(workspaceRoot,path)).filter(path=>declared.has(path)||!resources.has(path)||compiled.has(path)));for(const path of declared)if(/\.[cm]?[jt]sx?$/u.test(path))modules.add(path);compileModules.set(name,modules);}
  const modules=[...new Set([...compileModules.values()].flatMap(paths=>[...paths]))];
  for(const input of modules.length?await relativeScriptInputs(modules,workspaceRoot,scripts):[])if(typeof input==="string"&&input.startsWith("{workspaceRoot}/"))addFile(input.slice("{workspaceRoot}/".length));
 }
 const generatedDependencies={},roles=ownerAdmissionSchema.$defs.sourceDemand.const,generated=(producer,path,phase,name)=>{addFile(producer.manifest);if(producer.project!==json.name)dependencies.add(producer.project);if(name!==undefined){if(producer.target===json.name+":"+name)throw Error("Generated task cannot consume its own output: "+JSON.stringify({target:producer.target.slice(0,256),importer:buildPath(relative(workspaceRoot,path)).slice(0,256),output:producer.output.slice(0,256),phase}));addInput(producer.project,"default");const rows=generatedDependencies[name]??=[];if(!rows.includes(producer.target))rows.push(producer.target);}};
 const compiler=parserTypescriptV1(),host=observedTypeScriptHost(compiler.sys),pending=[],demands=new Map(),visited=new Map(),enqueue=(target,file,role)=>{const rows=demands.get(target)??new Map(),next=buildSourceDemandV1(rows.get(file),role);if(next!==rows.get(file)){rows.set(file,next);demands.set(target,rows);pending.push({target,file});}};let index=0;
 for(const [target,files] of compileModules)for(const file of files)enqueue(target,file,roles.compileInput);for(const [target,files] of declaredCompileEntries)for(const file of files)enqueue(target,file,roles.declaredEntry);for(const [target,files] of commandModules)for(const file of files)enqueue(target,file,roles.command);
 await stage("compile-closure");
 while(pending.length){await yieldInferenceV1(context,"build:selected-compile-imports",++index);const {target,file:absolute}=pending.pop(),phase=demands.get(target).get(absolute),seen=visited.get(target)??new Map();if(seen.get(absolute)===phase)continue;seen.set(absolute,phase);visited.set(target,seen);const path=buildPath(relative(workspaceRoot,absolute));readPhysicalSource(absolute);const facts=observedSourceAnalysisV1(prepareSourceAnalysesV1().module,absolute),imports=phase===roles.compileInput?facts.imports:facts.runtimeImports;
  for(const specifier of imports){
   await yieldInferenceV1(context,"build:selected-compile-imports",++index);
   const relativeImport=specifier.startsWith(".");if(!relativeImport){const name=specifier.startsWith("@")?specifier.split("/").slice(0,2).join("/"):specifier.split("/")[0],dependency=byPackage.get(name);if(!dependency)continue;if(dependency!==json.name)dependencies.add(dependency);}
   const resource=buildImportResourceV1(absolute,specifier),producer=relativeImport?scripts.generatedOutput(resource.specifier,{importer:path,specifier}):undefined;if(producer){generated(producer,absolute,phase,target);continue;}let resolved;try{resolved=observedRequireResolve(absolute,resource.specifier);}catch(error){if(error.code!=="MODULE_NOT_FOUND")throw error;resolved=compiler.resolveModuleName(resource.specifier,absolute,{moduleResolution:compiler.ModuleResolutionKind.Bundler,allowJs:true,resolveJsonModule:true},host).resolvedModule?.resolvedFileName;if(!resolved||phase===roles.command&&resource.source&&/\.d\.[cm]?ts$/u.test(resolved))throw Error("Selected compile source import missing: "+path+" -> "+specifier,{cause:error});}
   if(phase===roles.command&&resource.source&&/\.d\.[cm]?ts$/u.test(resolved))throw Error("Runtime source cannot be a declaration: "+path+" -> "+specifier);
   const typed=phase===roles.compileInput&&resource.source?compiler.resolveModuleName(resource.specifier,absolute,{moduleResolution:compiler.ModuleResolutionKind.Bundler,allowJs:true,resolveJsonModule:true},host).resolvedModule?.resolvedFileName:undefined;
   for(const file of new Set([resolved,...typed?[typed]:[]])){await yieldInferenceV1(context,"build:selected-compile-imports",++index);const local=buildPath(relative(workspaceRoot,file));if(local===".."||local.startsWith("../")||isAbsolute(local))throw Error("Selected compile source escapes workspace: "+local);const produced=scripts.generatedOutput(file,{importer:path,specifier});if(produced){generated(produced,absolute,phase,target);continue;}readPhysicalSource(file);addFile(local);if(resource.source&&/\.[cm]?[jt]sx?$/u.test(local)){addDependency(local);enqueue(target,file,phase);}}
  }
 }
 for(const dependency of dependencies)for(const input of caretInputs)addInput(dependency,input);
 return {project:json,files:[...files].sort(),dependencies:[...dependencies].sort(),...(Object.keys(generatedDependencies).length?{generatedDependencies}:{}),dependencyInputs:Object.fromEntries([...dependencyInputs].sort(([a],[b])=>a.localeCompare(b)).map(([name,inputs])=>[name,[...inputs].sort()]))};
}

/** 📜️ Reads the workspace's canonical owner declaration through original physical Source ancestry. */
export function readBuildOwnerRegistrySourceV1(workspaceRoot){return pinnedAuthorityReadV1(join(workspaceRoot,"📋️build-owners.json"),()=>readPhysicalSource(join(workspaceRoot,"📋️build-owners.json")).toString("utf8"));}
