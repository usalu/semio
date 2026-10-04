# 📢️ S3-NOTICES — App-Declared, Localized Fault Notices (design §20.12)

Owner: S3-NOTICES (session 3). Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`. Scratch: `🗑️generated/s3-notices/`.

## Session 3 — 2026-10-03

### Status

- 10:45 resumed after the usage cut (no edit had been written before the cut — nothing to repair). Exploration done, design below
  fixed, implementation starting.

### Design as built (target)

1. **Structured placeholders on the fault wire.** `Fault.params: Vec<(String, String)>` (wire `params: {name: value}`, omitted when
   empty; duplicate names refused) — the ONLY source of a notice's `{name}` placeholders. Never parsed from `message`.
2. **App declaration.** `ArtifactApp::fault_notices() -> &'static [(&'static str, LocalizedLabel)]` (default empty), forwarded by
   `ArtifactEditor`/`ArtifactViewer`; every registration path (`document_app`, `viewer`, `editor_app`, `editor_surface`,
   `viewer_surface`) stamps the table into `AppDefinition.fault_notices` and validates it.
3. **Schema-first descriptor.** `AppDefinition.faultNotices: [{code, label}]` (manifest schema `$defs.FaultNoticeDefinition`,
   code pattern `<app>.<area>.<name>`, label = complete locale × terminology matrix, placeholders `{name}`).
4. **Validation (Rust + TS twin, one language-agnostic fixture, ajv + i18next oracles).** Code syntax, uniqueness, every locale cell
   non-empty, the same placeholder set in every cell.
5. **Resolution (both shells).** A refused dispatch's `Fault.code` (then its causes) resolves against the framework table first, then
   the active app's `faultNotices`; the localized text fills `{name}` from `Fault.params`; shown as the transient notice
   (role=status). A code without a label is never shown raw (the generic refusal notice stays).
6. **Gate.** Static scan of every guest `FaultCode::new("…")`/named code literal under each plugin tree vs the app tables; planted
   violations prove it fails.

### Changes (as of 11:25)

| Area | File | Change |
|---|---|---|
| Fault wire | `🧰️framework/🔨️modules/⚠️diagnostic/🦀️.rs` | `FaultParams(Vec<(String,String)>)` + `get`, `is_fault_param_name`, `Fault.params: Option<Box<FaultParams>>` (inline size stays 112 B, the fixture budget), `Fault::with_param`/`param`, `FaultFrom::fault_params`, plain codec (`params: {name: value}`, omitted when empty) |
| | `⚠️diagnostic/🎛️controlled/🦀️.rs`, `🧬️retirement/🦀️.rs` | bounded `encode_params`/`decode_params` (name grammar, duplicates, string values, exact admission), Fault field 9, retirement |
| | `⚠️diagnostic/🧬️schema/{🎛️controlled,🧯️fault}/🔣️.json`, `🧫️fixtures/{🎛️controlled,🧯️fault}/🔣️.json` | `$defs.FaultParams`, `Fault.params`; 7 controlled cases + 1 duplicate case, 2 serde cases |
| | `⚠️diagnostic/🧪️tests/{🎛️controlled,🔬️fault-describe}/🦀️.rs` | FaultParams owner in the law + duplicate refusal, params round trip test |
| | `🎠️kernel/🟦️.ts` | TS `Fault.params` |
| Descriptor | `🛂️manifest/🦀️.rs` region `🔖️FaultNotices` | `FaultNoticeDefinition`, `FaultNoticeError`, `is_fault_notice_code`, `fault_notice_placeholders`, `validate_fault_notices`, `fault_notice_text`, `fill_fault_notice`, `fault_notice_definitions`; `AppDefinition.fault_notices` (serde/value default) |
| | `🛂️manifest/🧬️schema/🔣️.json` | `$defs.FaultNoticeCode/Text/Locales/Definition/Table` |
| | `🧬️schema/📽️projection/🦀️.rs` + `🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts` (hand-synced, gitignored) | `FaultNoticeDefinition` v1, `AppDefinition` v2 `faultNotices` |
| | 22 `AppDefinition { .. }` literals (platform, manifest app-label, mcp source-builders ×4, Shell tests ×4, Dock tests ×2, plugin host app-router, plugin `try_build_definition`, os host + host-unit ×5 + registry-unit) | `fault_notices: Vec::new()` (script `🗑️generated/s3-notices/appdef-sites.py`) |
| | `🧰️framework/📦️packages/🦀️rust/🦀️.rs` | re-export `FaultParams`, `is_fault_param_name` |
| | `🛂️manifest/🟦️.ts` region `🔖️FaultNotices` | TS twins `isFaultNoticeCode`, `faultNoticePlaceholders`, `validateFaultNotices`, `fillFaultNotice`, `faultNoticeText`; `AppDefinition.faultNotices` typed |
| Resolver | `🎠️kernel/🦀️.rs` region `🔖️FaultNotices` | `FaultNotice{code,text}`, `fault_notice(fault, notices, terminology, locale)`: code then causes; framework table (`HISTORY_NOTICE_LABELS`) first, app table second; params only |
| | `🎠️kernel/🟦️.ts` | TS twin `faultNotice` |
| Runtime | `🔌️plugin/🦀️.rs` | `ArtifactApp::fault_notices()` (default `&[]`), `ArtifactEditor`/`ArtifactViewer` defaults + `EditorApp`/`ViewerApp` forwards, `declarations::stamp_fault_notices::<A>` (stamps + validates, panics on an invalid table like `build_definition`) in `editor_surface`/`viewer_surface` |
| | `🔌️plugin/🏗️builder/🦀️.rs` | stamping in `document_app`, `viewer`, `editor_app` |
| React | `🛠️ShellHelpers/🟦️.tsx` | `appFaultNoticeV1(fault, app, terminology)` |
| | `🏛️ShellHost/🟦️.tsx` | app notice after the history notices at the three refusal sites (direct browser actor, `handleAction`, `handleCommand`); generic refusal otherwise |
| Corpus | `🛂️manifest/🧫️fixtures/🧫️fault-notices/🔣️.json` + `🧬️schema/🔣️.json` | 13 tables (5 rules, `schemaValid` verdict), 10 resolutions |
| Tests | `🛂️manifest/🧪️tests/🧪️fault-notices/{🦀️.rs,🟦️.ts}`, `🎠️kernel/🧪️tests/🧪️fault-notices/🦀️.rs`, `🛠️ShellHelpers/🧪️tests/🧪️fault-notices/🟦️.ts` (+ registered in `⚛️react/🧪️tests/🎚️config/🟦️.ts`) | Rust + TS twins on one corpus; oracles Ajv (schema verdict, Fault schema) + i18next (`{name}` interpolation) |
| Ticket input | `🧪️s3-notices-typecheck.tsconfig.json` | tsc over my TS files |

### Verification (11:25)

| Command | Result |
|---|---|
| `cargo test -p semio-framework-diagnostic --lib` (private target) | **11/11 pass** (incl. Ajv oracle via bun, inline-size budget 112 B) |
| `bun test ./🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️fault-notices/🟦️.ts` | **5/5 pass** (50 expects) |
| `SEMIO_TEST_LEVEL=long bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts 🧪️fault-notices 🧪️command-rejection` (cwd `⚛️react/📦️packages/🟦️typescript`) | **6/6 pass** (2 files) |
| `bunx tsc -p 🧪️s3-notices-typecheck.tsconfig.json` | 0 errors in my files; 1 peer error `🌐️World3dHost/🟦️.tsx:2226` TS2345 |
| `bun test ./🧰️framework/🔨️modules/🗣️dsl/🧪️tests/🧱️ownership/🟦️.ts` (fault fixture consumer) | 5/6 — the 1 failure is a peer dependency change (`semio-framework-async` added to the dsl crate), not the fault fixture |
| `cargo check -p semio-framework-plugin --lib` | run 1 SIGKILL (137); run 2 **blocked by peer**: os-kernel `🏪️store/🦀️.rs:17806/17823` E0425 `shared_prefix_len`, `:22084` E0061/E0308 (reported to coordinator) — manifest/kernel/plugin Rust changes WRITTEN BUT UNVERIFIED |

### Changes after 11:25

| Area | File | Change |
|---|---|---|
| History-full count | `🌿️vcs/🦀️.rs` (os-kernel) | `VcsError: FaultFrom::fault_params` — `history.full` carries `n = capacity`; law `🌿️vcs/🧪️tests/🧪️fault-params/🦀️.rs` |
| wgpu bridge | `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` | `ProgramFault { fault: Option<Fault>, text }` (+ `From<String>`/`From<&str>`/`Into<String>`/`Display`); `handle_action`/`handle_command` (bridge, native exchange, JS) answer it; native `app_frame_program_fault` decodes the guest fault, JS `js_program_fault` decodes `SemioFaultError.fault`; fixture actions stay `String` (mapped) |
| | `🌉️ProgramBridge/🧪️tests/🕹️wgpu-reserved-verb-answer/🦀️.rs` | assert `error.text` |
| wgpu shell | `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | `RefusedGuestFault {fault, notices}` slot in the chrome build state (set by `refused_guest_call` at the two guest call sites of `dispatch_action`, cleared at each dispatch start, taken by `note_dispatch_fault` only when the funnel string names the same code); `classify_dispatch_fault_notice(error, refused, terminology, locale) -> (String, Severity, Option<String>)`: viewer read-only → hub history refusal → **`kernel::fault_notice`** (framework history-lane as warning with `{n}` from params; app notices with the fault's severity) → `app.command.rejected` → a structured guest refusal without a notice is React's localized `dispatch-failed` ("The input could not be delivered.", info, no code) → other shell error text. Branch 3 (text-parsed history lane, `{n}` from message digits) **replaced** as agreed with S3-W2C |
| | `🐚️Shell/🎯️targets/🧊️wgpu/⏪️time-travel/🦀️.rs` | deleted `history_lane_notice_of_fault` (dead; read `{n}` from text) |
| | `🐚️Shell/🧪️tests/{🔬️wgpu-agent-overlays,🧪️wgpu-time-travel}/🦀️.rs` | adapted to the new classifier; the history-lane law now feeds structured faults with `n` and asserts no count is read from a message |
| | `🐚️Shell/🧪️tests/🧪️wgpu-fault-notices/🦀️.rs` (new, mounted after agent overlays) | corpus law in both severities + funnel/ARIA-mirror law (role `status`, label = text, description = code; mismatched recorded refusal ignored) |
| Gate | `🧪️test/🧬️schema/📋️orchestration/🟦️.ts` region `📢️FaultNotices` | `schema fault-notices` (code `schema-fault-notice`, folded into `test schema`): `rustFaultCodeSites`, `rustFaultNoticeDeclarations` (forwards resolved via `rustFaultNoticeTable`), `faultNoticeReport`, `runFaultNotices` |
| | `🧪️test/🟦️.ts`, `🧰️framework/🔨️modules/🧪️test/🧬️schema/🔣️.json` | code registered (table + protocol enum) |
| | `🧪️test/📋️project.json`, `🧪️test/📜️script.ts`, `.vscode/launch.json` | targets `test-schema-fault-notices`, `test-schema-fault-notices-census`, `test-fault-notices-gate`; 3 launch rows (orders 900.04775, 900.04901, 900.04902) |
| | `🧪️test/🧫️fixtures/🧫️fault-notices-gate/🔣️.json`, `🧪️test/🧪️tests/🧪️fault-notices-gate/🟦️.ts` | 5 snippet cases (gate == tree-sitter-rust oracle) + one planted plugin (sources, descriptor, framework table in a scratch repo) with 11 exact findings and its census row |
| Schema | `🛂️manifest/🧬️schema/🔣️.json` | `$defs.FaultNoticeCorpus` (the corpus' own schema moved here; the fixture-local `🧬️schema` dirs I had created were deleted again because the test domain refuses schema definitions inside `🧫️*` trees and non-draft-07 dialects) |

Deleted (created by me this session, rule 32): `🛂️manifest/🧫️fixtures/🧫️fault-notices/🧬️schema/🔣️.json`, `🧪️test/🧫️fixtures/🧫️fault-notices-gate/🧬️schema/🔣️.json` (+ their empty dirs).

### Verification (12:40)

| Command | Result |
|---|---|
| `cargo check -p semio-framework-plugin --lib` | **green** (after the peer store fix; diagnostic → os-kernel → framework → plugin rebuilt) |
| `cargo test -p semio-framework --lib fault_notice` (private target) | **4/4 pass** (manifest ×3, kernel ×1) |
| `cargo test -p semio-framework --lib --features typegen exports_typescript_bindings` | **1/1 pass** (hand-synced `🤖️generated/🪪️manifest/🟦️.ts` == projection render) |
| `cargo test -p semio-framework-os-kernel --lib fault_params` | **1/1 pass** |
| `cargo check -p semio-framework-os-renderer-wgpu --lib` (native) | **green** |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown` | **blocked by peer**: `🔌️plugin/⏯️tool-run/🦀️.rs:1895`, `🔌️plugin/⏪️time-travel/🦀️.rs:2072` E0308 `record_command` arg (reported); my JS-path code (`js_program_fault`) WRITTEN BUT UNVERIFIED on wasm32 |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- fault_notice history_lane_refusal dispatch_faults_are_classified history_edit_refusal a_failed_dispatch uncorrelated_answer` | **6/6 pass**; a wider rerun of the time-travel/overlays/bridge suites was then blocked by a peer mid-edit in `semio-framework-graph` (`PropertyBag` E0308/E0599) |
| `bun test ./…/🧪️test/🧪️tests/🧪️fault-notices-gate/🟦️.ts` (also `bun ./📜️script.ts test fault-notices-gate`) | **2/2 pass** |
| `bun test ./…/🧪️test/🧪️tests/🧪️mutation-history-gates/🟦️.ts` (shared orchestration file) | **39/39 pass** |
| `bun test ./…/🧪️test/🧪️tests/🧬️schema-invariants/🟦️.ts` | 119/122; the 3 failures are pre-existing peers' (`🌎️hub/💡️inference` fixture schema, submodule walk, test-domain schema files); none names a file of mine after the schema move |
| `bun test ./🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️fault-notices/🟦️.ts` | **5/5 pass** (corpus now checked against `$defs.FaultNoticeCorpus`) |
| `bun ./📜️script.ts schema fault-notices` (repo-wide, cwd `🧪️test`) | **exit 1 (expected)** — see the gate counts below |

### Gate counts (12:40, `bun ./📜️script.ts schema fault-notices --census`)

**12 labelled of 420 distinct guest fault codes, 12 declared notices, 1778 anonymous faults, 2187 `schema-fault-notice` findings** —
faultAnonymous 1778, faultNoticeMissing 258, faultNoticeSyntax 115 (codes with < 3 segments or non-kebab segments), faultNoticeFramework 35,
faultNoticeDescriptor 1 (procedural: describe owed for the 12 generation3d notices). The 12 labelled are generation3d's gumball codes
(S3-PROCEDURAL adopted the API: `gumball_fault_notices()` forwarded from the editor's `fault_notices`, `{kind}` via `with_param`).

### Routed codes (per plugin; to the coordinator)

| Plugin | anonymous `Fault::from(text)` | missing (app code, no notice) | syntax (< 3 segments) | framework namespace | other |
|---|---|---|---|---|---|
| 🗄️stdio | 288 | 111: `stdio.bcf.column-stale`, `stdio.bcf.row-stale`, `stdio.bcf.table-conflict`, `stdio.binary.invalid-hex`, `stdio.binary.unhandled-action`, `stdio.csv.cell-stale` … | 5: `stdio.ifc.2x3.cobie.validate-decode-failed`, `stdio.ifc.2x3.cv20.validate-decode-failed`, `stdio.ifc.2x3.sav.validate-decode-failed`, `stdio.semio_audio.validate-decode-failed`, `stdio.semio_video.pts-non-monotonic` | 1: `app.command.unsupported` | — |
| 🧩️puzzle | 313 | 3: `puzzle2d.action.flag-value-required`, `puzzle3d.action.flag-value-required`, `puzzle5d.action.flag-value-required` | 0 | 0 | — |
| 🏗️fem | 182 | 0 | 0 | 0 | — |
| 🖍️draw | 111 | 18: `drawing.bounded.tool-mismatch`, `drawing.canvas.window-required`, `drawing.example.parse`, `drawing.example.unknown`, `drawing.export.format`, `drawing.export.pdf` … | 2: `artifact-store.initializer-close`, `drawing.child-projection` | 2: `app.command.invalid-args`, `app.command.unsupported` | — |
| 🌀️procedural | 118 | 3: `generation3d.io.export`, `generation3d.io.import-accept`, `generation3d.widget.add` | 2: `generation2d.child-projection`, `generation3d.child-projection` | 2: `mutation.target-mismatch`, `mutation.target-missing` | faultNoticeDescriptor: describe owed — unpublished ["generation3d.gumball.component-selection","generation3d.gumball.host-edit","generation3d.g |
| 🌊️flow | 102 | 7: `extension.evaluate-cancel.bad-request`, `extension.evaluate.bad-request`, `extension.tessellate-cancel.bad-request`, `extension.tessellate.bad-request`, `flow.add-widget.child-delta-invalid`, `flow.retained.legacy-dispatch` … | 11: `extension.brep-close`, `extension.unknown-capability`, `flow.child-projection`, `flow.connect-incompatible`, `flow.content-dialect`, `flow.content-unavailable` … | 1: `mutation.target-missing` | — |
| 🖨️raster | 87 | 0 | 9: `artifact-store.initializer-close`, `raster-fill-invalid`, `raster-filter-invalid`, `raster-filter-pixels-only`, `raster-transform-invalid`, `raster-transform-pixels-only` … | 2: `app.command.invalid-args`, `app.command.unsupported` | — |
| 🎬️sequence | 85 | 3: `sequence.retained.config-command`, `sequence.retained.example-command`, `sequence.retained.tool-mismatch` | 1: `sequence.child-projection` | 0 | — |
| 🀄️wfc | 61 | 18: `wfc2d.edge.unknown-edge`, `wfc2d.example.unknown`, `wfc2d.id.taken`, `wfc2d.node-graph.row`, `wfc2d.retained.extent`, `wfc2d.retained.route` … | 0 | 2: `app.command.invalid`, `app.command.unsupported` | — |
| 🪐️space | 15 | 47: `s.home.apply-local-catalog-document.catalog-refused`, `s.home.apply-local-catalog-document.encoding-invalid`, `s.home.apply-local-catalog-document.id-invalid`, `s.home.apply-local-catalog-document.mismatch`, `s.home.apply-local-catalog-document.not-a-studio`, `s.home.apply-local-catalog-document.requires-retained-job` … | 0 | 0 | — |
| 📋️forms | 50 | 2: `forms.try-values.input-id-required`, `forms.try-values.window-stale` | 1: `forms.child-projection` | 2: `app.command.invalid-args`, `app.command.unsupported` | — |
| 🔱️trinity | 31 | 2: `trinity.rewriting.node-graph.row`, `trinity.rewriting.rule-undecodable` | 13: `artifact-store.initializer-close`, `jack.child-projection`, `jack.graph-window-required`, `jack.lod`, `jack.query-too-large`, `jack.results-window-required` … | 4: `app.command.invalid-args`, `app.command.targets-required`, `app.command.unsupported`, `mutation.target-missing` | — |
| ➗️mathematical | 45 | 0 | 2: `equation.child-projection`, `equation.graph-window-required` | 0 | — |
| 🌍️gis | 21 | 4: `gis.map.viewer.retained-route-required`, `gis.map.viewer.retained.tool-mismatch`, `gis.map.viewer.window-kind`, `gis.map.viewer.window-required` | 14: `approval.commit-unavailable`, `artifact-store.initializer-close`, `inference.bounds`, `inference.cancelled`, `inference.capacity`, `inference.conflict` … | 2: `app.command.invalid-payload`, `app.command.unsupported` | — |
| 🏛️architect | 17 | 0 | 14: `architect.adjacency-field-missing`, `architect.adjacency-kind-unknown`, `architect.adjacency-self`, `architect.adjacency-value-invalid`, `architect.analysis-kind-unknown`, `architect.child-projection` … | 1: `mutation.target-missing` | — |
| 🌿️vcs | 29 | 0 | 0 | 0 | — |
| 🗒️note | 14 | 10: `note.example.unknown`, `note.ink-events.invalid`, `note.ink-gesture.invalid`, `note.ink-phase.invalid`, `note.ink-tool.provisional`, `note.retained.extent` … | 1: `note.child-projection` | 3: `app.command.invalid-args`, `app.command.unsupported`, `mutation.target-missing` | — |
| 💡️reasoning | 23 | 0 | 4: `wires.canvas-window-required`, `wires.child-projection`, `wires.delete-selection-empty`, `wires.unhandled-action` | 0 | — |
| 📐️cad | 21 | 2: `cad.action.flag-value-required`, `cad.export.empty-pane` | 3: `cad.apply-transformation-unavailable`, `cad.import-object-unavailable`, `cad.import-unreadable` | 0 | — |
| 📕️norm | 9 | 0 | 16: `norm.apply-remedy-empty-path`, `norm.apply-remedy-int`, `norm.apply-remedy-missing-check`, `norm.apply-remedy-missing-option`, `norm.apply-remedy-missing-remedy`, `norm.apply-remedy-not-applicable` … | 0 | — |
| 📏️layout | 23 | 0 | 0 | 2: `app.command.invalid-args`, `app.command.unsupported` | — |
| 💠️lowpoly | 23 | 0 | 0 | 2: `app.command.invalid-args`, `app.command.unsupported` | — |
| 🏭️process | 21 | 2: `process3d.action.invalid`, `process3d.media.export` | 1: `artifact-store.initializer-close` | 0 | — |
| 📸️remodel | 13 | 8: `remodeling.import.open`, `remodeling.import.window-required`, `remodeling.qc-report.encode`, `remodeling.qc-report.missing`, `remodeling.retained.extent`, `remodeling.retained.route` … | 0 | 1: `app.command.unsupported` | — |
| ✒️writer | 16 | 0 | 3: `artifact-store.initializer-close`, `writer.child-projection`, `writer.main-window-required` | 0 | — |
| 🕸️dag | 9 | 5: `dag.move-media-node.malformed`, `dag.node-graph-edit.malformed`, `dag.node-graph-edit.unsupported`, `dag.patch-dag-nodes.malformed`, `dag.rename-dag-node.malformed` | 2: `dag.child-projection`, `dag.unhandled-action` | 0 | — |
| 📖️playbook | 14 | 1: `playbook.module.procedural.tool-mismatch` | 1: `playbook.unhandled-action` | 0 | — |
| 🎞️animate | 9 | 5: `animate.video.export.no-scenes`, `animate.video.export.program`, `animate.video.export.program-too-large`, `animate.video.export.scene-json`, `animate.video.export.source-kind` | 1: `animate.child-projection` | 0 | — |
| 🔋️energy | 1 | 3: `energy.model.retained.extent`, `energy.model.retained.tool-mismatch`, `energy.model.viewer.retained.tool-mismatch` | 5: `energy.model.3d.viewer.retained-route-required`, `energy.model.3d.viewer.window-kind`, `energy.model.3d.viewer.window-required`, `energy.model.3d.window-kind`, `energy.model.3d.window-required` | 5: `app.command.invalid-payload`, `app.command.kind-unavailable`, `app.command.target-in-use`, `app.command.unsupported`, `mutation.target-missing` | — |
| 🪵️sourcing | 10 | 0 | 0 | 1: `app.command.unsupported` | — |
| 🧱️block | 10 | 0 | 1: `block3d.unhandled-action` | 0 | — |
| 🎥️shooting | 0 | 4: `shooting.export.nothing-to-export`, `shooting.retained.extent`, `shooting.retained.route`, `shooting.retained.tool-mismatch` | 0 | 2: `app.command.invalid-args`, `app.command.unsupported` | — |
| 📜️imperative | 2 | 0 | 3: `extension.evaluate`, `imperative.unhandled-action`, `procedure.child-projection` | 0 | — |
| 🎪️demonstrator | 5 | 0 | 0 | 0 | — |

### Design notes (as built)

- **Why `Option<Box<FaultParams>>`:** `Fault` has a language-neutral inline budget of 112 B (`🧯️fault` fixture, law
  `fault_inline_layout_stays_within_the_language_neutral_budget`); a plain `Vec` field would make it 128 B. Params are rare, so they
  own a separate allocation like `scope`; wire `params: {name: value}` is omitted when empty and `{}` decodes to `None`.
- **Declaration shape:** `fn fault_notices() -> &'static [(&'static str, LocalizedLabel)]` (the decided signature) is written with a
  `static … LazyLock<[(&str, LocalizedLabel); N]>` of `LocalizedLabel::native(en, de)` rows — the exhaustive locale match makes a new
  locale a compile error at every table. A body may forward to a free function (generation3d does); the gate resolves the forward.
- **Resolution order (both shells):** framework table first (today `HISTORY_NOTICE_LABELS`), then the refusing app's
  `faultNotices`; the fault's own code first, then its causes; the first declared code decides; a placeholder without a param means
  no notice (never half-filled) → the generic localized refusal. History-lane notices stay warnings (React parity).
- **wgpu structured path:** the shell's dispatch error channel stays `String` (51 call sites); the structured guest fault rides a
  one-slot side record (`RefusedGuestFault`) set where a program call refuses and consumed by the funnel only when the funnel string
  names the same code. A structured guest refusal without a notice now shows React's localized "The input could not be delivered."
  instead of the raw `code: message` (unstructured shell errors keep W2C's behaviour).
- **Gate heuristics (static):** guest sources = non-test `.rs` under `✏️s/🔌️plugins`; codes = string literals of
  `FaultCode::new|from`, `Fault::new(_, "…")`, `fault_from_error!(…, "…")` and dotted literals in any `fn code`/`fn fault_code`;
  anonymous = `Fault::from(<literal | format! | ….to_string()/to_owned()>)` (a typed `Fault::from(refusal)` is not); framework
  namespaces `app plugin os framework module mutation viewer surface history timeTravel toolTransaction document pure`. Dynamic codes
  (`FaultCode::new(variable)`) are only seen through their `fn code`. Framework-runtime codes raised on a guest's behalf (plugin
  runtime) are outside this gate.

### Coordinator actions

1. `describe` the procedural composition (generation3d editor `faultNotices`, 12 rows) — and every plugin after it declares notices
   (gate class `faultNoticeDescriptor` names them).
2. Decide the home of notices for guest-raised framework codes (`app.command.*`, `mutation.target-*`): a framework table beside
   `HISTORY_NOTICE_LABELS` (35 findings).
3. Route the per-plugin debt above to the plugin owners (anonymous faults → named codes + notices; < 3-segment codes → rename).
4. Regenerate `.vscode/launch.json` (3 hand-added rows) next time the generator runs.
5. Register `i18next` (test oracle of `🛂️manifest/🧪️tests/🧪️fault-notices`) with the dependency gate.
6. Re-run the wgpu wasm32 check once `record_command`'s wasm-gated callers are fixed (peer), and the wider wgpu suites once
   `semio-framework-graph` is green.

## Session 4 — 2026-10-04

Continued by S4-GATES in `📓️s4-gates-report.md` (one report for the four inherited WPs, rule 34).
