import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { solveThermalAt } = await import(`${S}/🔨️modules/🌡️thermal/🟦️.ts`);
const { solveStructure } = await import(`${S}/🔨️modules/🏗️structure/🟦️.ts`);
const { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } = await import(`${S}/🔨️modules/🗂️catalog/📥️files/🟦️.ts`);
const { defaultDepictionOptions, depict } = await import(`${S}/🔨️modules/🎨️depiction/🟦️.ts`);
const { writeSvg } = await import(`${S}/🔨️modules/📤️export/🖼️svg/🟦️.ts`);
const args = new Map<string, string>();
for (let i = 2; i < process.argv.length; i += 2) args.set(process.argv[i]!.replace(/^--/, ""), process.argv[i + 1]!);
const out = args.get("out")!;
const ids = args.get("assets")!.split(",");
const scales = (args.get("scales") ?? "50,100").split(",").map(Number);
const views = (args.get("views") ?? "dark-line,thermal").split(",");
const viewpoints = (args.get("viewpoints") ?? "elevation").split(",");
mkdirSync(out, { recursive: true });
const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
for (const id of ids) {
  const asset = catalog.assets.find((a: { id: string }) => a.id === id);
  if (!asset) { console.log("missing", id); continue; }
  for (const denominator of scales) {
    const selector = { kind: "scale", scale: { denominator } };
    const pose = args.get("pose") ?? asset.presentation.defaultPose;
    const query = args.has("clip") ? { kind: "clip", clip: args.get("clip"), timeS: Number(args.get("time") ?? 0) } : { kind: "pose", pose };
    let state;
    try { state = evaluate(asset, query, selector); } catch (error) { console.log("skip", id, denominator, String(error).slice(0, 120)); continue; }
    if (state.representation.status === "unavailable" || (state.representation as { kind?: string }).kind === "unavailable") { console.log("unavailable", id, denominator); continue; }
    const analysis = { thermal: solveThermalAt(state.asset, query), structure: solveStructure(state) };
    for (const view of views) for (const viewpoint of viewpoints) {
      const o = defaultDepictionOptions(view, "en", denominator, viewpoint);
      try {
        const scene = depict(state, { ...o, representation: selector, annotations: { ...o.annotations, legend: false } }, analysis);
        const file = join(out, `${id}--${denominator}--${view}--${viewpoint}.svg`);
        writeFileSync(file, writeSvg(scene));
        console.log(file);
      } catch (error) { console.log("fail", id, denominator, view, viewpoint, String(error).slice(0, 160)); }
    }
  }
}
