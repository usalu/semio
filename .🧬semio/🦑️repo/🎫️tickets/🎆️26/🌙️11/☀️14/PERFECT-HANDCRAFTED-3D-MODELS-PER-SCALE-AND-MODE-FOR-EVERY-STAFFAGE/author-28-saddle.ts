import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const R = "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/";
const sil: any = await import(R + "🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts");
const meas: any = await import(R + "🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts");
const ev: any = await import(R + "🧮️evaluation/🟦️.ts");
const { entry, asset } = entryOf("humans/cyclist");
const m: any = merged(asset, loadSource(entry)!);
for (const sc of [50, 100]) {
  const state = ev.evaluate(m, { kind: "pose", pose: m.presentation.defaultPose }, { kind: "scale", scale: { denominator: sc } });
  const all = sil.worldSolids(m, state.bones, sc);
  const saddle = all.filter((w: any) => w.id === "saddle");
  const rest = all.filter((w: any) => w.id !== "saddle");
  console.log(sc, "saddle closed surface", meas.outerArea(saddle).toFixed(4), "exposed among all", meas.exposedArea(saddle, all).toFixed(4));
  const base = meas.exposedArea(saddle, all);
  for (const drop of [...new Set(all.map((w: any) => w.id))] as string[]) {
    const others = all.filter((w: any) => w.id !== drop);
    { const v = meas.exposedArea(saddle, others); if (v - base > 0.0005) console.log("   without", drop, v.toFixed(4)); }
  }
}
