const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { readFileSync, readdirSync } = await import("node:fs");
for (const cat of ["🪑️objects", "🛋️furniture"]) for (const slug of readdirSync(`${L}/🗂️catalog/${cat}`)) {
  const a = JSON.parse(readFileSync(`${L}/🗂️catalog/${cat}/${slug}/🔣️.json`, "utf8")).asset;
  console.log(slug, a.dimensions?.depthM ?? "", a.anchors.filter((x:any)=>x.kind!=="emitter").map((x:any)=>`${x.id}@${x.bone}(${x.x},${x.y},${x.z??0})${x.support??x.kind[0]}${x.footprintLengthM?`[${x.footprintLengthM}x${x.footprintWidthM}]`:""}`).join(" "));
}
