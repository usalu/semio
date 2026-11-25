/** 📏️ GR22 measurement of the module closure graph over every owner of the registry in one process: PHASE=cold builds the graph for the walk questions of the owners with a project manifest and the crate nodes of the owners with a Cargo manifest, answers every owner identity and saves the graph to STORE; PHASE=adopt loads STORE in a fresh process, trusts the nodes whose witness or stamps hold, passes again and compares the identities (COMPARE = the cold summary), then measures the read after one module edit and one Rust source edit (the node's trust withdrawn, as an edit withdraws it); PHASE=fleet offers STORE to the fleet and adopts it into a scratch graph by digests. Env: WAVE (directory with session.bin), STORE, OUT (summary json), SA (analysis store copy or off), SCOPE=all|sample, SAMPLE. */
import { readFileSync, writeFileSync, statSync, readdirSync } from "node:fs"; import { join, dirname } from "node:path"; import { deserialize } from "node:v8"; import { createHash } from "node:crypto";
const R = "/Users/ueli/Documents/semio", INF = R + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/", SI = INF + "🧭️script-inputs/", CG = SI + "🌳️closure-graph/", OBS = R + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs";
const { WAVE, SA, SCOPE = "all", SAMPLE = "40", STORE, PHASE = "cold", COMPARE, OUT } = process.env;
const MB = 1048576, peak = () => Math.round(process.resourceUsage().maxRSS / 1024), cpu = () => { const c = process.cpuUsage(); return (c.user + c.system) / 1e6; }, out = { phase: PHASE, scope: SCOPE, steps: [] };
let t0 = performance.now(), c0 = cpu();
const mark = (label, extra = {}) => { const row = { label, wallS: +((performance.now() - t0) / 1000).toFixed(1), cpuS: +(cpu() - c0).toFixed(1), rssMB: Math.round(process.memoryUsage().rss / MB), peakMB: peak(), heapMB: Math.round(process.memoryUsage().heapUsed / MB), ...extra }; out.steps.push(row); console.error("[GR22] " + JSON.stringify(row)); t0 = performance.now(); c0 = cpu(); };
const bytesOf = directory => { let sum = 0; for (const name of readdirSync(directory, { recursive: true })) { try { const info = statSync(join(directory, name)); if (info.isFile()) sum += info.size; } catch {} } return sum; };
const inference = await import(INF + "🟨️.mjs"); await inference.libraryBootstrap;
const si = await import(SI + "🟨️.mjs"), cg = await import(CG + "🟨️.mjs"), cargo = await import(CG + "🦀️cargo/🟨️.mjs"), obs = await import(OBS);
if (SA && SA !== "off") obs.useSourceAnalysisStoreV1({ root: R, directory: SA });
mark("bootstrap");
const session = deserialize(readFileSync(WAVE + "/session.bin")), registry = session.registry.owners;
let owners = registry.map(owner => { const manifest = String(owner.manifest ?? ""); if (manifest.endsWith("Cargo.toml")) return { ...owner, cargo: dirname(manifest) }; try { return { ...owner, json: JSON.parse(readFileSync(join(R, manifest), "utf8")) }; } catch { return undefined; } }).filter(Boolean);
if (SCOPE === "sample") { const n = Number(SAMPLE), step = Math.max(1, Math.floor(owners.length / n)); owners = owners.filter((_, at) => at % step === 0).slice(0, n); }
const cache = inference.createScriptInputCache({ workspaceRoot: R }); cache.pass = {};
const questions = [], ownerQuestions = [], index = new Map(), cargoOwners = owners.filter(owner => owner.cargo !== undefined); let failed = 0, targets = 0;
for (const owner of owners.filter(owner => owner.json !== undefined)) {
  const mine = [];
  for (const [name, target] of Object.entries(owner.json.targets ?? {}).sort(([a], [b]) => a < b ? -1 : 1)) {
    targets++;
    let question; try { question = await si.targetClosureQuestionV1(target, R, cache, owner.root); } catch { failed++; continue; }
    if (!question) continue;
    const text = JSON.stringify([question.roots.sort(), question.selectors, question.resources, question.stops.length]);
    if (!index.has(text)) { index.set(text, questions.length); questions.push(question); }
    mine.push([name, index.get(text)]);
  }
  ownerQuestions.push({ name: owner.name, questions: mine });
}
mark("questions", { registry: registry.length, projectOwners: ownerQuestions.length, cargoOwners: cargoOwners.length, targets, distinctQuestions: questions.length, failedTargets: failed });
const identities = (graph, digests) => [...ownerQuestions.map(owner => createHash("sha256").update(JSON.stringify([owner.name, owner.questions.map(([target, at]) => [target, digests[at]])])).digest("hex")), ...cargoOwners.map(owner => graph.closureDigest([cargo.cargoOwnerRootV1(R, owner.cargo)]) ?? "missing")];
const cargoPass = (graph, held) => { const nodes = new Map(), facts = new Map(); let refused = 0; for (const [at, owner] of cargoOwners.entries()) { try { cargo.cargoCrateNodesV1(R, owner.cargo, { nodes, held, facts }); } catch { refused++; } if (at % 500 === 499) console.error("[GR22] cargo " + (at + 1) + "/" + cargoOwners.length + " nodes " + nodes.size + " rss " + Math.round(process.memoryUsage().rss / MB)); } graph.update(nodes.values()); return { nodes: nodes.size, refused }; };
const control = new AbortController(); let ticks = 0;
const progress = ({ done, total }) => { if (++ticks % 400 === 0) console.error("[GR22] progress " + done + "/" + total + " rss " + Math.round(process.memoryUsage().rss / MB)); };
if (PHASE === "cold") {
  const graph = cg.createClosureGraphV1(), pass = await si.closureGraphPassV1(cache, R, questions, { graph, witness: true, signal: control.signal, progress });
  mark("module-pass", { visited: pass.visited, built: pass.built, nodes: graph.stats().nodes, undefinedDigests: pass.digests.filter(d => d === undefined).length });
  const crates = cargoPass(graph, () => undefined); mark("cargo-pass", { ...crates, nodes: graph.stats().nodes, edges: graph.stats().edges });
  const ids = identities(graph, pass.digests); mark("owner-identities", { owners: ids.length, distinct: new Set(ids).size, missing: ids.filter(id => id === "missing").length });
  const saved = await cg.saveClosureGraphV1(graph, STORE); mark("save", { ...saved, storeBytes: bytesOf(STORE) });
  out.digests = pass.digests; out.identities = ids;
}
if (PHASE === "adopt") {
  const graph = await cg.loadClosureGraphV1(STORE); mark("load", { nodes: graph.stats().nodes, edges: graph.stats().edges });
  const witnessed = await si.closureGraphWitnessedV1(graph), memo = new Map(), stamped = new Set(); for (const record of graph.records()) if (record.kind !== "module" && record.kind !== "resource" && record.kind !== "missing" && record.kind !== "skip" && cargo.cargoNodeStampHoldsV1(R, record, memo)) stamped.add(record.key);
  const trusted = new Set([...witnessed, ...stamped]); mark("trust", { trusted: trusted.size, untrusted: graph.stats().nodes - trusted.size });
  const pass = await si.closureGraphPassV1(cache, R, questions, { graph, trusted, witness: true, progress }); mark("adopt-module-pass", { built: pass.built, visited: pass.visited });
  const crates = cargoPass(graph, key => trusted.has(key) ? graph.get(key) : undefined); mark("adopt-cargo-pass", crates);
  const ids = identities(graph, pass.digests), prior = COMPARE ? JSON.parse(readFileSync(COMPARE, "utf8")).identities : undefined; mark("compare", { owners: ids.length, equal: prior ? ids.filter((id, at) => id === prior[at]).length : null, projectEqual: prior ? ids.slice(0, ownerQuestions.length).filter((id, at) => id === prior[at]).length : null, cargoEqual: prior ? ids.slice(ownerQuestions.length).filter((id, at) => id === prior[ownerQuestions.length + at]).length : null }); out.identities = ids; out.digests = pass.digests;
  const module = [...graph.records()].find(record => record.kind === "module" && record.name.endsWith("/👁️observation/🟨️.mjs")), leaf = [...graph.records()].find(record => record.kind === "file" && record.name.endsWith(".rs") && record.name.includes("/🧠️"));
  const cache2 = inference.createScriptInputCache({ workspaceRoot: R }); cache2.pass = {};
  const edited = new Set([...trusted].filter(key => key !== module?.key)), again = await si.closureGraphPassV1(cache2, R, questions, { graph, trusted: edited, witness: true }); mark("module-edit-pass", { edited: module?.name, built: again.built, questionsMoved: again.digests.filter((d, at) => d !== pass.digests[at]).length });
  const before = identities(graph, again.digests), withoutLeaf = new Set([...trusted].filter(key => key !== leaf?.key && !(leaf && graph.get(key)?.edges.includes(leaf.key) && graph.get(key).kind === "sources")));
  const leafPass = cargoPass(graph, key => withoutLeaf.has(key) ? graph.get(key) : undefined), after = identities(graph, again.digests); mark("rust-edit-pass", { edited: leaf?.name, ...leafPass, ownersMoved: after.filter((id, at) => id !== before[at]).length });
}
if (PHASE === "fleet") {
  const attempt = await import(R + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🤝️selected-exchange/🔌️attempt/🟦️.ts"), fleet = await import(CG + "🛰️fleet/🟦️.ts"), client = attempt.selectedFleetClientV1(R);
  if (!client) { mark("no-fleet-client"); }
  else {
    const offered = await fleet.offerClosureGraphV1({ client, directory: STORE, scope: "gr22-measure" }); mark("offer", offered);
    const again = await fleet.offerClosureGraphV1({ client, directory: STORE, scope: "gr22-measure" }); mark("offer-again", again);
    const graph = cg.createClosureGraphV1(), adopted = await fleet.adoptClosureGraphV1({ client, head: offered.head, directory: STORE + "-adopt", workspaceRoot: R, graph }); mark("adopt-by-digests", { ...adopted, nodes: graph.stats().nodes });
  }
}
if (OUT) writeFileSync(OUT, JSON.stringify(out, null, 1));
process.exit(0);
