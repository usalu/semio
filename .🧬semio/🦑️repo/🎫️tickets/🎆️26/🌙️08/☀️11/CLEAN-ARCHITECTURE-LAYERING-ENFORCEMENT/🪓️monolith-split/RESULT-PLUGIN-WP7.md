# 🧵️ Result PLUGIN-WP7: instance tier 2 and 3, builder, testing (design recorded, conversion in progress)

Executor: Sonnet 5.5, foreground, no sub-agents, no git-modifying command, no ticket tool. Slice `PLUGIN-WP7`, prefix `pwp7`. Tools: `pwp7-tools/` (this folder). Logs: `🗑️generated/pwp7-*`. `PLUG` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`, `INST` = `PLUG/🧵️instance`. Inputs: `📓️WAVE5-BRIEF.md`, `PLAN-PLUGIN-CRATES.md` (7, 11, 12, Appendices H to K), `RESULT-PLUGIN-WP0.md`, `RESULT-PLUGIN-WP5.md`, `RESULT-PLUGIN-WP6.md`, `RESULT-RENDERER-WP5.md` (the borrowed-bundle precedent).

## 0. Verdict

**Staging of tier 2 and 3 is done inside the monolith; no flip was possible.** The acyclic partition is designed and recorded (section 2), all 406 methods are converted to methods of borrowed bundles in eight use-case crates (seven of the plan plus the new `instance-dispatch`), the extension seam takes explicit parameters instead of `&VcsArtifactApp`, the three upward hook calls of tier 1 and the six old-field-name destructurings are fixed, the composer builds the bundles and keeps one-line delegations for its public API, 178 consumer call sites and the 13 `Plugin::builder` callers (D6) are re-pointed, the new crate is registered, and a language-agnostic partition test with a `graphlib` oracle passes. The patched full-resolution differential of the library shows **0 old and 25 new diagnostics, all unresolved external crates** (the added `use` lines in a world where external crates are neutralised); the `--test` differential (which includes the 17 re-pointed test files) shows **0 old and 22 new, again only unresolved external crates**.

**Nothing was compiled by cargo.** The launcher reaches cargo since 05:04 (`RESULT-INFRA-UNBLOCK-2.md`) but admits no crate of this family: every one depends on the oversized `semio-framework-os-kernel`, and the WP3, WP4 and WP5 crates that tier 1 depends on are still staged (`RESULT-PLUGIN-INTEGRATION-1.md` is in progress, nothing flipped). **Flips of tier 1, `instance-extension`, the ladder crate and tier 2 are therefore not done** (section 8 has the exact checklist). Typing, borrow checking and the privacy of methods rest on the static evidence of section 4.

## 1. Found state (WP6 left a cut result file)

* `RESULT-PLUGIN-WP6.md` still contains the three placeholders `@@VERIFY@@`, `@@TS@@`, `@@WP7@@` (sections 4, 5, 7): the executor was cut before it wrote them. Its tool folder and logs are complete; I read `pwp6-tools/` (`t2plan.py` is a partial tier-2 plan), `🗑️generated/pwp6-s0-recs.json` (678 method records with callee lists), `pwp6-oracle-t1c.log` (last differential: lib onlyNew 110 / onlyOld 0, test onlyNew 83 / onlyOld 0, every new key an external-crate E0432/E0433), `pwp6-orphan-final.log` (no inherent `impl VcsArtifactApp` in a tier-1 crate; 20 misplaced tier-2 impls = the 406 methods) and `pwp6-dag-final.log`.
* The live tree matches the log: 15 components are crate-staged (`<tier-1 root>/📥️staging/🧱️components`), 67 S0 staging files exist, the composer is `VcsArtifactApp` with 20 fields, the 406 use-case and composer methods are still `impl VcsArtifactApp` blocks in 23 files (`INST/🧷️command-routes` root and 4 staging files, `INST/🧾️typed-operations` root and `🛠️tool-machine`, `INST/🖼️render`, `PLUG/🪜️maintenance-ladder` root and 6 staging files, `PLUG/⏪️time-travel/🧵️instance`, `PLUG/⏯️tool-run/🧵️instance`, `INST/🔌️plugin-app-impl`, `INST/📥️staging/{📨️dispatch,🔒️close-ladder,🚪️ingress,🪜️maintenance-ladder,🧬️composer-api}`). No half-applied batch was found (no `.orig`, no apply-state outside the logs).
* **Defects WP6 left that WP7 must fix** (found by reading, none was in its logs):
  1. Three tier-1 methods call `InstanceExtensions::hook(self, ..)` with a component as `self`, an upward call the name-keyed homing could not see: `InstanceStores::validate_framework_reserved_commit` (`reserved_action_refusal`), `InstanceStores::advance_mounted_window_config_refresh` (`window_config_published`), `EnvelopeJobs::document_loading_refusal` (`admitted_while_loading`).
  2. Six destructuring patterns of the old field names remain in the two feature turn files (`let VcsArtifactApp { store, time_travel, children, child_content_retirements, .. } = self;`, five in time-travel, one in tool-run). The rewriter had no rule for patterns.
  3. The two instance-extension crates' impls (`TimeTravelExtension`, `ToolRunExtension`) and the `InstanceExtension` trait still take `&VcsArtifactApp`; `VcsArtifactApp::<A, M>::transaction_fault` in the time-travel hook is a composer path to a method that is now `MediaTransfers::<A>::transaction_fault`.
  4. `pwp6-dag-final.log` reports `instance-jobs -> instance-app close_step` (2): a name collision with `source.close_step(..)` of other types, not a real edge (verified, section 2.8).
  5. `pwp6-arity-t2.log` lists two arity findings, `register_child` (4 args at a test call, defs 5/6) and `run_framework_reserved_job` (9 args at a staging call, def 10): I recheck both against the live definitions.

## 2. Design (recorded before the first edit)

### 2.1 What the evidence says

The plan's file-aligned partition is cyclic (WP6: 19 crate-pair back edges, `commands <-> typed-ops`, `commands <-> time-travel-turns`, `render <-> commands`, `ladders -> *`). The method-level call graph is not: of the 406 methods (names resolved over the 678 records), **one** strongly connected component exists, three methods: `deliver_host_event` -> `dispatch_typed_command_inner` -> `deliver_utility_host_events` -> `deliver_host_event` (host event delivery re-enters the typed-command admission, a real recursion, verified in the source). Everything else is a DAG with 17 levels. The cycles of the file partition come from methods that sit in the wrong file (a history-view helper in the typed-operations file, a ladder step in the ingress file, a reserved-producer admission in the routes file), so the cure is a re-homing of methods by use case, not a port.

Re-homing moves, each justified by the call graph (`pwp7-tools/plan7.py` and `q7b.py` print the evidence; no new type, no trait):

| method | from | to | reason |
|---|---|---|---|
| `reprojection_status` | time-travel-turns | render | called by `history_patch` (render); it reads the ledger and the document load only |
| `finish_recorded`, `verb_dirties_history_panel` | typed-ops | render | history-panel patch helpers, called by the routes and the typed command; they only call `history_patch` |
| `child_root_retirement_step` | commands | ladders | it is a ladder step calling a ladder unit |
| `retained_fields_terminal_is_empty` | typed-ops | ladders | calls the ladder's `retained_fields_own_terminal_is_empty` |
| `mount_original_reserved_owners`, `admit_original_tool_lease`, `admit_reserved_emit_producer` (+ 3 `test_reserved_emit_*`), `dispatch_reserved_original_producer`, `commit_framework_clipboard_completion`, `admit_framework_reserved_spawn`, `mount_original_emit_publication` | commands | dispatch | the reserved emit producer admission; `dispatch_emit` calls it, so it must sit below the routes |
| `dispatch_group_history_action` | render | dispatch | group emission; called by the history route |
| the host-event delivery and the typed command admission (`deliver_*`, `dispatch_host_event_action`, `dispatch_typed_command_inner`, `capture_typed_command_roots`, `start_typed_command_operation`, `dispatch_typed`, `dispatch_import_media`, `dispatch_config_command_inner`, `finish_configuration_command`, `test_*` fixtures of these) | commands / typed-ops | dispatch | the one real recursion (3 methods) pins the two halves together; the rest of the typed command entry is its caller |
| `retire_typed_operation_*`, `advance_private_child_group*`, `publish_mounted_owned_child_operation_unit` | commands | typed-ops | typed-operation retirement and the private child group publication; called by the publication and the ladders |
| the tool-machine gesture and press methods (`follow_tool_machines`, gestures, overlay retirement, `settle_*`) | typed-ops | render | `refresh_cache` (render) calls `follow_tool_machines`; the tool overlay is what every render seam reads (`rendered_snapshot`, `rendered_config`) |

### 2.2 The partition (acyclic, verified over the 406 names: 0 upward calls)

Rank order, low to high; a crate calls only lower crates (`pwp7-tools/plan7.py check`):

| rank | crate | root | use-case group | methods | method lines | calls into |
|---:|---|---|---|---:|---:|---|
| 1 | `instance-render` | `INST/🖼️render` | tool overlays and presses, history view and projection cache, interaction state, reserved job inputs | 48 | 1,220 | - |
| 2 | `instance-dispatch` **(new)** | `INST/📨️dispatch` | `dispatch_emit` and the transaction groups, the reserved emit producers, host events, typed command admission and start, group history | 32 | 1,408 | render |
| 3 | `instance-typed-ops` | `INST/🧾️typed-operations` | typed operation publication, retirement, private child group publication, the typing and tool dispatch commits | 22 | 1,255 | render, dispatch |
| 3 | `time-travel-turns` | `PLUG/⏪️time-travel/🧵️instance` | the history-edit session turns, the supersede authoring, `TimeTravelExtension` | 53 | 1,114 - 19 | render, dispatch |
| 3 | `tool-run-turns` | `PLUG/⏯️tool-run/🧵️instance` | the tool run turns, `ToolRunExtension` | 37 | 972 | render |
| 4 | `instance-commands` | `INST/🧷️command-routes` | framework-reserved routes, action dispatch and preview, archive merge | 21 | 861 | render, dispatch, typed-ops, time-travel-turns |
| 5 | `instance-ladders` | `PLUG/🪜️maintenance-ladder` | maintenance, close and envelope ladders, the two cursor components | 57 | 1,434 | render, typed-ops, time-travel-turns, tool-run-turns |
| 6 | `instance-app` | `INST` | composer, `PluginApp`, constructors, `Drop`, public delegations | 136 | 2,145 | all |

Rank 3 has three siblings with no edge between them. Method totals: 8 crates, 406 methods. The plan's `instance-commands` shrinks (861 of 1,337 lines: its dispatch half becomes the new lower crate), `instance-typed-ops` and `instance-render` change content (the plan's descriptions stay true: "typed-operation publication", "render, interaction state, media jobs"). One crate is added: `instance-dispatch` (the plan had 7 tier-2/3 crates here; the registries get one new row set, 2.9).

### 2.3 Bundles

Every use-case crate defines its borrowed bundle (plan 7.2.3, renderer WP5 precedent) and never names `VcsArtifactApp`:

* `XCtx<'ctx, A, M, X>` with `pub` fields named exactly like the composer's (`app: &'ctx mut A`, `mounted_policy: MountedOwnerPolicyV1` (Copy, by value), `stores: &'ctx mut InstanceStores<A>`, ...), so a method body written for the composer keeps `self.stores.store` unchanged. The field set of a bundle is the union of the components its methods read and the components of every bundle it bridges to (`pwp7-tools/fields7.py`; most bundles are 15 to 18 of the 20 composer fields, the renderer WP5 found the same wide shape).
* `XRef<'v, A, M, X>`, the shared twin (`&'v` fields, `Clone + Copy`; named `Ref` because `ToolRunView` is already the tool-run panel type), exists for the crates that have `&self` methods called across a crate boundary (render, time-travel-turns, tool-run-turns, ladders). Reason: the composer's `PluginApp` queries are `&self` (`terminal_is_empty`, `close_ladder_demand`, `maintenance_*`), a `&self` receiver cannot build a bundle of `&mut` fields, and a `&self` method of a higher crate cannot bridge down through a `&mut` bundle. All `&self` methods of these four crates are members of `XRef`; `&mut self` methods are members of `XCtx`; `XCtx::shared(&self)` reborrows the shared twin.
* Bridges are explicit and point down only: `fn as_<lower>(&mut self) -> LowerCtx<'_, A, M, X>` and `fn <lower>_ref(&self) -> LowerRef<'_, A, M, X>`; both build the lower value from fields (reborrows). A call `self.m(a)` of a lower crate becomes `self.as_<lower>().m(a)`; an argument that names `self` is hoisted into a block (the WP6 rewriter rule).
* The composer (`instance-app`) builds each bundle: `fn <crate>_ctx(&mut self) -> XCtx<'_, A, M, InstanceExtensions>` and `fn <crate>_ref(&self) -> XRef<..>`; the 44 delegations of WP6 plus the 8 public use-case methods (`set_tool_clock_ms`, `rendered_snapshot`, `rendered_config`, `dispatch_typed`, `time_travel_ledger`, `apply_time_travel_action`, `apply_tool_run_action`, `next_maintenance_stage`) keep a one-line composer delegation, so the 284 external files and their `app.m(..)` calls do not change.
* Whole-component borrows inside bundle bodies are written `&mut *self.stores` / `&*self.stores` (a coercion does not apply in `Some(&mut self.history)` and generic positions); field paths (`self.stores.store`) auto-dereference and are unchanged.

