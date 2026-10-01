# Deployment Interface Audit — 2026-10-01

Read-only source review plus bounded Bun probes. No production files changed, builds run, Git state modified, ticket lifecycle changed, or child agents spawned. Instructions reviewed: workspace `AGENTS.md`, framework/products `AGENTS.md`, OS `AGENTS.md`, repo `AGENTS.md`, and supplied user rules. Current producer refresh queues are intentionally excluded from failure findings.

## Actionable Findings

### 1. Generic Catalog Still Requires Exactly One Specific Host

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts:225` filters all entries with `host`, demands exactly one, then demands exactly one app-less playground for that owner. `📽️projection/🟦️.ts:328` invokes this unconditionally before emitting any catalog file. A direct Bun invocation of `defaultHostVariant([], [])` logged refusal: `expected exactly one plugin crate ... found 0`. Therefore the lower-level empty deployment inventory test does not yet prove that deleting all specific owners preserves full generic catalog rendering. Multiple valid independent host owners are also prohibited by the global cardinality rule.

Recommended schema-first solution: the generic projection exposes an array of explicit host-session rows, keyed by owner and variant. Empty arrays and multiple hosts are valid. Specific deployment/launch owners select their boot variant explicitly in their own schema-backed launch/deployment metadata. Remove the generic default and its resolver; do not choose first, lexical minimum, a special namespace, or an invented fallback. If a narrower intermediate step is needed, emit a nullable explicitly selected boot reference, admit zero selection in generic render, and enforce required selection only at a boot caller boundary. Multiple host identities remain valid; only duplicate selections for the same explicit deployment owner should refuse.

Direct caller changes required after removal of `DEFAULT_HOST_VARIANT`:

- Browser boot `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts:14`: use owner-provided document/query boot variant; fail at actual boot if absent.
- Vite `🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts:37,249`: accept selected variant in explicit launch metadata/environment and validate it; no generic default.
- Build execution `🔌️plugin/🏗️build/🏃️execution/🟦️.ts:188`: callers requesting a session provide the selected variant, while unfiltered whole-catalog operations need no implicit host.
- Production plan `🧑‍💻dev/🚚️distribution/📋️plan/🟦️.ts:119,124`: explicit deployment plan selects its host.
- Session materialization `🧑‍💻dev/🎮️playground-session/🏃️execution/🟦️.ts:58,75`: render the explicitly requested session, with empty catalogs producing no synthetic session artifact.
- Parity `🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts:207,219,255`: explicit test scenario variant.
- Hub collaboration tests `🌎️hub/🧪️tests/🤝️dev-collaboration/🟦️.ts:471-496` and registry/session/browser-boot fixtures: fixture-owned selection.

`📽️projection/🟦️.ts:183,197,202` additionally requires one standalone playground and one studio session and asserts literal landing `home` and host `studio`. Iterate actual host rows and compare each session to its own admitted host metadata. Empty and host-only catalogs should not fail because no representative standalone exists. Add language-neutral cases for empty, standalone-only, two independent hosts, selected-owner removal, missing selected variant, and duplicate explicit selection; run Bun plus independent Node/AJV against the actual full render path.

### 2. Owner Deployment Schema Admits Values Rejected By Deployment Consumer

Owner declaration contract `🦑️repo/🔨️modules/📚️library/📇️catalog/🚚️deployment/🧬️schema/🔣️.json:7` admits any nonempty 128-code-point basename excluding slash, backslash, controls and dot/dot-dot. Consumer deployment uses `🧩️extension/🧬️schema/🔣️.json:6` `InstallationDirectoryV1`, requiring an explicit non-generic emoji plus portable slug, with max 192 code points. Its validator also requires NFC.

Actual Bun probe results:

| Authored Directory | Owner Admission | Deployment Admission |
| --- | --- | --- |
| `foo` | accepted | refused |
| `🧪️future` | accepted | accepted |
| `🧪️future?x` | accepted | refused |
| `🧪️future%2f` | accepted | refused |
| `🧪️future` followed by newline | refused | refused |

Keep generic compilation separate from optional deployment ownership, but use one schema authority for the value when deployment is declared. Prefer moving the shared authored portable directory definition to a neutral repo-owned contract and referencing it from both consumers; do not import OS product policy into the generic repo compilation layer. Include matching max length, whole-string anchoring, NFC/emoji identity rules, and malformed declaration fixtures in both portable implementations/oracles. Semantic sibling emoji uniqueness is currently enforced by runtime beyond plain JSON Schema; the existing emoji-regex oracle is a suitable independent semantic check.

### 3. Launch Declaration Duplicate Refusal Can Be Bypassed By Leading Whitespace

`📇️registry/🚀️launch/🏷️name-prefix/🧬️schema/🟦️.ts:14` filters raw lines with `^launch-name-prefix`, without trimming. `tomlBlocksAfterHeader` preserves body indentation (`🔎️discovery/🟦️.ts:393`). Actual Bun probe: canonical `launch-name-prefix = "🧪️test"` followed by an indented duplicate returned `🧪️test` instead of refusing; a sole indented declaration returned undefined. Whitespace is valid TOML. Trim each line before matching/counting to ensure duplicate refusal, or explicitly reject indented declarations rather than silently skipping them. Add duplicate-with-indentation, single-indented, and comment/string-decoy portable fixtures. The deployment declaration reader already trims its lines and detects duplicates correctly.

### 4. Source Registry Type Remains Weaker Than Emission Contract

`🔎️discovery/🟦️.ts:33` exposes optional `PluginRegistryEntry.directoryName`, but discovered deployment rows are filtered by declaration presence at lines167-170 and all row consumers require it through `📖️catalog-view/🟦️.ts:15`. Generated `PluginBuildTarget.directoryName` is correctly required. Make the deployed registry entry type required after deployment admission; preserve an optional declaration only on a separate generic component source type if needed. Direct source parsing should either return source metadata or require a declared deployment row according to its advertised return type. No external library types were found in the reviewed public deployment interfaces.

## Confirmed Improvements And Search Scope

The generic deployment module imports schema, route literals, and neutral installation validation; it no longer imports a central specific roster. Forward/reverse/static directory lookup require explicit supplied inventories. Generated `COMPONENT_MODULE_DIRECTORIES` is derived from generated owner rows, validated once, and does not establish a second hand-authored source authority. Caller-side imports bind a specific generated inventory at composition boundaries; the lookup implementation remains roster-independent. The routes file remains an intentional generic route authority.

Read-only repository `rg` checks found no direct old one-argument `moduleDirectoryName` or `moduleIdForDirectoryName` invocation and no references to removed `🗺️catalog.json` outside ticket history, dependencies, build outputs, or Git internals. The Rust/script checks found unrelated local configuration catalog JSON functions, not a remaining deployment roster consumer. `moduleStaticDirectoryNames` still permits undefined output for host mode by contract, but now resolves the supplied identity first; unknown owners refuse even in host mode. Forward lookup refuses missing/repeated identities; reverse lookup refuses repeated matching names and returns undefined for an unknown materialized basename.

The emitted target inventory requires each directory. Static generated bytes can remain stale while owner describe producers are queued; this review does not treat those queued bytes as implementation failure or request any compatibility fallback. No broad tests were rerun during concurrent Rust queues. Existing reported deployment/launch green suites were not represented here as independently rerun.
