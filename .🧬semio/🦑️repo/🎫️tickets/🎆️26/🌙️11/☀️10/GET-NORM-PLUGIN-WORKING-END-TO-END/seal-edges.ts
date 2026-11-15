import{closureLiteralSitesV1}from"../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🟦️.ts";
import{readFileSync}from"node:fs";import{resolve,dirname,relative}from"node:path";
const rows=JSON.parse(readFileSync(process.argv[2]!,"utf8")) as [string,number,string,string|null][];
const set=new Set(rows.map(r=>r[0])),edges:Record<string,{to:string;kind:string}[]>={};
for(const[p]of rows){if(!/\.[cm]?[jt]sx?$/.test(p))continue;let src="";try{src=readFileSync(p,"utf8").replace(/^#![^\r\n]*/u,"");}catch{continue;}
 let sites;try{sites=closureLiteralSitesV1(p,src);}catch{continue;}
 for(const s of sites){if(!s.specifier.startsWith("."))continue;const t=relative(".",resolve(dirname(p),s.specifier));if(set.has(t))(edges[p]??=[]).push({to:t,kind:s.kind});}}
await Bun.write(process.argv[3]!,JSON.stringify(edges));
