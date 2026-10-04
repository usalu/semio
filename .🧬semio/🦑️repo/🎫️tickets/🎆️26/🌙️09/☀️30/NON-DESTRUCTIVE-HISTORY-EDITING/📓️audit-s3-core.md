# 📓️ Audit S3-CORE — session-3 time-travel core (W2A, W1G, W1E, CLOSURE)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`. Reviewer S3-AUDIT-CORE (Sonnet), READ-ONLY. First pass 10-02 19:0x–19:45, resumed
10-03 10:43 after the usage cut. Evidence = the tree as read at 10-03 ~11:00 (files are edited by peers every few minutes: **cite by function
name, line numbers drift**). No cargo/bun/nx was run by me; every "verified" below means "a report says it ran green AND I read the code/law";
every "owed" means nobody has run it since the last edit. Binding references: `AGENTS.md`, `📋️design.md` §3 §4 §7 §10 §15 §16 §19 §20.

Aliases: `OS`=`🧰️framework/🛍️products/💻️os/🔨️modules`, `P`=`OS/🔌️plugin/🦀️.rs`, `TT`=`OS/🔌️plugin/⏪️time-travel/🦀️.rs`,
`ST`=`OS/🏪️store/🦀️.rs`, `TM`=`OS/🔌️plugin/🛠️tool-machine/🦀️.rs`, `RX`=`OS/🔌️plugin/⚛️reactor`, `UI`=`🧰️framework/🔨️modules/🖱️ui`,
`K`=`🧰️framework/🔨️modules/🎠️kernel`, `MF`=`🧰️framework/🔨️modules/🛂️manifest`.

## 0. Verdicts

| WP | Verdict | Critical | Major | Minor |
|---|---|---|---|---|
| S3-W2A runtime (§19.1, N1, N15, N3, N17, stepped load, cold-pair, pure-head, F7 ask, optionSource editor) | **accept with changes** | 0 | 6 | 8 |
| S3-W1G store (§15, N17 local steps, per-viewer oracle, initializer, probes) | **accept with changes** | 0 | 3 | 7 |
| S3-W1E UI (facets, N2, tree a11y, disclosure, F7 contract, optionSource) | **accept with changes** | 0 | 0 | 7 |
| S3-CLOSURE (amend/coalesce deletion, derived footprints, gate) | **reject as "done"; design and landed steps accepted with changes** | 0 | 3 | 2 |

No critical: I found no path that loses committed history, breaks the Supersede fold, or violates zero-trace on abort/cancel. The majors are
(1) liveness/race in new code, (2) O(history) work per mutation, (3) an unenforced binding clause, (4) work still not finished.

Positive confirmations (read, not assumed): §15 laws are real (abort compares projection, revision, ledger, messages, log, pack+text, sequence,
announcements); `begin_refusal` is mirrored Rust↔TS and fixture-driven over every context; `history.replaying`, `document.loading`,
`timeTravel.busy` and the 18 `timeTravel.*` codes are en/de in Rust, TS, fixture and both shells; `HistoryReprojection.local` → `kind` rename is
consistent across Rust, TS, schema and fixture (valid + invalid case) with no compat field; labels survive reload (§19.1 law covers authoring,
text reload, pack reload; verbs through `Edit.verb`); no `legacy|compat|deprecated|TODO` in S3 core code; `RowAction.reason` consistent across
Rust contract, typed catalog (`5 => reason`), TS decoder, Interpreter, wgpu ARIA mirror; new public APIs export in-repo types only.

## 1. S3-W2A — runtime

**Verdict: accept with changes.** Design clauses: §19.1 ok, N1 ok with caveats, N15 ok, N3 ok, N17 runtime half ok, stepped load/cold-pair
incomplete (see W2A-4/5/6).