### 2.4 The extension seam (decision D2: Phase 1 only)

`InstanceExtension` (crate `instance-extension`, WP6's, converted here) stops taking `&VcsArtifactApp`:

* The **static** hooks (`claims_action`, `reads_history`, `admitted_while_loading`) are unchanged.
* The **ledger** hooks (`reserved_action_refusal`, `mutation_refusal`, `freezes_document`, `document_refusal`, `transaction_refusal`, `holds_receipt`, `terminal_is_empty`) read only `app.time_travel` and `app.tool_runs`; they take `(time_travel: &TimeTravelLedger<A>, tool_runs: &ToolRunLedger<A>, ..)`. `window_config_published` takes both as `&mut`. Explicit parameters, no wrapper type; every bundle already holds both ledgers.
* The **context** hooks (`dispatch_action`, `drive_turn`, `render_body`, `after_app_render`, `has_pending_work`) take `ExtensionCtx<'_, A, M, X>` (`ExtensionRef` for `has_pending_work`), defined in `instance-extension` as the union of what the two implementations need; `TimeTravelExtension` and `TimeTravelCtx::of_extension(..)` / `ToolRunCtx::of_extension(..)` build the use-case bundle from it.
* `X: InstanceExtension` is a type parameter of every bundle (the plan's "monomorphisation unchanged"): `X::mutation_refusal(&*self.time_travel, &*self.tool_runs, verb)`. `instance-app` fixes `type InstanceExtensions = (ToolRunExtension, (TimeTravelExtension, ()))`.
* The three tier-1 upward hook calls (1.1) change to explicit parameters: `validate_framework_reserved_commit(.., refusal: Option<Fault>)`, `document_loading_refusal(.., admitted_by_extension: bool)`, and `advance_mounted_window_config_refresh` is split at the hook (it returns the published receipt; the use-case caller runs the hook, then `InstanceStores::queue_window_config_receipt`), which keeps the order hook-then-queue.

### 2.5 Staging layout (domain-driven)

As WP6 did for tier 1, every use-case method lives in `<crate root>/📥️staging/<domain>/🦀️.rs` (one file per domain, at most 2,500 active lines, each starting with a unique-emoji docstring) and the bundle in `<crate root>/📥️staging/🧺️bundle/🦀️.rs`; the instance parent mounts them (`//#region 📥️StagedInstanceMounts`), the class-B roots stay docstring-only until the flip moves the mounts into the crate root. The time-travel and tool-run turn files stay where they are (their crate is the whole file). Domains: render `🛠️tool-machine`, `📜️history-view`, `🧭️interaction`, `🏷️reserved-jobs`; dispatch `📤️emit`, `🎟️producers`, `📣️host-events`, `⌨️typed-command`, `🤝️group-history`; typed-ops `📬️publication`, `🪦️retirement`, `🪆️child-group`, `⌨️typing`; commands `🧊️reserved-routes`, `🚦️actions`, `📦️archive-merge`.

### 2.6 Free items and associated functions

Free items follow their users to the lowest crate that uses them (ladders: `MaintenanceUnit`, `MaintenanceTool`, `CloseRung`, `ClosingOwner` and its impls, the envelope and replacement helpers; time-travel-turns and tool-run-turns: their extension types and fault helpers; composer: the four `plugin-app-impl` free functions). Associated functions without a receiver (`history_command`, `history_command_authors`, `verb_dirties_history_panel`, the interaction helpers, `typed_operation_stall_streak`, the two store-replacement quotes) become free functions of their crate in the same staging file, and their `Self::f(..)` callers become `f(..)`; `dispatch_typed` (`pub`, no receiver in the old shape check, actually `&mut self`) is re-read by the signature parser.

### 2.7 Tests and consumers

Consumers of the non-composer use-case methods outside the family: 22 files, about 600 call sites (`ext7.py`), 170 in `🧪️tests/🔬️plugin-runtime-plugin-builder-contract`, 64 in `🧪️tests/🧪️time-travel`, 18 in `🧪️tests/🧪️gesture`, 13 each in `🧪️tests/🔬️tool-run` and the retained-command `🔄️full-operation` file. They are rewritten with the WP6 engine: `app.m(a)` becomes `app.<crate>_ctx().m(a)` (arguments that name `app` hoisted). The companion bins T10 to T21 receive the tests as the plan 11.3 packs them.

### 2.8 Checks against false findings

`close_step` (`pwp6-dag-final.log`): the 2 hits are `pending.close_step(grant)` and `source.close_step(..)` on retirement owners, not `VcsArtifactApp::close_step` (the composer method of the `PluginApp` ladder). Verified in `👷️jobs/📥️staging/🔒️close-ladder` line 49 and `🧾️typed-operations` line 228.

### 2.9 Registries for the one new crate

`semio-framework-plugin-instance-dispatch` (id `plugin-instance-dispatch`, root `INST/📨️dispatch`, class A: the directory is empty, the root `🦀️.rs` is the future crate root), rank between `instance-render` and `instance-typed-ops`: manifest, `📋️project.json`, workspace member (byte-sorted), `📋️build-owners.json` owner row, six launch routes in both launch files (the WP0 generator `pwp0-tools/gen_registry.py` extended by the one row; the order keys follow the rule of plan 12.5.8). Dependencies in the `Cargo.toml` of the other tier-2 crates are updated to the new edges (2.2).

## 3. Work log

* **05:45 to 06:10, conversion batch 1** (`pwp7-tools/apply7.py`, pre-edit copies in `🗑️generated/pwp7-snap/`, list in `🗑️generated/pwp7-touched.json`): the 406 methods were re-homed by the partition of section 2.2 and converted to bundle methods. Pipeline: `src7.py` (inventory of the pre-edit text, signature parser), `rw7.py` (call-site rewriter: `self.m(..)` to a same-crate call, `self.shared().m(..)`, `self.as_<crate>().m(..)` or `self.<crate>_ref().m(..)`; arguments that name `self` are hoisted into a block; `InstanceExtensions::hook(self, ..)` to the explicit forms; whole-component borrows `&mut self.c` to `&mut *self.c`), `gen7.py`/`bundle7.py` (field sets by closure over the bridges, the bundle files, the composer builders), `asm7.py`/`plan_all7.py` (domain staging files with usage-filtered imports, in-place rebuild of the ladders, time-travel, tool-run and composer files), `feat7.py`/`ext_text7.py` (the two extension implementations and the trait), `finish7.py` (mounts, tier-1 hook fixes, composer delegations). 46 files written, 5 old staging files deleted, `rustfmt` parses all of them.
* **Registries for `instance-dispatch`** (`reg7.py`): manifest, `📋️project.json`, class A root, workspace member, owner row, six launch routes in both launch files; the `[dependencies]` of the other seven tier-2/3 crates and of `instance-render` (feature `artifact-app-testing`) were rewritten to the new family edges. TOML of nine manifests parses; `launch.json` parses (12,719 configurations).
* **Consumers** (`cons7.py`): 178 call sites in 17 test files and one lower-crate test file (`retained-command/🔄️full-operation`, one line) use `app.<crate>_ctx().m(..)` or `app.<crate>_ref().m(..)`; name plus arity plus the absence of a second definition decided each site (`dispatch_action` of four arguments and `dispatch_emit` of four arguments belong to other types and were left).
* **D6** (`d6_7.py`): `Plugin::builder` was removed from `🏗️builder/🦀️.rs` (an inherent impl of a type that `plugin-program` now owns is E0116); its 13 callers (2 in `🧪️testing`, 3 in `🌎️hub/🧩️compositions`, 8 plugin SQLite snapshot tests under `✏️s`) call `PluginBuilder::<NeedsLabel, PA>::new(..)`; the hub files import the two names.
* **TS readers**: `familyText()` is imported by 7 TS tests but its helper file was missing from the tree (WP6 section 5 was never written); I created `🧷️companions/🧯️test-support/🧵️instance-family/🟦️.ts` (`instanceText`, `ladderText`, `turnText`, `familyText` over `readModuleTreeSource`). 16 `readFileSync` of files the S0 cut deleted now read `familyText()` (`ts7b.py`). The 19 reader files went from 45 passing and 41 failing tests to 49 and 37 (`bun test ./<file>`, runs in `🗑️generated/pwp7-ts-run1.log` and `-run2.log`); the remaining failures are exact-fragment assertions of the pre-WP6 text (section 8).
* **Oracle round 1** (`oracle7.py b2`, lib): onlyNew 39, onlyOld 1; 25 of the 39 are unresolved external crates (the added `use` lines), the 14 real ones were: a bundle named `ToolRunView` that collides with the existing tool-run panel type `ToolRunView` (renamed all shared twins `XRef`, accessor `shared()`, bridges `<crate>_ref()`), private imports of `TimeTravelLedger`/`ToolRunLedger` through the instance parent (now imported from `crate::tool_jobs`), composer associated functions (`with_registry`, `uncovered_close_frontier`, ...) that the rewriter had turned into bare names, a `pub(super)` free function re-exported with `pub(crate)`, two outcome types missing in the delegation file. All fixed in batch 4 (`apply7.py batch4-rename-and-fixes`).

## 4. Verification (commands run, real results)

| check | result |
|---|---|
| Method-level call graph over the 406 methods (`pwp7-tools/plan7.py`) | one SCC of three methods (kept in one crate), 0 upward calls for the partition of 2.2 |
| Definition scan of cross-file private methods (`priv7.py`) | 4 private methods called across new files widened to `pub(super)`; the assoc-to-free functions are `pub(crate)` |
| Existence scan (`exist7.py`): every `self.m(..)` in a bundle impl names a method of that bundle type (own members, `shared`, `as_<crate>`, `<crate>_ref`) | 0 unknown; every `self.<crate>_ctx()`/`_ref()` of the composer files is defined by the builders |
| Orphan scan | the only inherent `impl VcsArtifactApp` blocks are in the composer files (parent, `ctx-builders`, `composer-api`, four composer staging files); `PluginApp` and `Drop` are the trait impls |
| rustfmt (nightly, `skip_children`, `fmt7.py`) on all 46 generated or rewritten `.rs` files | 0 parse failures |
| Hubs, TOML, JSON | nine tier-2/3 and tier-1 manifests plus the root `Cargo.toml` parse (`toml7.ts`); `launch.json` parses (12,719 configurations); `📋️build-owners.json` has the one new owner row; longest new path 186 bytes |
| **Differential `lib` (`oracle7.py b3`, sandbox A current vs B = same tree with every WP7-touched file restored from `🗑️generated/pwp7-snap`)** | A 7,296 errors, B 7,271, onlyNew 25, onlyOld 0. The 25 are `E0432`/`E0433` for `semio_framework_tool_machine` (12), `semio_framework` (4), `store` (2), `semio_framework_value` (3), `semio_framework_os_kernel` (2), `semio_framework_job` (2): the added leaf `use` lines of the new staging files |
| Differential `--test` (same sandboxes, tests included; 1,971 s and 1,794 s under load 20 to 50) | A 3,632 errors, B 3,610, onlyNew 22, onlyOld 0; the 22 are the same external-crate classes (`tool_machine` 12, `semio_framework` 4, `store` 2, `semio_framework_value` 3, `semio_framework_os_kernel` 1). The batch 5 and 6 edits (region markers, stable docstring emoji) came after the sandboxes were cut and change no name |
| Round 1 of the differential (`b2`) | found 14 real regressions, listed in 3, all fixed |
| Partition test `🧵️instance/🧪️tests/🟦️.ts` (corpus `🧫️fixtures/🔣️.json`; `bun test ./…`) | 4 pass: the 14 declared bridges are acyclic under `graphlib` `alg.isAcyclic`/`alg.topsort` and agree with the rank order; the bridge calls found in the Rust sources equal the declared set and never point up; no inherent `impl VcsArtifactApp` outside the composer files; every bundle field names a composer field. Seeded sensitivity: an added upward edge fails two tests |
| TS source-text readers (19 files, `bun test ./<file>` one by one) | 49 pass, 37 fail, from 45 and 41 before this slice's helper and reads (3). Failures are fragments of the pre-WP6 text (section 8) |
| Census (`census7.ts`, gate counter with virtualised mounts, `🗑️generated/pwp7-census-final.log`) | section 5 |

## 5. Census (lib native / test native; files, bytes, active lines; limits 28 files / 230,000 bytes / 3,500 lines target)

| crate | lib | test | wasm32-wasip2 lib |
|---|---|---|---|
| `instance-extension` | 1 / 12,206 / 221 | same | same |
| `instance-render` | 6 / 102,400 / 1,346 | 6 / 102,400 / 1,360 | same |
| `instance-dispatch` | 7 / 119,901 / 1,438 | 7 / 119,901 / 1,507 | same |
| `instance-typed-ops` | 8 / 117,959 / 1,407 | same | same |
| `time-travel-turns` | 2 / 101,547 / 1,308 | same | same |
| `tool-run-turns` | 2 / 80,313 / 1,144 | same | same |
| `instance-commands` | 6 / 80,423 / 1,020 | same | same |
| `instance-ladders` | 10 / 148,016 / 2,069 | same | 10 / 148,016 / 2,072 |
| `instance-app` | 8 / **231,060** / 3,098 | 8 / 231,060 / 3,147 | same |

Tier 1 is unchanged (`instance-stores` 14 / 105,505 / 1,297, `instance-history` 5 / 30,425 / 460, `instance-children` 11 / 82,224 / 1,099, `instance-jobs` 14 / 134,639 / 1,695, `instance-docs` 14 / 162,735 / 1,808). `instance-app` is 1,060 bytes over the 230,000 target (hard limit 262,144): the parent still carries the roughly 32 KB of glob-support `use` lines that the staging children read through `use super::*`; the flip replaces them by explicit imports per file, which brings the crate far below the target. The test closure of the family is the companion bins (section 8).

## 6. Files

Created (24): the 16 domain staging files, the 7 bundle files, the builders file (`INST/📥️staging/🏗️ctx-builders`), `INST/📨️dispatch/🦀️.rs`, the dispatch manifest and project file, the partition test and its corpus, the TS helper `🧷️companions/🧯️test-support/🧵️instance-family/🟦️.ts`. Modified (40 plus 13 foreign consumer files): the parent, the extension module, the ladder, time-travel, tool-run and composer files, three tier-1 staging files (the hook fixes), eight manifests, `🏗️builder/🦀️.rs`, 17 test files (178 sites), 16 TS reader reads, `PLAN-PLUGIN-CRATES.md` (new 12.6), `Cargo.toml` (+1 member), `📋️build-owners.json` (+1 row), both launch files (+6 routes). Removed (5 files): the four `command-routes` staging files and the `tool-machine` staging file; empty directories left by S0 were removed. Exact lists: `🗑️generated/pwp7-touched.json`, `pwp7-repo-touched.json` (13 foreign files: 8 plugin SQLite snapshot tests under `✏️s`, 3 `🌎️hub/🧩️compositions` roots, the TS reads), pre-edit copies `🗑️generated/pwp7-snap/` and `pwp7-snap-repo/`.

**Edits outside the owned set (one-line consumer re-points, rule 4)**: `⏪️time-travel/♻️retirement/🧪️tests/🦀️.rs` (5 calls, WP5), `🧵️retained-command/🔄️full-operation/🦀️.rs` (1 call, WP3), the 8 plugin SQLite snapshot tests and the 3 hub compositions (`Plugin::builder`, D6), the TS reads of 11 reader files.

## 7. Real compile status

`RESULT-INFRA-UNBLOCK-2.md` (05:04) says the launcher reaches cargo. I did not run `bun repo cargo ... check` on any crate of this slice, on purpose: the closure of every family crate contains `semio-framework-os-kernel` (26,162 lines in 56 files, over the 4,000-line gate), the lower crates of WP3, WP4 and WP5 are not flipped (`RESULT-PLUGIN-INTEGRATION-1.md`: "IN PROGRESS, nothing is flipped"), and the staged family crates have stub roots, so the gate and the manifests refuse them before rustc. The first honest real check is `-p semio-framework-plugin-instance-extension` and then the tier-1 crates, after pi1 flips `job`, `command`, `child`, `retained-command`, `tool-run`, `time-travel`, `reactor-app`.

## 8. Foreign errors, hand-offs, residual risks

### 8.1 Flip checklist (tier 1, `instance-extension`, ladders, tier 2/3; bottom-up, one crate per check)

1. Pre-conditions: the lower crates of Appendix H up to rank 42 are real (pi1), `Cargo.lock` refreshed, the facade `PKG/Cargo.toml` gets one line for `instance-dispatch` (WP0 phase 2; the transitional block has 58 lines).
2. Order: `instance-stores`, `instance-history`, `instance-children`, `instance-jobs`, `instance-docs`, `instance-extension`, `instance-render`, `instance-dispatch`, then `instance-typed-ops`, `time-travel-turns`, `tool-run-turns` (independent of each other), `instance-commands`, `instance-ladders`, `instance-app`, `builder`, `testing-laws`, `testing-acceptance`.
3. Per crate: make the root (class A: the stub root already is the final root, replace its body by the `#[path] mod` mounts of its `📥️staging` files; class B: rework the module file, set `[lib] path = "../../🦀️.rs"`, delete the package-local stub), remove the mounts from the instance parent (`//#region 📥️StagedInstanceMounts`) and from the time-travel and tool-run roots (`instance_turns`, Appendix K), replace every `use super::*` and `use crate::...` by explicit imports (the parent's roughly 330 glob-support lines are the source list), widen the cross-crate names the definition scan reports (`pub(crate)` to `pub`, `#[doc(hidden)]`), add `extern crate semio_framework_plugin_<key> as <old module name>;` in the hub 6 or 7 unit, and `cargo check -p <crate> --lib` plus `--tests`.
4. The bundle constructors and the bridges are already `pub`; the composer builders are `#[doc(hidden)] pub`, which is the surface the companion bins use (decision D4).
5. The tests that name the composer from a lower crate must move to companion bins at the flip: `⏪️time-travel/♻️retirement/🧪️tests` (calls `app.typed_ctx()` and `app.time_travel_ref()`) and `🧵️retained-command/🔄️full-operation` (calls `app.dispatch_ctx()` and `app.typed_ctx()`).

### 8.2 Companion bins T10 to T21 (not applied)

64 test-only files mention the family (41,153 lines, 3.3 MB). `pwp7-tools/tbins7.py` packs them first-fit-decreasing into 12 bins of at most 3,300 lines / 215,000 bytes / 26 files, with the two oversize files cut into 5 (`builder-contract`: 213 tests in 7,903 lines plus 2,744 helper lines in 89 helper fns, 24 structs, 41 impls, all inside one `mod plugin_builder_contract_tests`, `include!`d at `⚛️reactor/🏃️runtime/🦀️.rs:97`) and 2 (`time-travel`); result `🗑️generated/pwp7-tbins.json` (T10 3,183 lines ... T21 2,518; 7 small files do not fit under the byte ceiling and need the real host rules of 11.2). **I did not mount or cut any test**: a companion crate may not mount a file the monolith still mounts (one owner per file), the cut of the builder-contract file edits a WP4 file, and nothing could be compiled to prove the helper widening. The companion roots `tests-10` to `tests-21` stay empty stubs.

### 8.3 Residual risks

* **No compiler.** Types, trait solving, borrow checking and method privacy are unproven. The riskiest shapes: `&mut *self.c` reborrows next to bridged calls, hoisted arguments (5 sites from my engine, about 50 from WP6), the async `dispatch_typed` that now takes the bundle by value (`mut self`, lifetime `'ctx`), the by-value `MountedOwnerPolicyV1` field (it is `Copy`), `PhantomData<fn() -> X>` and the `X: InstanceExtension` bound on every impl, `use<..>` capture list of `dispatch_typed` (lists `'ctx, 'b, 'context, A, M, X`).
* **Name-based call graph.** Calls through paths other than `self.name(` (a method passed as a value `Self::name`) were checked for the 16 `Self::` sites only; the existence scan covers method calls.
* **Source-text laws.** The Rust laws (`instance_source()`) lost one literal, fixed by the oracle round; windowed laws (`source[a..].find(b)`) depend on the order of the module tree, which changed (new mounts come after `extensions`). The TS readers still fail 37 tests from pre-WP6 fragment drift and deleted-file reads (5 ENOENT remain); I did not rewrite the expected fragments of laws that already failed before this slice.
* **Pre-existing test defects found, not mine**: `test_reserved_emit_admission_case`, `_cancelled_worker_handoff` and `_busy_completion_owner` (now in `instance-dispatch`, `cfg(test)`) call `admit_reserved_emit_producer` and `run_framework_reserved_job` with fewer arguments than the definitions (the `cx` parameter), as the arity scan of WP6 already listed; `register_child` in `🧪️tests/🔬️child-read-capture` has 4 arguments against 5.
* **`instance-app` bytes**: 231,060 of 230,000 target (section 5), a staging artefact.
* **Accessors renamed**: the shared twins are `XRef` because `ToolRunView` is already the tool-run panel type of `📜️declaration/⏯️tool-run`; any plan text that says `XView` means `XRef`.

### 8.4 Hand-offs

* **pi1**: after your flips of `job`, `command`, `child`, `retained-command`, `tool-run`, `time-travel`, `reactor-app`, `reactor-tasks`, the tier-1 crates are the next rank; the staging trees, the bundle files and the flip order are in 8.1. The three tier-1 hook fixes change signatures: `validate_framework_reserved_commit(&self, refusal: Option<Fault>, ..)`, `document_loading_refusal(.., admitted_by_extension: bool)`, `advance_window_config_refresh -> Result<bool, Fault>` plus `queue_window_config_receipt`.
* **WP0 / phase 2**: facade manifest line and `artifact-app-testing` forward for `instance-dispatch` and `instance-render` (new feature); Appendix H row (12.6 records it).
* **WP8**: the extension seam is Phase 1; Phase 2 replaces the two ledger parameters by `InstanceExtensions::State<A>` and removes the four `PluginApp` contract methods and two ladder rungs.
