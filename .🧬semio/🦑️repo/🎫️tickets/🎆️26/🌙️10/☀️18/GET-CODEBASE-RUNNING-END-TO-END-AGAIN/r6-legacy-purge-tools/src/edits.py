import os, re, sys

REQUIRE_SETS = []

OBJ_GEN = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/🟦️.ts"
OBJ_SCRIPT = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📜️script.ts"


def rename_pattern_shell(text):
    for old, new in [
        ("LEGACY_ENGINE_BIN", "PATTERN_SHELL_ENGINE_BIN"),
        ("LEGACY_RECIPE", "PATTERN_SHELL_RECIPE"),
        ("LEGACY_DIRECTORY", "PATTERN_SHELL_DIRECTORY"),
        ("LEGACY_FIXTURE_FILE", "PATTERN_SHELL_FIXTURE_FILE"),
        ("ensureLegacyBuilt", "ensurePatternShellEngineBuilt"),
        ("generateLegacy", "generatePatternShell"),
        ("manifestForLegacy", "manifestForPatternShell"),
        ("cargo build (legacy engine)", "cargo build (pattern-shell engine)"),
        ("legacy generator binary", "pattern-shell generator binary"),
    ]:
        assert old in text, old
        text = text.replace(old, new)
    return text


EDITS = {
    OBJ_GEN: [
        rename_pattern_shell,
        (
            "rebuilds ONLY the\n *  legacy `pattern-shell` recipe — the exact behavior its own committed `testEvidence` entry's\n *  `generator.command` (`generate`, no flags) has always invoked, unchanged.",
            "rebuilds ONLY the\n *  `pattern-shell` recipe — the one invocation its committed `testEvidence` entry's\n *  `generator.command` (`generate`, no flags) records.",
        ),
    ],
    OBJ_SCRIPT: [
        (
            "`pattern-shell` — the ORIGINAL single-fixture recipe (unchanged since it was first committed),\n//     built by `../🦀️engine` and admitted through `tobj` 4. Its own `testEvidence` entry's\n//     `generator.command` still reads plain `generate` with no `--only`, so that exact invocation\n//     must keep producing exactly this one file — this script preserves that path byte-for-byte.",
            "`pattern-shell` — the single-fixture recipe, built by the reference engine crate in\n//     `🧪️tests/🧰️support` and admitted through `tobj` 4. Its `testEvidence` entry's\n//     `generator.command` reads plain `generate` with no `--only`, so that invocation produces\n//     exactly this one file.",
        ),
    ],
}


HOST = "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🧊️mesh/🟦️.ts"
HOST_TEST = "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🧊️mesh/🧪️tests/🌐️off-browser-host/🟦️.ts"
ART = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/"
MESH_GEN = ART + "🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏭️generator/📜️script.ts"
MESH_PROBES = ART + "🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🔬️probes/📜️script.ts"
GLTF_BASE = ART + "🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🏭️generator/🏗️base-document/🟦️.ts"
GLTF_PROBES = ART + "🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔬️probes/📜️script.ts"
FEM2D = "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/📜️script.ts"
FEM3D = "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/📜️script.ts"
HOST_IMPORT = 'import { installOffBrowserGlobals } from "%s🔮️oracles/🧊️mesh/🟦️.ts";\n'


def between(start, end, replacement):
    def edit(text):
        a = text.index(start)
        b = text.index(end, a) + len(end)
        return text[:a] + replacement + text[b:]
    return edit


def add_import_after(anchor, ups):
    def edit(text):
        assert text.count(anchor) == 1, anchor
        return text.replace(anchor, anchor + HOST_IMPORT % ("../" * ups))
    return edit


FEM_REMOVE = (
    "// 🩹️ Defensive: three's PLY/GLTF exporters need these off-browser; STL/OBJ do not, in principle, but\n// the mesh pilot's spike found them cheap insurance and the cost of carrying them is one shim block.\n(globalThis as { requestAnimationFrame?: (cb: () => void) => number }).requestAnimationFrame ??= (cb) => { cb(); return 0; };\n",
    "",
)
EDITS[FEM2D] = [FEM_REMOVE]
EDITS[FEM3D] = [FEM_REMOVE]
EDITS[MESH_GEN] = [
    between("// 🩹️ three's exporters assume a browser", "ProgressEvent ??= ShimProgressEvent;\n", "installOffBrowserGlobals();\n"),
    add_import_after('import type { Family, Recipe } from "./📇️recipe-contract/🟦️.ts";\n', 7),
]
EDITS[MESH_PROBES] = [
    between("/** 🩹️ three's loaders and exporters reach for browser globals.", "FileReader ??= ShimFileReader as unknown as typeof FileReader;\n", "installOffBrowserGlobals();\n"),
    add_import_after('import Module from "manifold-3d";\n', 7),
]
EDITS[GLTF_BASE] = [
    ("/** 🏗️ glTF fixture generator: the FileReader shim, the untyped document model", "/** 🏗️ glTF fixture generator: the off-browser host, the untyped document model"),
    between("//#region 🩹️Shims\n", "//#endregion 🩹️Shims\n", "installOffBrowserGlobals();\n\n"),
    add_import_after('import { GLTFExporter } from "three/examples/jsm/exporters/GLTFExporter.js";\n', 8),
]
EDITS[GLTF_PROBES] = [
    between("//#region 🩹️Shims\n", "//#endregion 🩹️Shims\n", "installOffBrowserGlobals();\n"),
    add_import_after('import { NodeIO } from "@gltf-transform/core";\n', 7),
]
NEW = {HOST: "off-browser-host.ts", HOST_TEST: "off-browser-host.test.ts"}


MOD = "🧰️framework/🛍️products/🦑️repo/🔨️modules/"
ARTIFACT_REF = MOD + "📐️model/📦️packages/🐹️go/🧮️artifact-ref/🐹️.go"
ARTIFACT_URI_PARSE = MOD + "💻️client/⌨️cli/🔗️artifact-uri-parse/🐹️.go"
MODEL_UNIT_TEST = MOD + "📐️model/🧪️tests/🔬️unit/🐹️.go"
CLI_URI_TEST = MOD + "💻️client/⌨️cli/🧪️tests/🆔️artifact-id-and-uri/🐹️.go"

