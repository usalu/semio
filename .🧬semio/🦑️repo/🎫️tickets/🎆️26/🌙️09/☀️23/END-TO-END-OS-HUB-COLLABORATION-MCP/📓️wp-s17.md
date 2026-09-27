# WP-S17 — Plugin Extensions (26) and Plugins Without Live Evidence

Session 13 slice S17 (2026-09-26, Opus 5.5). Ports 8190–8199 / 6690–6699. Scripts `wp-s17/`, expendable captures
`wp-s17/generated/`, binaries `CARGO_TARGET_DIR=wp-s17/target`, durable logs `.🧬semio/🌐hub/s13-s17-logs/`.
Checklist: [📓️audit-s13-plugins.md](📓️audit-s13-plugins.md) (extension rows 35–60).

## Session 13

| # | Item | Status |
|---|---|---|
| 1 | Per-extension native evidence (lib tests, test quick, exact pin, descriptor en+de, apps open) | 26/26 compile natively (`--lib --tests`, 05:29); exact pins in source (descriptors regenerate in the rebuild); lib tests + test quick after the rebuild (rules 25/26); finding F1: 16/26 extensions duplicate parent-built content |
| 1a | (coord 20:0x) W2's exact-pin narrowing `VersionReq` → `VersionPin` (SDK, 26 ext + demonstrator, stdio, hub, `.sxt`, TS twin, laws) + brep/math version drift + 3 framework action descriptions | **LANDED**: applied 20:13:58 (`s17-version-pin.py`, 59 files); native phase A (framework, kernel, SDK, procedural, flow artifact, 26 ext; `--lib --tests`) EXIT 0 05:29; demonstrator/stdio/hub native running; wasm32 = REBUILD components step (coordinator); TS gates green |
| 1b | (coord 20:0x) English-only extension topic strings (+ parents' static catalogues): schema-first en+de | census 1 231 rows / 1 040 distinct; de tables complete (230 + 528); design below; not landable before REBUILD START → prepared set |
| 2 | playbook/procedural red + flow extension chain (procedural + demonstrator, flow's extensions) | IN PROGRESS — red = stale committed descriptor only; roster version drift + triplicated `describe` fixed in 1a |
| 3 | Live in `s` after W3's restage (load with parent, apps/kinds open, edit/↶/↷, en+de); hub creation after publish | BLOCKED on W3 restage |

### Per-extension table

Committed descriptors (`🔣️.json`, census `wp-s17/s17-extension-census.py`, 19:5x, before the pin landing):

| parent | extensions | apps / actions | topic | pin (committed) | own version | en+de gaps |
|---|---|---|---|---|---|---|
| 🌊️flow | bim, list, brep, dictionary, text, primitive, draw, logic, math | 0 / 0 | `flow.extension` ×2 (flow-play, procedural3d-play) | `flow *` | 0.1.0; brep **0.3.0**, math **0.2.0** (drift) | 0 manifest gaps; 692 English-only payload strings |
| 🏭️process | metal, robotic, concrete, wood | 0 / 0 | `process.machines` | `process *` | 0.1.0 | 130 English-only payload strings |
| 📐️cad | aec-building, aec-building-structure, aec-building-energy, spatial-shape | 0 / 0 | `cad.computer` | `cad *` (aec-building `^0.1.0`) | 0.1.0 | 4 |
| 📜️imperative | control, text, effect, logic, math | 0 / 0 | `imperative.module` | `imperative *` | 0.1.0 | 81 |
| 🪵️sourcing | slabs, windows, beams | 0 / 0 | `sourcing.module` | `sourcing *` | 0.1.0 | 25 |
| 📖️playbook | procedural | 1 app `s.playbook.procedural@1/*#editor` / 15 actions | `playbook.blockKind` | `playbook ^0.1.0` | 0.1.0 | 15 action descriptions missing in the committed descriptor (source declares the 2 own ones; stale descriptor), 1 payload label |

Every extension depends on its parent first (`extends == dependencies[0]`), 0 of 26 exact before 1a.

### 1b — English-only extension payload strings (measured census, design)

Census `wp-s17/s17-i18n-census.py` over the committed `🔣️.json` topic payloads (JSON-in-JSON walked; keys
`label/name/summary/title/description/abbreviation`), capture `wp-s17/generated/i18n-census-1.json` (1 231 rows, 1 040
distinct strings, each with its authoring source line where it is a literal): **0 German strings in any extension payload.**

| family (topic) | where the strings live | strings |
|---|---|---|
| flow (`flow.extension`) | neural `OperatorInfo` name/abbreviation/summary (181/180/188), operator port names/abbreviations (322/275, partly identifiers), schema name/summary (23/23), command titles 5, setting description 1, extension labels 18 — authored in the extension crates, typed by `🧠️neural/⚙️engine` `OperatorInfo` + `🌊️flow/🧩️extensions/🕸️wasm` manifest types | 964 |
| process (`process.machines`) | `MachineCatalog::label`, `WorkshopMachine.label`, `Capability.label`, `CapabilityParameter.label` — the process3d artifact's own model; a machine inserted into a document becomes user data (`RenameMachine`, snapshot/binary codecs) | 130 |
| imperative (`imperative.module`) | catalogue sections/items (title/name/abbreviation/summary) + operator infos via `✏️s/🔨️modules/📜️imperative/🧩️extension_sdk` | 107 |
| sourcing (`sourcing.module`) | typology labels + kind names owned by `semio_s_artifact_sourcing_curation` (the extension only wraps parent data) | 25 |
| cad (`cad.computer`) | computer labels | 4 |
| playbook (`playbook.blockKind`) | "Building Component" | 1 |

The parents' own built-in catalogues are English-only too (e.g. flow `static_catalogue_sections`: "Inputs", "Slider",
"Number input" …), so this is a domain-model i18n defect of flow/neural, imperative, process3d, sourcing and cad, with the
extensions as thin carriers — not an extension-local fix. Design (schema-first, next cycle): one catalogue text type =
the manifest's `LocalizedLabel` (`native`/`reuse` × `en`/`de`, already the SDK's compile-checked label type) for every
catalogue display field (`OperatorInfo.{name,abbreviation,summary}`, port labels, schema name/summary, command title,
setting description, machine/capability/parameter labels, typology/kind names, section titles, topic `label`); consumers
resolve with the view's `Locale` at render (`resolve_labels` path); where a catalogue entry is COPIED into a document
(process machine, sourcing kind) the copy is resolved once at insertion in the inserting user's locale and stays user data
(no document-codec change). Laws: per topic a JSON Schema (`🧬️schema`) requiring `en`+`de` non-empty on every display field,
checked by an AJV twin (third-party oracle) + a Rust census law per parent over its own catalogue and every contributed
payload. Not landable before REBUILD START (touches the neural engine and 5 guest parents + ~1 040 translations; LC is
landing flow's 23-file P8 set now) → prepared as a patch set + en→de translation table during the rebuild (below).

### 1b + F1 — prepared set (next landing window; design, inputs ready, codemod not yet written)

Order (each step compile-atomic native + wasm32, laws first red then green):

1. **Locale type below the neural engine.** `LocalizedLabel`/`Locale`/`Terminology` live in the os-kernel (`💻️os/🔨️modules/🌐️locale`,
   `#[path]`-mounted); `neural_engine` (`🧠️neural/⚙️engine`) depends only on replication + pack. Split `🌐️locale` into its own
   leaf crate (`semio-framework-locale`, generated axes stay generated), the kernel re-exports it, neural depends on it — no
   cycle (the kernel does not depend on neural; measured in both `Cargo.toml`s).
2. **Schema-first catalogue text.** Every catalogue display field becomes the manifest's `LocalizedLabel` wire shape
   (`native`/`reuse` × `en`/`de`): neural `OperatorInfo.{name, abbreviation, summary}` + port `ChannelSpec` display name, `Schema.{name,
   summary}`, flow `FlowExtensionManifest.name`, `FlowExtensionWidget.{name, summary}`, `FlowExtensionCommand.title`,
   `FlowExtensionSetting.description`, flow `CatalogueSection.title` / `CatalogueItem.{name, abbreviation, summary}` (incl. the
   parent's `static_catalogue_sections`), imperative extension SDK catalogue `sections[].title` / `items[].{name, abbreviation,
   summary}`, process `MachineCatalog::label` + a catalogue-side `CatalogMachine { label: LocalizedLabel, … }` (the document's
   `WorkshopMachine.label: String` stays user data: `catalog_machine` resolves the catalogue label ONCE in the inserting view's
   locale), sourcing typology labels + kind names (same insertion rule), cad computer labels, playbook `blockKind.label`, and
   every topic payload's `label`. Identifiers (port ids, scope keys, 3-letter port-derived abbreviations: 110 flow + 16 others,
   `null` in the tables) stay `String`.
3. **Resolve at render, not at contribution:** consumers resolve with the window's `Locale`/`Terminology` (`resolve_labels`/
   `LocalizedLabel::resolve`), so switching en ↔ de re-renders palettes without re-pushing contributions.
4. **F1 dedup in the same step per family:** process3d keeps only `GenericCatalog`; sourcing keeps only `reuse`; imperative's
   `procedure` artifact drops `pub mod extensions` (`#[path]` of the 5 extension sources) and its native default contributions;
   cad's TS runtime drops `shippedCadComputerContributionsJson`. Each parent `depends-on` its extensions (as procedural does for
   the flow extensions) so they load with it; the parent's tests link the extension crates as dev-dependencies (as generation3d
   does for brep/math).
