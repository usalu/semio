/** 🧬 Applies the anchored library edits of `edits.ts` to the tree under `SEMIO_ROOT` (default: this checkout): idempotent, refuses an anchor that does not occur exactly once, writes atomically.
 *
 * Each target is hash-checked against `plan.json` first: `base` (the tree the cache-fix set leaves), `applied` (this set's staged result) or `drift`; a drifted target is refused unless `--rebase` accepts that every anchor still occurs exactly once.
 *
 *   bun apply-library.ts --dry-run   # report only
 *   bun apply-library.ts             # write
 *   bun apply-library.ts --rebase    # accept drifted targets whose anchors still match
 */
import { createHash } from "node:crypto";
import { chmodSync, existsSync, readFileSync, renameSync, statSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { EDITS, type Edit } from "./edits.ts";

const root = process.env.SEMIO_ROOT ?? resolve(import.meta.dir, "../../../../../../../.."), dry = process.argv.includes("--dry-run");
const count = (text: string, part: string) => text.split(part).length - 1;
const sha = (text: string) => createHash("sha256").update(text).digest("hex");
const plan: { entries: { target: string; baseSha256: string; stagedSha256: string }[] } = JSON.parse(readFileSync(join(import.meta.dir, "plan.json"), "utf8"));
const rebase = process.argv.includes("--rebase");

export function applyEdits(text: string, edits: readonly Edit[], label: string): string {
  let out = text;
  for (const edit of edits) {
    if ("fn" in edit) { out = edit.fn(out); continue; }
    if (out.includes(edit.new)) continue;
    const n = count(out, edit.old);
    if (n !== 1) throw Error(`${label}: anchor occurs ${n} times: ${edit.old.slice(0, 90)}`);
    out = out.replace(edit.old, () => edit.new);
  }
  return out;
}

if (import.meta.main) {
  const problems: string[] = []; let changed = 0;
  for (const [target, edits] of Object.entries(EDITS)) {
    const path = join(root, target);
    if (!existsSync(path)) { problems.push(`missing: ${target}`); continue; }
    const before = readFileSync(path, "utf8"), entry = plan.entries.find((row) => row.target === target), hash = sha(before);
    const state = hash === entry?.stagedSha256 ? "applied" : hash === entry?.baseSha256 ? "base" : "drift";
    if (state === "drift" && !rebase) { problems.push(`drift: ${target} matches neither the cache-fix result nor this set's result (re-run build-staged.ts or pass --rebase)`); continue; }
    let after: string;
    try { after = applyEdits(before, edits, target); } catch (error) { problems.push((error as Error).message); continue; }
    if (after === before) { console.log(`unchanged\t${target}`); continue; }
    changed++;
    if (dry) { console.log(`would edit\t${target}`); continue; }
    if (readFileSync(path, "utf8") !== before) { problems.push(`changed while applying: ${target}`); continue; }
    const temp = `${path}.uncached-owners.tmp`;
    writeFileSync(temp, after); chmodSync(temp, statSync(path).mode); renameSync(temp, path);
    console.log(`edited\t${target}`);
  }
  if (problems.length) { console.error(problems.map((line) => `PROBLEM ${line}`).join("\n")); process.exit(1); }
  console.log(`${dry ? "library dry run" : "library applied"}: ${changed} file(s)`);
}
