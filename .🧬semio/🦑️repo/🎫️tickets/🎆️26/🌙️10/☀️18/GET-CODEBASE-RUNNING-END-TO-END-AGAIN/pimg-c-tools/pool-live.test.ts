import {expect,test} from "bun:test";
import {mkdirSync,readdirSync,rmSync,statfsSync,lstatSync} from "node:fs";
import {join} from "node:path";
import {captureBazelSourceSnapshotV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📸️snapshot/🟦️.ts";

const root="/Users/ueli/Documents/semio",image=join(root,".cache/b/p/m/tmp/pimg-c-probe"),checkout=join(root,"node_modules/typescript");
const walk=(base:string,prefix=""):string[]=>readdirSync(join(base,prefix),{withFileTypes:true}).flatMap(entry=>entry.isDirectory()?walk(base,prefix+entry.name+"/"):entry.isFile()?[prefix+entry.name]:[]);
const free=()=>{const space=statfsSync(image);return space.bavail*space.bsize;};
const request=(space:string,files:string[],pool?:string)=>({root:checkout,directory:space,...pool?{pool}:{},files,real:new Set<string>(),views:{alias:"a",real:"s"},alias:(directory:string)=>directory.replaceAll("/","_")||"_",check:()=>undefined,progress:async()=>undefined});
test("live pool cost on the products image",async()=>{
 rmSync(image,{recursive:true,force:true});mkdirSync(image,{recursive:true});
 try{
  const files=walk(checkout).slice(0,4000),bytes=files.reduce((sum,file)=>sum+lstatSync(join(checkout,file)).size,0);
  const marks:Record<string,number>={start:free()};
  mkdirSync(join(image,"w1"));await captureBazelSourceSnapshotV1(request(join(image,"w1"),files,join(image,"pool")));marks.afterFirstPooled=free();
  mkdirSync(join(image,"w2"));await captureBazelSourceSnapshotV1(request(join(image,"w2"),files,join(image,"pool")));marks.afterSecondPooled=free();
  mkdirSync(join(image,"w3"));await captureBazelSourceSnapshotV1(request(join(image,"w3"),files));marks.afterPlain=free();
  console.log("[DEBUG] files",files.length,"MiB",Math.round(bytes/1048576),"first pooled",Math.round((marks.start-marks.afterFirstPooled)/1048576),"second pooled",Math.round((marks.afterFirstPooled-marks.afterSecondPooled)/1048576),"plain",Math.round((marks.afterSecondPooled-marks.afterPlain)/1048576));
  expect(true).toBe(true);
 }finally{rmSync(image,{recursive:true,force:true});}
},120000);
