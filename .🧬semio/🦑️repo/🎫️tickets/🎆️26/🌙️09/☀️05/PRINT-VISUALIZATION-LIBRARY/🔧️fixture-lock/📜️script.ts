import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

/** 🔒️ Restores the exact Cargo-owned standalone fixture output required by the repository publication contract. */
const root=process.cwd(),directory=join(root,"🧰️framework/🛍️products/💻️os/🧪️testing/🧩️jcoprobe/👽️guest/📦️packages/🦀️rust"),manifest=join(directory,"Cargo.toml"),lock=join(directory,"Cargo.lock"),before=readFileSync(manifest,"utf8");
if(!before.includes('name = "semio-jcoprobe-guest"')||!before.includes("[workspace]")||existsSync(lock))throw Error("Fixture lock producer scope differs from the admitted missing output");
const {runTool}=await import(pathToFileURL(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/📜️script.ts")).href);
await runTool("cargo",["generate-lockfile","--manifest-path",manifest],root,AbortSignal.timeout(180000));
if(readFileSync(manifest,"utf8")!==before||!existsSync(lock)||!readFileSync(lock,"utf8").includes('name = "semio-jcoprobe-guest"'))throw Error("Fixture lock producer did not preserve its input and output authority");
console.log("[DEBUG] Cargo restored the exact standalone JCO guest lock and preserved its authored manifest");