- **W2A-1 [major] O(history) work per mutation and per history render (§3.6 "no whole-history on the hot path").**
  `refresh_cache` rebuilds the whole `HistoryView` on every store generation (comment above it: "every completion actually pays");
  `build_history_view` calls `store.mutation_ops()` + `mutation_outcomes()` (`P` ≈27540) and rebuilds `labelled`/`printed`/`ops_by_edit` maps over
  all rows; the streamed-tick arm calls `history_edit_mutation_views` (`TT` ≈1872) which re-reads the whole document per tick; N1 added a third
  consumer, `history_mutation_pages` (`TT` ≈1886, `mutation_ops()` ≈1906), on every render of a row that opens by default. The store half is
  W1G-3. The 64-edit ceiling is gone (`a_history_past_one_page_keeps_admitting_edits` admits 80, the reload law 240), so this is live.
  Fix: incremental `HistoryView` (patch the tail edit / dirty rows; reuse previous rows for untouched edits), `history_mutation_pages` reads only
  the wanted edit's slice via a store accessor (W1G-3). Owner: S3-W2A (+W1G-3).
- **W2A-2 [major] §20.1 L4 "config-lane edits never appear as history rows" is not implemented and its law is vacuous.**
  `history_row_is_recorded(kind, edited, inverse)` (`P` ≈12077) records every `View` with a store edit; `record_command` passes
  `edit_id.is_some() || config_edit_id.is_some()` (`P` ≈27303) and `record_typed_operation_lane(.., artifact_lane=false)` (`P` ≈26626) files an
  app-config release as a row (only a *window-config* View is silent). Existing laws pin the opposite
  (`config_lane_row_reports_itself_applied_so_the_host_can_count_it`, `an_op_less_view_action_is_logged_with_edit_id_none_and_count_one`,
  runtime-contract tests). The new law the closure report cites for L4, `a_config_press_is_one_config_edit_a_cancel_is_none_and_neither_is_a_history_row`
  (energy, `✏️s/🔌️plugins/🔋️energy/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` ≈523), asserts `transaction_rows(app).len() == rows` where
  `transaction_rows` filters `entry.edit_id.is_some()` (≈377): a config-only row has `edit_id: None`, so the law passes whether or not a row
  exists. Consequence after D1: every settings/cursor release is a row with raw-action-id fallback label (`record_command`:
  `LocalizedLabel::data(action_id)`, same text in en and de) and counts in `#s-checkin`'s uncommitted count.
  Fix: decide L4 once (either `history_row_is_recorded` drops config-only edits and the three laws invert, or amend §20.1), then make the energy
  law count ALL rows (`history_snapshot().upserts.len()`). Owner: coordinator decision; S3-W2A (predicate) + S3-CLOSURE (law).
- **W2A-3 [major] N17 deferred reprojection budget is operation-count only.** `TIME_TRAVEL_REPLAY_OPERATIONS = 256` (`TT` 31) feeds
  `defer_remote_replays/defer_local_replays` (`P` ≈24824, ≈25836); the store yields when `left == 0` (`ST` `advance_reprojection`, `let budget`
  ≈20346). The session's replay of the same Report stepper uses a 4 ms wall budget (`TIME_TRAVEL_TURN_WALL_US`, `TT` 27, used at ≈2638/≈3036).
  Two budget semantics for one mechanism; one CAD/brep/raster leaf costs ms, so 256 ops can stall the reactor for seconds, and the first budget
  runs synchronously INSIDE the dispatch (`defer_local_step`). The N17 law only asserts `≤ 256` ops per turn. Violates "≤ 4 ms slices" (§7).
  Fix: `step_reprojection(deadline)` taking the wall deadline (op cap secondary), runtime passes the turn deadline; law with a slow counted op.
  Owner: S3-W1G (signature) + S3-W2A (driver).
