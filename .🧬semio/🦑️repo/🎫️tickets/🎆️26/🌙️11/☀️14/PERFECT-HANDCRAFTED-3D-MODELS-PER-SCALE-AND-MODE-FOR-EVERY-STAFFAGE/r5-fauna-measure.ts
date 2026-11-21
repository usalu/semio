/** 📐️ Prints the world bounding box of every body solid of an animal scale in the default pose (metres), and the whole extent, for the model-versus-reference tables of the round-5 reports.
 * Usage: bun r5-fauna-measure.ts <asset> <scale> [id-regex] */
import { openCtx } from "./kit-lib.ts";
import { meshesAt } from "./volumes-lib.ts";
const [spec, sc, re] = [process.argv[2]!, Number(process.argv[3]), new RegExp(process.argv[4] ?? ".")];
const c = openCtx(spec, sc);
const ms = meshesAt(c.m, sc, c.vol.body);
const all = [Infinity, Infinity, Infinity, -Infinity, -Infinity, -Infinity];
(c.vol.body as any[]).forEach((s, i) => {
  const w = ms[i]!.world;
  const b = [Infinity, Infinity, Infinity, -Infinity, -Infinity, -Infinity];
  for (let k = 0; k < w.length; k += 3) for (let a = 0; a < 3; a++) { b[a] = Math.min(b[a]!, w[k + a]!); b[a + 3] = Math.max(b[a + 3]!, w[k + a]!); }
  for (let a = 0; a < 3; a++) { all[a] = Math.min(all[a]!, b[a]!); all[a + 3] = Math.max(all[a + 3]!, b[a + 3]!); }
  if (re.test(s.id)) console.log(s.id.padEnd(18), "x", b[0]!.toFixed(3), b[3]!.toFixed(3), "y", b[1]!.toFixed(3), b[4]!.toFixed(3), "z", b[2]!.toFixed(3), b[5]!.toFixed(3));
});
console.log("ALL x", all[0]!.toFixed(3), all[3]!.toFixed(3), "y", all[1]!.toFixed(3), all[4]!.toFixed(3), "z", all[2]!.toFixed(3), all[5]!.toFixed(3));
