/** 🔁️ Rewrites every workspace package specifier the zero-touch laws report into the relative source import their resolver names. */
import { readFileSync, writeFileSync } from "node:fs";
import { posix } from "node:path";
import { checkZeroTouchClosuresV1, checkZeroTouchCommandsV1, workspaceSpecifierTargetV1, diskZeroTouchTreeV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🛗️zero-touch/🟦️.ts";
const root = process.cwd();
const tree = diskZeroTouchTreeV1(root);
const all = [...checkZeroTouchClosuresV1(root).findings, ...checkZeroTouchCommandsV1(root).findings];
const edits = new Map<string, Set<string>>();
for (const f of all) if (f.kind === "workspace-specifier") (edits.get(f.file) ?? edits.set(f.file, new Set()).get(f.file)!).add(f.specifier);
for (const [file, specs] of edits) {

  let text = readFileSync(file, "utf8");
  for (const spec of specs) {
    const target = workspaceSpecifierTargetV1(tree.packages, spec);
    if (!target) { console.log("NOTARGET", file, spec); continue; }
    let rel = posix.relative(posix.dirname(file), target);
    if (!rel.startsWith(".")) rel = "./" + rel;
    const before = text;
    text = text.split(`"${spec}"`).join(`"${rel}"`).split(`'${spec}'`).join(`'${rel}'`).split("`" + spec + "`").join("`" + rel + "`");
    if (before === text) console.log("NOMATCH", file, spec);
  }
  writeFileSync(file, text);
  console.log("rewrote", file, specs.size);
}
console.log("total findings before", all.length, "other:", all.filter(f => f.kind !== "workspace-specifier").length);
