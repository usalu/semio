const j = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/cache-s-before.json").text());
import { keepReason } from "./rules.ts";
const flips:any={}, keeps:any={};
for (const t of j.targets.filter((t:any)=>t.owner==="✏️s")) {
  const off = t.classProposal==="deterministic-but-cache-off" || t.classProposal==="unclassified-cache-off";
  if(!off) continue;
  const r = keepReason(t.name);
  if (r) (keeps[t.name+" :: "+r]??=[]).push(t.file); else (flips[t.name]??=[]).push(t.file);
}
console.log("FLIP names", Object.keys(flips).length, "targets", Object.values(flips).flat().length);
console.log("KEEP", Object.entries(keeps).map(([k,v]:any)=>v.length+" "+k).join("\n"));
console.log(Object.entries(flips).map(([k,v]:any)=>v.length+" "+k).join("\n"));
