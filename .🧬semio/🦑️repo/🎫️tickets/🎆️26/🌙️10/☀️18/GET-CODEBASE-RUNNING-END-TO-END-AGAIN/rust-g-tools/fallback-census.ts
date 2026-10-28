/** bun fallback-census.ts : size of the native fallback command inputs (the cargo script closure part) with the scheduling strip applied. */
import {resolve} from "node:path";
const root=process.cwd(),lib="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const inference=await import(resolve(root,lib+"🏗️build/🕸️graph/📥️inference/🟨️.mjs")) as any;await inference.libraryBootstrap;
const scripts=await import(resolve(root,lib+"🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs")) as any;
const cache=inference.createScriptInputCache({workspaceRoot:root});
const rows=(await scripts.nativeCommandInputs(root,cache)).filter((row:any)=>typeof row==="string"&&row.includes("/"));
const prefix=lib+"⚡️caching/🦀️cargo/";
console.log("fallback string inputs",rows.length,"files under native-build",rows.filter((r:string)=>r.includes("/🏗️native-build/")).length,"under leases",rows.filter((r:string)=>r.includes("/🔒️leases/")).length,"under host-capacity",rows.filter((r:string)=>r.includes("/🌡️host-capacity/")).length,"under storage",rows.filter((r:string)=>r.includes("/💾️storage/")).length);
