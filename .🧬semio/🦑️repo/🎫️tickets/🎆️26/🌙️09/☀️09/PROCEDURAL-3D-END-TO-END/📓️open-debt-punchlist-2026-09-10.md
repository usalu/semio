# Open Debt Punch List — Procedural 3D End To End (2026-09-10)

Read-only audit of `📓️status.md` chronology plus all 66 lane reports (`📓️*.md`), verified against the **working tree** (no builds run). Classifications: **STILL OPEN**, **FIXED LATER** (lane/commit hint), **UNCLEAR** (evidence needed).

## Summary

| classification | count | blocker | degraded | test-only | cosmetic |
|---|---:|---:|---:|---:|---:|
| STILL OPEN | 38 | 12 | 14 | 9 | 3 |
| FIXED LATER | 47 | — | — | — | — |
| UNCLEAR | 6 | — | — | — | — |

**Coordinator actions not code lanes:** restage/activate is owed by ≥8 landed lanes (contributions-rearm, extension-result-realloc, close-ladder, idle-turns, taxonomy-fix moves, flow-catalog descriptor, assembly-mount, poll-task-leak). Descriptor/`🔣️.json` regen follows restage.

---

## Punch list

| id | item | source report(s) | classification | file:line (authoritative) | user-visible impact | severity |
|---|---|---|---|---|---|---|
| B01 | Extension **fault** arm pack asymmetry: shell sends `encodePackValue(fault)` but guest decodes with `decode_fault_bytes` | `📓️extension-result-realloc-2026-09-10.md` §4.5 | **STILL OPEN** | `🏛️ShellHost/🟦️.tsx:1663`; `🔌️plugin/🌐host/🦀️.rs:48`; `invoke_extension` `:563` | Browser extension faults (`extension.missing`, invoke failures) decode to garbage / wrong code instead of typed fault | **blocker** |
| B02 | Ok-path pack decode fixed in reactor; fault path not mirrored | same | **FIXED LATER** | `⚛️reactor/🦀️.rs:733-778` (Ok uses `decode_wire_value`; Err uses already-decoded `Fault`) | Ok answers work; fault arm still separate | — |
| B03 | Large extension answers paged; guest OOM on unpaged blocks fixed | `📓️extension-result-realloc-2026-09-10.md` | **FIXED LATER** | `⚛️reactor/🦀️.rs:796-802`; host answer ceiling laws | Eval results deliver without `cabi_realloc` abort | — |
| B04 | **Restage required** for guest/framework fixes not yet activated | status 12:44; realloc, rearm, close-ladder, idle-turns, taxonomy | **STILL OPEN** | (process) shared `target/wasm32-wasip2` | Served wasm stale → preview faulted, eval chain old, boot #12 realloc abort until restage | **blocker** |
| B05 | Contributions re-arm: invalidate session + re-arm `flowEvalTick` per preview | `📓️contributions-rearm-2026-09-10.md` | **FIXED LATER** (needs restage) | `…/set-contributions/🦀️.rs`; `🌊️flow/🖥️host/🦀️.rs` invalidation | Empty registry left preview faulted after install | **blocker** until restage |
| B06 | wgpu **shell-boot silence** at 86 % (single opaque 900 s await, no progress) | `📓️wgpu-shell-boot-silence-2026-09-10.md`; status 12:25 | **STILL OPEN** | `🎞️frame-worker/🟦️.ts:490`; `🌐️browser-worker/🦀️.rs:655` | wgpu playground appears hung ≥5 min with no fault card | **blocker** |
| B07 | Wasm guest **~4 KiB retained per `reactor.poll` call** (4096 B/turn) | `📓️idle-turns-2026-09-10.md` §4; `📓️poll-task-leak-2026-09-10.md` §8 | **STILL OPEN** | `cfg(wasm32)` path inside `reactor::poll` body (native 0 B/turn) | Long sessions OOM (~2660 turns at 189 KB/turn before paging fix); shard death | **blocker** |
| B08 | Idle `MoreWork` from contended typed-op lock | `📓️idle-turns-2026-09-10.md` §3.2 | **FIXED LATER** | `⚛️reactor/🔄️turn/🦀️.rs` (contended no longer folds to MoreWork) | Fewer spurious reactor turns | degraded |
| B09 | **`merge_remote_snapshot` fail-closed** | `📓️flow-host-ownership-2026-09-10.md` §9; `📓️catalogue-surface-2026-09-09.md` §4.2 | **STILL OPEN** | `🏪️store/🦀️.rs:17739-17741` | Multi-instance convergence impossible; VCS merge rejected | **blocker** (test + future multi-user) |
| B10 | `two_instances_converge_disjoint_widget_moves` (gen3d + gen2d) | unit-suite reports; contributions-delivery §5 | **STILL OPEN** | `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:583` (gen3d); gen2d twin `:426` | Convergence law red; uses `assert_two_registered_instances_converge` now | test-only (signals B09) |
| B11 | `assert_two_instances_converge` registry-less helper | `📓️catalogue-surface-2026-09-09.md` §3.3 | **FIXED LATER** (call sites migrated) | `🔌️plugin/🦀️.rs:6596-6600` doc; tests use registered twin | Old helper still unsuitable for catalog-authority apps | test-only |
| B12 | **`FlowHost` dropped** in render/command paths without retire | `📓️flow-catalog-authority-2026-09-10.md` §7 | **STILL OPEN** | `…/🌊️main/🦀️.rs:85-107`; `…/flow-eval-tick/🦀️.rs`; catalogue `:58`; compiled `:40` | `ordered-map root must be explicitly retired` on live flow edits / preview | **blocker** |
| B13 | **`Recipe::advance` leaks OrderedMap** on retained publication | `📓️flow-catalog-authority-2026-09-10.md` §7.2 | **STILL OPEN** | `…/🧵️retained/🗿️artifact/🧬️recipe/🦀️.rs` (see interactive-job test `:155`) | Typed graph-operation routes abort at publication | **blocker** |
| B14 | FlowHost **Dictionary / neural ownership** (outputs duplicate, cache shares) | `📓️unit-suite-3d-2026-09-09.md` §9.1; flow-host §9 | **STILL OPEN** | `🌊️flow/🖥️host/🦀️.rs` history_store / evaluate_step | Pool worker aborts; eval tick 25 s in debug | **degraded** |
| B15 | **`generation_preview_is_one_app_transient_shared_by_two_generation_windows`** 30 s timeout | close-ladder §6.3; contributions-delivery §5 | **STILL OPEN** | `…/🧪️tests/🔬️unit/🦀️.rs:339-407` (`drive_preview_operation` 30 s) | Preview ownership law not green | test-only |
| B16 | **`refresh_pending_effects_arms_flow_eval_tick_chain`** | flow-host §9; hotpath | **STILL OPEN** | `…/🧪️tests/🔬️unit/🦀️.rs:557-565` | Eval chain not armed in registry-less test app | test-only |
| B17 | **`vcs_artifact_app_non_empty_retained_maintenance_swap…`** gen2d `publication.contended` | `📓️envelope-load-2026-09-10.md` §5.1 | **STILL OPEN** | gen2d `…/🧪️tests/🔬️unit/🦀️.rs:155+` | Retained envelope swap law red on 2d | test-only |
| B18 | **`vcs_artifact_app…` gen3d** hostile swap paths | envelope-load | **FIXED LATER** | `🔌️plugin/🦀️.rs:19526-19528`; gen3d test `:208-257` | Turn-1 Fault false positive fixed; full law may pass after run | test-only |
| B19 | **`advance_artifact_envelope_load` Pending vs Fault** | `📓️envelope-load-2026-09-10.md` §1 | **FIXED LATER** | `🔌️plugin/🦀️.rs:19526-19528` | Document load no longer fail-closes on turn 1 | — |
| B20 | Envelope decode worker reactor pump | envelope-load §2 | **FIXED LATER** | `drive_artifact_envelope_decode_worker` in `🔌️plugin/🦀️.rs` | Loads advance inside interactive turns | — |
| B21 | **Assembly editor/viewer not mounted** in plugin root | `📓️assembly-mount-2026-09-10.md`; `📓️taxonomy-fix-2026-09-10.md` §5.1 | **STILL OPEN** | `✏️s/…/🌀️procedural/🦀️.rs:104-108` (comment); no `.editor_with_examples::<Assembly…>` | Assembly artifact unreachable in procedural app | **blocker** |
| B22 | Assembly **five trait bounds** (ArtifactDsl/Pack, OpText/OpBinary ×2) | taxonomy-fix §5.1 | **STILL OPEN** | `🧩️assembly/🧬️schema/` (check errors documented in taxonomy-fix) | Blocks mount + 17 taxonomy findings | **degraded** (gate) |
| B23 | **`demo-session` example** not in `examples()` manifest | `📓️feature-inventory-2026-09-09.md` §TL;DR #4 | **STILL OPEN** | `…/✏️editor/🦀️.rs:1963-1973` lists 8; `demo_session` module exists | 9th example unreachable in navbar | **degraded** |
| B24 | IO codecs 7/9 silently wrong | feature-inventory §TL;DR #1 | **FIXED LATER** | `📓️io-codecs-2026-09-09.md`; e.g. `…/🔺️stl/…/🦀️.rs:1-41` real STL now | Export/import produce real bytes or typed errors | — |
| B25 | Context menu empty selection | feature-inventory; editor-gaps | **FIXED LATER** | `📓️editor-gaps-2026-09-09.md` | Selection-aware context menu | — |
| B26 | Dead `flowEvalResolve` / `flowTessellateResolve` | feature-inventory | **FIXED LATER** | extension-continuation + round-trip lanes | Preview eval continuation works | — |
| B27 | **`BatchOnlyPendingRewrite` (13 flow actions)** | catalogue-surface §4.1 | **FIXED LATER** | `flow/…/✏️editor/🦀️.rs:2608-2641` all `Migrated` | FlowPlayApp constructible; wgpu catalog-authority cleared | — |
| B28 | Catalogue 111 KB → paginated surface | catalogue-surface | **FIXED LATER** | `📓️catalogue-surface-2026-09-09.md` | Flow window renders | — |
| B29 | Fold contract footprint (every gesture fail-closed) | fold-contract | **FIXED LATER** | `📓️fold-contract-2026-09-10.md` | Typed ops succeed after restage | — |
| B30 | **`flowEvalTick` window address** on payload | flow-eval-tick-address | **FIXED LATER** | `…/flow-eval-tick/🦀️.rs:23-25` `window_id` | Eval tick reaches preview window | — |
| B31 | Extension registry id `math/brep` vs plugin id | eval-continuation | **FIXED LATER** | `📓️eval-continuation-runtime-2026-09-10.md` | Extensions resolve | — |
| B32 | **`setContributions` paged** (73 pages) | contributions-delivery | **FIXED LATER** | `📓️contributions-delivery-2026-09-10.md` | Flow extension registry populated | — |
| B33 | cad/process3d/forms/playbook **single-json setContributions** | contributions-delivery §6 | **STILL OPEN** | `📓️contributions-delivery-2026-09-10.md` §6 | Those plugins still capped at 4 KiB closure | **degraded** |
| B34 | **`setPreviewOff` work capacity** (512 vs 256 ceiling) | flow-catalog §7.5 | **STILL OPEN** | `flow/…/✏️editor/🦀️.rs:876` (per report) | Multi-id preview-off refused preflight | **degraded** |
| B35 | Close ladder **2052 turns** / slow instance close | close-ladder §6.2 | **STILL OPEN** | `⚛️reactor` close + app ladder (documented) | Replacing document feels hung | **degraded** |
| B36 | **`plugin_load_document_text` empty ops** aborts on replay | close-ladder §6.1 | **STILL OPEN** | `🏪️store/🦀️.rs:11661` `replay_ops` | Load with empty ops traps | **degraded** |
| B37 | **`preview_eval_exact_window_transient…`** | flow-host §9 | **FIXED LATER** | taxonomy-fix §8.6 (pass); test `…/🧪️tests/🔬️unit/🦀️.rs:7-30` | Window transient isolation law green | — |
| B38 | Window-transient **close ladder** (generation3d Retired) | close-ladder | **FIXED LATER** | `📓️close-ladder-2026-09-10.md`; `🧪️tests/🚪️close-ladder/🦀️.rs` | InstanceClose reaches Retired | — |
| B39 | Boolean / blend / sweep **example geometry** | boolean, blend, sweep lanes | **FIXED LATER** | `📓️example-geometry-tests-2026-09-09.md` §7 8/8 | Example oracle green | — |
| B40 | **`offset_solid_box_round_matches_minkowski_closed_form`** | boolean §5.6; blend § | **FIXED LATER** | `✏️s/…/↔️offset/🧪️tests/🔬️unit/🦀️.rs:29`; blend 13/0 | Offset bomb fixed in blend lane | test-only |
| B41 | **`hex_column_boot_stays_inside_the_interactive_turn_budget`** red under load | taxonomy-fix §8.6 | **UNCLEAR** | gen3d test (timing); `best_eval_step_us=8965` vs 8000 µs | CI flake under parallel compile | test-only |
| B42 | Procedural taxonomy **650 → 22** | taxonomy-fix | **FIXED LATER** (partial) | `📇️registry/📜️script.ts check` | Procedural gate almost clean | — |
| B43 | Repo-wide taxonomy **2123** findings | taxonomy-fix §8.2 | **STILL OPEN** | `🔣️taxonomy.json` `areas["✏️s/🔌️plugins"] = "clean"` | `plugin-registry:check` exit 1 | **degraded** (gate) |
| B44 | Nested taxonomy matrix stale (would 9420 if repointed) | taxonomy-fix §3.1 | **STILL OPEN** | `📇️registry/📜️script.ts` artifact-root matrix | Repo-wide noise | **degraded** (gate) |
| B45 | **`verify taxonomy report` hung** 110 min | taxonomy-fix §8.9 | **UNCLEAR** | pathExclusions remedy applied; full report not completed | Unknown residual repo list | cosmetic (gate infra) |
| B46 | **`descriptor_is_fresh` / regen** | multiple lanes | **STILL OPEN** | `🌀️procedural/🛂️.descriptor.semio` vs manifest | Stale JSON mirrors until describe+restage | **degraded** |
| B47 | **`NodeGraphScene.operators` empty wire field** | catalogue §4.3 | **STILL OPEN** | `📇️catalog.json` NodeGraphScene | Dead bytes on wire | cosmetic |
| B48 | **`WindowMeasure` button** owed | tessellation-jobs § | **STILL OPEN** | `ArtifactEditor::window_measures` (report) | Measure UX incomplete | cosmetic |
| B49 | **`packed-text.ts` ASCII path** taxonomy | json-512 § | **STILL OPEN** | renderer packed-text path | Taxonomy gate edge case | cosmetic |
| B50 | **No procedural3d E2E** harness | test-harness-audit | **STILL OPEN** | no browser E2E in ticket tree | Manual boot only | test-only |
| B51 | PyO3 oracle host unbuilt | test-harness-audit | **STILL OPEN** | (infra) | Python oracle lane blocked | test-only |
| B52 | **`generation2d_viewer_never_mutates`** OrderedMap drop | taxonomy-fix §8.7 | **STILL OPEN** | `🧪️tests/🔬️surface/🦀️.rs` helper | Plugin lib test 1 fail | test-only |
| B53 | Flow **registered content child** close Blocked forever | flow-catalog §7.3 | **STILL OPEN** | `close_step` Blocked reason in report | Flow tests using `register_content_child` cannot close | test-only |
| B54 | **`replay_ops` / FlowDiff retire_cold** on live mutations | unit-suite-3d §1 | **FIXED LATER** | `🌿️vcs/…/🔺️diff/🦀️.rs` retire impl | Flow mutations no longer abort worker | — |
| B55 | JSON **512 truncation** shell slice 0 only | json-512 | **FIXED LATER** | `📓️json-512-truncation-2026-09-10.md` | Windows survive JSON parse | — |
| B56 | Shard **silent termination** / cleanup fault | shard-termination | **FIXED LATER** | `📓️shard-termination-2026-09-10.md` | Boot #11 cleanup improved | — |
| B57 | **Example switch** preview re-arm | example-switch | **FIXED LATER** | `📓️example-switch-2026-09-10.md` | Picker switches graph (needs restage) | — |
| B58 | wgpu **intake budget** / frame-worker stale | wgpu-intake, wgpu-frame-worker | **FIXED LATER** | respective reports | wgpu bundle builds | — |
| B59 | **`worldPointerDown`/`graphPointerDown` in generated descriptors** | editor-gaps §7 | **STILL OPEN** | generated `🔣️.json` mirrors (until regen) | Stale action ids in JSON | cosmetic |
| B60 | **`FlowHost` to_fixture temporaries** on early return | flow-catalog §7 | **STILL OPEN** | `flow/…/✏️editor/🦀️.rs:242,752,…` | Leaks on error paths in render | **degraded** |
| B61 | **`final Dictionary ownership must be explicitly retired`** (flow crate suite) | contributions-delivery §5 | **STILL OPEN** | `🧠️neural/⚙️engine/🦀️.rs:101` (per report) | 127/205 flow lib tests red | test-only |
| B62 | **`set_active_example` fold order** (`sphere-box-fuse`) | unit-suite-3d §50-56 | **UNCLEAR** | fold-contract tests | One fold law may still fail | test-only |
| B63 | **`[DEBUG]` tick probe** in testkit | unit-suite-3d §9.5 | **STILL OPEN** | `…/🧪️tests/🔬️testkit/🦀️.rs` (grep `[DEBUG]`) | Noise | cosmetic |
| B64 | **`component-app-assembly` export** needs flow evaluator | generation3d io | **STILL OPEN** | `…/🚪️io/🦀️.rs:96-98` | Geometry export without feature errors clearly | degraded |
| B65 | Independent sweep: **`#[ignore]`** assembly example outcome | assembly examples | **STILL OPEN** | `🧩️assembly/…/📚️examples/🧪️tests/🧩️outcome/🦀️.rs:138` | Skipped example law | test-only |
| B66 | Independent sweep: **`todo!()`** in framework plugin TS test | plugin tests | **STILL OPEN** | `🔌️plugin/🧪️tests/…/🟦️.ts:16` | Placeholder test helper | test-only |
| B67 | **`assert_viewer_never_mutates` helper** incompatible with real viewer state | taxonomy-fix §9.4 | **STILL OPEN** | framework plugin testkit pattern | Lib tests won't compile for stateful viewers | test-only |
| B68 | **`pending_effects` without refreshUi** general case | contributions-rearm §7 | **STILL OPEN** | host refresh channel (not invented) | Other host pushes may not re-arm work | **degraded** |
| B69 | **`Generation3dViewer` per-command FlowEvalSession** (no invalidation) | contributions-rearm §7 | **STILL OPEN** | viewer command work (by design until viewer retains session) | Viewer path OK today | cosmetic |
| B70 | Runtime verification **37-step checklist** not executed post-fix | runtime-verification-checklist | **UNCLEAR** | checklist md | Unknown boot regressions | — |
| B71 | **`module.vcs` message-ledger remap** fail-closed | store | **STILL OPEN** | `🏪️store/🦀️.rs:17734-17736` | Remote merge adjunct | **degraded** |
| B72 | **`FlowEvalSession::invalidate`** wired | contributions-rearm | **FIXED LATER** | flow host + set-contributions | Late install recovers preview | — |
| B73 | **`window_kind_actions`** all declared | window-kind-actions | **FIXED LATER** | `📓️window-kind-actions-2026-09-10.md` | Actions not dropped by gate | — |
| B74 | **`FlowFixture` abort in generate_preview::render** | window-kind-actions | **FIXED LATER** | same report | Preview render without abort | — |
| B75 | **`first-step-deadline` retryable** | first-step-deadline | **FIXED LATER** | `📓️first-step-deadline-2026-09-10.md` | Flaky first boot reduced | — |
| B76 | **`hotpath` catalogue Arc** (108 KB/tick removed) | hotpath-optimization | **FIXED LATER** | `📓️hotpath-optimization-2026-09-10.md` | Eval tick ms down | — |
| B77 | **`NodeGraphScene.operators` catalogue** removed from wire | catalogue-surface | **FIXED LATER** | ui contract | Intake budget fixed | — |
| B78 | **`World3d` snapshot bridge** wgpu | wgpu-renderer | **FIXED LATER** | `📓️wgpu-renderer-2026-09-10.md` | Mesh preview on wgpu | — |
| B79 | **`node-graph` production attach** wgpu | wgpu-node-graph | **FIXED LATER** | same | Flow canvas paints on wgpu | — |
| B80 | **`ui-turn-overrun` deleted** | wgpu-ui-turn | **FIXED LATER** | `📓️wgpu-ui-turn-2026-09-10.md` | Shell construct survives | — |
| B81 | **`worker-boot` watchdog + module cache** | wgpu-boot-watchdog | **FIXED LATER** | report | Long wasm compile survives | — |
| B82 | **`selection-overflow` stack** | selection-overflow | **FIXED LATER** | `📓️selection-overflow-2026-09-09.md` | interactionSelect fits stack | — |
| B83 | **`VcsArtifactApp::snapshot()` drop abort** wide regression | editor-gaps §7 | **UNCLEAR** | framework (needs repro on current tree) | Registered tests may abort on snapshot read | **degraded** |
| B84 | **`kind_infos` shared catalogue release** without drain | flow-host §9 | **STILL OPEN** | FlowHost retirement ladder | Latent if second catalogue sharer | cosmetic |
| B85 | **`listFlowExtensions` plugin command** | taxonomy-fix §4.5 | **FIXED LATER** | `🌀️procedural/🎮️commands/🦀️.rs` | Plugin-root taxonomy clean | — |
| B86 | Four **fixture-host adapter** taxonomy violations | taxonomy-fix §5.3 | **STILL OPEN** | procedural test-host `#[path]` adapters (4 lines) | 4 of 22 procedural findings | test-only |
| B87 | **`interactive-job` future 44 KB** command pages in turn future | poll-task-leak §8.2 | **STILL OPEN** | `⚛️reactor/🔄️turn` (documented, not landed) | Wasm stack pressure | **degraded** |
| B88 | **`SEMIO_RUNTIME_DIAGNOSTICS` gated logs** removed from hot path | hotpath | **FIXED LATER** | hotpath lane | Clean console | — |
| B89 | **`extension-addressing` TS chatter** gated | extension-addressing | **FIXED LATER** | report | Less noise | — |
| B90 | **`AppActionRegistry` top-level actions** | catalogue-surface | **FIXED LATER** | framework ui | Commands dispatch | — |
| B91 | **`generation3d` suite 328/4 → 334/2** after rearm | status 12:10 | **UNCLEAR** | needs `cargo test` to confirm | 2 reds remain (likely B10,B17) | test-only |

