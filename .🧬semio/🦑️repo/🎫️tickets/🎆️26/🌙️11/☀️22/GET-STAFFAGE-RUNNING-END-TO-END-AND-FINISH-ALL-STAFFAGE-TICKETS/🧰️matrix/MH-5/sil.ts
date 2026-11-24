/** 🌓️ The silhouette guard of several variants of one asset in one process: `bun sil.ts --scales 50,100 [--views front,plan] <file.json>…` (from `S/📦️packages/🟦️typescript`) measures every file with `measureView` of `🧊️catalog-silhouette` (IoU and largest boundary distance per view against the gate, as `../MR/measure.ts` prints them) and builds the posed volume only once per scale: the files must be variants of ONE asset whose volumes, rig and poses are the same (only drawn shapes differ). A `!` marks a failing value; the line ends with `OK` or `FAIL`.
 * @see ../MR/measure.ts — the same numbers for one file, with validateAsset and the three-views sweep */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { measureView, worldSolids, VIEWPOINTS } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { featureOf } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts"));
const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.includes(`--${name}`) ? args[args.indexOf(`--${name}`) + 1] : undefined);
const files = args.filter((arg, i) => !arg.startsWith("--") && !(i > 0 && args[i - 1]!.startsWith("--")));
const views: string[] = flag("views")?.split(",") ?? [...VIEWPOINTS];
const first = readDoc(files[0]!).doc.asset;
for (const scale of (flag("scales") ?? "50,100,200,500").split(",").map(Number)) {
  if (first.volumes?.[String(scale)]?.status !== "authored" || first.representations[String(scale)]?.status !== "authored") continue;
  const feature = featureOf(scale);
  const query = { kind: "pose", pose: first.presentation.defaultPose };
  const selector = { kind: "scale", scale: { denominator: scale } };
  const solids = worldSolids(first, evaluate(first, query, selector).bones, scale).filter((w: any) => w.texture !== true);
  for (const file of files) {
    const asset = readDoc(file).doc.asset;
    const state = evaluate(asset, query, selector);
    let bad = 0;
    const text = views.map((view) => {
      const m = measureView(asset, state, solids, view, scale, undefined, feature);
      return `${view} ${m.iou.toFixed(4)}${m.iou < 0.92 ? (bad++, "!") : ""}/${(m.hausdorffM * 1000).toFixed(1)}mm${m.hausdorffM > 2 * feature ? (bad++, "!") : ""}`;
    });
    console.log(`1:${scale} ${text.join("  ")} ${bad === 0 ? "OK" : "FAIL"}  ${file.split("/").pop()}`);
  }
}
