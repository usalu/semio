import type { BuildExecutionV1, BuildGraphInvalidationInventoryV1, BuildGraphModuleAuthorityV1, BuildGraphPartStubV1, BuildGraphPartV1, BuildGraphRowV1, BuildGraphV1, BuildProjectV1, BuildTargetV1, BuildUnavailableProjectV1 } from "./📋️model/🟦️.ts";
import{refuseBuildV1}from"../📖️texts/🟦️.ts";
import{staleImplementationV1}from"../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import{buildGraphRetryDiagnosticV1}from"./🔁retry/🟨️.mjs";
import type graphSnapshotPolicy from"../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🔣️policy.json";
import type exactReadSchema from"../📥️inputs/🧮️digest/📥️read/🧬️schema/🔣️.json";
import{captureBoundedSourceRevisionV1,capturePhysicalSourceTextV1,clearPinnedAuthorityV1,implementationPinnedV1,physicalSourceOriginV1,pinnedAuthorityReadV1,withImplementationPinV1,type PhysicalSourceModuleMetaV1}from"../📥️inputs/🔏️revision/🟨️.mjs";
import{fileURLToPath}from"node:url";
import{selectPackageManagerV1}from"../../⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/📚️catalog/🟦️.ts";
const javascriptManagerV1=selectPackageManagerV1("javascript");
import*as observationV1 from"../👁️observation/🟨️.mjs";
import{GENERATION_PIN_POLICY_V1,encodeGenerationPinV1,generationPinOwnerV1,generationPinStateV1,type GenerationPinV1}from"./🕰️generations/🧷️pin/🟨️.mjs";
import{readInventoryRecordLeanV1,rememberInventoryRecordV1}from"../📥️inputs/🪶️lean-inventory/🟨️.mjs";
import{createGraphPhaseClockV1,writeGraphPhaseRowV1}from"./⏱️phases/🟨️.mjs";
import{createProjectFileIndexV1}from"./🧭️file-owner/🟨️.mjs";
import{captureInferenceInventoryReferencesV1}from"../👁️observation/📥️reuse/🟨️.mjs";
import{requireCurrentSourceInventoryV1,captureSourceInputInventoryV1,captureCargoLockInventoryV1,admitExactSourceExtentV1}from"../📥️inputs/📋️data/📥️load/🟨️.mjs";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {withRepositoryBuildGraphConsumerV1,withRepositoryBuildGraphFactStoreV1,withRepositoryBuildGraphSelectionV1} from "../📥️inputs/💾️storage/🟨️.mjs";
import {buildGraphPublicationWriterResourceV1} from "../📥️inputs/💾️storage/🔒️writer/🟨️.mjs";
import {readBuildOwnerRegistryV1,readBuildGraphSelectionV1,closeBuildOwnerAdmissionV1,buildGraphScopeKeyV1,type BuildGraphSelectionV1,type BuildOwnerRegistryV1,type BuildOwnerAdmissionV1} from "./🚪️admission/🟦️.ts";
import{selectLiteralBuildProjectsV1}from"../🧭️selection/📥️options/🟦️.ts";
import { readdir, readFile, open, mkdir, writeFile, rename, lstat, rm } from "node:fs/promises";
import {constants,lstatSync} from "node:fs";
import{withArtifactDescriptorV1}from"../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";
import { join, dirname, relative, isAbsolute, resolve, sep } from "node:path";
import { randomUUID, randomBytes, createHash } from "node:crypto";
import { validateJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { checkScriptInvocation, advanceScriptInvocation, requireIssuedResourceControlInvocationV1, type ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {repositoryCachePartitionRootV1} from "../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import {acquireQueuedResourceLease,acquireResourceLease} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {buildGraphWriterResourceV1} from "../📥️inputs/💾️storage/🔒️writer/🟨️.mjs";
import {withRepositoryCacheStoreV1,type RepositoryCacheStoreV1} from "../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import{decodeGraphRecordsV1}from"../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📇️records/🟨️.mjs";
import{GENERATION_POLICY_V1,createGenerationStoreV1,createNodeFilePortV1,createFrontierV1,diffFrontiersV1,frontierHoldsV1,readGenerationPointerV1,stableDirectoriesV1,canonicalJsonV1 as canonicalGenerationJsonV1}from"./🕰️generations/🟨️.mjs";
import{createClaimRegistryV1,createQueuedClaimLeasePortV1,prepareStaleProjectsV1,renderClaimMessageV1,slotCountV1,type ClaimMessageKeyV1}from"./🕰️generations/🧩️claims/🟨️.mjs";
import{totalmem}from"node:os";
import{loadGenerationGraphV1,renderGenerationMessageV1,survivingStubsV1,withholdDanglingTestProjectsV1}from"./🕰️generations/🔌️load/🟨️.mjs";
import{deriveEnvironmentV1,environmentRowHoldsV1}from"./🕰️generations/🌍️environment/🟨️.mjs";
import{GRAPH_MEMORY_POLICY_V1,graphStringInternerV1,createMemoryPortsV1}from"../🧠️memory/🟨️.mjs";
import{residentGraphSeamsV1}from"../🏡️resident/🕰️graph-hooks/🔌️host/🟨️.mjs";
import{createGenerationFilesV1}from"./🗂️generation-files/🟦️.ts";
import generationSchemaV1 from"./🕰️generations/🧬️schema/🔣️.json"with{type:"json"};
import{declareFormatStampV1,requireCurrentFormatStampsV1}from"../🧷️format-stamps/🟨️.mjs";
import {resolveLanguageV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🟦️.ts";
import {CATALOG_LANGUAGES_V1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
declareFormatStampV1("graph.generation",generationSchemaV1);
/** 🔖️ The declared format stamps whose move makes a loaded graph implementation obsolete: it neither reads nor builds nor publishes a generation any more. */
const graphFormatStampsV1=Object.freeze(["graph.generation","graph.record","graph.inferrers"]);

const graphSnapshotPolicyURLV1=new URL("../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🔣️policy.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),exactReadPolicyURLV1=new URL("../📥️inputs/🧮️digest/📥️read/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),graphSnapshotPolicyCaptureV1=captureBoundedSourceRevisionV1(fileURLToPath(graphSnapshotPolicyURLV1),65536,true),exactReadPolicyCaptureV1=captureBoundedSourceRevisionV1(fileURLToPath(exactReadPolicyURLV1),65536,true);
const graphSnapshotPolicyV1:typeof graphSnapshotPolicy=Object.freeze(JSON.parse(graphSnapshotPolicyCaptureV1.text??"")),exactSourceReadPolicyV1:typeof exactReadSchema.definitions.Policy.const=Object.freeze(JSON.parse(exactReadPolicyCaptureV1.text??"").definitions.Policy.const);
function requireCurrentGraphReadPoliciesV1():void{if(implementationPinnedV1())return;if(captureBoundedSourceRevisionV1(fileURLToPath(graphSnapshotPolicyURLV1),65536).sha256!==graphSnapshotPolicyCaptureV1.sha256||captureBoundedSourceRevisionV1(fileURLToPath(exactReadPolicyURLV1),65536).sha256!==exactReadPolicyCaptureV1.sha256)throw staleImplementationV1("Stale loaded Graph reader policy; restart required");}

/** 📖️ Reads each declaration through the same exact source fact boundary as inference. */
function graphDeclarationV1(path:string):any{const url=new URL(path,physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL));return JSON.parse(pinnedAuthorityReadV1(url.href,()=>{const captured=capturePhysicalSourceTextV1(url,65536),observed=observationV1.observedFs.readFileSync(url,"utf8");if(observed!==captured)throw Error("Current graph declaration Source changed");return observed;}));}
/** 🗂️ The graph implementation files whose bytes the module authority covers, and whose loaded bytes every generation names. */
const graphImplementationUrlsV1=():URL[]=>[new URL(physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../👁️observation/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../👁️observation/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️inputs/🧮️digest/📥️read/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../👁️observation/📥️reuse/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../👁️observation/📥️reuse/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️inputs/📋️data/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️inputs/📋️data/📥️load/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️inputs/📋️data/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️inputs/🧮️digest/📥️read/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("./🚪️admission/🟦️.ts",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../🧭️selection/📥️options/🟦️.ts",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../🧬️schema/🚪️selection.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../🧬️schema/📋️owners.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../🧬️schema/🚪️admission.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../🧬️schema/🪶️lean.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️inputs/💾️storage/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️inputs/💾️storage/🔒️writer/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️inputs/💾️storage/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️inputs/💾️storage/📒️ledger/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️inputs/🔏️revision/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️inputs/🔏️revision/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("./🔁retry/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("./🔁retry/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("./🗂️generation-files/🟦️.ts",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("./🕰️generations/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("./🕰️generations/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("./🕰️generations/🔌️load/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("./🕰️generations/🌍️environment/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("./🧭️file-owner/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🟦️.ts",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/👷️closure-read-worker/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️source/🔏️closure/🟢️node/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️source/🔏️closure/🟦️.ts",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️source/🔏️closure/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/📡️session/🟦️.ts",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/📡️session/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️inputs/🎯️precision/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️inputs/🦀️resolution/🟦️.ts",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️inputs/🦀️resolution/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📥️inputs/🦀️resolution/💾️receipt/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../🧠️memory/🟨️.mjs",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),graphSnapshotPolicyURLV1,new URL("../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/📇️format/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL))];
const graphModuleAuthorityV1=():BuildGraphModuleAuthorityV1=>{requireCurrentSourceInventoryV1();requireCurrentGraphReadPoliciesV1();const observation=observationV1;if(typeof observation.requireCurrentObservationV1!=="function")throw Error("Loaded observation current-instance guard required");observation.requireCurrentObservationV1();const implementation=createHash("sha256");for(const path of graphImplementationUrlsV1())implementation.update(captureBoundedSourceRevisionV1(fileURLToPath(path),8388608).sha256);return {implementation:implementation.digest("hex"),publication:captureBoundedSourceRevisionV1(fileURLToPath(new URL("../🧬️schema/📣️publication.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL))),65536).sha256};};
const loadedGraphModuleAuthorityV1=graphModuleAuthorityV1();
/** 📋️ The loaded graph implementation as workspace facts of the bytes it had when this module loaded; a pinned build names these and no later edit. */
const loadedGraphImplementationRowsV1:readonly {kind:"digest";path:string;fact:{sha256:string;size:number;mtimeMs:number}}[]=graphImplementationUrlsV1().map(url=>{const path=fileURLToPath(url),status=lstatSync(path),captured=captureBoundedSourceRevisionV1(path,8388608);return {kind:"digest" as const,path,fact:{sha256:captured.sha256,size:status.size,mtimeMs:status.mtimeMs}};});

/** 🔒️ Refuses a loaded implementation or declaration superseded while an original invocation waited. */
export function requireBuildGraphModuleAuthorityV1(original:BuildGraphModuleAuthorityV1,current:BuildGraphModuleAuthorityV1):void{
 const schema=graphDeclarationV1("../🧬️schema/📥️authority.json").properties.module;
 if(validateJsonSchemaSubset(schema,original).length||validateJsonSchemaSubset(schema,current).length||original.implementation!==current.implementation||original.publication!==current.publication)throw staleImplementationV1("Loaded build graph source changed; restart the owned build invocation "+JSON.stringify({loaded:original,current}));
}
const requireCurrentGraphModuleV1=():void=>{if(implementationPinnedV1())return;requireBuildGraphModuleAuthorityV1(loadedGraphModuleAuthorityV1,graphModuleAuthorityV1());};

/** 🏛️ Reads immutable defining authority after checking that the loaded graph is still current. */
export function readBuildGraphModuleAuthorityV1():Readonly<BuildGraphModuleAuthorityV1>{requireCurrentGraphModuleV1();return Object.freeze({...loadedGraphModuleAuthorityV1});}

/** 🧷️ Admits only source receipts issued by the exact loaded producer protocol. */
export function buildGraphProducerAuthorityV1(receipt:unknown,module:BuildGraphModuleAuthorityV1):boolean{
 if(validateJsonSchemaSubset(graphDeclarationV1("../🧬️schema/📥️authority.json"),receipt).length)return false;
 const admitted=receipt as {module:BuildGraphModuleAuthorityV1};return admitted.module.implementation===module.implementation&&admitted.module.publication===module.publication;
}

/** ✅️ Admits the language-neutral graph contract and cross-project identities. */
export function readBuildGraphV1(value:unknown):BuildGraphV1 {
  const errors=validateJsonSchemaSubset(graphDeclarationV1("../🧬️schema/🔣️.json"),value);
  if(errors.length)throw Error(`Invalid build graph: ${JSON.stringify(errors.slice(0,4))}`);
  const graph=value as BuildGraphV1;
  if(graph.admission)readBuildGraphSelectionV1(graph.admission.selection);
  const roots=new Map<string,string>();
  for(const[name,project]of Object.entries(graph.projects)){if(name!==project.name)throw Error(`Build project identity differs: ${name}`);const previous=roots.get(project.root);if(previous)throw Error(`Duplicate build project root ${project.root}: ${previous}, ${name}`);roots.set(project.root,name);}
  for(const edge of graph.dependencies)if(!graph.projects[edge.source]&&!graph.externalNodes?.[edge.source]||!graph.projects[edge.target]&&!graph.externalNodes?.[edge.target])throw Error(`Unknown build dependency ${edge.source} -> ${edge.target}`);
  return graph;
}

/** 🔄️ Removes implementation-specific executor names from an inferred operation. */
export function neutralBuildTargetV1(value:Record<string,any>,identity?:string):BuildTargetV1 {
  const {executor,command,options:declaredOptions={},configurations,...rest}=value;
  if(command!==undefined&&(typeof command!=="string"||declaredOptions.command!==undefined||declaredOptions.commands!==undefined))throw Error("Build command shorthand requires one command form");
  const options=command===undefined?declaredOptions:{...declaredOptions,command};
  requireBuildArgumentsV1(options,identity);
  if(executor!==undefined&&!["repo:script","repo:noop"].includes(executor))throw Error(`Unsupported project executor${identity?` for ${identity}`:""}: ${executor}`);
  const normalize=(configuration:Record<string,any>)=>{requireBuildConfigurationV1(options,configuration,identity);return {execution:configuration};};
  return {...rest,execution:{kind:executor==="repo:noop"?"noop":"command",...options},...(configurations?{configurations:Object.fromEntries(Object.entries(configurations).map(([name,configuration])=>[name,normalize(configuration as Record<string,any>)]))}:{})};
}


/** 🔐️ Keeps configured execution options inside the same task and command identity. */
function requireBuildConfigurationV1(base:Record<string,unknown>,configuration:Record<string,unknown>,identity?:string):void {
  if(!configuration||typeof configuration!=="object"||Array.isArray(configuration))throw Error("Build configuration execution options required");
  if(configuration.env!==undefined&&(!configuration.env||typeof configuration.env!=="object"||Array.isArray(configuration.env)||Object.values(configuration.env).some(value=>typeof value!=="string")))throw Error("Build configuration environment strings required");
  requireBuildArgumentsV1(configuration,identity);
  const fixed=["kind","program","ownerArguments","nativeOwnerCommand","executor","inputs","outputs","dependsOn","cache","continuous","parallelism","metadata","defaultConfiguration","configurations","options"];
  if(Object.keys(configuration).some(key=>fixed.includes(key)))throw Error("Build configuration cannot change task identity");
  if(base.program!==undefined&&(configuration.command!==undefined||configuration.commands!==undefined)||configuration.command!==undefined&&(configuration.commands!==undefined||base.commands!==undefined)||configuration.commands!==undefined&&base.command!==undefined)throw Error("Build configuration cannot change command form");
}

/** 🔡️ Keeps authored argument boundaries literal through every backend's execution protocol. */
function requireBuildArgumentsV1(options:Record<string,unknown>,identity?:string):void {
  if(options.program!==undefined&&(typeof options.program!=="string"||!options.program||options.command!==undefined||options.commands!==undefined))throw Error("Literal build program requires one exclusive nonempty executable");
  if(options.args!==undefined&&(!Array.isArray(options.args)||options.args.some(value=>typeof value!=="string")))throw Error(`Build execution arguments must be literal string arrays${identity?` for ${identity}`:""}`);
}

/** 🗃️ Locates the sole published portable graph. */
export function buildGraphPathV1(root:string):string { const publication=graphDeclarationV1("../🧬️schema/📣️publication.json").properties;return join(repositoryCachePartitionRootV1(root,publication.partition.const),publication.graph.const); }

/** 🗑️ Selects only owned current publications and their bounded staging files. */
export function planBuildGraphInvalidationV1(inventory:BuildGraphInvalidationInventoryV1):string[] {
  const schema=graphDeclarationV1("../🧬️schema/🗑️reset.json");if(validateJsonSchemaSubset(schema,inventory).length)throw Error("Build graph reset inventory required");
  const owned=new RegExp(schema.$defs.OwnedName.pattern,"u"),selected=inventory.entries.filter(entry=>owned.test(entry.name));
  for(const entry of selected)if(validateJsonSchemaSubset(schema.$defs.OwnedEntry,entry,schema).length||entry.device!==inventory.device)throw Error("Owned regular bounded graph publication required");
  return selected.map(entry=>entry.name).sort();
}

/** 🛑️ Serializes graph publication and invalidation inside the original enforcing store. */
async function withBuildGraphStateV1<T>(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,body:(store:RepositoryCacheStoreV1,actualControl:ResourceLeaseControl)=>Promise<T>,scope?:string):Promise<T> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);requireCurrentGraphModuleV1();
  const run=async(store:RepositoryCacheStoreV1,actualControl:ResourceLeaseControl)=>{
    const writer=await acquireQueuedResourceLease({directory:join(root,".cache/b/l/m/data"),resource:scope?buildGraphPublicationWriterResourceV1(store.root,scope):"build-graph-state:"+store.root,mode:"exclusive",owner:randomBytes(16).toString("base64url"),control:actualControl,signal:actualControl.signal});
    try{requireCurrentGraphModuleV1();checkResourceLeaseControl(actualControl);return await body(store,actualControl);}finally{writer.release();}
  };
  return scope?withRepositoryBuildGraphSelectionV1(root,invocation,phaseControl,scope,run,scope===generationScopeV1?{optionalCaches:false,shared:true}:undefined):withRepositoryBuildGraphFactStoreV1(root,invocation,phaseControl,run);
}

/** ♻️ Invalidates current graph authority while retaining charged storage, immutable pins and source facts. */
export async function invalidateBuildGraphV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,beforeInvalidation?:(actualControl:ResourceLeaseControl)=>Promise<void>):Promise<void> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);await advanceScriptInvocation(invocation,"build:reset","running");
  await withBuildGraphStateV1(root,invocation,phaseControl,async(store,actualControl)=>{
    await store.recheck();if(store.root!==dirname(buildGraphPathV1(root)))throw Error("Original graph publication store required");
    const device=(await lstat(store.root)).dev,entries:BuildGraphInvalidationInventoryV1["entries"]=[];
    for(const name of await readdir(store.root)){
      checkScriptInvocation(invocation);const status=await lstat(join(store.root,name));entries.push({name,kind:status.isSymbolicLink()?"symlink":status.isFile()?"file":status.isDirectory()?"directory":"other",links:status.nlink,bytes:status.size,device:status.dev});
      if(entries.length%64===0)await advanceScriptInvocation(invocation,"build:reset-inventory","running");
    }
    const selected=planBuildGraphInvalidationV1({device,entries});checkScriptInvocation(invocation);checkResourceLeaseControl(actualControl);if(beforeInvalidation)await beforeInvalidation(actualControl);checkResourceLeaseControl(actualControl);checkScriptInvocation(invocation);
    for(const name of selected){
      checkScriptInvocation(invocation);await store.recheck();const path=requireNativeStoragePathV1(store.root,join(store.root,name)),status=await lstat(path);planBuildGraphInvalidationV1({device,entries:[{name,kind:status.isSymbolicLink()?"symlink":status.isFile()?"file":status.isDirectory()?"directory":"other",links:status.nlink,bytes:status.size,device:status.dev}]});await rm(path);await advanceScriptInvocation(invocation,"build:reset-publication","running");
    }
    await createGenerationStoreV1({root:join(store.root,"generations"),fs:generationFilesV1(store),holds:async()=>true,resources:{facts:"repository-graph-facts:"+resolve(root)}}).invalidate({reason:"reset",signal:invocation.control.signal});
  });
}

const inventoryDirectoriesV1=new WeakMap<string[],string[]>();

/** 🧊️ The expanded input lists of closure inventories this process already read, by repository and content address: a content addressed leaf expands to the same list in every chunk, so a chunked expansion reads each distinct closure once. Bounded by the declared entry count; over it the table starts again. */
const expandedClosuresV1=new Map<string,string[]>();let expandedClosureEntriesV1=0;
function rememberExpandedClosureV1(key:string,list:string[]):void{
  if(expandedClosureEntriesV1+list.length>GRAPH_MEMORY_POLICY_V1.maximumCachedClosureEntries){expandedClosuresV1.clear();expandedClosureEntriesV1=0;}
  expandedClosuresV1.set(key,list);expandedClosureEntriesV1+=list.length;
}

const releasedBuildGraphsV1=new WeakSet<object>(),buildGraphContentKeysV1=["projects","dependencies","namedInputs","externalNodes","unavailable","admission"] as const;

/** 🕊️ Drops the content of graphs whose planning is done and the caches built to expand them, then collects once the event loop turned (a collection inside the releasing call still sees the caller's stack), so a long build phase holds only what it plans from: every later read of a released graph throws instead of silently seeing nothing. Callers hold the shell, never the content; a consumer that needs the graph again reads the pinned snapshot. */
export async function releaseBuildGraphV1(...graphs:readonly BuildGraphV1[]):Promise<void>{
  for(const graph of graphs){
    if(releasedBuildGraphsV1.has(graph))continue;
    releasedBuildGraphsV1.add(graph);
    for(const key of buildGraphContentKeysV1)Object.defineProperty(graph,key,{configurable:true,enumerable:false,get(){throw refuseBuildV1("stale","graph.released-read");}});
  }
  expandedClosuresV1.clear();expandedClosureEntriesV1=0;graphStringInternerV1.clear();
  const memory=createMemoryPortsV1();await new Promise<void>(resolve=>setTimeout(resolve,0));memory.collect();
}

/** 🔨️ Refuses a graph whose content was released. */
export function requireLiveBuildGraphV1(graph:BuildGraphV1):BuildGraphV1{
  if(releasedBuildGraphsV1.has(graph))throw refuseBuildV1("stale","graph.released-read");
  return graph;
}

/** 📁️ Discovers real workspace authority while yielding cancellation and progress. */
export async function workspaceBuildFilesV1(root:string,invocation?:ScriptInvocation):Promise<string[]> {
  const cachePolicy=graphDeclarationV1("../../⚡️caching/🔣️policy.json"),ownership=graphDeclarationV1("../../🗂️workspaces/🔣️policy.json"),excluded=new Set([...cachePolicy.generatedDirectories,".git",".🧬semio",".nx",".cache","node_modules","dist","target",".venv","venv","__pycache__",".pytest_cache",".next",".turbo","temp","🗑️generated"]),pending=[""],files:string[]=[],directories:string[]=[];
  let visited=0;
  while(pending.length){
    if(invocation)checkScriptInvocation(invocation);
    const batch=pending.splice(-32),listings=await Promise.all(batch.map(async directory=>{
      try{return {directory,entries:await readdir(join(root,directory),{withFileTypes:true})};}
      catch(error){if(directory&&["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))throw Object.assign(Error(`Build directory removed during discovery: ${directory}`),{code:"BUILD_MEMBERSHIP_CHANGED"});throw error;}
    }));
    if(invocation)checkScriptInvocation(invocation);
    for(const {directory,entries}of listings){
      directories.push(directory||".");
      for(const entry of entries){
        if(entry.isSymbolicLink())continue;
        const path=join(directory,entry.name).replaceAll("\\","/");
        if(entry.isDirectory()){if(!excluded.has(entry.name)&&!entry.name.startsWith("extension-host-build-")&&!(ownership.collections.names.includes(entry.name)&&directory.split("/").at(-1)!==ownership.collections.moduleMember)&&path!=="compose")pending.push(path);}
        else if(entry.isFile())files.push(path);
      }
    }
    const previous=visited;visited+=batch.length;if(Math.floor(visited/128)!==Math.floor(previous/128)&&invocation)await advanceScriptInvocation(invocation,"build:discover","running");
  }
  files.sort();inventoryDirectoriesV1.set(files,directories.sort());if(invocation)await advanceScriptInvocation(invocation,"build:discover","complete");return files;
}

/** 📥️ Expands private input compaction into portable source declarations. */
function portableInputs(root:string,value:unknown,invocation:ScriptInvocation,control:ResourceLeaseControl):unknown|Promise<unknown> {
  const token=(item:unknown)=>{
    if(!item||typeof item!=="object"||Array.isArray(item))return undefined;
    const object=item as Record<string,unknown>;if(Object.keys(object).length!==1||typeof object.env!=="string")return undefined;
    const match=/^SEMIO_(SOURCE_CLOSURE|CARGO_LOCK)_([a-f0-9]{64})$/u.exec(object.env);
    return match?{key:object.env,digest:match[2]!,directory:match[1]==="SOURCE_CLOSURE"?"build-input-closures":"build-lock-closures"}:undefined;
  };
  captureInferenceInventoryReferencesV1(value);
  const pending:unknown[]=[value],seen=new WeakSet<object>(),tokens=new Map<string,NonNullable<ReturnType<typeof token>>>();let visited=0;
  while(pending.length){
    if(invocation&&visited++%1024===0)checkScriptInvocation(invocation);
    const item=pending.pop();if(!item||typeof item!=="object"||seen.has(item))continue;seen.add(item);
    const compact=token(item);if(compact)tokens.set(compact.key,compact);else for(const child of Object.values(item))if(child&&typeof child==="object")pending.push(child);
  }
  if(!tokens.size)return value;
  const expand=async()=>{
    const entries=[...tokens.values()],expanded=new Map<string,string[]>();
    await withRepositoryBuildGraphConsumerV1(root,control,async(store,actual)=>{
      const clock=createGraphPhaseClockV1({publish:resource=>actual.publish({resource,mode:"shared",elapsedMs:0}),write:writeGraphPhaseRowV1});
      let bytes=0,readRecords=0;const cadence=exactSourceReadPolicyV1.maximumInventoryRecordsPerYield;
      await store.recheck();checkResourceLeaseControl(actual);
      if(invocation)await advanceScriptInvocation(invocation,"build:portable-inputs","running");
      await clock.begin("expansion",entries.length);
      for(const compact of entries){
        checkScriptInvocation(invocation);checkResourceLeaseControl(actual);
        const known=expandedClosuresV1.get(root+"\0"+compact.key);
        if(known)expanded.set(compact.key,known);
        else{
          const directory=join(store.root,compact.directory),file=readInventoryRecordLeanV1(directory,compact.digest,exactSourceReadPolicyV1.maximumClosureBytes);
          if(file.sha256!==compact.digest)throw Error(`Build input closure content differs: ${compact.key}`);
          bytes+=file.byteLength;if(bytes>exactSourceReadPolicyV1.maximumSourceBytes)throw Error("Build input closure bytes exceeded");
          const list=compact.directory==="build-input-closures"?captureSourceInputInventoryV1(file.value,root).value.files.map(file=>graphStringInternerV1.share("{workspaceRoot}/"+file)):["{workspaceRoot}/Cargo.lock","{workspaceRoot}/"+captureCargoLockInventoryV1(file.value,root).value.manifest];
          expanded.set(compact.key,list);rememberExpandedClosureV1(root+"\0"+compact.key,list);
          rememberInventoryRecordV1(root,directory,compact.digest,file.stamp);
        }
        if(++readRecords%cadence===0){await store.recheck();await actual.yieldContinuation();checkScriptInvocation(invocation);checkResourceLeaseControl(actual);await clock.advance("expansion",readRecords);if(invocation)await advanceScriptInvocation(invocation,"build:portable-inputs","running");}
      }
      await store.recheck();checkResourceLeaseControl(actual);await clock.advance("expansion",readRecords);await clock.end("expansion",{records:readRecords,bytes});
    });
    const copies=new WeakMap<object,unknown>();visited=0;
    const transform=(item:unknown):unknown=>{
      if(invocation&&visited++%1024===0)checkScriptInvocation(invocation);
      if(!item||typeof item!=="object")return item;
      const compact=token(item);if(compact)return expanded.get(compact.key)!;
      if(copies.has(item))return copies.get(item);
      if(Array.isArray(item)){
        let result:unknown[]|undefined;
        for(let index=0;index<item.length;index++){const child=item[index],next=transform(child),flatten=token(child)!==undefined;if(!result&&(flatten||next!==child))result=item.slice(0,index);if(result){if(flatten)for(const entry of next as unknown[])result.push(entry);else result.push(next);}}
        copies.set(item,result??item);return result??item;
      }
      let result:Record<string,unknown>|undefined;
      for(const[key,child]of Object.entries(item)){const next=transform(child);if(next!==child&&!result)result={...item};if(result)result[key]=next;}
      copies.set(item,result??item);return result??item;
    };
    const result=transform(value);if(invocation)await advanceScriptInvocation(invocation,"build:portable-inputs","complete");return result;
  };
  return expand();
}

/** 📇️ Refuses caller registry data superseded before the current source boundary. */
export function requireBuildOwnerRegistryAuthorityV1(original:unknown,current:unknown):void{
 if(JSON.stringify(readBuildOwnerRegistryV1(original))!==JSON.stringify(readBuildOwnerRegistryV1(current)))throw Object.assign(Error("Original build owner registry changed before source consumption"),{code:"BUILD_SOURCE_AUTHORITY"});
}

/** 🚧️ The digest of the bytes of one workspace file as the snapshot observes them; a file that vanished while the graph was built has the digest of no bytes. */
function observedFileDigestV1(root:string,file:string):string{
  try{return createHash("sha256").update(observationV1.observedFs.readFileSync(join(root,file))).digest("hex");}
  catch(error){if(["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))return createHash("sha256").digest("hex");throw error;}
}

/** 📚️ Assembles all owned inference contributions without installing or invoking Nx. */
export async function discoverBuildGraphV1(root:string,selection:BuildGraphSelectionV1,registry:BuildOwnerRegistryV1,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,suppliedFiles?:string[],admission?:BuildOwnerAdmissionV1,compact=false):Promise<BuildGraphV1> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);selection=readBuildGraphSelectionV1(selection);registry=readBuildOwnerRegistryV1(registry);
  requireBuildOwnerRegistryAuthorityV1(registry,JSON.parse(pinnedAuthorityReadV1(join(root,"📋️build-owners.json"),()=>observationV1.observedFs.readFileSync(join(root,"📋️build-owners.json"),"utf8"))));
  if(selection.kind!=="all"&&!admission)throw Error("Closed selected owner admission required");
  const files=suppliedFiles??(selection.kind==="all"?buildGraphMembersV1(await workspaceBuildFilesV1(root,invocation)):admission!.files);
  const [owner,tests,policies]=await Promise.all([import("./📥️inference/🟨️.mjs"),import("../../../🧪️test/📦️packages/🟦️typescript/🟨️.mjs"),import("./📥️inference/🛡️policy/🟨️.mjs")]);checkResourceLeaseControl(phaseControl);requireCurrentGraphModuleV1();
  const policy=graphDeclarationV1("../🔣️policy.json"),options={analyzeLockfile:true,catalogProviders:selection.kind==="all"?registry.catalogProviders.map(({ownerRoot,sourcePath})=>({ownerRoot,sourcePath})):admission!.catalogProviders,...(selection.kind!=="all"?{ownerAdmission:admission}:{})};
  const primary=await owner.discoverBuildProjectsV1(files.filter(file=>file.endsWith("📋️project.json")||file.endsWith("Cargo.toml")||javascriptManagerV1.lockFileNames().includes(file)||file.endsWith(".patch")),options,{workspaceRoot:root,invocation,phaseControl,registry});
  if(invocation)await advanceScriptInvocation(invocation,"build:infer","running");
  const contributions=[...primary,...await tests.discoverTestBuildProjectsV1(files.filter(file=>file.endsWith(".feature")),{}, {workspaceRoot:root,invocation,phaseControl})];
  const policyOwners=Object.assign({},...contributions.map(([,contribution])=>contribution.projects??{}));
  contributions.push(...await policies.default.createNodesV2[1](files.filter(file=>file.endsWith("📜️script.ts")),{}, {workspaceRoot:root,projects:policyOwners}));
  const projects:Record<string,any>={},externalNodes:Record<string,unknown>={},unavailable=new Map<string,BuildUnavailableProjectV1>(),configs:Record<string,string>={};
  for(const[file,contribution]of contributions){
    Object.assign(externalNodes,contribution.externalNodes);
    for(const row of (contribution.unavailable??[]) as BuildUnavailableProjectV1[])unavailable.set(row.root,row);
    for(const[name,project]of Object.entries(contribution.projects??{}) as [string,any][]){
      if(projects[name]&&projects[name].root!==project.root)throw Error(`Duplicate build project ${name}`);
      projects[name]={...projects[name],...project,name,targets:{...projects[name]?.targets,...project.targets}};
      configs[name]??=file;
    }
  }
  withholdDanglingTestProjectsV1({projects,configs,unavailable,dangling:tests.danglingTestDependenciesV1(projects),digest:file=>observedFileDigestV1(root,file)});
  const ownerOf=createProjectFileIndexV1(projects),projectFileMap:Record<string,{file:string}[]>=Object.fromEntries(Object.keys(projects).map(name=>[name,[]]));
  const withheld=[...unavailable.keys()];
  for(const file of files){const owner=ownerOf(file);if(owner!==undefined&&!withheld.some(root=>file.startsWith(root+"/")&&root.length>projects[owner]!.root.length)){const entries=projectFileMap[owner];if(!entries)throw Error("Discovered project file owner is missing");entries.push({file});}}
  const context={workspaceRoot:root,projects,fileMap:{projectFileMap},invocation,phaseControl,selection,registry,completeSourceImports:selection.kind!=="all"};
  const testEdges:unknown=await tests.discoverTestBuildDependenciesV1({},context);if(!Array.isArray(testEdges))throw Error("Test dependency DATA array required");const edges=[...await owner.discoverBuildDependenciesV1(options,context),...testEdges];
  const metadata=await owner.createMetadata({nodes:Object.fromEntries(Object.entries(projects).map(([name,data])=>[name,{name,type:"lib",data}]))},options,context);
  if(!metadata||typeof metadata!=="object"||Array.isArray(metadata))throw Error("Graph metadata DATA object required");
  for(const[name,value]of Object.entries(metadata)){if(!value||typeof value!=="object"||!("metadata"in value)||!value.metadata||typeof value.metadata!=="object"||Array.isArray(value.metadata))throw Error("Project metadata DATA object required");if("semio"in value.metadata&&value.metadata.semio!==undefined&&(!value.metadata.semio||typeof value.metadata.semio!=="object"||Array.isArray(value.metadata.semio)))throw Error("Project semio metadata DATA object required");projects[name].metadata={...projects[name].metadata,...value.metadata,semio:{...projects[name].metadata?.semio,...("semio"in value.metadata&&value.metadata.semio&&typeof value.metadata.semio==="object"?value.metadata.semio:{})}};}
  const neutral=Object.fromEntries(Object.entries(projects).map(([name,project])=>[name,{...project,targets:Object.fromEntries(Object.entries(project.targets??{}).map(([target,value])=>[target,neutralBuildTargetV1(value as Record<string,any>,`${name}:${target}`)]))}]));
  const dependencies=[...new Map(edges.map(edge=>[JSON.stringify(edge),edge])).entries()].sort(([a],[b])=>a.localeCompare(b)).map(([,edge])=>edge);
  if(invocation)await advanceScriptInvocation(invocation,"build:graph","complete");
  const compactGraph=({version:1,projects:neutral,dependencies,namedInputs:policy.namedInputs,externalNodes,...(unavailable.size?{unavailable:[...unavailable.values()].sort((a,b)=>a.root<b.root?-1:a.root>b.root?1:0)}:{}),admission:{selection,registry:createHash("sha256").update(JSON.stringify(registry)).digest("hex"),closure:createHash("sha256").update(JSON.stringify(admission??files)).digest("hex")}});
  return compact?compactGraph as unknown as BuildGraphV1:readBuildGraphV1({...compactGraph,projects:await portableInputs(root,neutral,invocation,phaseControl)});
}

/** 📍️ The pin of the generation the newest graph load of this process served; the descendants that answer to this invocation read exactly that generation. */
let sealedGenerationPinTextV1:string|undefined;
/** 🎁️ Hands the pin of the newest generation load to the ones that start nested commands; with a `tool` only when that build tool issues every task environment itself (the others keep the graph of their own nested commands). */
export function sealedGenerationPinV1(tool?:string):string|undefined{return tool===undefined||GENERATION_PIN_POLICY_V1.tools.includes(tool)?sealedGenerationPinTextV1:undefined;}
/** 🏷️ Takes over the pin a graph child of this invocation reported (or forgets it with undefined). */
export function recordSealedGenerationPinV1(text:string|undefined):void{sealedGenerationPinTextV1=text;}
/** 🧭️ The pin this command is bound by: the one its Original invocation put in the environment while that process is alive; a pin of anyone else is none. */
function boundGenerationPinV1():GenerationPinV1|undefined{const state=generationPinStateV1(process.env);return state.state==="bound"?state.pin:undefined;}

/** 🏷️ The tags of the owners the test and policy discovery produces. An owner with one of them is kept from generation to generation and holds the observations of the whole discovery, so it is discovered again only when one of them is stale, the environment moved or an input of the discovery was added, removed or changed; an owner with another tag (the tags before the observations were kept) is discovered again once. */
const TEST_TAG_V1="tests-v2",POLICY_TAG_V1="policy-v2";

/** ♻️ Whether the test and policy owners of the previous generation are kept as they are: there is a previous generation whose owners carry the current tags, the environment did not move, no tagged owner is dirty (its root is among the roots being refreshed) and no input of the discovery (a `.feature` file or a `📜️script.ts`) was added, removed or changed since; an unknown change list counts as a change. */
export function taggedOwnersRetainedV1({cold,changed,roots,stubs,environment,previousEnvironment}:{cold:boolean;changed:readonly string[]|undefined;roots:ReadonlySet<string>;stubs:Record<string,{root:string;tag?:string}>;environment:string;previousEnvironment:string|null}):boolean{
  const tagged=Object.values(stubs).filter(stub=>stub.tag!==undefined);
  return !cold&&changed!==undefined&&tagged.length>0&&tagged.every(stub=>stub.tag===TEST_TAG_V1||stub.tag===POLICY_TAG_V1)&&environment===previousEnvironment&&!tagged.some(stub=>roots.has(stub.root))&&!changed.some(path=>path.endsWith(".feature")||path.endsWith("📜️script.ts"));
}

/** 🔌️ The identity of the one builder custody of the generation store: every builder of the all-workspace graph queues on it, shared readers never do. */
const generationScopeV1=createHash("sha256").update("semio-graph-generations-v1").digest("hex");

/** 📈️ Opens the generation documents of the graph partition: every path stays inside the original store and the store is rechecked while documents move. */
function generationFilesV1(store:RepositoryCacheStoreV1){
  return createGenerationFilesV1({store,base:createNodeFilePortV1(),requirePath:requireNativeStoragePathV1});
}

/** 🪪️ The identity of the generation a registry may read: the store format and the catalog providers whose output reaches every project. The owners of the registry are no identity: an added or removed project is a membership change the generation absorbs incrementally, and the one owner row an inference reads is a recorded `registry` fact. */
export function buildGraphGenerationIdentityV1(registry:BuildOwnerRegistryV1):string{
  return createHash("sha256").update(JSON.stringify([generationSchemaV1.definitions.Identity.const,registry.catalogProviders.map(({ownerRoot,sourcePath})=>[ownerRoot,sourcePath])])).digest("hex");
}

/** 🕰️ Binds the repository to the graph generation load: shared reader custody, the builder custody, the bounded facts lease, fact checks that understand environment rows, the membership frontier and the partial inference. */
function generationHostV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,registry:BuildOwnerRegistryV1,compact=false){
  const registryDigest=createHash("sha256").update(JSON.stringify(registry)).digest("hex"),state={files:[] as string[],stable:new Set<string>()},leaseDirectory=join(root,".cache/b/l/m/data");
  const read=async(path:string)=>{try{return await readFile(join(root,path),"utf8");}catch(error){if(["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))return undefined;throw error;}};
  const statFile=async(path:string)=>{try{const status=await lstat(join(root,path));return status.isFile()?{size:status.size,mtimeMs:status.mtimeMs}:undefined;}catch(error){if(["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))return undefined;throw error;}};
  const statEntry=async(path:string)=>{try{const status=await lstat(join(root,path));return {directory:status.isDirectory(),file:status.isFile(),mtimeMs:status.mtimeMs,ino:status.ino,mode:status.mode};}catch(error){if(["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))return undefined;throw error;}};
  let analyses:Promise<void>|undefined;
  const analysesPrepared=():Promise<void>=>analyses??=import("./📥️inference/🟨️.mjs").then(async producer=>{await producer.libraryBootstrap;producer.prepareSourceAnalysesV1();});
  let deriver:Promise<(row:{kind:string;path:string;fact:unknown},signal?:AbortSignal)=>Promise<{fact:unknown;rows:readonly {kind:string;path:string;fact:unknown}[]}>>|undefined;
  const deriveAgain=async(row:{kind:string;path:string;fact:unknown},signal?:AbortSignal)=>{deriver??=import("./📥️inference/🟨️.mjs").then(async producer=>{await producer.libraryBootstrap;producer.prepareSourceAnalysesV1();return producer.openGenerationDeriverV1({workspaceRoot:root,registry});});return(await deriver)(row,signal) as never;};
  const holdsByStat=async(rows:readonly {kind:string;path:string;fact:any}[],signal?:AbortSignal):Promise<boolean>=>{
    const plain:typeof rows[number][]=[],own:typeof rows[number][]=[];for(const row of rows){if(row.kind==="closure")continue;(row.kind==="field"||row.kind==="digest"?own:plain).push(row);}
    for(let at=0;at<own.length;at+=64){signal?.throwIfAborted();if(!(await Promise.all(own.slice(at,at+64).map(row=>environmentRowHoldsV1(row as never,{read,stat:statFile})))).every(Boolean))return false;}
    if(plain.length===0)return true;
    await analysesPrepared();
    return observationV1.observationsHold(plain,64,state.stable,true,signal);
  };
  const lease=(control:ResourceLeaseControl)=>({async exclusive<T>(resource:string,body:()=>Promise<T>,options:{signal?:AbortSignal;timeoutMs?:number}={}):Promise<T>{
    const held=await acquireQueuedResourceLease({directory:leaseDirectory,resource,mode:"exclusive",owner:randomBytes(16).toString("base64url"),control,signal:options.signal??control.signal,...(options.timeoutMs!==undefined?{timeoutMs:options.timeoutMs}:{})});
    try{return await body();}finally{held.release();}
  }});
  const membershipByStat=async(previous:{manifest:{membership:string};frontier:any}|undefined,signal?:AbortSignal)=>{
    if(previous&&await frontierHoldsV1(previous.frontier,statEntry,{signal})){state.files=previous.frontier.members.map((row:[string,number])=>row[0]);return {digest:previous.manifest.membership,frontier:previous.frontier,changes:[]};}
    const capturedAt=Date.now(),all=await workspaceBuildFilesV1(root,invocation),members=buildGraphMembersV1(all),directoryPaths=inventoryDirectoriesV1.get(all)??[];
    const stamp=async<T>(paths:readonly string[],row:(path:string,status:NonNullable<Awaited<ReturnType<typeof statEntry>>>)=>T):Promise<T[]>=>{
      const out:T[]=new Array(paths.length);let next=0;
      await Promise.all(Array.from({length:Math.min(64,paths.length)},async()=>{while(next<paths.length){const at=next++;checkScriptInvocation(invocation);const status=await statEntry(paths[at]!);if(!status)throw Object.assign(Error(`Build member removed during discovery: ${paths[at]}`),{code:"BUILD_MEMBERSHIP_CHANGED"});out[at]=row(paths[at]!,status);}}));
      return out;
    };
    const directories=await stamp(directoryPaths,(path,status)=>[path,status.mtimeMs,status.ino] as [string,number,number]),stamped=await stamp(members,(path,status)=>[path,status.mode] as [string,number]);
    const frontier=createFrontierV1({directories,members:stamped,capturedAt}),digest=createHash("sha256").update(canonicalGenerationJsonV1(frontier.members)).digest("hex");
    state.files=members;
    const config=(path:string)=>["📋️project.json","Cargo.toml"].includes(path.slice(path.lastIndexOf("/")+1));
    return {digest,frontier,changes:previous?diffFrontiersV1(previous.frontier,frontier).map(change=>({...change,...(!change.directory&&config(change.path)?{config:true}:{})})):[]};
  };
  const seams=residentGraphSeamsV1({root,membership:membershipByStat,holds:holdsByStat,adopt:(files:string[])=>{state.files=files;}}),holds=seams.holds;
  const membership=async(previous:Parameters<typeof seams.membership>[0],signal?:AbortSignal)=>{
    const live=await seams.membership(previous,signal);
    state.stable=previous?.frontier?new Set([...stableDirectoriesV1(previous.frontier,live.frontier)].map(path=>join(root,...path.split("/")))):new Set();
    return live;
  };
  return {
    workspaceRoot:root,separator:sep,language:resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language,namedInputs:graphDeclarationV1("../🔣️policy.json").namedInputs,registryDigest,signal:invocation.control.signal,verbose:process.env.SEMIO_GRAPH_PHASES==="1"||process.env.SEMIO_BUILD_PROFILE_INPUTS==="1",
    async identity(){requireCurrentFormatStampsV1(graphFormatStampsV1,"graph generation","refuse");return buildGraphGenerationIdentityV1(registry);},
    async implementationRows(){const producer=await import("./📥️inference/🟨️.mjs");await producer.libraryBootstrap;const rows:{kind:"digest";path:string;fact:{sha256:string;size:number;mtimeMs:number}}[]=[];for(const row of [...loadedGraphImplementationRowsV1,...producer.loadedImplementationRowsV1]){const inside=relative(root,row.path);if(inside===""||inside.startsWith("..")||isAbsolute(inside))continue;rows.push({...row,path:inside.split(sep).join("/")});}return producer.classifyImplementationRowsV1(rows);},
    release:async()=>{observationV1.releaseInferenceMemoryV1();(await import("../📥️inputs/🟨️.mjs")).releaseInputMemoryV1();(await import("./📥️inference/🧭️script-inputs/🟨️.mjs")).resetClosureEntriesV1();graphStringInternerV1.clear();createMemoryPortsV1().collect();},
    withShared:<T>(body:(store:RepositoryCacheStoreV1,control:ResourceLeaseControl)=>Promise<T>)=>withRepositoryCacheStoreV1(root,"graph",invocation,phaseControl,async(store,control)=>{await store.recheck();checkResourceLeaseControl(control);return body(store,control);}),
    withBuilder:async<T>(body:(store:RepositoryCacheStoreV1,control:ResourceLeaseControl)=>Promise<T>)=>{
      let image:string|undefined;
      const result=await withBuildGraphStateV1(root,invocation,phaseControl,(store,control)=>{image=store.root;return body(store,control);},generationScopeV1);
      if(image!==undefined)await sweepSupersededBlocksV1(root,image,invocation,phaseControl,(key,fields)=>process.stderr.write(renderGenerationMessageV1(key as never,fields,resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language)+"\n"));
      return result;
    },
    storeOptions:(store:RepositoryCacheStoreV1,control:ResourceLeaseControl)=>({root:join(store.root,"generations"),fs:generationFilesV1(store),lease:lease(control),holds,derive:deriveAgain,resources:{facts:"repository-graph-facts:"+resolve(root)}}),
    membership,
    discoverPart:(spec:{cold:boolean;owners:string[];discover:string[];removed:string[];changed?:string[];previousEnv:string|null;stubs:Record<string,BuildGraphPartStubV1>},signal:AbortSignal|undefined,control:ResourceLeaseControl)=>{signal?.throwIfAborted();requireCurrentFormatStampsV1(graphFormatStampsV1,"graph generation");return discoverBuildGraphPartV1(root,registry,invocation,control,state.files,{cold:spec.cold,changed:spec.changed,roots:new Set([...spec.owners.map(name=>spec.stubs[name]!.root),...spec.discover.map(path=>dirname(path))]),stubs:survivingStubsV1(spec.stubs,spec.removed),previousEnv:spec.previousEnv});},
    expand:async(projects:Record<string,unknown>,control:ResourceLeaseControl)=>compact?projects:(await portableInputs(root,projects,invocation,control)) as Record<string,unknown>,
    expansionLimits:{maximumProjects:32,maximumBytes:Math.floor(graphDeclarationV1("../👁️observation/📥️reuse/🧬️schema/🔣️.json").definitions.Policy.const.maximumTextBytes/4)},
    readLean:(value:unknown)=>readBuildGraphLeanV1(value),
    readGraph:(value:unknown)=>readBuildGraphV1(value),
    sealed:(record:{generation:number;manifest:string;owners:readonly string[];covered:ReadonlySet<string>|undefined;atMs:number})=>{sealedGenerationPinTextV1=encodeGenerationPinV1({owner:generationPinOwnerV1(process.env),...record});},
    clock:(control:ResourceLeaseControl)=>createGraphPhaseClockV1({publish:resource=>control.publish({resource,mode:"shared",elapsedMs:0}),write:writeGraphPhaseRowV1}),
    say:(key:string,fields:Record<string,string|number>)=>{process.stderr.write(renderGenerationMessageV1(key as never,fields,resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language)+"\n");},
    prefetch(request:GenerationPrefetchRequestV1){return prefetchStaleGenerationV1({root,invocation,phaseControl,registry,files:()=>state.files,request,release:()=>this.release(),language:resolveLanguageV1(CATALOG_LANGUAGES_V1,[],process.env).language});},
  };
}

/** 🧹️ After a build, removes the observation blocks no record names any more once a record directory holds many more blocks than its records can name (each re-inference supersedes blocks, and a graph store whose inventory is exhausted refuses every command), outside the builder custody, group by group under the writer lease of the group, and never lets a failing sweep fail the build. */
async function sweepSupersededBlocksV1(root:string,image:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,say:(key:string,fields:Record<string,string|number>)=>void):Promise<void>{
  try{
    const scan=await import("../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🔎️scan/🟦️.ts"),directory=join(root,".cache/b/l/m/data");
    const timedOut=(error:unknown)=>/timed out/iu.test(String((error as Error).message)),resourceOf=(kind:"selection"|"dataset"|"canonical",identity:string)=>buildGraphWriterResourceV1(root,kind,identity);
    const report=await scan.supersedeBuildGraphObservationBlocksV1(image,{lease:scan.createSweepLeasePortV1({
      acquire:async(kind,identity)=>{try{return await acquireResourceLease({directory,resource:resourceOf(kind,identity),mode:"exclusive",signal:invocation.control.signal,control:phaseControl,timeoutMs:0});}catch(error){if(timedOut(error))return undefined;throw error;}},
      acquireQueued:async(kind,identity,timeoutMs)=>{try{return await acquireQueuedResourceLease({directory,resource:resourceOf(kind,identity),mode:"exclusive",owner:randomBytes(16).toString("base64url"),signal:invocation.control.signal,control:phaseControl,timeoutMs});}catch(error){if(timedOut(error))return undefined;throw error;}},
      waitMilliseconds:GENERATION_POLICY_V1.sweepWaitMilliseconds})},{cancelled:()=>invocation.control.signal.aborted,progress:()=>{}},{minimumBlocks:24576,blocksPerHeader:6});
    if(report.removed)say("swept",{removed:report.removed,megabytes:Math.round(report.bytes/1048576),headers:report.headers});
  }catch(error){if(invocation.control.signal.aborted)throw error;process.stderr.write("graph: sweep of superseded observation blocks failed: "+String((error as Error).message)+"\n");}
}

/** 🔍️ Names the first difference between a generation graph and the graph a full inference produced; the diagnostic oracle of `SEMIO_BUILD_GRAPH_VERIFY=1`. */
function requireEqualBuildGraphsV1(actual:BuildGraphV1,expected:BuildGraphV1):void{
  const same=(left:unknown,right:unknown)=>canonicalGenerationJsonV1(left)===canonicalGenerationJsonV1(right);
  for(const name of new Set([...Object.keys(actual.projects),...Object.keys(expected.projects)]))if(!same(actual.projects[name],expected.projects[name]))throw Error(`Graph generation differs from full inference at project ${name}`);
  const edges=(graph:BuildGraphV1)=>graph.dependencies.map(edge=>canonicalGenerationJsonV1(edge)).sort();
  if(!same(edges(actual),edges(expected)))throw Error("Graph generation differs from full inference at dependencies");
  for(const key of["externalNodes","unavailable","namedInputs"] as const)if(!same(actual[key]??null,expected[key]??null))throw Error(`Graph generation differs from full inference at ${key}`);
}

/** 🧩️ The stale projects of a pending refresh: the generation they were planned against, their names, their roots and the implementation digest of the planner. */
type GenerationPrefetchRequestV1={readonly generation:number;readonly dirty:readonly string[];readonly roots:Readonly<Record<string,string>>;readonly implementation:string};

/** 🧩️ Fills the project records of the owners whose roots are in `roots` without assembling anything: the inference the build runs for them with the options it runs it with (so the records are the ones the build finds warm), without the test and policy contributions, the edges and the environment. */
export async function prefetchBuildGraphProjectsV1(root:string,registry:BuildOwnerRegistryV1,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,files:readonly string[],roots:ReadonlySet<string>):Promise<number> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);registry=readBuildOwnerRegistryV1(registry);
  requireBuildOwnerRegistryAuthorityV1(registry,JSON.parse(pinnedAuthorityReadV1(join(root,"📋️build-owners.json"),()=>observationV1.observedFs.readFileSync(join(root,"📋️build-owners.json"),"utf8"))));
  const owner=await import("./📥️inference/🟨️.mjs");checkResourceLeaseControl(phaseControl);requireCurrentGraphModuleV1();
  const context={workspaceRoot:root,invocation,phaseControl,registry};
  const captured=await observationV1.captureObservedSourceInputsV1(async()=>{
    const options={analyzeLockfile:false,catalogProviders:registry.catalogProviders.map(({ownerRoot,sourcePath})=>({ownerRoot,sourcePath})),snapshot:true,ownerRoots:roots,onOwnerObservations:()=>{}};
    return (await owner.discoverBuildProjectsV1(files.filter(file=>file.endsWith("📋️project.json")||file.endsWith("Cargo.toml")||javascriptManagerV1.lockFileNames().includes(file)||file.endsWith(".patch")),options,context)).length;
  },undefined,false);
  return captured.value;
}

/** 🤝️ Prepares the stale projects of a pending refresh together with the commands that wait for the same graph, before the exclusive build: claims a chunk of them, fills their project records in chunks that release the heap in between, joins what a peer computes, and never fails the command (the build computes what is left and reports a real failure). */
async function prefetchStaleGenerationV1(input:{root:string;invocation:ScriptInvocation;phaseControl:ResourceLeaseControl;registry:BuildOwnerRegistryV1;files:()=>readonly string[];request:GenerationPrefetchRequestV1;release:()=>Promise<void>;language:"en"|"de"}):Promise<void> {
  const{root,invocation,phaseControl,registry,request}=input;
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);requireCurrentGraphModuleV1();
  const say=(key:ClaimMessageKeyV1,fields:Record<string,string|number>)=>{process.stderr.write(renderClaimMessageV1(key,fields,input.language)+"\n");};
  try{await withRepositoryBuildGraphSelectionV1(root,invocation,phaseControl,generationScopeV1,async(store,control)=>{
    const files=generationFilesV1(store),directory=join(store.root,"generations"),clock=createGraphPhaseClockV1({publish:resource=>control.publish({resource,mode:"shared",elapsedMs:0}),write:writeGraphPhaseRowV1});
    const claims=createClaimRegistryV1({root:directory,fs:files as never,lease:createQueuedClaimLeasePortV1({acquire:acquireQueuedResourceLease as never,directory:join(root,".cache/b/l/m/data"),control})});
    let begun=false;
    try{
      await prepareStaleProjectsV1({registry:claims,base:request.generation,implementation:request.implementation,wanted:request.dirty,slots:slotCountV1({memoryMegabytes:Math.floor(totalmem()/1048576),environment:process.env}),signal:invocation.control.signal,say,
        served:async()=>{const bytes=await files.read(join(directory,GENERATION_POLICY_V1.pointer)) as Uint8Array|undefined,pointer=bytes?readGenerationPointerV1(bytes):undefined;return pointer&&pointer.generation!==request.generation?pointer.generation:undefined;},
        progress:async({done,total})=>{if(!begun){begun=true;await clock.begin("prefetch",total);}await clock.advance("prefetch",done);},
        run:async owners=>{
          checkScriptInvocation(invocation);
          const roots=new Set(owners.map(name=>request.roots[name]).filter((value):value is string=>value!==undefined));
          try{if(roots.size)await prefetchBuildGraphProjectsV1(root,registry,invocation,control,input.files(),roots);}finally{await input.release();}
        }});
    }finally{if(begun)await clock.end("prefetch").catch(()=>undefined);}
  },{optionalCaches:false,shared:true});}
  catch(error){if(invocation.control.signal.aborted)throw error;say("failed",{reason:String((error as Error)?.message??error).replace(/\s+/gu," ").slice(0,200)});}
}

/** 🧩️ Infers only the projects whose roots are in `part.roots` (all of them when cold) plus the test and policy contributions, recomputes the edges of every other owner when the environment moved, and returns the pieces owner by owner. Everything an owner consumed is recorded in its own project record or in the capture around this call, so a later edit finds exactly its consumers. */
export async function discoverBuildGraphPartV1(root:string,registry:BuildOwnerRegistryV1,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,files:readonly string[],part:{cold:boolean;roots:ReadonlySet<string>;stubs:Record<string,BuildGraphPartStubV1>;previousEnv:string|null;changed?:readonly string[]}):Promise<BuildGraphPartV1> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);registry=readBuildOwnerRegistryV1(registry);
  requireBuildOwnerRegistryAuthorityV1(registry,JSON.parse(pinnedAuthorityReadV1(join(root,"📋️build-owners.json"),()=>observationV1.observedFs.readFileSync(join(root,"📋️build-owners.json"),"utf8"))));
  const [owner,tests,policies]=await Promise.all([import("./📥️inference/🟨️.mjs"),import("../../../🧪️test/📦️packages/🟦️typescript/🟨️.mjs"),import("./📥️inference/🛡️policy/🟨️.mjs")]);checkResourceLeaseControl(phaseControl);requireCurrentGraphModuleV1();
  const nodeRows:Record<string,BuildGraphRowV1[]>={},derivedFacts=new Map<string,{row:BuildGraphRowV1;support:BuildGraphRowV1[]}>(),context={workspaceRoot:root,invocation,phaseControl,registry},stubs=Object.entries(part.stubs),realRoots=new Set(stubs.filter(([,stub])=>!stub.tag).map(([,stub])=>stub.root));
  const read=async(path:string)=>{try{return await readFile(join(root,path),"utf8");}catch(error){if(["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))return undefined;throw error;}},stat=async(path:string)=>{try{const status=await lstat(join(root,path));return status.isFile()?{size:status.size,mtimeMs:status.mtimeMs}:undefined;}catch(error){if(["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))return undefined;throw error;}};
  const captured=await observationV1.captureObservedSourceInputsV1(async()=>{
    const options={analyzeLockfile:true,catalogProviders:registry.catalogProviders.map(({ownerRoot,sourcePath})=>({ownerRoot,sourcePath})),snapshot:true,...(part.cold?{}:{ownerRoots:part.roots}),onOwnerObservations:(name:string,rows:BuildGraphRowV1[],derived?:{row:BuildGraphRowV1;support:BuildGraphRowV1[]}[])=>{nodeRows[name]=rows;for(const fact of derived??[])derivedFacts.set(fact.row.path+"\0"+fact.row.fact,fact);}};
    const primary=await owner.discoverBuildProjectsV1(files.filter(file=>file.endsWith("📋️project.json")||file.endsWith("Cargo.toml")||javascriptManagerV1.lockFileNames().includes(file)||file.endsWith(".patch")),options,context);
    if(invocation)await advanceScriptInvocation(invocation,"build:infer","running");
    const projects:Record<string,any>={},externalNodes:Record<string,unknown>={},unavailable=new Map<string,BuildUnavailableProjectV1>(),tags:Record<string,string>={},configs:Record<string,string>={},primaryNames=new Set<string>(),testNames=new Set<string>();
    const merge=(list:[string,any][],primaryList:boolean)=>list.forEach(([file,contribution])=>{
      Object.assign(externalNodes,contribution.externalNodes);
      for(const row of (contribution.unavailable??[]) as BuildUnavailableProjectV1[])unavailable.set(row.root,row);
      for(const[name,project]of Object.entries(contribution.projects??{}) as [string,any][]){
        if(projects[name]&&projects[name].root!==project.root)throw Error(`Duplicate build project ${name}`);
        projects[name]={...projects[name],...project,name,targets:{...projects[name]?.targets,...project.targets}};
        if(primaryList)primaryNames.add(name);
        configs[name]??=file;
      }
    });
    merge(primary as [string,any][],true);
    const kept=stubs.filter(([name,stub])=>!stub.tag&&stub.kind==="project"&&!projects[name]&&!part.roots.has(stub.root));
    const listed=[...kept.map(([name,stub])=>({name,root:stub.root,nativeRoot:stub.nativeRoot,config:stub.config})),...Object.entries(projects).map(([name,project])=>({name,root:project.root,nativeRoot:project.metadata?.nativeRoot as string|undefined,config:configs[name]}))];
    const env=await deriveEnvironmentV1({projects:listed,workspaceManifests:files.filter(file=>file==="Cargo.toml"||file.endsWith("/Cargo.toml")),lock:javascriptManagerV1.primaryLockName(),taxonomy:"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",read,stat});
    const tagged=stubs.filter(([,stub])=>stub.tag),retained=taggedOwnersRetainedV1({cold:part.cold,changed:part.changed,roots:part.roots,stubs:part.stubs,environment:env.digest,previousEnvironment:part.previousEnv});
    const tagRows:BuildGraphRowV1[]=[],discoveredForTags=async<T>(run:()=>Promise<T>|T):Promise<T>=>{const taken=await observationV1.captureObservedSourceInputsV1(async()=>run(),undefined,false);tagRows.push(...(taken.observations as BuildGraphRowV1[]));return taken.value;};
    const testContributions=retained?[]:await discoveredForTags(()=>tests.discoverTestBuildProjectsV1(files.filter(file=>file.endsWith(".feature")),{},context));
    for(const[,contribution]of testContributions)for(const name of Object.keys(contribution.projects??{}))testNames.add(name);
    const policyOwners=Object.assign({},...[...primary,...testContributions].map(([,contribution])=>contribution.projects??{}));
    const policyContributions=await discoveredForTags(()=>policies.default.createNodesV2[1](files.filter(file=>file.endsWith("📜️script.ts")&&(part.cold||part.roots.has(dirname(file))||!retained&&!realRoots.has(dirname(file)))),{}, {workspaceRoot:root,projects:policyOwners}));
    merge(testContributions as [string,any][],false);merge(policyContributions as [string,any][],false);
    for(const name of Object.keys(projects))if(!primaryNames.has(name))tags[name]=testNames.has(name)?TEST_TAG_V1:POLICY_TAG_V1;
    if(!retained)await discoveredForTags(()=>withholdDanglingTestProjectsV1({projects,configs,unavailable,dangling:tests.danglingTestDependenciesV1({...Object.fromEntries(kept.map(([name,stub])=>[name,{root:stub.root}])),...projects}),digest:file=>observedFileDigestV1(root,file)}));
    const edgeNames=!part.cold&&env.digest!==part.previousEnv?kept.map(([name])=>name):[],processed=new Set([...Object.keys(projects),...edgeNames]);
    const contextProjects:Record<string,any>={...Object.fromEntries([...kept,...(retained?tagged.filter(([,stub])=>stub.kind==="project"):[])].map(([name,stub])=>[name,{name,root:stub.root,targets:Object.fromEntries(stub.targets.map(target=>[target,{}])),...(stub.nativeRoot?{metadata:{nativeRoot:stub.nativeRoot}}:{})}])),...projects};
    const ownerOf=createProjectFileIndexV1(contextProjects),withheld=[...unavailable.keys(),...stubs.filter(([,stub])=>(!stub.tag||retained)&&stub.kind==="unavailable"&&!part.roots.has(stub.root)).map(([,stub])=>stub.root)],projectFileMap:Record<string,{file:string}[]>=Object.fromEntries([...processed].map(name=>[name,[]]));
    for(const file of files){const found=ownerOf(file);if(found!==undefined&&projectFileMap[found]&&!withheld.some(withheldRoot=>file.startsWith(withheldRoot+"/")&&withheldRoot.length>contextProjects[found].root.length))projectFileMap[found]!.push({file});}
    const graphContext={workspaceRoot:root,projects:contextProjects,fileMap:{projectFileMap},invocation,phaseControl,selection:{kind:"all"},registry,completeSourceImports:false};
    const testEdges:unknown=retained?[]:await discoveredForTags(()=>tests.discoverTestBuildDependenciesV1({},graphContext));if(!Array.isArray(testEdges))throw Error("Test dependency DATA array required");
    const edges=[...await owner.discoverBuildDependenciesV1(options,graphContext),...testEdges];
    const metadata=await owner.createMetadata({nodes:Object.fromEntries(Object.entries(contextProjects).map(([name,data])=>[name,{name,type:"lib",data}]))},options,graphContext);
    if(!metadata||typeof metadata!=="object"||Array.isArray(metadata))throw Error("Graph metadata DATA object required");
    for(const[name,value]of Object.entries(metadata)){if(!projects[name])continue;if(!value||typeof value!=="object"||!("metadata"in value)||!value.metadata||typeof value.metadata!=="object"||Array.isArray(value.metadata))throw Error("Project metadata DATA object required");projects[name].metadata={...projects[name].metadata,...value.metadata,semio:{...projects[name].metadata?.semio,...("semio"in value.metadata&&value.metadata.semio&&typeof value.metadata.semio==="object"?value.metadata.semio:{})}};}
    const neutral=Object.fromEntries(Object.entries(projects).map(([name,project])=>[name,{...project,targets:Object.fromEntries(Object.entries(project.targets??{}).map(([target,value])=>[target,neutralBuildTargetV1(value as Record<string,any>,`${name}:${target}`)]))}]));
    if(invocation)await advanceScriptInvocation(invocation,"build:graph","complete");
    const rows=[...unavailable.values()];
    return {
      owners:[...Object.entries(neutral).map(([name,node]:[string,any])=>({name,root:node.root as string,nativeRoot:node.metadata?.nativeRoot as string|undefined,config:configs[name],...(tags[name]?{tag:tags[name]}:{}),node,nodeRows:tags[name]?tagRows:nodeRows[name]})),...rows.filter(row=>row.project!==undefined).map(row=>({name:row.project!,root:row.root,config:row.source,...(tags[row.project!]?{tag:tags[row.project!]}:{}),unavailable:row,nodeRows:tags[row.project!]?tagRows:nodeRows[row.project!]}))],
      edgeOnly:edgeNames.map(name=>({name})),derived:[...derivedFacts.values()],edges:edges as BuildGraphPartV1["edges"],externalNodes,parseUnavailable:rows.filter(row=>row.project===undefined),env,replaceTags:retained?[]:["tests","policy",TEST_TAG_V1,POLICY_TAG_V1],
    };
  },undefined,false);
  return {...captured.value,rows:[...captured.observations] as BuildGraphRowV1[]};
}

/** 🧺️ Keeps exactly the discovered files whose presence the portable graph consumes, as declared by the build policy. */
export function buildGraphMembersV1(files:readonly string[]):string[] {
  const {basenames,suffixes,lockEcosystems}=graphDeclarationV1("../🔣️policy.json").graphMembers as {basenames:string[];suffixes:string[];lockEcosystems:string[]},names=new Set([...basenames,...lockEcosystems.flatMap(ecosystem=>selectPackageManagerV1(ecosystem).lockFileNames())]);
  return files.filter(file=>names.has(file.slice(file.lastIndexOf("/")+1))||suffixes.some(suffix=>file.endsWith(suffix)));
}

/** 🧾️ Identifies complete admitted physical membership, types and modes independently of implementation bytes. */
export async function buildGraphMembershipV1(root:string,files:string[],invocation?:ScriptInvocation):Promise<string> {
  const records:string[]=[],entries=[...files.map(file=>({file,directory:false})),...(inventoryDirectoriesV1.get(files)??[]).map(file=>({file,directory:true}))].filter(({file})=>![".nxignore","nx.json"].includes(file)).sort((a,b)=>a.file.localeCompare(b.file));
  for(let index=0;index<entries.length;index+=64){
    if(invocation)checkScriptInvocation(invocation);
    records.push(...await Promise.all(entries.slice(index,index+64).map(async({file,directory})=>{let info;try{info=await lstat(join(root,file));}catch(error){if(["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))throw Object.assign(Error(`Build member removed during discovery: ${file}`),{code:"BUILD_MEMBERSHIP_CHANGED"});throw error;}if(info.isSymbolicLink()||(directory?!info.isDirectory():!info.isFile()))throw Object.assign(Error(`Build membership changed during discovery: ${file}`),{code:"BUILD_MEMBERSHIP_CHANGED"});return JSON.stringify([file,info.isDirectory()?"directory":"file",info.mode]);})));
  }
  return createHash("sha256").update(records.join("\n")).digest("hex");
}

/** 🔏️ Admits content-aware observations through the same owned inference mechanism. */
export async function buildGraphSourceFactsHoldV1(observations:readonly unknown[],invocation?:ScriptInvocation,onStale?:(row:{kind:string;path:string;fact:unknown})=>void):Promise<boolean>{return observationV1.observationsHold(observations,64,new Set(),new Set(),invocation?.control.signal,onStale);}

type OwnerRegistryInvalidationV1={readonly kind:"invalidated";readonly reason:"owner-registry"};
const ownerRegistryInvalidationV1:OwnerRegistryInvalidationV1=Object.freeze({kind:"invalidated",reason:"owner-registry"});
const buildGraphErrorCodeV1=(error:unknown):unknown=>error!==null&&typeof error==="object"&&"code" in error?error.code:undefined;
const isOwnerRegistryInvalidationV1=(value:unknown):value is OwnerRegistryInvalidationV1=>value===ownerRegistryInvalidationV1;

/** 📌️ Reads immutable snapshot bytes while the existing original Graph store and exact content reader lease remain held. */
async function withRetainedGraphSnapshotV1<T>(workspace:string,store:RepositoryCacheStoreV1,path:string,control:ResourceLeaseControl,run:(source:string)=>Promise<T>):Promise<T>{
 const policy=graphSnapshotPolicyV1,lease=await acquireQueuedResourceLease({directory:join(workspace,".cache/b/l/m/data"),resource:"build-graph:"+path,mode:"shared",owner:randomBytes(16).toString("base64url"),control,signal:control.signal});let failed=false;
 try{await store.recheck();checkResourceLeaseControl(control);const directory=dirname(path),parent=await lstat(directory,{bigint:true}),before=await lstat(path,{bigint:true});checkResourceLeaseControl(control);if(!parent.isDirectory()||parent.isSymbolicLink()||!before.isFile()||before.isSymbolicLink()||before.nlink!==1n||before.size<0n||before.size>BigInt(policy.maximumSnapshotBytes))throw Error("Original bounded Graph snapshot required");
 return await withArtifactDescriptorV1(()=>open(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0)|(constants.O_NONBLOCK??0)),async file=>{const verify=async()=>{await store.recheck();checkResourceLeaseControl(control);requireNativeStoragePathV1(store.root,path);const linked=await lstat(path,{bigint:true}),opened=await file.stat({bigint:true}),actualParent=await lstat(directory,{bigint:true});checkResourceLeaseControl(control);if(!actualParent.isDirectory()||actualParent.isSymbolicLink()||actualParent.dev!==parent.dev||actualParent.ino!==parent.ino||[linked,opened].some(row=>!row.isFile()||row.isSymbolicLink()||["dev","ino","size","nlink","mtimeNs"].some(key=>row[key as keyof typeof row]!==before[key as keyof typeof before])))throw Error("Retained Graph snapshot descriptor changed");};await verify();const bytes=Buffer.alloc(Number(before.size));try{let offset=0;while(offset<bytes.length){await verify();const row=await file.read(bytes,offset,Math.min(65536,bytes.length-offset),offset);checkResourceLeaseControl(control);if(!row.bytesRead)throw Error("Retained Graph snapshot shortened");offset+=row.bytesRead;await control.yieldContinuation();}if((await file.read(Buffer.alloc(1),0,1,bytes.length)).bytesRead)throw Error("Retained Graph snapshot grew");await verify();const value=await run(new TextDecoder("utf-8",{fatal:true}).decode(bytes));await verify();return value;}finally{bytes.fill(0);}});
 }catch(error){failed=true;throw error;}finally{try{lease.release();}catch(error){if(!failed)throw error;}}
}

/** 🏪️ Delivers the pinned reusable graph only through its existing private Store consumer and retained exact content descriptor. */
export function withRepositoryBuildGraphPublicationV1<T>(root:string,control:ResourceLeaseControl,body:(graph:BuildGraphV1,digest:string,actual:ResourceLeaseControl)=>Promise<T>):Promise<T>{
 requireCurrentGraphModuleV1();const pin={path:process.env.SEMIO_BUILD_GRAPH_PATH,digest:process.env.SEMIO_BUILD_GRAPH_DIGEST};if(validateJsonSchemaSubset(graphDeclarationV1("../🧬️schema/📥️pin.json"),pin).length||!isAbsolute(pin.path!))throw Error("Original immutable build graph path and digest required");
 return withRepositoryBuildGraphConsumerV1(root,control,async(store,actual)=>{await store.recheck();checkResourceLeaseControl(actual);const expected=requireNativeStoragePathV1(store.root,join(store.root,"snapshots",pin.digest+".json"));if(pin.path!==expected)throw Error("Original retained Graph snapshot identity required");return withRetainedGraphSnapshotV1(root,store,expected,actual,async source=>{if(createHash("sha256").update(source).digest("hex")!==pin.digest)throw Error("Pinned build graph digest differs from original authority");let reads=0;const records=dirname(expected),graph=await decodeGraphRecordsV1(Buffer.from(source),async digest=>{if(++reads%64===0)await store.recheck();checkResourceLeaseControl(actual);const path=requireNativeStoragePathV1(store.root,join(records,digest+".gz")),status=await lstat(path);if(!status.isFile()||status.isSymbolicLink()||status.nlink!==1||status.size>graphSnapshotPolicyV1.maximumSnapshotBytes)throw Error("Original bounded Graph record required");return readFile(path);},{policy:graphSnapshotPolicyV1.records as never}) as BuildGraphV1;requireCurrentGraphModuleV1();checkResourceLeaseControl(actual);const value=await body(graph,pin.digest!,actual);await store.recheck();checkResourceLeaseControl(actual);requireCurrentGraphModuleV1();return value;});});
}

/** 🎬️ Resolves one candidate while keeping every candidate's memory within its own finite attempt. */
async function loadBuildGraphAttemptV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:{selection:BuildGraphSelectionV1;reuse?:boolean;sealedOnly?:boolean},registry?:BuildOwnerRegistryV1,scope?:string):Promise<BuildGraphV1|OwnerRegistryInvalidationV1|undefined> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);requireCurrentGraphModuleV1();
  const invalidated=(reason:string,details:Record<string,unknown>={})=>{if(process.env.SEMIO_BUILD_TRACE_AUTHORITY==="true"){let diagnostic;try{diagnostic=buildGraphRetryDiagnosticV1(root,reason,details,path=>captureBoundedSourceRevisionV1(path,8388608).sha256);}catch{diagnostic=buildGraphRetryDiagnosticV1(root,reason);}process.stderr.write("[graph-retry] "+JSON.stringify(diagnostic)+"\n");}return undefined;};
  const phaseRows:{phase:string;milliseconds:number}[]=[],timing=(phase:string,started:number,details:Record<string,unknown>={})=>{const row={phase,milliseconds:Math.round(performance.now()-started),...details};phaseRows.push(row);if(process.env.SEMIO_BUILD_PROFILE_INPUTS==="1"||process.env.SEMIO_GRAPH_PHASES==="1")process.stderr.write("[graph-phase] "+JSON.stringify(row)+"\n");};
  const image=async(source:string)=>{let started=performance.now();const decoded=JSON.parse(source);timing("graph-json-parse",started,{bytes:Buffer.byteLength(source)});started=performance.now();const expanded=await portableInputs(root,decoded,invocation,phaseControl);timing("graph-input-expand",started);started=performance.now();const graph=readBuildGraphV1(expanded);timing("graph-schema-validation",started,{projects:Object.keys(graph.projects).length,dependencies:graph.dependencies.length});return graph;};
  if(!registry)throw Error("Original canonical graph owner registry required");
  const selected=options.selection.kind!=="all",path=scope?join(dirname(buildGraphPathV1(root)),"selected",scope+".json"):buildGraphPathV1(root),authorityPath=scope?join(dirname(path),scope+".authority.json"):join(dirname(path),graphDeclarationV1("../🧬️schema/📣️publication.json").properties.authority.const);
  if(options.sealedOnly){if(selected)return undefined;try{if(!buildGraphProducerAuthorityV1(JSON.parse(await readFile(authorityPath,"utf8")),loadedGraphModuleAuthorityV1))return undefined;}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT"||error instanceof SyntaxError)return undefined;throw error;}}
  const observation=observationV1,[producer]=await Promise.all([import("./📥️inference/🟨️.mjs"),import("../../../🧪️test/📦️packages/🟦️typescript/🟨️.mjs"),import("./📥️inference/🛡️policy/🟨️.mjs")]);checkResourceLeaseControl(phaseControl);requireCurrentGraphModuleV1();
  const inventory=async()=>{let started=performance.now();const captured=selected?await observation.captureObservedSourceInputsV1(()=>observation.settleObservedComputationV1(async()=>{const current=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root)));if(buildGraphScopeKeyV1(options.selection,current)!==scope)return ownerRegistryInvalidationV1;const nativeCache=new Map(),nativeClosures=new Map();return closeBuildOwnerAdmissionV1(options.selection,registry!,async(owner,targets,inputs)=>{if(invocation)checkScriptInvocation(invocation);return producer.discoverBuildOwnerAdmissionFactsV1(owner,targets,registry,{workspaceRoot:root,invocation,phaseControl,selection:options.selection,nativeCache,nativeClosures},inputs);});},{signal:invocation?.control.signal})):undefined,candidate=captured?.value;if(isOwnerRegistryInvalidationV1(candidate))return candidate;const admission=candidate as BuildOwnerAdmissionV1|undefined,files=admission?.files??buildGraphMembersV1(await workspaceBuildFilesV1(root,invocation));timing("workspace-catalog-lookup",started,{files:files.length,directories:inventoryDirectoriesV1.get(files)?.length??0});started=performance.now();const digest=await buildGraphMembershipV1(root,files,invocation);timing("workspace-membership-hash",started);return {files,digest,admission,observations:captured?.observations??[]};};
  let initial:Awaited<ReturnType<typeof inventory>>;try{initial=await inventory();}catch(error){if(buildGraphErrorCodeV1(error)==="BUILD_MEMBERSHIP_CHANGED")return invalidated("membership");throw error;}
  if(isOwnerRegistryInvalidationV1(initial)){invalidated("owner-registry");return initial;}
  const {files,digest:before}=initial;
  let phaseStarted=performance.now();
  await producer.libraryBootstrap;producer.prepareSourceAnalysesV1();timing("source-parser-bootstrap",phaseStarted);
  let cachedReceipt:any;try{cachedReceipt=JSON.parse(await readFile(authorityPath,"utf8"));}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT"&&!(error instanceof SyntaxError))throw error;}
  if(cachedReceipt)try{
    phaseStarted=performance.now();timing("source-authority-read-parse",phaseStarted,{observations:cachedReceipt.observations?.length??0});phaseStarted=performance.now();
    const cachedReceiptValid=buildGraphProducerAuthorityV1(cachedReceipt,loadedGraphModuleAuthorityV1)&&cachedReceipt.membership===before;timing("source-authority-schema",phaseStarted,{admitted:cachedReceiptValid});phaseStarted=performance.now();const factsHold=cachedReceiptValid&&await buildGraphSourceFactsHoldV1(cachedReceipt.observations,invocation);timing("source-authority-facts",phaseStarted,{holds:factsHold});
    if(factsHold){requireCurrentGraphModuleV1();
      phaseStarted=performance.now();const source=await readFile(path,"utf8");timing("graph-file-read",phaseStarted,{bytes:Buffer.byteLength(source)});phaseStarted=performance.now();const digest=createHash("sha256").update(source).digest("hex");timing("graph-content-hash",phaseStarted);if(digest===cachedReceipt.graph){const graph=await image(source);requireCurrentGraphModuleV1();if(selected&&JSON.stringify(graph.admission?.selection)!==JSON.stringify(options.selection))throw Error("Original selected publication scope differs");if(await buildGraphSourceFactsHoldV1([...initial.observations,...cachedReceipt.observations],invocation))return graph;return undefined;}
    }
  }catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
  if(options.sealedOnly)return undefined;
  let captured:{value:BuildGraphV1|OwnerRegistryInvalidationV1;observations:readonly unknown[]},failedFacts:unknown[]=[],failedAuthority:unknown[]=[];
  phaseStarted=performance.now();
  try{captured=await observation.captureObservedSourceInputsV1(async()=>{
    for(const source of [new URL(physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../🧬️schema/📣️publication.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../🧬️schema/📥️authority.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../🔣️policy.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../⚡️caching/🔣️policy.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../🗂️workspaces/🔣️policy.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../📤️arguments/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../../../../../../🔨️modules/🏃️process/🏗️build/🎚️mode/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL))])observation.observedFs.readFileSync(source);
    const admittedRegistry=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root)));if(JSON.stringify(admittedRegistry)!==JSON.stringify(registry))return ownerRegistryInvalidationV1;
    for(const source of [new URL("../🧬️schema/🚪️selection.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../🧬️schema/📋️owners.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("../🧬️schema/🚪️admission.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),new URL("./🚪️admission/🟦️.ts",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL))])observation.observedFs.readFileSync(source);
    return discoverBuildGraphV1(root,options.selection,registry!,invocation,phaseControl,files,initial.admission,true);
  },(rows:unknown[],authority:unknown[])=>{failedFacts=rows;failedAuthority=authority;});}
  catch(error){if(invocation)checkScriptInvocation(invocation);if(buildGraphErrorCodeV1(error)==="BUILD_AUTHORITY_CHANGED")throw error;let current;try{current=await inventory();}catch(recheck){if(buildGraphErrorCodeV1(recheck)==="BUILD_MEMBERSHIP_CHANGED")return invalidated("membership",{before});throw recheck;}if(isOwnerRegistryInvalidationV1(current))throw error;const stale:{kind:string;path:string;fact:unknown}[]=[],holds=await buildGraphSourceFactsHoldV1(failedFacts,invocation,row=>{if(stale.length<8)stale.push(row);});if(current.digest===before&&holds)throw error;return invalidated("producer-error",{before,after:current.digest,stale});}
  if(isOwnerRegistryInvalidationV1(captured.value)){invalidated("owner-registry",{before});return captured.value;}
  timing("graph-original-inference",phaseStarted,{observations:captured.observations.length});
  let final:Awaited<ReturnType<typeof inventory>>;try{final=await inventory();}catch(error){if(buildGraphErrorCodeV1(error)==="BUILD_MEMBERSHIP_CHANGED")return invalidated("membership",{before});throw error;}
  if(isOwnerRegistryInvalidationV1(final)){invalidated("owner-registry",{before});return final;}
  const after=final.digest;
  captured.observations=[...initial.observations,...captured.observations];
  const stale:{kind:string;path:string;fact:unknown}[]=[],holds=await buildGraphSourceFactsHoldV1(captured.observations,invocation,row=>{if(stale.length<8)stale.push(row);});
  if(selected&&JSON.stringify(initial.admission)!==JSON.stringify(final.admission))return invalidated("admission",{before:createHash("sha256").update(JSON.stringify(initial.admission)).digest("hex"),after:createHash("sha256").update(JSON.stringify(final.admission)).digest("hex"),stale});
  if(before!==after||!holds)return invalidated(before!==after?"membership":"observed-facts",{before,after,stale});
  const publishStarted=performance.now(),compact=captured.value as BuildGraphV1,source=JSON.stringify(compact),digest=createHash("sha256").update(source).digest("hex"),authority=JSON.stringify({version:3,module:loadedGraphModuleAuthorityV1,membership:after,graph:digest,observations:captured.observations});timing("graph-serialize",publishStarted,{graphBytes:Buffer.byteLength(source),authorityBytes:Buffer.byteLength(authority)});if(Buffer.byteLength(source)>536870912||Buffer.byteLength(authority)>536870912)throw Error("Build graph cache exceeds its finite publication limit: "+Buffer.byteLength(source)+" graph bytes, "+Buffer.byteLength(authority)+" authority bytes, 536870912 allowed");await mkdir(dirname(path),{recursive:true});const expandStarted=performance.now(),graph=readBuildGraphV1(await portableInputs(root,compact,invocation,phaseControl));timing("graph-expansion",expandStarted);
  const temporary=path+"."+randomUUID(),receipt=authorityPath+"."+randomUUID();try{await writeFile(temporary,source,{flag:"wx"});await writeFile(receipt,authority,{flag:"wx"});if(invocation)checkScriptInvocation(invocation);requireCurrentGraphModuleV1();if(!await buildGraphSourceFactsHoldV1(captured.observations,invocation,row=>{if(stale.length<8)stale.push(row);}))return invalidated("publication-facts",{before,after,stale});timing("graph-publication",publishStarted);process.stderr.write("[graph-phase] "+JSON.stringify({summary:phaseRows.map(row=>[row.phase,row.milliseconds])})+"\n");await rename(temporary,path);await rename(receipt,authorityPath);}finally{await Promise.all([rm(temporary,{force:true}),rm(receipt,{force:true})]);}
  return graph;
}
/** 🔓️ Serves a current sealed graph to any number of shared readers: only a cache-store reader is held, the graph writer and its exclusive lease are never requested, so readers never queue behind a builder that is still inferring. */
async function readSealedBuildGraphV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:{selection:BuildGraphSelectionV1;reuse?:boolean},registry:BuildOwnerRegistryV1):Promise<BuildGraphV1|OwnerRegistryInvalidationV1|undefined> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);
  return withRepositoryCacheStoreV1(root,"graph",invocation,phaseControl,async(store,actualControl)=>{await store.recheck();checkResourceLeaseControl(actualControl);return loadBuildGraphAttemptV1(root,invocation,actualControl,{...options,sealedOnly:true},registry,undefined);});
}

/** 💾️ Publishes only a current graph and releases each invalidated candidate before retrying. */
export async function loadBuildGraphV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:BuildGraphLoadOptionsV1):Promise<BuildGraphV1> {
  if(options.selection?.kind&&!options.baseline&&!options.reuse&&invocation){
    clearPinnedAuthorityV1();
    if(options.selection.kind==="all")return withImplementationPinV1(()=>loadBuildGraphUnpinnedV1(root,invocation,phaseControl,options));
    return withImplementationPinV1(async()=>{
      const sliced=await loadBuildGraphSliceV1(root,invocation,phaseControl,options);
      if(process.env.SEMIO_BUILD_GRAPH_VERIFY==="1")requireSelectedSubgraphV1(sliced,await loadBuildGraphUnpinnedV1(root,invocation,phaseControl,options));
      return sliced;
    });
  }
  return loadBuildGraphUnpinnedV1(root,invocation,phaseControl,options);
}

/** 🍰️ Derives the graph of a selected scope (tasks or literal owners) from the newest sealed generation: the closure of the selected projects is read, verified and, under a fresh policy, refreshed incrementally. The generation is the only source of a graph: when none is usable it is built first, and a selected project the generation does not know refuses. */
async function loadBuildGraphSliceV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:BuildGraphLoadOptionsV1):Promise<BuildGraphV1> {
  const producer=await import("./📥️inference/🟨️.mjs"),registry=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root))),selection=readBuildGraphSelectionV1(options.selection);
  if(selection.kind==="all")throw Error("Selected build graph scope required");
  const roots=selection.kind==="tasks"?[...new Set(selection.tasks.map(task=>task.project))]:selectLiteralBuildProjectsV1(registry.owners,selection.projects,selection.exclude);
  return await loadGenerationGraphV1(generationHostV1(root,invocation,phaseControl,registry,options.graph?.compact===true),{policy:options.graph?.policy??"fresh",selection,slice:{selection,roots,providers:registry.catalogProviders.map(({consumerRoots,sourcePath})=>({consumerRoots,sourcePath}))},pin:boundGenerationPinV1()}) as BuildGraphV1;
}

/** 🪶️ Admits the lean graph of `show projects|targets`: project and target names, unavailable projects, no inputs. */
function readBuildGraphLeanV1(value:unknown):BuildGraphV1 {
  const errors=validateJsonSchemaSubset(graphDeclarationV1("../🧬️schema/🪶️lean.json"),value);
  if(errors.length)throw Error(`Invalid lean build graph: ${JSON.stringify(errors.slice(0,4))}`);
  return value as BuildGraphV1;
}

/** 🔭️ Names the first difference between a generation slice and the selected graph a full selected inference produced, in both directions: every selected project is equal, the dependencies among the selected projects are the same set, and every selected external node is equal; the slice may hold more projects. The oracle of `SEMIO_BUILD_GRAPH_VERIFY=1` for selected scopes. */
function requireSelectedSubgraphV1(slice:BuildGraphV1,selected:BuildGraphV1):void{
  const same=(left:unknown,right:unknown)=>canonicalGenerationJsonV1(left)===canonicalGenerationJsonV1(right),chosen=new Set(Object.keys(selected.projects));
  for(const[name,project]of Object.entries(selected.projects)){if(!slice.projects[name])throw Error(`Graph slice lacks the selected project ${name}`);if(!same(slice.projects[name],project))throw Error(`Graph slice differs from the selected graph at project ${name}`);}
  const inside=(edge:{source:string;target:string})=>chosen.has(edge.source)&&(chosen.has(edge.target)||selected.externalNodes?.[edge.target]!==undefined||slice.externalNodes?.[edge.target]!==undefined);
  const mine=new Set(slice.dependencies.filter(inside).map(edge=>canonicalGenerationJsonV1(edge))),theirs=new Set(selected.dependencies.map(edge=>canonicalGenerationJsonV1(edge)));
  for(const edge of theirs)if(!mine.has(edge))throw Error("Graph slice lacks a selected dependency: "+edge);
  for(const edge of mine)if(!theirs.has(edge))throw Error("Graph slice adds a dependency the selected graph lacks: "+edge);
  for(const[key,node]of Object.entries(selected.externalNodes??{}))if(!same(slice.externalNodes?.[key],node))throw Error(`Graph slice differs from the selected graph at external node ${key}`);
}

type BuildGraphLoadOptionsV1={selection:BuildGraphSelectionV1;reuse?:boolean;baseline?:boolean;graph?:{policy?:"fresh"|"current";closure?:readonly string[];lean?:boolean;compact?:boolean}};

/** 🕸️ Loads the graph with the strict self-checks of the loaded implementation; the generation path runs pinned and completes with what it loaded. */
async function loadBuildGraphUnpinnedV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:BuildGraphLoadOptionsV1):Promise<BuildGraphV1> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);requireCurrentGraphModuleV1();
  options={...options,selection:readBuildGraphSelectionV1(options.selection)};
  if(options.reuse)return withRepositoryBuildGraphPublicationV1(root,phaseControl,async graph=>{checkScriptInvocation(invocation);if(!graph.admission||JSON.stringify(readBuildGraphSelectionV1(graph.admission.selection))!==JSON.stringify(readBuildGraphSelectionV1(options.selection)))throw Error("Original selected build graph admission required");return graph;});
  if(!invocation)throw Error("Original build graph producer invocation required");
  if(options.selection.kind==="all"&&!options.baseline){
    const producer=await import("./📥️inference/🟨️.mjs"),registry=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root)));
    const graph=await loadGenerationGraphV1(generationHostV1(root,invocation,phaseControl,registry),{policy:options.graph?.policy??"fresh",closure:options.graph?.closure,lean:options.graph?.lean,pin:boundGenerationPinV1()}) as BuildGraphV1;
    if(process.env.SEMIO_BUILD_GRAPH_VERIFY==="1"&&!options.graph?.closure&&!options.graph?.lean)requireEqualBuildGraphsV1(graph,await loadBuildGraphV1(root,invocation,phaseControl,{selection:options.selection,baseline:true}));
    checkScriptInvocation(invocation);return graph;
  }
  for(let attempt=0;attempt<=8;attempt++){
    checkScriptInvocation(invocation);checkResourceLeaseControl(phaseControl);requireCurrentGraphModuleV1();
    const producer=await import("./📥️inference/🟨️.mjs");checkResourceLeaseControl(phaseControl);requireCurrentGraphModuleV1();const registry=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root))),scope=options.selection.kind!=="all"?buildGraphScopeKeyV1(options.selection,registry):undefined;
    if(scope===undefined){const sealed=await readSealedBuildGraphV1(root,invocation,phaseControl,options,registry);if(sealed&&!isOwnerRegistryInvalidationV1(sealed)){checkScriptInvocation(invocation);return sealed;}}
    const graph=await withBuildGraphStateV1(root,invocation,phaseControl,async(store,actualControl)=>{
      const staging=scope?join(store.root,"selected"):store.root;await store.recheck();checkResourceLeaseControl(actualControl);requireNativeStoragePathV1(store.root,staging);await mkdir(staging,{recursive:true});checkResourceLeaseControl(actualControl);const selectedStage=scope?new RegExp("^"+scope+"(?:\\.authority)?\\.json\\.[a-f0-9-]{36}$","u"):undefined;
      const staged=await readdir(staging);checkResourceLeaseControl(actualControl);for(const name of staged){if(!(selectedStage?.test(name)??/^(?:canonical|authority)\.json\.[a-f0-9-]{36}$/u.test(name)))continue;await store.recheck();checkResourceLeaseControl(actualControl);const path=join(staging,name),status=await lstat(path);checkResourceLeaseControl(actualControl);if(!status.isFile()||status.isSymbolicLink()||status.nlink!==1||status.size>536870912)throw Error("Owned bounded graph staging file required");await rm(path);checkResourceLeaseControl(actualControl);}
      checkResourceLeaseControl(actualControl);await store.recheck();return loadBuildGraphAttemptV1(root,invocation,actualControl,options,registry,scope);
    },scope);
    checkScriptInvocation(invocation);checkResourceLeaseControl(phaseControl);requireCurrentGraphModuleV1();
    if(graph&&!isOwnerRegistryInvalidationV1(graph))return graph;
    await advanceScriptInvocation(invocation,"build:refresh","running");
  }
  throw Error("Build source authority keeps changing during inference");
}

