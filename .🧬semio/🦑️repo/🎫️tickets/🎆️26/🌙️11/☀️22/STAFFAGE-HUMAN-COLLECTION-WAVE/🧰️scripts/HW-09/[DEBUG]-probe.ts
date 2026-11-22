import { loadHuman, stateOf, pointOf } from "./kin.ts";
const { asset } = await loadHuman(process.argv[2]!);
const p = asset.poses.find((x: any) => x.id === process.argv[3]);
const s = stateOf(asset, p.bones, p.root);
for (const a of process.argv.slice(4)) { const [bone, ...l] = a.split(","); const q = l.length ? pointOf(s, { bone, local: l.map(Number) as any }) : pointOf(s, { anchor: bone }); console.log("[DEBUG]", a, q.x.toFixed(3), q.y.toFixed(3), q.z.toFixed(3)); }
console.log("[DEBUG] offsets", JSON.stringify(Object.fromEntries(Object.entries(p.bones).filter(([k]) => /arm|hand/.test(k)))));
