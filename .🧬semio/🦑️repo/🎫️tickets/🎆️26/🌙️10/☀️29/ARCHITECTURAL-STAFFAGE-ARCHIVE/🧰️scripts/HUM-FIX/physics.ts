/** [DEBUG] Force-residual and support probe for human assets. */
import { readdir, readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveStructure } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts";
import type { Asset, EvaluationQuery } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";

function ballistic(asset: Asset, query: EvaluationQuery, phase: string | null): boolean {
  if (query.kind === "pose") return asset.poses.find((pose) => pose.id === query.pose)?.contacts.length === 0 && asset.structure.supports.length === 0;
  if (query.kind !== "clip") return false;
  return asset.clips.find((clip) => clip.id === query.clip)?.phases.some((candidate) => candidate.id === phase && candidate.ballistic === true) ?? false;
}

function problems(asset: Asset, samples: number): string[] {
  const out: string[] = [];
  const queries: EvaluationQuery[] = [
    ...asset.poses.filter((pose) => !pose.key).map((pose): EvaluationQuery => ({ kind: "pose", pose: pose.id })),
    ...asset.clips.flatMap((clip) => Array.from({ length: samples }, (_, i): EvaluationQuery => ({ kind: "clip", clip: clip.id, timeS: (clip.durationS * i) / Math.max(1, samples - 1) }))),
  ];
  for (const query of queries) {
    const label = query.kind === "pose" ? `pose ${query.pose}` : `clip ${query.clip}@${query.timeS.toFixed(3)}`;
    try {
      const physical = evaluate(asset, query);
      const structure = solveStructure(physical);
      const weight = structure.totalMassKg * structure.gravityMS2;
      const tolerance = Math.max(1e-6, weight * 1e-6);
      const supported = structure.reactions.some((reaction) => reaction.active);
      if (!supported && !ballistic(asset, query, physical.phase)) out.push(`${label}: no active support`);
      if (supported && (Math.abs(structure.residual.fxN) > tolerance || Math.abs(structure.residual.fyN) > tolerance)) out.push(`${label}: residual ${structure.residual.fxN.toExponential(2)} ${structure.residual.fyN.toExponential(2)} phase=${physical.phase}`);
    } catch (error) {
      out.push(`${label}: ${(error as Error).message}`);
    }
  }
  return out;
}

const root = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans";
const only = new Set(process.argv.slice(2));
for (const folder of (await readdir(root)).sort()) {
  if (only.size && !only.has(folder)) continue;
  const doc = JSON.parse(await readFile(resolve(root, folder, (await readdir(resolve(root, folder))).find((f) => f.endsWith(".json"))!), "utf8"));
  const asset = doc.asset as Asset;
  const found = problems(asset, 9);
  console.log(`\n== ${asset.id} ${asset.structure.dynamics} ${found.length}`);
  for (const line of found.slice(0, 12)) console.log(" ", line);
  if (found.length && asset.structure.dynamics === "inertial") {
    const clone = structuredClone(asset);
    (clone.structure as { dynamics: string }).dynamics = "quasi-static";
    const again = problems(clone, 9);
    console.log(`  quasi-static ${again.length}`);
    for (const line of again.slice(0, 4)) console.log("   ", line);
    if (again.length === 0) {
      doc.asset.structure.dynamics = "quasi-static";
      const docName = (await readdir(resolve(root, folder))).find((f) => f.endsWith(".json"))!;
      await writeFile(resolve(root, folder, docName), `${JSON.stringify(doc, null, 2)}\n`);
      console.log("  wrote quasi-static");
    }
  }
}
