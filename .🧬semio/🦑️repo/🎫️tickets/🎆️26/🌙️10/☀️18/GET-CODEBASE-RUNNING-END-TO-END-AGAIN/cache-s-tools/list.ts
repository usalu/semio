const j = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/aud-cachecov.json").text());
const [cls, name] = [Bun.argv[2], Bun.argv[3]];
for (const t of j.targets.filter((t:any)=>t.owner==="✏️s" && (cls==="*"||t.classProposal===cls) && (!name||t.name===name))) console.log(t.file, t.name, t.cache);
