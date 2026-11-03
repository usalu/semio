import { catalog } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
const rows: string[] = [];
for (const a of catalog()) {
  const c = a.anchors.filter((x) => x.kind === "contact" || x.kind === "support");
  const zs = new Set(c.map((x) => (x.z ?? 0).toFixed(3)));
  if (c.length >= 3) rows.push(`${a.id}\t${c.length}\tz:${[...zs].join(",")}`);
}
console.log("[DEBUG] assets with >= 3 supports", rows.length);
for (const r of rows) console.log("[DEBUG]", r);