URI_EXTRACTORS = """// 🟨️extractFileAndSectionsFromUri extracts the file path and section names from a URI.
// 🆕️Reads repo://section/{{id}} and repo://file/{{id}}, whose ID encodes file and sections as emojis; the `repo://` prefix may be stripped.
func extractFileAndSectionsFromUri(uri string) (filePath string, sectionSlugs []string) {
	if !strings.HasPrefix(uri, "repo://") {
		uri = "repo://" + uri
	}
	if !strings.HasPrefix(uri, "repo://section/") && !strings.HasPrefix(uri, "repo://file/") {
		return "", nil
	}
	id := UriToId(uri)
	if id == "" {
		return "", nil
	}
	return IdToPath(id), IdToSectionPath(id)
}

// 🟩️extractFileAndDefinitionFromUri extracts the file path and definition name from a URI.
// ▶️Reads repo://definition/{{id}}, whose ID encodes file, sections and definition as emojis; the `repo://` prefix may be stripped.
func extractFileAndDefinitionFromUri(uri string) (filePath string, defName string) {
	if !strings.HasPrefix(uri, "repo://") {
		uri = "repo://" + uri
	}
	if !strings.HasPrefix(uri, "repo://definition/") {
		return "", ""
	}
	id := UriToId(uri)
	if id == "" {
		return "", ""
	}
	return IdToPath(id), IdToDefinitionName(id)
}
"""

GO_URI_TEST = """
func TestArtifactUriExtractionReadsOnlyTheEntityKindFormat(t *testing.T) {
	fileID := BuildFileID("compose/js/src/design.tsx", nil)
	sectionID := BuildSectionID(fileID, []string{"state managment", "store"})
	definitionID := BuildDefinitionID(fileID, nil, "myclass", DefinitionKindImplementation)
	for _, uri := range []string{"repo://section/" + sectionID, "section/" + sectionID} {
		if path, parts := extractFileAndSectionsFromUri(uri); path != IdToPath(sectionID) || len(parts) != 2 {
			t.Errorf("extractFileAndSectionsFromUri(%q) = %q %v", uri, path, parts)
		}
	}
	for _, uri := range []string{"repo://definition/" + definitionID, "definition/" + definitionID} {
		if path, name := extractFileAndDefinitionFromUri(uri); path != IdToPath(definitionID) || name != "myclass" {
			t.Errorf("extractFileAndDefinitionFromUri(%q) = %q %q", uri, path, name)
		}
	}
	for _, retired := range []string{"repo://p/f/docs/guide.md/s/intro", "f/docs/guide.md/s/intro", "repo://p/f/docs/guide.md/d/x/y"} {
		if path, parts := extractFileAndSectionsFromUri(retired); path != "" || parts != nil {
			t.Errorf("extractFileAndSectionsFromUri(%q) = %q %v, want nothing", retired, path, parts)
		}
		if path, name := extractFileAndDefinitionFromUri(retired); path != "" || name != "" {
			t.Errorf("extractFileAndDefinitionFromUri(%q) = %q %q, want nothing", retired, path, name)
		}
	}
}
"""

for _path in (ARTIFACT_REF, ARTIFACT_URI_PARSE):
    EDITS[_path] = [between("// 🟨️extractFileAndSectionsFromUri", "\treturn filePath, defName\n}\n", URI_EXTRACTORS)]


def append(text_to_add):
    return lambda text: text.rstrip("\n") + "\n" + text_to_add


EDITS[MODEL_UNIT_TEST] = [append(GO_URI_TEST)]
EDITS[CLI_URI_TEST] = [append(GO_URI_TEST.replace("BuildFileID", "buildFileID").replace("BuildSectionID", "buildSectionID").replace("BuildDefinitionID", "buildDefinitionID"))]


LIB = MOD + "📚️library/"
WGPU_DEV = LIB + "🎮️playground/🧊️wgpu-dev/🟦️.ts"
DEV_TOOLING_TEST = LIB + "🧪️tests/🔬️workspace-contract/🛠️dev-tooling/🟦️.ts"
WGPU_DEV_NEW = """/** 🧊️ Play URL of the wgpu dev server on `port` for a plugin filter. */
export function wgpuDevPlayUrl(host: string, port: number, plugin: string): string {
  const probeHost = host === "0.0.0.0" ? "127.0.0.1" : host;
  return `http://${probeHost}:${port}/?plugin=${encodeURIComponent(plugin)}`;
}
"""
EDITS[WGPU_DEV] = [lambda text: WGPU_DEV_NEW]
EDITS[DEV_TOOLING_TEST] = [
    (
        '  test("builds root and legacy play urls", () => {\n    expect(wgpuDevPlayUrl("127.0.0.1", 6178, "lowpoly")).toBe("http://127.0.0.1:6178/?plugin=lowpoly");\n    expect(wgpuDevPlayUrl("127.0.0.1", 6178, "lowpoly", "/renderer-modules/wgpu/")).toBe("http://127.0.0.1:6178/renderer-modules/wgpu/?plugin=lowpoly");\n  });',
        '  test("builds the root play url and maps a wildcard host to loopback", () => {\n    expect(wgpuDevPlayUrl("127.0.0.1", 6178, "lowpoly")).toBe("http://127.0.0.1:6178/?plugin=lowpoly");\n    expect(wgpuDevPlayUrl("0.0.0.0", 6178, "low poly")).toBe("http://127.0.0.1:6178/?plugin=low%20poly");\n  });',
    ),
]


