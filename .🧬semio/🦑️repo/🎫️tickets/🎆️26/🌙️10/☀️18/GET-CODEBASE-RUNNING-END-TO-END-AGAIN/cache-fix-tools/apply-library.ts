/** 🚦 Applies the staged library edits to the live tree after anchoring every edit; refuses and writes nothing when any anchor drifted.
 *
 *   bun apply-library.ts --dry-run   # verify anchors and print the plan
 *   bun apply-library.ts             # verify everything first, then write
 */
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

const here = import.meta.dir, root = process.env.SEMIO_ROOT ?? resolve(here, "../../../../../../../..");
const dry = process.argv.includes("--dry-run");
const sha = (text: string | Buffer) => createHash("sha256").update(text).digest("hex");
const plan = JSON.parse(readFileSync(join(here, "plan.json"), "utf8"));
const writes: { target: string; text: string; how: string }[] = [], refusals: string[] = [];
for (const row of plan.entries) {
  const path = join(root, row.target);
  if (row.op === "create") {
    if (!existsSync(path)) writes.push({ target: row.target, text: row.content, how: "create" });
    else if (sha(readFileSync(path)) === row.stagedSha256) console.log(`skip ${row.id}: already applied`);
    else refusals.push(`${row.id}: target exists with different content`);
    continue;
  }
  const live = readFileSync(path, "utf8");
  if (sha(live) === row.stagedSha256) { console.log(`skip ${row.id}: already applied`); continue; }
  let text = live, ok = true, applied = 0;
  for (const edit of row.edits) {
    const old = text.split(edit.old).length - 1, done = text.split(edit.new).length - 1;
    if (old === 1) { text = text.replace(edit.old, () => edit.new); applied++; }
    else if (old === 0 && done >= 1) continue;
    else { refusals.push(`${row.id}: anchor ${old === 0 ? "missing" : "ambiguous"}: ${edit.old.slice(0, 80)}`); ok = false; break; }
  }
  if (ok && applied) writes.push({ target: row.target, text, how: `patch ${applied}/${row.edits.length} edit(s)` });
}
for (const row of writes) console.log(`${dry ? "would " : ""}${row.how}\t${row.target}`);
if (refusals.length) { console.error(refusals.map((line) => `REFUSED ${line}`).join("\n")); process.exit(1); }
if (!dry) for (const row of writes) {
  const path = join(root, row.target), temp = `${path}.cache-fix.tmp`;
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(temp, row.text);
  renameSync(temp, path);
}
console.log(dry ? `library dry run clean: ${writes.length} writes` : `library applied ${writes.length} writes`);
