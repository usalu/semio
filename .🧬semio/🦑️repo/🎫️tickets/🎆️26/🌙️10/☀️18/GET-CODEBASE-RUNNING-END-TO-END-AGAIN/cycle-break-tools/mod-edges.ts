import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { scan } from "../exports-stage-tools/scan.ts";
const [a, b] = process.argv.slice(2);
const { edges } = scan();
const isTest = (f: string): boolean => /(^|\/)🧪️|\.(test|spec|stories|story)\.[cm]?[jt]sx?$|\/📖️stories\/|(^|\/)📜️script\.ts$|vite(st)?\.config|\/🏗️builder\/|\/📖️/u.test(f);
const mod = (f: string): string => { const m = /^(🧰️framework\/(?:🔨️modules|🛍️products\/[^/]+\/🔨️modules)\/[^/]+)/u.exec(f); if (m) return m[1]!.replace("🧰️framework/", ""); const g = /^🧰️framework\/📦️packages\/🟦️typescript\/([^/]+)/u.exec(f); return g ? "glue:" + g[1] : f.split("/").slice(0, 3).join("/"); };
const by = new Map<string, Set<string>>();
for (const e of edges) { if (isTest(e.file) || isTest(e.target)) continue; if (mod(e.file) !== a || mod(e.target) !== b) continue; (by.get(e.target) ?? by.set(e.target, new Set()).get(e.target)!).add(e.file); }
for (const [t, fs] of by) console.log(fs.size, t.replace("🧰️framework/", ""), "<=", [...fs].slice(0, 2).map(x => x.replace("🧰️framework/", "")).join(" | "));