LAWS = LIB + "⚖️laws/🧩️plugin-consistency/"
FOUNDATION = LAWS + "🏛️foundation/🟦️.ts"
SHAPE_GUARD = "    if (crate.shape !== \"taxonomy\") continue;\n"
EDITS[FOUNDATION] = [
    (' * (`discoverPackages`/`discoverOwners` over `🔣️taxonomy.json`) rather than by a hardcoded area path.\n * PRIMARY: every `📦️packages/🦀️rust[/🎯️targets/<target>]/Cargo.toml` carrying a semio role marker —\n * plugins AND, for the first time (mechanism step M4), framework/hub/s-module/tool packages, which the\n * previous Shape-V1-only matcher could never see. SECONDARY (burn-down, deleted at the finalization\n * flip): residual `⚡️implementations/🦀️rust` sandwiches *inside* an already-discovered owner, so a\n * half-migrated owner keeps being checked; an owner that has not adopted `📦️packages` at all is\n * pre-contract and stays out of scope until it does.\n', ' * (`discoverPackages` over `🔣️taxonomy.json`) rather than by a hardcoded area path: every\n * `📦️packages/🦀️rust[/🎯️targets/<target>]/Cargo.toml` carrying a semio role marker — plugins and\n * framework/hub/s-module/tool packages alike; an owner that has not adopted `📦️packages` stays out of scope.\n'),
    ('import { discoverOwners, discoverPackages } from', 'import { discoverPackages } from'),
    (
        " * stable across the legacy per-module-crate layout AND the Shape V2 one-package-per-plugin layout, so\n * allowlists keyed off it survive the crate move untouched (see the master ticket's discovery contract).\n",
        " * stable across crate moves, so allowlists keyed off it survive them untouched (see the master ticket's discovery contract).\n",
    ),
    (
        " * 🔑Canonical `<pluginId>/<component>` key for a *source file path* under `✏️s/🔌️plugins/` — collapses\n * both the legacy per-module-crate layout (`…/🔨️modules/<module>/⚡️implementations/🦀️rust/📦️lib.rs`,\n * `…/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust/📦️lib.rs`) and the future taxonomy layout\n * (`…/🗿️artifacts/<artifact>/<module>/🦀️.rs`) onto the same string, so a completeness\n * allowlist written today still matches the same logical component after a plugin migrates. Widens to\n",
        " * 🔑Canonical `<pluginId>/<component>` key for a *source file path* under `✏️s/🔌️plugins/` — maps the taxonomy\n * layout (`…/🗿️artifacts/<artifact>/<module>/🦀️.rs`) onto a string that stays stable when a component moves. Widens to\n",
    ),
    between(
        "\n  // Legacy shape: <owner...>/🔨️modules/<module>/⚡️implementations/<lang>/<file>",
        "    if (ownerId) return `${ownerId}/${policyCanonicalComponent(moduleSeg)}${suffix}`;\n  }\n",
        "",
    ),
    ("  // Future taxonomy shape: <owner>/🗿️artifacts/<artifact>/<module>/🦀️.rs (+ sibling 🦀️<topic>.rs).\n", ""),
    (
        " * `libRelPath` its crate-root source file as the manifest itself declares it (`[lib]`/`[[bin]]`\n * `path`) — never assumed, because the two shapes disagree: Shape V2 keeps the entry beside the\n * manifest (`📦️packages/🦀️rust/🦀️.rs`) while the Shape V1 leftovers still point two levels up\n * (`path = \"../../🦀️.rs\"`). `role`/`ownerRel` come straight from the shared discovery contract;\n",
        " * `libRelPath` its crate-root source file as the manifest itself declares it (`[lib]`/`[[bin]]`\n * `path`) — never assumed. `role`/`ownerRel` come straight from the shared discovery contract;\n",
    ),
    ('  shape: "legacy" | "taxonomy";\n', ""),
    ('      shape: "taxonomy",\n', ""),
    (
        "  const forbiddenSegments = new Set<string>(taxonomy.forbiddenPathSegments);\n  const legacyEntryFilename = taxonomyEcosystemEntryFilenames(taxonomy, \"🦀️rust\")[0];\n  if (!legacyEntryFilename) throw new Error(\"[taxonomy] Rust ecosystem declares no configurable entry contract.\");\n",
        "",
    ),
    between("  for (const owner of discoverOwners(repoRoot, taxonomy)) {\n", "    walkLegacy(owner.ownerRel);\n  }\n\n", ""),
]
for _name, _count in [("✍️handcrafted-mutation/🟦️.ts", 1), ("🗺️taxonomy-shape/📚️examples/🟦️.ts", 2), ("🗺️taxonomy-shape/🟦️.ts", 3), ("🗺️taxonomy-shape/🧱️lib-and-barrel-shape/🟦️.ts", 1)]:
    EDITS[LAWS + _name] = [(SHAPE_GUARD, "", _count)]
EDITS[LAWS + "✍️handcrafted-mutation/🟦️.ts"].append(("crates.filter((crate) => crate.shape === \"taxonomy\").map(", "crates.map("))
EDITS[LAWS + "🗺️taxonomy-shape/🏷️naming/🟦️.ts"] = [("    if (crate.shape !== \"taxonomy\" || crate.role !== \"plugin\") continue;\n", "    if (crate.role !== \"plugin\") continue;\n")]
EDITS[LIB + "🧪️tests/🔬️workspace-contract/🧩️policy-laws/🟦️.ts"] = [('role: "", shape: "taxonomy", ownerRel', 'role: "", ownerRel', 2)]


COMMIT = LIB + "🎛️commands/📝️commit/"
METRICS = COMMIT + "📊️metrics/"
WC = LIB + "🧪️tests/🔬️workspace-contract/"
RUNNER_FALLBACK = 'metricsRunner && "countRepoUlocByLanguage" in metricsRunner ? metricsRunner : metricsRunnerFromUloc(metricsRunner as UlocRunner | undefined)'
EDITS[METRICS + "🏷️types/🟦️.ts"] = [
    ("/** 📊️Repo-wide unified LOC per language (tracked git files; JSON uses key count). */\nexport type UlocRunner = {\n  countRepoByLanguage(root: string): UlocByLanguage;\n};\n\n/** 📊️Repo-wide uloc and byte size per language for commit metrics. */", "/** 📊️Repo-wide uloc (tracked git files; JSON uses key count) and byte size per language for commit metrics. */"),
]
EDITS[METRICS + "💾️scan/🟦️.ts"] = [
    ('import type { MetricsRunner, UlocByLanguage, UlocRunner } from', 'import type { MetricsRunner, UlocByLanguage } from'),
    ("/** 📊️Default uloc runner (tracked-file unified scan). */\nexport function createDefaultUlocRunner(): UlocRunner {\n  return { countRepoByLanguage: scanRepoUnifiedLoc };\n}\n\n", ""),
    ("\n/** 📊️Adapts legacy uloc-only runners; size bloc still comes from repo scan. */\nexport function metricsRunnerFromUloc(ulocRunner?: UlocRunner): MetricsRunner {\n  const uloc = ulocRunner ?? createDefaultUlocRunner();\n  return {\n    countRepoUlocByLanguage: uloc.countRepoByLanguage,\n    countRepoSizeByLanguage: scanRepoSizeByLanguage,\n  };\n}\n", ""),
]
EDITS[METRICS + "🧮️summary/🟦️.ts"] = [
    ("MicroCommitLangMetrics, UlocByLanguage, UlocRunner } from", "MicroCommitLangMetrics, UlocByLanguage } from"),
    ("import { createDefaultMetricsRunner, createDefaultUlocRunner } from", "import { createDefaultMetricsRunner } from"),
    ("import { COMMIT_METRIC_HEADER, COMMIT_METRIC_TOTAL_EMOJI, formatMetricBody", "import { COMMIT_METRIC_TOTAL_EMOJI, formatMetricBody"),
    ("export function buildMicroCommitMetrics(root: string, ulocRunner: UlocRunner = createDefaultUlocRunner()): MicroCommitLangMetrics[] {", "export function buildMicroCommitMetrics(root: string, runner: MetricsRunner = createDefaultMetricsRunner()): MicroCommitLangMetrics[] {"),
    ("pathPrefixes?: string[], ulocRunner: UlocRunner = createDefaultUlocRunner()): MicroCommitLangMetrics[] {", "pathPrefixes?: string[], runner: MetricsRunner = createDefaultMetricsRunner()): MicroCommitLangMetrics[] {"),
    ("const codeByLang = ulocRunner.countRepoByLanguage(repoRoot);", "const codeByLang = runner.countRepoUlocByLanguage(repoRoot);", 2),
    ("/** 📊️Micro-commit metrics block header (legacy alias). */\nexport const MICRO_COMMIT_ULOC_HEADER = COMMIT_METRIC_HEADER;\n\n/** 🔢️Emoji for the aggregate total row (legacy alias). */\nexport const MICRO_COMMIT_ULOC_TOTAL_EMOJI = COMMIT_METRIC_TOTAL_EMOJI;\n\n", ""),
    ("/** 📊️Renders the unified LOC block (legacy: uloc-only footer). */", "/** 📊️Renders the unified LOC block for uloc rows alone. */"),
]
for _file, _cls in [("📄️prepared-message/🟦️.ts", 1), ("📈️attribution/🟦️.ts", 1), ("🔀️run-commit/🟦️.ts", 1)]:
    EDITS[COMMIT + _file] = [
        ('import type { MetricsRunner, UlocRunner } from', 'import type { MetricsRunner } from'),
        ("metricsRunner?: MetricsRunner | UlocRunner", "metricsRunner?: MetricsRunner"),
        ("const runner = " + RUNNER_FALLBACK + ";", "const runner = metricsRunner ?? createDefaultMetricsRunner();"),
        ('import { metricsRunnerFromUloc } from "../📊️metrics/💾️scan/🟦️.ts";', 'import { createDefaultMetricsRunner } from "../📊️metrics/💾️scan/🟦️.ts";'),
    ]
