/** 🕵️ Which changed shape makes a subject fail to evaluate: `bun culprit.ts <before.json> <after.json> --scale 50 --subject "<subject id>"` evaluates the subject on the changed document and, when it throws, once more per changed solid shape with that shape as the only geometry of the scale, and names the shapes that fail alone. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const before = readDoc(args[0]!).doc.asset;
const after = readDoc(args[1]!).doc.asset;
const scale = flag("scale")!;
const attempt = (asset: any): string | undefined => {
  const subject = engine.subjects(asset, Number(scale)).find((candidate: any) => candidate.id === flag("subject"));
  try {
    evaluate(asset, subject.query, engine.scaleSelector(Number(scale)));
    return undefined;
  } catch (error) {
    return (error as Error).message.slice(0, 160);
  }
};
const failure = attempt(after);
console.log(failure === undefined ? "evaluates" : `fails: ${failure}`);
if (failure !== undefined)
  for (const view of ["elevation", "front", "plan"]) {
    const old = before.representations[scale].geometry[view];
    for (const shape of after.representations[scale].geometry[view]) {
      const was = old.find((candidate: any) => candidate.id === shape.id && candidate.bone === shape.bone);
      if (shape.solid === undefined || (was !== undefined && JSON.stringify(was) === JSON.stringify(shape))) continue;
      const trial = structuredClone(after);
      trial.representations[scale].geometry = { elevation: [], front: [], plan: [], [view]: [structuredClone(shape)] };
      const alone = attempt(trial);
      if (alone !== undefined) console.log(`fails alone: ${view} ${shape.bone}/${shape.id}: ${JSON.stringify({ points: shape.points, solid: shape.solid })}`);
    }
  }
