import {createHash} from "node:crypto";
import {join,machine,mb,run,scratch,seconds,store,upstreamBytes} from "./proof-common.ts";
import {pinnedFetchV1} from "./proof-common.ts";
const tectonic=JSON.parse(await Bun.file("/Users/Shared/semio/🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/🔣️.json").text()) as {release:string;platforms:{target:string;archive:string;sha256:string;bytes:number}[]};
const row=tectonic.platforms.find(item=>item.target==="aarch64-apple-darwin")!,url=`${tectonic.release}/${row.archive}`;
const follow=async(m:ReturnType<typeof machine>)=>{
 let target=url;const hops:string[]=[];
 for(let i=0;i<5;i++){
  const response=await pinnedFetchV1(m.context(undefined),{url:target,sha256:row.sha256,bytes:row.bytes},{redirect:"manual"});
  hops.push(`${response.status}`);
  if([301,302,303,307,308].includes(response.status)){target=new URL(response.headers.get("location")!,target).href;await response.body?.cancel();continue;}
  const hash=createHash("sha256");let bytes=0;for await(const chunk of response.body as unknown as AsyncIterable<Uint8Array>){hash.update(chunk);bytes+=chunk.byteLength;}
  return {hops,bytes,ok:hash.digest("hex")===row.sha256};
 }
 throw Error("too many redirects");
};
const A=machine("A"),B=machine("B");
let started=performance.now();const a=await follow(A);const firstSeconds=seconds(performance.now()-started),firstInternet=upstreamBytes(A.ledger);
started=performance.now();const b=await follow(B);
console.log(JSON.stringify({archive:row.archive,firstMachine:{...a,seconds:firstSeconds,internetMB:mb(firstInternet)},secondMachine:{...b,seconds:seconds(performance.now()-started),internetBytes:upstreamBytes(B.ledger)}},null,1));
void join;void run;void store;void scratch;
