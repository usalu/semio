# 📓️ Audit Core: W1-A (replication), W1-G (store core), W2-E (hub) vs `📋️design.md` §2, §3, §9, §10

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`. Read-only audit, 2026-09-30. Auditor: read-only agent (no source edit, no cargo, no sub-agents).

Aliases: `S` = `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`; `OS` = `🧰️framework/🛍️products/💻️os/🔨️modules`;
`R` = `🧰️framework/🔨️modules/📡️replication`; `TR` = `R/🔗️causal/🔀️transition/🦀️.rs`;
`P` = `OS/🔌️plugin/🦀️.rs`; `HYD` = `OS/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs`;
`DBART` = `OS/🛢️db/🗿️artifact/🦀️.rs`; `SY` = `OS/🏪️store/🔄️sync/🦀️.rs`; `HUBB` = `🌎️hub/🏗️bootstrap/🦀️.rs`.

Line numbers are as read on 2026-09-30 ~05:10; peers edit `S`, `P` and the hub concurrently, so re-anchor by symbol name.
`P` and `S` carry peer changes (W2-A owns almost all of `P`); only the retained-initializer region of `P` (~17040-17170) is W1-G's.

Method and evidence run by me:
- Read the code and the three WP reports; `git diff` for attribution.
- `python3 R/🧪️tests/🧪️history-transition/🐍️.py`: exit 0 (committed transition corpus reproduced byte for byte).
- I wrote an independent Python model of the fold law (sorted by `(hlc, id)`, alternative from last `branch`/`checkout`, last applicable supersede wins) and ran it over every `expect` step of `R/🔗️causal/🧫️fixtures/🧫️supersede-fold/🔣️.json`: all 8 expect steps match (transition ids and final alternative).
- No cargo or bun was run. Nothing below claims a test passes.

## 0. Verdict by requested check

| # | Check | Verdict |
|---|---|---|
| 1 | Fold law exactness (Rust, TS), codec validation parity | Law: PASS. Codec validation parity: PARTIAL (F-m7) |
| 2 | Effective forwards at every fold site | FAIL: one missed site (F-C1), one defensive gap (F-m2), one divergent semantics (F-M1) |
| 3 | Report-mode semantics | PASS on never-quarantine, never-abort-on-apply, no per-edit clone, keyed outcomes, cancel, ring retirement. PARTIAL: undecodable replacement aborts (F-M2), convergence splice relies on an unmaintained invariant (F-M6), rebased inverse of a failed op kept (F-m4) |
| 4 | `commit_finished_replay` atomicity/freshness, authoring refusal, alternative flow | Freshness and refusal: PASS. Alternative flow order/HLC: PASS. Install atomicity: PARTIAL (F-M3) |
| 5 | Revision accumulator, reproject supersession detection | Applied stack: PASS. Redo stack: PARTIAL (F-m1). Reproject: PASS |
| 6 | `TransactionRef` persisted/carried/stamped/restored | PASS (minor gaps F-m5) |
| 7 | Hub admission/grading/check-in/refusal | PARTIAL: check-in strictness (F-M4), payload blindness (F-M2), restamping (F-M5), small predicates (F-m8, F-m9) |
| 8 | AGENTS.md compliance | Mostly PASS: no `[DEBUG]`, no in-body comments, no compat shims, old loop gone. Nits F-m12 |

## 1. Findings, ranked

### CRITICAL

#### F-C1. The retained document hydration path is a missed fold site: it folds raw `forwards`, never sets supersessions

Where:
- `HYD:561-587` (`Phase::ValidateReplay`) and `HYD:592-616` (`Phase::ReplayCursor`).
- `HYD:302` computes `history.fold()`, whose `supersessions` are stored (`*self.fold = Some(fold)`) and then only retired.
- The `Store` target finishes at `HYD` `Phase::Finish` with `ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, generation, owners)`.
- The runtime is built by `ArtifactStoreInitializationRuntime::new` (`S:14536-14545`, `supersessions: EffectiveSupersessions::new()`); `set_supersessions` is never called on this path.
- Callers: composition member open (`OS/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:621`, `PersistedDocumentHydrationTarget::Store { generation: 0 }`) and the recursive document archive load (`P:25785`, `Target::Envelope`).

Evidence:
```
Phase::ReplayCursor => { ... if let Some(operation) = edit.forwards.get(self.operation_index) {
    let next = match crate::os_vcs::apply_mutation(current, operation) { ... Err(_) => return self.reject(MemberOpenDiagnostic::Replay) };
```
(both loops use `edit.forwards`, no `effective_operation`).

Impact:
- A member document that carries any `Supersede` opens with `current` = the fold of the ORIGINAL inputs and an empty `supersessions` map. Withdrawn ops still apply, replaced inputs are ignored.
- `supersessions()`, `mutation_ops()`, `mutation_outcomes()` and `state_before` (which overlays drafts on the empty map) are blind to persisted supersessions.
- It self-heals only at the first `reproject` (any dispatch/ingest that touches history), because `reproject` sees `fold.supersessions != store.supersessions` and replays. Until then the rendered state is wrong.
- `ValidateReplay` replays ALL edits in file order with raw forwards and rejects with `MemberOpenDiagnostic::Replay` on any apply error. A history whose superseded/withdrawn op is what makes the raw replay fail is refused at open.
- This is exactly the class the design calls "each is a divergence bug" (§3.1).

Fix:
- After `history.fold()` call `runtime.set_supersessions(fold.supersessions.clone())`.
- Replace both loops by the effective accessor: for each applied edit iterate `effective_forwards` and skip `withdrawn()` (the same 6 lines as `fold_effective_edit`, `S:21078`).
- Replace the file-order `ValidateReplay` by the applied-order effective fold, like `settle_parsed_envelope` (`S:13124`), and keep-and-record apply errors (or refuse deliberately, but identically to the other sites).
- Add a hydration test with a withdraw and a replace, asserting `hydrated.snapshot() == parse_document_pack(...).snapshot()`.

### MAJOR

#### F-M1. The retained initializer refuses documents that every other fold site keeps-and-records

Where: `P` `BoundedStoreInitializationPhase::ApplyForward`, `P:17117-17168`.
- `P:17148-17155`: `if messages.iter().any(|m| m.level == Fatal) { self.fail(b"bounded-store.initializer-fatal-mutation") }`.
- An `Err` from `diff.apply(current)` goes to `RetireFault` (`P:~17163`).
- Its supersession read itself is correct: `FoldSupersessions` phase (`P:17050`), `Withdrawn` skip, replacement decode.

Impact:
- The store's folds (`fold_operation`, `S:21110`) treat a Fatal diff message and an apply error as "record and fold as a no-op". The initializer fails the whole load.
- A `Supersede` that the hub admitted (admission does not replay) and that produces a Fatal (`mutation.duplicate-id`, `mutation.invariant`) or an apply error makes the document unopenable in the plugin runtime, while the same document opens in `.spr`/hub paths.
- The user cannot open it to fix it with another Supersede. W1-G recorded this as "pre-existing inconsistency"; it is now remote-triggerable.
- The path has no supersession test (W1-G report says so).

Fix:
- Mirror `fold_operation`: on Fatal messages or an apply `Err` treat the op as a no-op and continue. The persisted message ledger already holds the outcome. Only refuse on decode/encoding faults.
- Add a plugin-level test (`withdraw a create, keep the dependent op`) for the retained initializer.

#### F-M2. An undecodable or foreign replacement aborts every fold site, and the hub cannot see it: a single garbage `Supersede` wedges a document

Where:
- `effective_operation` `S:21054-21064`: `Mutation::decode_op(payload).map_err(... VcsError::Deserialize ...)?`, then propagated by `fold_recorded` (`S:21131`), `state_before` (`S:17804`), `EditReplay::replay_operation` (`S:21482`), `fold_effective_edit`, `replay_superseded_history` (`S:12086`, load refused).
- Remote guard: `validate_remote_supersede` `S:19381-19395` refuses schema mismatch, undecodable and non-canonical payloads, and `admit_remote_transitions` (`S:18624-18640`) turns it into `Err`, so `ingest_remote` refuses the envelope (`refuse_with_candidate_dag`).
- Hub admission `DBART:2038-2061` (`history_transition_refusal`) only decodes the transition wrapper, never the replacement payload. The hub is schema-agnostic for plugin payloads: `diff_entries` returns nothing for any schema other than the db path-map (`DBART:176-181`, called from `plan_one` at `DBART:2012`), so it cannot decode a replacement either. The same is true today of an ordinary op envelope with a garbage payload (pre-existing, out of scope, flagged).

Impact:
- Report mode is specified as "never aborts" (design §0, §3.2), yet a decode failure of any effective input aborts the whole projection.
- Any writer can send `Supersede{Input{schema:"x", payload:garbage}}`. The hub commits it (WAL; it cannot decode the payload), every replica refuses to ingest it (deterministically, forever), and `materialize_check_in` fails permanently (`replay_envelopes_onto_pair` -> `ingest_remote` -> `Err`). A reconnecting replica that replays the ledger tail hits the same refusal.
- The same failure shape for a version-skewed replacement payload on disk: the document cannot be opened.

Fix:
- Make the effective accessor total: an undecodable/non-canonical/foreign-schema replacement folds as a Fatal no-op with message code `history.supersession-undecodable` (target = mutation id), recorded in the outcome. Then `validate_remote_supersede` becomes advisory (a `Conflict` event), not a refusal.
- The hub cannot validate plugin payloads (see above), so the fix belongs to replicas and check-in: the total accessor above is the remedy. Optionally let the check-in codec report unreplayable ledger envelopes as `ledger-not-replayable` (W2-E open item) instead of failing the whole check-in.

#### F-M3. Installing a supersession is not transactional; the dry run does not check the message-ledger bound the install enforces

Where:
- `reproject` `S:18711-18714`: `adopt_history_facts(&fold)?` runs BEFORE the replay. `adopt_history_facts` (`S:19021`) only adds/renames facts (changes, checkpoints, alternatives) and never removes them.
- Failure handlers only roll back `envelope.transitions`: `install_transitions` `S:18686-18700`, `admit_remote_transitions` `S:18624-18660`.
- The fallible steps that follow: `adopt_report_replay` (`S:18888`, refuses any entry over `ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES = 4096`, `S:14781`), the `replace_*_retained` reservations.
- `dry_run` (`S:19273`) and `replay_report` never apply that bound.

Impact:
- A Report replay that yields more than 4 KiB of messages in one edit (a multi-op transaction edit whose ops all warn or error after a withdraw, ~30 messages) passes the local blocking check (warnings only), then fails at install with `ValidationFailed("...exact byte authority")`.
- For `CreateAlternativeWithSupersede` the Commit/Branch facts are already adopted; the transitions are removed. The store keeps a ghost alternative/checkpoint, and every retry fails with "alternative ... already exists" (`author_head_alternative`, `S:19239`).
- Remotely triggered, the same bound makes `ingest_remote` refuse the Supersede on every replica while the hub holds it (convergence break). Loading such a document: `replay_superseded_history` assigns `*established = entry` without the bound (`S:12086` ff.), so load and live disagree.

Fix:
- Report replays must never produce an unstorable ledger: cap/summarize deterministically in `EditReplay` (keep the first N messages plus one `info` "N more messages", identical on every replica) instead of refusing.
- Move `adopt_history_facts` after the replay adoption (or snapshot/restore the three ledgers on failure). Add the byte bound check to `dry_run` so the refusal happens before any authoring.

#### F-M4. Hub check-in refuses on intermediate states, so a document whose final state is clean can be permanently un-checkable

Where: `replay_envelopes_onto_pair` `S:10823-10836`.
```
Ok(report) if report.accepted && report.conflict.is_none() && report.worst.is_some_and(|l| Normal.rejects(l)) => { folded = Err(Rejected{..}); break; }
```
It evaluates `Normal.rejects` after EACH ingested ledger envelope and stops at the first blocking one.

Impact:
- Ledger order `[S1, ..., S2]` where `S1` (clean at its author) plus a concurrent edit makes a downstream op Error, and `S2` later fixes it, is refused at `S1` although the final state is clean. The ledger cannot be rewritten, so the document can never be checked in ("a second attempt gives an identical error", W2-E report).
- Concurrency (two editors, one superseding while the other withdraws or edits a dependency) produces exactly this. Clients only gate locally on their own view.

Fix:
- Accumulate instead of breaking: only the report of the LAST supersession-changing envelope decides (or run one final `Report` replay over the whole applied history and refuse only if that final report blocks). Intermediate Errors are messages, not refusals.
- Keep the quarantine-conflict refusal as is.

#### F-M5. The sync actor re-stamps actor and HLC on every send while the author keeps its local copy, so "last by (hlc, id)" can differ between the author and its peers

Where:
- `SY:1497-1500` `next_timestamp` ("freshly stamped on every send (this actor never round-trips a locally-authored envelope's own timestamp back in ...)").
- `relay_operations_to_hub` `SY:3457-3470`: `MutationEnvelope { actor: ActorId(socket_actor), timestamp: next_timestamp(..), ..envelope.clone() }`. `handle_ack` (`SY:3413-3450`) replaces the local copy only for `Transformed`.

Impact:
- A `Supersede`'s `EffectiveSupersession.timestamp/actor` (and thus the last-wins order among concurrent supersessions of one op, plus `Revert` ownership) is the local HLC at the author and the send-time wire HLC at every other replica. Two concurrent supersessions of one operation can therefore resolve to different winners on different replicas. `transition_id` is unaffected (`mutation_id` is preserved by `..envelope.clone()`).
- This is pre-existing infrastructure that also orders ordinary concurrent edits, so it is cross-cutting. It needs a coordinator decision; I could not find a compensating adoption of the wire stamps in the store. Confirm before relying on the law across replicas.

Fix (choose one):
- Preserve the author's `(actor, timestamp)` for history transitions on the wire (the hub grades by `observed`, not by HLC).
- Order supersessions by hub commit order (the frontier `commit_seq` every replica sees).
- Adopt the wire stamp locally on `Accepted` (replace the author's envelope HLC) for transitions.

#### F-M6. The convergence early exit and `mutation_outcomes()` rely on durable messages being replay-consistent; the store does not maintain that

Where:
- Splice: `replay_report` `S:17837-17843` and `durable_outcomes` `S:17743-17765` (messages come from `messages_for_edit`); the early exit itself `EditReplay::converge` `S:21567-21587`.
- Not refreshed: `project_applied` `S:18731-18746` and `fold_recorded` (`S:21131`) discard messages; an interior undo/redo/checkout re-projects state without rewriting `edit_messages` (W1-G report: "Stale durable messages", accepted as open item).

Impact:
- After an interior undo by author A (others' edits after it), the later edits' durable messages describe the old state. `converge()` (state equality at a prefix) then splices those stale outcomes, so `blocks_finalize` can be false where a full replay would say Error, and the history rows show "success" for ops that fold as no-ops.
- "Early exit == full replay" (design §3.2 law) holds only for logs that never went through an interior re-projection.

Fix:
- Route every multi-edit re-projection (the third `project_applied` branch, checkout, interior redo) through the same Report replay + `adopt_report_replay`, so ledger and state are always replay-consistent (n <= 64 edits: cheap). Or store a "messages fresh up to position p" watermark and never splice past it.
- Add a law: interior revert, then supersede with early exit equals the same with early exit disabled.

### MINOR

#### F-m1. Revision accumulator: the redo stack ignores supersession-only changes
`S:14435-14436`: `reconcile_stack(... &mut None, supersessions, None)` for redo (no `dirty_from`). Records are rebuilt only when ids differ; a supersession that changes the effective input of a non-last redo edit leaves stale `edit_digest`s, so two replicas holding the same event set can report different `content_revision` depending on arrival order (redo-then-supersede vs supersede-then-redo). Fix: pass `Some(0)` (or the first redo position holding a changed target) whenever `supersessions` changed.

#### F-m2. The config retained loader has the same raw-forward fold and no guard
`OS/🏪️store/🎚️config/📥️retained/🦀️.rs:225-239` (refuses history with changes/checkpoints/alternatives), `:408`, `:440` (`apply_mutation(.., edit.forwards..)`). A `Supersede` dispatched on a config store (the command is generic on `ArtifactStore`) would be ignored on load. Fix: refuse `Supersede`/`CreateAlternativeWithSupersede` for config stores and make the loader refuse a fold with non-empty `supersessions` (same pattern as line 229).

#### F-m3. Persisted-message vocabulary excludes the codes Report mode now persists
`S:12296-12326` (`expected_mutation_message_level`) allows only 7 codes; `fold_operation` (`S:21110`) turns an apply error into a Fatal message carrying `error.code`, e.g. `mutation.apply.missing-target` (`R/🎮️mutation/🗂️map/🦀️.rs:84`). Report mode records such messages durably (`adopt_report_replay`), so the next `validate_durable_history` fails with "unknown mutation message code". Rare (ops re-diff each time) but load-fatal. Fix: admit the `mutation.apply.*` family at level Fatal.

#### F-m4. Report replay keeps the inverse of an op that failed to apply
`S:21500-21522`: `let mut back = operation.inverse(base); back.reverse();` ... `self.edit_inverse.extend(back);` runs before `next` is inspected. When `fold_operation` returns `None` (apply error, folds as no-op) the rebased inverse still undoes something that was never applied. Fix: retire `back` when `next.is_none()` (Merge already does).

#### F-m5. `.spr`/`.ops` drop `observed` and `target`; `fill_supersession_targets` only runs when supersessions are non-empty
`📡️spr/📜️history/🦀️.rs` `HistoryTransitionRecord::to_envelope` sets `observed: None`, `target: Vec::new()`, `transaction: None`. `fill_supersession_targets` (`S:12118`) is called only from `replay_superseded_history`, i.e. not when every supersession is scoped to an inactive alternative. A reloaded, still-unacknowledged Supersede is re-announced ungraded (`observed: None`, whole-artifact target). Fix: persist `observed` and call `fill_supersession_targets` for every load that holds a Supersede.

#### F-m6. Validation gaps between authoring, remote ingest and load
- Load paths (`.spr`, `.ops`) do not run `validate_remote_supersede`; `effective_operation` ignores `Input.schema` (`S:21058`).
- Remote admission does not refuse `may_emit_foreign_steps` on the original or the replacement (authoring does, `S:19330-19345`; design §3.4, §9.7).
- Nothing requires the replacement to be the same leaf kind as the target, so `MutationMeta.semantic_kind/label` (stamped from the original forward) can describe a different op than the one that folds.
Fix: one `validate_supersede_input(original, replacement)` used by authoring, remote ingest and load; require equal `descriptor().semantic_kind` or restamp labels from the effective op.

#### F-m7. Schema-first / codec validation parity is partial
- `R/🔗️causal/🧬️schema/🔣️history-transition/🔣️.json` encodes only `inputs.minItems: 1` for `supersede`. Unique targets, `scope` <= 256 bytes and payload <= 256 KiB (design §2) live only in Rust (`TransitionSupersede::validate`, `TR:89-108`) and the Python malformed corpus.
- The TS test (`R/🧪️tests/🧪️history-transition/🟦️.ts`) only re-encodes accepted cases; there is no TS decoder, so the 7 malformed supersede cases (`supersede-repeated-target`, `-scope-too-long`, `-payload-too-long`, ...) are pinned by two implementations (Python producer, Rust consumer), not three.
- `Id` is `minLength: 1` in the schema while Rust accepts an empty target id (no corpus case either way).
Fix: express the ceilings in the schema (`Hex.maxLength` for payload, a strict custom keyword `x-semio-maxUtf8Bytes` for scope, `x-semio-uniqueBy: target`) with Ajv keyword implementations; add a small TS decoder that applies the same laws to every `malformed` row; add an empty-target case.

#### F-m8. A refused Supersede with no canonical checkpoint leaves the author ahead; `for_good` is coarse
- `rebootstrap_document_socket` returns `true` (socket continues) when the document has no canonical checkpoint (`HUBB:6182-6193`). The author's replica keeps the refused supersession, no compensation exists (`rollback_envelope` returns `None` for transitions, `SY:1403-1407`), and only a `Conflict` event is emitted. Fix: give the store a "retract unacknowledged local transition" (drop it from `envelope.transitions`, reproject) and call it from `handle_ack` `Rejected`; keep the rebootstrap for the checkpoint case.
- `for_good = !matches!(error, DbError::Unavailable(_))` (`HUBB:5308`) treats `Timeout`, `Io`, `Fenced`, `StaleGeneration`, `Closed` as permanent, forcing a rebuild + reconnect for transient faults. Acceptable because clients treat every `Rejected` as final today, but it should be a named predicate (`DbError::is_transient`) so the two stay in sync.

#### F-m9. Hub durable-group target predicate admits `edit#<any digits>`
`DBART:2038-2061` accepts a Supersede target when `names_durable_group_operation(target)` holds, which is true for `<durable edit>#<any position>` (`DBART:2065`). A position that does not exist makes the replicas' fold fail with "transition references unknown operation" (same poison class as F-M2). The predicate is right for dependencies, not for targets. Fix: for Supersede targets require the exact operation id list of the durable decision.

#### F-m10. Unbounded synchronous replay on remote ingest and on the command path
`replay_supersessions` (`S:18796`) and `dry_run` (`S:19273`) drive the stepper with `|| false`; `admit_remote_transitions` -> `reproject` therefore replays a whole suffix in one actor turn without progress or cancel (AGENTS: "progress and cancellation for all expensive operations"). Bounded today by the 64-edit ledger (`ARTIFACT_HISTORY_LEDGER_CAPACITY`, `OS/🌿️vcs/🦀️.rs:196`), so a nit until the compaction task lands. Fix: give `reproject` a deadline and resume state, or document the 64-edit bound as the budget.

#### F-m11. Retirement hygiene
- `evict_prefix_snapshot_reserved` (`S:18978-18985`) checks `Arc::strong_count(&entry.snapshot) > 1` and then drops; a non-atomic check-then-drop. Use `Arc::try_unwrap` (as `retire_shared_projection` does) and hand the unique owner to the factory.
- `read_supersede_inputs` (`S:13400-13415`) bare-drops already decoded `Op`s when a later input fails; `Op::retire_cold` is required for ops owning fail-closed roots.

#### F-m12. AGENTS.md nits (compliance is otherwise clean)
- Verified none: `[DEBUG]` leftovers, `dbg!`/`println!` (the hub `eprintln!` lines are pre-existing), inline `//` comments added in function bodies, `deprecat*`/`legacy`/`compat` text or shims in the added lines of `S`, `SY`, `TR`, `⚔️conflict`, `🎮️mutation`, `🔗️causal`, `DBART`, `HUBB`.
- Docstrings all start with an emoji, but not unique: the new `EffectiveForwards`+`EditReplay` regions have 57 doc blocks over 30 distinct emoji (`🧭️` x5, `🏁️` x5, `✍️` x4, `🪞️` x3, `▶️` x3, `🧊️` x3, `🔢️` x3). Same as the pre-existing habit of the file, but not the stated rule.
- The old `replay_suffix_partitioned` loop is gone (now a wrapper, `S:17603`). Four near-duplicate "fold effective ops" loops remain, not close to each other: `fold_effective_edit` (`S:21078`), `fold_recorded` (`S:21131`), `EditReplay::replay_operation` (`S:21482`) and the inline loop in `state_before` (`S:17804`, ~3.3k lines away). Reuse one prefix-fold helper. `fold_operation` (`S:21110`) restates `protocol::MutationOutcome::apply_to` (W1-G suggests returning the displaced projection from the protocol one).
- Dead so far: `EffectiveOperation::original`, `EditReplay::from_position` (0 callers in the OS modules and the hub) and `disable_convergence_early_exit` (definition only). At the time of the first read `P` had no caller of `begin_report_replay`/`commit_finished_replay`/`state_before`; W2-A has since landed `OS/🔌️plugin/⏪️time-travel/🦀️.rs`, which consumes them (`replay_edits()` at line 1083). That module is W2-A's and was not audited here.

#### F-m13. Small codec/format points
- `encode_history_transition` (`TR:276`) does not validate: `history_transition_envelope` can build an envelope the decoder refuses (empty `inputs`, repeated target). Make it return `Result` or `debug_assert!(supersede.validate().is_ok())`.
- The `.ops` `supersede` line carries the replacement as base64 `OpBinary` (`S:11732-11760`) inside an otherwise human-readable text mirror.
- The `.spr` op-meta presence byte now uses all 8 bits (`bit7` = transaction); the next field needs a second byte.
- `decode_envelope` (`R/🔗️causal/🦀️.rs`) reads the transaction id/tool with `read_str` and no `identifier-bytes` cap, unlike the exact batch decoder.
- TS tiebreak compares UTF-16 strings, Rust compares UTF-8 bytes; identical for ASCII ids (all current ids), differs for non-BMP.

#### F-m14. Fold sites without supersessions discard Fatal messages and load no longer fails closed
`settle_parsed_envelope` (`S:13124-13140`) uses `fold_history` -> `fold_effective_edit` -> `fold_operation_into`, which drops messages; with supersessions the Report replay records them. Same document, different durable outcome depending on whether a supersession exists. The removed file-order validation replay used to refuse an op that cannot apply; it now loads and silently skips it. Fix: on load, record the Fatal messages of any skipped op (or refuse), consistently with the supersession path.

## 2. Verified correct (evidence)

1. Fold law (`TR:555-684`): candidates collected in `(cmp_key, id)` order (`TR:564-579`, `TR:667-672`); after the loop `scope.is_none() || scope == fold.alternative` selects, later candidate overwrites (`TR:675-679`) = last applicable by `(hlc, id)`; unknown target = same `fold_error` as `Revert` via `owned()` (`TR:580-589`, `TR:668`); no author check; `applied/redo/refused` untouched. Independent Python model matches all 8 fixture expects. TS `foldSupersessions` (`R/🟦️.ts:1021-1054`) sorts on `(physical_ms, logical, actor, id)` like `cmp_key()` (`R/🆔️ids/🦀️.rs:181`) and `Transition` ordering.
2. Effective forwards used at: both `fold_history` (`S:17584`, `S:20977`), `materialize_document_snapshot` (`S:11545-11556`, map derived from the envelope's own fold), `project_applied` incl. the one-edit branch (`S:18731-18746`), replay engine (`S:21482`), `fold_recorded`, `state_before`, `settle_parsed_envelope` (`S:13124`, both `.spr` and `.ops`), `ArtifactStore::new`/adoption (`S:16471-16474`, `S:16719-16738`), `ingest_remote`/`accept_quarantined_events` (`S:20186-20198`, `S:20456-20464`), `replay_envelopes_onto_pair` (`S:10805`, through `ingest_remote`), retained initializer (`P:17050`, `P:17117-17160`). The hub check-in test drives a real Supersede through `materialize_check_in` (W2-E report §4).
3. Report mode: `ReplayMode::Report` -> `close_edit` `rejects = false` (`S:21533-21540`), `fold_operation` maps an apply error to a Fatal message and a no-op (`S:21110-21128`), no `P::clone` (Arc state, `S:21482-21530`), outcomes keyed by `effective.mutation_id` (`S:21526`), `begin_report_replay(&self)` mutates nothing so cancel = drop (`S:17824`; test `cancelling_a_report_replay_at_every_step_leaves_the_store_untouched`), ring entries retire through the snapshot factory or drop only a non-last alias (`S:18978`), `converge()` only checks at edit boundaries at or past `horizon` (`S:21567-21587`), `horizon` = last changed applied position + 1, only when the order is unchanged (`S:18814-18823`), and ring/tail checkpoints are filtered by the live effective prefix digest (`S:18839-18851`).
4. `commit_finished_replay` (`S:19178`): `ensure_durable_group_idle`, generation AND revision check -> `VcsError::Stale`, drafts required, drafts re-validated exactly like typed inputs (`draft_inputs`), one `author_supersession`. A blocking report refuses with `Rejected{Normal, messages}` before any state change (`S:19224-19229`). Alternative flow order: pending Commit -> Branch at head -> scoped Supersede, each envelope stamped by `clock.tick(now_ms())` on a local clock that is committed once (`S:19206-19236`, `S:19239-19262`), dependencies Branch->Commit, Supersede->targets, one outbound batch.
5. Revision: `edit_supersession_digest`/`effective_edit_digest` fold `transition_id`, kind, schema and payload into every applied edit's digest (`S:21196-21208`, `S:14369-14374`), records rebuilt from `dirty_from` (`S:14379-14432`); `reproject` compares the fold's supersession map with the store's, so supersession-only changes replay (`S:18716-18718`).
6. `TransactionRef`: `.spr` presence bit 7, dict-interned (`📡️spr/📜️history/🦀️.rs:693`, `:739-741`, `:783`); `history_op_meta_from_operation_meta`/`mutation_meta_from_history_op_meta` carry it (`S:12263`, `S:12290`); envelope binary + batch decoders carry it (`R/🔗️causal/🦀️.rs` diff); `apply_command`/`amend_command` (`S:19515-19580`), `fold_batch_item` (`S:18468`), `edit_from_operation_envelope` (`S:20814`) stamp/restore; TS mint is byte-identical to Rust (`R/🟦️.ts:977-984`, `R/🎮️mutation/🦀️.rs` `TransactionRef::mint`, vectors incl. third-party `blake3`).
7. Hub: decode failures refused, `Revert/Reinstate` ownership unchanged, `Supersede` has no ownership rule, `dependencies == targets`, unknown targets refused (`DBART:2038-2061`, call `DBART:2137`); grading by declared `target` with `db.supersede` touch kind (`DBART:276-315`, `:358-395`); reserved refusal codes; refused batch holding a transition rebuilds the author via `RebootstrapRequired` (`HUBB:5289-5310`, `:5440-5470`, `:6182-6210`; `ClientFrameStepV1` has no other exhaustive match in the crate).

## 3. Test gaps to close

- Hydration/member-open with a withdraw and a replace (F-C1). No test exists.
- Retained initializer with a supersession, including a supersession-induced Fatal (F-M1).
- Interior revert followed by a supersede with early exit vs full replay (F-M6).
- `.ops` text round trip of a `supersede` line (W1-A "written but unverified"; `S:11732-11760`, `S:13038-13045`). `parse_document_text`/`print_document_text` are not exercised by `🧪️supersede-replay`.
- Ledger message-bound overflow at install and at load (F-M3), and a garbage replacement payload at hub and check-in (F-M2).
- Intermediate-Error ledger (`[S1, S2]`) at check-in (F-M4).
- Two concurrent supersessions of one op across two replicas through the sync actor (F-M5).
- Two-replica revision equality for redo-then-supersede vs supersede-then-redo (F-m1).
- W2-E's hub crate work is documented as unverified (peer breakage); the hub bin-unit socket law and `materialize_check_in` tests still need their rerun. I read the `HUBB` diff (no exhaustiveness or ownership problems found by reading).
