/** 🖨️ Keeps a patched asset printable: as long as the print profile of the product refuses a scale (`requireDrawable`: a gap or a feature below its minimum on paper), the shapes that `ops/<id>.json` added at that scale and that a violation names are taken out of the ops again; ends when every scale is drawable or no violation names an added shape any more (those are printed).
 *  bun printable.ts <id>… */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { defaultDepictionOptions, depict } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
import { fileOf } from "./files.ts";
import { applyOps, readOps, writeOps, type Op } from "./ops.ts";

for (const id of process.argv.slice(2)) {
  const opsFile = `${import.meta.dir}/ops/${id}.json`;
  let ops: Op[] = await readOps(opsFile);
  let dropped = 0;
  for (let round = 0; round < 40; round++) {
    const { asset } = await Bun.file(fileOf(id)).json();
    applyOps(asset, ops);
    const remove = new Set<string>();
    const left: string[] = [];
    for (const scale of [50, 100, 200, 500].filter((d) => asset.representations[String(d)]?.status === "authored")) {
      const sel = { kind: "scale", scale: { denominator: scale } };
      try {
        const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, sel as never);
        depict(state, { ...(defaultDepictionOptions("light-solid" as never, "en", scale, "elevation" as never) as object), representation: sel } as never, {});
      } catch (e) {
        const violations = ((e as any).resolution?.suitability?.violations ?? []) as { check: string; shapes?: string[]; shape?: string }[];
        if (violations.length === 0) throw e;
        for (const v of violations) {
          const mine = (v.shapes ?? [v.shape!]).filter((shape) => ops.some((o) => o.op === "shape-add" && o.scale === String(scale) && o.shape.id === shape));
          if (mine.length === 0) left.push(`1:${scale} ${v.check} ${(v.shapes ?? [v.shape]).join("/")}`);
          for (const shape of mine) remove.add(`${scale}|${shape}`);
        }
      }
    }
    if (remove.size === 0) { console.log(`${id}: ${dropped} added shapes taken out again${left.length > 0 ? `; NOT printable: ${left.join("; ")}` : "; every scale drawable"}`); break; }
    ops = ops.filter((o) => !(o.op === "shape-add" && remove.has(`${o.scale}|${o.shape.id}`)));
    dropped += remove.size;
  }
  await writeOps(opsFile, ops);
}
