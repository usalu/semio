import { load } from "../machines/lib.ts";
import { ALL } from "./ids.ts";
const only = (process.env.IDS ?? "").split(",").filter(Boolean);
for (const id of ALL) {
  if (only.length && !only.includes(id)) continue;
  const d = load(id).asset;
  console.log(`## ${id} dims=${JSON.stringify(d.dimensions)}`);
  console.log(JSON.stringify(d.structure, null, 0));
  for (const p of d.poses ?? []) if (p.contacts) console.log(" pose", p.id, JSON.stringify(p.contacts));
  for (const c of d.clips ?? []) if (c.contacts) console.log(" clip", c.id, JSON.stringify(c.contacts));
}
