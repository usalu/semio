const R="/Users/ueli/Documents/semio/";
const L=Bun.JSONC.parse(await Bun.file(R+".vscode/launch.json").text());
const S=Bun.JSONC.parse(await Bun.file(R+".vscode/🧩️launch.seed.jsonc").text());
const canon=(v:any):string=>v===undefined?"U":Array.isArray(v)?"["+v.map(canon).join(",")+"]":v&&typeof v==="object"?"{"+Object.keys(v).sort().map(k=>JSON.stringify(k)+":"+canon(v[k])).join(",")+"}":JSON.stringify(v);
const lm=new Map<string,any[]>();for(const c of L.configurations){const a=lm.get(c.name)??[];a.push(c);lm.set(c.name,a)}
let n=0;
for(const s of S.configurations){ if(typeof s==='string')continue; const ls=lm.get(s.name); if(!ls)continue; if(ls.some(l=>canon(l)===canon(s)))continue; const l=ls[0];
 const ks=[...new Set([...Object.keys(s),...Object.keys(l)])].filter(k=>canon(s[k])!==canon(l[k]));
 if(ks.length===1&&ks[0]==="presentation"){ if(Math.abs(s.presentation.order-l.presentation.order)<1e-3 && s.presentation.group===l.presentation.group) continue;}
 if(n++<40){console.log("==",s.name,ks.join(","));for(const k of ks){console.log("  S:",canon(s[k]).slice(0,300));console.log("  L:",canon(l[k]).slice(0,300))}}
}
console.log(n);
