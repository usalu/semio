/** 📤 Files outside `dir` that files inside `dir` import (relative and workspace-bare), runtime and test separated. */
import { scan } from "../exports-stage-tools/scan.ts";
import { isTest } from "./sim.ts";
const [dir, mode = "runtime"] = process.argv.slice(2);
const { edges, bare } = scan();
const by = new Map<string, Set<string>>();
for (const e of edges) { if (!e.file.startsWith(dir + "/") || e.target.startsWith(dir + "/")) continue; if ((mode === "runtime") === (isTest(e.file) || isTest(e.target))) continue; const k = e.target.replace("🧰️framework/", ""); (by.get(k) ?? by.set(k, new Set()).get(k)!).add(e.file.replace(dir + "/", "")); }
for (const b of bare) { if (!b.file.startsWith(dir + "/")) continue; if ((mode === "runtime") === isTest(b.file)) continue; (by.get(b.spec) ?? by.set(b.spec, new Set()).get(b.spec)!).add(b.file.replace(dir + "/", "")); }
for (const [k, v] of [...by].sort((a, b) => b[1].size - a[1].size)) console.log(v.size, k, "<=", [...v].slice(0, 2).join(" | "));
