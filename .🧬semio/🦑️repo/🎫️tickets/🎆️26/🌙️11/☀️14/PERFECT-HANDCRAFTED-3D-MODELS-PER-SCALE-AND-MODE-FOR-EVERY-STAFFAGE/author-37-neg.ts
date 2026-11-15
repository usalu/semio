import { entryOf, loadSource, meshesAt } from "./volumes-lib.ts";
const { entry, asset } = entryOf(process.argv[2]!);
const src = loadSource(entry)!;
for (const scale of ["50", "100", "200", "500"]) {
  const v = src.volumes[scale];
  if (!v || v.status !== "authored") continue;
  for (const list of ["body", "structure", "thermal"]) {
    const solids = v[list].filter((s: any) => s.of === undefined);
    const ms = meshesAt(asset, Number(scale), solids);
    solids.forEach((s: any, i: number) => {
      const m: any = ms[i]!;
      const p = m.world;
      let vol = 0;
      for (let t = 0; t < m.indices.length; t += 3) {
        const q = [0, 1, 2].map((k) => [p[m.indices[t + k] * 3], p[m.indices[t + k] * 3 + 1], p[m.indices[t + k] * 3 + 2]]);
        vol += (q[0][0] * (q[1][1] * q[2][2] - q[1][2] * q[2][1]) - q[0][1] * (q[1][0] * q[2][2] - q[1][2] * q[2][0]) + q[0][2] * (q[1][0] * q[2][1] - q[1][1] * q[2][0])) / 6;
      }
      if (vol < 0 && !m.open) console.log(scale, list, s.id, s.kind, vol.toExponential(2));
    });
  }
}
console.log("done");
