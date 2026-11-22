/** 🔬️ Prints the evaluated front and elevation rings of the sling legs and the hook anchors of `reclaimed-floorboard-stack` in a pose: `bun author-probe-legs.ts <pose>`. */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const asset = JSON.parse(readFileSync(process.argv[3] ?? new URL("./🔣️.json", import.meta.url), "utf8")).asset;
const state = evaluate(asset, { kind: "pose", pose: process.argv[2] ?? "lifted" }, { kind: "scale", scale: { denominator: 50 } } as never);
for (const view of ["elevation", "front"] as const)
  for (const item of state.shapes[view] as readonly { shape: { id: string }; rings: readonly (readonly { x: number; y: number }[])[] }[])
    if (item.shape.id.startsWith("leg-")) {
      const pts = item.rings.flat();
      console.log(`[DEBUG] ${view} ${item.shape.id} x ${Math.min(...pts.map((p) => p.x)).toFixed(3)}..${Math.max(...pts.map((p) => p.x)).toFixed(3)} y ${Math.min(...pts.map((p) => p.y)).toFixed(3)}..${Math.max(...pts.map((p) => p.y)).toFixed(3)}`);
    }
for (const id of ["hook-a", "hook-b"]) console.log(`[DEBUG] ${id}`, JSON.stringify(state.anchors[id]?.at), state.anchors[id]?.z);
for (const b of ["leg-a1", "leg-a2"]) console.log(`[DEBUG] bone ${b}`, JSON.stringify(state.bones[b]?.frame));
