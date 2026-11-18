import{readFileSync,writeFileSync,mkdirSync,copyFileSync,renameSync,rmSync,existsSync}from"node:fs";import{dirname}from"node:path";import{LIVE,SC,STAGE,BK,changed,sha}from"./base.mjs";
const base=JSON.parse(readFileSync(SC+"base.json","utf8")),files=Object.keys(base);
for(const r of files){const now=existsSync(LIVE+r)?sha(LIVE+r):null;if(now!==base[r]){console.log("LIVE MOVED",r,base[r],"->",now);process.exit(4);}}
rmSync(BK,{recursive:true,force:true});
for(const r of files)if(existsSync(LIVE+r)){mkdirSync(dirname(BK+r),{recursive:true});copyFileSync(LIVE+r,BK+r);}
for(const r of files){mkdirSync(dirname(LIVE+r),{recursive:true});copyFileSync(STAGE+r,LIVE+r+".r113tmp");renameSync(LIVE+r+".r113tmp",LIVE+r);console.log(r,base[r],"->",sha(LIVE+r));}
