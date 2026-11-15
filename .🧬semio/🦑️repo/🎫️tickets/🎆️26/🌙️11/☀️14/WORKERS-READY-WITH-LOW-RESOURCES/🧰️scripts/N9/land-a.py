#!/usr/bin/env python3
"""Landing A of package N9: the conformance stamps wired into the inference protocol. usage: land-a.py check|apply  (anchored edits, every anchor asserted, root-relative)."""
import json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = HERE
while not os.path.exists(os.path.join(ROOT, "AGENTS.md")):
    ROOT = os.path.dirname(ROOT)
LIB = os.path.join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library")
INF = os.path.join(LIB, "🏗️build/🕸️graph/📥️inference")
APPLY = sys.argv[1] == "apply"
M = "⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/"
P = "🏗️build/📥️inputs/🎯️precision/"
CONF = "🏗️build/🕸️graph/📥️inference/🔖️stamps/🧮️conformance/"


def read(path):
    return open(path, encoding="utf8").read()


def write(path, text):
    if APPLY:
        open(path, "w", encoding="utf8").write(text)


def patch(path, edits):
    text = read(path)
    for old, new in edits:
        assert text.count(old) == 1, (path, old[:80], text.count(old))
        text = text.replace(old, new)
    write(path, text)


def jpatch(path, change):
    text = read(path)
    data = json.loads(text)
    assert json.dumps(data, ensure_ascii=False, indent=2) + "\n" == text, path
    change(data)
    write(path, json.dumps(data, ensure_ascii=False, indent=2) + "\n")


def schema(data):
    d = data["definitions"]
    p = d["Protocol"]["const"]
    p["inferrers"]["input-precision"] = {"stamp": "semio.graph-inferrer.input-precision/1", "scope": "project", "sources": [P + "🟦️.ts", P + "🧬️schema/"]}
    p["inferrers"]["package-managers"] = {"stamp": "semio.graph-inferrer.package-managers/1", "scope": "project", "sources": [M + "🟦️.ts", M + "📚️catalog/", M + "📖️formats/", M + "🍞️bun/", M + "📦️npm/", M + "🦀️cargo/", M + "🐹️go/", M + "🐍️uv/", M + "🔷️dotnet/", M + "🔣️catalog.json", M + "🔣️policy.json", M + "🧬️schema/"]}
    p["derived"] = ["input-precision", "package-managers"]
    p["none"] = p.pop("none")
    doc = d["Document"]
    doc["required"] = ["version", "scopes", "inferrers", "derived", "none"]
    doc["properties"]["derived"] = {"type": "array", "uniqueItems": True, "items": {"$ref": "#/definitions/Id"}}
    data["description"] += " `derived` names the inferrers whose effective stamp is the declared stamp plus a digest of what their code answers on the conformance corpus (`../🧮️conformance`): an edit that changes no answer leaves the stamp and re-infers nothing. While such a stamp is not available the sources of the inferrer count as content facts again."


def roster(data):
    for entry in data["components"]:
        if entry["name"] in ("bootstrap", "project-assembly") and "stamps" not in entry["needs"]:
            entry["needs"].insert(1 if entry["name"] == "bootstrap" else 1, "stamps")


