/** 🦴️ Prints the evaluated bone frames of an asset (catalog asset or `--file` candidate document) for one pose or clip instant, or the world position of bone-local points in every pose and clip sample.
 *
 * Usage: `bun rework-03-fk.ts <category>/<slug> | --file <doc.json> [--pose ID | --clip ID --t S] [--points "bone:u,v,w;bone:u,v,w"] [--all] [--bones a,b]`
 * Without `--points`: one line per bone `id o=(x,y,z) tip=(x,y,z) u=(..) v=(..) w=(..)`. With `--points`: world x (ahead), y (up), z (left) of each point; `--all` repeats it for every pose and for every clip at five instants.
 * @see ./r3-lib.ts */
import { readFileSync } from "node:fs";
import { cli, load, queryOf, rigAt, type Query } from "./r3-lib.ts";

const { spec, flag, has } = cli(["--file", "--pose", "--clip", "--t", "--points", "--bones"]);
const file = flag("--file");
const asset = file ? JSON.parse(readFileSync(file, "utf8")).asset : load(spec, "rework-03-fk.ts <asset>").asset;
const f = (n: number) => n.toFixed(4);
const v3 = (p: readonly number[]) => `(${p.map(f).join(",")})`;
const points = (flag("--points") ?? "").split(";").filter(Boolean).map((s) => ({ bone: s.split(":")[0], p: s.split(":")[1].split(",").map(Number) }));
const only = flag("--bones")?.split(",");
function show(query: Query, label: string): void {
  const rig = rigAt(asset, 50, query);
  if (points.length === 0) {
    for (const b of rig.bones) {
      if (only && !only.includes(b.id)) continue;
      const fr = rig.frames[b.id];
      console.log(`${b.id} o=${v3(fr.o)} tip=${v3(b.tip)} u=${v3(fr.u)} v=${v3(fr.v)} w=${v3(fr.w)}`);
    }
    return;
  }
  const out = points.map(({ bone, p }) => {
    const fr = rig.frames[bone];
    return `${bone}:${v3([0, 1, 2].map((i) => fr.o[i] + fr.u[i] * p[0] + fr.v[i] * p[1] + fr.w[i] * (p[2] ?? 0)))}`;
  });
  console.log(`${label} ${out.join(" ")}`);
}
if (has("--all")) {
  for (const pose of asset.poses) show({ kind: "pose", pose: pose.id }, pose.id);
  for (const clip of asset.clips) for (let i = 0; i < 5; i++) show({ kind: "clip", clip: clip.id, timeS: (clip.durationS * i) / 5 }, `${clip.id}@${((clip.durationS * i) / 5).toFixed(2)}`);
} else show(queryOf(asset, flag("--pose"), flag("--clip"), flag("--t")), "");
