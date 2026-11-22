import {readdirSync,statSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {scanRemoteCacheDirectoryV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🧹️collector/🟦️.ts";
const root=process.argv[2]!,index=await scanRemoteCacheDirectoryV1(root),kinds:Record<string,{entries:number;bytes:number}>={};
for(const [key,row] of index){const [namespace,kind]=key.split("/"),name=namespace+"/"+kind;(kinds[name]??={entries:0,bytes:0}).entries++;kinds[name]!.bytes+=row.size;}
let metas=0,declared=0;const walk=(directory:string)=>{let rows;try{rows=readdirSync(directory,{withFileTypes:true});}catch{return;}for(const row of rows){const path=join(directory,row.name);if(row.isDirectory())walk(path);else if(row.name.endsWith(".json")){metas++;try{if(JSON.parse(readFileSync(path,"utf8")).declaration)declared++;}catch{}}}};
walk(join(root,".meta"));
let lock="";try{lock=readFileSync(join(root,".server.lock"),"utf8").replace(/\n/g," ");}catch{}
console.log(JSON.stringify({root,entries:index.size,kinds,sidecars:metas,declared,serverLock:lock,retention:(()=>{try{return statSync(join(root,".retention")).isDirectory();}catch{return false;}})()},null,1));
