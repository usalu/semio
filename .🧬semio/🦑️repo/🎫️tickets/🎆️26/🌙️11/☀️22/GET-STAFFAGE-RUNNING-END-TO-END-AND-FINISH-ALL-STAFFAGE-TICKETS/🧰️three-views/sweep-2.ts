/** 🧹️ Runs the pose-and-clip sweep and the default-pose consistency of the three-views guard for the named assets only, with the guard's own exported functions (nothing of the test is changed or copied).
 *
 * Usage: `bun sweep-2.ts <id> [<id> …] [--file <asset document json>] [--explain]`
 * Prints every finding as the test prints it, then one line per asset `<id>: sweep N, consistency M`. `--file` sweeps a candidate document instead of the catalog file (one id). `--explain` adds, for every instant with a spread finding, the shape that sets each edge of each view.
 * Exit 1 when any finding is left.
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { VIEWS, authoredScales, catalog, consistency, extentOf, queries, sweep, toleranceM } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";

type Asset = ReturnType<typeof catalog>[number];

/** 🔎️ The shapes that set the four edges of every view at every instant that the sweep compares, as text lines. */
export function explain(asset: Asset, denominator: number, only?: (name: string) => boolean): string[] {
  const lines: string[] = [];
  for (const { name, query } of queries(asset)) {
    if (only !== undefined && !only(name)) continue;
    let state: ReturnType<typeof evaluate>;
    try {
      state = evaluate(asset, query, { kind: "scale", scale: { denominator } });
    } catch (error) {
      lines.push(`  1:${denominator} ${name}: ${(error as Error).message}`);
      continue;
    }
    for (const view of VIEWS) {
      const drawn = state.shapes[view].filter((evaluated) => evaluated.shape.role !== "ground");
      const edge = (pick: (e: NonNullable<ReturnType<typeof extentOf>>) => number, sign: number): string => {
        let best = "";
        let value = -Infinity;
        for (const evaluated of drawn) {
          const extent = extentOf([evaluated]);
          if (extent !== undefined && sign * pick(extent) > value) [value, best] = [sign * pick(extent), evaluated.shape.id];
        }
        return `${(sign * value).toFixed(3)} ${best}`;
      };
      lines.push(`  1:${denominator} ${name} ${view}: x [${edge((e) => e.minX, -1)} | ${edge((e) => e.maxX, 1)}] y [${edge((e) => e.minY, -1)} | ${edge((e) => e.maxY, 1)}]`);
    }
  }
  return lines;
}

if (import.meta.main) {
  const args = process.argv.slice(2);
  const fileAt = args.indexOf("--file");
  const file = fileAt < 0 ? undefined : args[fileAt + 1];
  const wantExplain = args.includes("--explain");
  const ids = args.filter((arg, index) => !arg.startsWith("--") && (fileAt < 0 || index !== fileAt + 1));
  const all = file === undefined ? catalog() : [(JSON.parse(readFileSync(file, "utf8")) as { asset: Asset }).asset];
  let left = 0;
  for (const id of ids) {
    const asset = all.find((candidate) => candidate.id === id);
    if (asset === undefined) {
      console.log(`${id}: not found`);
      left += 1;
      continue;
    }
    const swept = authoredScales(asset).flatMap((scale) => sweep(asset, scale));
    const consistent = authoredScales(asset).flatMap((scale) => consistency(asset, scale));
    for (const finding of [...swept, ...consistent]) console.log(`[${finding.check}] ${finding.message}`);
    if (wantExplain)
      for (const scale of authoredScales(asset)) {
        const named = new Set(swept.filter((finding) => finding.scale === scale).map((finding) => finding.message));
        for (const line of explain(asset, scale, (name) => [...named].some((message) => message.includes(` ${name} `) || message.includes(` ${name}:`)))) console.log(line);
      }
    console.log(`${id}: tolerance ${toleranceM(asset).toFixed(3)} m, scales ${authoredScales(asset).join(",")}, instants ${queries(asset).length}, sweep ${swept.length}, consistency ${consistent.length}`);
    left += swept.length + consistent.length;
  }
  process.exit(left === 0 ? 0 : 1);
}