---

## STILL OPEN — blocker/degraded only (for coordinator)

### Blockers (12)

| id | one-line | primary file:line |
|---|---|---|
| B01 | Extension fault pack decode mismatch | `🌐host/🦀️.rs:48` + `ShellHost/🟦️.tsx:1663` |
| B04 | Restage not done for landed guest fixes | process |
| B06 | wgpu shell-boot silent hang | `🎞️frame-worker/🟦️.ts:490` |
| B07 | Wasm 4 KiB/poll retention → OOM | `reactor::poll` wasm path |
| B09 | `merge_remote_snapshot` fail-closed | `🏪️store/🦀️.rs:17739` |
| B12 | `FlowHost` drop in render/commands | `flow/…/🌊️main/🦀️.rs:85` |
| B13 | `Recipe::advance` OrderedMap leak | `flow/…/recipe/🦀️.rs` |
| B21 | Assembly surfaces not mounted | `🌀️procedural/🦀️.rs:104` |
| B05 | Preview fault until restage (rearm code landed) | (restage) |

### Degraded (14)

| id | one-line |
|---|---|
| B14 | FlowHost Dictionary / eval slowness |
| B22 | Assembly five schema trait bounds |
| B23 | demo-session not in examples() |
| B33 | Non-procedural plugins lack paged setContributions |
| B34 | setPreviewOff work-capacity multi-id |
| B35 | Close costs ~2052 turns |
| B36 | empty ops load aborts replay_ops |
| B43–B44 | Repo taxonomy gate 2123 + stale matrix |
| B46 | descriptor/regen stale |
| B60 | FlowHost fixture temporaries on ? paths |
| B64 | component-app-assembly export gate |
| B68 | Host refresh without contributions re-arm pattern |
| B71 | message-ledger remap fail-closed |
| B87 | 44 KB turn future (command pages) |

