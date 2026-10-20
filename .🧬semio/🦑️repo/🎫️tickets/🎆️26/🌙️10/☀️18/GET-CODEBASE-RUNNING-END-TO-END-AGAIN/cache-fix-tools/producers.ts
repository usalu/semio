import { readFileSync, existsSync } from "node:fs";
import { dirname } from "node:path";
const WS = "/Users/ueli/Documents/semio";
const C = process.argv[2];
const list = readFileSync(`${C}/🗑️generated/cachefix-x/pj-list.txt`, "utf8").split("\n").filter(Boolean);
const policy = JSON.parse(readFileSync(`${WS}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json`, "utf8"));
const tax = JSON.parse(readFileSync(`${WS}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, "utf8"));
const contractTargets = new Map<string, string>();
for (const [id, c] of Object.entries<any>(tax.generatorContracts)) { contractTargets.set(c.target, id + ":target"); if (c.previewTarget) contractTargets.set(c.previewTarget, id + ":preview"); if (c.checkTarget) contractTargets.set(c.checkTarget, id + ":check"); }
const matchesCommand = (name: string, cmds: string[]) => cmds.some((c) => name === c || name.startsWith(`${c}-`));
const prod = (n: string) => /^(build|generate|schema|stdio|generator)(-|$)/.test(n.replace(/^[^A-Za-z0-9]+/, "")) || ["wasm","native-build","package","extension-package","cpp-configure","cpp-build","toolchain","artifact-package-contract"].includes(n);
for (const file of list) { let j: any; try { j = JSON.parse(readFileSync(`${WS}/${file}`, "utf8")); } catch { continue; }
  const root = dirname(file); const cargo = existsSync(`${WS}/${root}/Cargo.toml`);
  for (const [name, t] of Object.entries<any>(j.targets ?? {})) {
    if (t.cache === false || matchesCommand(name, policy.continuous) || policy.uncachedExact.includes(name) || matchesCommand(name, policy.uncached)) continue;
    if (!prod(name)) continue; if (/-check$/.test(name)) continue;
    if (t.outputs && t.outputs.length) continue;
    const key = `${j.name}:${name}`;
    const ct = contractTargets.get(key);
    const intent = t.metadata?.semio?.rustBuildIntent ? "rustIntent" : "";
    console.log([key.replace("@semio-tech/",""), ct ?? "-", intent, cargo ? "cargo" : "", (t.options?.command ?? t.command ?? "").slice(0, 80)].join(" | "));
  } }
