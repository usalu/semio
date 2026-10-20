/** 📊 Summarises explicit target input/output token shapes across tracked project.json files. */
import { readFileSync } from "node:fs";
const C = process.argv[2];
const list = readFileSync(`${C}/cache-audit-tools/pj-list.txt`, "utf8").split("\n").filter(Boolean);
const tokens: Record<string, number> = {}; const names: Record<string, number> = {}; const exec: Record<string, number> = {}; const cacheVals: Record<string, number> = {}; const envs: Record<string, number> = {}; const optsKeys: Record<string, number> = {};
const bump = (m: Record<string, number>, k: string) => (m[k] = (m[k] ?? 0) + 1);
for (const f of list) {
  const j = JSON.parse(readFileSync(`/Users/ueli/Documents/semio/${f}`, "utf8"));
  for (const [n, t] of Object.entries<any>(j.targets ?? {})) {
    bump(cacheVals, String(t.cache));
    bump(exec, String(t.executor ?? "(default)") + "|" + String(t.metadata?.semio?.execution?.kind ?? "-"));
    for (const k of Object.keys(t.options ?? {})) bump(optsKeys, k);
    for (const i of t.inputs ?? []) {
      if (typeof i === "string") bump(tokens, i.replace(/^!/, "").replace(/\/.*$/, "").slice(0, 40) || i);
      else { const k = Object.keys(i).sort().join(","); bump(tokens, "{" + k + "}"); if (i.env) bump(envs, i.env); }
    }
    if (!Array.isArray(t.inputs)) bump(names, "no-explicit-inputs");
    if (!Array.isArray(t.outputs)) bump(names, "no-explicit-outputs");
  }
}
const top = (m: Record<string, number>, n = 40) => Object.entries(m).sort((a, b) => b[1] - a[1]).slice(0, n);
console.log(JSON.stringify({ cacheVals, exec: top(exec, 15), optsKeys: top(optsKeys, 20), inputTokens: top(tokens, 60), names, envs: top(envs, 40) }, null, 1));
