# 📓️ KN1 — `semio-framework-os-kernel --lib` gate

Slice KN1 of ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END, 2026-09-21.
Gate: `cargo test -p semio-framework-os-kernel --lib --no-fail-fast`.

**Baseline 1068 passed / 48 failed → final 1104 passed / 12 failed** (`🗑️generated/kn1-run-9.txt`).
Every run: foreground, one cargo at a time, `RUST_MIN_STACK=67108864`,
`CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-kn1` (shared build-dir untouched),
`CARGO_INCREMENTAL=0` from run 9 on.

## 0. Baseline and how to read the captures

`🗑️generated/kn1-baseline.txt` (gate form) and `kn1-baseline-serial.txt` (`--test-threads=1`) both read
**1068 / 48**, byte-identical to FL2's measurement (`📓️fl2-child-lane-replication-and-window-config.md`).

**Why the gate-form capture prints almost no panic messages.**
`🏪️store/🧪️tests/🔬️unit/🦀️.rs` (`a_panicking_body_holding_a_live_store_unwinds_instead_of_aborting_the_process`)
installs a PROCESS-WIDE silencing `std::panic::set_hook(Box::new(|_| {}))` and restores it four lines
later. Under libtest's default thread pool every other test that panics inside that window loses its
message, and one of them (`artifact_store_one_item_drop_rejects_an_unclosed_publication_owner`) even
reported a FOREIGN panic string as its own — which is what made HT12's and FL2's earlier reads of this
suite hard to bucket. Every diagnosis below was taken from the serial captures. The hook itself is a
real law and stays; the collision is recorded as a gap.

## 1. Bucket table

| run | passed | failed | landed in that run |
|---|---|---|---|
| `kn1-baseline.txt` / `kn1-baseline-serial.txt` | 1068 | **48** | — |
| `kn1-run-2.txt` | 1068 | 48 | `ArtifactStore::new` retires a rejected candidate (count unchanged: each of those tests then reached the NEXT law, which is how the ledger-drain hole surfaced) |
| `kn1-run-3.txt` | 1083 | **33** | `ArtifactEnvelope::retire_unadopted`, the `compile_dsl` codec envelope leak, 9 test-side envelope/store leaks, fixture member factories close on drop |
| `kn1-run-4.txt` | 1088 | **28** | space meta owner catalog + `SpaceHost` drain, canonical alternative order |
| `kn1-run-5.txt` | 1091 | **25** | `set_state` retires a refused reload, space checkpoint `Apply` announces again, timestamped member owners |
| `kn1-run-6.txt` | 1096 | **20** | `MutationDag::insert` cascade, laws retire their own dags, chunked `Bytes64` under a compressing pack |
| `kn1-run-7.txt` | 1100 | **16** | `.spr` carries the history lane, grammar discovery filename, `#[value(required)]`, contributed-origin key casing, two harness preconditions |
| `kn1-run-8.txt` | 1102 | **14** | backbone detach refuses before it mutates, generation counter refuses instead of overflowing |
| `kn1-run-9.txt` (gate form) | **1104** | **12** | field-decoder ticket reclamation is monotone |

## 2. Fixes — product defects

All line numbers are post-edit.

