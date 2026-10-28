"""Builds the overlay of the setup-g batch: exact-anchor edits of the original library files plus the new files already in overlay/. usage: python3 build-overlay.py [--check]
Every anchor must match exactly once in the ORIGINAL file; the base copy of every edited file is stored in base/ with its sha256 (revert source)."""
import hashlib, json, os, sys
ROOT = os.environ.get("SEMIO_STAGE_ROOT", "/Users/ueli/Documents/semio").rstrip("/") + "/"
HERE = os.path.dirname(os.path.abspath(__file__)) + "/"
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
TS = L + "🏗️build/🧰️tool-setup/"
PF = L + "🏗️build/🩺️preflight/"
DR = ".devcontainer/Dockerfile"

def read(path): return open(ROOT + path, encoding="utf8").read()
def sha(text): return hashlib.sha256(text.encode("utf8")).hexdigest()

def edit_text(path, pairs):
    text = read(path)
    for old, new in pairs:
        n = text.count(old)
        if n != 1: raise SystemExit(f"ANCHOR {n}x in {path[-70:]}: {old[:80]!r}")
        text = text.replace(old, new)
    return text

def edit_json(path, mutate):
    data = json.loads(read(path)); mutate(data)
    return json.dumps(data, indent=2, ensure_ascii=False) + "\n"

edits = {}

# ---- tool-setup table: group, row, steps, messages
def table(data):
    i = data["groups"]["all"].index("rust"); data["groups"]["all"].insert(i + 1, "node-native-core")
    j = next(k for k, row in enumerate(data["tools"]) if row["id"] == "rust")
    data["tools"].insert(j + 1, {
        "id": "node-native-core",
        "summary": {"en": "Native core that gives Node the system calls the repository tools bind (built once from the pinned Rust toolchain).", "de": "Nativer Kern, der Node die Systemaufrufe gibt, die die Repository-Werkzeuge binden (einmal mit der gepinnten Rust-Toolchain gebaut)."},
        "pin": {"format": "regex", "file": "🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🧬️schema/🔣️.json", "pattern": "\"Protocol\"[\\s\\S]*?\"version\":\\s*(\\d+)"},
        "probe": {"mode": "hook", "arguments": [], "pattern": ""},
        "locate": [{"kind": "hook"}],
        "setup": {"kind": "script", "script": "🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🏗️addon/🛠️prepare/📜️script.ts", "prepare": ["prepare"], "verify": ["verify"]},
    })
    data["steps"] = [{
        "id": "compiler-helpers",
        "kind": "cargo-helpers",
        "summary": {"en": "The memory-admitting rustc scheduler and the unit-cache compiler wrapper (a native executable on Windows) that every Cargo build starts with.", "de": "Der speicherzulassende rustc-Scheduler und der Compiler-Wrapper des Unit-Caches (unter Windows eine native Programmdatei), mit denen jeder Cargo-Build beginnt."},
        "platforms": ["darwin", "linux", "win32"],
        "configuration": "prototype",
        "timeoutSeconds": 1200,
        "optional": True,
    }]
    t = data["messages"]["texts"]
    t["warm"] = {"en": "Preparing {step}: {summary}", "de": "Bereite {step} vor: {summary}"}
    t["warmed"] = {"en": "{step} is ready.", "de": "{step} ist bereit."}
    t["warmFailed"] = {"en": "{step} could not be prepared ({reason}); it is built on the first compile instead.", "de": "{step} konnte nicht vorbereitet werden ({reason}); es wird stattdessen beim ersten Kompilieren gebaut."}
edits[TS + "🔣️.json"] = edit_json(TS + "🔣️.json", table)

