import { readFileSync } from "node:fs";
const { rustTokens } = await import(process.argv[2]);
const o={"(":")","[":"]","{":"}"};
for (const f of process.argv.slice(3)) { const s=readFileSync(f,"utf8"),t = rustTokens(s); const st=[];
 for (const x of t){ if(o[x.text]&&x.text.length===1)st.push(x); else if(")]}".includes(x.text)&&x.text.length===1){const a=st.pop(); if(!a||o[a.text]!==x.text){console.log(f.slice(-50),"mismatch open",a&&a.text,"@",a&&a.start,"close",x.text,"@",x.start);console.log(JSON.stringify(s.slice(a.start-150,a.start+200)));console.log(JSON.stringify(s.slice(x.start-200,x.start+50)));break;}}}}
