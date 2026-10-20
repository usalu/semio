/** 📊 Runs the live (old) and staged (new) targetPolicy over every explicit target and reports every flip, the classes and the unclassified residue. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { load } from "./overlay.ts";
const WS = "/Users/ueli/Documents/semio", C = process.argv[2]!;
const list = readFileSync(`${C}/🗑️generated/cachefix-x/pj-list.txt`, "utf8").split("\n").filter(Boolean);
const oldP = await load("live"), newP = await load("staged");
const contracts: Record<string, any> = JSON.parse(readFileSync(`${WS}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, "utf8")).generatorContracts;
const injected = new Map<string, string[]>();
for (const c of Object.values<any>(contracts)) { if (c.ownership !== "owned") continue; if (!injected.has(c.target)) injected.set(c.target, []); for (const o of c.outputRoots) { const t = o.producer?.target ?? c.target; if (!injected.has(t)) injected.set(t, []); injected.get(t)!.push(`{workspaceRoot}/${o.path}`); } }
const flips: Record<string, string[]> = {}, classes: Record<string, number> = {}, unclassified: string[] = [];
let total = 0;
for (const file of list) {
  let json: any; try { json = JSON.parse(readFileSync(`${WS}/${file}`, "utf8")); } catch { continue; }
  for (const [name, t] of Object.entries<any>(json.targets ?? {})) {
    total++;
    const base = { ...(oldP.POLICY.targetDefaults?.[name] ?? {}), ...t, ...(t.cache === false && t.inputs === undefined ? { inputs: ["default"] } : {}), options: t.options ?? {} };
    if (typeof t.command === "string" && !base.options.command) base.options = { ...base.options, command: t.command };
    const inj = injected.get(`${json.name}:${name}`); if (inj?.length) base.outputs = inj;
    const o = oldP.module.targetPolicy(name, structuredClone(base), oldP.POLICY), n = newP.module.targetPolicy(name, structuredClone(base), newP.POLICY);
    const gated = newP.module.cachedOutputsAdmittedV1(n);
    const klass = gated.metadata?.semio?.cache ? gated.metadata.semio.cache.class + "/" + gated.metadata.semio.cache.reason : "none(authored cache:false)";
    classes[klass] = (classes[klass] ?? 0) + 1;
    if (klass === "unclassified/no-cache-classification") unclassified.push(`${json.name}:${name}`);
    const key = `${o.cache ? "cached" : "uncached"}->${gated.cache ? "cached" : "uncached"}`;
    if (o.cache !== gated.cache) (flips[key] ??= []).push(`${json.name}:${name} [${klass}]`);
    else flips[key + " (same)"] = (flips[key + " (same)"] ?? []).length ? flips[key + " (same)"] : [];
  }
}
console.log("targets", total, classes);
for (const [k, v] of Object.entries(flips)) console.log(k, v.length);
writeFileSync(join(C, "🗑️generated/cachefix-x/census-flips.json"), JSON.stringify({ flips, unclassified }, null, 1));
console.log("unclassified", unclassified.length, [...new Set(unclassified.map((x) => x.split(":").slice(-1)[0]))].sort().join(" "));
