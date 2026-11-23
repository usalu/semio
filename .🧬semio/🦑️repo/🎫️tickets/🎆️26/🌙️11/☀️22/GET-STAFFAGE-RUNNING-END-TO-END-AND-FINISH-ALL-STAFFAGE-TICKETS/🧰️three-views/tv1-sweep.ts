/** 🧹️ Runs the pose-and-clip sweep and the default-pose consistency of the three-views guard (the exported `sweep` and `consistency` of the test, unchanged) for the given asset ids: `bun tv1-sweep.ts <id>… [--detail N] [--scale N] [--file <asset document json>]`. `--file` sweeps a candidate document as it is instead of the catalog assets. Prints every finding and one summary line per asset; `--detail N` adds, per failing instant, the N shapes reaching furthest on each edge the finding names. `--margin N` lists, per scale, the N spread or twisted instants that use most of the tolerance (largest edge difference of the three pairs of views, as a share of the tolerance). Exit 1 on any finding. */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const staffage = join(resolve(import.meta.dir, "../../../../../../../.."), "🎓️teaching/🏛️architecture/🧍️staffage");
const { authoredScales, catalog, consistency, extentOf, queries, sweep, toleranceM } = await import(join(staffage, "🧪️tests/🧊️three-views/🧰️.ts"));
const { evaluate } = await import(join(staffage, "🔨️modules/🧮️evaluation/🟦️.ts"));

type Row = { id: string; bone: string; kind: string; minX: number; maxX: number; minY: number; maxY: number };
const args = process.argv.slice(2);
const option = (name: string): string | undefined => (args.includes(name) ? args[args.indexOf(name) + 1] : undefined);
const detail = Number(option("--detail") ?? 0);
const only = option("--scale");
const file = option("--file");
const margin = Number(option("--margin") ?? 0);
const ids = args.filter((arg, index) => !arg.startsWith("--") && !args[index - 1]?.startsWith("--"));
const AXES: Record<string, [string, "X" | "Y", string, "X" | "Y"]> = { "spread-elevation-x-plan-x": ["elevation", "X", "plan", "X"], "spread-elevation-y-front-y": ["elevation", "Y", "front", "Y"], "spread-front-x-plan-y": ["front", "X", "plan", "Y"] };

function rows(state: { shapes: Record<string, { shape: { id: string; bone: string; kind: string; role: string; solid?: unknown }; rings: { x: number; y: number }[][] }[]> }, view: string): Row[] {
  return state.shapes[view]!.filter((e) => e.shape.role !== "ground" && e.rings.flat().length > 0).map((e) => {
    const points = e.rings.flat();
    return { id: e.shape.id, bone: e.shape.bone, kind: e.shape.solid === undefined ? e.shape.kind : "solid", minX: Math.min(...points.map((p) => p.x)), maxX: Math.max(...points.map((p) => p.x)), minY: Math.min(...points.map((p) => p.y)), maxY: Math.max(...points.map((p) => p.y)) };
  });
}

function furthest(list: Row[], edge: "minX" | "maxX" | "minY" | "maxY"): string {
  return [...list].sort((a, b) => (edge.startsWith("min") ? a[edge] - b[edge] : b[edge] - a[edge])).slice(0, detail).map((r) => `${r.id}[${r.bone},${r.kind}] ${r[edge].toFixed(3)}`).join(" | ");
}

let red = 0;
for (const asset of file === undefined ? catalog().filter((candidate: { id: string }) => ids.includes(candidate.id)) : [JSON.parse(readFileSync(resolve(file), "utf8")).asset]) {
  const scales = authoredScales(asset).filter((scale: number) => only === undefined || scale === Number(only));
  const swept = scales.flatMap((scale: number) => sweep(asset, scale));
  const consistent = scales.flatMap((scale: number) => consistency(asset, scale));
  for (const finding of [...swept, ...consistent]) {
    console.log(`[${finding.check}] ${finding.message}`);
    const axes = AXES[finding.check];
    if (detail === 0 || axes === undefined) continue;
    const query = queries(asset).find((q: { name: string }) => finding.message.includes(` ${q.name} (`))!.query;
    const state = evaluate(asset, query, { kind: "scale", scale: { denominator: finding.scale } });
    for (const [view, axis] of [[axes[0], axes[1]], [axes[2], axes[3]]] as const) for (const side of ["min", "max"] as const) console.log(`    ${view} ${side}${axis}: ${furthest(rows(state, view), `${side}${axis}`)}`);
  }
  for (const scale of margin > 0 ? scales : []) {
    const used: { name: string; share: number; pair: string }[] = [];
    for (const { name, query } of queries(asset)) {
      let state;
      try {
        state = evaluate(asset, query, { kind: "scale", scale: { denominator: scale } });
      } catch {
        continue;
      }
      if (!Object.values(state.bones as Record<string, { offset: { abductionDeg: number; twistDeg: number } }>).some((bone) => Math.abs(bone.offset.abductionDeg) > 1e-9 || Math.abs(bone.offset.twistDeg) > 1e-9)) continue;
      const [e, f, p] = ["elevation", "front", "plan"].map((view) => extentOf(state.shapes[view]));
      if (e === undefined || f === undefined || p === undefined) continue;
      const pairs: [string, number][] = [["elevation x / plan x", Math.max(Math.abs(e.minX - p.minX), Math.abs(e.maxX - p.maxX))], ["elevation y / front y", Math.max(Math.abs(e.minY - f.minY), Math.abs(e.maxY - f.maxY))], ["front x / plan y", Math.max(Math.abs(f.minX - p.minY), Math.abs(f.maxX - p.maxY))]];
      const worst = pairs.sort((a, b) => b[1] - a[1])[0]!;
      used.push({ name, share: worst[1] / toleranceM(asset), pair: worst[0] });
    }
    console.log(`  margin 1:${scale} (${used.length} spread instants): ${used.sort((a, b) => b.share - a.share).slice(0, margin).map((u) => `${u.name} ${(u.share * 100).toFixed(0)} % ${u.pair}`).join(" | ")}`);
  }
  red += swept.length + consistent.length;
  console.log(`${swept.length + consistent.length === 0 ? "✓" : "✗"} ${asset.id}: sweep ${swept.length} findings, default-pose consistency ${consistent.length} findings, scales ${scales.join(", ")}`);
}
process.exit(red === 0 ? 0 : 1);
