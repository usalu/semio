import {checkZeroTouchCommandsV1,checkZeroTouchClosuresV1,diskZeroTouchTreeV1,workspaceSpecifierTargetV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🛗️zero-touch/🟦️.ts";
import {posix} from "node:path";import {readFileSync,writeFileSync} from "node:fs";
const R="/Users/Shared/semio";const tree=diskZeroTouchTreeV1(R);
const fs=[...checkZeroTouchCommandsV1(R).findings,...checkZeroTouchClosuresV1(R).findings].filter(f=>f.kind==="workspace-specifier");
const pairs=new Map<string,[string,string]>();const other:any[]=[];
for(const f of [...checkZeroTouchCommandsV1(R).findings,...checkZeroTouchClosuresV1(R).findings])if(f.kind!=="workspace-specifier")other.push(f);
for(const f of fs)pairs.set(f.file+"\0"+f.specifier,[f.file,f.specifier]);
const edits:any[]=[],bad:any[]=[];
for(const [file,spec] of pairs.values()){
 const t=workspaceSpecifierTargetV1(tree.packages,spec);if(!t){bad.push([file,spec,"notarget"]);continue}
 let rel=posix.relative(posix.dirname(file),t);if(!rel.startsWith("."))rel="./"+rel;
 const text=readFileSync(R+"/"+file,"utf8");let n=0,imp=0;
 for(const q of ['"',"'"]){const needle=q+spec+q;let i=-1;while((i=text.indexOf(needle,i+1))>=0){n++;const ls=text.lastIndexOf("\n",i)+1,le=text.indexOf("\n",i);const line=text.slice(ls,le<0?undefined:le);if(/\b(from|import|require|export)\b/.test(line))imp++;}}
 if(n===0||n!==imp){bad.push([file,spec,n,imp]);continue}
 const q=text.includes('"'+spec+'"')?'"':"'";
 edits.push({file,old:q+spec+q,new:q+rel+q,count:n});
}
writeFileSync("edits.json",JSON.stringify(edits,null,1));
console.log("pairs",pairs.size,"edits",edits.length,"bad",JSON.stringify(bad),"other",other.length,JSON.stringify(other.slice(0,5)));