def schema(data):
    d = data["definitions"]
    data["required"].insert(data["required"].index("tools") + 1, "steps")
    props = data["properties"]; new = {}
    for k, v in props.items():
        new[k] = v
        if k == "tools": new["steps"] = {"type": "array", "items": {"$ref": "#/definitions/Step"}}
    data["properties"] = new
    d["Step"] = {
        "description": "A preparation that is not a pinned tool: it has no version to compare, runs only when the whole table is set up and, when optional, never fails the setup.",
        "type": "object", "additionalProperties": False,
        "required": ["id", "kind", "summary", "platforms", "configuration", "timeoutSeconds", "optional"],
        "properties": {
            "id": {"$ref": "#/definitions/Id"},
            "kind": {"enum": ["cargo-helpers"]},
            "summary": {"$ref": "#/definitions/Text"},
            "platforms": {"type": "array", "minItems": 1, "uniqueItems": True, "items": {"enum": ["darwin", "linux", "win32"]}},
            "configuration": {"enum": ["prototype", "release"]},
            "timeoutSeconds": {"type": "integer", "minimum": 60, "maximum": 7200},
            "optional": {"type": "boolean"},
        },
    }
    texts = d["Messages"]["properties"]["texts"]
    texts["required"] += ["warm", "warmed", "warmFailed"]
    for k in ("warm", "warmed", "warmFailed"): texts["properties"][k] = {"$ref": "#/definitions/Text"}
edits[TS + "🧬️schema/🔣️.json"] = edit_json(TS + "🧬️schema/🔣️.json", schema)

# ---- tool-setup engine
edits[TS + "🟦️.ts"] = edit_text(TS + "🟦️.ts", [
 ('/** 🏓️ One row of the table. */',
  '/** 🔥️ One preparation of the table that is not a pinned tool. */\nexport type ToolStepV1 = (typeof document.steps)[number];\n/** 🏓️ One row of the table. */'),
 ('/** 🏠️ The default home directory of a context. */',
  '''/** 🪜️ The steps of the table that apply to a platform, in table order. */
export function toolStepsV1(platform: NodeJS.Platform): readonly ToolStepV1[] {
  return document.steps.filter(step => (step.platforms as readonly string[]).includes(platform));
}

/** 🔥️ What preparing one step needs from the machine: the preparer of its kind, which may take as long as the step allows. */
export type ToolStepRunnerV1 = (step: ToolStepV1, signal: AbortSignal) => Promise<void>;

/** 🌡️ Prepares the steps one after the other and says so in the resolved language; a cancelled signal stops between steps and inside a step, an optional step that fails is reported and the setup goes on, a required one fails the setup. */
export async function warmToolStepsV1(steps: readonly ToolStepV1[], run: ToolStepRunnerV1, signal: AbortSignal, log: (line: string) => void, environment: Readonly<Record<string, string | undefined>> = process.env): Promise<readonly Readonly<{id: string; status: "ready" | "failed"; message?: string}>[]> {
  const results: Readonly<{id: string; status: "ready" | "failed"; message?: string}>[] = [];
  for (const step of steps) {
    signal.throwIfAborted();
    log(toolSetupMessageV1("warm", {step: step.id, summary: toolStepSummaryV1(step, environment)}, environment));
    try {
      await run(step, AbortSignal.any([signal, AbortSignal.timeout(step.timeoutSeconds * 1000)]));
      log(toolSetupMessageV1("warmed", {step: step.id}, environment));
      results.push(Object.freeze({id: step.id, status: "ready" as const}));
    } catch (error) {
      signal.throwIfAborted();
      if (!step.optional) throw error;
      const message = toolSetupMessageV1("warmFailed", {step: step.id, reason: String((error as Error)?.message ?? error).split("\\n")[0]!.slice(0, 300)}, environment);
      log(message);
      results.push(Object.freeze({id: step.id, status: "failed" as const, message}));
    }
  }
  return Object.freeze(results);
}

/** 🏷️ The summary of a step in the resolved language, or in every language when none resolves. */
export function toolStepSummaryV1(step: ToolStepV1, environment: Readonly<Record<string, string | undefined>> = process.env): string {
  try {
    return step.summary[resolveLanguageV1(document.messages.languages, [], environment, () => []).language as "en" | "de"];
  } catch {
    return document.messages.languages.map(language => step.summary[language as "en" | "de"]).join(" / ");
  }
}

/** 🏠️ The default home directory of a context. */'''),
])