RUNNER_LITERAL = lambda body: "{ countRepoUlocByLanguage: () => (%s), countRepoSizeByLanguage: () => ({}) }" % body
EDITS[WC + "📝️commit-and-budgets/🟦️.ts"] = [("{ countRepoByLanguage: () => ({ TypeScript: 1000 }) }", RUNNER_LITERAL("{ TypeScript: 1000 }"))]
EDITS[WC + "🔬️micro-commit/🔹️buildmicrocommitmetrics-merges-uloc-git/🟦️.ts"] = [("{ countRepoByLanguage: () => ({ Rust: 100, TypeScript: 50, JSON: 20 }) }", RUNNER_LITERAL("{ Rust: 100, TypeScript: 50, JSON: 20 }"))]
EDITS[WC + "🔬️micro-commit/🔶️safegitenv-replaces-unusable-configured/🟦️.ts"] = [
    ("{\n        countRepoByLanguage: () => ({ TypeScript: 1000, Rust: 500 }),\n      }", "{\n        countRepoUlocByLanguage: () => ({ TypeScript: 1000, Rust: 500 }),\n        countRepoSizeByLanguage: () => ({}),\n      }"),
    ("{\n          countRepoByLanguage: () => ({}),\n        }", "{\n          countRepoUlocByLanguage: () => ({}),\n          countRepoSizeByLanguage: () => ({}),\n        }"),
    ('const { MICRO_COMMIT_ULOC_HEADER: ulocHeader } = await import("../../../../🎛️commands/📝️commit/📊️metrics/🧮️summary/🟦️.ts");', 'const { COMMIT_METRIC_HEADER: ulocHeader } = await import("../../../../🎛️commands/📝️commit/📊️metrics/🎨️format/🟦️.ts");'),
]


TAX_VALIDATION = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🗿️taxonomy-validation/🟦️.ts"
EDITS[TAX_VALIDATION] = [
    ('export const LEGACY_CONFIG_FACET_DIR = "🧮️config";\n\n', ""),
    ('export const LEGACY_WASM_DIR = "🕸️wasm";\n\n', ""),
    (
        '    if (existsSync(join(surfaceAbs, LEGACY_CONFIG_FACET_DIR))) {\n      findings.push(`${pluginId}: surface "${label}" still has ${LEGACY_CONFIG_FACET_DIR}/ — rename to ${CONFIG_FACET_DIR}/`);\n    }\n    if (existsSync(join(surfaceAbs, LEGACY_WASM_DIR))) {\n      findings.push(`${pluginId}: surface "${label}" still has ${LEGACY_WASM_DIR}/ — rename to 🌉️wasm/`);\n    }\n',
        "",
    ),
    (
        '  if (existsSync(join(pluginRoot, LEGACY_CONFIG_FACET_DIR))) {\n    findings.push(`${pluginId}: plugin-root still has ${LEGACY_CONFIG_FACET_DIR}/ — rename to ${CONFIG_FACET_DIR}/`);\n  }\n',
        "",
    ),
]


OWNER_DISCOVERY = LIB + "🧬️schema/🗺️surface/🔍️owner-discovery/🟦️.ts"
EDITS[OWNER_DISCOVERY] = [
    ("import { POLICY_APP_CONFIG_DIR, POLICY_APP_CONFIG_LEGACY_DIR, POLICY_APP_PRESENCE_DIR,", "import { POLICY_APP_CONFIG_DIR, POLICY_APP_PRESENCE_DIR,"),
    ("        legacyRel = `${surfaceRel}/${POLICY_APP_CONFIG_LEGACY_DIR}`,\n", ""),
    (
        "      for (const candidate of [sliderRel, legacyRel]) {\n        const source = policySourceDirectory(repoRoot, candidate, operations);\n        if (source.state === \"directory\") {\n          ownerRel = candidate;\n          break;\n        }\n        if (source.state !== \"missing\") throw new Error(`Surface owner source directory ${candidate} is ${source.state}.`);\n      }\n",
        "      const sliderSource = policySourceDirectory(repoRoot, sliderRel, operations);\n      if (sliderSource.state === \"directory\") ownerRel = sliderRel;\n      else if (sliderSource.state !== \"missing\") throw new Error(`Surface owner source directory ${sliderRel} is ${sliderSource.state}.`);\n",
    ),
]


TICKETS = MOD + "🎫️tickets/"
GO_TICKET_MODEL = MOD + "📐️model/📦️packages/🐹️go/🎫️ticket/🐹️.go"
GO_TICKET_DATA = MOD + "📐️model/📦️packages/🐹️go/📼️ticket-data/🐹️.go"
CLI_TICKET_MODEL = MOD + "💻️client/⌨️cli/🎟️ticket-model/🐹️.go"
CLI_INTERACTIONS = MOD + "💻️client/⌨️cli/💬️interactions/🐹️.go"
GO_CODEC = TICKETS + "📦️packages/🐹️go/🌦️codec/🐹️.go"
RUST_CODEC = TICKETS + "📦️packages/🦀️rust/📄️codec/🦀️.rs"
RUST_TICKETS_ROOT = TICKETS + "📦️packages/🦀️rust/🦀️.rs"
TICKET_SCHEMA = TICKETS + "🧬️schema/🔣️.json"
TICKET_CODEC_TEST_TS = TICKETS + "🧪️tests/🎫️ticket-document-codec/🟦️.ts"
MODEL_FUNC_END = "\n// 😀️MarshalJSON"

