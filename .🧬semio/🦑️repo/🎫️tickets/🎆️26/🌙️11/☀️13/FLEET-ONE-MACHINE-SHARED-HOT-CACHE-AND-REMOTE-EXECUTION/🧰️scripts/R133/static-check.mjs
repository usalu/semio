import {readFileSync,existsSync} from "node:fs";
import {dirname,resolve} from "node:path";
import {files,transform} from "./seam.mjs";
let changed=0,bad=0;
for(const f of files()){const t=readFileSync(f,"utf8"),o=transform(f,t);if(o===t)continue;changed++;
 const loader=/\.(ts|mts)$/u.test(f)?"ts":"js";
 try{new Bun.Transpiler({loader}).transformSync(o);}catch(e){bad++;console.log("PARSE",f,String(e).slice(0,200));}
 const m=/import\{machinePathV1\}from"([^"]+)";/u.exec(o);
 if(!m||!existsSync(resolve(dirname(f),m[1]))){bad++;console.log("IMPORT",f,m?.[1]);}
 const a=t.split("\n"),b=o.split("\n");let ai=0;
 if(b.length-a.length!==1){bad++;console.log("LINES",f,b.length-a.length);}
 const extra=b.filter(l=>l.startsWith("import{machinePathV1}"));if(extra.length!==1){bad++;console.log("IMPORTS",f,extra.length);}
 const rest=b.filter(l=>!l.startsWith("import{machinePathV1}"));
 rest.forEach((l,i)=>{if(l!==a[i]&&!l.includes("machinePathV1")){bad++;console.log("DIFF",f,i);}});
 if((o.match(/machinePathV1/gu)??[]).length<2){bad++;console.log("USE",f);}
}
console.log(JSON.stringify({changed,bad}));
