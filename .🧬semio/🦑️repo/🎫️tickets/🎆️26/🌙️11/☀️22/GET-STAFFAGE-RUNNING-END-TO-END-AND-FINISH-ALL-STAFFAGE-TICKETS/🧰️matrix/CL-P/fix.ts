/** 🩹️ Applies the detail repairs of lane CL-P to one asset document and writes a trial copy in the indentation of the source: `bun fix.ts <id|file.json> --ops <ops.json> --out <file.json>`. An op is `{ "set": "<path>", "value": …, "was": … }` (refuses unless the present value equals `was`), or `{ "insert": "<path of a list>", "after": "<id>", "from": "<path of a solid>", "patch": {…} }` (a copy of another solid placed behind the entry `after`; skipped when its id is already in the list). A path runs from the asset: `volumes.200.structure[tibia-l].sections.2.rx`, an entry of a list by its id in brackets.
 * @see ../MR/lib.ts */
import { readFileSync, writeFileSync } from "node:fs";
import { parseArgs } from "node:util";
import { readDoc } from "../MR/lib.ts";

type Op = { set?: string; value?: unknown; was?: unknown; insert?: string; after?: string; from?: string; patch?: Record<string, unknown> };
const { values, positionals } = parseArgs({ options: { ops: { type: "string" }, out: { type: "string" } }, allowPositionals: true });
const read = readDoc(positionals[0]!);
const steps = (path: string): string[] => path.split(".").flatMap((part) => { const m = /^([^\[]+)\[(.+)\]$/.exec(part); return m ? [m[1]!, `#${m[2]}`] : [part]; });
const step = (node: any, key: string, path: string): any => {
  const next = key.startsWith("#") ? (node as { id: string }[]).find((entry) => entry.id === key.slice(1)) : node[key];
  if (next === undefined) throw new Error(`${path}: nothing at "${key}"`);
  return next;
};
const at = (path: string): any => steps(path).reduce((node, key) => step(node, key, path), read.doc.asset);
let changed = 0;
for (const op of JSON.parse(readFileSync(values.ops!, "utf8")) as Op[]) {
  if (op.set !== undefined) {
    const keys = steps(op.set), last = keys.pop()!, holder = keys.reduce((node, key) => step(node, key, op.set!), read.doc.asset);
    if (JSON.stringify(holder[last]) === JSON.stringify(op.value)) continue;
    if (JSON.stringify(holder[last]) !== JSON.stringify(op.was)) throw new Error(`${op.set}: is ${JSON.stringify(holder[last])}, expected ${JSON.stringify(op.was)}`);
    holder[last] = op.value;
    changed++;
  } else if (op.insert !== undefined) {
    const list = at(op.insert) as { id: string }[], solid = { ...structuredClone(at(op.from!)), ...(op.patch ?? {}) } as { id: string };
    if (list.some((entry) => entry.id === solid.id)) continue;
    const index = list.findIndex((entry) => entry.id === op.after);
    if (index < 0) throw new Error(`${op.insert}: no entry "${op.after}"`);
    list.splice(index + 1, 0, solid);
    changed++;
  }
}
const second = read.text.split("\n", 2)[1] ?? "";
writeFileSync(values.out!, JSON.stringify(read.doc, null, second.length - second.trimStart().length) + (read.text.endsWith("\n") ? "\n" : ""));
console.log(`${positionals[0]}: ${changed} ops changed something -> ${values.out}`);
