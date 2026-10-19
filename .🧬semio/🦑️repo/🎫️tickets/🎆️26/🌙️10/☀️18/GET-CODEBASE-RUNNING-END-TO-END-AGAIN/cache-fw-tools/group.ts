const j = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/aud-cachecov.json").text());
const T = j.targets.filter((t:any)=>t.owner==="🧰️framework");
const cls = process.argv[2];
const m = new Map<string, any[]>();
for (const t of T.filter((t:any)=>t.classProposal===cls)) { const k=t.name; if(!m.has(k)) m.set(k,[]); m.get(k)!.push(t); }
for (const [k,v] of [...m].sort((a,b)=>b[1].length-a[1].length)) console.log(v.length, k, "|", (v[0].command||"").slice(0,110), "|", v[0].file.slice(-70));
