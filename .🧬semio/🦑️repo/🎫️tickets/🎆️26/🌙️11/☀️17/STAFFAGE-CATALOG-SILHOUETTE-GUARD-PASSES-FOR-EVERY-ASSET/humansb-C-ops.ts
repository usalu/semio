/** 🧾 Converts in-memory patches into a humans-ops.py ops file by diffing shapes and solids by id. bun humansb-C-ops.ts <asset> <patches> <out.json> */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { applyPatch } from "./humansb-C-patch.ts";
const [name, patch, out] = process.argv.slice(2);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset: before } = (await readStreamedAsset(entry)) as { asset: Asset };
const after = JSON.parse(JSON.stringify(before));
await applyPatch(after, patch);
const ops: unknown[] = [];
const diff = (a: any[], b: any[], mk: (kind: "set" | "add", id: string, item: any, prev?: string) => unknown) => {
  const old = new Map(a.map((x) => [x.id, JSON.stringify(x)]));
  b.forEach((x, i) => {
    if (!old.has(x.id)) ops.push(mk("add", x.id, x, b[i - 1]?.id));
    else if (old.get(x.id) !== JSON.stringify(x)) ops.push(mk("set", x.id, x));
  });
  for (const x of a) if (!b.some((y) => y.id === x.id)) throw new Error("removal not supported: " + x.id);
};
for (const sc of Object.keys((before as any).representations))
  for (const view of Object.keys((before as any).representations[sc].geometry))
    diff((before as any).representations[sc].geometry[view], (after as any).representations[sc].geometry[view], (k, id, item, prev) => k === "add" ? { op: "shape-add", scale: sc, view, after: prev, shape: item } : { op: "shape", scale: sc, view, id, shape: item });
for (const sc of Object.keys((before as any).volumes)) {
  const a = (before as any).volumes[sc].body, b = (after as any).volumes[sc]?.body;
  if (a && b) diff(a, b, (k, id, item, prev) => k === "add" ? { op: "solid-add", scale: sc, after: prev, solid: item } : { op: "solid", scale: sc, id, solid: item });
}
await Bun.write(out, JSON.stringify(ops, null, 1));
console.log(ops.length, "ops");
