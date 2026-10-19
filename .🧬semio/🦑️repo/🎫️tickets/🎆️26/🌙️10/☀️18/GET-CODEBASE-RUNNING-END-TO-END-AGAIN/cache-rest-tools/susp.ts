const d = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/cache-rest-census.json").text());
const own = new Set(["🌎️hub","🎓️teaching","🏢️semio-tech","♻️mit-bestand","👴️leutwiler","root"]);
for (const t of d.targets) if (own.has(t.owner) && t.classProposal==="deterministic-cacheable" && t.flags.includes("outputs-empty-suspect")) console.log(t.file.replace("/📋️project.json","").slice(-60), "|", t.name, "|", (t.command??"").replace("bun ./📜️script.ts ","").slice(0,70));
