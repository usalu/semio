import {readFileSync} from "node:fs";
const j=JSON.parse(readFileSync(process.argv[2],"utf8"));
const pct=(r:any)=>Math.max(r.bytes/r.lim.bytes,r.files/r.lim.files,r.lines/r.lim.lines);
const rows=[...j.rows].sort((a:any,b:any)=>pct(b)-pct(a)).slice(0,25);
for(const r of rows)console.log(`${(pct(r)*100).toFixed(1)}% ${r.name} | B ${r.bytes}/${r.lim.bytes} F ${r.files}/${r.lim.files} L ${r.lines}/${r.lim.lines} | ${r.dir}`);
const byName=(p:string)=>j.rows.filter((r:any)=>r.name.includes(p)).map((r:any)=>`${r.name} B${r.bytes} F${r.files} L${r.lines}`);
console.log("--- stdio-ifc:", byName("stdio-ifc").join(" ; "));
console.log("--- hub components:", byName("semio-hub-").map((x:string)=>x).join(" ; "));
console.log("--- framework-plugin:", byName("semio-framework-plugin-").length);
