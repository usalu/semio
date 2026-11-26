/** 🧯️ Native fallback: the graph-manifest projection of every component that declares `🛂️manifest/📇️outputs.json`, without the build fleet. */
import {readFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {readGraphOutputCatalog} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📇️catalog/🟦️.ts";
import {renderGraphArtifacts} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📽️projection/🟦️.ts";
import {writeGraphArtifacts} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📤️publication/🟦️.ts";
const root="/Users/ueli/Documents/semio",read=(path:string)=>readFileSync(path).toString("utf8");
const found=Bun.spawnSync(["find",root,"-name","📇️outputs.json","-not","-path","*/node_modules/*","-not","-path","*/target/*","-not","-path","*/.cache/*","-not","-path","*/🕸️graph/🛂️manifest/*"]).stdout.toString().split("\n").filter(Boolean);
for(const file of found){const component=dirname(dirname(file)),outDir=join(component,"🤖️generated"),catalog=readGraphOutputCatalog(file,read),rendered=renderGraphArtifacts(root,outDir,catalog,true,{read,observe:()=>{}});writeGraphArtifacts(outDir,rendered.artifacts);console.log(rendered.manifestCount+" manifests -> "+outDir);}