MODEL_UNMARSHAL = """func (t *Ticket) UnmarshalJSON(data []byte) error {
	aux := &struct {
		Title        string                `json:"title"`
		Emoji        string                `json:"emoji,omitempty"`
		Description  string                `json:"description,omitempty"`
		Status       TicketStatus          `json:"status"`
		Summary      string                `json:"summary,omitempty"`
		Management   *TicketManagementData `json:"github,omitempty"`
		Goal         string                `json:"goal,omitempty"`
		Parent       string                `json:"parent,omitempty"`
		Plan         *TicketPlan           `json:"plan,omitempty"`
		Sessions     []string              `json:"sessions"`
		Interactions []Interaction         `json:"interactions"`
	}{}
	if err := json.Unmarshal(data, aux); err != nil {
		return err
	}
	if !aux.Status.IsValid() {
		return fmt.Errorf("ticket status must be explicitly \\"open\\" or \\"closed\\"")
	}
	*t = Ticket{
		Title:       aux.Title,
		Emoji:       aux.Emoji,
		Description: aux.Description,
		Status:      aux.Status,
		Summary:     aux.Summary,
		Management:  aux.Management,
		Goal:        aux.Goal,
		Parent:      aux.Parent,
		Plan:        aux.Plan,
	}
	if len(aux.Interactions) > 0 {
		t.Interactions = aux.Interactions
	}
	for _, sessionID := range aux.Sessions {
		AppendTicketSessionID(t, sessionID)
	}
	return nil
}
"""


def replace_function(start, end_marker, new):
    def edit(text):
        a = text.index(start)
        b = text.index(end_marker, a)
        return text[:a] + new + text[b:]
    return edit


EDITS[GO_TICKET_MODEL] = [replace_function("func (t *Ticket) UnmarshalJSON(data []byte) error {", MODEL_FUNC_END, MODEL_UNMARSHAL)]

CLI_AUX_OLD = """		PromptLegacy       string          `json:"prompt"`
		InteractionsLegacy []Interaction   `json:"interactions"`
		AgentsLegacy       []TicketAgent   `json:"agents"`
		SessionsRaw        json.RawMessage `json:"sessions"`
"""
CLI_AUX_NEW = """		Sessions     []string      `json:"sessions"`
		Interactions []Interaction `json:"interactions"`
"""
CLI_BODY_NEW = """	if len(aux.Interactions) > 0 {
		t.Interactions = aux.Interactions
	}
	for _, sessionID := range aux.Sessions {
		appendTicketSessionID(t, sessionID)
	}
"""
EDITS[CLI_TICKET_MODEL] = [
    (CLI_AUX_OLD, CLI_AUX_NEW),
    ("		Plan               *TicketPlan     `json:\"plan,omitempty\"`\n", "		Plan         *TicketPlan   `json:\"plan,omitempty\"`\n"),
    replace_function("	if t.Description == \"\" && aux.PromptLegacy != \"\" {", "	if t.Summary == \"\" {", CLI_BODY_NEW),
    ("	for _, agent := range t.Agents {\n		appendTicketSessionID(t, agent.Session)\n	}\n\n", ""),
    ("	for i := range t.Agents {\n		if strings.HasPrefix(t.Agents[i].Contributor, contributorPrefix) {\n			t.Agents[i].Contributor = composeIDToContributorGithub(t.Agents[i].Contributor)\n		}\n	}\n", ""),
]

for _path, _next in ((GO_TICKET_DATA, "\n// 🎁️InteractionResource"), (CLI_INTERACTIONS, "\n// 🔐️resolveAuthorToAlias")):
    EDITS[_path] = [replace_function("// 🔲️UnmarshalJSON MUST handle both legacy and current JSON layouts.\nfunc (i *Interaction) UnmarshalJSON(", _next, "")]
EDITS[CLI_INTERACTIONS].append(("import (\n	\"context\"\n	\"encoding/json\"\n	\"fmt\"\n	\"strings\"\n)", "import (\n	\"context\"\n	\"fmt\"\n	\"strings\"\n)"))
EDITS[GO_TICKET_DATA].append(("import (\n	json \"encoding/json\"\n	fmt \"fmt\"\n	time \"time\"\n)", "import (\n	time \"time\"\n)"))

AUTHOR_TEST_OLD = """		{
			name:         "object author",
			authorJSON:   `{"name":"Ueli Saluz","email":"ueli@semio-tech.com","github":"usalu"}`,
			expectedAuth: "Ueli Saluz <ueli@semio-tech.com>",
		},
	}
"""
AUTHOR_TEST_NEW = """	}
"""
OBJECT_AUTHOR_TEST = """
func TestInteractionUnmarshalRefusesAnObjectAuthor(t *testing.T) {
	var interaction Interaction
	if err := json.Unmarshal([]byte(`{"date":"2026-02-06 22:03:11","author":{"name":"Ueli Saluz","email":"ueli@semio-tech.com"}}`), &interaction); err == nil {
		t.Fatalf("an object author is not part of the interaction schema, got %+v", interaction)
	}
}
"""
for _path in (MODEL_UNIT_TEST, MOD + "💻️client/⌨️cli/🧪️tests/🪟️interactions-contributors-mcp-bootstrap/🐹️.go"):
    EDITS.setdefault(_path, [])
    EDITS[_path] += [
        (AUTHOR_TEST_OLD, AUTHOR_TEST_NEW),
        ("func TestInteractionUnmarshalAuthorShapes(t *testing.T) {", "func TestInteractionUnmarshalReadsTheTextAuthor(t *testing.T) {"),
        lambda text: text.replace("\n// 📌️#region 🔑️Compose Repo ID Conversion", OBJECT_AUTHOR_TEST + "\n// 📌️#region 🔑️Compose Repo ID Conversion", 1) if "// 📌️#region 🔑️Compose Repo ID Conversion" in text else text.replace("\nfunc TestContributorDiscovery(", OBJECT_AUTHOR_TEST + "\nfunc TestContributorDiscovery(", 1),
    ]