- **W2A-4 [major] Cold-pair stepped load: one global load slot for a per-instance ingress registry.** `COLD_PAIR_DOCUMENT_LOAD` is a single
  `Option` (`RX/🦀️.rs` ≈134) while `ColdDocumentPairIngressRegistry<N>` is slot-per-instance (`RX/📥️cold-pair/🦀️.rs` ≈175-290). A terminal page of
  instance B while instance A's load is stepping runs `begin_load(B)` and overwrites A's pending (`RX/🔄️turn/🦀️.rs` ≈1044); `supersede_cold_pair_document_load`
  only cancels a stale load of the SAME instance (≈1637). A's `ColdDocumentPairLoad` drops (→ `Faulted`) while A's archive load keeps running
  with nobody to poll/acknowledge it, so A stays `document.loading` (everything but view verbs refused) until closed; A's host transfer throws
  "invalid loading receipt". Reachable when two instances of one plugin restore/ingest concurrently. The turn response carries one
  `ColdPairIngressStatus`, so a second concurrent load cannot be answered either. Fix: refuse B's terminal page with `Backpressure` while any load
  is pending (cheapest) or key the slot by lifetime and answer per instance; law: two instances, interleaved pages. Owner: S3-W2A.
- **W2A-5 [major] Publication-gate fix (`PUBLICATION_RETURNED_ROOT_ALLOWANCE = 1`) is unmeasured and only amortises the O(document) drain.**
  The gate (`P` ≈31610) drains ALL returned roots (256 items × 4096 steps) whenever more than one is outstanding; the maintenance rotation
  retires one item per stage per turn (doc of `PUBLICATION_SNAPSHOT_READ_RECLAIM_ITEMS`). In a sustained one-item stream (drag ticks) the second
  publication after a large root still sees >1 returned root and pays the full-document drain on its critical path; only isolated edits become
  size-independent. Report says the confirming run (`b54_measures…`, store≈17 on Concrete Forest and Nakagin) is still owed. Fix: measure
  streamed ticks, not one edit; if >1 root persists, retire returned roots by size-bounded steps per publication (grant ∝ bytes published) instead of a
  drain-to-zero; keep the 816-op fill OOM law. Owner: S3-W1G (measure) + S3-W2A (gate).
- **W2A-6 [major] N17 reload half (G9) is not closed: synchronous whole-document loads remain.** `AppCommand::LoadDocument` still has 17 references;
  `load_document_pack/text` 118 call sites in 60 files; `parse_document_pack` folds history before any initializer runs; MCP workspace and
  `🏃️run` senders still use the sync command; `PluginApp::load_document_*` is reached through `resolve_ready` which panics on suspension
  (report §9.3, §9.4 open items). Only archive load, cold-pair and checkpoint restore are stepped. Fix as planned in
  `📓️api-stepped-document-load.md` §8 (senders → archive admit/poll/ack; delete `LoadDocument` in the bump wave; drop `load_document_*` from
  `PluginApp`). Owner: S3-W2A + coordinator (bump wave).
- **W2A-7 [minor] Up to three whole-document `to_value` per history render.** `resolve_time_travel_reference_labels` (`TT` ≈2005) converts the previewed
  snapshot and walks ≤262 144 values (ids that never resolve always pay the full walk); `resolve_time_travel_snaps` and the new
  `resolve_time_travel_options` share `time_travel_previewed_value` (≈1947) but each call builds its own value; labels do not share it.
  Runs on every draft keystroke and progress refresh. Fix: one memoised previewed value per render, keyed by (session generation, preview
  revision), cached names per id set. Owner: S3-W2A.
- **W2A-8 [minor] Reprojection status can lie.** `reprojection_paused` is cleared only on Rerun/close (`TT` ≈2135, 392), so a paused remote change
  that a local step adopts leaves the flag set and the NEXT remote change shows "paused" and is never driven; `reprojection_fault` is cleared only
  by the next successful step/discard (`TT` ≈2097), so a refused step's banner persists with no dismiss control (controls need `total > 0`).
  `reason(code)` falls back to the RAW code for any vcs fault (`TT` ≈3544; `history.full`, `history.shape`, …) instead of `HISTORY_NOTICE_LABELS`
  (`K/🦀️.rs` `history_notice`); Load kind reuses `LocalReplayRefused` copy and a `cloud-download` icon. Fix: clear pause when no progress; dismiss
  on next dispatch; localized fallback. Owner: S3-W2A.
