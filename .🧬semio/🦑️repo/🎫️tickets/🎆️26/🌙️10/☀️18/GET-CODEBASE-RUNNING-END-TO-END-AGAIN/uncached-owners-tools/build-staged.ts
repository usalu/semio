/** 🧱 Stages the library edits as hashes in `plan.json` (no source copy is written: the deepest target path would exceed the path limit inside the ticket folder): the base is the cache-fix staged file when it has one (that set is applied first), otherwise the live file. Reads only; the live tree is never written. */
import { createHash } from "node:crypto";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { EDITS } from "./edits.ts";
import { applyEdits } from "./apply-library.ts";

const root = resolve(import.meta.dir, "../../../../../../../.."), sha = (text: string) => createHash("sha256").update(text).digest("hex");
const entries: { target: string; base: string; baseSha256: string; stagedSha256: string }[] = [];
for (const [target, edits] of Object.entries(EDITS)) {
  const fixed = join(import.meta.dir, "../cache-fix-tools/staged", target), live = join(root, target);
  const [source, base] = existsSync(fixed) ? [fixed, "cache-fix-staged"] : [live, "live"];
  if (!existsSync(source)) throw Error(`no base for ${target}`);
  const before = readFileSync(source, "utf8"), after = applyEdits(before, edits, target);
  entries.push({ target, base, baseSha256: sha(before), stagedSha256: sha(after) });
}
writeFileSync(join(import.meta.dir, "plan.json"), JSON.stringify({ version: 1, entries }, null, 1) + "\n");
console.log(entries.map((entry) => `${entry.base}\t${entry.target.split("/").slice(-3).join("/")}`).join("\n"));
