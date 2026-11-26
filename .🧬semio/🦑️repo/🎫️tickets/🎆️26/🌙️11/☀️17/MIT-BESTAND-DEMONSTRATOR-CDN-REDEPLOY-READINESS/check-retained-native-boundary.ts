import { readFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { createRustSourceAnalysisV1, readRustSourceClosureV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { cargoSourceBoundaryExcessV1, effectiveCargoSourceBoundaryV1, mergeCargoCfgFeaturesV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts";

const root=resolve(import.meta.dirname,"../../../../../../.."),generated=join(import.meta.dirname,"🗑️generated");
const policy=JSON.parse(readFileSync(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json"),"utf8")).storage.sourceBoundary;
const companion=process.argv[2]==="companion";
const puzzle=process.argv[2]==="puzzle";
const rewrite=process.argv[2]==="rewriting";
const platform=readFileSync(join(generated,puzzle?"boundary-wasm32-wasip2.cfg":"boundary-host.cfg"),"utf8").trim().split("\n");
const companionRoot=join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧷️companions/🪝️tests-12-2");
const rewriteRoot=join(root,"✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🔨️modules/♻️rewriting");
const graphName=puzzle?"all-canonical-roots-retained-current.json":"cad-retained-native-units.json";
const targetName=puzzle?"semio_s_artifact_puzzle_5d_puzzle5d":"semio_plugin_retained_command";
const testRoot=companion?companionRoot:rewrite?rewriteRoot:undefined;
const wanted=testRoot
 ? [{pkg_id:"path+"+pathToFileURL(join(testRoot,"📦️packages/🦀️rust")).href,target:{src_path:join(testRoot,"🦀️.rs")},features:[]}]
 : JSON.parse(readFileSync(join(generated,graphName),"utf8")).units.filter(unit=>unit.target.name===targetName&&unit.mode==="build");
if(!wanted.length||(!testRoot&&!puzzle&&!wanted.some(unit=>unit.features.includes("artifact-app-testing"))))throw Error("Exact source boundary target required");
const analysis=createRustSourceAnalysisV1();
for(const unit of wanted){
 const manifestDirectory=fileURLToPath(unit.pkg_id.slice(5).split("#")[0]);
 const manifest=Bun.TOML.parse(readFileSync(join(manifestDirectory,"Cargo.toml"),"utf8")) as {package:{metadata:{semio:{["rust-source-boundary"]:typeof policy}}}};
 const limits=effectiveCargoSourceBoundaryV1(policy,manifest.package.metadata.semio["rust-source-boundary"]);
 const cfg=mergeCargoCfgFeaturesV1(testRoot?[...platform,"test"]:platform,unit.features);
 const sources=await readRustSourceClosureV1([unit.target.src_path],root,cfg,async path=>{
  try{return readFileSync(path);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return null;throw error;}
 },{manifestDirectory,generated:()=>undefined},analysis);
 const actual={bytes:sources.reduce((sum,row)=>sum+row.bytes,0),files:sources.length,lines:sources.reduce((sum,row)=>sum+row.activeLines,0)};
 const excess=cargoSourceBoundaryExcessV1(limits,actual);
 console.log(JSON.stringify({features:unit.features,actual,limits,excess,sources:sources.map(row=>[relative(root,row.path),row.bytes])},null,2));
 if(excess.length)process.exitCode=1;
}