- **W2A-9 [minor] Edit is disabled only for the reducer's refusals.** `TimeTravelPanel.begin_refusal` (`TT` ≈954) cannot see the runtime's
  `timeTravel.busy` (open tool transaction, waiting local step, authoring: `TT` ≈2396), so Edit looks enabled and activation answers a notice.
  Fix: add `Busy` to the panel's refusal. Owner: S3-W2A. (Visible reason: W1E-1.)
- **W2A-10 [minor] Slicing mirror can hard-fail the whole history body.** `history_row_window_rows` (`TT` ≈1635) re-implements the `TreeWindows` cut;
  a miss in `ui_history_panel` returns `history-panel.mutation-page` (`P` ≈12328) and the body fails. It ignores the shared first-paint budget
  (over-fetches only), no law cross-checks the two. Fix: derive the slice from `tree_window_indexed_item` (callback) or render a placeholder row on a
  miss; property law over (request,total,viewport). Owner: S3-W2A.
- **W2A-11 [minor] N1 caps skew severity and leave members unreachable.** `HistoryEntry.worst` and the row tone are the max over the PROJECTED rows
  (`P` ≈27705; cap 32 + ≤32 flagged, `TT` ≈1545): an Error beyond the 64th flagged op is under-reported. Composed-member rows past a member's
  projection are not paged (reported open item), so "every mutation reachable" (design §12) is incomplete for member stores. Fix: rank flagged
  by severity before the cap or compute `worst` from outcomes in O(edit); page member rows. Owner: S3-W2A.
- **W2A-12 [minor] `tool_intent_kinds(tool)` makes every adopter parse `<appId>#<toolId>`.** generation3d does
  `tool.strip_prefix(GENERATION3D_EDITOR_APP_ID)` (`✏️s/…/generation3d/…/✏️editor/🦀️.rs` ≈1959); its law builds the string from the same constant, so
  a change of the stamped format yields silent first-leaf labels. Fix: pass the tool id part, add a runtime law that a real machine's
  `TransactionRef.tool` resolves. Owner: S3-W2A + S3-PROCEDURAL.
- **W2A-13 [minor] `document_loading_refusal` (`P` ≈30277) exempts only `View` verbs.** `Interaction` verbs (hover/pick) are refused with a fault while a
  load runs (notice spam during a restore); fault text is English-only (notice is localized by code, fine). The history route emits
  `history_changed_event` at admission of a deferred step (`P` ≈30084) and not at adoption. Also `if action==CANCEL_TYPED…{return …}` is unformatted
  (rustfmt/"concise" rule). Fix: exempt `Interaction`; emit on adoption. Owner: S3-W2A.
- **W2A-14 [minor] RFC 6901 index aliases.** `time_travel_pointer_*` use `segment.parse::<usize>()`, which accepts `01` and `+1`; two pointers address one
  item (W2A noted `01`). Fix: require `segment == position.to_string()`. Owner: S3-W2A (W1E regions).

Owed runs (no run after 06:29; tree red from the peer DSL/ValueError sweep): plugin lib `-- time_travel …` (N1/N15/N3/§19.1/N17/pure-head/N2 list
law/optionSource law), puzzle 2d `history_edit_runtime_tests`, kernel `history_edit_actions`, wasm check of plugin+puzzle. Green so far: kernel
`history_patch history_notices history_edit` 10/10 (06:04), `semio-framework-time-travel` 14/14 + FWT TS 20/20, plugin lib native+wasip2 check 06:29.

## 2. S3-W1G — store

**Verdict: accept with changes.** §15 and N17 local steps match the design; the gaps are leftovers, duplicates and missing laws.

- **W1G-1 [major] `[DEBUG]` probes still in hot paths; bisect never concluded.** Six `SEMIO_DEBUG_STORE_UNITS` sites: `ST` ≈17939 (`maintenance_retirements_step`),
  ≈18034 (`take_returned_snapshot_read_retirement`), ≈18727 (`begin_typed_apply_batch`), ≈18932 (`advance_apply_batch`, every phase of every publication),
  and two in `ReturnedSnapshotReadRetirement` (≈1732, ≈1745). `std::env::var_os` allocates an `OsString` per call in the publication ladder; AGENTS: temporary
  logs must be removed. The O(document) regression (store=22 vs 1590 units) is still hypothesis + an unmeasured fix (W2A-5). Fix: run the bisect once
  (blocked by the peer red), delete all six probes. Owner: S3-W1G. Blocks ticket close.
