import{readFileSync,readdirSync,statSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const g="/Users/ueli/Documents/semio/.cache/b/g/m/generations",ld=f=>JSON.parse(brotliDecompressSync(readFileSync(f)).toString());
const ms=readdirSync(join(g,"m")).map(f=>[f,statSync(join(g,"m",f)).mtimeMs]).sort((a,b)=>a[1]-b[1]).map(r=>JSON.parse(readFileSync(join(g,"m",r[0]))));
const pick=(a,b)=>[ms.find(m=>m.generation===a),ms.find(m=>m.generation===b)];
console.log(ms.map(m=>m.generation));
const A=Number(process.argv[2]),B=Number(process.argv[3]);const[ma,mb]=pick(A,B);if(!ma||!mb){console.log("gen missing");process.exit(0)}
const tokens=(m,name)=>{const f=ld(join(g,"o",m.owners[name].fragment+".json.br"));const s=JSON.stringify(f.node);return[...new Set(s.match(/SEMIO_SOURCE_CLOSURE_[a-f0-9]{64}/g)??[])].sort();};
const names=Object.keys(ma.owners).filter(n=>mb.owners[n]&&ma.owners[n].kind==="project");
let n=0,changed=0;const diffs=[];const roster=t=>{try{return JSON.parse(readFileSync("/Users/ueli/Documents/semio/.cache/b/g/m/build-input-closures/"+t.slice(21)+".json")).files}catch{return null}};
for(let i=0;i<names.length;i+=Math.ceil(names.length/150)){const name=names[i];n++;const ta=tokens(ma,name),tb=tokens(mb,name);if(JSON.stringify(ta)!==JSON.stringify(tb)){changed++;const ra=new Set(ta.flatMap(t=>roster(t)??[])),rb=new Set(tb.flatMap(t=>roster(t)??[]));diffs.push([[...rb].filter(x=>!ra.has(x)).length,[...ra].filter(x=>!rb.has(x)).length]);}}
console.log("sampled",n,"token set changed",changed);console.log(diffs.slice(0,12).join(" | "));
const add=diffs.map(d=>d[0]),rem=diffs.map(d=>d[1]);console.log("added files max/avg",Math.max(...add,0),add.reduce((a,b)=>a+b,0)/Math.max(add.length,1),"removed",Math.max(...rem,0));