# ---- tool-setup command wiring
edits[TS + "⌨️command/🟦️.ts"] = edit_text(TS + "⌨️command/🟦️.ts", [
 ('import {locateToolV1, renderToolReportV1, resolveSiblingPackageV1, setupToolsV1, runtimeWarningV1, runningRuntimeLineV1, toolRowV1, toolRowsV1, toolSetupMessageV1, verifyToolsV1, type ToolContextV1, type ToolProvisionerV1, type ToolReportV1, type ToolRowV1} from "../🟦️.ts";',
  'import {locateToolV1, renderToolReportV1, resolveSiblingPackageV1, setupToolsV1, runtimeWarningV1, runningRuntimeLineV1, toolRowV1, toolRowsV1, toolSetupMessageV1, toolStepsV1, verifyToolsV1, warmToolStepsV1, type ToolContextV1, type ToolProvisionerV1, type ToolReportV1, type ToolRowV1, type ToolStepRunnerV1} from "../🟦️.ts";'),
 ('  return setupToolsV1(rows, context, provision, control.signal, line => console.log(line));\n}',
  '''  const reports = await setupToolsV1(rows, context, provision, control.signal, line => console.log(line));
  if (names.length === 0 && reports.every(report => report.status === "ok" || report.status === "deferred")) {
    const prepare: ToolStepRunnerV1 = async (step, signal) => {
      const {prepareRepositoryCargoHelpersV1} = await import("../../../🏃️process/🦀️cargo/🧰️native-helpers/🟦️.ts");
      signal.throwIfAborted();
      await prepareRepositoryCargoHelpersV1(invocation, entry("cargo:helpers"), root, step.configuration, process.env, step.timeoutSeconds * 1000);
    };
    await warmToolStepsV1(toolStepsV1(process.platform), prepare, control.signal, line => console.log(line), context.environment);
  }
  return reports;
}'''),
])

# ---- tool-setup tests: fixture + new tests
def fixture(data):
    data["files"]["🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🧬️schema/🔣️.json"] = json.dumps({"definitions": {"Protocol": {"const": {"version": 7}}}}, ensure_ascii=False)
    data["expected"]["node-native-core"] = "7"
    data["expected"] = dict(sorted(data["expected"].items()))
edits[TS + "🧪️tests/🧫️fixtures/🔣️.json"] = edit_json(TS + "🧪️tests/🧫️fixtures/🔣️.json", fixture)


