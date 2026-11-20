/** 🔍️ Which drawn shape a slender bamboo branch follows in the product's silhouette (C24): prints, for the branches of one bone at 1:50 in the elevation, the candidate shapes of that bone with their pen and mean distance. Usage: `bun r3-bamboo-clump-follow.ts [bone]` */
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { drawnOf } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { load } from "./r3-lib.ts";

const bone = process.argv[2] ?? "culm-2-c";
const { asset } = load("vegetation/bamboo-clump", "r3-bamboo-clump-follow.ts [bone]");
const state = evaluate(asset as never, { kind: "pose", pose: asset.presentation.defaultPose } as never, { kind: "scale", scale: { denominator: 50 } } as never) as never as { shapes: Record<string, { shape: { id: string; bone: string; role: string }; rings: { x: number; y: number }[][]; closed: boolean }[]> };
const ev = state.shapes.elevation!;
const drawn = drawnOf(ev as never, 50, asset.representations["50"].pens);
ev.forEach((s, i) => {
  if (s.shape.bone !== bone) return;
  const d = (drawn as never as { closed: boolean; filled: boolean; penM: number; rings: unknown[] }[])[i]!;
  console.log(s.shape.id, s.shape.role, "closed", s.closed, "filled", d.filled, "penM", d.penM, "rings", s.rings.length, "pts", s.rings.reduce((n, r) => n + r.length, 0));
});
const { followedWidth } = await import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts");
const br = ev.find((s) => s.shape.id === `${bone}-branches`)!;
for (const ring of br.rings.slice(0, 4)) console.log("followed width of a branch line:", followedWidth(bone, ring as never, drawn as never));
