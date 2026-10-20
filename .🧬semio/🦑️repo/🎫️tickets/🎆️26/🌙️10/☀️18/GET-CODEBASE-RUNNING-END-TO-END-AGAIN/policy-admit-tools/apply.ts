/**
 * 🚦 Applies the staged SDK foundation admission to the live tree after anchoring every target.
 *
 *   bun apply.ts --dry-run   # verify anchors and print the plan, write nothing
 *   bun apply.ts             # verify everything first, then write all targets
 *
 * A target whose live hash equals the staged base is replaced by the staged copy; a drifted target is patched in place only when every edit anchor still occurs exactly once; anything else refuses and nothing is written.
 */
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const here = import.meta.dir;
const root = process.env.SEMIO_ROOT ?? resolve(here, "../../../../../../../..");
const dry = process.argv.includes("--dry-run");
const sha = (text: string | Buffer): string => createHash("sha256").update(text).digest("hex");
const plan = JSON.parse(readFileSync(join(here, "plan.json"), "utf8"));
const staged = (name: string): string => readFileSync(join(here, "staged", name), "utf8");
const writes: { target: string; text: string; how: string }[] = [];
const refusals: string[] = [];

for (const row of plan.entries) {
  const path = join(root, row.target), text = staged(row.staged);
  if (sha(text) !== row.stagedSha256) { refusals.push(`${row.id}: staged copy changed since build (${row.staged})`); continue; }
  if (row.op === "create") {
    if (!existsSync(path)) writes.push({ target: row.target, text, how: "create" });
    else if (sha(readFileSync(path)) === row.stagedSha256) console.log(`skip ${row.id}: already applied`);
    else refusals.push(`${row.id}: target exists with different content`);
    continue;
  }
  const live = readFileSync(path, "utf8");
  if (sha(live) === row.stagedSha256) { console.log(`skip ${row.id}: already applied`); continue; }
  if (sha(live) === row.baseSha256) { writes.push({ target: row.target, text, how: "replace (hash equals base)" }); continue; }
  let patched = live, ok = true;
  for (const edit of row.edits) {
    if (patched.split(edit.old).length - 1 !== 1 || patched.includes(edit.new)) { refusals.push(`${row.id}: drift, anchor missing or ambiguous: ${edit.old.slice(0, 70)}`); ok = false; break; }
    patched = patched.replace(edit.old, () => edit.new);
  }
  if (ok) writes.push({ target: row.target, text: patched, how: "patch (drifted, anchors intact)" });
}

for (const row of writes) console.log(`${dry ? "would " : ""}${row.how}\t${row.target}`);
if (refusals.length) { console.error(refusals.map(line => `REFUSED ${line}`).join("\n")); process.exit(1); }
if (!dry) for (const row of writes) {
  const path = join(root, row.target);
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, row.text);
}
console.log(dry ? `dry run clean: ${writes.length} writes` : `applied ${writes.length} writes`);
