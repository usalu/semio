import {readFileSync} from "node:fs";
const issues=JSON.parse(readFileSync("census.json","utf8")) as string[][];
const names=new Set(readFileSync("names.tsv","utf8").split("\n").map(l=>l.split("\t")[0]));
const R="/Users/ueli/Documents/semio/";
const c=new Map<string,number>();
for(const i of issues){if(i[1].includes("/.🧬semio/"))continue;const k=i[0]+" "+(i[0]==="NAME-MISMATCH"?i[2]+" => "+i[3]:i[2]+(i[0]==="PATH-NO-PKG"?" -> "+i[3].slice(R.length):""));c.set(k,(c.get(k)??0)+1);}
const rows=[...c].filter(([k])=>!k.startsWith("WS-MISSING")||!names.has(k.split(" ")[1]));
console.log(rows.length);for(const[k,v]of rows.sort((a,b)=>a[0]<b[0]?-1:1).filter(([k])=>!k.startsWith("NAME-MISMATCH")).slice(0,150))console.log(v,k.slice(0,230));
