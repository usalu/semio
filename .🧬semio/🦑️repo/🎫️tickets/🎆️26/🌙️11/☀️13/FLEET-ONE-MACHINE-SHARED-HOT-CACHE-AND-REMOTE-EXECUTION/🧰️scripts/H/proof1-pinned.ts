import {drain,fleetFetchContextV1 as _unused,join,machine,mb,pinnedFetchV1,readFileSync,run,scratch,seconds,serve,store,upstreamBytes} from "./proof-common.ts";
void _unused;
const registry=JSON.parse(readFileSync("/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🗂️registry/🔣️.json","utf8")).runtimes.bun as {download:{base:string};distributions:{platform:string;arch:string;archive:string;bytes:number;sha256:string}[]};
const bunRow=registry.distributions.find(row=>row.platform==="darwin"&&row.arch==="arm64")!;
const rustup=JSON.parse(readFileSync("/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🦀️rustup/🧬️schema/🔣️.json","utf8")).definitions.Protocol.const as {bootstrapVersion:string;bootstrapSha256:Record<string,string>};
const pins=[
 {label:"bun runtime 1.4.2 (pinned, darwin arm64)",url:`${registry.download.base}/${bunRow.archive}`,sha256:bunRow.sha256,bytes:bunRow.bytes},
 {label:"rustup-init 1.29.0 (pinned in the schema now, aarch64-apple-darwin)",url:`https://static.rust-lang.org/rustup/archive/${rustup.bootstrapVersion}/aarch64-apple-darwin/rustup-init`,sha256:rustup.bootstrapSha256["aarch64-apple-darwin"]!},
];
const server=await serve(),remote={endpoint:server.url,instance:"main",token:"proof-write-token"},rows:unknown[]=[];
const A=machine("A"),B=machine("B"),C=machine("C"),D=machine("D");
for(const pin of pins){
 const row:Record<string,unknown>={artifact:pin.label};
 let started=performance.now();
 const a=await drain(await pinnedFetchV1(A.context(remote),pin));let elapsed=(performance.now()-started)/1000;
 row.firstMachine={seconds:seconds(elapsed*1000),bytes:a.bytes,internetBytes:upstreamBytes(A.ledger),mbPerSecondFromInternet:Math.round(mb(a.bytes)/elapsed*10)/10,shaMatchesPin:a.sha256===pin.sha256};
 row.storedFileEqualsShasum=(await run(["shasum","-a","256",join(store,"v1","fleet","fetch",pin.sha256.slice(0,2),pin.sha256)],scratch)).out.split(" ")[0]===pin.sha256;
 started=performance.now();const beforeB=upstreamBytes(B.ledger);
 const b=await drain(await pinnedFetchV1(B.context(remote),pin));elapsed=(performance.now()-started)/1000;
 row.secondMachineFromStore={seconds:seconds(elapsed*1000),bytes:b.bytes,internetBytes:upstreamBytes(B.ledger)-beforeB,mbPerSecondFromStore:Math.round(mb(b.bytes)/elapsed*10)/10,shaMatchesPin:b.sha256===pin.sha256};
 rows.push(row);
}
const http:Record<string,unknown>={};
{
 const pin=pins[1]!;
 const started=performance.now(),c=await drain(await pinnedFetchV1(C.context(remote,false),pin));
 http.secondMachineHttpTierOnly={seconds:seconds(performance.now()-started),bytes:c.bytes,internetBytes:upstreamBytes(C.ledger),shaMatchesPin:c.sha256===pin.sha256};
}
await server.stop();
{
 const pin=pins[1]!,started=performance.now(),d=await drain(await pinnedFetchV1(D.context(remote,false),pin));
 http.mirrorStoppedNoStoreFallsBackToUpstream={seconds:seconds(performance.now()-started),bytes:d.bytes,internetBytes:upstreamBytes(D.ledger),shaMatchesPin:d.sha256===pin.sha256};
}
console.log(JSON.stringify({scratch,pinned:rows,http},null,1));
