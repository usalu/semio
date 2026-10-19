const j = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/aud-cachecov.json").text());
const projs = new Map<string,any>();
for (const t of j.targets.filter((t:any)=>t.owner==="✏️s")) { const k=t.file; (projs.get(k)??projs.set(k,{file:k,ts:[]}).get(k)).ts.push(t); }
const kinds:any={};
for (const [f,p] of projs){ const lang=f.includes("🦀️rust")?"rust":f.includes("🟦️typescript")?"ts":"other:"+f.split("/").slice(-3,-1).join("/"); kinds[lang]=(kinds[lang]??0)+1;}
console.log(projs.size, kinds);
