const a = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/cache-rest-census.json").text());
const b = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/cache-rest-census-after.json").text());
const own = new Set(["🌎️hub","🎓️teaching","🏢️semio-tech","♻️mit-bestand","👴️leutwiler","root"]);
const tally = (d:any)=>{ const c:any={}; for (const t of d.targets) if(own.has(t.owner)) c[t.classProposal]=(c[t.classProposal]??0)+1; const f:any={}; for (const t of d.targets) if(own.has(t.owner)&&(t.classProposal==="deterministic-cacheable"||t.classProposal==="deterministic-but-cache-off")) for (const x of t.flags) f[x]=(f[x]??0)+1; return {c,f}; };
console.log("BEFORE", JSON.stringify(tally(a))); console.log("AFTER", JSON.stringify(tally(b)));
const key=(t:any)=>t.file+"::"+t.name; const bm=new Map(b.targets.map((t:any)=>[key(t),t]));
if (process.argv[2]==="left") for (const t of b.targets) if (own.has(t.owner) && (t.classProposal==="deterministic-but-cache-off"||t.classProposal==="unclassified-cache-off")) console.log(t.classProposal, t.file.slice(-70), t.name, "c="+t.cache);