5. **Texts:** `wp-s17/s17-i18n-de-parents.json` (230) + `wp-s17/s17-i18n-de-flow.json` (528) — the codemod rewrites each
   authoring literal (source lines are in `generated/i18n-census-1.json`; 317 rows are built by `format!`/helpers, e.g. math's
   `format!("{name} with x, y, z decimal fields")`, and need their helper taking a `LocalizedLabel` pair).
6. **Laws:** one JSON Schema per topic payload (`🧬️schema`) requiring non-empty `en` + `de` on every display field, evaluated by
   an AJV twin (third-party oracle) over every committed descriptor's topic payloads, plus a Rust census law per parent over
   its own static catalogue and every contributed payload it decodes; F1 law per parent: no contributed id equals a
   parent-built id.

### Item 3 — per-extension live checklist (for S16's matrix after W3's restage)

Every row: open the parent app in `s` (local + hub document), observe the extension's contribution arrive (host console
`setContributions` push / palette section), perform one verb that uses it, then ↶/↷, in en and de.

| extension(s) | parent app to open | what proves the extension loaded | verb that uses it | de expectation today |
|---|---|---|---|---|
| flow ×9 (bim, brep, dictionary, draw, list, logic, math, primitive "Core", text) | `s.flow.flow` editor and `s.procedural.generation3d` editor (+ demonstrator generator pane) | the node palette shows one section per extension (label = extension name) and `listFlowExtensions` (procedural, palette command) lists 9 rows at the tree version | add an operator node of that extension, connect it, preview/evaluate shows a value (brep: box → preview mesh) → ↶ removes the node → ↷ restores it | palette/operator texts English (1b) — record, not a new bug |
| playbook-procedural | `s.playbook.playbook` editor | block palette shows "Building Component" (`playbook.blockKind`) | insert the block → Params + Preview windows render; `exportSolidGeometry` (OBJ) stashes `__solidExport`; ↶/↷ | block palette label English (1b); app/window/action labels German |
| process ×4 (metal, wood, robotic, concrete) | `s.process.process3d` editor | the workshop catalogue lists Metal/Wood/Robotic/Concrete sections — NOTE F1: the parent builds these in, so the section is visible even without the extension; proof of the extension itself = its `process.machines` entry in the host's pushed contributions JSON | add a catalogue machine to the workshop → ↶/↷ | machine labels English |
| sourcing ×3 (beams, slabs, windows) | `s.sourcing.curation` editor | curation module list shows Beams/Slabs/Windows — same F1 caveat (parent built-ins) | add a catalogue kind to the curation → ↶/↷ | kind names English |
| cad ×4 (aec-building, -structure, -energy, spatial-shape) | `s.cad.cad` editor | `cad.computer` registration: stats/properties panels show `energy.demand`/`structure.stability`/`spatial.shape.volume` — F1 caveat: cad's TS runtime ships a default copy | import a model with the aec-building profile / run a stat → ↶/↷ | 4 labels English |
| imperative ×5 (control, effect, logic, math, text) | `s.imperative.procedure` editor | catalogue sections Control/Actions/Logic/Math/Text — F1 caveat: the artifact `#[path]`-includes the extension sources | add an `If`/`Add`/`Text Concat` block, run → ↶/↷ | catalogue English |