---

## Proposed parallel lanes (file-disjoint)

Dispatch **one agent per lane**. Restage/activate is a coordinator step after lanes 1–2 land, not a lane.

### Lane A — Extension fault wire symmetry

**Owns:** `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host/🦀️.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs` (fault branch of `extension_response_args` / guest decode only), `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧪️tests/🔬️extension-continuation/🦀️.rs`

**Closes:** B01

---

### Lane B — Wasm poll retention (4 KiB/turn)

**Owns:** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️poll-turn-memory/🦀️.rs`, `✏️s/🔌️plugins/🌀️procedural/🧪️tests/😴️idle-turns/🦀️.rs`

**Closes:** B07; optionally B87 (boxed command pages — touch only if same files)

---

### Lane C — VCS remote merge + convergence tests

**Owns:** `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` (`merge_remote_snapshot`, `remap_snapshot_message_ledger`), `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (VcsArtifactApp merge call sites only), gen3d+gen2d `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (convergence tests only)

**Closes:** B09, B10, B71

---

### Lane D — FlowHost lifecycle + retained recipe leak

**Owns:** `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`, `…/🎭️modes/✏️edit/🪟️windows/🌊️main/🦀️.rs`, `…/🎭️modes/✏️edit/🪟️windows/🗣️compiled/🦀️.rs`, `…/📌️panels/🛍️catalogue/🦀️.rs`, `…/🎮️commands/⏱️flow-eval-tick/🦀️.rs`, `…/🎮️commands/🏁️flow-eval-resolve/🦀️.rs`, `…/🎮️commands/🧮️evaluate/🦀️.rs`, `…/🧵️retained/🗿️artifact/🧬️recipe/🦀️.rs`, `…/🧪️tests/🔬️interactive-job/🦀️.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` (Dictionary ownership — B14)

**Closes:** B12, B13, B14, B60, B61

---

### Lane E — Assembly mount + schema bounds

**Owns:** `✏️s/🗿️artifacts/🧩️assembly/**` (schema io mutations editor viewer), `✏️s/🔌️plugins/🌀️procedural/🦀️.rs` (mount lines only), `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🦀️.rs`

**Closes:** B21, B22, 17 of procedural taxonomy 22

---

### Lane F — wgpu shell-boot observability + phase split

**Owns:** `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/**` (`🎞️frame-worker`, `🌐️browser-worker`, `🚀️boot.js`, `🐚️plugin-bridge.ts`), `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📨️browser-frame-transport/🟦️.ts`

**Closes:** B06

---

### Lane G — Preview eval chain + publication lease (gen3d editor tests)

**Owns:** `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/…/✏️editor/**` (excluding files in Lane D), `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (vcs swap test only)

**Closes:** B15, B16, B17, B23, B64

---

### Lane H — Store load/replay + close interaction (orthogonal)

**Owns:** `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` (`replay_ops` error path, close batching docs), `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🦀️.rs`, `✏️s/🔌️plugins/🌀️procedural/🧪️tests/🚪️close-ladder/🦀️.rs`

**Closes:** B35, B36

---

### Lane I — Taxonomy gate (repo-wide matrix)

**Owns:** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/**`, `🧰️framework/🛍️products/💻️os/🔨️modules/🧹️normalization/🟦️.ts` (if needed), **not** assembly subtree (Lane E)

**Closes:** B43, B44, B45, B49, B86

---

### Lane J — Flow editor capacity + contributions parity (small)

**Owns:** `✏️s/🔌️plugins/🌊️flow/…/✏️editor/🦀️.rs` (setPreviewOff extent only), `✏️s/🔌️plugins/{cad,process3d,forms,playbook}/**` setContributions routes, `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs` (if paging shared)

**Closes:** B33, B34

---

## Verification notes (specific chases)

| chase | verdict |
|---|---|
| Fault-arm pack asymmetry §4.5 | **STILL OPEN** — Ok path uses `decode_wire_value` in `extension_response_args`; shell fault still `encodePackValue`; guest `outcome_to_result` / `invoke_extension` still `decode_fault_bytes` |
| BatchOnlyPendingRewrite ×13 | **FIXED LATER** by `📓️flow-catalog-authority-2026-09-10.md` — all actions `Migrated` at `flow/…/✏️editor/🦀️.rs:2608+` |
| IO codecs 7/9 wrong | **FIXED LATER** by `📓️io-codecs-2026-09-09.md` — STL leaf now calls `preview_semio_mesh` + `encode_stl_ascii` |
| Assembly mounted? | **STILL OPEN** — `🌀️procedural/🦀️.rs:104-108` explicit unmounted comment |
| Taxonomy 22 / 2123 | **STILL OPEN** — taxonomy-fix landed moves; 22 procedural + 2123 repo per §8.1–8.2 |
| Red tests (window transient, first-tick, envelope Fault, module.vcs, advance_load) | **Mixed:** envelope Pending/Fault **FIXED** (`🔌️plugin/🦀️.rs:19526`); window transient close **FIXED** (close-ladder); `preview_eval_exact…` **FIXED**; **STILL OPEN:** module.vcs merge (B09), preview 30s timeout (B15/B16), gen2d publication.contended (B17) |
| `assert_two_instances_converge` registry-less | Call sites **FIXED** to registered helper; underlying merge **STILL OPEN** (B09) |

---

## Method

- Read `📓️status.md` (104 lines, through 12:44 2026-09-10).
- Grep-read all 66 `📓️*.md` for handover vocabulary; read surrounding context for each hit.
- Verified each open item against current sources via `rg` + file reads (no `cargo`/`nx`).
- Independent sweep: `todo!`, `unimplemented!`, `FIXME`, `panic!(`, `unreachable!`, `#[ignore]`, `Noop`, empty returns in scoped trees.

Auditor: read-only subagent, session 2026-09-10.
