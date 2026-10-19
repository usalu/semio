const d = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/cache-rest-census.json").text());
const own = new Set(["🌎️hub","🎓️teaching","🏢️semio-tech","♻️mit-bestand","👴️leutwiler","root"]);
for (const t of d.targets) if (own.has(t.owner)) { if ((t.cache==="true" && !t.effectiveCache) || (t.cache==="absent" && !t.effectiveCache && !t.effectiveContinuous)) console.log(t.owner, t.project, t.name, "raw="+t.cache, "eff="+t.effectiveCache, t.classProposal); }
