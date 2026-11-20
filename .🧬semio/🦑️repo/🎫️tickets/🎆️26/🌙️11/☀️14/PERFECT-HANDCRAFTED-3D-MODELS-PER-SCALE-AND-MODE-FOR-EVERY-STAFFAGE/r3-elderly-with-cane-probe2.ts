/** 🔬 Compares the planted root of the gait with and without the cane tip as a contact: hip x and the world x of the foot anchors over the cycle. */
import { rigAt } from "./r3-lib.ts";
import { entryOf, type Json } from "./volumes-lib.ts";

const { asset } = entryOf("humans/elderly-with-cane");
const feet = JSON.parse(JSON.stringify(asset)) as Json;
for (const c of feet.clips as Json[]) c.contacts = c.contacts.filter((x: Json) => x.anchor !== "cane-tip");
const world = (a: Json, t: number, anchorId: string): number => {
  const rig = rigAt(a, 50, { kind: "clip", clip: "cane-gait", timeS: t });
  const an = a.anchors.find((x: Json) => x.id === anchorId);
  const f = rig.frames[an.bone]!;
  return f.o[0] + f.u[0] * an.x + f.v[0] * an.y;
};
for (let i = 0; i <= 34; i++) {
  const t = i * 0.05;
  const row = [asset, feet].map((a) => `hip=${rigAt(a, 50, { kind: "clip", clip: "cane-gait", timeS: t }).frames.pelvis!.o[0].toFixed(3)} heelL=${world(a, t, "heel-l").toFixed(3)} ballL=${world(a, t, "ball-l").toFixed(3)} heelR=${world(a, t, "heel-r").toFixed(3)} ballR=${world(a, t, "ball-r").toFixed(3)}`);
  console.log(`t=${t.toFixed(2)}  real: ${row[0]}   feet-only: ${row[1]}`);
}
