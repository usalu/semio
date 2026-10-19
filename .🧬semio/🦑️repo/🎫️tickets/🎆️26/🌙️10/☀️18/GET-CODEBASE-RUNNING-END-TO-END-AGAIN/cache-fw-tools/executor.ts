import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { ROOT, TICKET } from "./lib.ts";
import { apply, parse, prop, setProps, type Edit } from "./jsonspan.ts";
const list = JSON.parse(readFileSync(join(TICKET, "rules-fix-tools/non-repo-script-targets.json"), "utf8")).groups["🧰️framework"] as any[];
const byFile = new Map<string, string[]>();
for (const r of list) (byFile.get(r.file) ?? byFile.set(r.file, []).get(r.file)!).push(r.target);
const INF = "semio-framework-os-infinite";
let done = 0, skipped: string[] = [];
for (const [file, targets] of byFile) {
  const text = readFileSync(join(ROOT, file), "utf8");
  const ast = parse(text);
  const json = JSON.parse(text);
  const tn = prop(ast, "targets")!.value;
  const edits: Edit[] = [];
  for (const name of targets) {
    const node = prop(tn, name)?.value;
    if (!node || node.type !== "object") { skipped.push(name); continue; }
    if (prop(node, "executor")) continue;
    const multi = text.slice(node.start, node.end).includes("\n");
    const first = node.props[0]!;
    if (multi) {
      const ls = text.lastIndexOf("\n", first.keyStart - 1) + 1;
      const indent = text.slice(ls, first.keyStart);
      edits.push({ start: first.keyStart, end: first.keyStart, text: `"executor": "repo:script",\n${indent}` });
    } else edits.push({ start: first.keyStart, end: first.keyStart, text: `"executor": "repo:script", ` });
    if (json.name === INF && ["build", "check", "test"].includes(name)) {
      const opts = prop(node, "options")!.value;
      edits.push(...setProps(text, opts, [["cwd", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust"], ["command", `bun ./📜️script.ts ${name}`], ["forwardAllArgs", true]], ["command"]));
    }
  }
  if (!edits.length) continue;
  const next = apply(text, edits);
  JSON.parse(next);
  if (readFileSync(join(ROOT, file), "utf8") !== text) { console.log("stale", file); continue; }
  writeFileSync(join(ROOT, file), next); done++;
}
console.log({ done, skipped });
