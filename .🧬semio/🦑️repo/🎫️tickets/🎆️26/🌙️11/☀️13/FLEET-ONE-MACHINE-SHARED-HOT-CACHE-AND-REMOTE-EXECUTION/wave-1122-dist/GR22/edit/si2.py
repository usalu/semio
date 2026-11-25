import sys
p=sys.argv[1];s=open(p,encoding='utf-8').read()
def rep(old,new,count=1):
    global s
    n=s.count(old);assert n==count,(n,old[:140]);s=s.replace(old,new)
rep('''  const compiler = parserTypescriptV1(), host = observedTypeScriptHost(compiler.sys), pass = cache.pass ??= {}, done = new Set(), answers = [], put = new Map(), memo = cache.closureGraphRecords ??= new WeakMap();''','''  const compiler = parserTypescriptV1(), host = observedTypeScriptHost(compiler.sys), pass = closureWalkPassV1(cache), done = new Set(), answers = [], put = new Map(), memo = cache.closureGraphRecords ??= new WeakMap();''')
anchor='''/** 🧭️ The walk question of one target's command entries as {@link targetScriptSources} asks it'''
assert s.count(anchor)==1
block='''const WORKSPACE_NAME_V1 = "{workspaceRoot}/";
const closureRoundV1 = () => { const round = closurePassV1 ?? observationRoundV1(); return round !== null && typeof round === "object" ? round : undefined; };
/** 🗃️ The table of the running validation round in `tables` (made by `make` on first use), undefined outside a round. */
const roundTableV1 = (tables, make) => { const round = closureRoundV1(); if (round === undefined) return undefined; let table = tables.get(round); if (table === undefined) tables.set(round, table = make()); return table; };
const graphAnswersV1 = new WeakMap(), graphTrustV1 = new WeakMap(), storedGraphsV1 = createBoundedCacheV1({ name: "inference:closure-graphs", maxEntries: 4, maxBytes: 4 }), closureGraphSavesV1 = new Set();

/** 🧭️ The pass the walks and the module graph of a cache share in the running validation round: `cache.pass` when a caller set one (a generation deriver), else one per round, so a node is checked once per round and an answer of the round stays its answer; outside a round every walk has its own. */
function closureWalkPassV1(cache) {
  if (cache.pass !== undefined) return cache.pass;
  const passes = cache.roundPasses ??= new WeakMap(), round = closureRoundV1();
  if (round === undefined) return {};
  let pass = passes.get(round);
  if (pass === undefined) passes.set(round, pass = {});
  return pass;
}

/** 🏷️ A walk question as the segments of its `closure-graph` row: workspace names only (`{workspaceRoot}/…`), so the row travels between machines and checkouts unchanged. */
function closureGraphSegmentsV1(cache, workspaceRoot, question) {
  const nameOf = path => closurePathNameV1(cache, workspaceRoot, path);
  return { roots: question.roots.map(nameOf), requiredSources: Boolean(question.requiredSources), resources: (question.resources ?? []).map(nameOf), selectors: [...(question.selectors ?? [])].map(([path, words]) => [nameOf(path), [...words]]), stops: (question.stops ?? []).map(([path, tool]) => [nameOf(path), tool]) };
}
const workspaceNameAdmittedV1 = name => typeof name === "string" && name.startsWith(WORKSPACE_NAME_V1) && !name.split("/").includes("..");
/** 🚏️ Whether the segments of a `closure-graph` row are a walk question: workspace names inside the checkout, selector words and tool names as strings. */
const closureGraphSegmentsAdmittedV1 = segments => segments !== null && typeof segments === "object" && Array.isArray(segments.roots) && segments.roots.every(workspaceNameAdmittedV1) && typeof segments.requiredSources === "boolean" && Array.isArray(segments.resources) && segments.resources.every(workspaceNameAdmittedV1) && Array.isArray(segments.selectors) && segments.selectors.every(row => Array.isArray(row) && row.length === 2 && workspaceNameAdmittedV1(row[0]) && Array.isArray(row[1]) && row[1].every(word => typeof word === "string")) && Array.isArray(segments.stops) && segments.stops.every(row => Array.isArray(row) && row.length === 2 && workspaceNameAdmittedV1(row[0]) && typeof row[1] === "string");
/** 🧭️ The walk question the segments of a `closure-graph` row name, in the paths of `workspaceRoot`. */
function closureGraphQuestionOfV1(workspaceRoot, segments) {
  const local = name => resolve(workspaceRoot, name.slice(WORKSPACE_NAME_V1.length));
  return { roots: segments.roots.map(local), requiredSources: segments.requiredSources, resources: segments.resources.map(local), selectors: segments.selectors.map(([name, words]) => [local(name), words]), stops: segments.stops.map(([name, tool]) => [local(name), tool]) };
}

/** 🗄️ The directory the module graph of a workspace is kept in on this machine: `g/` beside the closure entries the process reads (the store a pool child binds, or the workspace's own), shared by every process of the machine and never swept by the entry collector; undefined where there is no closure store. */
export function closureGraphDirectoryV1(workspaceRoot) { const directory = closureStoreBindingsV1.get(workspaceRoot)?.directory ?? closureStoreDirectoryV1(workspaceRoot); return directory === undefined ? undefined : join(directory, "g"); }
/** 🗄️ The module graph of one script input cache: the machine's stored graph when the cache first needs one (a damaged store counts as none), kept current by every pass of the cache. */
function machineClosureGraphV1(cache, workspaceRoot) {
  if (cache.closureGraph !== undefined) return cache.closureGraph;
  const directory = closureGraphDirectoryV1(workspaceRoot);
  let graph; try { graph = directory === undefined ? undefined : loadClosureGraphSyncV1(directory); } catch {}
  return cache.closureGraph = graph ?? createClosureGraphV1();
}
/** 🗄️ The machine's stored module graph of a workspace for a process that binds no script input cache (a reader that only judges records), loaded again when its head was replaced. */
function storedClosureGraphV1(workspaceRoot) {
  const directory = closureGraphDirectoryV1(workspaceRoot);
  if (directory === undefined) return undefined;
  let mtime; try { mtime = vLstatV1(join(directory, "head.json")).mtimeMs; } catch { return undefined; }
  const held = storedGraphsV1.get(directory);
  if (held?.mtime === mtime) return held.graph;
  let graph; try { graph = loadClosureGraphSyncV1(directory); } catch { return undefined; }
  if (graph !== undefined) storedGraphsV1.set(directory, { mtime, graph });
  return graph;
}
/** 🪪️ The stored nodes of `graph` a pass may use without visiting them: those whose witness holds, each judged once per round (one stat per distinct path and one probe per distinct live row, shared by every node and graph of the round). */
function storedTrustV1(graph) {
  const memo = roundTableV1(graphTrustV1, () => ({ tuples: new Map(), verdicts: new Map(), nodes: new WeakMap() })) ?? { tuples: new Map(), verdicts: new Map(), nodes: new WeakMap() };
  preparedAnalysesV1();
  return { has(key) { const record = graph.get(key); if (record === undefined) return false; let holds = memo.nodes.get(record); if (holds === undefined) { const witness = record.extra?.witness; holds = witness !== null && typeof witness === "object" && observationWitnessHoldsSyncV1(witness, memo); memo.nodes.set(record, holds); } return holds; } };
}

/** 🌳️ The closure digest the module graph of the tree answers for one walk question, once per validation round: the machine's stored graph is the start, a stored node whose witness holds is used as it is, every other node is visited through the walk's own nodes ({@link closureGraphPassV1}), and a graph that changed is saved for the next process of the machine. */
export async function closureGraphAnswerV1(cache, workspaceRoot, question) {
  const root = resolve(workspaceRoot), id = root + "\\0" + JSON.stringify(closureGraphSegmentsV1(cache, root, question)), answers = roundTableV1(graphAnswersV1, () => new Map()), held = answers?.get(id);
  if (held !== undefined) return held;
  const graph = machineClosureGraphV1(cache, root), { digests: [digest], built } = await closureGraphPassV1(cache, root, [question], { graph, witness: true, trusted: storedTrustV1(graph) });
  if (digest === undefined) throw refuseBuildV1("missing", "closure.graph-incomplete", { key: String(question.roots[0] ?? "").slice(0, 160) });
  answers?.set(id, digest);
  if (built > 0) scheduleClosureGraphSaveV1(cache, root);
  return digest;
}

/** 🌳️ The closure digest of a walk question from the stored nodes of `graph` alone, without visiting a module: the traversal of {@link closureGraphPassV1} where every reached node is trusted (`trusted.has`); undefined as soon as a reached node is missing or not trusted (the question must then be passed). */
function closureGraphDigestSyncV1(graph, segments, trusted) {
  const required = segments.requiredSources ? 1 : 0, words = new Map(segments.selectors), resources = new Set(segments.resources), resourceKey = [...resources].sort().join("\\0"), whole = resources.size > 0, roots = segments.roots;
  const keyOf = (name, source, chosen, resource = "") => JSON.stringify([name, source ? 1 : 0, required, resource, chosen ?? null]), stops = new Map(), redirect = new Map(), seen = new Set(), stack = roots.map(name => [name, true]);
  for (const [name, tool] of segments.stops) if (!roots.includes(name) && !words.has(name)) stops.set(keyOf(name, true), tool);
  for (const [name, chosen] of words) for (const source of [true, false]) redirect.set(keyOf(name, source), keyOf(name, source, chosen));
  while (stack.length) {
    const [name, source] = stack.pop(), plain = keyOf(name, source), selected = keyOf(name, source, words.get(name));
    if (seen.has(selected) || source && stops.has(plain)) continue;
    seen.add(selected);
    if (!trusted.has(selected)) return undefined;
    const base = graph.get(selected), varied = whole && base.extra?.urls?.some(url => resources.has(url)) ? keyOf(name, source, words.get(name), resourceKey) : undefined;
    if (varied !== undefined && !trusted.has(varied)) return undefined;
    const held = varied === undefined ? base : graph.get(varied);
    if (held.key !== plain) redirect.set(plain, held.key);
    for (const edge of held.edges) { const [target, flag] = JSON.parse(edge); stack.push([target, flag === 1]); }
  }
  return graph.closureDigest(roots.map(name => redirect.get(keyOf(name, true)) ?? keyOf(name, true)), { stops, redirect });
}

/** 🪪️ Whether the `closure-graph` row of a walk still holds: the digest the running round answered for its question, else the digest the stored nodes of the machine's graph give while every node the question reaches is witnessed ({@link closureGraphDigestSyncV1}); a question that cannot be decided without visiting modules does not hold here (an asynchronous judge prepares it first, {@link prepareClosureGraphRowsV1}), so the verdict is never wrong, at worst an entry is derived again. */
function closureGraphFactHoldsV1(workspaceRoot, segments, digest) {
  if (!closureGraphSegmentsAdmittedV1(segments) || typeof digest !== "string") return false;
  const id = workspaceRoot + "\\0" + JSON.stringify(segments), answers = roundTableV1(graphAnswersV1, () => new Map()), held = answers?.get(id);
  if (held !== undefined) return held === digest;
  const scripts = closureScriptsV1.get(workspaceRoot), graph = scripts !== undefined ? machineClosureGraphV1(scripts, workspaceRoot) : storedClosureGraphV1(workspaceRoot), now = graph === undefined ? undefined : closureGraphDigestSyncV1(graph, segments, storedTrustV1(graph));
  if (now !== undefined) answers?.set(id, now);
  return now === digest;
}
/** 🛎️ Gets the `closure-graph` rows among `rows` answered for the running round, each question once, through the script input cache bound for its workspace; a question whose workspace binds none, or that cannot be walked, is left to {@link closureGraphFactHoldsV1}. */
async function prepareClosureGraphRowsV1(rows) {
  for (const row of rows) {
    let workspaceRoot, segments;
    try { [, workspaceRoot, segments] = JSON.parse(row.path); } catch { continue; }
    const scripts = closureScriptsV1.get(workspaceRoot);
    if (scripts === undefined || !closureGraphSegmentsAdmittedV1(segments)) continue;
    try { await closureGraphAnswerV1(scripts, workspaceRoot, closureGraphQuestionOfV1(workspaceRoot, segments)); } catch {}
  }
}
registerGlobalFactV1(CLOSURE_GRAPH_FACT_V1, closureGraphFactHoldsV1, prepareClosureGraphRowsV1);

/** 💾️ Saves the module graph of a cache into the machine's directory `saveDelayMilliseconds` after it changed, one save at a time and again when it changed meanwhile, and at the latest before the process exits ({@link flushClosureGraphsV1}); a process that only reads the store (a worker child) never writes it. */
function scheduleClosureGraphSaveV1(cache, workspaceRoot) {
  const bound = closureStoreBindingsV1.get(workspaceRoot), directory = closureGraphDirectoryV1(workspaceRoot);
  if (directory === undefined || bound !== undefined && !bound.writable) return;
  const state = cache.closureGraphSave ??= { directory, cache, dirty: false, timer: undefined, running: undefined };
  state.dirty = true;
  closureGraphSavesV1.add(state);
  if (!closureGraphSavesV1.registered) { closureGraphSavesV1.registered = true; process.once("beforeExit", () => { void flushClosureGraphsV1(); }); }
  if (state.timer !== undefined || state.running !== undefined) return;
  state.timer = setTimeout(() => { state.timer = undefined; void saveClosureGraphStateV1(state); }, CLOSURE_GRAPH_POLICY_V1.saveDelayMilliseconds);
  state.timer.unref?.();
}
async function saveClosureGraphStateV1(state) {
  while (state.dirty) { state.dirty = false; state.running = saveClosureGraphV1(state.cache.closureGraph, state.directory).catch(() => undefined); await state.running; state.running = undefined; }
  closureGraphSavesV1.delete(state);
}
/** 💾️ Saves now every module graph of this process that changed since its last save. */
export async function flushClosureGraphsV1() {
  for (const state of [...closureGraphSavesV1]) { if (state.timer !== undefined) { clearTimeout(state.timer); state.timer = undefined; } await state.running; await saveClosureGraphStateV1(state); }
}

'''
s=s.replace(anchor,block+anchor)
rep('''export async function flushClosureOffersV1() { for (const lane of [...closureFleetLanesV1.values()]) await (await lane)?.flush().catch(() => undefined); }''','''export async function flushClosureOffersV1() { for (const lane of [...closureFleetLanesV1.values()]) await (await lane)?.flush().catch(() => undefined); await flushClosureGraphsV1(); }''')
open(p,'w',encoding='utf-8').write(s)
print("ok")
