/** 🧭️ The facts behind the whole-figure `sync-*` findings of one plant: `bun extent.ts <id|file.json> [--scales 50,100]` runs every subject of the matrix at every authored scale and prints, for each subject whose three views differ beyond the tolerance, the axis, the two ranges and the shape (id, bone, solid kind) that sets each end in each view; then the count of failing subjects per scale and axis. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";
const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const args = process.argv.slice(2);
const only = args.includes("--scales") ? args[args.indexOf("--scales") + 1]!.split(",") : undefined;
const asset = readDoc(args[0]!).doc.asset;
const tolerance: number = engine.toleranceM(asset);
const AXES = [["height", "elevation", "y", "front", "y"], ["forward", "elevation", "x", "plan", "x"], ["lateral", "front", "x", "plan", "y"]] as const;
const ends = (state: any, view: string, axis: "x" | "y"): { low: any; high: any } => {
  let low: any, high: any;
  for (const shape of state.shapes[view]) {
    if (shape.shape.role === "ground") continue;
    for (const p of shape.rings.flat()) {
      if (low === undefined || p[axis] < low.value) low = { value: p[axis], shape: shape.shape };
      if (high === undefined || p[axis] > high.value) high = { value: p[axis], shape: shape.shape };
    }
  }
  return { low, high };
};
const name = (e: any): string => `${e.value.toFixed(4)} "${e.shape.id}"@${e.shape.bone}${e.shape.solid ? `(${e.shape.solid.kind})` : ""}`;
for (const key of Object.keys(asset.representations).filter((k) => engine.authoredOf(asset, k) !== undefined && (only === undefined || only.includes(k)))) {
  const count: Record<string, number> = {};
  const worst: Record<string, number> = {};
  for (const subject of engine.subjects(asset, Number(key))) {
    const state = evaluate(asset, subject.query, engine.scaleSelector(Number(key)));
    for (const [axisName, a, aAxis, b, bAxis] of AXES) {
      const [ea, eb] = [ends(state, a, aAxis), ends(state, b, bAxis)];
      if (ea.low === undefined || eb.low === undefined) continue;
      const [dl, dh] = [ea.low.value - eb.low.value, ea.high.value - eb.high.value];
      const d = Math.max(Math.abs(dl), Math.abs(dh));
      worst[axisName] = Math.max(worst[axisName] ?? 0, d);
      if (d <= tolerance) continue;
      count[axisName] = (count[axisName] ?? 0) + 1;
      console.log(`1:${key} ${axisName} ${(d * 1000).toFixed(1)} mm @ ${subject.id}\n   ${a}: ${name(ea.low)} … ${name(ea.high)}\n   ${b}: ${name(eb.low)} … ${name(eb.high)}`);
    }
  }
  console.log(`[DEBUG] 1:${key} tolerance ${(tolerance * 1000).toFixed(1)} mm, failing subjects ${JSON.stringify(count)}, worst mm ${JSON.stringify(Object.fromEntries(Object.entries(worst).map(([k, v]) => [k, Math.round(v * 10000) / 10])))}`);
}
