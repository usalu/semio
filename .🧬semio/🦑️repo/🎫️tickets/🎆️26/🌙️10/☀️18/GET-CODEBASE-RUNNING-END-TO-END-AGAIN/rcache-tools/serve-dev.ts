import {startRemoteCacheServerV1} from "./staged/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
const directory=process.argv[2]!,port=Number(process.argv[3]??0),token=process.env.DEV_TOKEN;
const server=await startRemoteCacheServerV1({directory,host:"127.0.0.1",port,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"default",tokens:{write:token,read:process.env.DEV_READ_TOKEN},onEvent:event=>console.log("[DEBUG] "+JSON.stringify(event))});
console.log("listening "+server.url);
process.on("SIGTERM",()=>void server.stop().then(()=>process.exit(0)));
await server.stopped;
