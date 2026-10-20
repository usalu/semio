import { readFileSync } from "node:fs";
const WS = "/Users/ueli/Documents/semio";
const C = process.argv[2];
const list = readFileSync(`${C}/🗑️generated/cachefix-x/pj-list.txt`, "utf8").split("\n").filter(Boolean);
const policy = JSON.parse(readFileSync(`${WS}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json`, "utf8"));
const matchesCommand = (name: string, cmds: string[]) => cmds.some((c) => name === c || name.startsWith(`${c}-`));
const heads = ["test","check","lint","typecheck","format-check","generate","schema","verify","stdio","build"];
const exact = ["wasm","native-build","package","extension-package","cpp-configure","cpp-build","cpp-test","graph-check","policy-check","artifact-check","artifact-package-contract","toolchain"];
const branch = (n: string) => { for (const h of heads) if (n === h || n.startsWith(h + "-")) return "head:" + h; if (exact.includes(n)) return "exact"; if (/(?:^|-)contract(?:-|$)/.test(n)) return "contract"; if (/^generator(?:-|$)/.test(n) && !n.includes("generator-inputs")) return "generator"; return null; };
const agg: Record<string, { n: number; empty: number; names: Set<string> }> = {};
for (const file of list) { let j: any; try { j = JSON.parse(readFileSync(`${WS}/${file}`, "utf8")); } catch { continue; }
  for (const [name, t] of Object.entries<any>(j.targets ?? {})) {
    if (t.cache === false || matchesCommand(name, policy.continuous) || policy.uncachedExact.includes(name) || matchesCommand(name, policy.uncached)) continue;
    const b = branch(name); if (!b) continue;
    const a = (agg[b] ??= { n: 0, empty: 0, names: new Set() }); a.n++; if (!t.outputs || !t.outputs.length) a.empty++; if (a.names.size < 400) a.names.add(name);
  } }
for (const [k, v] of Object.entries(agg).sort()) console.log(k, v.n, "emptyOutputs", v.empty, "distinct", v.names.size);