STAMPS_IMPORT = 'import { declareFormatStampV1 } from "../../../🧷️format-stamps/🟨️.mjs";\n'
patch(os.path.join(INF, "🔖️stamps/🟨️.mjs"), [
    (STAMPS_IMPORT, STAMPS_IMPORT + 'import { conformanceTextV1, resolveStampV1 } from "./🧮️conformance/🟨️.mjs";\nimport { homedir } from "node:os";\nimport { join } from "node:path";\nimport { fileURLToPath } from "node:url";\n'),
    ('let protocol;\n', 'let protocol;\nconst effective = new Map();\nlet fleetPort;\n'),
    ('closed(value, ["version", "scopes", "inferrers", "none"], "protocol");', 'closed(value, ["version", "scopes", "inferrers", "derived", "none"], "protocol");'),
    ('  const ids = Object.keys(value.inferrers);\n  if (!ids.length) throw Error("Inferrers required");\n',
     '  const ids = Object.keys(value.inferrers);\n  if (!ids.length) throw Error("Inferrers required");\n  if (!Array.isArray(value.derived) || new Set(value.derived).size !== value.derived.length || value.derived.some((id) => !ids.includes(id))) throw Error("Distinct derived inferrers of the protocol required");\n'),
    ('  registerInferenceStampsV1(inferenceStampEntriesV1(protocol), inferenceFragmentsV1(protocol));\n}\n',
     '  effective.clear();\n  registerInferenceStampsV1(inferenceStampEntriesV1(protocol), inferenceFragmentsV1(protocol));\n}\n'),
    ('/** 🧭️ Every path fragment the protocol covers, by stamp or as infrastructure. */\nexport function inferenceFragmentsV1(document) {\n  return [...document.none, ...Object.values(document.inferrers).flatMap((group) => group.sources)];\n}\n\n/** 🗂️ The registry entries of a protocol: one `inferrer:<id>` stamp per inferrer. */\nexport function inferenceStampEntriesV1(document) {\n  return Object.entries(document.inferrers).map(([id, row]) => ["inferrer:" + id, row.stamp]);\n}\n',
     '''/** 🎚️ True while an inferrer carries a stamp: every declared one, and a derived one only once its conformance stamp exists; without it its sources count as content facts again. */
export const inferrerActiveV1 = (id, stamps = effective, document = protocol) => !document?.derived.includes(id) || stamps.has(id);

/** 🧭️ Every path fragment the protocol covers, by stamp or as infrastructure. */
export function inferenceFragmentsV1(document, stamps = effective) {
  return [...document.none, ...Object.entries(document.inferrers).filter(([id]) => inferrerActiveV1(id, stamps, document)).flatMap(([, group]) => group.sources)];
}

/** 🗂️ The registry entries of a protocol: one `inferrer:<id>` stamp per active inferrer, the effective stamp of a derived one. */
export function inferenceStampEntriesV1(document, stamps = effective) {
  return Object.entries(document.inferrers).filter(([id]) => inferrerActiveV1(id, stamps, document)).map(([id, row]) => ["inferrer:" + id, stamps.get(id) ?? row.stamp]);
}
'''),
    ('export function projectStampPathsV1({ native }) {\n  return Object.entries(protocol.inferrers).filter(([, row]) => row.scope === "project"',
     'export function projectStampPathsV1({ native }) {\n  return Object.entries(protocol.inferrers).filter(([id]) => inferrerActiveV1(id)).filter(([, row]) => row.scope === "project"'),
    ('export function classifyImplementationRowsV1(rows, document = protocol) {\n  const out = [], none = document.none, groups = Object.entries(document.inferrers);',
     'export function classifyImplementationRowsV1(rows, document = protocol, stamps = effective) {\n  const out = [], none = document.none, groups = Object.entries(document.inferrers).filter(([id]) => inferrerActiveV1(id, stamps, document));'),
    ('  for (const [id, group] of groups) out.push({ kind: "stamp", path: inferrerStampPathV1(id), fact: group.stamp, scope: group.scope });',
     '  for (const [id, group] of groups) out.push({ kind: "stamp", path: inferrerStampPathV1(id), fact: stamps.get(id) ?? group.stamp, scope: group.scope });'),
])

