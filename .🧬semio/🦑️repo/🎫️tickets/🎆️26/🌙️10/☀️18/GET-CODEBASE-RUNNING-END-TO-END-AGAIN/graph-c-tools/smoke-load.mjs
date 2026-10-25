import{join}from"node:path";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const inference=await import(join(L,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await inference.libraryBootstrap;inference.prepareSourceAnalysesV1();
const g=await import(join(L,"🏗️build/🕸️graph/🟦️.ts"));
console.log("loaded",typeof g.buildGraphGenerationIdentityV1,typeof inference.emojiProjectJsonNodes);
const od=await import(join(L,"🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs"));console.log(typeof od.readBuildGeneratedCompileOutputV1);
const pg=await import(join(L,"🏗️build/🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs"));console.log(typeof pg.playgroundSiteTargetsForAdmissionV1);
