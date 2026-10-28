F = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"
EDITS = [
(F, '''import { PROVIDER_CLOSURE_POLICY_V1, expandClosureValueV1, joinWithProviderViewV1, providerGroupNameV1, providerProjectNameV1, providerReferenceV1, providerReferencesOfV1 } from "./🔌️provider-closures/🟨️.mjs";''', '''import { PROVIDER_CLOSURE_POLICY_V1, expandClosureValueV1, joinWithProviderViewV1, providerGroupNameV1, providerProjectNameV1, providerReferenceV1, sharedReferencesOfV1, toolGroupNameV1, toolReferenceV1 } from "./🔌️provider-closures/🟨️.mjs";'''),
(F, '''/** 🔗️ The references a command closure read through `view` holds for the providers of `ports`: one named input of the providers project per port, never the provider's files. */
function providerReferencesV1(cache, ports, view) {
  if (!ports.length) return [];
  const project = providerProjectV1(cache);
  return ports.map(port => providerReferenceV1(project, port, view));
}''', '''/** 🔗️ The references a command closure read through `view` holds for the providers of `ports`: one named input of the providers project per port, never the provider's files. */
function providerReferencesV1(cache, ports, view) {
  if (!ports.length) return [];
  const project = providerProjectV1(cache);
  return ports.map(port => providerReferenceV1(project, port, view));
}

/** 🧰️ The reference a command closure holds for the declared tool command `name` (the cargo driver, the lock owner): its closure is the same for every target that runs the tool, so the providers project holds it once (`./🔌️provider-closures`). */
function toolReferencesV1(cache, name) {
  return [toolReferenceV1(providerProjectV1(cache), name)];
}'''),
(F, '''/** 🔧️ Native leaves hash their compiler contract and exact command implementation, independently of UI selection; the scheduling code of leases, capacity, storage, supervision, retries and progress texts stays out of it (it decides when a compile runs, never what it writes), while the native-build modules that assemble the Cargo arguments stay in. */
export async function nativeCommandInputs(workspaceRoot, scripts) {
  const script = join(LIBRARY_ROOT, "⚡️caching/🦀️cargo/📜️script.ts"), walk = await boundedScriptInputsV1([script], workspaceRoot, scripts);
  const cargo = POLICY.toolchains.cargo, javascript = POLICY.toolchains.javascript;
  return [
    ...withoutSchedulingV1(walk, buildSchedulingNativePrefixesV1()),
    ...providerReferencesV1(scripts, scripts.walkPorts.get(walk) ?? [], "native"),''', '''/** 🔧️ Native leaves hash their compiler contract and exact command implementation, independently of UI selection: the cargo driver by reference to its tool closure (read through the native view: the scheduling code of leases, capacity, storage, supervision, retries and progress texts stays out, the native-build modules that assemble the Cargo arguments stay in), the toolchain files, environment and commands by value. */
export async function nativeCommandInputs(workspaceRoot, scripts) {
  const cargo = POLICY.toolchains.cargo, javascript = POLICY.toolchains.javascript;
  return [
    ...toolReferencesV1(scripts, "cargo-command"),'''),
(F, '''/** 🪢️ The previous blanket router contract, kept only as a correctness fallback for a command naming no script. */''', '''/** 🧰️ The named input group of one declared tool command (`./🔌️provider-closures`): the bounded walk of its entry modules (relative to the providers project) and the bounded walks of every provider port they name, transitively, without the tool's view's prefixes, the designation and its schema when it names a port, and its resources. A tool closure that imports a generated output is refused, like a provider closure. */
async function toolClosureGroupV1(workspaceRoot, scripts, name) {
  const tool = PROVIDER_CLOSURE_POLICY_V1.tools.closures[name];
  if (!tool) throw refuseBuildV1("invalid", "closure.tool-unknown", { name: String(name) });
  const designation = providerDesignationV1(scripts), base = dirname(designation.path), walk = await boundedScriptInputsV1(tool.entries.map(entry => resolve(base, entry)), workspaceRoot, scripts), ports = scripts.walkPorts.get(walk) ?? [], files = new Set();
  for (const each of [walk, ...await providerWalksV1(scripts, workspaceRoot, ports, designation)]) {
    const generated = scripts.generatedImports.size ? generatedTargetsV1(scripts, each, workspaceRoot) : [];
    if (generated.length) throw refuseBuildV1("invalid", "closure.provider-generated-import", { port: String(name), targets: generated.join(", ") });
    for (const file of withoutSchedulingV1(each, walkViewPrefixesV1(tool.view))) files.add(file);
  }
  for (const resource of tool.resources) { const path = resolve(base, resource); requirePhysicalSourceV1(path); files.add(closurePathNameV1(scripts, workspaceRoot, path)); }
  if (ports.length) for (const path of [designation.path, designation.schema]) files.add(closurePathNameV1(scripts, workspaceRoot, path));
  return [...files].sort();
}

/** 🪢️ The previous blanket router contract, kept only as a correctness fallback for a command naming no script. */'''),
(F, '''    case "script-provider": return scriptProviderGroupV1(workspaceRoot, scripts, recipe.port, recipe.view);''', '''    case "script-provider": return scriptProviderGroupV1(workspaceRoot, scripts, recipe.port, recipe.view);
    case "tool-closure": return toolClosureGroupV1(workspaceRoot, scripts, recipe.name);'''),
(F, '''/** 🛰️ The recipe of the provider group one reference names. */
const providerRecipeV1 = reference => ({ fn: "script-provider", port: reference.port, view: reference.view });

/** 🛰️ Brings the provider entries of every reference an entry's value holds up to date, detached: the asking project holds only the answers its queries record. */
async function ensureProviderEntriesV1(workspaceRoot, entry) { for (const reference of providerReferencesOfV1(entry.value)) await ensureClosureEntryV1(workspaceRoot, providerRecipeV1(reference)); }

/** 🧩️ An entry's value with its provider references expanded into the provider files, or undefined while a provider entry is missing or no longer holds. */
function expandedEntryValueV1(workspaceRoot, entry) {
  const references = providerReferencesOfV1(entry.value);
  if (!references.length) return entry.value;
  const files = [];
  for (const reference of references) {
    const provider = closureEntriesV1.get(workspaceRoot + "\\0" + closureRecipeKeyV1(providerRecipeV1(reference)));''', '''/** 🛰️ Brings the shared entries of every reference an entry's value holds up to date, detached: the asking project holds only the answers its queries record. */
async function ensureProviderEntriesV1(workspaceRoot, entry) { for (const reference of sharedReferencesOfV1(entry.value)) await ensureClosureEntryV1(workspaceRoot, reference.recipe); }

/** 🧩️ An entry's value with its shared references expanded into the files of the shared closures, or undefined while a shared entry is missing or no longer holds. */
function expandedEntryValueV1(workspaceRoot, entry) {
  const references = sharedReferencesOfV1(entry.value);
  if (!references.length) return entry.value;
  const files = [];
  for (const reference of references) {
    const provider = closureEntriesV1.get(workspaceRoot + "\\0" + closureRecipeKeyV1(reference.recipe));'''),
(F, '''  const key = JSON.stringify(query), stable = !providerReferencesOfV1(entry.value).length;''', '''  const key = JSON.stringify(query), stable = !sharedReferencesOfV1(entry.value).length;'''),
(F, '''if (answer === undefined) throw refuseBuildV1("invalid", "closure.provider-entry-stale", { port: providerReferencesOfV1(entry.value).map(row => row.port).join(", ") });''', '''if (answer === undefined) throw refuseBuildV1("invalid", "closure.provider-entry-stale", { port: sharedReferencesOfV1(entry.value).map(row => row.recipe.port ?? row.recipe.name).join(", ") });'''),
(F, '''  const expanded = async value => { const references = providerReferencesOfV1(value); if (!references.length) return value; const files = []; for (const reference of references) for (const item of await scriptProviderGroupV1(workspaceRoot, scripts, reference.port, reference.view)) if (typeof item === "string" && item.startsWith("{workspaceRoot}/")) files.push(item); return expandClosureValueV1(value, files); };''', '''  const expanded = async value => { const references = sharedReferencesOfV1(value); if (!references.length) return value; const files = []; for (const reference of references) for (const item of await produceClosureRecipeV1(reference.recipe, workspaceRoot, scripts)) if (typeof item === "string" && item.startsWith("{workspaceRoot}/")) files.push(item); return expandClosureValueV1(value, files); };'''),
(F, '''    async scriptProviderGroups(projectRoot, declared) {
      const designation = providerDesignationV1(scripts);
      if (resolve(workspaceRoot, projectRoot, declared) !== resolve(designation.path)) throw refuseBuildV1("invalid", "closure.provider-designation-differs", { designation: String(declared) });
      const groups = [];
      for (const port of Object.keys(designation.providers).sort()) for (const view of Object.keys(PROVIDER_CLOSURE_POLICY_V1.views)) groups.push([providerGroupNameV1(port, view), late ? closureHandleV1({ fn: "script-provider", port, view }) : await scriptProviderGroupV1(workspaceRoot, scripts, port, view)]);
      return groups;
    },''', '''    async toolClosure(name) { return toolReferencesV1(scripts, name); },
    async sharedClosureGroups(projectRoot, declared) {
      const designation = providerDesignationV1(scripts);
      if (resolve(workspaceRoot, projectRoot, declared) !== resolve(designation.path)) throw refuseBuildV1("invalid", "closure.provider-designation-differs", { designation: String(declared) });
      const groups = [];
      for (const port of Object.keys(designation.providers).sort()) for (const view of Object.keys(PROVIDER_CLOSURE_POLICY_V1.views)) groups.push([providerGroupNameV1(port, view), late ? closureHandleV1({ fn: "script-provider", port, view }) : await scriptProviderGroupV1(workspaceRoot, scripts, port, view)]);
      for (const name of Object.keys(PROVIDER_CLOSURE_POLICY_V1.tools.closures).sort()) groups.push([toolGroupNameV1(name), late ? closureHandleV1({ fn: "tool-closure", name, view: PROVIDER_CLOSURE_POLICY_V1.tools.closures[name].view }) : await toolClosureGroupV1(workspaceRoot, scripts, name)]);
      return groups;
    },'''),
]
A = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs"
EDITS += [
(A, '''      const lockOwner = join(LIBRARY_ROOT, "🏗️build/📥️inputs/🔒️lock/📜️script.ts");
      policy.inputs.push(...internCommandSources(await closures.closureOf([lockOwner], [`{workspaceRoot}/${buildPath(relative(workspaceRoot, join(dirname(lockOwner), "🔣️schema.json")))}`], "cargo-lock-owner")), loaded.cargoLockInput(nativeManifest, workspaceRoot));''', '''      policy.inputs.push(...internCommandSources(await closures.toolClosure("cargo-lock-owner")), loaded.cargoLockInput(nativeManifest, workspaceRoot));'''),
(A, '''closures.scriptProviderGroups(root, json.metadata.semio.scriptProviders)''', '''closures.sharedClosureGroups(root, json.metadata.semio.scriptProviders)'''),
]
W = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/👓️walk-views/"
EDITS += [
(W + "🟨️.mjs", '''/** 🎯️ The view every consumer of one closure recipe reads its walks through, or undefined for the whole walk: a `closure-of` recipe reads through the view its declared role names, a `script-provider` recipe through its own view, any other recipe through the view the policy names for it (none: the whole walk). */
export function walkViewOfRecipeV1(recipe) {
  if (recipe.fn === "closure-of") return closureRoleViewV1(recipe.role);
  if (recipe.fn === "script-provider") { walkViewPrefixesV1(recipe.view); return recipe.view; }''', '''/** 🎯️ The view every consumer of one closure recipe reads its walks through, or undefined for the whole walk: a `closure-of` recipe reads through the view its declared role names, a shared closure (`script-provider`, `tool-closure`) through its own view, any other recipe through the view the policy names for it (none: the whole walk). */
export function walkViewOfRecipeV1(recipe) {
  if (recipe.fn === "closure-of") return closureRoleViewV1(recipe.role);
  if (recipe.fn === "script-provider" || recipe.fn === "tool-closure") { walkViewPrefixesV1(recipe.view); return recipe.view; }'''),
(W + "🔣️.json", '''    "cargo-lock-owner": "native",
''', ''''''),
(W + "🧬️schema/🔣️.json", '''"The declared role of every `closure-of` command closure and the view it is read through: a command closure that lands in an action key (the cargo lock owner, an artifact's command) leaves the scheduling code out like every other command closure;''', '''"The declared role of every `closure-of` command closure and the view it is read through: a command closure that lands in an action key (an artifact's command) leaves the scheduling code out like every other command closure (the cargo driver and the lock owner are tool closures of the providers project, read through the native view);'''),
]
X = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📖️texts/🔣️.json"
EDITS += [
(X, '''    "closure.provider-entry-stale": {''', '''    "closure.tool-unknown": {
      "en": "The shared closure policy declares no tool command {name}",
      "de": "Die Richtlinie der geteilten Abschlüsse deklariert keinen Werkzeugbefehl {name}"
    },
    "closure.provider-entry-stale": {'''),
]
EDITS += [
(F, '''import { buildSchedulingNativePrefixesV1, buildSchedulingPrefixesV1 } from "../../../📥️source/🔏️closure/🗓️scheduling/🟦️.ts";''', '''import { buildSchedulingPrefixesV1 } from "../../../📥️source/🔏️closure/🗓️scheduling/🟦️.ts";'''),
]
UL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🧵️unit-cache-lanes/🟦️.ts"
EDITS += [
(UL, '''.closureApiV1(inference.createScriptInputCache({workspaceRoot: root}), root).scriptProviderGroups(''', '''.closureApiV1(inference.createScriptInputCache({workspaceRoot: root}), root).sharedClosureGroups('''),
]
