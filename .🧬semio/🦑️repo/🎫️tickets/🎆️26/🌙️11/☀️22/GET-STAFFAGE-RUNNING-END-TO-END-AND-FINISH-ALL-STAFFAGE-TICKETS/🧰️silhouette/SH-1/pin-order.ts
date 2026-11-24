/** 📌️ Pins the drawing order of every view the repair touched: appends one `order` op per scale and view (the ids of the catalog file as it is) to ops/<slug>.json in place of earlier ones, then proves that the ops applied to the catalog file give the catalog file again (same JSON value). The last step of the repair of a person.
 *  bun pin-order.ts <slug>… */
import { applyOps, readOps, writeOps } from "./ops.ts";
import { catalogFile, opsFile } from "./people.ts";

for (const slug of process.argv.slice(2)) {
  const doc = await Bun.file(catalogFile(slug)).json();
  const ops = (await readOps(opsFile(slug))).filter((o) => o.op !== "order");
  const touched = new Set(ops.filter((o) => String(o.op).startsWith("shape")).map((o) => `${o.scale}|${o.view}`));
  const pins = [...touched].map((key) => { const [scale, view] = key.split("|") as [string, string]; return { op: "order", scale, view, ids: (doc.asset.representations[scale].geometry[view] as any[]).map((s) => s.id) }; });
  const all = [...ops, ...pins];
  const again = structuredClone(doc);
  applyOps(again.asset, all as never);
  const same = JSON.stringify(again) === JSON.stringify(doc);
  if (same) await writeOps(opsFile(slug), all as never);
  console.log(`${slug}: ${pins.length} views pinned, ops on the catalog file give ${same ? "the same file" : "ANOTHER file (ops not written)"}`);
}
