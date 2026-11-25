/** 🩻️ The fixer's gate with its reasons: `bun why.ts <id|file.json>[,…] [--scales 50,100] [--poses]` runs the matrix's own `geometryFindings` over every subject of the matrix (poses, clip instants, transitions) at every authored scale like `../MR/gate.ts`, prints per scale and code the count and the worst delta, and for the worst subject of every hard code the extents of the three views with the shape that sets each end, the boxes of the shapes of the bones the findings name and the anchors the findings name. The exit code is 1 when a hard finding remains.
 * @see ../MR/gate.ts — the gate this one extends */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.includes(`--${name}`) ? args[args.indexOf(`--${name}`) + 1] : undefined);
const scales = flag("scales")?.split(",");
const f = (n: number): string => n.toFixed(4);

/** 📦️ The boxes of the drawn shapes of a view of a state. */
export function boxesOf(state: any, view: string): { id: string; bone: string; solid: string; x0: number; x1: number; y0: number; y1: number }[] {
  return state.shapes[view].filter((s: any) => s.shape.role !== "ground" && s.rings.flat().length > 0).map((s: any) => {
    const points = s.rings.flat();
    const xs = points.map((p: any) => p.x);
    const ys = points.map((p: any) => p.y);
    return { id: s.shape.id, bone: s.shape.bone, solid: s.shape.solid === undefined ? (s.shape.followTip ? "followTip" : "flat") : `${s.shape.solid.kind}:${s.shape.solid.sweep ?? s.shape.solid.axis ?? ""}`, x0: Math.min(...xs), x1: Math.max(...xs), y0: Math.min(...ys), y1: Math.max(...ys) };
  });
}

/** 🖨️ Prints the extents of the three views of a state with the shape at each end, and the shapes of the named bones. */
export function explain(state: any, bones: ReadonlySet<string>, anchors: ReadonlySet<string>): void {
  for (const view of ["elevation", "front", "plan"]) {
    const boxes = boxesOf(state, view);
    if (boxes.length === 0) continue;
    const end = (key: "x0" | "x1" | "y0" | "y1", low: boolean): string => {
      const best = boxes.reduce((a, b) => ((low ? b[key] < a[key] : b[key] > a[key]) ? b : a));
      return `${f(best[key])} (${best.id}/${best.bone} ${best.solid})`;
    };
    console.log(`      ${view}: x ${end("x0", true)} … ${end("x1", false)}   y ${end("y0", true)} … ${end("y1", false)}`);
    for (const b of boxes) if (bones.has(b.bone)) console.log(`         ${b.id} [${b.bone}] ${b.solid} x ${f(b.x0)} ${f(b.x1)} y ${f(b.y0)} ${f(b.y1)}`);
  }
  for (const [id, a] of Object.entries(state.anchors) as [string, any][]) if (anchors.has(id)) console.log(`      anchor ${id} [${a.anchor.bone}] x ${f(a.at.x)} y ${f(a.at.y)} z ${f(a.z)}`);
}

/** 🚦️ Runs the gate over the subjects of an asset at its authored scales (or the given ones), prints the lines and returns the number of hard findings. */
export function gate(asset: any, options: { scales?: readonly string[]; poses?: boolean; quiet?: boolean } = {}): number {
  let hard = 0;
  const authored = Object.keys(asset.representations).filter((key) => engine.authoredOf(asset, key) !== undefined && (options.scales === undefined || options.scales.includes(key)));
  for (const key of authored) {
    const found: Record<string, { tier: string; count: number; worstMm: number; subject: any; message: string; bones: Set<string>; anchors: Set<string>; subjects: Set<string> }> = {};
    const all = engine.subjects(asset, Number(key));
    for (const subject of options.poses ? all.filter((candidate: any) => candidate.query.kind === "pose") : all) {
      let state: any;
      try {
        state = evaluate(asset, subject.query, engine.scaleSelector(Number(key)));
      } catch (error) {
        console.log(`${asset.id} 1:${key} HARD evaluate-error @ ${subject.id}: ${(error as Error).message.slice(0, 200)}`);
        hard++;
        continue;
      }
      for (const finding of engine.geometryFindings(asset, state, subject.isDefault)) {
        const entry = (found[finding.check] ??= { tier: finding.tier, count: 0, worstMm: -1, subject, message: finding.message, bones: new Set(), anchors: new Set(), subjects: new Set() });
        entry.count++;
        entry.subjects.add(subject.id);
        if (finding.bone !== undefined) entry.bones.add(finding.bone);
        if (finding.anchor !== undefined) entry.anchors.add(finding.anchor);
        if ((finding.delta?.deltaMm ?? 0) > entry.worstMm) Object.assign(entry, { worstMm: finding.delta?.deltaMm ?? 0, subject, message: finding.message });
      }
    }
    for (const [code, entry] of Object.entries(found).sort()) {
      if (entry.tier === "hard") hard += entry.count;
      if (entry.tier !== "hard" && options.quiet) continue;
      console.log(`${asset.id} 1:${key} ${entry.tier === "hard" ? "HARD" : entry.tier} ${code} ×${entry.count} worst ${entry.worstMm.toFixed(1)} mm @ ${entry.subject.id}: ${entry.message}`);
      if (entry.tier !== "hard") continue;
      console.log(`      subjects: ${[...entry.subjects].slice(0, 14).join(" | ")}${entry.subjects.size > 14 ? ` … (${entry.subjects.size})` : ""}`);
      explain(evaluate(asset, entry.subject.query, engine.scaleSelector(Number(key))), entry.bones, entry.anchors);
    }
    console.log(`${asset.id} 1:${key} subjects ${all.length} tolerance ${(engine.toleranceM(asset) * 1000).toFixed(1)} mm`);
  }
  return hard;
}

if (import.meta.main) {
  let hard = 0;
  for (const arg of args[0]!.split(",")) hard += gate(readDoc(arg).doc.asset, { scales, poses: args.includes("--poses") });
  console.log(`hard findings: ${hard}`);
  process.exit(hard === 0 ? 0 : 1);
}
