const BUILD="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const {graphAdmissionFormatHashV1,graphAdmissionFormatUrlsV1}=await import(BUILD+"/🕸️graph/🔐️authority/🟦️.ts");
const {readdirSync,openSync,readSync,closeSync,statSync}=await import("node:fs");
console.log("current",graphAdmissionFormatHashV1());
for(const url of graphAdmissionFormatUrlsV1()){const p=decodeURIComponent(url.pathname);console.log(statSync(p).mtime.toISOString(),p.split("🏗️build/")[1]);}
const root="/Users/ueli/Documents/semio/.cache/b/g/m";
for(const name of readdirSync(root+"/selected").filter(n=>n.endsWith(".admission.json"))){const fd=openSync(root+"/selected/"+name,"r"),b=Buffer.alloc(200);readSync(fd,b,0,200,0);closeSync(fd);const m=/"format":"([0-9a-f]+)"/.exec(b.toString());console.log("memo",name.slice(0,8),m?.[1]?.slice(0,12),statSync(root+"/selected/"+name).mtime.toISOString());}
for(const scope of ["592b0e78bdc4b639af620b1835e90fd73e08b9819d5989ec783018605f01893e","a8c1977cc98f4b154a12f0480ef92bb8982da3110bb4f09cd082db89815f768a"]){const dir=root+"/inference/selections/"+scope+"/oc";for(const n of readdirSync(dir)){const fd=openSync(dir+"/"+n,"r"),b=Buffer.alloc(300);readSync(fd,b,0,300,0);closeSync(fd);const m=/"format":"([0-9a-f]+)"/.exec(b.toString()),q=/"sequence":(\d+)/.exec(b.toString());console.log("pack",scope.slice(0,8),n,m?.[1]?.slice(0,12),q?.[1],statSync(dir+"/"+n).mtime.toISOString());}}
