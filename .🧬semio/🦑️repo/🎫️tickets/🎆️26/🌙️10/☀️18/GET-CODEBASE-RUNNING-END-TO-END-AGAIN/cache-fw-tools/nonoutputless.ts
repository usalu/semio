const j = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/cache-fw-census-before.json").text());
const T = j.targets.filter((t:any)=>t.owner==="🧰️framework" && ["deterministic-cacheable","deterministic-but-cache-off","unclassified-cache-off"].includes(t.classProposal));
const OL=/^(test|check|lint|typecheck|format-check|verify|stdio|graph-check|policy-check|artifact-check|artifact-package-contract|test-contract)(-|$)|-check$|-contract$/;
for (const t of T) if(!OL.test(t.name)) console.log(t.classProposal.slice(0,12), t.cache, "out="+t.outputs.kind, "|", t.project, ":", t.name, "|", (t.command||"").replace(/\s+/g," ").slice(0,140));
