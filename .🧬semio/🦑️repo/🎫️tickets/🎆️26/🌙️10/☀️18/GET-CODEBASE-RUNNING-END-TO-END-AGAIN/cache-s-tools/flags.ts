const j = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/cache-s-before.json").text());
const by:any={};
for (const t of j.targets.filter((t:any)=>t.owner==="✏️s")) for (const f of t.flags) { const k=f+"|"+t.name; (by[k]??=[]).push(t.file); }
const OUTPUTLESS = /^(test|check|lint|typecheck|format-check|verify|stdio|graph-check|policy-check|artifact-check|artifact-package-contract|test-contract)(-|$)/;
for (const [k,v] of Object.entries(by).sort() as any){ const [f,n]=k.split("|"); if((f==="no-outputs"||f==="outputs-empty-suspect") && !OUTPUTLESS.test(n)) console.log(v.length,k); }
