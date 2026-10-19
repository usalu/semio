const d = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/cache-rest-census.json").text());
const m = new Map<string,number>();
for (const t of d.targets) if (["🌎️hub","🎓️teaching","🏢️semio-tech","♻️mit-bestand","👴️leutwiler","root"].includes(t.owner)) { const k=t.file; m.set(k,(m.get(k)??0)+1); }
for (const [k,v] of [...m].sort((a,b)=>b[1]-a[1])) console.log(v,k);
