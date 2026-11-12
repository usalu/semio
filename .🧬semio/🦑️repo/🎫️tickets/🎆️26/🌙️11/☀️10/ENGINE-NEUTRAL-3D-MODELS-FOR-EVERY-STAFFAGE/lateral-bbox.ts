import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { catalogEntries, loadAsset, toWorld, type Json, type V3 } from "./volume-lib.ts";
import { localMeshes } from "./lateral-posed.ts";
import { select } from "./lateral-snap.ts";
const slug = process.argv[2]!; const pose = process.argv[3];
const a = loadAsset(catalogEntries().find((x) => x.slug === slug)!.path);
const st = evaluate(a, { kind: "pose", pose: pose ?? a.presentation.defaultPose } as never, select(50)) as Json;
const arr = (p: Json) => [p.x, p.y, p.z] as V3;
const lo = [1e9, 1e9, 1e9], hi = [-1e9, -1e9, -1e9];
for (const t of localMeshes(a)) { const f = st.bones[t.bone].frame; const fr = { o: arr(f.o), u: arr(f.u), v: arr(f.v), w: arr(f.w) }; for (let i = 0; i < t.local.length; i += 3) { const p = toWorld(fr, [t.local[i]!, t.local[i + 1]!, t.local[i + 2]!]); for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, p[k]!); hi[k] = Math.max(hi[k]!, p[k]!); } } }
console.log(slug, pose ?? a.presentation.defaultPose, "posed volume size x,y,z:", [0, 1, 2].map((k) => +(hi[k]! - lo[k]!).toFixed(3)), "dimensions", a.dimensions.widthM, a.dimensions.heightM, a.dimensions.depthM, "x", lo[0]!.toFixed(3), hi[0]!.toFixed(3), "z", lo[2]!.toFixed(3), hi[2]!.toFixed(3));
for (const v of ["elevation", "front", "plan"]) { const p = (st.shapes[v] as Json[]).flatMap((s) => s.rings.flat()); console.log(" ", v, "2D extent", +(Math.max(...p.map((q: Json) => q.x)) - Math.min(...p.map((q: Json) => q.x))).toFixed(3), +(Math.max(...p.map((q: Json) => q.y)) - Math.min(...p.map((q: Json) => q.y))).toFixed(3)); }
