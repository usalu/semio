/** 🖨️ Whether every authored scale of an asset can still be drawn in its three views in the default pose (the print profile of the product finds no gap or feature below its minimum, `requireDrawable`), with the violations when not: bun drawable.ts <id|file.json>… [--ops ops.json] */
import { parseArgs } from "node:util";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { defaultDepictionOptions, depict } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
import { fileOf } from "./files.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { ops: { type: "string" } } });
for (const arg of positionals) {
  const { asset } = await Bun.file(fileOf(arg)).json();
  if (values.ops !== undefined) (await import("./ops.ts")).applyOps(asset, JSON.parse(await Bun.file(values.ops).text()));
  const bad: string[] = [];
  for (const scale of [50, 100, 200, 500].filter((d) => asset.representations[String(d)]?.status === "authored")) {
    const sel = { kind: "scale", scale: { denominator: scale } };
    try {
      const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, sel as never);
      for (const viewpoint of ["elevation", "front", "plan"]) depict(state, { ...(defaultDepictionOptions("light-solid" as never, "en", scale, viewpoint as never) as object), representation: sel } as never, {});
    } catch (e) {
      const violations = ((e as any).resolution?.suitability?.violations ?? []) as { check: string; shapes?: string[]; shape?: string; measuredMm: number; requiredMm: number }[];
      bad.push(`1:${scale} ${violations.length} violations: ${violations.slice(0, 6).map((v) => `${v.check} ${(v.shapes ?? [v.shape]).join("/")} ${v.measuredMm.toFixed(3)}<${v.requiredMm}`).join("; ")}${violations.length === 0 ? String((e as Error).message) : ""}`);
    }
  }
  console.log(`${asset.id}: ${bad.length === 0 ? "drawable at every scale" : bad.join(" | ")}`);
}