### Session 13 Log

- 19:42 started; read AGENTS.md, preambles 13/12, `audit-s13-plugins.md`, `wp-p8.md`, extension parts of `wp-t12.md`,
  `wp-s15.md`, `wp-w2.md`, `landing.md`, `wp-w3.md`, `fleet-13-agents.md`.
- 19:4x correction to the audit: 25 of 26 extensions DO have individual session-12 evidence — T12's `test quick` chain ran
  every extension package (`.🧬semio/🌐hub/s12-t12-captures/quick/<parent>+<ext>.txt`): 25 EXIT 0, only
  `📖️playbook+🌀️procedural` red (`component::descriptor_is_fresh`, 12/13 passed). Re-measured below on the current tree.
- 19:50 native chain `s17-native-chain.sh` (pid 43677) started; 19:5x census + findings to main (topic-only extensions, 0/26
  exact pins, 933 English-only strings). 20:0x coordinator: S17 lands W2's whole exact-pin narrowing (W3 drops it) and the
  en+de strings schema-first if feasible before REBUILD START (~23:30), else a prepared patch set.
- 20:0x preamble rule 22 (memory): chain stopped (killed my 43677 + its cargo 47769, verified no children left) — it would
  have run in parallel with the landing gate; its gate is now `< 10` rustc; it restarts on the landed tree.
