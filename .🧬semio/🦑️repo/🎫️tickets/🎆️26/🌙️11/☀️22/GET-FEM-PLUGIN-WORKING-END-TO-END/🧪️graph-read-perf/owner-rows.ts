import {readFileSync} from "node:fs";
const m=JSON.parse(readFileSync(process.argv[2],"utf8"));
console.log("owners",m.owners.length,"obs",m.observations.length);
const kinds=new Map<string,number>();
const rows=m.owners.map((o:any)=>({s:o.suffix,n:o.rows.length}));
rows.sort((a:any,b:any)=>b.n-a.n);
console.log(JSON.stringify(rows.slice(0,15)));
console.log("sum",rows.reduce((a:number,b:any)=>a+b.n,0),"median",rows[rows.length>>1].n);
const o=m.owners[0];console.log(JSON.stringify(o.rows.slice(0,3)).slice(0,300));
