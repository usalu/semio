F = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"
EDITS = [
(F, '''import { dirname, isAbsolute, join, relative, resolve, sep } from "node:path";''', '''import { basename, dirname, isAbsolute, join, relative, resolve, sep } from "node:path";'''),
(F, '''import { WALK_VIEW_POLICY_V1, walkViewAnswerV1, walkViewOfRecipeV1 } from "./👓️walk-views/🟨️.mjs";''', '''import { WALK_VIEW_POLICY_V1, closureRoleViewV1, walkViewAnswerV1, walkViewOfRecipeV1, walkViewPrefixesV1 } from "./👓️walk-views/🟨️.mjs";
import { PROVIDER_CLOSURE_POLICY_V1, expandClosureValueV1, joinWithProviderViewV1, providerGroupNameV1, providerProjectNameV1, providerReferenceV1, providerReferencesOfV1 } from "./🔌️provider-closures/🟨️.mjs";'''),
(F, '''compiledNames:new WeakMap(),moduleSets:new WeakMap()''', '''compiledNames:new WeakMap(),walkPorts:new WeakMap(),moduleSets:new WeakMap()'''),
(F, '''/** 🔌️ Names what a script that defines provider ports reads at run time and no literal import reaches: the designation, its schema and the designated module of every port its closure defines; a port the designation does not name requests nothing. */
export function scriptProviderImportsV1(cache, ports) {
  const designation = cache.providerDesignation ?? join(LIBRARY_ROOT, SCRIPT_PROVIDER_DESIGNATION), schema = join(LIBRARY_ROOT, SCRIPT_PROVIDER_DESIGNATION_SCHEMA);
  const declared = JSON.parse(readPhysicalSource(designation).toString("utf8"));
  if (validateJsonSchemaSubset(JSON.parse(readPhysicalSource(schema).toString("utf8")), declared).length) throw Error("Complete repository script provider declaration required");
  const requested = ports.includes("*") ? Object.keys(declared.providers) : ports.filter((name) => Object.hasOwn(declared.providers, name));
  return [{ path: designation, source: false }, { path: schema, source: false }, ...requested.map((name) => ({ path: resolve(dirname(designation), declared.providers[name]), source: true }))];
}''', '''/** 📜️ The script provider designation one walk resolves ports through, validated against the schema beside it (where the repository's designation keeps its own): its path, the schema's path and the declared providers. */
function providerDesignationV1(cache) {
  const path = cache.providerDesignation ?? join(LIBRARY_ROOT, SCRIPT_PROVIDER_DESIGNATION), schema = join(dirname(path), relative(dirname(SCRIPT_PROVIDER_DESIGNATION), SCRIPT_PROVIDER_DESIGNATION_SCHEMA));
  const declared = JSON.parse(readPhysicalSource(path).toString("utf8"));
  if (validateJsonSchemaSubset(JSON.parse(readPhysicalSource(schema).toString("utf8")), declared).length) throw Error("Complete repository script provider declaration required");
  return { path, schema, providers: declared.providers };
}

/** 🔌️ The designated ports among the ports one module defines, which a walk names instead of descending into their modules: every designated port when a use is not a literal call (`*`); a port the designation does not name requests nothing. */
export function designatedPortsV1(cache, ports) {
  const { providers } = providerDesignationV1(cache);
  return ports.includes("*") ? Object.keys(providers).sort() : ports.filter((name) => Object.hasOwn(providers, name));
}

/** 🪪️ The providers project: the project whose root holds the designation the walk resolved ports through and declares it (`./🔌️provider-closures`); read through the observed memo, so every project that names a provider records the read. */
function providerProjectV1(cache) {
  const designation = cache.providerDesignation ?? join(LIBRARY_ROOT, SCRIPT_PROVIDER_DESIGNATION), config = join(dirname(designation), PROVIDER_CLOSURE_POLICY_V1.project.basename);
  return memoizedObservedComputationV1(cache.declared, memoDigestV1("provider-project", config), () => {
    if (!existsSync(config)) throw refuseBuildV1("invalid", "closure.provider-project-required", { designation: String(designation) });
    return providerProjectNameV1(JSON.parse(readPhysicalSource(config).toString("utf8")), basename(designation));
  });
}

/** 🔗️ The references a command closure read through `view` holds for the providers of `ports`: one named input of the providers project per port, never the provider's files. */
function providerReferencesV1(cache, ports, view) {
  if (!ports.length) return [];
  const project = providerProjectV1(cache);
  return ports.map(port => providerReferenceV1(project, port, view));
}'''),
(F, '''    if (Array.isArray(part)) { hash.update("["); for (const item of part) hash.update(typeof item === "string" ? item : "\\u0001").update("\\0"); hash.update("]"); }''', '''    if (Array.isArray(part)) { hash.update("["); for (const item of part) hash.update(typeof item === "string" ? item : `\\u0001${JSON.stringify(item)}`).update("\\0"); hash.update("]"); }'''),
(F, '''/** 🧭️ Tracks literal relative command imports through the existing TypeScript tooling boundary; a router script named in `commands` (absolute path to the registered command words it is run with) contributes only the imports those commands need, and every router this reading does not fully understand keeps all of its imports. */
export async function relativeScriptInputs(entries, workspaceRoot, cache = createScriptInputCache(), requiredSources=false, resourceEntries=[], commands=new Map()) {''', '''/** 🧭️ Tracks literal relative command imports through the existing TypeScript tooling boundary, up to the script provider ports: a module that defines ports adds the designated ports to the walk's ports (`cache.walkPorts`) instead of the provider modules, so a walk and its derived fact never contain a provider's files; a router script named in `commands` (absolute path to the registered command words it is run with) contributes only the imports those commands need, and every router this reading does not fully understand keeps all of its imports. */
async function boundedScriptInputsV1(entries, workspaceRoot, cache = createScriptInputCache(), requiredSources=false, resourceEntries=[], commands=new Map()) {'''),
(F, '''  const compiler = parserTypescriptV1(), host = observedTypeScriptHost(compiler.sys), files = new Set(),compiled=new Set();''', '''  const compiler = parserTypescriptV1(), host = observedTypeScriptHost(compiler.sys), files = new Set(),compiled=new Set(),defined=new Set();'''),
(F, '''      const ports = observedSourceAnalysisV1(PROVIDER_PORTS, path), imports = ports.length ? scriptProviderImportsV1(cache, ports) : [], words''', '''      const ports = observedSourceAnalysisV1(PROVIDER_PORTS, path), imports = ports.length ? designatedPortsV1(cache, ports).map(port => ({ port })) : [], words'''),
(F, '''    for (const imported of imports) await visit(imported.path,imported.source);''', '''    for (const imported of imports) if (imported.port === undefined) await visit(imported.path,imported.source); else defined.add(imported.port);'''),
(F, '''  cache.compiledNames.set(result, new Set([...compiled].map((file) => closurePathNameV1(cache, workspaceRoot, file))));''', '''  cache.compiledNames.set(result, new Set([...compiled].map((file) => closurePathNameV1(cache, workspaceRoot, file))));
  cache.walkPorts.set(result, [...defined].sort());'''),
(F, '''   for(const path of compiledPaths??[]){const inner=cache.files.get(JSON.stringify([path,requiredSources,cache.providerDesignation??null,resourceKey,selectors.get(path)??null]));if(inner){let walks=walkMembersV1.get(inner.observations);if(!walks)walkMembersV1.set(inner.observations,walks=new Set());walks.add(question);}}
  }
  return walked;
}''', '''   for(const path of compiledPaths??[]){const inner=cache.files.get(JSON.stringify([path,requiredSources,cache.providerDesignation??null,resourceKey,selectors.get(path)??null]));if(inner){let walks=walkMembersV1.get(inner.observations);if(!walks)walkMembersV1.set(inner.observations,walks=new Set());walks.add(question);}}
  }
  return walked;
}

/** 🧩️ The full command closure of `entries` as a process loads it: the bounded walk and, for every script provider port it names, the designation, its schema and the provider's own bounded walk, transitively; composed from the bounded walks, so each provider walk is computed once and every caller holds the same walk facts. Callers that list files read this (owner discovery, component targets, input precision, a whole-walk closure such as the test discovery); a command closure read through a view names its providers by reference instead (`./🔌️provider-closures`). */
export async function relativeScriptInputs(entries, workspaceRoot, cache = createScriptInputCache(), requiredSources=false, resourceEntries=[], commands=new Map()) {
  const own = await boundedScriptInputsV1(entries, workspaceRoot, cache, requiredSources, resourceEntries, commands), pending = [...(cache.walkPorts.get(own) ?? [])];
  if (!pending.length) return own;
  const designation = providerDesignationV1(cache), files = new Set([...own, closurePathNameV1(cache, workspaceRoot, designation.path), closurePathNameV1(cache, workspaceRoot, designation.schema)]), compiled = new Set(cache.compiledNames.get(own)), seen = new Set(pending);
  for (const walk of await providerWalksV1(cache, workspaceRoot, pending, designation)) { for (const name of walk) files.add(name); for (const name of cache.compiledNames.get(walk) ?? []) compiled.add(name); }
  const result = [...files].sort();
  cache.compiledNames.set(result, compiled);
  return result;
}

/** 🏭️ The bounded walks of the designated modules of `ports` and of every port those walks name in turn; a designated module's sources are required (a designated module that is missing fails closed). */
async function providerWalksV1(cache, workspaceRoot, ports, designation = providerDesignationV1(cache)) {
  const pending = [...ports], seen = new Set(pending), walks = [];
  while (pending.length) {
    const walk = await boundedScriptInputsV1([resolve(dirname(designation.path), designation.providers[pending.shift()])], workspaceRoot, cache, true);
    walks.push(walk);
    for (const port of cache.walkPorts.get(walk) ?? []) if (!seen.has(port)) { seen.add(port); pending.push(port); }
  }
  return walks;
}'''),
(F, '''  return walkViewAnswerV1(value, compiled, generated, view);''', '''  return walkViewAnswerV1(value, compiled, generated, view, cache.walkPorts.get(value) ?? []);'''),
(F, '''  const asked = await observeDetachedV1(() => relativeScriptInputs(question.roots, question.workspaceRoot, cache, question.requiredSources, question.resources, new Map(question.selectors)));''', '''  const asked = await observeDetachedV1(() => boundedScriptInputsV1(question.roots, question.workspaceRoot, cache, question.requiredSources, question.resources, new Map(question.selectors)));'''),
(F, '''  const script = join(LIBRARY_ROOT, "⚡️caching/🦀️cargo/📜️script.ts");''', '''  const script = join(LIBRARY_ROOT, "⚡️caching/🦀️cargo/📜️script.ts"), walk = await boundedScriptInputsV1([script], workspaceRoot, scripts);'''),
(F, '''    ...withoutSchedulingV1(await relativeScriptInputs([script], workspaceRoot, scripts), buildSchedulingNativePrefixesV1()),''', '''    ...withoutSchedulingV1(walk, buildSchedulingNativePrefixesV1()),
    ...providerReferencesV1(scripts, scripts.walkPorts.get(walk) ?? [], "native"),'''),
(F, '''base=baseSize?await relativeScriptInputs([...entries],workspaceRoot,scripts,true,resources,commands):undefined;''', '''base=baseSize?await boundedScriptInputsV1([...entries],workspaceRoot,scripts,true,resources,commands):undefined;'''),
(F, ''' let closure=entries.size===baseSize?base:entries.size?await relativeScriptInputs([...entries],workspaceRoot,scripts,true,resources,commands):undefined,''', ''' let closure=entries.size===baseSize?base:entries.size?await boundedScriptInputsV1([...entries],workspaceRoot,scripts,true,resources,commands):undefined,'''),
(F, ''';const project=closure?declaredProjectClosureV1(''', ''';const walked=closure,project=closure?declaredProjectClosureV1('''),
(F, ''' const kept=closure&&withoutSchedulingV1(closure);''', ''' const references=walked?providerReferencesV1(scripts,scripts.walkPorts.get(walked)??[],"scheduling"):[],pruned=closure&&withoutSchedulingV1(closure),kept=pruned&&references.length?[...pruned,...references]:pruned;'''),
(F, '''  return closure ? [...new Set([...closure, ...fallback])] : fallback;''', '''  return closure ? joinWithProviderViewV1([closure, fallback], "native") : fallback;'''),
(F, '''  const packages = new Set(), graph = existsSync(join(workspaceRoot, javascriptManagerV1.primaryLockName)) ? readBunLockGraph(workspaceRoot) : undefined;
  for (const input of closure) {
    await checkpoint();
    const file = resolve(workspaceRoot, input.slice("{workspaceRoot}/".length));
    if (!existsSync(file)) continue;
    for (const specifier of observedSourceAnalysisV1(prepareSourceAnalysesV1().module,file).imports) {
      if (specifier.startsWith(".") || specifier.startsWith("/") || isBuiltin(specifier) || isRuntimeBuiltinSpecifierV1(specifier)) continue;
      const name = specifier.startsWith("@") ? specifier.split("/").slice(0, 2).join("/") : specifier.split("/")[0];
      if (!graph) { packages.add(name); continue; }
      const key = graph.resolveImport(buildPath(relative(workspaceRoot, file)), name);
      if (key && graph.externalNodes[`npm:${key}`]) packages.add(`npm:${key}`);
    }
  }
  for (const name''', '''  const graph = existsSync(join(workspaceRoot, javascriptManagerV1.primaryLockName)) ? readBunLockGraph(workspaceRoot) : undefined, packages = await closurePackagesV1(closure, workspaceRoot, graph, checkpoint);
  for (const name'''),
(F, '''/** 🪢️ The previous blanket router contract, kept only as a correctness fallback for a command naming no script. */''', '''/** 📦️ The external packages the modules of a closure's files import, as lock nodes when the workspace has a lock graph; references and other non-file items name no module. */
async function closurePackagesV1(closure, workspaceRoot, graph, checkpoint) {
  const packages = new Set();
  for (const input of closure) {
    if (typeof input !== "string") continue;
    await checkpoint();
    const file = resolve(workspaceRoot, input.slice("{workspaceRoot}/".length));
    if (!existsSync(file)) continue;
    for (const specifier of observedSourceAnalysisV1(prepareSourceAnalysesV1().module,file).imports) {
      if (specifier.startsWith(".") || specifier.startsWith("/") || isBuiltin(specifier) || isRuntimeBuiltinSpecifierV1(specifier)) continue;
      const name = specifier.startsWith("@") ? specifier.split("/").slice(0, 2).join("/") : specifier.split("/")[0];
      if (!graph) { packages.add(name); continue; }
      const key = graph.resolveImport(buildPath(relative(workspaceRoot, file)), name);
      if (key && graph.externalNodes[`npm:${key}`]) packages.add(`npm:${key}`);
    }
  }
  return packages;
}

/** 🎭️ A `closure-of` list: read through the view its declared role names (`./👓️walk-views`), the bounded walk without that view's prefixes and its providers by reference; a role read whole gets the full closure (`relativeScriptInputs`). */
async function closureOfV1(entries, workspaceRoot, scripts, extras, role) {
  const view = closureRoleViewV1(role);
  if (view === undefined) return [...await relativeScriptInputs(entries, workspaceRoot, scripts), ...extras];
  const walk = await boundedScriptInputsV1(entries, workspaceRoot, scripts);
  return [...withoutSchedulingV1(walk, walkViewPrefixesV1(view)), ...providerReferencesV1(scripts, scripts.walkPorts.get(walk) ?? [], view), ...extras];
}

/** 🛰️ The named input group of one provider closure read through `view` (`./🔌️provider-closures`): the designation, its schema and the bounded walks of the port's module and of every port they name in turn, without the view's prefixes, and for a view that carries packages the external packages their modules import. A provider closure that imports a generated output is refused: the projects that name it could not depend on its producer. */
async function scriptProviderGroupV1(workspaceRoot, scripts, port, view) {
  const designation = providerDesignationV1(scripts);
  if (!Object.hasOwn(designation.providers, port)) throw refuseBuildV1("invalid", "closure.provider-port-unknown", { port: String(port) });
  const files = new Set();
  for (const walk of await providerWalksV1(scripts, workspaceRoot, [port], designation)) {
    const generated = scripts.generatedImports.size ? generatedTargetsV1(scripts, walk, workspaceRoot) : [];
    if (generated.length) throw refuseBuildV1("invalid", "closure.provider-generated-import", { port: String(port), targets: generated.join(", ") });
    for (const name of withoutSchedulingV1(walk, walkViewPrefixesV1(view))) files.add(name);
  }
  let index = 0;
  const roster = [closurePathNameV1(scripts, workspaceRoot, designation.path), closurePathNameV1(scripts, workspaceRoot, designation.schema), ...[...files].sort()];
  if (!PROVIDER_CLOSURE_POLICY_V1.views[view].packages) return roster;
  const graph = existsSync(join(workspaceRoot, javascriptManagerV1.primaryLockName)) ? readBunLockGraph(workspaceRoot) : undefined;
  return [...roster, { externalDependencies: [...await closurePackagesV1(roster, workspaceRoot, graph, (force = false) => yieldInferenceV1(scripts.context, "build:command-source-packages", ++index, force))].sort() }];
}

/** 🪢️ The previous blanket router contract, kept only as a correctness fallback for a command naming no script. */'''),
(F, '''    case "closure-of": return [...await relativeScriptInputs(recipe.entries.map(entry => resolve(workspaceRoot, entry)), workspaceRoot, scripts), ...recipe.extras];''', '''    case "closure-of": return closureOfV1(recipe.entries.map(entry => resolve(workspaceRoot, entry)), workspaceRoot, scripts, recipe.extras, recipe.role);
    case "script-provider": return scriptProviderGroupV1(workspaceRoot, scripts, recipe.port, recipe.view);'''),
(F, '''const closureAnswerV1 = (entry, query) => {
  const key = JSON.stringify(query);
  if (!entry.answers.has(key)) {
    const run = closureQueriesV1.get(query[0]);
    if (!run) throw refuseBuildV1("invalid", "closure.query-unknown", { query: String(query[0]) });
    entry.answers.set(key, run(entry.value, ...query.slice(1)));
  }
  return entry.answers.get(key);
};''', '''/** 🛰️ The recipe of the provider group one reference names. */
const providerRecipeV1 = reference => ({ fn: "script-provider", port: reference.port, view: reference.view });

/** 🛰️ Brings the provider entries of every reference an entry's value holds up to date, detached: the asking project holds only the answers its queries record. */
async function ensureProviderEntriesV1(workspaceRoot, entry) { for (const reference of providerReferencesOfV1(entry.value)) await ensureClosureEntryV1(workspaceRoot, providerRecipeV1(reference)); }

/** 🧩️ An entry's value with its provider references expanded into the provider files, or undefined while a provider entry is missing or no longer holds. */
function expandedEntryValueV1(workspaceRoot, entry) {
  const references = providerReferencesOfV1(entry.value);
  if (!references.length) return entry.value;
  const files = [];
  for (const reference of references) {
    const provider = closureEntriesV1.get(workspaceRoot + "\\0" + closureRecipeKeyV1(providerRecipeV1(reference)));
    if (!provider || !closureEntryHoldsV1(provider)) return undefined;
    for (const item of provider.value) if (typeof item === "string" && item.startsWith("{workspaceRoot}/")) files.push(item);
  }
  return expandClosureValueV1(entry.value, files);
}

/** ❔️ One query answered over the closure an entry stands for, provider closures expanded; undefined while a provider entry does not hold. An entry without provider references keeps its answers. */
const closureAnswerV1 = (workspaceRoot, entry, query) => {
  const key = JSON.stringify(query), stable = !providerReferencesOfV1(entry.value).length;
  if (stable && entry.answers.has(key)) return entry.answers.get(key);
  const run = closureQueriesV1.get(query[0]);
  if (!run) throw refuseBuildV1("invalid", "closure.query-unknown", { query: String(query[0]) });
  const value = expandedEntryValueV1(workspaceRoot, entry);
  if (value === undefined) return undefined;
  const answer = run(value, ...query.slice(1));
  if (stable) entry.answers.set(key, answer);
  return answer;
};'''),
(F, '''    if (!entry || !closureEntryHoldsV1(entry)) return "!stale";
    return JSON.stringify(closureAnswerV1(entry, query));''', '''    if (!entry || !closureEntryHoldsV1(entry)) return "!stale";
    const answer = closureAnswerV1(workspaceRoot, entry, query);
    return answer === undefined ? "!stale" : JSON.stringify(answer);'''),
(F, '''    for (const [workspaceRoot, recipe, key] of wanted.values()) { try { await ensureClosureEntryV1(workspaceRoot, recipe); } catch { closureEntriesV1.delete(workspaceRoot + "\\0" + key); } }''', '''    for (const [workspaceRoot, recipe, key] of wanted.values()) { try { await ensureProviderEntriesV1(workspaceRoot, await ensureClosureEntryV1(workspaceRoot, recipe)); } catch { closureEntriesV1.delete(workspaceRoot + "\\0" + key); } }'''),
(F, '''  const used = new Map(), fact = async (recipe, query) => { const key = closureRecipeKeyV1(recipe); knownClosureRecipesV1.set(key, recipe); used.set(key, recipe); const entry = await ensureClosureEntryV1(workspaceRoot, recipe), answer = closureAnswerV1(entry, query); recordClosureFactV1(JSON.stringify([workspaceRoot, key, query]), JSON.stringify(answer)); return answer; };''', '''  const used = new Map(), fact = async (recipe, query) => { const key = closureRecipeKeyV1(recipe); knownClosureRecipesV1.set(key, recipe); used.set(key, recipe); const entry = await ensureClosureEntryV1(workspaceRoot, recipe); await ensureProviderEntriesV1(workspaceRoot, entry); const answer = closureAnswerV1(workspaceRoot, entry, query); if (answer === undefined) throw refuseBuildV1("invalid", "closure.provider-entry-stale", { port: providerReferencesOfV1(entry.value).map(row => row.port).join(", ") }); recordClosureFactV1(JSON.stringify([workspaceRoot, key, query]), JSON.stringify(answer)); return answer; };
  const expanded = async value => { const references = providerReferencesOfV1(value); if (!references.length) return value; const files = []; for (const reference of references) for (const item of await scriptProviderGroupV1(workspaceRoot, scripts, reference.port, reference.view)) if (typeof item === "string" && item.startsWith("{workspaceRoot}/")) files.push(item); return expandClosureValueV1(value, files); };'''),
(F, '''    async closureOf(entries, extras) { return late ? closureHandleV1({ fn: "closure-of", entries: relativeEntries(entries), extras }) : [...await relativeScriptInputs(entries, workspaceRoot, scripts), ...extras]; },''', '''    async closureOf(entries, extras, role) { closureRoleViewV1(role); return late ? closureHandleV1({ fn: "closure-of", entries: relativeEntries(entries), extras, role }) : closureOfV1(entries, workspaceRoot, scripts, extras, role); },
    async scriptProviderGroups(projectRoot, declared) {
      const designation = providerDesignationV1(scripts);
      if (resolve(workspaceRoot, projectRoot, declared) !== resolve(designation.path)) throw refuseBuildV1("invalid", "closure.provider-designation-differs", { designation: String(declared) });
      const groups = [];
      for (const port of Object.keys(designation.providers).sort()) for (const view of Object.keys(PROVIDER_CLOSURE_POLICY_V1.views)) groups.push([providerGroupNameV1(port, view), late ? closureHandleV1({ fn: "script-provider", port, view }) : await scriptProviderGroupV1(workspaceRoot, scripts, port, view)]);
      return groups;
    },'''),
(F, '''      return handle ? fact(handle.recipe, ["some", substrings]) : list.some(path => typeof path === "string" && substrings.some(part => path.includes(part)));''', '''      return handle ? fact(handle.recipe, ["some", substrings]) : (await expanded(list)).some(path => typeof path === "string" && substrings.some(part => path.includes(part)));'''),
(F, '''      return handle ? fact(handle.recipe, [name, ...args]) : closureQueriesV1.get(name)(list, ...args);''', '''      return handle ? fact(handle.recipe, [name, ...args]) : closureQueriesV1.get(name)(await expanded(list), ...args);'''),
(F, ''': (await targetScriptClosure(target, workspaceRoot, scripts, projectRoot))?.some(path => typeof path === "string" && substrings.some(part => path.includes(part))) ?? false;''', ''': (await expanded((await targetScriptClosure(target, workspaceRoot, scripts, projectRoot)) ?? [])).some(path => typeof path === "string" && substrings.some(part => path.includes(part)));'''),
(F, '''      for (const [key, recipe] of recipes) { const entry = await ensureClosureEntryV1(workspaceRoot, recipe), view = walkViewOfRecipeV1(recipe.fn);''', '''      for (const [key, recipe] of recipes) { const entry = await ensureClosureEntryV1(workspaceRoot, recipe), view = walkViewOfRecipeV1(recipe);'''),
(F, '''  const api = closureApiV1(closureScriptsV1.get(workspaceRoot) ?? cache, workspaceRoot), handle = closureHandleOfV1(await api.closureOf(entries, [])),''', '''  const api = closureApiV1(closureScriptsV1.get(workspaceRoot) ?? cache, workspaceRoot), handle = closureHandleOfV1(await api.closureOf(entries, [], "test-discovery")),'''),
]
TD = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.d.mts"
EDITS += [
(TD, '''export function scriptProviderImportsV1(cache: ScriptInputsValueV1, ports: readonly string[]): readonly { readonly path: string; readonly source: boolean }[];''', '''export function designatedPortsV1(cache: ScriptInputsValueV1, ports: readonly string[]): readonly string[];'''),
]
X = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️texts/🔣️.json"
EDITS += [
(X, '''    "closure.walk-views-required": {''', '''    "closure.provider-closures-required": {
      "en": "Closed script provider closure policy required",
      "de": "Geschlossene Richtlinie der Skriptanbieter-Abschlüsse erforderlich"
    },
    "closure.provider-project-required": {
      "en": "The directory of the script provider designation {designation} must be the providers project: its project file declares the designation under metadata.semio.scriptProviders",
      "de": "Das Verzeichnis der Skriptanbieter-Bezeichnung {designation} muss das Anbieterprojekt sein: seine Projektdatei deklariert die Bezeichnung unter metadata.semio.scriptProviders"
    },
    "closure.provider-designation-differs": {
      "en": "The providers project declares {designation}, but the command walks resolve ports through another script provider designation",
      "de": "Das Anbieterprojekt deklariert {designation}, aber die Befehlsdurchläufe lösen Ports über eine andere Skriptanbieter-Bezeichnung auf"
    },
    "closure.provider-port-unknown": {
      "en": "The script provider designation names no port {port}",
      "de": "Die Skriptanbieter-Bezeichnung nennt keinen Port {port}"
    },
    "closure.provider-generated-import": {
      "en": "The closure of script provider {port} imports generated outputs of {targets}: a provider closure may not import a generated output, because the projects that name it cannot depend on its producer; import it from the command instead",
      "de": "Der Abschluss des Skriptanbieters {port} importiert generierte Ausgaben von {targets}: ein Anbieterabschluss darf keine generierte Ausgabe importieren, weil die Projekte, die ihn nennen, nicht von ihrem Erzeuger abhängen können; importiere sie stattdessen im Befehl"
    },
    "closure.provider-entry-stale": {
      "en": "The provider closures {port} changed while a query over a command closure was answered; run the command again",
      "de": "Die Anbieterabschlüsse {port} haben sich geändert, während eine Abfrage über einen Befehlsabschluss beantwortet wurde; führe den Befehl erneut aus"
    },
    "closure.walk-views-required": {'''),
]
EDITS += [
(F, '''recordClosureFactV1(JSON.stringify([workspaceRoot, key, ["digest"]]), JSON.stringify(closureAnswerV1(entry, ["digest"])));''', '''recordClosureFactV1(JSON.stringify([workspaceRoot, key, ["digest"]]), JSON.stringify(closureAnswerV1(workspaceRoot, entry, ["digest"])));'''),
]
