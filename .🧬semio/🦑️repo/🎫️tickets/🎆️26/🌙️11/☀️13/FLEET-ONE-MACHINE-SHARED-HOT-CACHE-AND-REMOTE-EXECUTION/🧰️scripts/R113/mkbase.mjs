import{writeFileSync,existsSync}from"node:fs";import{LIVE,SC,changed,sha}from"./base.mjs";
const base={};for(const r of changed())base[r]=existsSync(LIVE+r)?sha(LIVE+r):null;
writeFileSync(SC+"base.json",JSON.stringify(base,null,1));console.log(base);
