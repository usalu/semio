import { entryOf, loadSource, merged, scaleMeshes, unionVolume } from "./volumes-lib.ts";
const spec = process.argv[2]!;
const { entry, asset } = entryOf(spec);
const m = merged(asset, loadSource(entry)!);
for (const S of [50, 100, 200, 500]) {
  const v = m.volumes[S]; if (v.status !== "authored") continue;
  const sm = scaleMeshes(m, S, v);
  const ids = v.body.map((s: any) => s.id);
  const parts: Record<string, number> = {};
  const total = unionVolume(sm.body, 300);
  const lobes = sm.body.filter((_, i) => /^(lobe|fill|crown)/.test(ids[i]));
  const wood = sm.body.filter((_, i) => !/^(lobe|fill|crown)/.test(ids[i]));
  console.log(S, "total", total.toFixed(2), "foliage", unionVolume(lobes, 300).toFixed(2), "wood", unionVolume(wood, 300).toFixed(2), process.argv[3] ? "" : "");
  void parts;
}
