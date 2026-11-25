/** ⏱️ The time `evaluate` needs per subject of the matrix: `bun timing.ts <id|file.json> [scale…]` prints per authored scale the number of subjects, then for every eighth subject the milliseconds of its evaluation and at the end the sum projected over all subjects; the fact behind a gate or matrix that does not end. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";
const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const [arg, ...keys] = process.argv.slice(2);
const asset = readDoc(arg!).doc.asset;
for (const key of Object.keys(asset.representations).filter((k) => engine.authoredOf(asset, k) !== undefined && (keys.length === 0 || keys.includes(k)))) {
  const all = engine.subjects(asset, Number(key));
  let sum = 0, n = 0, worst = 0, worstId = "";
  for (let i = 0; i < all.length; i += 8) {
    const start = performance.now();
    try { evaluate(asset, all[i].query, engine.scaleSelector(Number(key))); } catch (error) { console.log(`1:${key} ${all[i].id}: ${(error as Error).message.slice(0, 160)}`); }
    const ms = performance.now() - start;
    sum += ms; n++;
    if (ms > worst) { worst = ms; worstId = all[i].id; }
  }
  console.log(`${asset.id} 1:${key}: ${all.length} subjects, ${n} timed, mean ${(sum / n).toFixed(0)} ms, worst ${worst.toFixed(0)} ms @ ${worstId}, all subjects about ${((sum / n) * all.length / 1000).toFixed(0)} s`);
}
