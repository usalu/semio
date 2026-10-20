/** 🧪 Runs the staged cache corpus through the staged target-policy overlay and the independent Ajv validator (pre-apply check of the staged fixtures). */
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { load } from "./overlay.ts";
const WS = "/Users/ueli/Documents/semio", L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", I = `${L}/🏗️build/🕸️graph/📥️inference`;
const staged = (path: string) => JSON.parse(readFileSync(join(import.meta.dir, "staged", path), "utf8"));
const Ajv = createRequire(join(WS, "package.json"))("ajv").default ?? createRequire(join(WS, "package.json"))("ajv");
const corpus = staged(`${I}/🧪️tests/💾️cache/🔣️.json`), corpusSchema = staged(`${I}/🧪️tests/💾️cache/🧬️schema/🔣️.json`), cacheSchema = staged(`${L}/🏗️build/🏃️execution/🧬️schema/💾️cache.json`), policySchema = staged(`${L}/⚡️caching/🧬️schema/🔣️.json`), policy = staged(`${L}/⚡️caching/🔣️policy.json`);
const ajv = new Ajv({ strict: true, allErrors: true });
if (!ajv.validate(corpusSchema, corpus)) throw Error("corpus: " + JSON.stringify(ajv.errors));
const ajv2 = new Ajv({ strict: false, allErrors: true });
if (!ajv2.validate(policySchema, policy)) throw Error("policy: " + JSON.stringify(ajv2.errors));
const validateDecision = new Ajv({ strict: true }).compile(cacheSchema);
const { module: m } = await load("staged");
const decisionOf = (t: any) => (t?.metadata?.semio?.cache ? [t.metadata.semio.cache.class, t.metadata.semio.cache.reason] : null);
let n = 0;
for (const row of corpus.decisions) { const ok = validateDecision(row.value); if (ok !== row.valid) throw Error("decision " + row.id); const v = row.value; if (v.version === 1 && Object.keys(v).sort().join() === "class,reason,version") { if (ok) m.cacheDecisionV1(v.class, v.reason); else { let threw = false; try { m.cacheDecisionV1(v.class, v.reason); } catch { threw = true; } if (!threw) throw Error("should refuse " + row.id); } } n++; }
for (const row of corpus.policy) { const t = m.targetPolicy(row.name, { cache: row.authored === false ? false : undefined, ...structuredClone(row.target ?? {}), options: { command: "bun ./📜️script.ts " + row.name, ...row.target?.options } }); if (t.cache !== row.cache || (t.continuous === true) !== (row.continuous === true) || JSON.stringify(decisionOf(t)) !== JSON.stringify(row.decision)) throw Error(`policy ${row.name}: got ${t.cache} ${JSON.stringify(decisionOf(t))}`); if (row.decision && !validateDecision(t.metadata.semio.cache)) throw Error("schema " + row.name); n++; }
for (const row of corpus.outputsGate) { const t = m.cachedOutputsAdmittedV1(m.targetPolicy(row.name, structuredClone(row.target))); if (t.cache !== row.expect.cache || JSON.stringify(decisionOf(t)) !== JSON.stringify(row.expect.decision)) throw Error(`gate ${row.id}: got ${t.cache} ${JSON.stringify(decisionOf(t))}`); n++; }
for (const bad of [{ class: "cacheable", reason: "pure-function-of-inputs" }, { class: "deterministic", reason: "pure-function-of-inputs", note: "x" }]) { let threw = false; try { m.targetPolicy("unheard-of-kind", { metadata: { semio: { cache: { version: 1, ...bad } } } }); } catch { threw = true; } if (!threw) throw Error("open declaration admitted"); n++; }
console.log(`staged corpus ok: ${n} checks`);
