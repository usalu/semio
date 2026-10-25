import {readFileSync} from "node:fs";
const inference=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs");
await inference.libraryBootstrap;
const ci=inference.cacheInternals;
const rows=JSON.parse(readFileSync("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json","utf8")).targetClasses;
for(const n of ["build","generate","typegen","build-storybook","generate-taxonomy-census","build-check","test","package","generate-wgpu","🏭️generator-build","typecheck","wasm"])console.log(n,ci.targetExclusionsV1(n,rows),ci.targetClassV1(n,rows)?.reason);
const m=(path,pattern)=>true;
console.log(ci.excludedInputConflictV1(["default","{workspaceRoot}/a/🧪️tests/x.ts","^default"],{default:[]},["**/🧪️tests/**/*"],(p,pat)=>true,"."));