- **W1G-2 [major] The quadratic initializer survives in 8 plugin-owned copies.** The framework initializer is linear now (`ValidateEdit{index}` +
  `edit_index`, `P` ≈17267), but `ValidateEditPair{left,right}` (N²/2 steps; 50 M at 10 k edits by W1G's own count) is still hand-written in writer,
  generation2d, generation3d, gismap, process3d, jack, drawing, raster (`git grep ValidateEditPair ✏️s`: 8 files × 5), and the coverage gate pins the old
  phase names (`OS/🔌️plugin/🧪️tests/🔬️tool-job-coverage/🟦️.ts` ≈713, 782, 879, 939, 1409). "0 `ValidateEditPair` left" in the report is true for the
  framework only. Duplicated mechanism (AGENTS: repeated code must be close). Fix: one framework `ValidateEdit` helper/phase used by all, delete the 8
  copies, update the gate. Owner: S3-W1G (helper) + plugin owners; coordinator schedules.
- **W1G-3 [major] Store accessors are O(E²) + O(ops) allocations per call.** `mutation_ops` (`ST` ≈18490), `durable_outcomes` (≈18517),
  `locate_mutation` (≈18540) do `edits.iter().find(|e| e.id == id)` for every applied edit (no id index; the paged ledger iterates pages) and build
  `mutation_ids_for_edit` strings for every op; `durable_outcomes` also filters all messages per op. `fold_event_log` (≈22363) does the same for every
  edit on every command-path `reproject()` (each `Apply` via dispatch). Fix: iterate edits by applied index (zip applied ids with the ledger cursor) or
  give the ledger an id index; `applied_ops_of_edit(edit_id)` and `edit_outcomes(edit_id)` accessors for the runtime (W2A-1). Owner: S3-W1G.
- **W1G-4 [minor] One-step O(history) work inside the "bounded" initializer, and a law blind to it.** `FoldSupersessions` runs
  `fold_envelope_history` (O(edits·ops), an id string per op) in a single step (`P` ≈17283); `CloneInitial` sums `applied_operation_count` over all
  applied edits (≈17135); `edit_index` (HashMap<String,usize>) is dropped bare at the end of `FindRedo` (≈17416). The reload law counts `Mutation::diff`
  calls only, so it cannot see non-fold O(history) steps. Fix: slice by edit index with fuel = ops touched; extend the counter to count ids allocated.
  Owner: S3-W1G.
- **W1G-5 [minor] §15 law gaps.** `an_aborted_transaction_leaves_no_trace_anywhere` (`OS/🏪️store/🧪️tests/🧪️tool-transaction/🦀️.rs` ≈237) never compares
  `redo_edit_ids`/cursor/`tail_undo_cache`, although the batched first publish clears redo (`replace_redo_edit_ids_retained(Vec::new())`) and abort
  relies on `reproject_now` re-deriving it; no law for remote ingest under an open transaction followed by an abort (`lift/restore` reset the tail cache;
  correct by reading, untested); none for a transaction that streams while a local step waits (`AppendTransaction` is not `moves_history`, each tick
  restarts the replay, a drag can starve the step). Fix: three scenarios. Owner: S3-W1G.
- **W1G-6 [minor] N17 has Rust-only laws.** No language-agnostic fixture and no third-party oracle for deferred remote/local steps (AGENTS: one of each per
  feature); the TS store twin has no counterpart (`git grep deferRemoteReplays|discardLocalStep` in os TS: none). Fix: corpus (log, step, budget →
  head/supersessions/outcomes) checked by Rust and the existing TS fold twin, fast-check over budgets. Owner: S3-W1G.
- **W1G-7 [minor] `adopt_pending` drops waiting remote transitions on a projection error.** In the `reproject*` failure arm (`ST` ≈20395+) `pending` is
  already destructured: the admitted transitions are removed from the log and from the waiting set, and the local step is lost, whereas the doc says a
  refused local step leaves its remote transitions waiting (`refuse_local_step`). `defer_local_step`'s synchronous adoption does not set
  `last_projection_cause = Replay` (only `step_reprojection` does). Fix: route that arm through `refuse_local_step`; set the cause on both. Owner: S3-W1G.
- **W1G-8 [minor] One-op edit identity depends on the route.** `commit_open_transaction` and `apply_command` call `stamp_primary_operation_identity`
  (single op → `MutationId = edit.id`, `ST` ≈12044); the retained batched close never does (ids stay `<edit>#0` from `next_edit`/staging). Same gesture,
  different `MutationId` per route; the law covers commands only. Fix: one identity rule at one place + batched assertion. Owner: S3-W1G.
- **W1G-9 [minor] Per-viewer oracle fix is right but thin.** `assert_document_text_round_trip` now proves edits/transitions survive and the text
  hydrates to the trunk tip (matches the closed PER-VIEWER design); for non-trunk viewers nothing round-trips the persisted viewer head. Fix: add the
  `.spr` `REC_VIEWER` pack round trip for viewer-head stores. Owner: S3-W1G.
- **W1G-10 [minor] Host cold-pair settle has no wall deadline.** `settleColdPairLoading` / `retryColdPairBackpressure`
  (`OS/🏪️store/👷️worker/🟦️.ts` ≈1799-1830) poll up to `1<<20` turns with no backoff; a guest stuck `loading` busy-polls for a very long time.
  Fix: wall deadline + backoff. Owner: S3-W1G. (The 7-case corpus + Ajv + fast-check law is a good pattern.)

Verified by report and read: kernel laws 92/92 (§15 ×8, deferred-remote ×4, local-step ×3, supersede-replay incl. both PER-VIEWER laws), TS store oracles
7/7, worker 17/17 incl. cold-pair-loading. Owed: reload law in `🧪️bounded-reload` (progress monotonic, steps < 16·240), plugin archive laws, hub bin, replication.

## 3. S3-W1E — UI

**Verdict: accept with changes** (every W1E law that could run is green: UI lib 769 ✔ / 2 peer ✘, contract 215+12+1, TS corpus 77, facets 13, React 78+7,
ContextMenu 11/11, mutation-inputs 90 ✔). Both renderers carry facets, N2 verbs, tree row semantics, disclosure.

- **W1E-1 [minor, a11y] The reason of a disabled tree row action is screen-reader-only.** `TreeRowActionButton` (`UI/🧱️elements/🌳️Tree/🟦️.tsx` ≈694-718)
  sets `aria-describedby` + `sr-only` text but no `title`/visible text, while the table action does (`title="label · reason"`, `OS/📺️renderer/…/🗣️Interpreter/🟦️.tsx`
  ≈2348); wgpu mirror is AT-only too. Sighted pointer/touch users see a dimmed Edit with no why (N15 intent). Fix: tooltip/focus text with the reason on
  both; extend `💬️row-semantics` with a visible-reason expectation. Owner: S3-W1E (+W2C paint).
- **W1E-2 [minor, a11y] Long-option rows show the chosen state by icon only.** `TT` ≈3647: `check`/`circle` swap on a button row, no
  pressed/selected semantics (the parent row's description names the current option, which mitigates). Fix: contract `selected` on the row, both renderers.
- **W1E-3 [minor, a11y] N17 has no live announcement.** `HistoryPatch.reprojection` is consumed by no shell code (`git grep reprojection OS/📺️renderer`:
  one wgpu test); only the session band announces. A screen-reader user pressing Undo on an interior step hears nothing until the row appears; a refusal
  banner is silent. Fix: shell status region for `patch.reprojection` (React `🛠️ShellHelpers/⏪️time-travel`, wgpu `history_lane_notice`), en/de. Owner: S3-W1E + W2C.
- **W1E-4 [minor] F7 is contract-only.** `ContextMenuItemSpec.reason` + `disabled_because(String)` exist (Rust, TS, React 11/11); the wgpu hand-painted
  menu does not paint/describe it (routed to S3-W2C), no producer uses it yet (`Menu::of` sweep not started), `reason: String` vs `RowAction.reason: Label`,
  and `reason` may be set without `disabled`. Fix: wgpu menu, sweep, assert `reason ⇒ disabled`. Owner: S3-W2C / S3-W2A.
- **W1E-5 [minor] `number_facets` lives in the neutral manifest but returns ui-contract types** (`SliderAppearance`, `UiNumberScale`, `UiNumberLimits`,
  `Label` in a `pub` struct, `MF/🦀️.rs` ≈1063-1076): a manifest→ui-contract layering inversion (in-repo, not third-party). Fix: plain numbers in the manifest, map in
  time-travel/ui. Optional. Owner: S3-W1E.
- **W1E-6 [minor] A live e2e probe predates N1.** `OS/🧑‍💻dev/🧪️tests/🧪️time-travel/🟦️.ts` ≈3476 (`g9-every-mutation-of-the-long-transaction-is-reachable`) reads N1 as
  "`HISTORY_PANEL_MUTATION_ROWS` = 8, no more row" and passes on `mutations.length >= created || more`; windows virtualise rows in the DOM. Fix: derive from
  `window.total`. Owner: S3-W3E2E.
- **W1E-7 [minor] optionSource review.** Schema-first end to end (meta-schema, vocabulary, Rust+TS reader, corpus +4 cases, Python jsonschema 207 verdicts) is
  good; walking `{field}` templates scans arrays linearly per render and rides W2A-7's per-render conversion; template pointer fill is RFC 6901-escaped. Rust
  side not compiled (red since 06:56). Owner: S3-W1E.

Owed: manifest `number_facets` law, plugin `time_travel` laws, puzzle `select_tool_history`, `ui_node_wire_format`, `exports_typescript` (stale `coalesceKey` in
`🤖️generated/🪪️manifest/🟦️.ts` until CLOSURE regenerates).

## 4. S3-CLOSURE — deletion of amend/coalesce, derived footprints

**Verdict: reject as "done"; accept steps 1–3, 5, 6a/6b, derived footprint design with changes.** Report steps 7/8 read "10-03 06:30"; gate census `amend-emit 0 ·
amend-last 0 · coalesce-key 14 · preview-contract 0 · bracket-verb 11 · host-snapshot-bracket 0 · edit-literal 0 · release-plain-commit 0 · footprint-hand 0`.

- **CLOSURE-1 [major] Not finished, and nothing enforces what is.** Still present: the `AppFrame::TransactionProposal.coalesce_key` field on 3 codecs + tests
  (`OS/📡️spr/🧵️channel/🦀️.rs`, `OS/🟦️.ts` ≈2705/3414/3584, `🧪️backbone-envelope-io`, plugin literal `P` ≈42816 sending `String::new()`) — a dead legacy wire
  field (AGENTS: no legacy); `AppCommand::LoadDocument` (17 refs, W2A-6); `Emit.description` (`P` ≈12482) + `Emit::commit(.., description)` (≈12909) with
  `authored_row_label` still turning a description into a data label (≈27323) — §20.6 breach (3 stdio `Load example` emitters + `CommitLabel` test remain);
  `bracket-verb 11` = generated descriptors not regenerated; `verify history-closure` is not in `runGate` and its CLI route is refused by a peer
  `📋️project.json`. Compile status: framework plugin lib check green 06:08–06:29; the 34+45 plugin crates and every kernel/plugin law of steps 2–7 are
  owed (list in the report). Fix: coordinator bump wave (§6), descriptor regeneration, wire gate into `runGate`. Owner: S3-CLOSURE + coordinator.
- **CLOSURE-2 [major] L4 law vacuous** — see W2A-2 (the closure's own L4 evidence is the energy law that cannot see config rows). Owner: S3-CLOSURE.
- **CLOSURE-3 [major] Config-press release can lose the final value on a refusal.** `settle_tool_operation` (`TM` ≈566) calls `settle_press_config` (hold / release,
  marks the press closed) BEFORE the fallible `scrubs.send(..)?` / `child_scrubs.send(..)?` (≈567-568); on a refusal the press is already closed and the
  release's config lanes leave with the dropped publication: slider final value lost, only a fault. Three parallel "closed press" ledgers (`ScrubLedger`,
  child `ScrubLedger`, `config_closed`) hand-implement the same captureLost / late-release rules. Fix: settle config after both sends succeed (or return a
  rollback), extract one `PressLedger`; law: refused release keeps/restores the press. Owner: S3-CONTROLS / S3-CLOSURE.
- **CLOSURE-4 [minor] Derived footprint moved the hazard rather than removing it.** `for_leaf` = `inverse_rows() + 1` (`ST` ≈15870); the 87 hand-written
  aggregates default to 1 inverse row (the old "flat 2" class, now silent); L3 only checks committed fixture cases (`inverse(before).len() <= inverse_rows()`),
  no property test at the cap edge; 62 schemas carry hand-typed `x-semio-inverse-rows`. The store enforces `forwards + inverse <= work_items` at publication
  (≈19032/19204), so an under-declaration is a fail-closed refusal, not an overrun. New ceilings (bounded 1025/768/258/4096, e.g. wfc, dag, graph, brep, mesh,
  stdio table) turn previously working cascades into refusals surfaced as raw English `plugin_sdk_fault` (`store_publication_fault`, `P` ≈24216), not a
  localized notice. Fix: property test per `perTarget` leaf (grow targets to cap+1), localized refusal code `mutation.too-large` en/de. Owner: S3-CLOSURE.
- **CLOSURE-5 [minor] `TransactionRef` ids are not unique per press.** The scrub path mints from `authoring_clock(0)` (`TM` ≈561; `OS` tool-machine crate
  `🔨️modules/🛠️tool-machine/🦀️.rs` ≈901 = wall-ms, logical 0) and `TransactionRef::mint` hashes only `(actor, hlc, tool)` (`🔨️modules/📡️replication/🎮️mutation/🦀️.rs`
  ≈1540): two commits by one actor+tool in the same millisecond, or any build where `default_now_ms()` is `None` (→ 0), share an id; ids key open-transaction
  admission and `backfill_command_log`'s parent-transaction grouping. Fix: feed a per-instance monotonic counter as `logical` (typing already uses
  `tool_machines.clock()`). Owner: S3-CONTROLS.

## 5. Cross-cutting

- **X-1 [major, process] Acceptance needs green runs, not source.** Between 06:29 and now nothing core was compiled or run (peer ValueError/DSL/IoError sweeps).
  Do not close until: plugin lib + tests (time-travel, scrub, runtime-contract streamed laws, bounded-reload, archive, cold-pair), kernel store/vcs/spr unit +
  tool-transaction + outbound-announcement + canonical-edit, manifest facets/mutation-inputs, UI wgpu corpus laws, puzzle 2d corpus, the b54 measurement.
- **X-2 [minor] Reports lag the tree.** W1G/W2A/W1E text still says `HistoryReprojection.local`; the tree is `kind: remote|step|load` (+`editCount`).
- **X-3 [note] Two paging mechanisms were avoided (tree windows only), but two *budget* mechanisms remain (W2A-3) and two *slicers* (W2A-10).**

## 6. Order of work (cheapest first)

1. W1G-1 delete probes after one bisect run; W2A-5 measurement. 2. W2A-2 decide L4. 3. W2A-4 refuse concurrent cold-pair loads. 4. W1G-3 + W2A-1
accessors/incremental history. 5. W2A-3 wall deadline. 6. W1G-2 delete 8 duplicate initializers. 7. CLOSURE bump wave + descriptors + gate in `runGate`.
