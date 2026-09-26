# WP-S17 — Plugin Extensions (26) and Plugins Without Live Evidence

Session 13 slice S17 (2026-09-26, Opus 5.5). Ports 8190–8199 / 6690–6699. Scripts `wp-s17/`, expendable captures
`wp-s17/generated/`, binaries `CARGO_TARGET_DIR=wp-s17/target`, durable logs `.🧬semio/🌐hub/s13-s17-logs/`.
Checklist: [📓️audit-s13-plugins.md](📓️audit-s13-plugins.md) (extension rows 35–60).

## Session 13

| # | Item | Status |
|---|---|---|
| 1 | Per-extension native evidence (lib tests, test quick, exact pin, descriptor en+de, apps open) | IN PROGRESS — census done; native chain restarts after the pin landing |
| 1a | (coord 20:0x) W2's exact-pin narrowing `VersionReq` → `VersionPin` (SDK, 26 ext + demonstrator, stdio, hub, `.sxt`, TS twin, laws) + brep/math version drift | APPLIED 20:13:58 (`s17-version-pin.py`, 59 files / 116 anchored ops, dry run clean); native gate running |
| 1b | (coord 20:0x) 933 English-only extension topic strings (+ parents' static catalogues): schema-first en+de | PENDING (quantified; decide feasibility before REBUILD START) |
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
  `.describe(…)` three times. Remaining, not fixed (no reader): every extension's topic payload carries a dead `appId` with
  pre-migration ids (`flow-play`, `procedural3d-play`, `process3d-play`, `cad-play`, `imperative-play`, `sourcing-curation`)
  that no consumer reads (Rust or TS). Live proof of the chain is item 3 (after the restage).