| # | file:line | defect and fix |
|---|---|---|
| 1 | `🏪️store/🦀️.rs:15347` `ArtifactStore::new` / `construct` | A REJECTED construction dropped the candidate envelope it had consumed. `ArtifactEnvelope::drop` asserts its owners were detached, so every malformed-document refusal — dangling cursor edit id, duplicated authoritative edit, alternative pinned to a checkpoint that was never recorded — **aborted the process instead of returning the `VcsError` the caller reads**. The candidate now lives in a slot the validator empties only on success; whatever is left is retired. |
| 2 | `🏪️store/🦀️.rs:2686` `ArtifactEnvelope::retire_unadopted` (new) | `into_owners()` hands back fixed ledgers (`vcs.edits`/`changes`/`checkpoints`/`alternatives`, `edit_messages`) that each carry their OWN terminal-empty `Drop` witness, so `drop(envelope.into_owners())` — the idiom everywhere — aborts for any envelope that actually has history. One drain, in the same tail-first order the store's own final-envelope transfer leaves the ledgers in. |
| 3 | `🏪️store/🦀️.rs:15584` `set_state` / `adopt_state` | Same hole on the reload path: a refused `reset(envelope)` dropped the candidate. Same slot discipline; the live store still survives a rejected reset. |
| 4 | `🏪️store/🦀️.rs:10402` `ArtifactCodec::of::compile_dsl_impl` | Leaked the whole `ParsedDocumentText` (envelope + replayed projection) on the REGISTERED document-codec path — a real leak on document compile, not a test artifact. Now mirrors `print_mirror_impl`: `into_envelope()` (which retires the replayed projection) then `retire_unadopted()`. `print_mirror_impl` moved to the same retirement, since its `drop(envelope.into_owners())` would have aborted for any document with edits. |
| 5 | `🏪️store/🦀️.rs:21016` `space_history_store_owners` (new) + `SpaceHost::new` | The dogfooded `os.space.history` meta document was constructed with NO owner catalog. `install_document_store_owners_exact`'s own doc says there is no default, and every history insertion asks for the exact mutation retirement factory — so **`SpaceHost::commit_space_checkpoint` could not record a single space checkpoint** (`edit history insertion requires its exact mutation retirement factory`), and the meta store could never reach its terminal witness. Added the catalog plus a `Drop` for `SpaceHost` that drains the meta store (it owns it outright; registered members are untouched). |
| 6 | `🏪️store/🦀️.rs:21136` `commit_space_checkpoint` | The space checkpoint's `Apply` went through `dispatch_inner`, skipping `flush_outbound`, on a W6 comment whose premise the event-sourced transition deleted (`BackboneMessage::Snapshot` → `Genesis`, identity only). After it, `CommitCheckpoint` flushes only its own transition events, so **the space checkpoint never left the process**: a second host on the same backbone folded 0 checkpoints instead of 1. Both commands now `dispatch`; `flush_outbound` drains a pending queue, so nothing is re-sent. |
| 7 | `🏪️store/🦀️.rs:20852` `SpaceHistoryDiff::apply` + `SpaceHistorySnapshot.alternatives` | `add_alternative` appended, so the inverse of a removal put the alternative back at the END — undoing a removal changed the document (`space_history_op_round_trips`: "operation inverse did not restore pre-state"). Alternatives are now held in ascending `id` order, which is both an exact inverse and the only order two replicas that branched concurrently can agree on. |
| 8 | `📡️replication/🔗️causal/🦀️.rs:487` `MutationDag::insert` | `insert` unblocked exactly ONE pending envelope where its own docstring promised a cascade. A reverse-arriving chain `c→b→a` released `b` and left `c` pending forever, so **the same closed dependency set converged to different applied sets depending on arrival order** — the core replication promise. Now a bounded cascade (each step moves one id out of the fixed-capacity pending ledger). |
| 9 | `🎒️pack/📐️format/🦀️.rs:702` `PackWriter::begin_identity_chunk` | Refused whenever the pack's codec was non-identity, which is the DEFAULT (`EncodeOptions.codec: CodecId(1)`). **No document with a `Bytes64` field past `chunk_threshold` could be encoded at all** (`encode: UnsupportedCodec(1)`). A chunk is framed with its own `flags = 0` and the chunk table indexes it by raw payload offset + content hash, so raw framing is the contract, not an accident; the guard is gone and the reason is written down. |
| 10 | `📡️spr/📜️history/🦀️.rs:129,803,870` + `🏪️store/🦀️.rs:11887,12127` | `HistoryEdit` carries no lane, so a `.pack`+`.spr` save/load **turned every `Interaction`-lane edit back into a document edit** and the reloaded store's plain `Undo` then reverted it. Added `HistoryEdit.lane: Option<String>` (presence bit 6, absent for the default `Document` lane so ordinary edits cost nothing), written from `envelope.lanes` and folded back on parse. |
| 11 | `🏪️store/🦀️.rs:18081` `ArtifactStore::detach_backbone` | Cleared the persisted backbone descriptor BEFORE `bump()`, so a refused detach (full displaced-retirement destination) reported the refusal while having already forgotten which backbone it was attached to — descriptor gone, live transport and queued payload still there. `bump` now runs first. |
| 12 | `🏪️store/🦀️.rs:18729` `ArtifactStore::bump` | `self.generation += 1` panicked at `u64::MAX` instead of refusing, and the reservation was already taken. Now a `checked_add` refusal BEFORE the reservation, so the generation is preserved on refusal. |
| 13 | `🏪️store/🦀️.rs:7658` `ArtifactEnvelopeFieldDecoderRegistry::ticket_reclaimed` | `reclaimed[slot] == ticket.generation` flipped a finished authority's own `Drop` witness back to "outstanding" the moment a sibling took the NEXT generation of the same slot — the exact ABA the per-slot generation counter exists to rule out. Now `>=` (monotone). |
| 14 | `🎯️restore-active-space-alternative/🦀️.rs:22` | `alternative_id: Option<String>` decoded a MISSING key as `None`, so "restore the active alternative to nothing" and "the sender omitted the field" were the same wire word. Marked `#[value(required)]` (the derive already supports it); the docstring's claim that the derive's missing-field rule already did this was simply wrong. |
| 15 | `🧵️canonical-edit/🧫️fixtures/🔏️canonical-edit-sealer.json` + `🧬️schema/🔣️.json` | The `contributed` `MutationOrigin` variant was `plugin_id`/`mutation_id`/`payload_hash` in the fixture and its Ajv schema while the Rust codec emits and reads `pluginId`/`mutationId`/`payloadHash` (its sibling `transaction` variant is already camelCase). Fixture and schema realigned with the codec, which is the normative encoder. |

