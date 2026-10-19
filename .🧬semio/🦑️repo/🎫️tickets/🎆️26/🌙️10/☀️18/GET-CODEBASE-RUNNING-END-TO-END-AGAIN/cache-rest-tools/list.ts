const d = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/cache-rest-census.json").text());
const owners = new Set(["🌎️hub","🎓️teaching","🏢️semio-tech","♻️mit-bestand","👴️leutwiler","root"].map(s=>s));
const rows = d.targets.filter((t:any)=>owners.has(t.owner));
const cls = process.argv[2];
for (const t of rows.filter((t:any)=>!cls||t.classProposal===cls)) console.log([t.owner,t.project,t.name,t.classProposal,"c="+t.cache,"in="+t.inputs.declaredKind+"/"+t.inputs.effectiveKind,"out="+t.outputs.kind,t.flags.join(",")].join(" | "));
