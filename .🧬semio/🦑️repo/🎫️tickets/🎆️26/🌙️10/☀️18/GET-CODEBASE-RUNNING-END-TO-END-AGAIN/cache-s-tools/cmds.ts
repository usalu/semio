const j = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/aud-cachecov.json").text());
const by:any={};
for (const t of j.targets.filter((t:any)=>t.owner==="✏️s")) { const lang=t.file.includes("🦀️rust")?"rs":t.file.includes("🟦️typescript")?"ts":"x"; const k=lang+"|"+t.classProposal.slice(0,14)+"|"+t.name+"|"+(t.command??"").replace(/^bun \.\/📜️script\.ts /,"").slice(0,70); (by[k]??=[]).push(t.file);}
for (const [k,v] of Object.entries(by).sort() as any) console.log(v.length,k);
