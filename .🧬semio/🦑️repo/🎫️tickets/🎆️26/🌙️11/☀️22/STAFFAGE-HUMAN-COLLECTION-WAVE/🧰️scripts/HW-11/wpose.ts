/** 🔎️ Prints a few bone tips of a waving probe on standing-adult. */
import { probe, readAsset } from "./lib.ts";
const a = await readAsset("standing-adult");
const p = (v: any) => `(${v.x.toFixed(3)}, ${v.y.toFixed(3)}, ${v.z.toFixed(3)})`;
for (const spec of JSON.parse(process.argv[2]!)) {
  const f = probe(a, spec);
  console.log(JSON.stringify(spec));
  for (const id of ["arm-r", "forearm-r", "hand-r", "middle-distal-r"]) console.log(`  ${id.padEnd(16)} tip ${p(f[id]!.tip)} u ${p(f[id]!.u)} v ${p(f[id]!.v)} w ${p(f[id]!.w)}`);
}