- 20:13:58 **landed `s17-version-pin.py --write`** (dry run clean 20:13 on the current tree; 59 files, 116 anchored ops):
  `semio_framework::VersionPin(pub Version)` replaces `VersionReq` (wire form `=X.Y.Z` only; `NotExact` for `*`/`^`/`~`/`>=`/bare;
  `const fn of_tree` + `#[macro_export] tree_pin!()` evaluated in a `const` in the declaring crate → a non-triple crate version
  is a compile error, never a guest trap); both builders' `depends_on(id, VersionPin)`; 26 extensions + demonstrator + stdio
  pin `semio_framework::tree_pin!()`; every extension's own version, the flow extension manifest versions and procedural's
  flow-extension roster use `env!("CARGO_PKG_VERSION")` (brep 0.3.0 / math 0.2.0 drift gone; `listFlowExtensions` now reports
  the tree version); procedural's triplicated `.describe(…)` (codemod injury) reduced to one; hub fixture reads `version.0`;
  kernel `.sxt` `PackagePluginDependency::from_json` refuses non-`=X.Y.Z`; TS twin `VersionPin` + exact-only `versionSatisfies`,
  registry pre-build edges carry ids only (`version?`); laws: manifest `🔬️plugin-dependency` rewritten (pin parse/refusal,
  `of_tree`, `tree_pin!`, serde + descriptor-codec refusal of ranges = the emitter refusal), SDK host/builder/extension-bundle
  tests on pins (hot reload: any bump breaks a pin), `.sxt` range refusal law, TS backbone/broadcast/plugin-runtime tests,
  Gherkin `✅️satisfy-version-requirements` (exact-pin vectors, `semver` oracle) + `🚫️reject-malformed-version-input` (+6 range
  rows). Native gate (`s17-check-pin.sh`, 34 crates, `--lib --tests`) pid 61358 → `s13-s17-logs/check-pin-1.txt`.
- 20:2x found while reading the procedural extension's descriptor: every app of every plugin lacks descriptions for the 3
  framework-injected actions `setHistoryCommandFilter`, `noteShellCommand`, `recordTutorial` (forms, playbook, procedural
  extension all show the same 3 holes) → en+de `.describe(…)` added at their one definition in `🛂️manifest/🦀️.rs` (same
  compile wave as the pin set, before the gate reached `semio-framework`).
