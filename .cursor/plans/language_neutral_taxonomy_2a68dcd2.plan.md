---
name: Language Neutral Taxonomy
overview: "Extend the taxonomy mechanism so language-specific folders are only ever the last leaf: add a per-facet-kind schema format declaration (making WIT a first-class neutral schema format without forcing new leaves anywhere), promote package-folder purity from a soft burndown metric to a hard lint gate, and migrate all 21 violating folders / ~95 files to their correct neutral tree level."
todos:
  - id: w0-ticket-baseline
    content: "W0 (serial, 1 Composer 2.5 executor): read repo://goals, open ticket 26/08/17/LANGUAGE-NEUTRAL-TAXONOMY-AND-PACKAGE-PURITY (Go client ticket open if repo MCP is down), and capture baselines for verify taxonomy report, policy, and the burndown packaging-violation list into the ticket folder"
    status: completed
  - id: explore-references
    content: "X1-X3 (parallel, read-only Composer 2.5): X1 emit a per-migration reference manifest for every migrating path (Rust #[path]/include_str!, TS imports, script.ts constants, project.json, package.json, tsconfig.json, launch.json); X2 classify violating paths as tracked vs already gitignored; X3 re-audit packages after W1 to confirm completeness and no false positives"
    status: cancelled
  - id: w1-vocabulary
    content: "W1 (serial, sole writer of taxonomy.json and discovery component.ts): add schemaFormats.wit, schemaFacetKinds with normative-leaf-derived kind resolution, packagingDirNames global plus per-ecosystem, Taxonomy interface fields, validateTaxonomy SchemaFacetKindContract and kebab casing, schemaFacetFormats helper, promote collectPackagingViolations into census problems, extend the taxonomy and burndown tests"
    status: completed
  - id: w2-policy-engine
    content: "W2 (serial, sole writer of root script.ts): rewrite the five completeness policies to consult schemaFacetFormats, add a wit case to policyLoadSchemaFacetLeaves, add policyPackageLanguagePurityBreaches with its three statutes, register it in export const policy and VerifyScript.runGate, add nx targets; land reporting-only"
    status: completed
  - id: m1-plugin-wit
    content: "M1 (parallel): merge the 12 .wit files into plugin/schema/component.wit, repoint all 4 bindgen path literals, fix the dead plugin-world/extension-world references so both crates compile, update the 11 doc-comment path references, and delete the old wit folder"
    status: completed
  - id: m2-plugin-ts
    content: "M2 (parallel): hoist registry, window-kits and store out of plugin/packages/typescript to the plugin owner root; update the registry generator output paths, its project.json, plugin-web-materialize.ts and store.ts imports, and request the root script.ts runWorkspaceCodegen path change from the W2 owner"
    status: completed
  - id: m3-styling
    content: "M3 (parallel): hoist tokens.json, theme, tailwind, generated, generated.rs, net and vite-elements-assets.ts out of styling/packages/rust and tokens.generated.ts out of the typescript leaf to the styling owner root; update generateStylingArtifacts and the TS and Python generator wrappers"
    status: completed
  - id: m4-print
    content: "M4 (parallel): hoist print/packages/typescript/asset to print/assets and print/packages/typescript/template to print/template (29 files) and update the print script.ts path constants"
    status: completed
  - id: m5-small-hoists
    content: "M5 (parallel): hoist vscode extension assets, move os/packages/fixtures (20 tracked .bin) to os/fixtures, and move the four non-language sourcing windows package children to sibling dirs under windows"
    status: completed
  - id: m6-animate-cache
    content: "M6 (parallel, sole writer of .gitignore): delete the 19 tracked animate partial_movie_files runtime artifacts from the worktree and add an ignore rule; no git commands"
    status: completed
  - id: m7-residual-leaves
    content: "M7 (parallel): hoist the residual neutral leaf files (components.json, ui-axes.json, semio_logo.svg, the css files, the malformed manifest.jsonadapters.manifest.json) and fold dsl/schema/dsl_value_serde.rs into its own folder/component.rs"
    status: completed
  - id: w4-flip-and-verify
    content: "W4 (serial, 1 executor plus N parallel Composer 2.5 fixers): flip policyPackageLanguagePurityBreaches to high priority and make census problems fail verify taxonomy enforce; run verify taxonomy enforce, policy, verify-gate, run-many lint, full cargo build and the plugin dev pipeline; dispatch one fixer per independent failure cluster until zero breaches, confirming runtime behaviour with logs"
    status: completed
  - id: w5-register-and-close
    content: "W5 (serial, sole writer of launch.json): register the new commands in .vscode/launch.json following existing order, grouping and naming; document the new mechanism in taxonomy.json comments; verify the W0 vs W4 baseline diff shows zero new required leaves; close the ticket with summary and full file list"
    status: completed
