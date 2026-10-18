import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
const reg="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry";
const { componentLibraryManifestV1 } = await import(reg+"/📽️projection/🟦️.ts");
const R="/Users/ueli/Documents/semio";
const registry = JSON.parse(readFileSync(join(reg, "🤖️generated/🔌️plugins.json"), "utf8"));
console.log(registry.length);
for (const e of registry.filter((e:any)=>["space","reasoning"].includes(e.pluginId))) {
  const m=componentLibraryManifestV1(e.cratePath);
  console.log(e.pluginId,e.cratePath,m, resolve(R,m,"..","..",".."));
}
console.log(registry.slice(0,3).map((e:any)=>e.cratePath));
