import {readFileSync, writeFileSync} from "node:fs";
const b = readFileSync(process.argv[2]); let p = 8;
const leb = () => { let r=0,s=0,x; do { x=b[p++]; r|=(x&127)<<s; s+=7; } while (x&128); return r>>>0; };
let nImports=0; const secs=[];
while (p<b.length) { const id=b[p++]; const size=leb(); const end=p+size; if(id===0){ const n=leb(); const name=b.toString("utf8",p,p+n); p+=n; secs.push(["custom:"+name,end-p]); if(name==="name"){ const out={}; while(p<end){ const sid=b[p++]; const sz=leb(); const e2=p+sz; if(sid===1){ const c=leb(); for(let i=0;i<c;i++){ const idx=leb(); const l=leb(); out[idx]=b.toString("utf8",p,p+l); p+=l; } } p=e2; } writeFileSync(process.argv[3], JSON.stringify(out)); console.log("names",Object.keys(out).length); } } else { secs.push([id,size]); if(id===2){ const q=p; const c=leb(); for(let i=0;i<c;i++){ let l=leb(); p+=l; l=leb(); p+=l; const k=b[p++]; if(k===0){leb();nImports++;} else if(k===1){p++;const f=b[p++];leb(); if(f&1)leb();} else if(k===2){const f=b[p++];leb(); if(f&1)leb();} else if(k===3){p+=2;} } p=q; } } p=end; }
console.log(secs.map(s=>s.join(":")).join(" "), "funcImports",nImports);