- 20:2x TS gates (measured): Gherkin `✅️satisfy-version-requirements` `parity fundamental` → executed 2, passed 2, parity 1/1
  (`semver` oracle agrees on 9 exact-pin vectors); `🚫️reject-malformed-version-input` `parity quick` → 1/1 (12 rows incl. the 6
  range rows — the pre-landing `versionSatisfies` returned true for `*`/`^`/`~`/`>=`, so these rows are red on the old code).
  Framework kernel in-source vitest (`test quick 🎠️kernel`, includes the broadcast/AppRouter/registry-expansion suites I
  edited) **53/53**; OS `test quick -t PluginGraph` (backbone suite I edited) **9/9**. (A peer's new `🎯️acceptance`
  orchestration file had a wrong relative import for ~2 min at 20:2x and broke the test router; fixed by its owner.)
- 20:46 gate run 1 stopped by me (preamble rule 25: 16 min without a rustc child — the edited `semio-framework` unit needs an
  exclusive lock behind ~28 fleet cargos holding shared ones); landing row + `wp-w3/requests/s17.txt` written; main told.
  20:49 run 2 (pid 91697) → `s13-s17-logs/check-pin-2.txt`.
- Item 2 analysis (source + session-12 captures): the playbook/procedural red is ONLY `component::descriptor_is_fresh`
  (T12 capture: 12 passed, 1 failed) — the committed `🛂️.descriptor.semio` (09-25 18:43) predates the two own action
  descriptions in source; after 1a it also carries `^0.1.0` pins the new decoder refuses. Root fix = re-describe (W3's
  describe-all in the consolidated rebuild); no code defect. Flow extension chain: procedural does not `consume`
  `flow.extension` and does not need to — the shell cuts `flow.extension` by operator reachability
  (`scopeContributionsJson`, `🎠️kernel/🟦️.ts` "operator-keyed topic") and pushes it through `setContributions`; procedural
  `depends-on` the 9 extension actors. Defects found on the chain and fixed in 1a: the roster `FLOW_EXTENSIONS` reported
  math at 0.1.0 while the extension declared 0.2.0 (now one tree version everywhere); `listFlowExtensions` carried the same
  `.describe(…)` three times. (Correction 05:0x: the payload `appId`s `flow-play`/`process3d-play`/`cad-play`/`sourcing-curation`
  are live internal controller ids that the process, cad and sourcing consumers filter on — not dead.) Live proof of the
  chain is item 3 (after the restage).
- 05:0x **Finding S17-F1 (architecture, measured in source): 16 of 26 extensions are redundant copies of content their
  parent already compiles in**, so the extension actor changes nothing at runtime:
  process ×4 — `process3d` builds the same `wood/concrete/metal/robotic` catalogs in (`builtin_installed_catalogs`) and
  `installed_catalogs` DROPS a contribution whose id is already installed ("the installed build wins");
  sourcing ×3 — `curation` builds `beams/windows/slabs` in and `installable_contributions` drops same-id contributions (the
  extension crates even import the parent's `BeamsModule`/… data);
  imperative ×5 — the `procedure` artifact `#[path]`-includes every extension's `🦀️.rs` (`pub mod extensions`) and registers
  them natively as its default contributions (`default_imperative_contributions_json`, `register_native_imperative_module`);
  cad ×4 — the cad TS runtime ships a hard-coded copy of all four `cad.computer` payloads (`shippedCadComputerContributionsJson`,
  aec-building's copy already drifted: `layerTypology: {}` vs the extension's full typology) and the payload only switches
  cad-js registrars compiled into cad.
  Only flow ×9 (separate operator actors; procedural/demonstrator `depends-on` them) and playbook-procedural (own app) are
  real extensions. AGENTS.md (domain-neutral framework, domain-specific extensions; repeated code close together) → the parent
  keeps only its domain-neutral core (process: generic catalog; sourcing: reuse module; imperative/cad: no built-in modules) and
  the extension is the single source, loaded with its parent (`depends-on`, like procedural→flow extensions). Routed to main
  (guest refactor across 4 parents + 16 extensions + their laws/fixtures; not before REBUILD START).
- 21:05 gate run 2 exited 143 (stopped under the rule-26 switch); run 3 in `build-fleet-b` (pid 11951) reached third-party
  crates only before the ~21:30 usage cut; its processes died overnight (preamble rule 28). 21:1x i18n translation tables
  written: `wp-s17/s17-i18n-de-parents.json` (230 strings of process/cad/playbook/imperative/sourcing, 16 identifiers kept),
  `wp-s17/s17-i18n-de-flow.json` (528 flow display strings; 110 port-derived identifiers kept) — both measured complete
  against the census (0 missing).
- 05:00 (09-27) resumed; reconciled: the pin set + the 3 framework descriptions are intact on the tree (auto-commit
  `40a2736e661` 22:00 carries them; `git grep VersionReq` → only 2 doc comments in `📇️registry/🔎️discovery/🟦️.ts`, now
  `VersionPin`). Gate run 4 (`s17-check-pin.sh`, phases A=SDK core+procedural+flow artifact+26 ext / B=demonstrator /
  C=stdio+hub, `build-fleet-b`, nice 10) detached via `w2-detach.py` pid 93023 → `s13-s17-logs/check-pin-4.txt`.
- 05:11 run 4 phase A EXIT 101: 4× E0422 `EmitWire` in `🔌️plugin/🦀️.rs` plugin_runtime — a peer's in-flight SDK edit (file
  written 05:11:23, 420 uncommitted lines adding `EmitWire`/owned-child `children`), not mine; run 5 (05:14) got past the lib
  but the SDK lib TEST target failed on the same peer's unfinished test updates (`transaction_prepare` now 7 args ×4,
  `FaultCode::as_str` ×2) → gate switched to `--keep-going` and no early phase break.
