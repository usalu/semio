import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const idx=JSON.parse(brotliDecompressSync(readFileSync(process.argv[2])).toString()),census=JSON.parse(readFileSync(process.argv[3],"utf8"));
const ownersOf=new Map();idx.classes.forEach(c=>{for(const r of c.r){let s=ownersOf.get(r);if(!s)ownersOf.set(r,s=new Set());for(const o of c.o)s.add(o);}});
const by={};for(const s of census.stale){(by[s.kind]??=new Set());for(const o of ownersOf.get(s.i)??[])by[s.kind].add(o);}
const all=new Set();for(const k in by){console.log(k,by[k].size);for(const o of by[k])all.add(o);}
const nonAnalysis=new Set();for(const k in by)if(k!=="analysis")for(const o of by[k])nonAnalysis.add(o);
console.log("all",all.size,"non-analysis",nonAnalysis.size,"owners",idx.owners.length);
const small=new Set();for(const s of census.stale)if(s.owners<=300)for(const o of ownersOf.get(s.i)??[])small.add(o);
console.log("rows with <=300 owners union",small.size);
