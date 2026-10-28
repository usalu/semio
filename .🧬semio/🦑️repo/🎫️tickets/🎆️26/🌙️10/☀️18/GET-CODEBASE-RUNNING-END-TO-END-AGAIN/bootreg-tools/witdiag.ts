/** 🔍 Diagnoses which check of the Cargo preparation witness fails for the print test selection, and times the observation replay (read-only). `bun witdiag.ts <root>` */
import {readFileSync,existsSync} from "node:fs";
import {createHash} from "node:crypto";
const root=process.argv[2]!,L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const w=await import(L+"/🗂️workspaces/🦀️cargo/🛠️preparation/👁️witness/🟦️.ts"),prep=await import(L+"/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts"),cu=await import(L+"/🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🟦️.ts");
const storage=prep.repositoryCargoPreparationStorageV1(root);
for(const [manifest,pk] of [["Cargo.toml",["semio-framework-print-tests"]]] as const){
 const paths=w.cargoPreparationRecordPathsV1(storage,root,manifest,pk);console.log("[DEBUG] paths",paths,existsSync(paths.witness),existsSync(paths.receipt));
 if(!existsSync(paths.witness))continue;
 const witness=JSON.parse(readFileSync(paths.witness,"utf8")),receipt=JSON.parse(readFileSync(paths.receipt,"utf8"));const h=witness.hashed;
 console.log("[DEBUG] receipt match",receipt.inputsSha256===witness.receipt.inputsSha256,createHash("sha256").update(JSON.stringify(h)).digest("hex")===receipt.inputsSha256,"recipes",h.recipes.length);
 console.log("[DEBUG] compilerEnv",JSON.stringify(h.compilerEnvironment),"now",JSON.stringify(w.cargoPreparationCompilerEnvironmentV1(process.env)));
 console.log("[DEBUG] global equal",JSON.stringify(h.globalConfiguration)===JSON.stringify(w.cargoPreparationGlobalConfigurationV1(process.env)));
 console.log("[DEBUG] lock",cu.cargoPreparationInputV1(root,witness.lock,"file").sha256===receipt.lockSha256);
 let bad=0;for(const input of [...h.inputs,...h.configuration])if(cu.cargoPreparationInputV1(root,input.path,input.kind).sha256!==input.sha256){bad++;if(bad<4)console.log("[DEBUG] input changed",input.path);}
 console.log("[DEBUG] inputs",h.inputs.length,"bad",bad,"tools",witness.tools.map((t:any)=>w.cargoToolStampV1(t.path).stamp===t.stamp));
 const a=performance.now(),ok=w.cargoPhysicalObservationsHoldV1(root,witness.observations);console.log("[DEBUG] observations",witness.observations.length,"hold",ok,(performance.now()-a).toFixed(0),"ms");
 const kinds:Record<string,number>={};for(const o of witness.observations)kinds[o.operation??o[0]]=(kinds[o.operation??o[0]]??0)+1;console.log("[DEBUG] kinds",JSON.stringify(kinds).slice(0,300));
}