- **05:29:55 run 6 phase A EXIT 0** (`check-pin-6.txt`): semio-framework, -os-kernel, -plugin (lib + lib test), procedural,
  flow artifact and all 26 extension crates (`--lib --tests`); 58 warning lines, none in a hunk of mine (manifest 4926
  "unnecessary qualification" is 09-14 code; cad extension dead-code warnings are the wasm-gated `extension_exports!`).
  Phases B/C (demonstrator, stdio, hub) relaunched without the rustc gate (41 rustc fleet-wide) → `check-pin-bc-1.txt`; wasm32
  gate queued in the wasm mutex behind t13/wg9/h12 (`check-wasm-1.txt`, starts after the native capture's ALL_DONE).
  Coordinator 05:4x: the set stays; wasm32 proof = the REBUILD components step. Landing row updated; roll-call sent 05:40.
- 05:5x R9 → S17: `🎮️playground/🧭️session/🟦️.ts:56` TS2322 from 1a — `PlaygroundSessionPlugin.dependencies` still typed
  `{ pluginId; version }` while the builder, the emitted session type and `📽️projection` produce `{ pluginId }` (I had edited
  the emitter string and the map, not the exported type). Fixed: the type is `{ pluginId }[]` with a docstring (pre-build edges
  carry ids; the pin travels on the loaded descriptor). OS `tsc --noEmit -p 💻️os/tsconfig.json`: 46 errors, 0 in
  registry/session/projection/kernel/backbone/plugin-runtime (all in peers' files). R9 answered.
- 06:04 my wasm32 gate got the wasm mutex but its cargo waits for the native B/C capture (stdio still compiling in
  fleet-b) → it would have held the mutex idle ahead of LD/LB/LA (landing, critical path): released 06:13 (killed my own
  24723 + child 42028; lock dir gone, LD next). wasm32 proof = REBUILD components step (coordinator 05:4x, rule 29).
- 06:13 the coordinator stopped my B/C native check (24721, fleet-b convoy); I had relaunched it (48370) before reading the
  fleet log and stopped it again at 06:15. stdio (`full-artifact-catalog`, via the hub's default `native-artifact-execution`)
  + hub are covered by H11's `cargo check -p semio-hub --bins` GREEN on the current tree (06:0x, fleet log) and its
  `--bins --tests` run (my hub hunk is `integration-fixtures`/test-gated). Demonstrator alone: `s17-check-demonstrator.sh` in
  `build-landing` (coordinator: final landing checks until 06:55), pid 48714 → `check-demonstrator-1.txt`.
