import {readFileSync} from "node:fs";
const [file,rootsFile]=process.argv.slice(2);
const j=JSON.parse(readFileSync(file,"utf8"));
const roots=new Set(readFileSync(rootsFile,"utf8").split("\n").map(s=>s.trim()).filter(Boolean));
const act=new Set(readFileSync(rootsFile.replace("roots.txt","activation-names.txt"),"utf8").split("\n").map(s=>s.trim()).filter(Boolean));
const viol=[];let errs=[];let limRaised=[];
for(const r of j.rows){
  const o=r.over;
  const isV=o.bytes>0||o.files>0||o.lines>0;
  if(r.err)errs.push(r);
  if(r.lim.bytes!==262144||r.lim.files!==32||r.lim.lines!==4000)limRaised.push(r);
  if(isV)viol.push(r);
}
const actReached=(r)=>{let c=r.chain;return c.some(n=>act.has(n))};
console.log(JSON.stringify({closure:j.closure,rows:j.rows.length,violators:viol.length,errRows:errs.length,limitMismatch:limRaised.length},null,0));
for(const r of viol.sort((a,b)=>b.over.bytes-a.over.bytes)){console.log(`V ${r.name} | ${r.dir} | B ${r.bytes}/${r.lim.bytes} (over ${r.over.bytes}) | F ${r.files}/${r.lim.files} (over ${r.over.files}) | L ${r.lines}/${r.lim.lines} (over ${r.over.lines}) | optOnly=${r.viaOptionalOnly} | chain=${r.chain.join(" < ")} | actChain=${actReached(r)}`);}
for(const r of errs)console.log(`ERR ${r.name} | ${r.dir} | ${r.err}`);
for(const r of limRaised)console.log(`LIM ${r.name} | ${JSON.stringify(r.lim)}`);
