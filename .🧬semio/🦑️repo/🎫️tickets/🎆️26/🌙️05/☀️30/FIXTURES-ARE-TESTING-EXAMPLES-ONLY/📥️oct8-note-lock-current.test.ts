import {test,expect} from "bun:test";
import {resolve} from "node:path";
const {cargoStreamingStatus}=await import(resolve(process.cwd(),"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts"));
test("current Note oracle lock resolves current workspace dependencies before the unchanged locked fetch",async()=>{
const root=process.cwd(),manifest=resolve(root,"✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🔮️oracles/📦️packages/🦀️rust/Cargo.toml");
expect(await cargoStreamingStatus(["update","--workspace","--offline","--manifest-path",manifest],root,process.env,120000)).toBe(0);
expect(await cargoStreamingStatus(["fetch","--locked","--manifest-path",manifest],root,process.env,120000)).toBe(0);
console.log("[DEBUG] current Note oracle locked fetch completed after bounded workspace lock resolution");
},240000);
