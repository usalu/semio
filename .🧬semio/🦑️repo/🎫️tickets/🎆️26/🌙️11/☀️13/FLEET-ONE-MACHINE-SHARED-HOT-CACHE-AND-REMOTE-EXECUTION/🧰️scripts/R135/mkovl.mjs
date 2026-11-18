import{readFileSync,writeFileSync,mkdirSync}from"node:fs";import{basename}from"node:path";
const S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R135/";
mkdirSync(S+"ovl",{recursive:true});
const out=[];let n=0;
for(const line of readFileSync(S+"over.list","utf8").split("\n").filter(Boolean)){
 const at=line.indexOf("="),live=line.slice(0,at),staged=line.slice(at+1).replace("@S@",S.slice(0,-1));
 const text=readFileSync(staged,"utf8");
 if(text.includes("🐢️speed/🟨️.mjs")&&!live.includes("🐢️speed/")){const copy=S+"ovl/"+(n++)+"-"+basename(staged);writeFileSync(copy,text.replace(/"(?:\.\.\/)+🐢️speed\/🟨️\.mjs"/gu,'"'+S+'speedlab/🟨️.mjs"'));out.push(live+"="+copy);}
 else out.push(live+"="+staged);
}
writeFileSync(S+"over.resolved",out.join("\n")+"\n");