CONFORMANCE_BLOCK = '''
/** 🛰️ Binds the port that shares conformance records through the fleet cache (`get(id, key)`, `put(id, key, record)`); a record from the fleet is used only after it checks against its key and its parts. */
export function bindConformanceFleetPortV1(port) { fleetPort = port; }

/** 📁️ The per user directory of the conformance records: a record is named by the content digest of its closure, so one directory serves every checkout of the machine. */
export function conformanceDirectoryV1(environment = process.env, platform = process.platform, home = homedir()) {
  const base = platform === "darwin" ? join(home, "Library", "Caches") : platform === "win32" ? environment.LOCALAPPDATA ?? join(home, "AppData", "Local") : environment.XDG_CACHE_HOME?.startsWith("/") ? environment.XDG_CACHE_HOME : join(home, ".cache");
  return join(base, "semio", "inference-conformance");
}

/** 🧮️ Settles the conformance stamp of every derived inferrer once per process: the local record of its closure, the fleet's, or one evaluation of the corpus; an inferrer without a stamp keeps the content facts of its files. Never fails and never waits longer than the evaluation limit. */
export async function settleConformanceStampsV1({ libraryRoot, directory = conformanceDirectoryV1(), runners, fleet = fleetPort, signal, say = (process.env.SEMIO_GRAPH_PHASES === "1" ? (text) => process.stderr.write("[conformance] " + text + "\\n") : undefined), language = process.env.SEMIO_LANGUAGE === "de" ? "de" : "en", timeoutMilliseconds } = {}) {
  const results = [];
  try {
    const loaded = runners ?? (await import("./🧮️conformance/🏃️runners/🟨️.mjs")).RUNNERS_V1;
    const own = fileURLToPath(new URL("./🧮️conformance/", import.meta.url)), codeFiles = ["🟨️.mjs", "🔣️.json", "🏃️runners/🟨️.mjs", "🧬️schema/🔣️.json"].map((name) => join(own, name).slice(libraryRoot.length + 1));
    for (const id of protocol.derived) {
      const result = await resolveStampV1({ id, declared: protocol.inferrers[id].stamp, root: libraryRoot, directory, runners: loaded, fleet, signal, codeFiles, ...(timeoutMilliseconds === undefined ? {} : { timeoutMilliseconds }) });
      if (result.stamp) effective.set(id, result.stamp); else effective.delete(id);
      say?.(conformanceTextV1("stamp." + result.status, language, { id, cases: Object.keys(result.record?.parts.cases ?? {}).length, milliseconds: Math.round(result.record?.milliseconds ?? 0), reason: result.reason ?? "" }));
      results.push(result);
    }
  } catch (error) {
    for (const id of protocol.derived) { effective.delete(id); results.push({ status: "unavailable", id, reason: String(error?.message ?? error) }); }
  }
  registerInferenceStampsV1(inferenceStampEntriesV1(protocol), inferenceFragmentsV1(protocol));
  return results;
}
'''
patch(os.path.join(INF, "🔖️stamps/🟨️.mjs"), [
    ('/** 🔬️ True when a workspace relative path lies below one of the fragments. */', CONFORMANCE_BLOCK.lstrip("\n") + '\n/** 🔬️ True when a workspace relative path lies below one of the fragments. */'),
])

patch(os.path.join(INF, "🔖️stamps/🟨️.d.mts"), [
    ('export interface InferenceProtocolV1 { readonly version: 1; readonly scopes: readonly InferenceScopeV1[]; readonly inferrers: Readonly<Record<string, InferrerV1>>; readonly none: readonly string[] }',
     'export interface InferenceProtocolV1 { readonly version: 1; readonly scopes: readonly InferenceScopeV1[]; readonly inferrers: Readonly<Record<string, InferrerV1>>; readonly derived: readonly string[]; readonly none: readonly string[] }\nexport interface ConformanceSettlementV1 { readonly status: "stored" | "fleet" | "computed" | "unavailable"; readonly id: string; readonly stamp?: string; readonly reason?: string }\nexport interface ConformanceFleetPortV1 { get?(id: string, key: string): Promise<unknown> | unknown; put?(id: string, key: string, record: unknown): Promise<unknown> | unknown }'),
    ('export function inferenceFragmentsV1(document: InferenceProtocolV1): string[];', 'export function inferenceFragmentsV1(document: InferenceProtocolV1, stamps?: ReadonlyMap<string, string>): string[];\nexport function inferrerActiveV1(id: string, stamps?: ReadonlyMap<string, string>, document?: InferenceProtocolV1): boolean;\nexport function bindConformanceFleetPortV1(port: ConformanceFleetPortV1 | undefined): void;\nexport function conformanceDirectoryV1(environment?: Readonly<Record<string, string | undefined>>, platform?: string, home?: string): string;\nexport function settleConformanceStampsV1(options?: { libraryRoot: string; directory?: string; runners?: Readonly<Record<string, (fixtures: never) => Promise<unknown>>>; fleet?: ConformanceFleetPortV1; signal?: AbortSignal; say?: (text: string) => void; language?: "en" | "de"; timeoutMilliseconds?: number }): Promise<ConformanceSettlementV1[]>;'),
    ('export function inferenceStampEntriesV1(document: InferenceProtocolV1): [string, string][];', 'export function inferenceStampEntriesV1(document: InferenceProtocolV1, stamps?: ReadonlyMap<string, string>): [string, string][];'),
    ('export function classifyImplementationRowsV1(rows: readonly ImplementationRowV1[], document?: InferenceProtocolV1): ImplementationRowV1[];', 'export function classifyImplementationRowsV1(rows: readonly ImplementationRowV1[], document?: InferenceProtocolV1, stamps?: ReadonlyMap<string, string>): ImplementationRowV1[];'),
])

