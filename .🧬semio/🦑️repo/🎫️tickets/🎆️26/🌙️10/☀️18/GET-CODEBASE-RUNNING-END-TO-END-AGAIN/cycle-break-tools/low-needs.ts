/** 🧱 Framework modules that packages below `framework` need, runtime vs test. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { scan } from "../exports-stage-tools/scan.ts";
import { loadPolicy, ownerOfFile } from "../exports-stage-tools/rules.ts";
import { lane, model } from "../exports-stage-tools/model.ts";
const FW = "@semio-tech/framework";
const policy = loadPolicy();
const isTest = (f: string): boolean => /(^|\/)🧪️|\.(test|spec|stories|story)\.[cm]?[jt]sx?$|\/📖️stories\/|(^|\/)📜️script\.ts$|vite(st)?\.config|\/🏗️builder\/|\/📖️/u.test(f);
const fw = model.byName.get(FW)!;
const keyModule = (sub: string): string => { const v = fw.exports[sub]; const t = typeof v === "string" ? v : JSON.stringify(v); const m = /^\.\/([^/]+)/u.exec(t ?? ""); return m ? m[1]! : "?" + sub; };
const own = new Map<string, string>();
const pk = (f: string): string => { let v = own.get(f); if (v === undefined) { const o = ownerOfFile(policy, f); v = o ? lane(o.owner)?.name ?? o.names[0]! : "-"; own.set(f, v); } return v; };
const modOfFile = (f: string): string => { const m = /^🧰️framework\/(?:🔨️modules|🛍️products\/[^/]+\/🔨️modules)\/([^/]+)/u.exec(f); if (m) return m[1]!; const g = /^🧰️framework\/📦️packages\/🟦️typescript\/([^/]+)/u.exec(f); return g ? g[1]! : "?" + f; };
const { edges, bare } = scan();
const need = new Map<string, { run: Set<string>; test: Set<string> }>();
const put = (x: string, m: string, t: boolean): void => { const e = need.get(x) ?? need.set(x, { run: new Set(), test: new Set() }).get(x)!; (t ? e.test : e.run).add(m); };
for (const e of edges) { const a = pk(e.file), b = pk(e.target); if (b === FW && a !== FW && a !== "-") put(a, modOfFile(e.target), isTest(e.file) || isTest(e.target)); }
for (const b of bare) { const a = pk(b.file); if (b.pkg === FW && a !== FW && a !== "-") put(a, keyModule(b.sub), isTest(b.file)); }
const who = process.argv.slice(2);
for (const [x, v] of [...need].sort()) if (!who.length || who.includes(x)) console.log(x.padEnd(48), "RUN", [...v.run].sort().join(" "), " | TEST", [...v.test].sort().join(" "));
