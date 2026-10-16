const K = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🟦️.ts";
const R = (lo, hi) => (i) => i >= lo && i <= hi;
const set = (...xs) => (i) => xs.includes(i);
const any = (...fs) => (i) => fs.some((f) => f(i));
export const table = [
  ["descriptor", R(10, 18)],
  ["turnOutcome", R(19, 21)],
  ["wasmHandle", R(22, 23)],
  ["contributionsJson", R(24, 26)],
  ["registry", set(27, 29, 30, 31, 32, 288)],
  ["artifactContribution", R(33, 36)],
  ["dialect", R(37, 38)],
  ["appRouter", R(39, 45)],
  ["ioRouter", any(R(46, 57), set(289))],
  ["opening", R(58, 66)],
  ["catalog", any(R(67, 72), set(263))],
  ["mutationVocabulary", R(74, 81)],
  ["fault", R(82, 92)],
  ["effect", R(93, 94)],
  ["mediaExport", R(95, 98)],
  ["videoProgram", R(99, 122)],
  ["videoJob", R(123, 128)],
  ["fileImport", R(129, 138)],
  ["spawnedJob", R(139, 149)],
  ["uiDirty", R(150, 161)],
  ["history", any(R(162, 171), R(177, 182))],
  ["notices", R(172, 176)],
  ["invocation", R(183, 185)],
  ["merge", R(186, 205)],
  ["backbone", R(206, 212)],
  ["activationReason", set(73, 213, 214)],
  ["memory", R(219, 228)],
  ["metrics", R(234, 237)],
  ["activation", any(R(215, 218), R(229, 233), set(238))],
  ["pluginSource", any(R(239, 254), set(290))],
  ["playground", any(R(255, 262), set(264))],
  ["graph", R(265, 278)],
  ["instanceDirectory", R(279, 280)],
  ["routers", R(281, 287)],
];
export const glueDir = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript";
export default {
  file: K,
  parentName: "🎠️kernel",
  keepKey: "parent",
  noBarrel: true,
  assign: (u) => {
    for (const [k, f] of table) if (f(u.idx)) return k;
    return u.exportDecl || u.idx === 207 ? (u.idx === 207 ? "backbone" : "parent") : null;
  },
  parentHeader: () => "",
  childDoc: (m) => `/** ${m.emoji} ${m.doc}\n *\n * 🧱️ Child module of \`🎠️kernel\`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.\n */`,
  postProcess: (k, text) => {
    let t = text;
    if (t.includes("import.meta.vitest")) {
      t = "/// <reference types=\"vitest/importMeta\" />\n" + t;
      t = t.split('await import("./🧪️tests/').join('await import("../🧪️tests/');
      t = t.split('{ directory: (await import("node:path")).dirname((await import("node:url")).fileURLToPath(import.meta.url)), url: import.meta.url }').join('{ directory: (await import("node:path")).dirname((await import("node:path")).dirname((await import("node:url")).fileURLToPath(import.meta.url))), url: import.meta.url }');
    }
    return t;
  },
  modules: [
    { key: "descriptor", dir: "🛂️descriptor-admission", emoji: "🛂️", doc: "Package descriptor admission: the bounded read of a served plugin descriptor, its owner and app-roster checks, the package receipt identity and the package assets it declares." },
    { key: "turnOutcome", dir: "📡️turn-outcome-broadcast", emoji: "📡️", doc: "The multicast queue of one instance's turn replies: the outcome type and `createTurnOutcomeBroadcast` with its own and force-close contract." },
    { key: "wasmHandle", dir: "🪝️plugin-wasm-handle", emoji: "🪝️", doc: "The kernel's plugin handle: the dispatch hint and the `PluginWasmHandle` type every host adapts a loaded plugin to." },
    { key: "contributionsJson", dir: "🧾️contributions-json", emoji: "🧾️", doc: "The contributions JSON a host pushes to the guest: building it from the loaded manifests, scoping it to an app and resolving the layout of a mode." },
    { key: "registry", dir: "🗂️plugin-registry", emoji: "🗂️", doc: "The plugin registry entries and their dependency pins, and `expandPluginRegistry`, which adds a primary plugin's dependencies and host-mode entries." },
    { key: "artifactContribution", dir: "🔖️artifact-contribution", emoji: "🔖️", doc: "What a plugin contributes to an artifact kind: mutation semantics and metadata, inference metadata and the contribution descriptor." },
    { key: "dialect", dir: "🔀️dialect-equality", emoji: "🔀️", doc: "Equality of artifact dialects and app references, shared by the app router, the IO entry graph and the opening resolver." },
    { key: "appRouter", dir: "🧭️app-router", emoji: "🧭️", doc: "The app router: which app opens an artifact dialect in a role, from the manifests of the loaded plugins, with its surface fault codes." },
    { key: "ioRouter", dir: "🚪️io-router", emoji: "🚪️", doc: "The IO entry graph: fidelity and confidence ranking, the entry graph of hops between dialects, `ioRun` and `ioIdentify`." },
    { key: "opening", dir: "📖️opening-resolver", emoji: "📖️", doc: "Opening preferences: the default app per dialect and role, their config mutations and fold, and `resolveOpeningApp`." },
    { key: "catalog", dir: "📚️plugin-catalog", emoji: "📚️", doc: "The plugin catalog: catalog and playground targets, the host config, the activation-event prefixes and the artifact-kind activation owner." },
    { key: "mutationVocabulary", dir: "🧬️mutation-vocabulary", emoji: "🧬️", doc: "The mutation vocabulary an invocation answers with: logical timestamps, artifact diffs, undo policy, inverse and kernel mutations, edit references, undo groups and app events." },
    { key: "fault", dir: "🚨️fault", emoji: "🚨️", doc: "Faults and diagnostics: severity with its wire order, fault origin, scope and cause, text spans and `SemioFaultError`." },
    { key: "effect", dir: "⚡️effect", emoji: "⚡️", doc: "The `Effect` union a guest turn requests from the host, and the service operation payload." },
    { key: "mediaExport", dir: "🎬️media-export", emoji: "🎬️", doc: "Media export encoding: the base64 and UTF-8 encodings, their error and the decoding of an exported payload to bytes." },
    { key: "videoProgram", dir: "🎥️video-render-program", emoji: "🎥️", doc: "The video render program: its limits, vector scene types, admission checks and the decoding of a wire program." },
    { key: "videoJob", dir: "🧵️video-render-job", emoji: "🧵️", doc: "The video render job: encoder tier, job events, outcomes and rows, and the ledger that applies them in order." },
    { key: "fileImport", dir: "🗄️file-open-import", emoji: "🗄️", doc: "File open import: slicing one opened file into wire chunks and the exact argument envelope each chunk is dispatched with." },
    { key: "spawnedJob", dir: "🛠️spawned-job", emoji: "🛠️", doc: "Spawned jobs: placement, the framework reserved job kind, the drive limits, step and completion shapes and the drive error." },
    { key: "uiDirty", dir: "🎯️ui-dirty-scope", emoji: "🎯️", doc: "The UI dirty scope: which window bodies, panel bodies and sections a refresh must rebuild, window layout ids and the merge of two scopes." },
    { key: "history", dir: "🕰️history", emoji: "🕰️", doc: "The history wire every host renders: transactions, mutation entries, time travel, history entries and their labels, reprojection and its status, and the history patch." },
    { key: "notices", dir: "🏷️notice-labels", emoji: "🏷️", doc: "Localized notices: history and framework fault notice labels and the lookup that answers a code in both languages." },
    { key: "invocation", dir: "🧮️invocation-response", emoji: "🧮️", doc: "The invocation response a guest answers a command with, its empty value and its parser." },
    { key: "merge", dir: "🔄️merge-outcome", emoji: "🔄️", doc: "Merge outcomes: merge policy and conflict resolution with their wire order, mutation messages, apply errors, conflicts and dispatch and merge reports." },
    { key: "backbone", dir: "🌉️plugin-backbone-routing", emoji: "🌉️", doc: "Backbone routing: the main-thread relay a direct-import plugin reaches through a well-known global, the inbound push and the per-document routes." },
    { key: "activationReason", dir: "🚦️activation-reason", emoji: "🚦️", doc: "Why an actor is activated: the activation reason, the WIT activation event it justifies and the reason an app id implies." },
    { key: "memory", dir: "🧯️memory-pressure", emoji: "🧯️", doc: "The resident actor cap under memory pressure: the memory probe and the cap it yields." },
    { key: "metrics", dir: "⏱️runtime-metrics", emoji: "⏱️", doc: "Runtime metrics: the per-actor rows, the snapshot and the publish interval rule." },
    { key: "activation", dir: "🎛️activation-registry", emoji: "🎛️", doc: "The activation registry: manifest-only records seeded from a catalog, activation with LRU and memory-pressure suspension, resume from a checkpoint, capability grant intersection and the default asset fetchers." },
    { key: "pluginSource", dir: "🔌️plugin-source", emoji: "🔌️", doc: "Plugin sources: the development, bundled and extension sources, their events, module acquisition and multiplexing." },
    { key: "playground", dir: "🎮️playground-boot", emoji: "🎮️", doc: "Playground resolution: the registry id and default app of a playground, the boot planner and the host config." },
    { key: "graph", dir: "🕸️plugin-graph", emoji: "🕸️", doc: "The plugin dependency graph: version pins, validation, load order, dependents, the graph class and the localized dependency fault message." },
    { key: "instanceDirectory", dir: "📍️instance-directory", emoji: "📍️", doc: "The directory of artifact instances: the instance reference and the directory that registers and finds them." },
    { key: "routers", dir: "🧲️artifact-routers", emoji: "🧲️", doc: "Artifact routers: ownership of a mutation or inference route, conflict-checked registration, and the mutation and inference routers." },
  ],
};