CODEC_SESSIONS_OLD_START = "	if entries, ok := members[\"sessions\"].([]interface{}); ok {"
CODEC_SESSIONS_NEW = """	if entries, ok := members["sessions"].([]interface{}); ok {
		for _, entry := range entries {
			if sessionID, ok := entry.(string); ok {
				AppendSessionID(ticket, sessionID)
			}
		}
	}
"""
EDITS[GO_CODEC] = [
    replace_function("// 🔳️decodeAgent decodes one agent record leniently", "// 📖️DecodeTicketDocument decodes one ticket document.", ""),
    ("	agents := make([]model.TicketAgent, 0)\n	for _, entry := range objectsOf(members[\"agents\"]) {\n		agents = append(agents, decodeAgent(entry))\n	}\n\n", "\n"),
    ("	if ticket.Description == \"\" {\n		ticket.Description = textMember(members, \"prompt\")\n	}\n", ""),
    replace_function(CODEC_SESSIONS_OLD_START, "	if ticket.Summary == \"\" {", CODEC_SESSIONS_NEW),
    ("	for _, agent := range ticket.Agents {\n		AppendSessionID(ticket, agent.Session)\n	}\n", ""),
]


_TOOLS = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
_ROOT = os.path.abspath(os.path.join(_TOOLS, *([".."] * 8)))
sys.path.insert(0, os.path.join(_TOOLS, "convert"))
import tickets as _ticket_conversion

for _ticket in _ticket_conversion.legacy_paths(_ROOT):
    EDITS[_ticket] = [lambda text, _path=_ticket: _ticket_conversion.convert_text(text, _path)]

RUST_CODEC_DOC_OLD = """/// than `open`/`closed`, is refused rather than defaulted. Three pre-schema spellings are absorbed:
/// a `prompt` member fills an absent `description`, a `sessions` array of agent objects contributes
/// its session ids, and a closing interaction's summary fills an absent `summary`.
"""
RUST_CODEC_DOC_NEW = """/// than `open`/`closed`, is refused rather than defaulted. A closing interaction's summary fills an
/// absent `summary`; `sessions` entries that are not text are skipped.
"""
RUST_SESSIONS_NEW = """    if let Some(Value::Array(entries)) = members.get("sessions") {
        for entry in entries.iter().filter_map(Value::as_str) {
            append_session_id(&mut ticket, entry);
        }
    }
"""
EDITS[RUST_CODEC] = [
    replace_function("/// 🔳️ Decodes one agent record leniently", "/// 📖️ Decodes one `🎫️ticket.json` document.", ""),
    (RUST_CODEC_DOC_OLD, RUST_CODEC_DOC_NEW),
    ("    let mut agents: Vec<TicketAgent> = decode_array(members.get(\"agents\"), decode_agent);\n", ""),
    ("    if ticket.description.is_empty() {\n        ticket.description = text_member(\"prompt\");\n    }\n", ""),
    replace_function("    match members.get(\"sessions\") {", "    if ticket.summary.is_empty() {", RUST_SESSIONS_NEW),
    ("    let agent_sessions: Vec<String> = ticket.agents.iter().map(|agent| agent.session.clone()).collect();\n    for session in agent_sessions {\n        append_session_id(&mut ticket, &session);\n    }\n", ""),
]
EDITS[RUST_TICKETS_ROOT] = [("use semio_framework_repo_model::{Interaction, InteractionFile, Ticket, TicketAgent, TicketManagementData, TicketPlan, TicketStatus};", "use semio_framework_repo_model::{Interaction, InteractionFile, Ticket, TicketManagementData, TicketPlan, TicketStatus};")]
EDITS[TICKET_SCHEMA] = [
    ('        "prompt": {\n          "description": "📥️ Pre-schema spelling of `description`, absorbed when `description` is absent.",\n          "type": "string"\n        },\n', ""),
    ('        },\n        "agents": {\n          "description": "📥️ Pre-schema agent log, absorbed but never re-encoded.",\n          "type": "array"\n        }\n      },\n      "required": [\n        "title",', '        }\n      },\n      "required": [\n        "title",'),
]


EDITS[TICKET_CODEC_TEST_TS] = [
    ("    description: text(members, \"description\") === \"\" ? text(members, \"prompt\") : text(members, \"description\"),", "    description: text(members, \"description\"),"),
    ("    for (const entry of sessions) append(typeof entry === \"string\" ? entry : ((entry as Record<string, unknown> | null)?.session ?? \"\"));", "    for (const entry of sessions) append(entry);"),
    ("  const agents = members.agents;\n  if (Array.isArray(agents)) for (const entry of agents) append((entry as Record<string, unknown> | null)?.session ?? \"\");\n", ""),
]

SUBSET_CONFORMANCE = LAWS + "🪆️subset-conformance/🟦️.ts"
SHIM_LAW_NEW = """/** 🚫 A crate-root `🦀️.rs` under `🗿️artifacts` must not carry a block that keeps pre-migration module paths alive. */
export function policyNoGlueShimBlocksBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const artifactRel of policyListPluginArtifactDirs(repoRoot)) {
    const rel = `${artifactRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`;
    if (!/Shims:\\s*keep pre-migration|pre-migration module paths/i.test(policyReadFileSafe(repoRoot, rel))) continue;
    breaches.push({
      id: `glue-shim-${rel}`,
      summary: `"${rel}" still contains pre-migration shim blocks`,
      kind: "subset-conformance/no-glue-shim-blocks",
      scope: rel,
      priority: "medium",
      reason: "Greenfield forbids compatibility shims; the crate-root module paths are the crate's public surface and are named so.",
      solution: "Name the block after what it is (the crate's public surface) and delete any path nothing uses.",
    });
  }
  return breaches;
}

"""
EDITS[SUBSET_CONFORMANCE] = [replace_function("/** 🚫 Glue compatibility shims keeping pre-migration module paths must be deleted. */", "/** 📦 Aggregate subset-conformance scanners. */", SHIM_LAW_NEW)]

SHIM_HEADING_OLD = "// ---- Shims: keep pre-migration module paths resolving for external callers ----"
SHIM_HEADING_NEW = "// ---- Crate-root module paths: the crate's public surface ----"


def shim_artifact_roots(root):
    plugins = os.path.join(root, "✏️s", "🔌️plugins")
    for plugin in sorted(os.listdir(plugins)):
        artifacts = os.path.join(plugins, plugin, "🗿️artifacts")
        if not os.path.isdir(artifacts):
            continue
        for artifact in sorted(os.listdir(artifacts)):
            path = os.path.join(artifacts, artifact, "🦀️.rs")
            if os.path.isfile(path) and SHIM_HEADING_OLD in open(path, encoding="utf8").read():
                yield os.path.relpath(path, root).replace(os.sep, "/")


for _shim in shim_artifact_roots(_ROOT):
    EDITS[_shim] = [(SHIM_HEADING_OLD, SHIM_HEADING_NEW)]

