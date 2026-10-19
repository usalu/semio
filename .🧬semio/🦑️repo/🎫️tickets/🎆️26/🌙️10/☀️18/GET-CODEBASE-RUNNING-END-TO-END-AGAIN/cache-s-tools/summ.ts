const j = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/aud-cachecov.json").text());
const rows = j.targets.filter((t:any)=>t.owner==="✏️s");
console.log(rows.length);
const by:any={};
for (const t of rows){ const k=t.classProposal+"|"+t.name+"|cache="+t.cache+"|in="+t.inputs.kind+"|out="+t.outputs.kind; by[k]=(by[k]??0)+1;}
for (const [k,v] of Object.entries(by).sort((a:any,b:any)=>b[1]-a[1])) console.log(v,k);
