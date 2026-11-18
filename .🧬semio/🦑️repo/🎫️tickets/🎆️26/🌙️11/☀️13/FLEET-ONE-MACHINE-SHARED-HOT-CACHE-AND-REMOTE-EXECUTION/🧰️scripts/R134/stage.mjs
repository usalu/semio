import {readFileSync,writeFileSync,mkdirSync,cpSync,rmSync,existsSync,readdirSync,statSync} from "node:fs";
import {dirname,join,relative,resolve} from "node:path";
import {PATCHES,NEWS,L,SC,HERE,applyEdits} from "./patches.mjs";
const BASE=process.env.R134_BASE==="1",TESTS=process.env.R134_TESTSONLY==="1",MIRROR=SC+(TESTS?"mirror-red/":BASE?"mirror-base/":"mirror/");
const B="🏗️build/",INF=B+"🕸️graph/📥️inference/",RES=B+"🏡️resident/",GS=B+"🪪️graph-state/";
const ROOTS=[GS.slice(0,-1),B+"🕸️graph/🕰️generations",B+"🕸️graph/🧮️derived-nodes",RES.slice(0,-1),INF.slice(0,-1)];
rmSync(MIRROR,{recursive:true,force:true});mkdirSync(MIRROR,{recursive:true});
for(const root of ROOTS){const from=L+root,to=MIRROR+root;mkdirSync(dirname(to),{recursive:true});cpSync(from,to,{recursive:true,filter:p=>!p.includes("node_modules")});}
const patched=new Map();
if(!BASE){for(const{file,edits}of PATCHES){if(TESTS&&!file.includes("/🧪️tests/"))continue;const rel=relative(L,file);let text;try{text=readFileSync(MIRROR+rel,"utf8");}catch{text=readFileSync(file,"utf8");}
 text=applyEdits(text,edits,file);mkdirSync(dirname(MIRROR+rel),{recursive:true});writeFileSync(MIRROR+rel,text);patched.set(rel,true);}
 for(const{to,from}of TESTS?[]:NEWS){const rel=relative(L,to);mkdirSync(dirname(MIRROR+rel),{recursive:true});writeFileSync(MIRROR+rel,readFileSync(HERE+"new/"+from));}}
const walk=dir=>readdirSync(dir,{withFileTypes:true}).flatMap(e=>e.isDirectory()?(e.name==="node_modules"?[]:walk(join(dir,e.name))):[join(dir,e.name)]);
const code=/\.(ts|mjs|mts|tsx)$/u,spec=/(from\s*|import\s*\(\s*|import\s*)(["'])(\.{1,2}\/[^"']+)\2/gu;
for(const file of walk(MIRROR)){if(!code.test(file))continue;const rel=relative(MIRROR,file),origDir=dirname(L+rel);
 let text=readFileSync(file,"utf8");
 text=text.replace(spec,(all,head,quote,s)=>{const target=resolve(origDir,s),trel=relative(L,target);const inMirror=existsSync(MIRROR+trel);return head+quote+(inMirror?MIRROR+trel:target)+quote;});
 writeFileSync(file,text);}
console.log("staged",MIRROR);