SHIM_LAW_TEST = """
describe("no-glue-shim-blocks policy", () => {
  const block = (heading: string): string => `//! crate\\n\\n${heading}\\npub mod schema {}\\n`;
  const fixture = (heading: string): { root: string; rel: string } => {
    const root = mkdtempSync(join(tmpdir(), "semio-glue-shim-"));
    const rel = "✏️s/🔌️plugins/🧪️probe/🗿️artifacts/📊️csv";
    mkdirSync(join(root, rel), { recursive: true });
    writeFileSync(join(root, rel, "🦀️.rs"), block(heading));
    return { root, rel };
  };
  test("reads the crate-root file of every plugin artifact and refuses the pre-migration shim heading", () => {
    const { root, rel } = fixture("// ---- Shims: keep pre-migration module paths resolving for external callers ----");
    try {
      expect(policyNoGlueShimBlocksBreaches(root).map((breach) => breach.scope)).toEqual([`${rel}/🦀️.rs`]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("accepts the crate's public surface block", () => {
    const { root } = fixture("// ---- Crate-root module paths: the crate's public surface ----");
    try {
      expect(policyNoGlueShimBlocksBreaches(root)).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});
"""
EDITS[LIB + "🧪️tests/🔬️workspace-contract/🧩️policy-laws/🟦️.ts"] += [
    ('import { policyEmojiPrefixBreaches } from', 'import { policyNoGlueShimBlocksBreaches } from "../../../⚖️laws/🧩️plugin-consistency/🪆️subset-conformance/🟦️.ts";\nimport { policyEmojiPrefixBreaches } from'),
    lambda text: text.rstrip("\n") + "\n" + SHIM_LAW_TEST,
]

HUB = "🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/"
SPACE_ENGINE = HUB + "⚙️engine/🦀️.rs"
SPACE_INSPECTION = HUB + "📌️panels/🔍️inspection/🦀️.rs"
EDITS[SPACE_ENGINE] = [("os_parameter_types_compatible_shim", "workflow_parameter_fits_os_type")]
EDITS[SPACE_INSPECTION] = [("use semio_hub_space_operations::{os_parameter_types_compatible_shim, parameter_entity_id};", "use semio_hub_space_operations::{parameter_entity_id, workflow_parameter_fits_os_type};"), ("os_parameter_types_compatible_shim", "workflow_parameter_fits_os_type", 2)]

FLOW_HOST = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/"
FLOW_PORTS = FLOW_HOST + "🔌️ports/🦀️.rs"
FLOW_EVALUATION_SOURCE = FLOW_HOST + "📥️evaluation-source/🦀️.rs"
FLOW_DAG_TEST = FLOW_HOST + "🧪️tests/🔬️unit/🕸️node-graph-surface-and-dag-edges/🦀️.rs"
EDITS[FLOW_PORTS] = [
    ("/// An exact id is kept. Legacy `out` / `in` and an empty id select the first port on that side.", "/// An exact id is kept. An empty id selects the first port on that side."),
    ("    let legacy = matches!((side, port_id), (PortSide::Output, \"\" | \"out\") | (PortSide::Input, \"\" | \"in\"));\n    if legacy { ports.first().cloned() } else { None }\n", "    if port_id.is_empty() { ports.first().cloned() } else { None }\n"),
]
EDITS[FLOW_EVALUATION_SOURCE] = [
    ("let alias=if self.side{\"in\"}else{\"out\"};match self.compare(requested,alias,grant){Some(equal)=>{if requested.is_empty()||equal{if self.numeric_port(source,infos){self.number(b\"\",0)}self.port_choice=self.port(source,infos,0).map(|_|0)}self.phase=40},None=>{}}},", "self.progress.copied_items=1;if requested.is_empty(){if self.numeric_port(source,infos){self.number(b\"\",0)}self.port_choice=self.port(source,infos,0).map(|_|0)}self.phase=40},"),
]
FLOW_TEST_OLD = """#[test]
fn legacy_out_in_synapse_joins_canonical_ports_and_paints_an_engine_edge() {
    let mut host = host_with_test_fixture();
    let mut snapshot = host.host_snapshot.clone();
    snapshot.synapses.clear();
    snapshot.synapses.push(SynapseSpec { id: "legacy-wire".into(), from: "slider".into(), to: "add".into(), from_port: "out".into(), to_port: "in".into() });
    host.replace_host_snapshot(snapshot);
    let edge = host.dag.graph.host_snapshot.edges.iter().find(|edge| edge.id == "legacy-wire").expect("legacy synapse");
    assert_eq!(edge.source, "slider@number");
    assert_eq!(edge.target, "add@a");
    assert_eq!(host.dag.graph.engine.edges.len(), 1);
    host.retire_cold();
}
"""
FLOW_TEST_NEW = """#[test]
fn empty_synapse_ports_join_the_first_port_of_each_side_and_paint_an_engine_edge() {
    let mut host = host_with_test_fixture();
    let mut snapshot = host.host_snapshot.clone();
    snapshot.synapses.clear();
    snapshot.synapses.push(SynapseSpec { id: "default-wire".into(), from: "slider".into(), to: "add".into(), from_port: String::new(), to_port: String::new() });
    host.replace_host_snapshot(snapshot);
    let edge = host.dag.graph.host_snapshot.edges.iter().find(|edge| edge.id == "default-wire").expect("default synapse");
    assert_eq!(edge.source, "slider@number");
    assert_eq!(edge.target, "add@a");
    assert_eq!(host.dag.graph.engine.edges.len(), 1);
    host.retire_cold();
}

#[test]
fn retired_out_in_port_ids_no_longer_resolve_and_create_no_engine_edge() {
    let mut host = host_with_test_fixture();
    let mut snapshot = host.host_snapshot.clone();
    snapshot.synapses.clear();
    snapshot.synapses.push(SynapseSpec { id: "retired-wire".into(), from: "slider".into(), to: "add".into(), from_port: "out".into(), to_port: "in".into() });
    host.replace_host_snapshot(snapshot);
    assert!(host.host_snapshot.synapses.iter().any(|synapse| synapse.id == "retired-wire"), "an unresolved port stays in the document until it can be joined");
    assert!(host.dag.graph.engine.edges.is_empty());
    host.retire_cold();
}
"""
EDITS[FLOW_DAG_TEST] = [
    (FLOW_TEST_OLD, FLOW_TEST_NEW),
    ('from_port: "out".into(), to_port: "in".into() }],\n        layout', 'from_port: String::new(), to_port: String::new() }],\n        layout'),
    ('"legacy out/in synapse becomes one workflow edge"', '"an empty-port synapse becomes one workflow edge"'),
    ('from_port: "out".into(), to_port: "missing".into()', 'from_port: String::new(), to_port: "missing".into()', 2),
]

