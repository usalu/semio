/** 🚥️ The gate of MR (`../MR/gate.ts`) with two additions for the iteration on one asset: `--only <regex>` keeps the subjects whose id matches, and every bone or anchor line carries the message of its worst subject (the two ranges). `bun gate.ts <id|file.json>[,…] [--poses] [--only <regex>] [--scales 50,100] [--twins] [--soft]` runs the matrix's own `geometryFindings` (extent sync, per-bone sync, anchor landmarks, default-pose dimensions) over every subject of the matrix (poses, clip instants, transitions; `--poses` keeps the poses only) at every authored scale and prints, per scale and code, the count, the worst delta with its subject and the bones or anchors concerned (`--soft` lists them for the reported tier too). `--twins` adds the fixer's own tier `twin-*`: every shape against its twin of the same id and bone in another view, whatever else the bone draws (reported only). The exit code is 1 when a hard finding remains. The official pose gate is `T/🔎️scan.ts`; this one adds the clip instants and transitions the matrix evaluates.
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/🧰️.ts */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => {
  const i = args.indexOf(`--${name}`);
  return i < 0 ? undefined : (args[i + 1] ?? "");
};
const has = (name: string): boolean => args.includes(`--${name}`);
const scales = flag("scales")?.split(",");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));

type Entry = { tier: string; count: number; worstMm: number; subject: string; message: string; keys: Record<string, { mm: number; subject: string; n: number; message: string }> };
const AXES = [["height", "elevation", "y", "front", "y"], ["forward", "elevation", "x", "plan", "x"], ["lateral", "front", "x", "plan", "y"]] as const;
const rangeOf = (rings: any[], axis: "x" | "y"): [number, number] | undefined => {
  const values = rings.flat().map((p: any) => p[axis] as number);
  return values.length === 0 ? undefined : [Math.min(...values), Math.max(...values)];
};
let hard = 0;
for (const arg of args[0]!.split(",")) {
  const asset = readDoc(arg).doc.asset;
  const tolerance = engine.toleranceM(asset);
  const authored = Object.keys(asset.representations).filter((key) => engine.authoredOf(asset, key) !== undefined && (scales === undefined || scales.includes(key)));
  for (const key of authored) {
    const found: Record<string, Entry> = {};
    const note = (tier: string, code: string, key2: string, mm: number, subject: string, message: string): void => {
      const entry = (found[code] ??= { tier, count: 0, worstMm: -1, subject, message, keys: {} });
      entry.count++;
      if (mm > entry.worstMm) Object.assign(entry, { worstMm: mm, subject, message });
      const known = (entry.keys[key2] ??= { mm: -1, subject, n: 0, message });
      known.n++;
      if (mm > known.mm) Object.assign(known, { mm, subject, message });
    };
    const only = flag("only") === undefined ? undefined : new RegExp(flag("only")!, "u");
    const all = engine.subjects(asset, Number(key)).filter((candidate: any) => only === undefined || only.test(candidate.id));
    for (const subject of has("poses") ? all.filter((candidate: any) => candidate.query.kind === "pose") : all) {
      let state: any;
      try {
        state = evaluate(asset, subject.query, engine.scaleSelector(Number(key)));
      } catch (error) {
        note("hard", "evaluate-error", "-", 0, subject.id, (error as Error).message.slice(0, 200));
        continue;
      }
      for (const finding of engine.geometryFindings(asset, state, subject.isDefault)) note(finding.tier, finding.check, finding.anchor !== undefined ? `${finding.anchor}@${finding.views.join("/")}` : (finding.bone ?? finding.views.join("/")), finding.delta?.deltaMm ?? 0, subject.id, finding.message);
      if (!has("twins")) continue;
      for (const [name, a, aAxis, b, bAxis] of AXES)
        for (const left of state.shapes[a]) {
          if (left.shape.role === "ground") continue;
          const right = state.shapes[b].find((other: any) => other.shape.id === left.shape.id && other.shape.bone === left.shape.bone);
          const ra = rangeOf(left.rings, aAxis);
          const rb = right === undefined ? undefined : rangeOf(right.rings, bAxis);
          if (ra === undefined || rb === undefined) continue;
          const delta = Math.max(Math.abs(ra[0] - rb[0]), Math.abs(ra[1] - rb[1]));
          if (delta > tolerance) note("twin", `twin-${name}`, `${left.shape.bone}/${left.shape.id}`, delta * 1000, subject.id, `"${left.shape.id}" ${a} [${ra[0].toFixed(3)}, ${ra[1].toFixed(3)}] vs ${b} [${rb[0].toFixed(3)}, ${rb[1].toFixed(3)}]`);
        }
    }
    for (const [code, entry] of Object.entries(found).sort()) {
      if (entry.tier === "hard") hard += entry.count;
      console.log(`${asset.id} 1:${key} ${entry.tier === "hard" ? "HARD" : entry.tier} ${code} ×${entry.count} worst ${entry.worstMm.toFixed(0)} mm @ ${entry.subject}: ${entry.message}`);
      if (entry.tier !== "reported" || has("soft")) for (const [k, v] of Object.entries(entry.keys).sort((x, y) => y[1].mm - x[1].mm)) console.log(`    ${k} ×${v.n} ${v.mm.toFixed(0)} mm @ ${v.subject} :: ${v.message.replace(/^bone "[^"]+" /u, "").slice(0, 150)}`);
    }
    console.log(`${asset.id} 1:${key} subjects ${all.length} tolerance ${(tolerance * 1000).toFixed(1)} mm`);
  }
}
console.log(`hard findings: ${hard}`);
process.exit(hard === 0 ? 0 : 1);
