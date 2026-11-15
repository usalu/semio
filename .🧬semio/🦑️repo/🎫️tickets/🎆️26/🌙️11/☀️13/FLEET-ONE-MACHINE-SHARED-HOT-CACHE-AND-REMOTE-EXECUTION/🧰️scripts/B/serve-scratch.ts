import {startRemoteCacheServerV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
/** usage: bun serve-scratch.ts <directory> <port> <token>: a scratch cache server with the current code (claims, slots), for live-tree proofs when the fleet server runs old code. */
const [directory,port,token]=process.argv.slice(2) as [string,string,string];
const server=await startRemoteCacheServerV1({directory,host:"127.0.0.1",port:Number(port),maximumBytes:2**34,maximumEntryBytes:2**29,maximumAgeDays:30,defaultInstance:"semio",tokens:{write:token}});
console.log("listening",server.url);
await server.stopped;
