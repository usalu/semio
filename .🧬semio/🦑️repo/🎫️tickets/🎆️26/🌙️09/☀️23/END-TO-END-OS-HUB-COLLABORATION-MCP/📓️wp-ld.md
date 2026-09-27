# WP-LD — Landing Window: Collaboration Store/Sync Correctness

Session 13 slice LD (landing window, normal build priority). Ports 8130–8139 / 6630–6639. Private test target
`.tmp-ticket/wp-ld/target`. Durable data `.🧬semio/🌐hub/s13-ld-*`. Inputs: `wp-ld/`. Captures `wp-ld/generated/` (expendable).
Handovers: `📓️wp-c10.md` items 3/5/6/7, `📓️wp-h9.md` row Qa, `📓️work-packages.md` "Post-`--packages all` landing window".

## Session 13

| # | Item | Status |
|---|---|---|
| 1 | Guest store re-announces an Accepted op (blocks collab-e2e STEPs 4/8/11/12/13) | **LANDED 19:56, root-fixed + law**: native law 1/1 (mutant red), vitest 2/2, native + wasm32 (wasip2, unknown-unknown `sync`) green; W3 request `ld.txt` (kernel, replication) — needs W3's rebuild to reach the live guests |
| 2 | H9 Qa opaque concurrency + field precision | **LANDED (10:0x)**: envelope `observed` (advisory) + `target` (declared fields); db grades only unseen foreign same-target writes; native + wasm32 green; laws: db 1/1, store 1/1, hub vigilant 1/1, replication 293/293, vitest 108/108 + 12/12; hubs need fresh roots (rule 23) |
| 3 | C10 item 6 RED: B's staged submit held during a 5–20 s cut | **DONE (15:2x)**: original hold does not reproduce on B3 (staged submit applied at +3.5 s of the cut); the real red was the rebuild after reconnect (socket closed every 15–20 s, B empty) → **root-fixed in the worker + law (red→green) + live 7/7** |
| 4 | C10 item 5: same-field conflict loser sees localized outcome, author never keeps a refused edit | **DONE live (15:3x)**: vigilant hub 8130, 12 s cut: de 4/5 + en 5/5 — loser sees "Änderung vom Hub abgelehnt: Jemand anderes hat gleichzeitig dieselbe Stelle geändert" / "Change refused by the hub: Someone else changed the same part at the same time", keeps no refused text, both converge on the winner; hub WAL holds only the winner's edit (+ its check-in transition, which the probe's head+1 criterion counted in the de run) |

### Log