jpatch(os.path.join(INF, "🔖️stamps/🧬️schema/🔣️.json"), schema)
jpatch(os.path.join(INF, "🗂️components/🔣️.json"), roster)

patch(os.path.join(INF, "🔁️bootstrap/🟨️.mjs"), [
    ('export let libraryBootstrap;\n', 'export let libraryBootstrap;\nlet stamps;\n'),
    ('  ({ LIBRARY_ROOT, LIBRARY_URL, RUNTIME_COMPONENT_MODULE,', '  stamps = registry.stamps;\n  ({ LIBRARY_ROOT, LIBRARY_URL, RUNTIME_COMPONENT_MODULE,'),
    ('    loaded.playgroundCompositionPathV1 = composition.playgroundCompositionPathV1;\n  });\n}', '    loaded.playgroundCompositionPathV1 = composition.playgroundCompositionPathV1;\n  }).then(() => stamps.settleConformanceStampsV1({ libraryRoot: LIBRARY_ROOT }).catch(() => undefined));\n}'),
])

patch(os.path.join(INF, "🧱️project-assembly/🟨️.mjs"), [
    ('let closureApiV1, closureHandleOfV1, registerClosureQueryV1,', 'let stampActiveV1, closureApiV1, closureHandleOfV1, registerClosureQueryV1,'),
    ('  ({ libraryBootstrap, loaded } = registry.bootstrap);\n', '  ({ libraryBootstrap, loaded } = registry.bootstrap);\n  ({ inferrerActiveV1: stampActiveV1 } = registry.stamps);\n'),
    ('() => { for (const file of PRECISION_FILES) readFileSync(join(LIBRARY_ROOT, file)); for(const file of managerSourceFilesV1())readFileSync(file); const rules',
     '() => { if (!stampActiveV1("input-precision")) for (const file of PRECISION_FILES) readFileSync(join(LIBRARY_ROOT, file)); if (!stampActiveV1("package-managers")) for(const file of managerSourceFilesV1())readFileSync(file); const rules'),
    ('/** 📏️ The input precision rules as every project that applies them records them:', '/** 📏️ The input precision rules as every project that applies them records them (the rules file always; the engine and the manager sources only while their conformance stamp is unavailable, since the stamp row stands for them otherwise):'),
])

patch(os.path.join(INF, "🧭️script-inputs/🟨️.mjs"), [
    ('''    if (!existsSync(config)) throw refuseBuildV1("invalid", "closure.provider-project-required", { designation: String(designation) });
    return providerProjectNameV1(JSON.parse(readPhysicalSource(config).toString("utf8")), basename(designation));
  });
}''', '''    if (!existsSync(config)) throw refuseBuildV1("invalid", "closure.provider-project-required", { designation: String(designation) });
    return providerProjectOfConfigV1(config, basename(designation));
  });
}

/** 🪪️ The name of the providers project read as two fields of its configuration (its name and the designation it declares), so an edit of any other line of the configuration moves no walk of any project. */
export function providerProjectOfConfigV1(config, designation) {
  const key = PROVIDER_CLOSURE_POLICY_V1.project.metadata, field = (pointer) => { try { return observedFieldV1(config, "json", pointer); } catch (error) { if (String(error?.message).startsWith("Field absent")) return undefined; throw error; } };
  return providerProjectNameV1({ name: field("/name"), metadata: { semio: { [key]: field("/metadata/semio/" + key) } } }, designation);
}'''),
])

patch(os.path.join(INF, "♻️revision/🟨️.mjs"), [
    ('"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/📇️format/🧬️schema/🔣️.json"];\n}', '"⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/📇️format/🧬️schema/🔣️.json", "' + CONF + '🟨️.mjs", "' + CONF + '🔣️.json", "' + CONF + '🧬️schema/🔣️.json", "' + CONF + '🏃️runners/🟨️.mjs"];\n}'),
])
print("applied" if APPLY else "checked: every anchor found exactly once")
