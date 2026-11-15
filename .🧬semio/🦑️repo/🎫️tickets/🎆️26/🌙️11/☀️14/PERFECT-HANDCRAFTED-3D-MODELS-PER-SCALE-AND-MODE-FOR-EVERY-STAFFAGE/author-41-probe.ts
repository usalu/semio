import { openCtx, drawn } from "./kit-lib.ts";
const c = openCtx("animals/domestic-cat", 50);
const rep = (c.m as any).representations["50"].geometry;
for (const [v, id] of [["elevation", "near-thigh"], ["elevation", "chest"], ["front", "near-humerus"], ["front", "chest"], ["plan", "chest"], ["plan", "near-forearm"]] as const) {
  const sh = rep[v].find((s: any) => s.id === id);
  const d = drawn(c, v).find((s) => s.id === id)!;
  const f: any = c.state.frames[sh.bone];
  console.log(v, id, "bone", sh.bone, "o", f.o, "u", f.u, "v", f.v, "w", f.w);
  console.log(" local", JSON.stringify((sh.points ?? []).slice(0, 3)), sh.kind, JSON.stringify(sh).slice(0, 200));
  console.log(" world", JSON.stringify(d.rings[0]!.slice(0, 3)));
}