- 19:0x read AGENTS.md, preambles 13 + 12, handovers (C10 §09:4x/10:0x/10:1x, H9 15:0x/17:0x, work-packages landing list).
- 19:1x **Item 1 root cause (measured in the code, then reproduced by the law's mutant).** Not `pending_report.outbound` (every
  batched publication resets it at `Publishing`): the browser gesture path is the OUTBOUND BATCHED publication
  (`begin_outbound_apply_batch` → `advance_apply_batch` → `flush_published_apply_batch`). A keystroke carries the typing
  coalesce key, so the second keystroke AMENDS the tail edit (`batch_amend_target`) and `flush_apply_outbound` announced the
  whole `envelope.vcs.edits.last()` via `mutation_envelope_from_edit` → `[edit-A#0, edit-B#0]` (C10's exact capture). The
  non-batched `amend_command` already announced `split_off(announced_from)`. Also wrong before: `edits.last()` is the ledger's
  last edit, not the tail APPLIED edit (after an undo they differ).
- 19:2x **Fix** (`🏪️store/🦀️.rs`, `📡️replication/🔗️causal/🦀️.rs`, `📡️spr/🦀️.rs`): the publication records `announce_from` (the tail
  edit's op count before this gesture; 0 for a fresh edit) at `Publishing`; `flush_apply_outbound(from, items)` finds the tail
  APPLIED edit, requires `forwards.len() == from + items` (else a typed validation error), encodes only `[from..]`
  (`mutation_envelopes_from_edit_since`, new; `mutation_envelope_from_edit` = `since(0)`, one id chain `edit_operation_mutation_id`)
  and sends it through `announce_operations` + `flush_outbound` (seeded once, queued, drained). `amend_command` uses the same range
  encoder (was: encode all, then `split_off`; quadratic over a long coalesced typing run).
- 19:2x **Law** (schema-first): `🏪️store/🧬️schema/📤️outbound-announcement/🔣️.json`, fixture `🏪️store/🧫️fixtures/📤️outbound-announcement/🔣️.json`
  (5 vectors / 16 steps: coalesced typing ×4, multi-item amend, key change, keyless, undo then same key); Rust
  `🏪️store/🧪️tests/📤️outbound-announcement/🦀️.rs` (mounted from the unit module; real outbound batches on a channel backbone:
  per step announced ops/transitions, ledger edits, tail ops; globally every op announced exactly once and the announced set ==
  the ledger's op ids); TS twin `💻️os/🧪️tests/📤️outbound-announcement/🟦️.ts` (Ajv admits the fixture; an independent coalescing
  reference derives every step) registered in `💻️os/🟦️.ts`.
- 19:4x native `cargo check -p semio-framework-replication -p semio-framework-os-kernel --lib --tests` EXIT 0 (`item1-check-native-1.txt`);
  law run 1 hit a peer's in-flight `DirectoryTransport::get_accepting` edit (fake transport, fixed by its owner a minute later);
  run 2 **1/1 PASS** (`item1-law-native-2.txt`). Mutation check: announcing the whole tail edit → **FAILED** at vector 1 step 1 with
  `["edit-403cee90b2b50c81#0", "edit-b0f63852754e7e58#0"]` (`item1-law-mutant.txt`) — the live defect, reproduced; restored.
- 19:5x wasm32 through the fleet mutex (`wp-ld/wasm-check.sh`): kernel `--target wasm32-wasip2` rc 0, kernel `--features sync --target
  wasm32-unknown-unknown` rc 0, replication `--target wasm32-wasip2` rc 0 (`item1-check-wasm-1.txt`). vitest `OutboundAnnouncement`
  **2/2** (`item1-vitest-1.txt`). Landing row + `wp-w3/requests/ld.txt` written.
- Note for the audit (conflict 5): the "unrelated refactor" in `📡️replication/🔗️causal/🦀️.rs` (`edit_operation_mutation_id`) is THIS
  item's change, not a peer's. H9's `store-causal-dependencies.py` targets `🏪️store/🦀️.rs`, not that file.
- Seen, not changed: composed-member lanes (`announce_member_tail_edits` → `announce_tail_edit_payload`) announce the member's whole
  `edits.last()`; member edits never coalesce (`GroupMeta.coalesce_key: None`), so they cannot re-announce today; a member's undo
  transition is not announced on its lane (flow composition; outside this slice).
- 20:0x **Item 2 re-derived (H9's `land.sh` NOT applied — unsafe as written).** Measured in the code: (1) H9's store half stamps the
  causal head into `dependencies`, which are ORDERING constraints: the hub refuses a dependency it never applied ("depends on unseen
  operation") and a peer replica's `MutationDag` holds the op `Pending` until it sees the id — a head a peer lacks (local-only
  edits, rolled-back ops) would freeze the op on every other replica; (2) it stamps only `replay_mutations` (the `dispatch` path),
  while every browser gesture goes through the batched publication (plugin SDK `begin_outbound_apply_batch`), whose app factories
  build meta with `dependencies: Vec::new()` → with the db half, a vigilant hub would refuse almost every second-author edit;
  (3) the db half judges against the frontier HEAD only, so an edit authored after seeing everything except the author's OWN latest
  write would still be "concurrent". Design (approved by the coordinator 20:1x, preamble rule 23 = fresh data roots after the
  rebuild): `MutationEnvelope.observed: Option<MutationId>` (advisory: the newest operation of ANOTHER author the author's replica had
  applied; never orders anything) + `MutationEnvelope.target: Vec<String>` (the mutation's declared `MutationKind::target()`, outermost
  first; empty = whole artifact) — the field precision. Store: `announce_operations` stamps `observed` for every locally authored op
  (batched gestures included) at authoring/announce time (`observed_foreign_operation`: last ledger edit by another actor).
  `Mutation::conflict_target()` (default empty) forwarded by `#[derive(Mutations)]` from each leaf's `target()` (2 452 leaves already
  declare one); writer's four document leaves declare their field (`text`/`id`/`uri`/`language_id`). db (`🗿️artifact`): a write is
  graded only against writes by OTHER actors committed after its `observed` in the recent commit window (+ earlier envelopes of the
  batch); conflict = shared paths (readable path-map diffs, `ConflictDetector`) or overlapping targets (shared segment; empty = whole
  artifact) → `mutation.clamped` Warning (normal: accepted + reported; vigilant: refused). History transitions never grade/conflict.
  Durable group decisions join the window as markers (`<edit>#<n>` observable) and a dependency on a durable group op is known
  (H9's `durable_group_edit_ids`). Codecs: record + batch codec (Rust + TS twin), value codec, batch limits
  (`maximum_target_segments_*`); decode capacities bounded by input length (DB1's hostile-input note, incl. `wire` `read_vec_envelope`).
- 20:1x–21:2x codemods (ticket-local, `wp-ld/opaque-concurrency/`): `envelope-fields.py` (40 Rust literals in 29 files),
  `ts-envelope-fields.py` (14 TS literals in 6 files) + worker conversions by hand, `batch-fixture.py` (fixture + schema of
  `🧮️document-backbone-batch-v1`: every vector re-encoded, 5 new cases: observed+target canonical, bad observed flag, per-envelope and
  total target-segment limits, observed identifier limit), `db-grading.py` (12 db edits). Laws written: db `⚔️concurrent-write`
  (schema + fixture 10 vectors / 27 commits + Rust law + TS twin), store `📤️outbound-announcement` extended (observed per step, remote
  op observing an unseen id applies at once — no buffering), hub bin law
  `a_vigilant_hub_refuses_a_same_target_write_authored_without_observing_the_other_authors_latest` + fixture
  `🌎️hub/🧫️fixtures/⚔️vigilant-concurrent-edit-v1` (two sessions / two socket actors).
- 21:1x vitest `OutboundAnnouncement` 2/2, `ConcurrentWrite` 2/2, `DocumentEchoSuppression` 2/2, worker `browser document actor transfers
  one verified cold pair…` PASS (C11's relay; C11 told) (`item2-vitest-1.txt`); os typecheck: 0 errors in the wire set (27 peer errors).
- 21:3x native `cargo check -p semio-framework-replication -p semio-framework-os-kernel -p semio-framework-os-kernel-db --lib --tests`
  **EXIT 0** in the landing build dir (`item2-check-native-2.txt`); two earlier attempts stalled in the jammed shared build-dir (killed,
  mine). ~21:30 usage cut.
- 05:0x resumed (rule 28): edits all present in the tree (auto-committed). Native check of the dependents launched (kernel `sync,ureq`,
  mcp, renderer wgpu, hub lib/bins/tests, plugin, plugin-host, writer; `item2-check-native-3.txt`).
- 05:1x–06:3x the rest of the wire set in one step (`wire-remainder.py`): app-channel paged envelope writer (was a third, stale
  encoder: `app_command_apply_envelopes_round_trips` red "observed flag 7"), sync message-size accounting, hub inference canonical
  command (a protocol envelope record: server-stamped approvals carry `observed` = none + whole-artifact target; decoder refuses
  others) + both hub script oracles + fixtures (`🗺️gis-inference-job-v1`, `🧾️inference-wal-proof-v1`: command hash 9727e3… → 5cf215…),
  batch fixture's observed case (own bug: expected envelope lost its observation), 3 binary `📡️wire` fixtures, 2 hand-hex TS batches
  (C11's red). Peer breaks met and left to owners: `🌱️value/🔁️codec` quote (fixed by its owner 05:04), plugin `EmitWire`/test arity,
  hub `valid_user_preference_record_v1`, hash `pbkdf2_sha256` (all fixed by owners by 06:2x).
- 05:4x native lib tests: replication **293/293**; kernel (`sync,ureq`) **1223/1230** — the 7 reds are peers' directory client
  (`admit_document_socket` Denied), open-plan fixture and one norm grammar, none envelope-related; `outbound_announcement`,
  `wire_fixtures_stay_byte_identical_across_rust_and_ts`, `app_command_apply_envelopes_round_trips`, `document_backbone_batch_fixture`
  all ok (`item2-laws-native-2.txt`). vitest os wire subset **108/108**, replication TS **12/12**.
- 06:1x wasm32 (fleet mutex): replication + kernel + plugin + writer artifact/plugin `wasip2` rc 0; kernel `sync` and renderer-wgpu
  `wasm32-unknown-unknown` rc 0 (`item2-check-wasm-1.txt`). Hub law's message check fixed (my own: `ApplyOutcome::Rejected.messages` is a
  JSON MutationMessage blob, not a list — H9's law assumed a list). ~06:35 usage cut.
- 09:5x resumed (rule 30). `cargo check -p semio-hub --bins --tests` (native lane, build-fleet-b) **green** (`item2-check-hub-3.txt`);
  hub law `a_vigilant_hub_refuses_a_same_target_write…` **1/1** (3 rows) + socket-grant law 1/1 (`item2-law-hub-1.txt`). Item-2 landing
  row + `wp-w3/requests/ld.txt` written.
- 10:2x **Item 4 (law level).** Shell refusal corpus `🛠️ShellHelpers/🧫️fixtures/⚔️hub-command-rejection` gained the row for the db's new
  opaque same-target refusal (message text, `target: ["text"]`) → `ui.conflict.hubConcurrentEdit` en + de; suite **2/2** (engine react
  package `test long ⚔️hub-command-rejection`, `item4-vitest-rejection-2.txt`). Author side (C10's): worker `rebootstraps a browser actor
  whose batch the hub refused or transformed…` + `remote operations folded over pending local ones` **2/2**. Chain now proven at law
  level: vigilant hub refuses (hub law, item 2) → localized notice (corpus) → the author's actor rebuilds from the hub (worker law).
- 10:3x **Item 3 (open).** Code read of the whole path found no link gate: Shell `BrowserActorActionMailboxV1` (FIFO) →
  `dispatchDirectBrowserActorCommand` → worker `browser-actor-action` → `dispatchAction` (`awaitUiQuiescence` → `enqueueTurn`); the
  suspended actor's checks (`documentBackboneReady`, `coldApplied`, `linkCurrent()` = current socket OR suspended) pass during a short cut,
  which matches C10's measurement that plain actions apply in 50–250 ms. What differs for a STAGED verb: opening/filling the action pane
  changes the view state → `browser-actor-view-state` → `refreshHostView` sets `viewRefresh`, and every later action waits for it in
  `awaitUiQuiescence` while the FIFO mailbox holds the submit behind it. Hypothesis: that render turn (or a turn queued before it on
  `enqueueTurn`) awaits something that only resolves after the socket returns. Not provable without a live run; no hub/serve can run
  during the rebuild. Plan once 7800 is on B3: own hub 8130 (fresh root, `OS_HUB_MERGE_POLICY=vigilant`) + C10's link proxy + serve
  6630, `[DEBUG] ld` timing at mailbox enqueue, worker receive, `awaitUiQuiescence` enter/exit, `enqueueTurn` start, `renderSurface`
  start/end; cut 15 s; fix at the root; law with a simulated cut (worker test seam: suspended link + view-state change + staged action
  must apply within the cut).
- 10:4x **H11 find: the db history walker.** A fourth hand-rolled envelope reader (`🛢️db/🗿️artifact` `HistoryEnvelopeCursor`,
  field-by-field over WAL pages) did not know `observed`/`target` → `history envelope has trailing bytes` on every new-wire record
  (H11: 5 db reds). Fixed (flag + id, count + segments, bounded like dependencies) + round-trip law
  `the_history_replays_every_committed_write_with_its_observation_and_target` (27 fixture writes → Fsync WAL → replay in order).
  Native lane: H11's 5 + new law green (`item2-db-history-2.txt`, `-3.txt`); one isolated-child flake of
  `artifact_history_empty_and_two_batch_replay_are_deterministic` (store Drop witness), 2/2 green alone. Landing row added.
  Envelope readers now covered: `causal` record/batch codec, value codec, TS twin ×2, app-channel paged writer, sync size estimate, hub
  inference canonical command (+2 TS oracles), db history walker; DB1's WAL decoder delegates to `protocol::decode_envelope`.
- 14:0x **Item 3 live (B3).** Own hub **8130** (current-tree binary `s13-w3-bin/…/os-hub` sha 962ba372…, B3 clone generation e3c0c98e…,
  fresh root `.🧬semio/🌐hub/s13-ld-hub-8130`, normal policy, hold pid 6334), serves `s` dev lane **6630** (A → 8130, pid 6480) and **6631**
  (B → link proxy **8131** → 8130, control 8132, proxy pid 6478; serve pid 6486); harness copied from C10 into `wp-ld/live/` (outputs to
  `wp-ld/generated/`), temporary `[DEBUG] ld` timing in the worker + mailbox (`wp-ld/live/ld-debug.py`, `--reverse` removes).
  Run `ldout-1` (de, 15 s real cut, note): no freeze (worst frame 15 ms), German link state, **B's staged submit applied DURING the cut
  at +3.8 s** (worker `action-in … suspended=true`, `awaitUiQuiescence` waited 0 ms, mailbox never queued behind anything), A's too,
  hub head 1→4. **The original item-3 RED does not reproduce on B3.** New RED: after the link returns B's document socket is upgraded
  and closed by the client within 10–250 ms, every ~15–20 s (hub log: `server.document.socket` upgrade → closed), B ends on an empty
  document (#93, below the baseline) while A shows both edits. Socket-close / rebootstrap / bootstrap-reject instrumentation added;
  the next run waits for the coordinator's "go" (fleet memory warning, 14:1x).
- 15:0x **Item 3 root cause (instrumented run `ldout-2`).** After the reconnect B's cut-time edit is Accepted; A's edit had been folded
  over it → `requireArtifactRebootstrap` (C10's divergence rule) closes the socket and waits for a `Welcome` carrying the canonical
  pair. The hub never sends one: `db.hello` → `decide_bootstrap` answers None/Tail (floor 0), and pairs are served only over
  `GET …/active-checkpoint/pair`. So the rebuild's first reconnect came only after the outage backoff (15 s, deadline exceeded) and
  every later `Welcome` Tail was refused ("artifact rebootstrap returned tail without a canonical pair") → close → reopen, forever;
  B's document stayed empty. **Fix** (worker, host TS): an actor-bound rebuild accepts the `Welcome` None/Tail exactly like a first
  open (`abortArtifactRebootstrap`, the new child is seeded from the pair route once `Session` admits it, then the tail); the socket
  `onclose` treats a rebuild's own close like a healthy close (reconnect at once, no outage backoff); `requireArtifactRebootstrap`
  documented. Law: schema + fixture `🔁️document-rebuild-welcome` (actor Tail/None accepted, local Tail/None refused) → worker
  `document rebuild welcome` **1/1**, mutant (fix line removed) **red** on the actor row; fold + peer-refetch neighbours 2/2; os
  typecheck **EXIT 0, 0 errors**. Live `ldout-3` (de, 15 s cut): **7/7**, converged at +37 s; B re-seeded 1.4 s after the rebuild close.
  Instrumentation removed (`ld-debug.py --reverse`, 0 `[DEBUG] ld` left).
- 15:2x **Rust twin: NOT landing.** `🔄️sync` `on_hub_frame` (both actors) has the same dead "pair inside the Welcome" rule, reached by
  every hub `RebootstrapRequired` (GIS approval checkpoints, socket lag). But the native actor cannot fetch a pair itself: its pair is
  verified and loaded into the guest by the shell (`seed_hub_document`), so accepting the tail in the actor alone would keep the
  diverged guest. The real fix spans the native/wgpu shell (re-seed the guest from `active-checkpoint/pair` on a rebuild) → routed to
  WG10/WG9 with this finding; no half-applied change.
- 15:3x **Item 4 live** on a vigilant hub 8130 (fresh root, `OS_HUB_MERGE_POLICY=vigilant` confirmed in the hub's environment; the
  normal hub stopped, its pids were mine): `ldconf-1` (de) 4/5, `ldconf-2` (en) **5/5** — see status table. The de run's only red is the
  probe's `headAfter == headBefore + 1`: the WAL holds exactly `edit-fac3…#0` (A) + `transition-9009…` (A's auto check-in), no op of B.
- 15:4x **Infra stopped** (all mine, verified by ppid): vigilant hub 8130 (hold 32016 / os-hub 32024; the normal hub hold 6334 / os-hub
  6336 was stopped at 15:1x), link proxy 8131 (6478), serves 6630 (6480 → vite 6499) and 6631 (6486 → vite 6500); ports 8130/8131/6630/6631
  free. Restart recipe: `OS_HUB_MERGE_POLICY=<normal|vigilant> zsh .tmp-ticket/wp-ld/live/c10-hub.sh 8130 <B3 catalog> <s13-w3-bin os-hub>
  <name>`; proxy `python3 .tmp-ticket/wp-w2/w2-detach.py <log> bun <abs>/wp-ld/live/c10-link-proxy.ts 8131 8130 8132`; serves
  `… zsh <abs>/wp-ld/live/serve.sh s 6630 http://127.0.0.1:8130 dev` / `… s 6631 http://127.0.0.1:8131 dev`; probes
  `wp-ld/live/probe-s12-outage.mjs`, `probe-s12-conflict.mjs` (env `S_MATRIX_HUB`, `S_MATRIX_ADMIN_FILE`, `S_CONFLICT_*`). Durable data
  `.🧬semio/🌐hub/s13-ld-*`.
