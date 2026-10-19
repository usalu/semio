const d = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/cache-rest-census.json").text());
const own = new Set(["🌎️hub","🎓️teaching","🏢️semio-tech","♻️mit-bestand","👴️leutwiler","root"]);
const skel = (c:string|null)=> (c??"").replace(/\s+/g," ").slice(0,160);
const m = new Map<string,any[]>();
for (const t of d.targets) if (own.has(t.owner)) { const k = skel(t.command).replace(/[A-Za-z0-9_-]+(?= native| oracle|$)/g,"").slice(0,60); }
const by:Record<string,number>={};
for (const t of d.targets) if (own.has(t.owner)) { const w=(t.command??"").replace(/^bun\s+/,"").split(/\s+/); const key=w.slice(0,3).join(" ").replace(/\/[^ ]*📜️script.ts/,"<S>"); by[key]=(by[key]??0)+1;}
for (const [k,v] of Object.entries(by).sort((a,b)=>b[1]-a[1])) console.log(v,k);
