const fs=require("fs");const [,,b,a]=process.argv;
const files=f=>{const s=new Set(),all=new Set();for(const l of fs.readFileSync(f,"utf8").split("\n")){if(!l)continue;const [st,file]=l.split("\t");all.add(file);if(st!=="passed")s.add(file);}return {fail:s,all};};
const norm=f=>f.replace(/\/🧪️tests\/[^/]*\/🟦️\.tsx?$/,"/§").replace(/\/🧪️tests\/🟦️\.ts$/,"/§");
const B=files(b),A=files(a);
console.log(`files collected before=${B.all.size} after=${A.all.size}; files with a failure before=${B.fail.size} after=${A.fail.size}`);
const tail=f=>f.split("/").slice(-4).join("/");
for(const f of A.fail) if(!B.fail.has(f)) console.log(" failing only after:",tail(f), B.all.has(f)?"(collected before, passing)":"(not collected before)");
for(const f of B.fail) if(!A.fail.has(f)) console.log(" failing only before:",tail(f));
