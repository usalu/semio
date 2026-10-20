/** 🧾 Applies the manifest operations by span edits: every `📋️project.json` is re-read right before its write, edited without touching untouched text, proven semantically equal to the intended change and replaced atomically.
 *
 *   bun apply-manifests.ts --dry-run   # plan only
 *   bun apply-manifests.ts             # write
 */
import { chmodSync, readFileSync, renameSync, statSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { apply, parse, prop, removeProp, setProps, type Edit, type Node } from "./jsonspan.ts";

const here = import.meta.dir, root = process.env.SEMIO_ROOT ?? resolve(here, "../../../../../../../..");
const dry = process.argv.includes("--dry-run");
type Op = { manifest: string; target: string; id: string; cache?: false; decision?: unknown; removeInputs?: true };
const ops: Op[] = [...JSON.parse(readFileSync(join(here, "manifest-ops.json"), "utf8")), ...JSON.parse(readFileSync(join(here, "manifest-extra-ops.json"), "utf8"))];
const canon = (value: unknown): unknown => Array.isArray(value) ? value.map(canon) : value && typeof value === "object" ? Object.fromEntries(Object.entries(value).sort(([a], [b]) => (a < b ? -1 : 1)).map(([k, v]) => [k, canon(v)])) : value;
const same = (a: unknown, b: unknown) => JSON.stringify(canon(a)) === JSON.stringify(canon(b));
const targetNode = (rootNode: Node, name: string): Node | undefined => { const targets = prop(rootNode, "targets"); return targets?.value.type === "object" ? prop(targets.value, name)?.value : undefined; };

function plan(text: string, rows: Op[]): { edits: Edit[]; expect: (json: any) => void; notes: string[] } {
  const rootNode = parse(text), edits: Edit[] = [], todo: ((json: any) => void)[] = [], notes: string[] = [];
  for (const op of rows) {
    const node = targetNode(rootNode, op.target);
    if (!node || node.type !== "object") { notes.push(`missing ${op.id}`); continue; }
    const cache = prop(node, "cache");
    if (op.cache === false && !(cache?.value.type === "literal" && cache.value.value === false)) {
      edits.push(...setProps(text, node, [["cache", false]], ["executor"]));
      todo.push((json) => { json.targets[op.target].cache = false; });
    }
    if (op.decision !== undefined) {
      const meta = prop(node, "metadata"), semio = meta?.value.type === "object" ? prop(meta.value, "semio") : undefined, have = semio?.value.type === "object" ? prop(semio.value, "cache") : undefined;
      const current = have ? JSON.parse(text.slice(have.value.start, have.value.end)) : undefined;
      if (!same(current, op.decision)) {
        if (!meta) edits.push(...setProps(text, node, [["metadata", { semio: { cache: op.decision } }]], ["options", "outputs", "cache"]));
        else if (!semio) edits.push(...setProps(text, meta.value, [["semio", { cache: op.decision }]]));
        else edits.push(...setProps(text, semio.value, [["cache", op.decision]]));
        todo.push((json) => { const t = json.targets[op.target]; t.metadata ??= {}; t.metadata.semio ??= {}; t.metadata.semio.cache = op.decision; });
      }
    }
    if (op.removeInputs) {
      const inputs = prop(node, "inputs");
      if (inputs?.value.type === "array" && inputs.value.items.length === 0) { const edit = removeProp(text, node, "inputs"); if (edit) edits.push(edit); todo.push((json) => { delete json.targets[op.target].inputs; }); }
    }
  }
  return { edits, expect: (json) => todo.forEach((f) => f(json)), notes };
}

const byManifest = new Map<string, Op[]>();
for (const op of ops) (byManifest.get(op.manifest) ?? byManifest.set(op.manifest, []).get(op.manifest)!).push(op);
let changed = 0; const problems: string[] = [];
for (const [manifest, rows] of [...byManifest].sort((a, b) => (a[0] < b[0] ? -1 : 1))) {
  const path = join(root, manifest);
  for (let attempt = 0; ; attempt++) {
    if (attempt > 3) { problems.push(`manifest keeps changing: ${manifest}`); break; }
    const before = readFileSync(path, "utf8"), result = plan(before, rows);
    for (const note of result.notes) problems.push(`${manifest}: ${note}`);
    if (!result.edits.length) break;
    const after = apply(before, result.edits), expected = JSON.parse(before);
    result.expect(expected);
    if (!same(JSON.parse(after), expected)) { problems.push(`edit changed more than intended: ${manifest}`); break; }
    if (dry) { changed++; console.log(`would edit\t${manifest}`); break; }
    if (readFileSync(path, "utf8") !== before) continue;
    const temp = `${path}.cache-fix.tmp`;
    writeFileSync(temp, after); chmodSync(temp, statSync(path).mode);
    if (readFileSync(path, "utf8") !== before) continue;
    renameSync(temp, path); changed++; console.log(`edited\t${manifest}`);
    break;
  }
}
if (problems.length) { console.error(problems.map((line) => `PROBLEM ${line}`).join("\n")); process.exit(1); }
console.log(`${dry ? "manifest dry run" : "manifests applied"}: ${changed} manifest(s)`);