/** 🎚️ Resolves one configuration with environment inherited from the declared execution. */
export function resolveBuildTargetV1(target:BuildTargetV1,configuration?:string):BuildTargetV1 {
  if(!configuration)return target;
  const configured=target.configurations?.[configuration];
  if(!configured)throw Error(`Unknown build configuration: ${configuration}`);
  if(Object.keys(configured).some(key=>key!=="execution")||!configured.execution)throw Error("Build configuration execution options required");
  const overrides=configured.execution as Partial<BuildExecutionV1>;requireBuildConfigurationV1(target.execution,overrides);
  return {...target,execution:{...target.execution,...overrides,env:{...target.execution.env,...overrides.env}}};
}


/** 🎯️ Expands owned named inputs while preserving explicit dependency and runtime declarations. */
export function resolveBuildInputsV1(graph:BuildGraphV1,project:string|BuildProjectV1,inputs:readonly unknown[]):unknown[] {
  const owner=typeof project==="string"?graph.projects[project]:project;
  if(!owner)throw Error(`Unknown build input owner: ${project}`);
  const named={...graph.namedInputs,...owner.namedInputs as Record<string,unknown[]>|undefined},active=new Set<string>();
  const expand=(values:readonly unknown[]):unknown[]=>values.flatMap(value=>{
    if(typeof value!=="string"||!Object.hasOwn(named,value))return [value];
    if(active.has(value))throw Error(`Build named input cycle: ${[...active,value].join(" -> ")}`);
    active.add(value);const result=expand(named[value]!);active.delete(value);return result;
  });
  return expand(inputs);
}
