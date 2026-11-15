import {createHash} from "node:crypto";
import {mkdir,readFile,rename,writeFile} from "node:fs/promises";
import {dirname} from "node:path";
import {hostname} from "node:os";
import {fleetCacheEndpointsV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🛰️fleet/🔑️credential/🟦️.ts";
import {fleetCacheFactsPathV1,requireFleetCacheShapeV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🛰️fleet/🟦️.ts";

const store="/Users/Shared/semio-store",env=await readFile("/etc/semio/remote-cache.env","utf8");
const line=env.split("\n").find(row=>row.startsWith("SEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN="));
if(line===undefined)throw Error("write token line missing");
const token=line.slice(line.indexOf("=")+1).trim().replace(/^['"]|['"]$/gu,"");
const now=Date.now(),facts={version:1,group:"g1",generation:Number(process.argv[2]??2),publishedAtMs:now,publisher:{directory:"/var/lib/semio-remote-cache",port:6090,host:hostname()},cache:{endpoints:fleetCacheEndpointsV1(6090,[],["http://cache.g1.semio.internal:6090"]),instance:"semio",access:"read-write",protocols:["bazel-http","nx-self-hosted"],protocolVersion:1},credential:{scope:"fleet",id:createHash("sha256").update(token).digest("hex").slice(0,16),token,rotatedAtMs:now}};
requireFleetCacheShapeV1("Facts",facts,"Closed fleet cache facts required");
const path=fleetCacheFactsPathV1(store),temporary=`${path}.${process.pid}.tmp`;
await mkdir(dirname(path),{recursive:true,mode:0o700});await writeFile(temporary,JSON.stringify(facts,null,1)+"\n",{mode:0o600});await rename(temporary,path);
console.log(`published ${path} with ${facts.cache.endpoints.length} endpoints: ${facts.cache.endpoints.join(", ")}`);
