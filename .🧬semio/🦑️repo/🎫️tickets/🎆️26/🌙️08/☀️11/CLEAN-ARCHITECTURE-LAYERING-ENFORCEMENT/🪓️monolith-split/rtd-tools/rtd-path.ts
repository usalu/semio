// 🧭️ Shortest dev-edge path from a crate to a target crate dir: root dev + normal edges, then normal edges only (the census's dev closure rule). argv: manifest dir (repo-relative) of the root, target package name.
// @see ../RESULT-RENDERER-TEST-DEPS.md
import {join} from "node:path";
import {ROOT, loadNode} from "./rtd-lib.ts";
const root = join(ROOT, process.argv[2]!);
const target = process.argv[3]!;
const queue: {dir: string; path: string[]; viaDev: boolean}[] = [{dir: root, path: [], viaDev: false}];
const seen = new Set<string>([root + "|false"]);
while (queue.length) {
  const cur = queue.shift()!;
  const node = loadNode(cur.dir);
  if (!node) continue;
  if (node.name === target && cur.viaDev) { console.log(cur.path.join(" -> ") + " -> " + node.name); process.exit(0); }
  for (const s of node.specs) {
    if (s.kind === "dev" && cur.dir !== root) continue;
    const via = cur.viaDev || s.kind === "dev";
    const key = s.dir + "|" + via;
    if (seen.has(key)) continue;
    seen.add(key);
    queue.push({dir: s.dir, path: [...cur.path, `${node.name.replace("semio-framework-os-renderer-", "")}${s.kind === "dev" ? " (dev)" : ""}`], viaDev: via});
  }
}
console.log("no dev path");
