/** 🔺️ Finds where a clip instant fails as the check CLI evaluates it (state, thermal, structure, every scale, viewpoint and view) and, for a failing depiction, the shapes that fail alone: bun union.ts <slug|file.json> <clip> <timeS>
 * @see ./feature.ts — the sibling probe */
import { join, resolve } from "node:path";
const SR = "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${SR}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { defaultDepictionOptions, depict } = await import(`${SR}/🔨️modules/🎨️depiction/🟦️.ts`);
const [slug, clip, time] = process.argv.slice(2);
const file = slug!.endsWith(".json") ? slug! : join(resolve(import.meta.dir, SR, "🗂️catalog/👤️humans"), slug!, "🔣️.json");
const asset = (await Bun.file(file).json()).asset;
const query = { kind: "clip", clip, timeS: Number(time) };
const attempt = (name: string, f: () => unknown): boolean => { try { f(); return true; } catch (error) { console.log(`${name}: ${String((error as Error).message).slice(0, 110)}`); return false; } };
attempt("state", () => evaluate(asset, query));
const views = ["dark-solid", "light-solid", "dark-line", "light-line"];
for (const scale of [50, 100, 200, 500]) {
  const selector = { kind: "scale", scale: { denominator: scale } };
  if (!attempt(`1:${scale} evaluate`, () => evaluate(asset, query, selector))) {
    const geometry = asset.representations[String(scale)].geometry;
    const kept = { ...geometry };
    for (const viewpoint of ["elevation", "front", "plan"]) {
      for (const shape of kept[viewpoint]) {
        for (const v of ["elevation", "front", "plan"]) geometry[v] = v === viewpoint ? [shape] : [];
        try { evaluate(asset, query, selector); } catch { console.log(`   alone: ${viewpoint} ${shape.id} (bone ${shape.bone}, ${shape.kind}${shape.solid ? `, ${shape.solid.kind} ${shape.solid.sweep ?? shape.solid.axis}` : ""})`); }
      }
    }
    Object.assign(geometry, kept);
    continue;
  }
  const state = evaluate(asset, query, selector);
  for (const viewpoint of ["elevation", "front", "plan"]) {
    for (const view of views) {
      if (attempt(`1:${scale} ${viewpoint} ${view}`, () => depict(state, defaultDepictionOptions(view, "en", scale, viewpoint), {}))) continue;
      const all = state.shapes[viewpoint];
      for (const one of all) {
        const single = { ...state, shapes: { ...state.shapes, [viewpoint]: [one] } };
        try { depict(single, defaultDepictionOptions(view, "en", scale, viewpoint), {}); } catch { console.log(`   alone: ${one.shape.id} (bone ${one.shape.bone})`); }
      }
      break;
    }
  }
}
