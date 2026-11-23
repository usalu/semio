import {readFileSync} from "node:fs";
const m=JSON.parse(readFileSync(process.argv[2],"utf8"));
const rows=m.owners.map((o:any)=>{const at=o.suffix.indexOf("\0");return {n:o.suffix.slice(0,at),t:JSON.parse(o.suffix.slice(at+1))[0].length,i:JSON.parse(o.suffix.slice(at+1))[1].length,r:o.rows.length};}).filter((o:any)=>o.t>0).sort((a:any,b:any)=>a.r-b.r);
console.log(rows.length,JSON.stringify(rows.filter((_:any,i:number)=>i%12===0)));
