import { developmentRuntimeRoot, builtPluginModulesRootIn } from "@semio-tech/framework-os-dev/activation";
import { existsSync } from "node:fs"; import { join } from "node:path";
const W="/Users/ueli/Documents/semio";
for(const l of ["generator","energy","fem3d"]){const d=join(developmentRuntimeRoot(W,l,"prototype","react"),"activation","🔣️receipt.json");console.log(l,d.length,existsSync(d),d);}
console.log("modules",builtPluginModulesRootIn(W,"prototype"),existsSync(builtPluginModulesRootIn(W,"prototype")));
import { repositoryCachePartitionRootDataV1 } from "@semio-tech/repo-lib/storage";
process.env.SEMIO_BUILD_MODE="prototype";process.env.SEMIO_VITE_CACHE_DIRECTORY=join(repositoryCachePartitionRootDataV1(W,"products"),"serveprep-vite-probe-unused");
const m=await import(W+"/♻️mit-bestand/🧺️demonstrator/🏗️builder/🌐️vite/🟦️.ts");
console.log("default",typeof m.default);
const cfg=await (m.default as any)({command:"build",mode:"prototype"});
console.log("buildcfg keys",Object.keys(cfg||{}).slice(0,10), (cfg?.plugins??[]).length);
