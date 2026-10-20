/** 🧱 Builds the staged cache-correctness set: applies anchored edits to the live base files into `staged/` and writes `plan.json` for `apply.ts`. Reads only; the live tree is never written. */
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { EDITS, CREATES } from "./edits/all.ts";

const root = join(import.meta.dir, "../../../../../../../.."), here = import.meta.dir;
const sha = (text: string | Buffer) => createHash("sha256").update(text).digest("hex");
const entries: any[] = [];
for (const [target, edits] of Object.entries(EDITS)) {
  const base = readFileSync(join(root, target), "utf8");
  let text = base;
  for (const edit of edits) {
    const count = text.split(edit.old).length - 1;
    if (count !== 1) throw Error(`${target}: anchor occurs ${count} times: ${edit.old.slice(0, 80)}`);
    text = text.replace(edit.old, () => edit.new);
  }
  const path = join(here, "staged", target);
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, text);
  entries.push({ id: target, target, op: "edit", baseSha256: sha(base), stagedSha256: sha(text), edits });
}
for (const [target, content] of Object.entries(CREATES)) {
  const path = join(here, "staged", target);
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, content);
  entries.push({ id: target, target, op: "create", stagedSha256: sha(content), content });
}
writeFileSync(join(here, "plan.json"), JSON.stringify({ version: 1, entries }, null, 1));
console.log(`staged ${entries.length} files`);
