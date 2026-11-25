import { readFileSync, readdirSync, statSync } from "node:fs"; import { join } from "node:path"; import { deserialize } from "node:v8"; import { createHash } from "node:crypto";
const R = "/Users/ueli/Documents/semio", INF = R + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/", SI = INF + "🧭️script-inputs/", OBS = R + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs";
const { WAVE, SA, SCOPE = "sample", SAMPLE = "40", STORE, PHASE = "cold", DIRTY } = process.env;
const MB = 1048576, peak = () => Math.round(process.resourceUsage().maxRSS / 1024), cpu = () => { const c = process.cpuUsage(); return (c.user + c.system) / 1e6; }, out = { phase: PHASE, scope: SCOPE, steps: [] };
let t0 = performance.now(), c0 = cpu();
const mark = (label, extra = {}) => { const row = { label, wallS: +((performance.now() - t0) / 1000).toFixed(1), cpuS: +(cpu() - c0).toFixed(1), rssMB: Math.round(process.memoryUsage().rss / MB), peakMB: peak(), heapMB: Math.round(process.memoryUsage().heapUsed / MB), ...extra }; out.steps.push(row); console.error("[GR21] " + JSON.stringify(row)); t0 = performance.now(); c0 = cpu(); };
const inference = await import(INF + "🟨️.mjs"); await inference.libraryBootstrap;
const si = await import(SI + "🟨️.mjs"), cg = await import(SI + "🌳️closure-graph/🟨️.mjs"), obs = await import(OBS);
if (SA && SA !== "off") obs.useSourceAnalysisStoreV1({ root: R, directory: SA });
mark("bootstrap");
const session = deserialize(readFileSync(WAVE + "/session.bin")), registry = session.registry.owners;
const kindOf = json => (json.tags ?? []).find(tag => tag.startsWith("language:")) ?? "project";
let owners = registry.map(owner => { try { return { ...owner, json: JSON.parse(readFileSync(join(R, owner.manifest), "utf8")) }; } catch { return undefined; } }).filter(Boolean);
if (SCOPE === "ts") owners = owners.filter(owner => kindOf(owner.json) === "language:typescript" || kindOf(owner.json) === "project");
if (SCOPE === "sample") { const n = Number(SAMPLE), step = Math.max(1, Math.floor(owners.length / n)); owners = owners.filter((_, at) => at % step === 0).slice(0, n); }
const cache = inference.createScriptInputCache({ workspaceRoot: R }); cache.pass = {};
const questions = [], ownerQuestions = [], index = new Map(); let failed = 0, targets = 0;
for (const owner of owners) {
  const mine = [];
  for (const [name, target] of Object.entries(owner.json.targets ?? {}).sort(([a], [b]) => a < b ? -1 : 1)) {
    targets++;
    let question; try { question = await si.targetClosureQuestionV1(target, R, cache, owner.root); } catch { failed++; continue; }
    if (!question) continue;
    const text = JSON.stringify([question.roots.sort(), question.selectors, question.resources, question.stops.length]);
    if (!index.has(text)) { index.set(text, questions.length); questions.push(question); }
    mine.push([name, index.get(text)]);
  }
  ownerQuestions.push({ name: owner.name, kind: kindOf(owner.json), questions: mine });
}
mark("questions", { owners: owners.length, targets, questionsAsked: ownerQuestions.reduce((s, o) => s + o.questions.length, 0), distinctQuestions: questions.length, failedTargets: failed, kinds: ownerQuestions.reduce((m, o) => (m[o.kind] = (m[o.kind] ?? 0) + 1, m), {}) });
const control = new AbortController(); let ticks = 0;
const identities = digests => ownerQuestions.map(owner => createHash("sha256").update(JSON.stringify([owner.name, owner.questions.map(([target, at]) => [target, digests[at]])])).digest("hex"));
if (PHASE === "cold") {
  const pass = await si.closureGraphPassV1(cache, R, questions, { witness: true, signal: control.signal, progress: ({ done, total }) => { if (++ticks % 200 === 0) console.error("[GR21] progress " + done + "/" + total + " rss " + Math.round(process.memoryUsage().rss / MB)); } });
  const stats = pass.graph.stats(), undef = pass.digests.filter(d => d === undefined).length;
  mark("cold-pass", { visited: pass.visited, built: pass.built, nodes: stats.nodes, edges: stats.edges, componentDigests: stats.digests, families: stats.families, undefinedDigests: undef, walkNodes: cache.walkNodes?.size });
  const started = performance.now(), ids = identities(pass.digests); mark("owner-identities", { owners: ids.length, distinct: new Set(ids).size, perOwnerMicros: +((performance.now() - started) * 1000 / Math.max(1, ids.length)).toFixed(2) });
  const saved = await cg.saveClosureGraphV1(pass.graph, STORE); mark("save", saved);
  if (DIRTY) {
    cache.pass = { dirty: { paths: new Set(DIRTY.split(",")), directories: new Set(), subtrees: new Set() } };
    const warm = await si.closureGraphPassV1(cache, R, questions, { witness: true }); const moved = warm.digests.filter((d, at) => d !== pass.digests[at]).length;
    mark("warm-dirty-pass", { dirty: DIRTY, built: warm.built, visited: warm.visited, questionsMoved: moved });
    const resaved = await cg.saveClosureGraphV1(warm.graph, STORE); mark("resave", resaved);
  }
  cache.pass = {};
  const full = await si.closureGraphPassV1(cache, R, questions, { witness: false }); mark("warm-full-revalidation", { built: full.built, visited: full.visited, questionsMoved: full.digests.filter((d, at) => d !== pass.digests[at]).length });
  out.digests = pass.digests;
}
if (PHASE === "adopt") {
  const graph = await cg.loadClosureGraphV1(STORE); mark("load", { nodes: graph.stats().nodes, edges: graph.stats().edges });
  const trusted = await si.closureGraphWitnessedV1(graph); mark("witness", { trusted: trusted.size, untrusted: graph.stats().nodes - trusted.size });
  const pass = await si.closureGraphPassV1(cache, R, questions, { graph, trusted, witness: true }); mark("adopt-pass", { built: pass.built, visited: pass.visited, undefinedDigests: pass.digests.filter(d => d === undefined).length, walkNodes: cache.walkNodes?.size ?? 0 });
  if (process.env.COMPARE) { const prior = JSON.parse(readFileSync(process.env.COMPARE, "utf8")).digests; out.compare = { equal: pass.digests.filter((d, at) => d === prior[at]).length, total: pass.digests.length }; }
  out.digests = pass.digests;
}
let storeBytes = 0; try { for (const name of readdirSync(join(STORE, "c"))) storeBytes += statSync(join(STORE, "c", name)).size; } catch {}
out.storeBytes = storeBytes; out.load = (await import("node:os")).loadavg();
console.log(JSON.stringify(out));
process.exit(0);
