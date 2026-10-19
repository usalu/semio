const d = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/cache-rest-census.json").text());
const own = new Set(["🌎️hub","🎓️teaching","🏢️semio-tech","♻️mit-bestand","👴️leutwiler","root"]);
const rows=d.targets.filter((t:any)=>own.has(t.owner));
const c:any={};
for (const t of rows){ if(t.classProposal!=="deterministic-cacheable")continue; const k=`cache=${t.cache} out=${t.outputs.kind} in=${t.inputs.declaredKind}`; c[k]=(c[k]??0)+1;}
console.log(c);
