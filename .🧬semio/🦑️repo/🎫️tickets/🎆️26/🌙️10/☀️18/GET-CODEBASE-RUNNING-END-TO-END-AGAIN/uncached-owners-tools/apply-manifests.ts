/** 🧾 Applies the manifest operations of `manifest-ops.json` by span edits: every `📋️project.json` is re-read right before its write, edited without touching untouched text, proven semantically equal to the intended change and replaced atomically.
 * A target a peer already decided (another `metadata.semio.cache`, an authored `cache: true` over an uncached decision) is left alone and reported.
 *
 *   bun apply-manifests.ts --dry-run   # plan only
 *   bun apply-manifests.ts             # write
 */
import { chmodSync, readFileSync, renameSync, statSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { apply, parse, prop, setProps, type Edit, type Node } from "../cache-fix-tools/jsonspan.ts";
import type { Op } from "./plan.ts";

const here = import.meta.dir, root = process.env.SEMIO_ROOT ?? resolve(here, "../../../../../../../..");
const dry = process.argv.includes("--dry-run");
const ops: Op[] = JSON.parse(readFileSync(join(here, "manifest-ops.json"), "utf8"));
const canon = (value: unknown): unknown => Array.isArray(value) ? value.map(canon) : value && typeof value === "object" ? Object.fromEntries(Object.entries(value).sort(([a], [b]) => (a < b ? -1 : 1)).map(([k, v]) => [k, canon(v)])) : value;
const same = (a: unknown, b: unknown) => JSON.stringify(canon(a)) === JSON.stringify(canon(b));
const targetNode = (rootNode: Node, name: string): Node | undefined => { const targets = prop(rootNode, "targets"); return targets?.value.type === "object" ? prop(targets.value, name)?.value : undefined; };
const literal = (node: Node | undefined) => node?.type === "literal" ? node.value : undefined;

function plan(text: string, rows: Op[]): { edits: Edit[]; expect: (json: any) => void; notes: string[]; skipped: string[] } {
  const rootNode = parse(text), json = JSON.parse(text), edits: Edit[] = [], todo: ((json: any) => void)[] = [], notes: string[] = [], skipped: string[] = [];
  for (const op of rows) {
    const node = targetNode(rootNode, op.target), current = json.targets?.[op.target];
    if (!node || node.type !== "object") { notes.push(`missing ${op.id}`); continue; }
    const have = current.metadata?.semio?.cache;
    if (have !== undefined && !same(have, op.decision ?? have)) { skipped.push(`${op.id}: decided by a peer as ${have.class}/${have.reason}`); continue; }
    if (op.cache && have !== undefined && have.class !== "deterministic") { skipped.push(`${op.id}: decided by a peer as ${have.class}/${have.reason}`); continue; }
    if (!op.cache && current.cache === true) { skipped.push(`${op.id}: authored cache true by a peer`); continue; }
    const entries: [string, unknown][] = [];
    if (literal(prop(node, "cache")?.value) !== op.cache) entries.push(["cache", op.cache]);
    if (op.continuous && current.continuous !== true) entries.push(["continuous", true]);
    if (op.outputs !== undefined && current.outputs === undefined) entries.push(["outputs", op.outputs]);
    if (op.inputs !== undefined && !same(current.inputs, op.inputs)) entries.push(["inputs", op.inputs]);
    const wantDecision = op.decision !== undefined && have === undefined;
    if (wantDecision) {
      const meta = prop(node, "metadata"), semio = meta?.value.type === "object" ? prop(meta.value, "semio") : undefined;
      if (!meta) entries.push(["metadata", { semio: { cache: op.decision } }]);
      else if (meta.value.type === "object" && !semio) edits.push(...setProps(text, meta.value, [["semio", { cache: op.decision }]]));
      else if (semio?.value.type === "object") edits.push(...setProps(text, semio.value, [["cache", op.decision]]));
      else { notes.push(`metadata is not an object: ${op.id}`); continue; }
    }
    if (entries.length) edits.push(...setProps(text, node, entries));
    if (!entries.length && !wantDecision) continue;
    todo.push((expected) => {
      const t = expected.targets[op.target];
      for (const [key, value] of entries) if (key !== "metadata") t[key] = value;
      if (wantDecision) { t.metadata ??= {}; t.metadata.semio ??= {}; t.metadata.semio.cache = op.decision; }
    });
  }
  return { edits, expect: (expected) => todo.forEach((f) => f(expected)), notes, skipped };
}

const byManifest = new Map<string, Op[]>();
for (const op of ops) (byManifest.get(op.manifest) ?? byManifest.set(op.manifest, []).get(op.manifest)!).push(op);
let changed = 0; const problems: string[] = [], skippedAll: string[] = [];
for (const [manifest, rows] of [...byManifest].sort((a, b) => (a[0] < b[0] ? -1 : 1))) {
  const path = join(root, manifest);
  for (let attempt = 0; ; attempt++) {
    if (attempt > 3) { problems.push(`manifest keeps changing: ${manifest}`); break; }
    const before = readFileSync(path, "utf8"), result = plan(before, rows);
    for (const note of result.notes) problems.push(`${manifest}: ${note}`);
    if (attempt === 0) skippedAll.push(...result.skipped);
    if (!result.edits.length) break;
    const after = apply(before, result.edits), expected = JSON.parse(before);
    result.expect(expected);
    if (!same(JSON.parse(after), expected)) { problems.push(`edit changed more than intended: ${manifest}`); break; }
    if (dry) { changed++; console.log(`would edit\t${manifest}`); break; }
    if (readFileSync(path, "utf8") !== before) continue;
    const temp = `${path}.uncached-owners.tmp`;
    writeFileSync(temp, after); chmodSync(temp, statSync(path).mode);
    if (readFileSync(path, "utf8") !== before) continue;
    renameSync(temp, path); changed++; console.log(`edited\t${manifest}`);
    break;
  }
}
for (const line of skippedAll) console.log(`SKIPPED ${line}`);
if (problems.length) { console.error(problems.map((line) => `PROBLEM ${line}`).join("\n")); process.exit(1); }
console.log(`${dry ? "manifest dry run" : "manifests applied"}: ${changed} manifest(s), ${skippedAll.length} target(s) left to a peer`);
