import { loadAsset, resolveAsset, restState } from "./volumes-base.ts";
const a = loadAsset(resolveAsset(process.argv[2]!).path);
for (const s of [50, 100, 200, 500]) {
  if (a.representations[String(s)].status === "unavailable") continue;
  const st: any = restState(a, s);
  for (const v of ["elevation", "front", "plan"]) {
    const ids = new Set<string>();
    for (const e of st[v]) if (e.closed && e.shape.role === "body" && e.rings.length) ids.add(e.shape.id + "@" + e.shape.bone);
    console.log(`${s} ${v} (${ids.size}): ${[...ids].join(" ")}`);
  }
}
