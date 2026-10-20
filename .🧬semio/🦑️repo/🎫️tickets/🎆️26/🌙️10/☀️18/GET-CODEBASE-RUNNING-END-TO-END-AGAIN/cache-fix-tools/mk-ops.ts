/** 🧮 Derives the manifest operations that make every authored target agree with the staged classification (read-only; writes `manifest-ops.json`). */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { load } from "./overlay.ts";
const WS = "/Users/ueli/Documents/semio", C = join(import.meta.dir, ".."), here = import.meta.dir;
const list = readFileSync(`${C}/🗑️generated/cachefix-x/pj-list.txt`, "utf8").split("\n").filter(Boolean);
const oldP = await load("live"), newP = await load("staged");
const contracts: Record<string, any> = JSON.parse(readFileSync(`${WS}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, "utf8")).generatorContracts;
const injected = new Map<string, string[]>();
for (const c of Object.values<any>(contracts)) { if (c.ownership !== "owned") continue; if (!injected.has(c.target)) injected.set(c.target, []); for (const o of c.outputRoots) { const t = o.producer?.target ?? c.target; if (!injected.has(t)) injected.set(t, []); injected.get(t)!.push(`{workspaceRoot}/${o.path}`); } }
const D = (klass: string, reason: string) => ({ version: 1, class: klass, reason });
const OVERRIDE: Record<string, { cache: boolean; decision: any }> = {
  "workspace:prepare": { cache: false, decision: D("aggregate", "orders-prerequisites") },
  "@semio-tech/framework-os-dev:build": { cache: false, decision: D("aggregate", "orders-prerequisites") },
  "@semio-tech/mit-bestand-bericht:build": { cache: false, decision: D("aggregate", "orders-prerequisites") },
  "@semio-tech/repo-cli-go:build": { cache: false, decision: D("driver-owned", "freshness-owned-by-driver") },
  "@semio-tech/repo-client:build": { cache: false, decision: D("driver-owned", "freshness-owned-by-driver") },
  "@semio-tech/repo-vscode:build": { cache: false, decision: D("side-effecting", "mutates-workspace") },
  "@semio-tech/repo-vscode:build-vsix": { cache: false, decision: D("side-effecting", "mutates-workspace") },
  "@semio-tech/print:build": { cache: false, decision: D("aggregate", "orders-prerequisites") },
  "@semio-tech/print:build-viz": { cache: false, decision: D("aggregate", "orders-prerequisites") },
  "@semio-tech/framework-os-dev:build-inputs": { cache: false, decision: D("diagnostic", "measures-environment") },
  "@semio-tech/framework-os-dev:plugin": { cache: false, decision: D("driver-owned", "freshness-owned-by-driver") },
  "@semio-tech/repo-dashboard-rs:compile": { cache: false, decision: D("driver-owned", "freshness-owned-by-driver") },
  "@semio-tech/print:fonts": { cache: false, decision: D("managed-output", "outputs-in-managed-store") },
  "semio-framework-canvas:stage-runtime-assets": { cache: false, decision: D("managed-output", "outputs-in-managed-store") },
  "@semio-tech/framework-plugin-web:support": { cache: false, decision: D("managed-output", "outputs-in-managed-store") },
  "semio-framework-os-infinite:fonts": { cache: true, decision: D("deterministic", "pure-function-of-inputs") },
};
type Op = { manifest: string; target: string; id: string; cache?: false; decision: any };
const ops: Op[] = [];
for (const file of list) {
  let json: any; try { json = JSON.parse(readFileSync(`${WS}/${file}`, "utf8")); } catch { continue; }
  for (const [name, t] of Object.entries<any>(json.targets ?? {})) {
    const id = `${json.name}:${name}`;
    const base = { ...(oldP.POLICY.targetDefaults?.[name] ?? {}), ...t, ...(t.cache === false && t.inputs === undefined ? { inputs: ["default"] } : {}), options: t.options ?? {} };
    if (typeof t.command === "string" && !base.options.command) base.options = { ...base.options, command: t.command };
    const inj = injected.get(id); if (inj?.length) base.outputs = inj;
    const o = oldP.module.targetPolicy(name, structuredClone(base), oldP.POLICY);
    const n = newP.module.cachedOutputsAdmittedV1(newP.module.targetPolicy(name, structuredClone(base), newP.POLICY));
    const klass = n.metadata?.semio?.cache;
    const forced = OVERRIDE[id];
    if (forced) { const have = t.metadata?.semio?.cache; if (t.cache === forced.cache && have && JSON.stringify(have) === JSON.stringify(forced.decision)) continue; ops.push({ manifest: file, target: name, id, ...(forced.cache === false ? { cache: false as const } : {}), decision: forced.decision }); continue; }
    if (t.cache === false) continue;
    if (n.cache === false && klass && klass.class !== "continuous") ops.push({ manifest: file, target: name, id, cache: false, decision: { version: 1, class: klass.class, reason: klass.reason } });
    else if (o.cache && !n.cache) throw Error(`flip without class: ${id}`);
  }
}
writeFileSync(join(here, "manifest-ops.json"), JSON.stringify(ops, null, 1));
const by: Record<string, number> = {}; for (const op of ops) { const k = op.decision.class + "/" + op.decision.reason; by[k] = (by[k] ?? 0) + 1; }
console.log(ops.length, "ops in", new Set(ops.map((o) => o.manifest)).size, "manifests", by);