## 3. Fixes — harness preconditions that no longer held

None of these weaken a law; each restores the precondition the law was written against.

- **Unadopted envelopes / stores.** 11 laws held a raw `ArtifactEnvelope`, a `ParsedDocumentText`, or a
  bare `super::ArtifactStore` and let it drop. Retired through the file's own
  `retire_demo_envelope` / `close_test_store` / `into_envelope()` (`🔬️unit/🦀️.rs` ×9,
  `📦️codec/🧵️send/🧪️tests/🧵️send/🦀️.rs` ×2).
- **`ArtifactStore::new` (test wrapper) now installs `P::member_store_owners()`**, so four laws that
  install their OWN catalog (`retained_demo_member_owners`, `demo_closable_store_owners`) double-installed
  and tripped `a freshly constructed member store must not carry preinstalled or terminal owner
  authority`. Switched to `ArtifactStore::bare` + a new `closes_on_drop()` that arms the drop-close once
  the law has installed its own catalog.
- **`fixture_member_factory!`** built child stores with no closer, so `dispatch_group`'s created children
  never reached their witness. They now close on drop.
- **`MemberStoreOwner<TimestampedMutation> for DemoSnapshot`** added; the two HLT space members were
  `bare` and could not record an edit at all.
- **`os_dsl::grammar`** discovery looked for `📖️component.grammar.semio`; the shipped name is
  `📖️.grammar.semio` (503 of them). Matcher widened to `*.grammar.semio` — see gap 1 for what that revealed.
- **`a_panicking_body_...`** asserted the payload downcast to `String`; `panic!("literal")` yields
  `&'static str`. Now accepts either, same exact message.
- **`protocol-laws`** `assert_op_dag_convergence` / `assert_merge_convergence` built dags and dropped them
  without retiring the applied/pending identity ledgers; added `retire_dag`.
- Removed the leftover `[DEBUG]` line in `🔗️backbone/✂️detach/🧪️tests/✂️detach/🦀️.rs`.

## 4. Gaps — the 12 reds still standing (all verified present in `kn1-run-9.txt`)

1. **`os_dsl::grammar::every_shipped_grammar_semio_parses_and_compiles` — the big one, and now honest.**
   With discovery fixed it finds 503 shipped grammars and **70 of them do not parse**: 22 `` `.grammar`
   files cannot contain a Slash token here``, 15 `expected a symbol, found LParen`, 6 `unexpected
   character '🚧'`, 6 `expected Ident, found Pipe "|"`, 4 `unexpected character '`'`, 4 `LBracket`, plus
   singletons. The shape says the grammar SOURCES moved ahead of `parse_grammar` (slashes, parens, pipes,
   backticks). This is a DSL-parser/plugin-source ticket of its own, far outside this slice; before my
   change the same law was red for a stale filename and showed none of it.
