/** 🗑️ Retires idle Cargo lanes of the repository cache through the framework's own API: each lane under its claim (never one a live process holds), then every image whose lane is gone, whole. usage: bun lanes-retire.ts <cache root `.cache/b/r/m/u`> <lane id>... */
import{retireIdleLaneImagesV1,laneImagePortsV1}from"../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🧩️image/🟦️.ts";
import{removeIdleLaneV1}from"../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🟦️.ts";
const[root,...ids]=process.argv.slice(2);if(root===undefined||ids.length===0)throw Error("usage: lanes-retire.ts <root> <lane>...");
const ports=await laneImagePortsV1(root);
for(const id of ids){const done=removeIdleLaneV1(root,id,Date.now(),(kind,parameters)=>{console.log("[report]",kind,JSON.stringify(parameters));});console.log(id,done?"removed":"kept (held, live or custody unresolved)");}
console.log("images retired:",JSON.stringify(await retireIdleLaneImagesV1(root,ports,Date.now())));
