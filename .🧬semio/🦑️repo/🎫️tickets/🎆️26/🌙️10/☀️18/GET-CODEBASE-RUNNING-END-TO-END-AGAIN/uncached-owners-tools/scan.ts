/** 🔎 Lists every authored target of the tracked manifests with what the live and the staged cache policy resolve it to (read-only; writes `🗑️generated/uncached-owners/rows.json`).
 *
 *   bun scan.ts
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { load } from "../cache-fix-tools/overlay.ts";

const WS = "/Users/ueli/Documents/semio", C = join(import.meta.dir, ".."), G = join(C, "🗑️generated/uncached-owners");
const list = readFileSync(join(G, "pj-list.txt"), "utf8").split("\n").filter(Boolean);
const oldP = await load("live"), newP = await load("staged");
const contracts: Record<string, any> = JSON.parse(readFileSync(`${WS}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, "utf8")).generatorContracts;
const injected = new Map<string, string[]>();
for (const c of Object.values<any>(contracts)) { if (c.ownership !== "owned") continue; if (!injected.has(c.target)) injected.set(c.target, []); for (const o of c.outputRoots) { const t = o.producer?.target ?? c.target; if (!injected.has(t)) injected.set(t, []); injected.get(t)!.push(`{workspaceRoot}/${o.path}`); } }
const rows: any[] = [];
for (const file of list) {
  let json: any; try { json = JSON.parse(readFileSync(`${WS}/${file}`, "utf8")); } catch { continue; }
  const root = file.includes("/") ? file.replace(/\/[^/]*$/, "") : ".";
  for (const [name, t] of Object.entries<any>(json.targets ?? {})) {
    const id = `${json.name}:${name}`;
    const base = { ...(oldP.POLICY.targetDefaults?.[name] ?? {}), ...t, ...(t.cache === false && t.inputs === undefined ? { inputs: ["default"] } : {}), options: t.options ?? {} };
    if (typeof t.command === "string" && !base.options.command) base.options = { ...base.options, command: t.command };
    const inj = injected.get(id); if (inj?.length) base.outputs = inj;
    const o = oldP.module.targetPolicy(name, structuredClone(base), oldP.POLICY);
    const n = newP.module.cachedOutputsAdmittedV1(newP.module.targetPolicy(name, structuredClone(base), newP.POLICY));
    rows.push({
      id, manifest: file, root, project: json.name, target: name, authoredCache: t.cache ?? null,
      authoredDecision: t.metadata?.semio?.cache ?? null, executor: t.executor ?? null, command: t.options?.command ?? t.command ?? null, commands: t.options?.commands ?? null, cwd: t.options?.cwd ?? null,
      outputs: t.outputs ?? null, inputs: t.inputs ?? null, dependsOn: t.dependsOn ?? null,
      oldCache: o.cache, oldDecision: o.metadata?.semio?.cache ?? null, newCache: n.cache, newDecision: n.metadata?.semio?.cache ?? null, continuous: n.continuous === true,
    });
  }
}
writeFileSync(join(G, "rows.json"), JSON.stringify(rows));
const count = (f: (r: any) => string) => { const m: Record<string, number> = {}; for (const r of rows) m[f(r)] = (m[f(r)] ?? 0) + 1; return Object.fromEntries(Object.entries(m).sort()); };
console.log("targets", rows.length);
console.log(count((r) => `authored=${r.authoredCache} new=${r.newCache} decision=${r.newDecision ? r.newDecision.class : "none"}`));
