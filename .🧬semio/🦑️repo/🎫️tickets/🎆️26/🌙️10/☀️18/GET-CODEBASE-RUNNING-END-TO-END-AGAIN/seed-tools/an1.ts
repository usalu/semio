const R="/Users/ueli/Documents/semio/";
const L=Bun.JSONC.parse(await Bun.file(R+".vscode/launch.json").text());
const S=Bun.JSONC.parse(await Bun.file(R+".vscode/🧩️launch.seed.jsonc").text());
const canon=(v:any):string=>v===undefined?"U":Array.isArray(v)?"["+v.map(canon).join(",")+"]":v&&typeof v==="object"?"{"+Object.keys(v).sort().map(k=>JSON.stringify(k)+":"+canon(v[k])).join(",")+"}":JSON.stringify(v);
const lc:any[]=L.configurations, sc:any[]=S.configurations;
const lnames=new Map<string,number[]>(); lc.forEach((c,i)=>{const a=lnames.get(c.name)??[];a.push(i);lnames.set(c.name,a)});
const snames=new Map<string,number[]>(); sc.forEach((c,i)=>{if(typeof c==='string')return;const a=snames.get(c.name)??[];a.push(i);snames.set(c.name,a)});
console.log("launch",lc.length,"names",lnames.size,"seed",sc.length,"names",snames.size);
const lOnly=[...lnames.keys()].filter(n=>!snames.has(n));
const sOnly=[...snames.keys()].filter(n=>!lnames.has(n));
console.log("launch-only names",lOnly.length,"seed-only names",sOnly.length);
const pre=(n:string)=>n.replace(/\s.*/,"").slice(0,6);
const cnt=(a:string[])=>{const m:Record<string,number>={};for(const n of a){const k=pre(n);m[k]=(m[k]??0)+1}return Object.entries(m).sort((x,y)=>y[1]-x[1]).slice(0,15)};
console.log("lonly prefixes",JSON.stringify(cnt(lOnly)));
console.log("sonly prefixes",JSON.stringify(cnt(sOnly)));
// positional: first launch-only index
const lidx=lOnly.map(n=>lnames.get(n)![0]).sort((a,b)=>a-b);
console.log("launch-only index ranges first",lidx.slice(0,5),"last",lidx.slice(-5), "count",lidx.length);
// contiguous blocks
let blocks:number[][]=[];for(const i of lidx){const b=blocks[blocks.length-1];if(b&&b[1]===i-1)b[1]=i;else blocks.push([i,i])}
console.log("blocks",blocks.length,JSON.stringify(blocks.slice(0,40)));
console.log("sonly samples",sOnly.slice(0,30));
console.log("lonly samples", lOnly.filter((_,i)=>i%200==0).slice(0,30));
// differences among shared names
let diffc=0, same=0; const dk:Record<string,number>={};
for(const [n,li] of lnames){const si=snames.get(n); if(!si)continue; const ls=li.map(i=>canon(lc[i])); const ss=si.map(i=>canon(sc[i])); if(ss.every(x=>ls.includes(x))&&ls.every(x=>ss.includes(x)))same++; else {diffc++; const a=lc[li[0]],b=sc[si[0]]; const ks=new Set([...Object.keys(a),...Object.keys(b)]); const d=[...ks].filter(k=>canon(a[k])!==canon(b[k])).join(","); dk[d]=(dk[d]??0)+1}}
console.log("shared same",same,"diff",diffc,JSON.stringify(Object.entries(dk).sort((x,y)=>y[1]-x[1]).slice(0,15)));
