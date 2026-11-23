/** 🧺️ Takes the silhouette ops of the greedy repair (🗑️generated/REP-B/ops/<slug>.json, in their order) into the repair file of the person: the drawing ops the file had are replaced by them; the volume, thermal and dimension ops and the drawing ops written by hand (`manual: true`) stay in front. `OPSDIR` names another folder of silhouette ops.
 *  bun merge.ts <slug>… */
import { readOps, writeOps } from "./ops.ts";
import { K, opsFile } from "./people.ts";

for (const slug of process.argv.slice(2)) {
  const drawn = await readOps(`${process.env.OPSDIR ?? `${K}/🗑️generated/REP-B/ops`}/${slug}.json`);
  if (drawn.length === 0) { console.log(`${slug}: no silhouette ops`); continue; }
  const kept = (await readOps(opsFile(slug))).filter((o) => !o.op.startsWith("shape") || (o as { manual?: boolean }).manual === true);
  await writeOps(opsFile(slug), [...kept, ...drawn.filter((o) => o.op.startsWith("shape"))]);
  console.log(`${slug}: ${kept.length} volume ops, ${drawn.filter((o) => o.op.startsWith("shape")).length} drawing ops`);
}
