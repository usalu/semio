import { readFileSync } from "node:fs"; import { path } from "./sy.ts";
for (const id of process.argv.slice(2)) {
  const a = JSON.parse(readFileSync(path(id),"utf8")).asset;
  console.log("##",id,"anchors:",(a.anchors??[]).map((x:any)=>`${x.id}(${x.bone},${x.x},${x.y},z=${x.z})`).join(" "), "| supports:", JSON.stringify(a.structure?.supports ?? []).slice(0,600));
  for (const p of a.poses) for (const l of p.loads ?? []) console.log("  ",p.id, JSON.stringify(l).slice(0,260));
}
