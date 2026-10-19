const b = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/cache-rest-census-after.json").text());
const own = new Set(["🌎️hub","🎓️teaching","🏢️semio-tech","♻️mit-bestand","👴️leutwiler","root"]);
const t=b.targets.filter((x:any)=>own.has(x.owner));
console.log(t.length, new Set(t.map((x:any)=>x.file)).size);
const o=b.targets.filter((x:any)=>own.has(x.owner)); const c:any={}; for(const x of o) c[x.classProposal]=(c[x.classProposal]??0)+1; console.log(JSON.stringify(c));
