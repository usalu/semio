/** 🎞️ Evaluates people at every authored scale in every named pose and at nine instants of every clip (what the check CLI samples) and names the drawn shapes whose evaluation throws: per failing instant every shape of the scale is evaluated alone.
 *  bun evalcheck.ts <slug|file.json>… */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { fileOf } from "./people.ts";

for (const arg of process.argv.slice(2)) {
  const asset = (await Bun.file(fileOf(arg)).json()).asset;
  const queries: { name: string; query: any }[] = asset.poses.map((p: any) => ({ name: `pose ${p.id}`, query: { kind: "pose", pose: p.id } }));
  for (const clip of asset.clips) for (let i = 0; i < 9; i++) { const t = (clip.durationS * i) / 8; queries.push({ name: `clip ${clip.id}@${t.toFixed(3)}`, query: { kind: "clip", clip: clip.id, timeS: t } }); }
  const culprits = new Set<string>();
  let failures = 0;
  for (const d of [50, 100, 200, 500]) {
    if (asset.representations[String(d)]?.status !== "authored") continue;
    for (const { name, query } of queries) {
      try { evaluate(asset, query, { kind: "scale", scale: { denominator: d } }); } catch (error) {
        failures++;
        const before = culprits.size;
        for (const view of ["elevation", "front", "plan"] as const) for (const shape of asset.representations[String(d)].geometry[view]) {
          const one: any = structuredClone(asset);
          for (const v of ["elevation", "front", "plan"] as const) one.representations[String(d)].geometry[v] = v === view ? [shape] : one.representations[String(d)].geometry[v].slice(0, 1);
          try { evaluate(one, query, { kind: "scale", scale: { denominator: d } }); } catch { culprits.add(`${d}|${view}|${shape.id}`); }
        }
        console.log(`${asset.id} 1:${d} ${name}: ${String((error as Error).message).slice(0, 90)}${culprits.size === before ? " (no single shape found or already named)" : ""}`);
      }
    }
  }
  console.log(`${asset.id}: ${failures} failing instants; shapes: ${[...culprits].join(" ") || "none"}`);
}
