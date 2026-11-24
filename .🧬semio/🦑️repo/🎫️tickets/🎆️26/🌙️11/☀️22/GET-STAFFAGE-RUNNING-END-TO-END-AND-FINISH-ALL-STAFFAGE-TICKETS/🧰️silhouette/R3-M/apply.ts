/** 🧷️ The named, idempotent repair of an asset of batch R3-M: applies the repair ops `ops/<id>.json` (drawings, volumes, declared dimensions) to the catalog file and writes it back in its own indentation; the file is read right before it is written.
 *  bun apply.ts <id>… [--out <file>] [--take-out <earlier ops.json>]   (`--take-out`: shapes that an earlier version of the ops added and the present ops no longer add are removed from the file first)
 * @see ./ops.ts — the ops */
import { parseArgs } from "node:util";
import { applyOps, type Op } from "./ops.ts";
import { catalogFile } from "./files.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { out: { type: "string" }, "take-out": { type: "string" } } });
for (const id of positionals) {
  const ops = JSON.parse(await Bun.file(`${import.meta.dir}/ops/${id}.json`).text()) as Op[];
  const text = await Bun.file(catalogFile(id)).text();
  const second = text.split("\n", 2)[1] ?? "";
  const indent = second.length - second.trimStart().length;
  const doc = JSON.parse(text);
  if (values["take-out"] !== undefined) {
    const gone = JSON.parse(await Bun.file(values["take-out"]).text()) as Op[];
    for (const o of gone) if (o.op === "shape-add" && !ops.some((x) => x.op === "shape-add" && x.scale === o.scale && x.view === o.view && x.shape.id === o.shape.id)) {
      const list = doc.asset.representations[o.scale!].geometry[o.view!] as { id: string }[];
      const at = list.findIndex((x) => x.id === o.shape.id);
      if (at >= 0) list.splice(at, 1);
    }
  }
  const skipped = applyOps(doc.asset, ops);
  const next = JSON.stringify(doc, null, indent) + (text.endsWith("\n") ? "\n" : "");
  if (next !== text) await Bun.write(values.out ?? catalogFile(id), next);
  console.log(`${id}: ${ops.length} ops applied${skipped.length > 0 ? `, ${skipped.length} already in place` : ""}${next === text ? ", file unchanged" : ""}`);
}