isProject: false
---

## Problem

The repo declares Shape V2 tree purity in [🔣️taxonomy.json](🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json) `_treePurityComment`: inside an owner tree only a per-language leaf file, the `📦️packages` folder, and plain component folders may exist, and `📦️packages/<lang>/` holds ONLY packaging code, never data. That rule is **declared but not enforced** — the check exists in [🔍️discovery/🟦️component.ts](🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts) `collectPackagingViolations` (lines 1162-1169), but its output lands in `discoverBurndown().packagingViolations`, which no gate reads. Consequently 21 folders and ~95 files of language-neutral content sit inside language leaves.

The canonical example is WIT: 12 `.wit` files (827 lines) forming package `semio:framework@1.0.0` live at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️wit/`. WIT is a language-neutral IDL consumed by the Rust guest, the Rust host, and (indirectly, via jco's `--map semio:framework/host=./🟨️host-shim.js`) TypeScript. Its lowest common owner is the `🔌️plugin` module, not the Rust leaf.

## Mechanism design

### 1. Facet kind derived from the normative leaf

Adding `📜️wit` to `schemaFormats` naively is unsafe: `policySchemaFormatLeafBreaches` (📜️script.ts:11728), `policyArtifactSchemaFacetCompletenessBreaches` (:10208), `policyAppSchemaFacetCompletenessBreaches` (:10659), `policyOsConfigShapeBreaches` (:7406) and `policyInferenceFamilyRootCompletenessBreaches` (:9396) all iterate `Object.entries(taxonomy.schemaFormats)` and require every leaf. There is no per-facet subset mechanism.

So introduce one. A schema facet's **kind** is derived structurally from which normative leaf it carries — no marker file, no ambiguity:

```json
"schemaFormats": {
  "🔣️jsonschema": { "leafFilename": "🔣️component.json", "extension": ".json", "fieldCasing": "camel" },
  "🦀️rust": { ... }, "🟦️typescript": { ... }, "🔗️graphql": { ... }, "🛰️protobuf": { ... },
  "📜️wit": { "leafFilename": "📜️component.wit", "extension": ".wit", "fieldCasing": "kebab" }
},
"schemaFacetKinds": {
  "🧬️data": { "normativeFormat": "🔣️jsonschema", "formats": ["🔣️jsonschema", "🦀️rust", "🟦️typescript", "🔗️graphql", "🛰️protobuf"] },
  "📜️interface": { "normativeFormat": "📜️wit", "formats": ["📜️wit"] }
}
```

Every existing facet carries `🔣️component.json`, resolves to kind `🧬️data`, and its required format set is exactly today's five — **blast radius zero**. `validateTaxonomy()` gains a `SchemaFacetKindContract` region asserting: `normativeFormat` values are pairwise distinct, each kind's `formats` includes its own `normativeFormat`, every `formats` entry exists in `schemaFormats`, and every `schemaFormats` key is claimed by at least one kind. The `fieldCasing` enum in `validateTaxonomy()` (🟦️component.ts:684) must grow `"kebab"` for WIT identifiers.

Then a single shared helper `schemaFacetFormats(facetAbs)` replaces the five raw `Object.entries(schemaFormats)` loops. A facet carrying no normative leaf becomes a new breach ("undeclared schema facet kind") rather than silently unvalidated.

### 2. Package purity as a hard gate

Three statutes in one new `policyPackageLanguagePurityBreaches`, registered in `export const policy` (📜️script.ts:15180) and in `VerifyScript.runGate`, and mirrored as `buildSemanticCensus` problems so `verify taxonomy enforce` fails:

- **Children of `📦️packages/`** must be a declared `langs` key. Kills `📦️packages/fixtures/`, `📦️packages/🎚️options/`, `🎬️actions/`, `👥️presence/`, `🪛️utilities/`.
- **Files inside `📦️packages/<lang>[/🎯️targets/<t>]/`** must be packaging files — promote `collectPackagingViolations` from burndown to breach.
- **Directories inside `📦️packages/<lang>/`** must be `🎯️targets` or an ecosystem-declared packaging dir. New taxonomy keys: global `packagingDirNames: ["🎯️targets"]` plus per-ecosystem `ecosystems.<lang>.packagingDirNames` (`🦀️rust: ["benches"]`, `🐍️python` importable package dir).

The scanner must honour `.gitignore`, not walk the raw filesystem. `**/pkg/` and `**/🤖️generated/` are already ignored (`.gitignore:87,98`), so the four wasm-pack `pkg/` folders are ignored build output, not source violations — they need no migration, only exclusion. A raw walk also finds three phantom `📦️packages` roots inside build output that `git check-ignore` confirms are ignored (`storybook-static/asset/📦️packages`, `🧑️‍💻️dev/dist/asset/📦️packages`, `🧑️‍💻️dev/📦️packages/🟦️typescript/fixture/dist/asset/📦️packages`); the last one would otherwise be misread as a nested-packages violation. Reuse the existing `SEMANTIC_SKIP_DIRS` / `semanticWalk` skip discipline in `🔍️discovery/🟦️component.ts` rather than inventing a second exclusion list.

### 3. WIT lands as a neutral schema facet

New facet `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️component.wit`, one file holding `package semio:framework@1.0.0;` plus all 11 interfaces and `world actor`. Rationale: WIT resolves a *directory* as one package and treats subdirectories as `deps`, so per-interface child folders would break bindgen; and the repo already precedents one spec file per facet covering many rules (`📖️component.grammar.semio`). WIT `interface` blocks are the region mechanism.

Both bindgen sites repoint:

- `🔌️plugin/🦀️component.rs:18-21` and `:324-327`: `path: "📜️wit"` → `path: "../🧬️schema"` (relative to crate root `📦️packages/🦀️rust/`)
- `🔌️plugin/🖥️host/🦀️component.rs:17-22` and `:3567-3574`: `path: "../../../📦️packages/🦀️rust/📜️wit"` → `path: "../../🧬️schema"`

Blocking pre-existing defect surfaced by this work: `📜️world.wit` defines only `world actor`, but Rust still requests `world: "plugin-world"` (host:18) and `world: "extension-world"` (guest:325, host:3568). Those worlds no longer exist, so the plugin crates cannot compile today. The WIT migration must reconcile this — the honest fix is deleting the dead `extension-world` guest/host bindgen blocks and repointing the host to `world: "actor"`, since `describe()` → `PackageDescriptor.role` already distinguishes plugin from extension at runtime per `📜️world.wit:6`.

## Migrations

Disjoint owner trees, each with its own reference updates.

- **M1 plugin WIT** — 12 files merged into `🔌️plugin/🧬️schema/📜️component.wit`; 4 bindgen sites; doc refs at `🔌️plugin/🦀️component.rs:9688,9907,9921,15817`, `🖥️host/🦀️component.rs:27`, `🎠️kernel/🦀️component.rs:242,772,886,900,921`, `🛂️manifest/🟦️component.ts:1064`; dead-world reconciliation.
- **M2 plugin TS** — `📦️packages/🟦️typescript/{📇️registry,🪟️window-kits,🏪️store}/` → `🔌️plugin/{📇️registry,🪟️window-kits,🏪️store}/`; registry generator `📇️registry/📜️script.ts` output paths and its `📋️project.json`; `🌐plugin-web-materialize.ts` and `🏪️store/📜️store.ts` imports; root `📜️script.ts` `runWorkspaceCodegen()` target path.
- **M3 styling** — `🔣️tokens.json`, `🎨️theme/`, `🎨️tailwind/`, `🤖️generated/`, `🤖️generated.rs`, `net/`, `🟦️vite-elements-assets.ts` → `🎨️styling/` root; `🟦️tokens.generated.ts` from the TS leaf; `generateStylingArtifacts()` in `🎨️styling/📦️packages/🦀️rust/📜️script.ts` plus the TS and Python wrappers.
- **M4 print** — `📦️packages/🟦️typescript/{asset,📄️template}/` → `📓️print/{🖼️assets,📄️template}/` (29 files); `📓️print/📦️packages/🟦️typescript/📜️script.ts` path constants.
- **M5 small hoists** — `🧩️vscode/📦️packages/🟦️typescript/🖼️assets/` → `🧩️vscode/🖼️assets/`; `💻️os/📦️packages/fixtures/` (20 tracked `.bin`) → `💻️os/🧫️fixtures/`; four `🪵️sourcing/🧩️extensions/🪟️windows/📦️packages/{🎚️options,🎬️actions,👥️presence,🪛️utilities}/📌️empty.md` → sibling dirs under `🪟️windows/`.
- **M6 animate cache** — `🎞️animate/📦️packages/🦀️rust/partial_movie_files/` is 19 tracked runtime `.mp4`/`.png`/`index.json`; delete from the worktree and add an ignore rule. No git commands — file deletion plus `.gitignore` edit only.
- **M7 residual leaf files** — `🔣️components.json` and `🔣️ui-axes.json` under `🎯️targets/`, `🔣️semio_logo.svg` under `🎯️targets/🧊️wgpu/`, the `🎨️*.css` files under styling and UI react targets, `🛂️manifest.jsonadapters.manifest.json` (malformed name), `🗣️dsl/🧬️schema/dsl_value_serde.rs` (non-component leaf). Each hoists to its owner root or folds into a `<folder>/component.<ext>`.

## Agent workforce

Roles are fixed by the request: this plan is the Opus 5 artifact; one **Cursor Grok 4.5 High** coordinator owns the DAG, the single-writer registry, and gate flipping; **Composer 2.5** executors do the work; **Composer 2.5** explorers do read-only reconnaissance.

### Single-writer registry (hard rule)

Files touched by more than one wave get exactly one owning agent for the whole run. Any other agent needing a change there files a request to the coordinator instead of editing.

- `📜️script.ts` (root, 15264 lines) → W2 agent only
- `🔣️taxonomy.json` → W1 agent only
- `🔍️discovery/🟦️component.ts` → W1 agent only
- `.vscode/launch.json` → W5 agent only
- `.gitignore` → M6 agent only
- `Cargo.toml` (root) and `Cargo.lock` → M1 agent only

### Wave DAG

```mermaid
flowchart TD
  W0["W0 ticket + baseline (1 exec)"] --> X1["X1..X3 explorers read-only"]
  W0 --> W1["W1 vocabulary: taxonomy.json + discovery + tests (1 exec)"]
  X1 --> W1
  W1 --> W2["W2 policy engine: root script.ts (1 exec)"]
  W1 --> M1["M1 plugin WIT"]
  W1 --> M2["M2 plugin TS hoists"]
  W1 --> M3["M3 styling"]
  W1 --> M4["M4 print"]
  W1 --> M5["M5 small hoists"]
  W1 --> M6["M6 animate cache"]
  W1 --> M7["M7 residual leaves"]
  W2 --> W4
  M1 --> W4["W4 flip gates, run verify + policy + builds (1 exec + N fixers)"]
  M2 --> W4
  M3 --> W4
  M4 --> W4
  M5 --> W4
  M6 --> W4
  M7 --> W4
  W4 --> W5["W5 launch.json + docs + ticket close"]
