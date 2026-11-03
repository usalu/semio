import { load } from "./lib.ts";
for (const id of process.argv.slice(2)) {
  const a = load(id).asset;
  console.log("##", id, JSON.stringify(a.dimensions), "root", a.rig.root);
  for (const n of a.anchors ?? []) console.log("  ", n.kind.padEnd(8), n.id.padEnd(24), n.bone.padEnd(16), n.x, n.y, "z=" + (n.z ?? "-"), n.tiltDeg ?? "");
  for (const p of a.poses ?? []) { const c = p.contacts ?? []; const l = p.loads ?? []; console.log("   pose", p.id, "contacts", JSON.stringify(c), "loads", l.map((x: any) => `${x.anchor ?? x.id}:${["fxN","fyN","fzN","momentNm","momentXNm","momentYNm","wzNPerM"].filter(k=>x[k]).map(k=>k+"="+x[k]).join(",")}`).join(" | ")); }
}
