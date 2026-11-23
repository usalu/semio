import {createHash} from "node:crypto";
import {lstatSync,readFileSync,readdirSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {componentStageTargetsV1,type ComponentStageTargetV1} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧩️component-stage/🟦️.ts";
import {readComponentReceiptV1} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🟦️.ts";
import modelSchema from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️model/🧬️schema/🔣️.json";
import {componentModelLayoutV1} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️model/📋️contract/🟦️.ts";
import {stripWasmDebugSectionsV1} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🪶️strip-debug/🟦️.ts";
import {fileSha256V1,stageKeyDigestV1,STAGE_CACHE_POLICY_V1,type StageEntryV1,type StageFileV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🗃️stage-cache/🟦️.ts";
import schema from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🗃️stage-cache/🧬️schema/🔣️.json";
import {validateJsonSchemaSubset} from "../../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

type Stage = "transpile"|"optimize"|"probe";
type Request = {stagedReceipt:string;wasmSha256:string;semanticDescriptorSha256:string};
const workspace=resolve(import.meta.dirname,"../../../../../../..");
const stages=join(workspace,".cache/b/r/m/components",STAGE_CACHE_POLICY_V1.warmRoot);
const sha=(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex");

/** 🧾️ Validates stage key custody and the closed output manifest. */
function stageEntries(stage:Stage,root=stages):readonly StageEntryV1[]{
  const directory=join(root,stage);
  return readdirSync(directory,{withFileTypes:true}).filter(item=>item.isDirectory()&&/^[0-9a-f]{64}$/u.test(item.name)).flatMap(item=>{
    const path=join(directory,item.name,STAGE_CACHE_POLICY_V1.entryFile);
    let entry:StageEntryV1;
    try{entry=JSON.parse(readFileSync(path,"utf8"));}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return [];throw error;}
    if(validateJsonSchemaSubset(schema.definitions.StageEntryV1,entry,schema).length||entry.digest!==item.name||entry.key.stage!==stage||stageKeyDigestV1(entry.key)!==entry.digest)throw Error(`Invalid browser stage evidence ${path}`);
    return [entry];
  });
}

/** 🧬️ Resolves the exact release-transpile input when linked bytes contain stripped debug sections. */
function strippedReleaseDigest(target:ComponentStageTargetV1,receipt:ReturnType<typeof readComponentReceiptV1>):string{
  const layout=componentModelLayoutV1(join(workspace,".cache/b/p/m"),{version:1,mode:"release",pluginId:target.pluginId,cratePath:target.cratePath});
  const bytes=readFileSync(join(layout.path,modelSchema.definitions.Protocol.const.component));
  if(bytes.byteLength!==receipt.wasmBytes||sha(bytes)!==receipt.wasmSha256)throw Error(`${target.pluginId}: raw model does not match the successful release receipt`);
  return sha(stripWasmDebugSectionsV1(bytes));
}

/** 🔐️ Matches the published release component against the exact key of either admitted transpile producer. */
export function matchesTranspileKey(entry:StageEntryV1,component:string,base:string):boolean{
  const {stage,schemaVersion,inputs,tools}=entry.key;
  const exact=(actual:Readonly<Record<string,string>>,expected:readonly string[])=>Object.keys(actual).length===expected.length&&expected.every(key=>Object.hasOwn(actual,key));
  const digest=(value:unknown)=>typeof value==="string"&&/^[0-9a-f]{64}$/u.test(value);
  if(stage!=="transpile"||!exact(inputs,["component"])||inputs.component!==component||!digest(component)||tools.base!==base||!digest(tools.materialization))return false;
  const common=["base","materialization"];
  if(schemaVersion===3)return exact(tools,[...common,"native-transpile","maps"])&&digest(tools["native-transpile"])&&digest(tools["maps"]);
  if(schemaVersion===2)return exact(tools,[...common,"native-transpile"])&&digest(tools["native-transpile"]);
  if(schemaVersion!==1||!tools.jco)return false;
  return exact(tools,[...common,"jco"])||
    exact(tools,[...common,"jco","native-transpile"])&&digest(tools["native-transpile"]);
}

/** 🗜️ Admits an optimizer stage that names wasm-opt, including the schema that records its binary and arguments. */
export function admittedOptimizeSchema(schemaVersion:number,tools:Readonly<Record<string,string>>):boolean{
  return (schemaVersion===1||schemaVersion===2)&&typeof tools["wasm-opt"]==="string"&&tools["wasm-opt"].length>0;
}

/** 🧷️ Binds the raw receipt to a unique complete set of optimized cores through the build's transpile, optimizer and browser probe. */
export function sealedBrowserCores(transpiles:readonly StageEntryV1[],optimizes:readonly StageEntryV1[],probes:readonly StageEntryV1[],base:string,semantic:string):readonly StageFileV1[]{
  const known=new Map<string,readonly StageFileV1[]>();
  for(const transpile of transpiles){
    const cores=transpile.files.filter(file=>file.path.startsWith(base+".core")&&file.path.endsWith(".wasm"));
    if(!cores.length||cores.length!==transpile.files.filter(file=>file.path.endsWith(".wasm")).length)continue;
    const coreNames=new Set(cores.map(core=>core.path));
    for(const probe of probes){
      if(probe.files.length!==1||probe.files[0]!.path!=="semantic.bin"||probe.files[0]!.sha256!==semantic)continue;
      const probedCores=Object.keys(probe.key.inputs).filter(path=>path.endsWith(".wasm")&&!path.startsWith("vendor/"));
      if(probedCores.length!==coreNames.size||probedCores.some(path=>!coreNames.has(path)))continue;
      const files=cores.flatMap(core=>{
        const outputs=optimizes.filter(row=>row.key.inputs.core===core.sha256&&row.files.length===1&&row.files[0]!.path==="core.wasm"&&row.files[0]!.sha256===probe.key.inputs[core.path]).map(row=>({...row.files[0]!,path:core.path}));
        return new Map(outputs.map(file=>[JSON.stringify(file),file])).size===1?[outputs[0]!]:[];
      }).sort((a,b)=>a.path.localeCompare(b.path));
      if(files.length===cores.length&&new Set(files.map(file=>file.path)).size===files.length)known.set(JSON.stringify(files),files);
    }
  }
  if(known.size!==1)throw Error(`expected one optimized browser core proof for ${base}, found ${known.size}`);
  return [...known.values()][0]!;
}

async function verifyReceipt(request:Request,targets:readonly ComponentStageTargetV1[],transpiles:readonly StageEntryV1[],optimizes:readonly StageEntryV1[],probes:readonly StageEntryV1[]):Promise<readonly StageFileV1[]>{
  const receipt=readComponentReceiptV1(request.stagedReceipt);
  if(receipt.wasmSha256!==request.wasmSha256||receipt.semanticDescriptorSha256!==request.semanticDescriptorSha256)throw Error(`${receipt.pluginId}: expected raw component receipt changed before optimized core certification`);
  const owners=targets.filter(target=>target.pluginId===receipt.pluginId&&target.packageId===receipt.packageId);
  if(owners.length!==1||receipt.profile!=="release")throw Error(`Exact release browser component owner required for ${receipt.pluginId}`);
  const target=owners[0]!,base=target.packageName.replaceAll("-","_");
  const matching=(digest:string)=>transpiles.filter(row=>matchesTranspileKey(row,digest,base));
  const initial=matching(receipt.wasmSha256);
  const files=sealedBrowserCores(initial.length?initial:matching(strippedReleaseDigest(target,receipt)),optimizes,probes,base,receipt.semanticDescriptorSha256);
  const root=dirname(request.stagedReceipt),actual=readdirSync(root,{recursive:true}).map(name=>name.replaceAll("\\","/")).filter(name=>name.endsWith(".wasm")).sort();
  if(JSON.stringify(actual)!==JSON.stringify(files.map(file=>file.path).sort()))throw Error(`${receipt.pluginId}: optimized core inventory differs from its build in ${root}`);
  for(const file of files){
    const core=join(root,file.path),facts=lstatSync(core);
    if(!facts.isFile()||facts.isSymbolicLink()||facts.size!==file.bytes||await fileSha256V1(core)!==file.sha256)throw Error(`${receipt.pluginId}: optimized browser core differs from its successful build: ${core}`);
  }
  return files;
}

if(import.meta.main){
  try{
    const requests=JSON.parse(process.argv[2]!) as Request[];
    const targets=componentStageTargetsV1(workspace),transpiles=stageEntries("transpile"),optimizes=stageEntries("optimize").filter(row=>admittedOptimizeSchema(row.key.schemaVersion,row.key.tools)),probes=stageEntries("probe").filter(row=>row.key.schemaVersion===2);
    const collected:(readonly StageFileV1[])[]=[];
    for(const request of requests)collected.push(await verifyReceipt(request,targets,transpiles,optimizes,probes));
    console.log(JSON.stringify(collected));
  }catch(error){
    console.error(error instanceof Error?error.message:String(error));
    process.exitCode=1;
  }
}