GENERATION2D = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/"
GEN2D_BRIDGE = GENERATION2D + "🕹️play-app-editor/🦀️.rs"
GEN2D_COMMAND = GENERATION2D + "🎮️commands/🖱️canvas-pointer-move/🦀️.rs"
GEN2D_TEST = GENERATION2D + "🧪️tests/🔬️unit/📜️manifest-and-cross-cutting-laws/🦀️.rs"
EDITS[GEN2D_BRIDGE] = [
    replace_function("        let pointer_samples = || {\n            let parsed = args\n", "        let pointer_cancelled =", """        let pointer_samples = || {
            args.get("samples")
                .and_then(semio_framework_value::DslValue::as_array)
                .map(|items| items.iter().filter_map(|item| { let pair = item.as_array()?; Some([pair.first()?.as_f64()?, pair.get(1)?.as_f64()?]) }).collect::<Vec<[f64; 2]>>())
                .unwrap_or_default()
        };
"""),
]
EDITS[GEN2D_COMMAND] = [("    /// 🧵️ Every pointer sample of this batch as canvas pixels, oldest first (design L4 / §2 D);\n    /// the action bridge folds a legacy `{x, y}` wire into the single `[[x, y]]`. This app keeps\n    /// no drag gesture on its canvas, so the batch is carried, not consumed.", "    /// 🧵️ Every pointer sample of this batch as canvas pixels, oldest first (design L4 / §2 D).\n    /// This app keeps no drag gesture on its canvas, so the batch is carried, not consumed.")]
EDITS[GEN2D_TEST] = [
    ("/// 🧵️ LAW (design L4 / §2 D): a legacy `{x, y}` `canvasPointerMove` wire folds into one sample, a\n/// batched wire keeps every `[x, y]` pair in order, and `canvasPointerUp` without `cancelled` is a\n/// real release. Generation2d keeps no canvas drag gesture, so both handlers stay no-ops either way.", "/// 🧵️ LAW (design L4 / §2 D): a `canvasPointerMove` wire carries its `samples` batch in order (a bare\n/// `{x, y}` pair is no batch and yields none), and `canvasPointerUp` without `cancelled` is a real\n/// release. Generation2d keeps no canvas drag gesture, so both handlers stay no-ops either way."),
    ("DslValue::object([(\"x\".into(), f(3.0)), (\"y\".into(), f(4.0)), (\"samples\".into(), ", "DslValue::object([(\"samples\".into(), "),
    ("fn canvas_pointer_wire_defaults_samples_and_cancelled() {", "fn canvas_pointer_wire_reads_only_a_samples_batch_and_defaults_cancelled() {"),
    ("    let legacy = semio_framework_value::DslValue::object([(\"x\".into(), f(5.0)), (\"y\".into(), f(6.0))]);\n    assert_eq!(bridge(\"canvasPointerMove\", legacy.clone()), Generation2dCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { samples: vec![[5.0, 6.0]] }), \"an absent `samples` is the single (x, y)\");", "    let bare_point = semio_framework_value::DslValue::object([(\"x\".into(), f(5.0)), (\"y\".into(), f(6.0))]);\n    assert_eq!(bridge(\"canvasPointerMove\", bare_point.clone()), Generation2dCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { samples: Vec::new() }), \"a bare (x, y) pair is not a sample batch\");"),
    ("bridge(\"canvasPointerUp\", legacy)", "bridge(\"canvasPointerUp\", bare_point)"),
]

WAV_GRAMMAR = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/📖️.grammar.semio"
WFC_GRAMMAR = "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/📖️.grammar.semio"
EDITS[WAV_GRAMMAR] = [("start operation\n", "start mutation\n"), ("\noperation = object\n", "\nmutation = object\n")]
EDITS[WFC_GRAMMAR] = [("start operation\n", "start mutation\n"), ("\noperation = keyword", "\nmutation = keyword")]

TICKETS_UNIT_TEST = TICKETS + "🧪️tests/🔬️unit/🐹️.go"
TICKET_SCHEMA_TEST = """
func TestTicketDocumentsAreReadOnlyInTheCurrentSchema(t *testing.T) {
	text := `{"title":"T","status":"closed","prompt":"retired description","agents":[{"session":"agent-session"}],"sessions":["kept",{"session":"object-session","client":"x"}],"interactions":[{"kind":"ticket.open","date":"2026-02-11 20:37:49","author":"a","system":"mac","client":"c","checkpoint":""}]}`
	ticket, err := DecodeTicketDocument(text)
	if err != nil {
		t.Fatal(err)
	}
	if ticket.Description != "" || len(ticket.Agents) != 0 {
		t.Fatalf("a retired prompt or agents member must not be read: %+v", ticket)
	}
	if len(ticket.Sessions) != 1 || ticket.Sessions[0] != "kept" {
		t.Fatalf("only text session ids are read, got %v", ticket.Sessions)
	}
	if len(ticket.Interactions) != 1 || ticket.Interactions[0].Date != "2026-02-11 20:37:49" {
		t.Fatalf("interactions are read as written, got %+v", ticket.Interactions)
	}
	var decoded model.Ticket
	if err := json.Unmarshal([]byte(text), &decoded); err == nil {
		t.Fatalf("the model decoder must refuse an object session, got %+v", decoded)
	}
}
"""
EDITS[TICKETS_UNIT_TEST] = [lambda text: text.rstrip("\n") + "\n" + TICKET_SCHEMA_TEST]


STDIO_ART = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/"
EDITS.setdefault(STDIO_ART + "💾️binary/🦀️.rs", []).extend([("`pub mod engine` shim below)", "`engine` re-export below)")])
EDITS.setdefault(STDIO_ART + "🔤️txt/🦀️.rs", []).extend([("`pub mod engine` shim)", "`engine` re-export)")])
for _name in ("🖋️dxf", "📕️xlsx", "📜️docx", "📽️pptx"):
    EDITS.setdefault(STDIO_ART + _name + "/🦀️.rs", []).extend([("`engine` shim", "`engine` re-export")])
NORM_ART = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/"
EDITS.setdefault(NORM_ART + "🪨️en1996/🦀️.rs", []).extend([("/// ♻️ Compatibility alias — former nested `part_2` module.\n\n", "")])
EDITS.setdefault(NORM_ART + "🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs", []).extend([("/// 📋️ Compatibility alias used by older call sites.\npub fn check_full_geotechnical(document: &En1997Snapshot) -> CheckReport {\n    check_project(document)\n}\n", "")])


# round 6: the ticket `agents` member is a first-class, schema-defined field now (live schema: durable agent sessions and their plans), so the edits that treat it as a pre-schema log are dropped
EDITS = {key: [pair for pair in pairs if callable(pair) or not ("gent" in pair[0] or "gent" in pair[1])] for key, pairs in EDITS.items()}