2. **`os_spr::channel` ×2** — paged generic decoder does not admit `LoadDocumentArchive`, and the paged
   recursive-archive owner does not reach terminal. Named by FL2 as pre-existing `os_spr` debt; untouched.
3. **`owned_field_rejected_page_tests` ×2** — rejected-page close reports `released_items: 1,
   released_bytes: 4096` where the fixture expects a different first step. Grant-accounting, untouched.
4. **`canonical_sealer_preserves_large_domains_and_all_wire_metadata_origins`** — past the origin decode
   now, fails on the sealed byte stream vs the `serde_json` oracle. Untouched.
5. **`presence_retirement::retained_presence_local_capture_cancel_closes_mounted_worker_while_store_remains_open`**
   — `false` vs `true`. Untouched.
6. **`artifact_store_batch_commit_refuses_an_under_declared_multi_item_gesture…`** — refuses with
   `batched fold exceeded its admitted fixed inverse capacity` where the law requires the gesture-wide
   COMMIT gate (`batched prepared candidate failed its exact fixed commit contract`) to be the refuser;
   i.e. the per-item fold is rejecting too early. Real defect, not started.
7. **`artifact_store_batch_publication_stages_two_hundred_mutations…`** — store does not reach its
   terminal witness after a 200-mutation batch. Real defect, not started.
8. **`retained_member_group_preparation…` and `retained_member_publication_preserves_order…`** — the
   retained member wire publication never reaches `Published` under the fixture's grant. These two are
   the closest to a shared root with 6/7 (the one-item/batch publication ladder) and are where I would
   start next.

Other honest limits:
- **Native only.** No wasm32 build, no `activate`, no serve, no browser. Every claim here is a native law
  run in this crate; the guest behaviour of the `.spr` lane field, the pack chunk framing and the dag
  cascade is unverified.
- **Cross-crate blast radius unmeasured.** Fixes 8 (`semio-framework-replication`) and 9
  (`semio-framework-pack`) are outside `-p semio-framework-os-kernel`; I did not run those crates' own
  suites, nor `semio-framework-plugin` (FP7 owns that number). The `.spr` `HistoryEdit.lane` presence bit
  is Rust-only — I checked there is no TypeScript decoder for spr edits.
- **The silencing panic hook** (§0) still makes the gate-form capture unreadable for any test that panics
  concurrently with it. Diagnosing this suite needs `--test-threads=1`.
- `🏪️store/🦀️.rs`, `🔄️sync/🦀️.rs` and the store unit tests are edited by peers concurrently; every edit
  here was re-read immediately before writing and is scoped to the lines named above.

## 5. Gate registration

`.vscode/launch.json`: new row **`⚖️gate🎠️kernel🦀️lib`** (group `4_gate`, order 408.4, directly after
`⚖️gate🔌️plugin🦀️lib`) running `bun x nx run @semio-tech/framework-os-kernel:test --skip-nx-cache`.
No new nx target was needed: `@semio-tech/framework-os-kernel:test` already exists and its
`📜️script.ts` `TestScript` runs exactly `cargo test --manifest-path Cargo.toml --lib`.

## 6. Files changed

Product:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️schema/🧬️mutations/🎯️restore-active-space-alternative/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs`
- `🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs`
- `🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs`

Contracts/fixtures:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧫️fixtures/🔏️canonical-edit-sealer.json`

Laws/harness:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧵️send/🧪️tests/🧵️send/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/✂️detach/🧪️tests/✂️detach/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️tests/⚖️protocol-laws/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🔌️io/🧪️tests/🔬️native-unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/💎️materialize/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/🧪️tests/🔬️unit/🦀️.rs`

Tooling:
- `.vscode/launch.json`

Captures: `🗑️generated/kn1-baseline.txt`, `kn1-baseline-serial.txt`, `kn1-probe-1.txt`,
`kn1-run-2.txt` … `kn1-run-9.txt`.
