const d = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/cache-rest-census.json").text());
const own = new Set(["🌎️hub","🎓️teaching","🏢️semio-tech","♻️mit-bestand","👴️leutwiler","root"]);
for (const t of d.targets) if (own.has(t.owner) && t.classProposal==="deterministic-cacheable" && (t.cache==="absent"||t.outputs.kind==="absent")) console.log(t.file.replace("/📋️project.json","").replace("/📦️packages/🦀️rust","/RS").slice(-45), "|", t.name, "| c="+t.cache, "o="+t.outputs.kind, "|", (t.command??"").replace("bun ./📜️script.ts ","").slice(0,60));
