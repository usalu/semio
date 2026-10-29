import {readFileSync} from "node:fs";
import {dirname} from "node:path";
const REPO="/Users/ueli/Documents/semio";
const list=readFileSync(process.argv[2],"utf8").split("\n").filter(Boolean);
const D={bytes:262144,files:32,lines:4000};
let n=0;const raised:string[]=[],odd:string[]=[],all:string[]=[];
for(const m of list){
  let t:any;try{t=Bun.TOML.parse(readFileSync(REPO+"/"+m.replace(/^\.\//,""),"utf8"));}catch{continue;}
  const b=t.package?.metadata?.semio?.["rust-source-boundary"];
  if(!b)continue;n++;
  const name=t.package?.name??"?";
  const row=`${name} | ${dirname(m)} | bytes=${b.maxSourceBytes} files=${b.maxSourceFiles} lines=${b.maxActiveLines} keys=${Object.keys(b).join(",")}`;
  all.push(row);
  if((b.maxSourceBytes??0)>D.bytes||(b.maxSourceFiles??0)>D.files||(b.maxActiveLines??0)>D.lines)raised.push(row);
  const ks=Object.keys(b);if(ks.length!==3||!["maxSourceBytes","maxSourceFiles","maxActiveLines"].every(k=>ks.includes(k)))odd.push(row);
}
console.log(`DECLARED ${n}`);
console.log(`RAISED ${raised.length}`);for(const r of raised)console.log("R "+r);
console.log(`ODD-KEYS ${odd.length}`);for(const r of odd)console.log("O "+r);
