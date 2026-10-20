import fs from "node:fs";import {createHash} from "node:crypto";
const M="/Users/ueli/Documents/semio/.cache/b/g/m/";let bad={nlink:[],nonfile:[],size:[],sha:[],tmp:[]},n=0;
for(const d of ["build-input-closures","build-lock-closures"]){
 for(const e of fs.readdirSync(M+d)){
  if(!/^[a-f0-9]{64}\.json$/.test(e)){bad.tmp.push(d+"/"+e);continue;}
  const p=M+d+"/"+e,st=fs.lstatSync(p,{bigint:true});n++;
  if(!st.isFile()||st.isSymbolicLink()){bad.nonfile.push(p);continue;}
  if(st.nlink!==1n)bad.nlink.push([p.slice(-24),Number(st.nlink)]);
  if(st.size<1n||st.size>BigInt(1<<30))bad.size.push([p.slice(-24),Number(st.size)]);
  if(process.argv[2]==="sha"){const b=fs.readFileSync(p);if(createHash("sha256").update(b).digest("hex")!==e.slice(0,-5))bad.sha.push(p.slice(-24));}
 }}
console.log(n,JSON.stringify({nlink:bad.nlink.length,nonfile:bad.nonfile.length,size:bad.size.length,sha:bad.sha.length,tmp:bad.tmp.length}));
console.log(JSON.stringify({nlink:bad.nlink.slice(0,5),tmp:bad.tmp.slice(0,8),sha:bad.sha.slice(0,5),size:bad.size.slice(0,5)}));