```



W0, W1, W2, W4, W5 are strictly serial. X1-X3 and M1-M7 run fully parallel within their wave. Peak concurrency is 7 executors.

### Explorers (read-only, parallel, during W0)

- **X1** — enumerate every reference to each migrating path (Rust `#[path]`, `include_str!`, TS imports, `📜️script.ts` constants, `📋️project.json`, `package.json`, `tsconfig.json` paths, `.vscode/launch.json`) and emit a per-migration reference manifest into the ticket folder.
- **X2** — verify the `.gitignore` interaction: which violating paths are already ignored, which are tracked, so M6/M7 do not chase build output.
- **X3** — re-audit `📦️packages/` after the W1 vocabulary lands, using the new keys, to confirm the violation list is complete and that no legitimate packaging dir was mis-flagged.

### Executor briefs

Each executor gets: the ticket path, its exclusive file set, the single-writer registry, its acceptance command, and a mandate to write findings to a markdown file in the ticket folder rather than the chat.

- **W0** — open ticket `26/08/17/LANGUAGE-NEUTRAL-TAXONOMY-AND-PACKAGE-PURITY` (read `repo://goals` first and associate; if the repo MCP server is down, use the Go client `ticket open`, since `.cursor/mcp.json` shows the repo server is configured but was not connected). Capture baselines: `bun ./📜️script.ts verify taxonomy report`, `bun ./📜️script.ts policy`, and the current burndown packaging-violation list, all into the ticket folder.
- **W1** — `schemaFormats.📜️wit`, `schemaFacetKinds`, `packagingDirNames`, per-ecosystem `packagingDirNames`; `Taxonomy` interface fields; `validateTaxonomy()` `SchemaFacetKindContract` + `"kebab"` casing; `schemaFacetFormats()` helper; promote `collectPackagingViolations` into `discoverPackageProblems`/census problems; extend `🧪️index.test.ts` (`describe("validateTaxonomy")` ~1514) and the packaging burndown test (~2125). Acceptance: taxonomy tests green, `verify taxonomy report` shows zero new `taxonomy-schema` problems.
- **W2** — rewrite the five completeness policies to call `schemaFacetFormats()`; add a `📜️wit` case to `policyLoadSchemaFacetLeaves` so parity does not treat WIT as empty; add `policyPackageLanguagePurityBreaches`; register in `export const policy` and `runGate`; add nx targets in `📋️project.json`. Land the gate **reporting-only** first; W4 flips it to blocking.
- **M1-M7** — as scoped above. Each ends with its own targeted acceptance (M1: `cargo build -p semio-framework-plugin-host` and `cargo build -p semio-framework-plugin --target wasm32-wasip2 --features component-guest`; M2/M4: the owning nx `generate` target plus `bun nx run @semio-tech/framework-os-dev:plugin`; M3: styling `generate` across rust/ts/python wrappers).
- **W4** — flip `policyPackageLanguagePurityBreaches` to `priority: "high"` and wire census problems to fail `verify taxonomy enforce`. Run `bun ./📜️script.ts verify taxonomy enforce`, `bun ./📜️script.ts policy`, `bun nx run workspace:verify-gate`, `bun nx run-many -t lint`, full cargo build, and the plugin dev pipeline. Dispatch one Composer 2.5 fixer per independent failure cluster; iterate until zero breaches. Confirm runtime behaviour with logs, not inspection.
- **W5** — register the new commands in `.vscode/launch.json` following existing order/grouping/naming; update the `_treePurityComment` and add a `_languageNeutralityComment` to `🔣️taxonomy.json` documenting the ticket; close the ticket with the summary and full file list.

## Acceptance

1. No directory or non-packaging file exists under any `📦️packages/<lang>/` except `🎯️targets` and declared packaging dirs, and every `📦️packages/` child is a declared language.
2. `bun ./📜️script.ts verify taxonomy enforce` and `bun ./📜️script.ts policy` both exit zero.
3. `bun nx run workspace:verify-gate` passes; `nx run-many -t lint` clean.
4. Plugin guest and host crates compile, and the jco transpile pipeline still resolves `semio:framework/host`.
5. Adding `📜️wit` created zero new required leaves on any pre-existing schema facet — verified by diffing the W0 and W4 policy baselines.
6. Re-running the X3 audit against the new lint reports zero violations.

