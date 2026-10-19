const j = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/cache-fw-census-before.json").text());
const T = j.targets.filter((t:any)=>t.owner==="🧰️framework");
const re = new RegExp(process.argv[2]);
for (const t of T) if (re.test(t.name) || re.test(t.command||"")) console.log(t.classProposal.slice(0,10), t.cache, t.project, ":", t.name, "|", (t.command||"").replace(/\s+/g," ").slice(0,150));
