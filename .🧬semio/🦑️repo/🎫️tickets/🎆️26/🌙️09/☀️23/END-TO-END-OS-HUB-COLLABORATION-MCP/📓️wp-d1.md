# WP-D1 — Capability Descriptions + Capability Audit (Session 12)

Slice D1 · session 12 · 2026-09-25. Outcome 4 (semio MCP): an agent can only use a verb well if the verb explains itself.
Mandate: `📓️audit-s12-ai-mcp.md` §2, G12-P1-1 (empty `description` catalog-wide), G12-P1-3 (`capability-audit-check`
not re-run since 09-22). No ports, no hub. Captures `wp-d1/generated/` (expendable); binaries
`CARGO_TARGET_DIR=wp-d1/target`.

Status legend: **measured** = ran here, capture named; **source** = read from source only; **written, not run**.

## Status

| # | Item | State | Evidence |
|---|---|---|---|
| 1 | Single source of truth for verb descriptions → descriptor → MCP gateway → shell; schema-required non-empty en + de; locale decision | **done** (§1, §2) | schema `CapabilityDescription*` in `🛂️manifest/🧬️schema/🔣️.json`; locale = en served, en+de required |
| 2 | Census law over every plugin's (and extension's) descriptor source: non-empty, specific en + de description per verb | **done, measured** | red 1038 before authoring (committed descriptors); `search::long` **4/4 green** over the landed source + frozen patch set projected, 3/4 over landed source alone (only the frozen crates' 55 verbs); quick laws 23/23, TS 2/2 (§2, §4) |
| 3 | Author descriptions per plugin (compile-atomic, `cargo check -p` per crate) | **done, measured** (source) | **782 handwritten en + de texts**: 772 in 25 plugins / 56 files + 7 framework tool-run + 3 SDK window-kit verbs; every `cargo check -p` EXIT 0; source census 962 + 14 SDK verbs covered, missing only the frozen crates (§3) |
| 4 | `capability-audit-check` re-run, findings → 0 at the root | **done at the source, measured on the projection**; frozen tail = patch set | committed: 4 + 1031 (red until W2 restage); landed projection: **0 audit findings**, 55 description findings (all frozen: gis/stdio/vcs); + frozen patch set: **0 + 0 over 60 descriptors**, Rust = AJV at every stage (§4) |
| 5 | Measured proof: census green, audit 0, offline `capabilities_search "delete"` score spread before/after; W2 request | **done, measured** | real `semio-os-mcp stdio` over each root: distinct scores 19/48 → 33/51, largest tie 12 → 5, empty descriptions in top 15: 15 → 0, score range 0.77 → 2.30 (§5); request `wp-w1/requests/d1.txt` filed. **The live MCP sees the new descriptions only after W2's restage** (describe-all of the `--packages all` publish) |

## 1. Source of truth and pipeline (source, measured where marked)

- **SSOT**: `ActionSemantics.description: Option<LocalizedLabel>` (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` `🔖️ActionSemantics`),
  shared by `ActionDefinition` and `CommandDefinition`. Authored in each plugin's manifest chain with
  `ActionDefinition::describe(LocalizedLabel::native(en, de))` or `AppBuilder/EditorBuilder::action_describe(id, …)`
  (`🔌️plugin/🦀️.rs:5287`, reaches app-scope, window-kind, app and mode commands).
- **Pipeline**: plugin Rust → wasm guest `describe()` → committed `<plugin>/🔣️.json` + `🛂️.descriptor.semio` (W2's
  describe-all; single-plugin `describe`) → `🌉️mcp/📇️registry::discover_catalog_source` → `🗂️catalog::compile(Locale::En,
  Terminology::Native)` → `capabilities_search` (BM25, description field ×2) / `capabilities_describe`. Contributions
  (io/composer/inference rows) get a gateway-derived sentence from their typed metadata (`ContributionRow::row_description`).
- **Shell**: no React/wgpu code reads `semantics.description` (grep over `🧰️framework` TS/TSX/RS: only the builder setters
  and the catalog projection) — the palette shows labels only.
- **Locale decision**: the gateway negotiates no catalog locale — `build_catalog`/`catalog_from_descriptors` compile with
  `Locale::En` only; `context_resolve` echoes a `locale` argument but nothing recompiles the catalog. So the MCP serves
  **en**; the descriptors carry **en + de** (no default language, AGENTS.md), and the census law requires both, so a
  locale-aware catalog (G10's gateway) can serve de without new authoring.
- **Scale (measured, `wp-d1/d1-census.py` over the 60 committed descriptors, restage4 18:36)**:
  `wp-d1/generated/census-before.json` — 1154 agent-published plugin verbs, **1031 without en+de description**
  (≈780 distinct verb ids per plugin), 105 input, 306 chrome; described already: cad 23, draw 15, forms 20, layout 12,
  note 20, raster 10, demonstrator 23 (composed). Extensions (cad/flow/imperative/process/sourcing modules) publish **0
  verbs** (topic contributions only). Framework-injected agent verbs without description: the 7 tool-run verbs.
- **Frozen crates** (session-12 rule 1: hub-native codec crates): stdio (4 distinct verbs, 9 apps), gis (19), vcs (6) →
  patch set in `wp-d1/`, lands after W2's `--packages all`.
- **capability-audit baseline (measured 23:1x)**: staged `semio-os-mcp` (built 23:15) `audit --folder <repo>` →
  **4 finding(s) over 60 descriptor(s)** (`wp-d1/generated/audit-before-staged.txt`), all `replace-text` (stdio html/md/txt +
  demonstrator playground), minted by the SDK `TextWindowKit` (M5b §9.2). The "29 findings" of 09-22 is stale.

## 2. The law (landed 23:17–23:38, measured)

- **Schema-first contract** — `🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json` `$defs`: `CapabilityDescription`
  (the `LocalizedLabel` matrix, both terminologies required), `CapabilityDescriptionLocales` (en + de required),
  `CapabilityDescriptionSentence` (24–480 chars, one line, capital start, sentence end), `CapabilityDescriptionFixture`.
  The three cross-field rules (German ≠ English; no native cell repeats the title; no two verbs of one app share an English
  description) are stated in the schema prose and implemented identically in Rust and TS.
- **Rust** (`🌉️mcp/🗂️catalog/🦀️.rs` `🔖️DescriptionLaw`): `DescriptionProblem {Missing, Schema, Untranslated, RepeatsTitle,
  SharedWithinApp}`, `description_problems(app verbs)` (owned validator over the manifest schema's `CapabilityDescription*`
  defs), `description_findings(source)` = the census over every descriptor app/command set + once over the framework-injected
  verbs (the framework's own definitions — `framework_action_definitions`, now shared with `framework_capabilities`).
- **TS twin + third-party oracle** (`🌉️mcp/🗂️catalog/🟦️.ts`, AJV): `capabilityDescriptionProblems`,
  `proveCapabilityDescriptionFixture`, `capabilityDescriptionCensus` (committed descriptors; plugin verbs).
- **Language-agnostic fixture** `🌉️mcp/🗂️catalog/🧫️fixtures/💬️capability-description.json` (11 app rows: pass, missing,
  empty de, too short, no punctuation, two lines, untranslated, reuse row incomplete, repeats title, shared within app, all
  three at once). Rust `catalog::quick::description_problems_match_the_language_agnostic_fixture` + census law
  `the_description_census_reports_undescribed_agent_verbs_by_capability_id`; TS `🧪️tests/💬️capability-description`.
- **Gate** — `semio-os-mcp audit` now prints `DescriptionFinding`s (`<capability id> [<problem>] <reason>`) beside the
  `CatalogAuditFinding`s and exits 1 on either; `capability-audit-check` (`📦️packages/🦀️rust/📜️script.ts`) additionally holds
  the Rust description census against the AJV census (plugin verbs) and fails on any disagreement. Launch row unchanged
  (`⚖️gate🌉️os-mcp🚨️capability-audit`).
- **Installed-catalog long laws** (`🔎️search/🧪️tests/🔬️long`): the six-plugin `AUTHORED_PLUGINS` scope is gone —
  `every_installed_agent_verb_explains_itself_in_en_and_de` (description census + non-empty en/de over the compiled catalog)
  and `every_installed_plugin_gates_destructive_and_user_path_verbs_and_keeps_chrome_out` (audit over every plugin).
- nx inputs: the manifest schema + catalog fixtures are inputs of `@semio-tech/framework-os-mcp-rs`; the TS twin, fixtures and
  suite of `@semio-tech/framework-os-mcp`.

| run (CARGO_TARGET_DIR=wp-d1/target, CARGO_INCREMENTAL=0) | result | capture |
|---|---|---|
| `cargo check -p semio-framework-os-mcp --tests` | EXIT 0, no warning in touched files | `generated/check-mcp-1.txt` |
| `cargo test --lib -- catalog:: search::quick` | **23 passed / 0 failed** (after scoping the validator to the `CapabilityDescription*` defs; first run 2 red: owned validator refused sibling cross-document refs) | `generated/test-catalog-2.txt` |
| TS `test quick capability-description` | **2/2 passed** | `generated/vitest-desc-1.txt` |
| `cargo test --lib -- search::long` (committed descriptors, before authoring) | 2 passed / **2 failed by design**: 1038 description findings (1031 plugin + 7 `framework.toolRun*`), 4 audit findings | `generated/test-long-before.txt` |
| `capability-audit-check` with this tree's binary (`SEMIO_OS_MCP_BIN`) | red by design: `4 finding(s), 1038 description finding(s) over 60 descriptor(s)`; **oracle rust=1031 ajv=1031, 0 disagreements** | `generated/audit-gate-before.txt` |

## 3. Authoring (measured compile, source content)

- **Method**: one codemod per plugin, `wp-d1/d1-<plugin>.py`, over `wp-d1/d1_apply.py`. Each adds one
  `.action_describe(id, LocalizedLabel::native(en, de))` step per verb (plus `.action_audience` / `.action_destructive`) before
  the chain's `.build_definition()`. It refuses duplicates, quotes, newlines and en == de, qualifies `LocalizedLabel` by the
  file's imports, and has an awaited-chain mode and a `builder = builder…;` statement mode. Every text was written after reading
  the verb's handler or reducer: what it does, to what, and with which consequence. Destructive and import/export verbs name
  exactly what is replaced, discarded or written.
- **Counts**: 771 descriptions in 55 editor/engine files + `listFlowExtensions` (`🌀️procedural/🎮️commands/🦀️.rs`,
  `CommandDefinition::describe`) + 7 framework tool-run verbs (`🛂️manifest` `tool_run_action_description`) + 3 SDK window-kit
  verbs (`🔌️plugin/🦀️.rs`: `replace-text`, `set-cell`, `set-node`).
  Per plugin: wfc 93, puzzle 133, norm 60 (15 standards × 4, standard-specific), space 59, fem 58, procedural 52 (+1 command),
  lowpoly 41, block 34, shooting 34, remodel 31, process 20, energy 19, architect 18, flow 18, trinity 17, sequence 14, animate 13,
  imperative 10, playbook 10, dag 9, writer 9, sourcing 7, mathematical 6, reasoning 4, demonstrator 2. The composed demonstrator
  apps inherit their upstream `create_*` definitions.
- **Compile-atomic**: `cargo check -p <crate>` right after each plugin (captures `generated/check-*.txt`), every one EXIT 0.
  There was one red (wfc grid2d/grid3d, my own missing import), fixed within 1 min (`check-wfc-1` 23:47 → `check-wfc-2` 23:48).
- **Source census** (`wp-d1/d1-source-census.py`, re-run 00:5x after peers' edits): 962 descriptor verbs + 14 SDK verbs
  covered. Missing only the frozen crates: 37 in gis/stdio/vcs, plus the demonstrator's gis composition (18).
- **Audience/destructive flags fixed in the SSOT** (the full list is in the per-plugin codemods, `generated/flags.txt`):
  - **destructive** (the verb replaces or discards document content, or writes a file):
    - whole-document imports/resets: architect importProgram/importRegistersCsv; block 2d/3d/5d edit; lowpoly
      importSnapshotJson/importMeshFile; shooting importSnapshotJson; process3d setStock/importModelFile; puzzle 2d/3d/5d
      importFixture; gen3d importDocument; rewriting setLhsJson/setRhsJson; writer setText/openDocument; mathematical setPoints;
      animate seedGrid/setSource;
    - example resets: dag/imperative/playbook setActiveExample; playground changeSchema;
    - layout overwrites: reorganize in flow/sequence/gen2d/gen3d/rewriting/puzzle2d, puzzle2d forceLayout, studio
      reorganizeWorkflow;
    - a download: animate copyPrompt;
    - a graph write: jack runQuery (Cypher CREATE/SET/DELETE);
    - an SDK text replace: `replace-text` (clears the 4 baseline audit findings).
  - **Input** (raw gesture phases): nodeGraphEdit (architect, dag, wfc2d/3d, sequence, flow, gen2d/3d, rewriting, studio);
    wfc bitmap stroke-begin/extend/commit; grid3d worldSelect; block3d worldSurfaceLeave/Place; flow spotlightCommit; lowpoly
    paint stroke phases; puzzle applyBoardEvents and the puzzle3d/5d engagement, brush and transform phases; writer textEdit.
  - **Chrome** (view state or feeds): camera, projection and viewport verbs (grid2d/3d, wfc2d/3d, energy editor+viewer, fem
    2d/3d, shooting, block3d, lowpoly, gen2d/3d + viewer, puzzle3d/5d, sequence, architect, dag, reasoning, mathematical, jack,
    rewriting, studio); animate noMutation; hub directory and presence feeds (home applyDirectoryEventPage, space index
    foldDirectoryEvents, presenceHeartbeat ×3).
- **wasm32 check**: not run here. No cfg(wasm32) code was touched (chain steps only); W2's describe-all compiles every guest
  for wasm32.

## 4. Capability audit and census (measured)

`semio-os-mcp` = this tree's binary (`wp-d1/target/debug/semio-os-mcp`, rebuilt 00:47 after the framework and SDK edits,
`generated/build-mcp-2.txt`). `audit --folder <root>`; the root is picked with `SEMIO_REPO_ROOT` (`find_repo_root` honours
it when `nx.json` is present). The projection (`wp-d1/d1-project.py [--with-frozen]`) copies the committed descriptors plus
registry, `nx.json` and manifest schema to `.🧬semio/🌐hub/s12-d1-projected[-with-frozen]`. It then writes the landed source
declarations (descriptions + audience + destructive) onto the matching descriptor verbs. The source is not built, so W2's
describe is still needed for the real descriptors.

| root | audit findings | description findings | Rust vs AJV (`wp-d1/d1-oracle.ts`) | `search::long` | capture |
|---|---|---|---|---|---|
| committed descriptors (restage4, unchanged) | 4 | 1031 plugin (+7 framework before the manifest edit) | 1031 = 1031, 0 disagreements | 2/4 (by design) | `audit-committed-2`, `oracle-audit-committed-2`, `test-long-before` |
| landed source projected | **0** | 55: gis 22, demonstrator's gis 18, stdio 9, vcs 6 (all frozen) | 55 = 55 | **3/4**, only the description law red (the same 55) | `audit-projected-1`, `oracle-audit-projected-1`, `test-long-projected` |
| landed + frozen patch set projected | **0** | **0** over 60 descriptors, exit 0 | 0 = 0 | **4/4 green** | `audit-projected-with-frozen`, `oracle-audit-projected-with-frozen`, `test-long-projected-frozen` |

Quick laws after all edits: `cargo test -p semio-framework-os-mcp --lib -- catalog:: search::quick` **23/23**
(`test-catalog-3`). No schema, untranslated, repeatsTitle or sharedWithinApp problem among the new texts: every remaining
finding is `missing` in a frozen crate. The intent-winner law (`the_installed_catalog_answers_three_intent_queries_with_a_clear_winner`)
stays green with the new texts.

**Frozen patch set** `wp-d1/d1-frozen.py` (NOT applied — session-12 ABI freeze on the stdio/gis/vcs codec crates):
- gis `create_gis2d_app` 10 descriptions + 8 Chrome audiences; gisterrain 2 + setCamera → Chrome;
- vcs 5 + noMutation → Chrome;
- stdio: contract fn `set_active_example_description()` plus one `.action_describe` after each of the 62 destructive
  `setActiveExample` sites.

Dry run clean at 00:44 and again at 00:53 (`frozen-dry-run[-2]`). It lands after W2's `--packages all` publish (`--apply`,
then `cargo check -p` for gis/stdio/vcs/demonstrator and describe those four).

## 5. `capabilities_search "delete"` before/after (measured, the real gateway)

Probe `wp-d1/d1-search-probe.py`: spawns `semio-os-mcp stdio --folder <scratch> --no-bridge` over a root, sends initialize →
`tools/call capabilities_search {"query":"delete","limit":100}` and keeps the structured result. Captures:
`generated/search-delete-{before,after,after-frozen}.{json,txt}` and `search-delete-summary.txt`.

| root | matches | distinct scores | largest tie | ties in top 15 | empty descriptions (all / top 15) | score range | stdev |
|---|---|---|---|---|---|---|---|
| committed (before) | 48 | 19 / 48 | 12 | 15 | 41 / 15 | 0.771 | 0.120 |
| landed source projected | 51 | 33 / 51 | 5 | 6 | 2 / 2 (gis) | 2.322 | 0.476 |
| + frozen patch set | 51 | 33 / 51 | 5 | 8 | **0 / 0** | 2.302 | 0.469 |

- **Before**: the top 10 all score 6.6949 with empty descriptions. Rank was decided by title length alone.
- **After**: every hit explains itself.
- **Remaining ties**: equal-length texts matching a single query term, a BM25 property rather than missing data. Examples are
  wfc2d/wfc3d, which share a verb in different apps, and "Deletes one target region/volume by id."
- **Intent queries** (`generated/search-intent-queries.txt`):
  - "delete a tile from the presentation deck" before: deleteTile, then exportVideoFromDeck (wrong).
  - After: deleteTile 28.8, deleteSelection 28.2, clearTiles 21.4.
  - "export the drawing as a file": the second hit was a bare puzzle exportFixture; it now carries its description.

## 6. Hand-offs

- **W2** (`wp-w1/requests/d1.txt`, filed 00:5x): describe-all of the `--packages all` publish (every guest links the touched
  framework and SDK), rebuild the staged `semio-os-mcp`, then the frozen patch set. **The live MCP sees none of the new
  descriptions before W2's restage.**
- **T12** (noted while reading handlers, not changed by D1):
  - architect `runAnalysis` is `View` but emits an artifact mutation;
  - reasoning `addRelationship` always wires `node-1 → node-2`;
  - flow `duplicateWidget` is a no-op handler (described honestly);
  - writer `open-document` has a peer's `[DEBUG]` `eprintln!`;
  - gen3d viewer `setActiveExample` is marked destructive although it only changes config (left as declared);
  - lowpoly keeps the transform gumball agent-facing while puzzle3d/5d classify transformBegin/End as Input. Each follows its
    handler's contract, but the two plugins disagree.

## 7. Files

- Law and gate:
  - `🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json` (`CapabilityDescription*`)
  - `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` (tool-run descriptions)
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (SDK window kits)
  - under `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/`:
    - `🗂️catalog/🦀️.rs`
    - `🗂️catalog/🟦️.ts`
    - `🗂️catalog/🧫️fixtures/💬️capability-description.json`
    - `🗂️catalog/🧪️tests/🔬️quick/🦀️.rs`
    - `🏗️bootstrap/🦀️.rs`
    - `🔎️search/🧪️tests/🔬️long/🦀️.rs`
    - `🧪️tests/💬️capability-description/🟦️.ts`
    - `📦️packages/🦀️rust/📜️script.ts`
    - `📦️packages/🦀️rust/📋️project.json`
    - `📦️packages/🟦️typescript/📋️project.json`
- Plugins: the 55 `create_*` files of `generated/touched-files.txt`, one row per codemod call in `wp-d1/d1-*.py`, plus
  `✏️s/🔌️plugins/🌀️procedural/🎮️commands/🦀️.rs`.
- Ticket-local: `wp-d1/` holds the codemods, `d1_apply.py`, the census, projection, oracle and search-probe tools, and
  `d1-frozen.py`. Durable data lives in `.🧬semio/🌐hub/s12-d1-projected` and `s12-d1-projected-with-frozen`.

## Log

Times are the capture mtimes (an earlier draft of this log carried estimated times that ran ahead of the clock; corrected).

- 23:09 started; read AGENTS.md, preambles 12/11, `audit-s12-ai-mcp.md`, `wp-g10.md` (item 11, S3), `wp-u5.md` §1.
- 23:12–23:17 pipeline traced (§1); census `wp-d1/d1-census.py`; audit baseline 4 findings with the staged binary.
- 23:17–23:38 law landed (§2): schema, Rust + TS twin, fixture, gate cross-check, long laws. All quick laws green; the census is
  red at 1038 by design.
- 23:39 framework: 7 tool-run verbs described in `🛂️manifest`, `cargo check -p semio-framework` EXIT 0.
- 23:41 SDK window kits: `replace-text` described + **destructive**, `set-cell`/`set-node` described,
  `cargo check -p semio-framework-plugin` EXIT 0.
- 23:45–00:01 **wfc** 93 (first check red at 23:47, my missing import; green at 23:48), **architect** 18, **animate** 13,
  **dag** 9, **reasoning** 4, **mathematical** 6 — flags in §3.
- 00:03–00:11 **imperative** 10, **playbook** 10, **sourcing** 7, **writer** 9, **sequence** 14, **trinity** 17, **energy** 19
  (property vocabularies read from the reducers), **flow** 18.
- 00:12–00:23 **process3d** 20, **remodel** 31, **shooting** 34, **block** 34, **fem** 58. First drafts that guessed a
  vocabulary were corrected against the reducers: shot shapes, block3d fields, fem3d loads.
- 00:24–00:31 **procedural** 52 + `listFlowExtensions`, **lowpoly** 41, **space** 59.
- 00:32–00:43 **puzzle** 133, **norm** 60, **demonstrator** 2. Every `cargo check -p` EXIT 0.
- 00:44 frozen patch set written, dry run clean. The source census shows only frozen verbs missing.
- 00:47–00:49 `semio-os-mcp` rebuilt; audits over committed / projected / projected + frozen; Rust = AJV at each (§4).
- 00:49–00:53 search before/after through the real gateway (§5); `search::long` over both projections (4/4 with frozen, 3/4
  without); quick laws 23/23; frozen dry run re-verified; oracle captures re-run.
- 00:5x W2 request `wp-w1/requests/d1.txt` filed. Coordinator told about the frozen patch set and the restage dependency.