# ---- preflight: policy, schema, wiring, remedies, tests
def jq(text): return json.dumps(text, ensure_ascii=False)
BOOT_SH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh"
BOOT_PS = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🔵️.ps1"
edits[PF + "🔣️policy.json"] = edit_text(PF + "🔣️policy.json", [
 ('"tools": ["cc", "pkg-config"], "exchange": true}', '"tools": ["cc", "pkg-config", "rust"], "apt": {"cc": ["build-essential"], "pkg-config": ["pkg-config"], "rust": ["curl", "ca-certificates"]}, "exchange": true}'),
 ('"tools": ["cc", "hdiutil"], "exchange": false}', '"tools": ["cc", "hdiutil", "developer-tools", "rust"], "exchange": false}'),
 ('"tools": [], "exchange": true}', '"tools": ["msvc-linker", "rust"], "exchange": true}'),
])
edits[PF + "🧬️schema/🔣️.json"] = edit_text(PF + "🧬️schema/🔣️.json", [
 ('        "exchange": {"type": "boolean"}\n      }\n    },\n    "Policy"',
  '        "exchange": {"type": "boolean"},\n        "apt": {"type": "object", "additionalProperties": {"type": "array", "minItems": 1, "uniqueItems": true, "items": {"type": "string", "pattern": "^[a-z0-9][a-z0-9.+-]*$"}}}\n      }\n    },\n    "Policy"'),
])
edits[PF + "🟦️.ts"] = edit_text(PF + "🟦️.ts", [
 ('import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";',
  'import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";\nimport {defaultNativeToolchainPortsV1,nativeToolchainHostV1,probeNativeToolV1} from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🧲️native/🧰️toolchain/🟦️.ts";'),
 ('export type BuildPreflightPlatformV1 = Readonly<{architectures:readonly("x64"|"arm64")[];minimumAvailableBytes:number;tools:readonly string[];exchange:boolean}>;',
  'export type BuildPreflightPlatformV1 = Readonly<{architectures:readonly("x64"|"arm64")[];minimumAvailableBytes:number;tools:readonly string[];exchange:boolean;apt?:Readonly<Record<string,readonly string[]>>}>;'),
 ('const tools:string[]=[],extensions=', 'const toolchainHost=nativeToolchainHostV1(),toolchainPorts=defaultNativeToolchainPortsV1(),tools:string[]=[],extensions='),
 ('for(const tool of required.tools){search:for(', 'for(const tool of required.tools){const probe=probeNativeToolV1(tool,toolchainHost,toolchainPorts);if(probe!==undefined){if(probe.found)tools.push(tool);checkScriptInvocation(invocation);continue;}search:for('),
])
RUST_TEXT = lambda boot, run: {
  "en": "No Rust compiler or rustup was found on the PATH or in the cargo directory (CARGO_HOME/bin, else ~/.cargo/bin): run the repository bootstrap (" + run + "), which installs rustup and the pinned toolchain, then run repo tools setup all. The native core that lets Node call the system is built with it.",
  "de": "Weder ein Rust-Compiler noch rustup wurde im PATH oder im Cargo-Verzeichnis (CARGO_HOME/bin, sonst ~/.cargo/bin) gefunden: führe das Bootstrap des Repositorys aus (" + run + "), es installiert rustup und die gepinnte Toolchain, und danach repo tools setup all. Damit wird auch der native Kern gebaut, mit dem Node das System aufruft.",
}
rust = {"linux": RUST_TEXT(BOOT_SH, "bash " + BOOT_SH + " setup"), "darwin": RUST_TEXT(BOOT_SH, "bash " + BOOT_SH + " setup"), "win32": RUST_TEXT(BOOT_PS, "powershell -NoProfile -ExecutionPolicy Bypass -File " + BOOT_PS + " setup")}
developer = {"darwin": {
  "en": "The command line developer tools are not installed: /usr/bin/cc is only a stub that opens an installer. Run xcode-select --install, accept the license (sudo xcodebuild -license accept for a full Xcode) or point DEVELOPER_DIR at an installed Xcode, then run repo preflight again.",
  "de": "Die Kommandozeilen-Entwicklerwerkzeuge sind nicht installiert: /usr/bin/cc ist nur ein Platzhalter, der ein Installationsprogramm öffnet. Führe xcode-select --install aus, akzeptiere die Lizenz (sudo xcodebuild -license accept bei einem vollständigen Xcode) oder setze DEVELOPER_DIR auf ein installiertes Xcode, und führe repo preflight danach erneut aus."}}
msvc = {"win32": {
  "en": "The Microsoft C++ build tools (the MSVC linker Rust needs) were not found. Install the Desktop development with C++ workload: run the Windows bootstrap (" + "powershell -NoProfile -ExecutionPolicy Bypass -File " + BOOT_PS + " setup), which installs Microsoft.VisualStudio.BuildTools through winget, or add the workload in the Visual Studio Installer, then open a new terminal.",
  "de": "Die Microsoft-C++-Buildtools (der MSVC-Linker, den Rust braucht) wurden nicht gefunden. Installiere die Workload Desktopentwicklung mit C++: führe das Windows-Bootstrap aus (" + "powershell -NoProfile -ExecutionPolicy Bypass -File " + BOOT_PS + " setup), es installiert Microsoft.VisualStudio.BuildTools über winget, oder füge die Workload im Visual Studio Installer hinzu, und öffne danach ein neues Terminal."}}
