/** 🧾 Anchored, all-or-nothing applier of `staged/ops.json`: every manifest is re-read, each target must still equal its recorded `before` (or already equal its `after`, then it is left alone); a drifted target refuses the whole run (exit 3) unless `--rebase` accepts it. Edits are spans of the changed top-level keys only, proven semantically equal to the intended end state, written through a temporary file after a last re-read.
 *
 *   bun apply.ts            # dry run (default): prints the plan
 *   bun apply.ts --write    # apply all, or nothing
 *   bun apply.ts --status   # one line per op: pending | applied | drift | pending-requires
 *   bun apply.ts --rebase   # accept drift (targets keep their peer edits for untouched keys)
 * @see staged/plan.ts */
import { chmodSync, readFileSync, renameSync, statSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { apply, parse, prop, removeProp, setProps, type Edit } from "../cache-fix-tools/jsonspan.ts";
import type { Op } from "./staged/plan.ts";

const root = process.env.SEMIO_ROOT ?? resolve(import.meta.dir, "../../../../../../../..");
const argv = new Set(process.argv.slice(2)), write = argv.has("--write"), rebase = argv.has("--rebase"), status = argv.has("--status");
const ops: Op[] = JSON.parse(readFileSync(join(import.meta.dir, "staged/ops.json"), "utf8"));
const canon = (v: unknown): unknown => Array.isArray(v) ? v.map(canon) : v && typeof v === "object" ? Object.fromEntries(Object.entries(v).sort(([a], [b]) => (a < b ? -1 : 1)).map(([k, x]) => [k, canon(x)])) : v;
const same = (a: unknown, b: unknown) => JSON.stringify(canon(a)) === JSON.stringify(canon(b));

type Verdict = "pending" | "applied" | "drift" | "pending-requires" | "missing";
const verdictOf = (json: any, op: Op): Verdict => {
  const current = json.targets?.[op.target];
  if (!current) return "missing";
  if (same(current, op.after)) return "applied";
  if (op.requires.some((r) => !JSON.stringify(json.targets?.[r.target]?.[r.key] ?? null).includes(r.contains))) return "pending-requires";
  return same(current, op.before) ? "pending" : "drift";
};

function editTarget(text: string, op: Op): Edit[] {
  const rootNode = parse(text), targets = prop(rootNode, "targets")?.value, node = targets && prop(targets, op.target)?.value;
  if (!node || node.type !== "object") throw new Error(`target node missing: ${op.id}`);
  const current = JSON.parse(text).targets[op.target], edits: Edit[] = [], fresh: [string, unknown][] = [];
  for (const key of new Set([...Object.keys(current), ...Object.keys(op.after)])) {
    const had = key in current, want = key in op.after;
    if (had && !want) { const e = removeProp(text, node, key); if (e) edits.push(e); }
    else if (!had && want) fresh.push([key, op.after[key]]);
    else if (!same(current[key], op.after[key])) edits.push(...setProps(text, node, [[key, op.after[key]]]));
  }
  if (fresh.length) edits.push(...setProps(text, node, fresh));
  return edits;
}

const byManifest = new Map<string, Op[]>();
for (const op of ops) (byManifest.get(op.manifest) ?? byManifest.set(op.manifest, []).get(op.manifest)!).push(op);
const plan: { path: string; before: string; after: string; ops: Op[] }[] = [], problems: string[] = [], lines: string[] = [];
for (const [manifest, rows] of [...byManifest].sort((a, b) => (a[0] < b[0] ? -1 : 1))) {
  const path = join(root, manifest), before = readFileSync(path, "utf8"), json = JSON.parse(before);
  let text = before; const todo: Op[] = [];
  for (const op of rows) {
    const verdict = verdictOf(json, op);
    lines.push(`${verdict}\t${op.id}`);
    if (verdict === "drift" && !rebase) problems.push(`drift: ${op.id} differs from its recorded before-state (rerun plan.ts to re-record, or --rebase)`);
    if (verdict === "missing") problems.push(`missing: ${op.id}`);
    if (verdict === "pending" || (verdict === "drift" && rebase)) todo.push(op);
  }
  if (!todo.length) continue;
  for (const op of todo) {
    const edits = editTarget(text, op), after = apply(text, edits), expected = JSON.parse(text);
    expected.targets[op.target] = op.after;
    if (!same(JSON.parse(after), expected)) { problems.push(`edit changed more than intended: ${op.id}`); continue; }
    text = after;
  }
  plan.push({ path, before, after: text, ops: todo });
}
if (status) { console.log(lines.sort().join("\n")); process.exit(problems.some((p) => p.startsWith("missing")) ? 1 : 0); }
const pendingRequires = lines.filter((l) => l.startsWith("pending-requires"));
if (problems.length) { console.error(problems.map((p) => `PROBLEM ${p}`).join("\n")); process.exit(problems.some((p) => p.startsWith("drift")) ? 3 : 1); }
for (const row of plan) console.log(`${write ? "writing" : "would edit"}\t${row.path.slice(root.length + 1)}\t${row.ops.map((o) => o.target).join(",")}`);
for (const line of pendingRequires) console.log(`PENDING ${line.split("\t")[1]}: waits for the producer edit it requires (apply demo-bzl first)`);
if (!write) { console.log(`dry run: ${plan.length} manifest(s), ${plan.reduce((n, r) => n + r.ops.length, 0)} target(s), ${pendingRequires.length} pending-requires; nothing written`); process.exit(0); }
const done: typeof plan = [];
try {
  for (const row of plan) {
    if (readFileSync(row.path, "utf8") !== row.before) throw new Error(`manifest changed during the run: ${row.path}`);
    const temporary = `${row.path}.bzl-nocache.tmp`;
    writeFileSync(temporary, row.after); chmodSync(temporary, statSync(row.path).mode);
    renameSync(temporary, row.path); done.push(row);
  }
} catch (error) {
  for (const row of done) writeFileSync(row.path, row.before);
  console.error(`rolled back ${done.length} manifest(s): ${error instanceof Error ? error.message : error}`); process.exit(2);
}
console.log(`applied: ${plan.length} manifest(s), ${plan.reduce((n, r) => n + r.ops.length, 0)} target(s), ${pendingRequires.length} pending-requires`);
