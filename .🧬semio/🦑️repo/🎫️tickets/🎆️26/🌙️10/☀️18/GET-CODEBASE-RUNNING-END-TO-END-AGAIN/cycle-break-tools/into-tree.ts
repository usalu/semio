import { scan } from "../exports-stage-tools/scan.ts";
import { loadPolicy, ownerOfFile } from "../exports-stage-tools/rules.ts";
import { lane } from "../exports-stage-tools/model.ts";
const [tree, fromOwner] = process.argv.slice(2);
const policy = loadPolicy();
const { edges } = scan();
const isTest = (f: string): boolean => /(^|\/)🧪️|\.(test|spec|stories|story)\.[cm]?[jt]sx?$|\/📖️stories\/|(^|\/)📜️script\.ts$|vite(st)?\.config|\/🏗️builder\/|\/📖️/u.test(f);
const by = new Map<string, number>();
const ex: string[] = [];
for (const e of edges) {
  if (e.file.startsWith(tree!) || !e.target.startsWith(tree!)) continue;
  const o = ownerOfFile(policy, e.file); const n = o ? lane(o.owner)?.name ?? o.names[0]! : "-";
  if (fromOwner && n !== fromOwner) continue;
  const k = (isTest(e.file) ? "test " : "run  ") + n; by.set(k, (by.get(k) ?? 0) + 1);
  if (!isTest(e.file) && ex.length < 400) ex.push(`${n}: ${e.file} -> ${e.target}`);
}
console.log([...by].sort((a, b) => b[1] - a[1]).map(([k, v]) => `${v} ${k}`).join("\n"));
if (fromOwner) console.log(ex.join("\n"));
