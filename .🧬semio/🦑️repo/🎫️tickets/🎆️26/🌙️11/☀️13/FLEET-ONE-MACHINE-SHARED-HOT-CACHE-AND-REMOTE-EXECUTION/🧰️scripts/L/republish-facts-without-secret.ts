const L="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/";
const {publishFleetCacheFactsV1}=await import(L+"🛰️fleet/📣️publication/🟦️.ts");
const {readFileSync}=await import("node:fs");
const store="/Users/Shared/semio-store",old=JSON.parse(readFileSync(store+"/v1/fleet/cache.json","utf8"));
const result=await publishFleetCacheFactsV1({storeRoot:store,directory:old.publisher.directory,group:old.group,instance:old.cache.instance,endpoints:old.cache.endpoints,port:old.publisher.port,host:old.publisher.host});
console.log("changed",result.changed,"generation",result.facts.generation,"hasCredential","credential" in result.facts);
