# WP-LD — Landing Window: Collaboration Store/Sync Correctness

Session 13 slice LD (landing window, normal build priority). Ports 8130–8139 / 6630–6639. Private test target
`.tmp-ticket/wp-ld/target`. Durable data `.🧬semio/🌐hub/s13-ld-*`. Inputs: `wp-ld/`. Captures `wp-ld/generated/` (expendable).
Handovers: `📓️wp-c10.md` items 3/5/6/7, `📓️wp-h9.md` row Qa, `📓️work-packages.md` "Post-`--packages all` landing window".

## Session 13

| # | Item | Status |
|---|---|---|
| 1 | Guest store re-announces an Accepted op (blocks collab-e2e STEPs 4/8/11/12/13) | **LANDED 19:56, root-fixed + law**: native law 1/1 (mutant red), vitest 2/2, native + wasm32 (wasip2, unknown-unknown `sync`) green; W3 request `ld.txt` (kernel, replication) — needs W3's rebuild to reach the live guests |
| 2 | H9 Qa opaque concurrency (`land.sh`) + field-precise follow-up | pending |
| 3 | C10 item 6 RED: B's staged submit held during a 5–20 s cut | pending |
| 4 | C10 item 5: same-field conflict loser sees localized outcome, author never keeps a refused edit | pending |

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