def block(name, rows):
    inner = ",\n".join('      %s: {\n        "en": %s,\n        "de": %s\n      }' % (jq(k), jq(v["en"]), jq(v["de"])) for k, v in rows.items())
    return '    %s: {\n%s\n    }' % (jq(name), inner)
edits[PF + "💊️remedies/📖️texts/🔣️.json"] = edit_text(PF + "💊️remedies/📖️texts/🔣️.json", [
 ('    "hdiutil": {\n      "darwin": {', block("developer-tools", developer) + ",\n" + block("msvc-linker", msvc) + ",\n" + block("rust", rust) + ',\n    "hdiutil": {\n      "darwin": {'),
])
edits[PF + "💊️remedies/🧪️tests/🟦️.ts"] = edit_text(PF + "💊️remedies/🧪️tests/🟦️.ts", [
 ('const platforms=["linux","darwin","win32"] as const,exchanges=["supported","unsupported","unprobed"] as const,cases=["yes","no","unprobed"] as const;',
  'const platforms=["linux","darwin","win32"] as const,exchanges=["supported","unsupported","unprobed"] as const,cases=["yes","no","unprobed"] as const,toolPool=["cc","pkg-config","hdiutil","rust","developer-tools","msvc-linker"];'),
 ('for(const missing of [[],["cc"],["cc","pkg-config","hdiutil"]])', 'for(const missing of [[],["cc"],["cc","pkg-config","hdiutil"],["rust"],["developer-tools","rust"],["msvc-linker","rust"],toolPool])'),
 ('tools:["cc","pkg-config","hdiutil"].filter(tool=>!missing.includes(tool))},requiredExchange:policy.platforms[platform].exchange,requiredTools:["cc","pkg-config","hdiutil"],container}',
  'tools:toolPool.filter(tool=>!missing.includes(tool))},requiredExchange:policy.platforms[platform].exchange,requiredTools:toolPool,container}'),
])
def pfixture(data):
    for row in data["windows"]: row["facts"]["tools"] = ["msvc-linker", "rust"]
    for row in data["neutral"]: row["facts"]["tools"] = ["cc", "hdiutil", "developer-tools", "rust"]
edits[PF + "🧫️fixtures/🔣️.json"] = edit_json(PF + "🧫️fixtures/🔣️.json", pfixture)
edits[PF + "🧪️tests/🧮️matrix/🔣️.json"] = edit_text(PF + "🧪️tests/🧮️matrix/🔣️.json", [
 ('   "cc",\n   "pkg-config",\n   "hdiutil",\n   "a",\n   "extra"\n  ],', '   "cc",\n   "pkg-config",\n   "hdiutil",\n   "rust",\n   "msvc-linker",\n   "a"\n  ],'),
])

if "--check" in sys.argv:
    print("anchors ok:", len(edits)); sys.exit(0)
os.makedirs(HERE + "overlay", exist_ok=True); os.makedirs(HERE + "base", exist_ok=True)
index = {}
for path, text in edits.items():
    out = HERE + "overlay/" + path; os.makedirs(os.path.dirname(out), exist_ok=True)
    open(out, "w", encoding="utf8").write(text)
    base = HERE + "base/" + path; os.makedirs(os.path.dirname(base), exist_ok=True)
    open(base, "w", encoding="utf8").write(read(path))
    index[path] = {"base": sha(read(path)), "new": sha(text)}
json.dump(index, open(HERE + "index-edited.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("edited files:", len(index))
