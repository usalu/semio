# 📓️ W1-A Report: Replication `Supersede`, `TransactionRef`, `ReplayReport`

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, executor W1-A, 2026-09-30. Contract: `📋️design.md` §2.
Aliases: `REPL` = `🧰️framework/🔨️modules/📡️replication`, `OS` = `🧰️framework/🛍️products/💻️os/🔨️modules`.

## 1. Public API landed (crate `semio-framework-replication`, lib `protocol`, every item re-exported at `protocol::`)

| Item | Where | Notes |
|---|---|---|
| `HistoryTransition::Supersede(TransitionSupersede)` (tag 6) | `REPL/🔗️causal/🔀️transition/🦀️.rs` | codec `6 \| scope option \| count varint \| (target str \| u8 0: schema str, payload bytes \| u8 1: withdrawn)*` |
| `TransitionSupersede { scope: Option<String>, inputs: Vec<SupersededInput> }` | same | `targets()` (= envelope `dependencies`), `validate()`. Consts `SUPERSEDE_SCOPE_MAX_BYTES = 256`, `SUPERSEDE_PAYLOAD_MAX_BYTES = 262_144` |
| `SupersededInput { target: MutationId, replacement: InputReplacement }` | same | |
| `InputReplacement { Input { schema: String, payload: Vec<u8> }, Withdrawn }` | same | ToValue/FromValue `{"kind":"input",schema,payload}` / `{"kind":"withdrawn"}` |
| `EffectiveSupersession { transition_id, actor, timestamp, scope, replacement }` | same | ToValue/FromValue (camelCase) |
| `HistoryFold.supersessions: BTreeMap<MutationId, EffectiveSupersession>` | same | fold law below. `MutationId` now derives `PartialOrd, Ord` (`REPL/🆔️ids/🦀️.rs`) |
| `TransactionRef { id, tool }` + `TransactionRef::mint(&ActorId, &HybridLogicalTimestamp, tool)` | `REPL/🎮️mutation/🦀️.rs` (🔖️Meta region) | id `tx-{hex16(blake3(actor str \| hlc.actor \| hlc.physical_ms \| hlc.logical varints \| tool str))}` |
| `MutationMeta.transaction: Option<TransactionRef>` | same | sparse in ToValue, `None` when absent in FromValue |
| `MutationEnvelope.transaction: Option<TransactionRef>` | `REPL/🔗️causal/🦀️.rs` | last field. Binary: `… \| hlc \| transaction (varint 0 \| 1 id str tool str)` in `encode_envelope`/`decode_envelope` and the exact batch decoder (`transaction-flag` malformed reason, ids under `identifier-bytes`). `mutation_envelopes_from_edit_since` copies it from the meta. `history_transition_envelope` sets `None` |
| `MutationReplayOutcome`, `ReplayReport` (+ `Default`), `ReplayReport::blocks_finalize()` | `REPL/⚔️conflict/🦀️.rs` | blocks iff any outcome worst is `Error`/`Fatal` (`MergePolicy::Normal.rejects`) |

Fold law (in `fold_history`): each `Supersede` names only known operations (otherwise the same fold error as `Revert`: `transition references unknown operation X`). Candidates are collected in `(hlc.cmp_key(), id)` order. After the loop, a candidate applies iff its `scope` is `None` or equals the final `fold.alternative`. The later candidate wins, so the result is the last by `(hlc, id)`. There is no ownership rule. `applied`/`redo`/`refused` are untouched, so Revert, Reinstate and Commit compose independently.

`protocol_*` re-exports for the OS facade: `OS/📡️spr/🦀️.rs` now also exports `EffectiveSupersession, InputReplacement, SupersededInput, TransitionSupersede, SUPERSEDE_*_MAX_BYTES, TransactionRef, MutationReplayOutcome, ReplayReport` through `crate::os_spr::…`.

## 2. TypeScript twin (`REPL/🟦️.ts`, package `@semio-tech/framework-replication`)

- `TransactionRef`, `mintTransactionRef(actor, hlc, tool)`: byte-identical to Rust, using the first-party blake3 `🧰️framework/🔨️modules/🔏️hash/🟦️.ts` (no new dependency).
- `WireMutationEnvelope.transaction: TransactionRef | null` and `ExactWireMutationEnvelope.transaction: TransactionRef | null`. Both are required.
- The actor-protocol `MutationEnvelope.transaction?`, mapped by `mutationEnvelopeToWire`/`mutationEnvelopeFromWire`.
- Codecs: `encodeEnvelope`/`decodeEnvelope` (frames, `writeVecEnvelope`/`readVecEnvelope`) and `encode/readDocumentBackboneEnvelopeBatchExact`.
- History editing region: `InputReplacement`, `SupersededInput`, `TransitionSupersede`, `EffectiveSupersession`, `SupersessionFoldTransition`, `SupersessionFoldEvent`, and `foldSupersessions(operations, events)`, the TS twin of the supersession half of `fold_history`.
- Replay report twin: `REPLAY_SEVERITIES`, `ReplaySeverity`, `ReplayMutationMessage`, `MutationReplayOutcome`, `ReplayReport`, `replayReportBlocksFinalize(report)`.
- `📦️packages/🟦️typescript/📋️project.json` inputs now cover `REPL/**/🧫️fixtures/**`, `REPL/**/🧬️schema/**` and `🔏️hash/🟦️.ts`.

## 3. Schema-first corpora, fixtures, independent encoders

| Artifact | Path | Checked by |
|---|---|---|
| Transition payload corpus (renamed from `🔀️history-transition-v1`) | `REPL/🔗️causal/🧫️fixtures/🧫️history-transition/🔣️.json` | Python generator, Rust codec (`the_language_agnostic_fixture_matches_the_codec_byte_for_byte`), TS encoder + Ajv |
| Transition schema (renamed, `$id` …/history-transition.json, `supersede` added to `Transition.oneOf`, `Replacement`, `SupersededInput`, `Hex`) | `REPL/🔗️causal/🧬️schema/🔣️history-transition/🔣️.json` | Ajv strict |
| Independent Python encoder (moved from `☀️19/EVENT-SOURCED-FRAMEWORK-VERSION-CONTROL-END-TO-END/🧪️generate-history-transition-fixture.py`) | `REPL/🧪️tests/🧪️history-transition/🐍️.py` | `python3 <file>` verifies the committed corpus (exit 1 on drift); `--write` regenerates |
| Supersede fold steps (17 steps: foreign supersede, last-wins, multi-op withdrawal, late older arrival, revert interleave, commit, branch + scoped, checkout drops scope, equal-HLC id tiebreak, reload, unknown-target refusal) | `REPL/🔗️causal/🧫️fixtures/🧫️supersede-fold/🔣️.json` + `REPL/🔗️causal/🧬️schema/🔣️supersede-fold/🔣️.json` | Rust `fold_history` + rotation test; TS `foldSupersessions` + fast-check shuffled arrival; Ajv |
| Transaction mint vectors (4) | `REPL/🎮️mutation/🧫️fixtures/🧫️transaction-ref/🔣️.json` + `REPL/🎮️mutation/🧬️schema/🔣️transaction-ref/🔣️.json` | Rust mint, third-party `blake3` crate, TS mint, Ajv |
| Replay report verdicts (6) | `REPL/⚔️conflict/🧫️fixtures/🧫️replay-report/🔣️.json` + `REPL/⚔️conflict/🧬️schema/🔣️replay-report/🔣️.json` | Rust FromValue/ToValue round trip + `blocks_finalize`; TS `replayReportBlocksFinalize`; Ajv |
| Backbone batch corpus: every complete envelope gains the flag byte, `transaction: null`; 4 new cases (`transaction-canonical`, `-flag-invalid`, `-truncated`, `-identifier-over-limit`) | `REPL/🔗️causal/🧫️fixtures/🧮️document-backbone-batch-v1/🔣️.json` + schema `Envelope.transaction` | Rust exact decoder, TS exact decoder, Ajv |
| Wire frames with envelopes (flag byte inserted after the HLC) | `REPL/🧫️fixtures/📡️wire/{🕹️client-commands,🎮️server-commands,🔀️server-ack-transformed}/💾️.bin` | Rust `wire_fixtures_stay_byte_identical_across_rust_and_ts` (os-kernel, `sync`), TS wire fixture test |

Corpus regeneration kept every pre-existing case byte-identical. The one intended change is `unknown-tag`: `06` became `07`, since tag 6 is now `Supersede`.

## 4. Consumers fixed (minimal arms, no semantics beyond what compiles correctly)

- Workspace-wide struct literals outside replication: I added `transaction: None` (one site: `envelope.transaction.clone()`) to 101 `MutationMeta {…}`/`MutationEnvelope {…}` literals in 72 files. I used a one-shot codemod, deleted with `🗑️generated`. The crates touched:
  - os-kernel: store, sync, spr and tests.
  - os-kernel-db: engine, cli, cluster, projection, sync, preview, artifact, and the tests.
  - 36 `semio-s-*` artifact/plugin crates in 38 files: one editor `MutationMeta` literal each, plus the writer binary test and the flow retained preparation.
  - `semio-hub` tests, `semio-framework-os-mcp` test, `semio-framework-os-renderer-wgpu`, and `semio-framework-plugin` (`🔌️plugin/🦀️.rs` edited by hand).
- TS wire-envelope literals (`transaction: null`, or propagation) are in:
  - `OS/🏪️store/👷️worker/🟦️.ts`: `toWireEnvelope`, `exactWireEnvelope` and `hubWireEnvelope` propagate the field.
  - `OS/🏪️store/🔄️sync/🧪️tests/🔬️backbone-parity/🟦️.ts`, `💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts`, `OS/🧑‍💻dev/🧪️tests/👥️two-human/🟦️.ts` and `OS/📇️directory/🧪️testkit/📡️client-probe/🟦️.ts`.
  - The three hub tests `🤝️two-client-document`, `🤖️agent-ceiling` and `📈️document-growth`, plus `🌎️hub/📦️packages/🦀️rust/📜️script.ts`.
- Hand-rolled envelope codecs outside replication:
  - `OS/📡️spr/🧵️channel/🦀️.rs` `CommandPageWriter::envelope` now writes the transaction; before this, `app_command_apply_envelopes_round_trips` failed with `transaction flag 4`.
  - `OS/🛢️db/🗿️artifact/🦀️.rs` `HistoryEnvelopeCursor` gains `TransactionFlag`/`TransactionId`/`TransactionTool` states plus a `finish()` trailing-bytes guard.
- Retirement: `OS/🏪️store/🧩️composition/🚪️open/🦀️.rs` retires the new `HistoryFold.supersessions`. It adds `artifact_retire_struct!(EffectiveSupersession {…})`, `artifact_retire_leaf!(HybridLogicalTimestamp)`, and `RetireOwned` for `MutationId` and `InputReplacement`.

### Store `🦀️.rs` edits for W1-G to complete (`OS/🏪️store/🦀️.rs`, minimal compile arms only)

1. `ArtifactStoreMutationMetaRetirement::new` destructures `transaction` and retires `id`/`tool` into the free `strings[8]`/`strings[9]` slots.
2. `mutation_meta_from_history_op_meta`: `transaction: None`. **W1-G**: fill it from the new `HistoryOpMeta.transaction` once `.spr` carries it.
3. `replay_mutations` meta push: `transaction: None`. **W1-G/W2-A**: stamp it from `Emit.transaction`/the committed `ToolTransaction`.
4. `edit_from_operation_envelope`: `transaction: envelope.transaction.clone()`. This is semantically final.
5. The `.ops` `supersede` line:
   - New `OpsSupersededInput { target (positional), schema?, payload? (base64 OpBinary) }` with `From<SupersededInput>`/`TryFrom` (both absent means withdrawn; one absent is refused).
   - New `OpsHeaderLine::Supersede { id, actor, clock, after, scope?, inputs }`, printed in `ops_line_from_transition` and parsed (with `TransitionSupersede::validate`) next to `repin`.
   - **WRITTEN BUT UNVERIFIED**: it compiles, but no `.ops` round-trip test exercises it. W1-G's planned `.ops`/`.spr` round trip must cover it.
6. `OS/🏪️store/🔄️sync/🦀️.rs` `envelopes_from_history_edit`: `transaction: None`. **W1-G**: take it from `HistoryOpMeta.transaction`. `rollback_envelope`: `transaction: None` (correct: a rollback is not part of the original transaction).

Everything else in design §3 (effective forwards, `EditReplay`, the prefix ring, `reproject` supersession detection, commands, the revision accumulator, `HistoryOpMeta.transaction`, `HistoryLog::fold`) is untouched and belongs to W1-G.

## 5. Verification (all foreground, gated, run by me)

| Command | Result |
|---|---|
| `cargo check -p semio-framework-replication --lib` | green (6 pre-existing `unnecessary qualification` warnings in `⚙️codec`, not mine) |
| `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-w1a cargo test -p semio-framework-replication --lib` | **306 passed, 0 failed**. New tests include `supersede_validation_refuses_what_the_codec_refuses`, `a_supersede_envelope_depends_on_its_targets`, `the_supersede_fold_fixture_matches_the_fold_law`, `supersessions_are_a_function_of_the_event_set`, `supersession_values_round_trip`, `the_mint_vectors_match_the_first_party_and_third_party_blake3`, `mutation_meta_transaction_is_sparse_and_round_trips`, `envelope_transaction_round_trips_through_binary_and_value`, `replay_report_fixture_decodes_round_trips_and_gates_finalize` and `replay_report_blocks_finalize_on_error_or_fatal_only`; the extended existing ones are the corpus, backbone and bridge tests |
| `bun nx run @semio-tech/framework-replication:test --skip-nx-cache -- --reporter=verbose` | **17 passed, 0 failed**: supersede fold twin ×2, transaction ref ×2, replay report ×1, history transition payloads ×2, backbone batch, and wire fixtures byte identity |
| `python3 REPL/🧪️tests/🧪️history-transition/🐍️.py` | exit 0: corpus reproduced byte for byte |
| `cargo check -p semio-framework-os-kernel` | green (4 pre-existing warnings in store lines 12408/12654/19616/19625, not mine) |
| `cargo check -p semio-framework-os-kernel --tests --features sync` | green |
| `cargo test -p semio-framework-os-kernel --features sync --lib -- --skip os_spr::channel` | **1176 passed, 0 failed** |
| `cargo test -p semio-framework-os-kernel --features sync --lib -- os_spr::channel` | **92 passed, 0 failed**, after the channel writer fix |
| `cargo check -p semio-framework-os-kernel-db` | green |
| `cargo test -p semio-framework-os-kernel-db --lib` | 717 passed, 2 failed; neither is caused by W1-A (see below) |
| `tsc` over `REPL/🟦️.ts` + the new/edited replication tests | 0 errors in my files. The one error is `🛂️manifest/🟦️.ts(117)` `DialogChoice`, W1-E's in-flight work |
| `tsc` over the TS consumers edited in §4 | no error involving `transaction`/`WireMutationEnvelope`. Pre-existing: hub `📜️script.ts` missing names and lodash typings, mcp test import, the `🛂️manifest` `DialogChoice` |
| `bun ./📜️script.ts verify taxonomy report --scope REPL` | 22 errors, **none in a directory I created**. The renamed `🔀️history-transition-v1` dirs no longer appear; all others are pre-existing |

The two db failures are not caused by W1-A:
- `db_compact::…perpetual_release_error…` passes when run alone, so it is load-flaky.
- `db_sync::a_fifty_thousand_edit_document_streams…` fails deterministically at `wal.close()` with "WAL has pending records; force_flush is required before close". That happens before any envelope is decoded. The WAL group commit is count- and time-based (256 records / 20 ms, `now = i`), so it is independent of the one extra flag byte. `🛢️db/📝️wal/🦀️.rs` was changed today (commit `3eeee4f9119`).

**Consumers**:

- *Green on a later retry.* On my first two attempts, `semio-framework` failed to compile in `🛂️manifest/🦀️.rs` (W1-D's `ArgSchema` region). On a later retry it compiled, and these were green:
  - `cargo check -p semio-framework-plugin -p semio-s-artifact-dag-dag -p semio-s-artifact-puzzle-2d -p semio-s-plugin-space -p semio-s-artifact-writer-writer -p semio-hub -p semio-framework-os-mcp`: **Finished**, no errors.
  - `cargo check -p semio-hub -p semio-framework-os-mcp -p semio-s-artifact-writer-writer --tests`: **Finished**, no errors.
- *Blocked by peer WIP, not by W1-A:*
  - `semio-framework-os-renderer-wgpu` fails only in `OS/📺️renderer/…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (lines 6549–6591, 21709–21758, 29633). The causes are W1-D's new `ArgSchema` number fields (`precision`, `display_unit`, `snaps`, …) and `LocalizedLabel`. My edit in that crate (`🧊️renderer/🦀️.rs`, one `transaction: None`) is not among the errors.
  - The remaining 31 `semio-s-artifact-*` crates, in one batched `cargo check`, stop at `semio-s-artifact-stdio-png`: `MutationLeaf payload schema failed: No such file or directory` in `…/🗄️stdio/🗿️artifacts/📷️png/…/📸️set-snapshot/🦀️.rs:9`. That is W1-D's `MutationLeaf::PAYLOAD_SCHEMA` derive rollout. Its dependents could not be reached.
  - Each of their W1-A edits is a single `transaction: None` field in an otherwise unchanged literal.

## 6. Open items

- **W1-G**: see the store list above; `HistoryOpMeta.transaction` in `.spr` meta; effective forwards consuming `HistoryFold.supersessions`. The conflict target of a supersede envelope is still `[]` at protocol level; the store fills it from the replacement op.
- **W2-A**: populate `MutationMeta.transaction` from `Emit.transaction`. The TS `mintTransactionRef`/`TransactionRef::mint` are the single mint (W1-C should import them).
- **W2-E (hub)**, in `OS/🛢️db/🗿️artifact/🦀️.rs`:
  - `foreign_history_transition_refusal` passes `Supersede` (no ownership rule, per design §9.1);
  - `touch_writes_fields`/grading still skip transitions, so a supersede is ungraded until W2-E adds target-based grading.
- **Kernel TS twin**: `🎠️kernel/🟦️.ts` keeps its own `Severity`/`MutationMessage` twins. The replication twin now has `ReplaySeverity`/`ReplayMutationMessage`; W2-A/W2-B should reuse the replication ones for history-row severity rather than add a third.
- **Pre-existing debt** (flagged only, not mine):
  - `🔗️causal/🔀️transition`, the `-v1` fixture dirs and `🎮️mutation/🧪️tests/🔬️mutation-leaf-metadata` have no registered taxonomy kind;
  - `📦️packages/🦀️rust/📜️script.ts` is rejected by the root-script validator;
  - the `🔗️causal/🧫️fixtures/🧬️mutations/➕️causal-add` descriptor.
- Generated scratch in `🗑️generated/w1-a/` (codemod, tsconfigs, test logs) is left for the coordinator's sweep.

## Follow-up (coordinator request: framework-os "document backbone batch: truncated")

**Root cause.** Three hand-built hex batches in the os TS test tree were copies of the replication corpus case `maximum-u64-hlc` (`…03ffffffffffffffffff0105`). They predate the `transaction` flag. The decoder was correct, so I fixed the producers and did not add a lenient decoder:

| Site | Suite | Symptom | Fix |
|---|---|---|---|
| `💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts:2358` | `💻️os/🟦️.ts`: "retains one exact causal OpBinary through actor send and receive frames" (the failure W2-B reported) | `document backbone batch: truncated` | append the `00` flag |
| `💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts:958` | worker: "recovers a child that refuses an inbound hub frame…" | `mutation envelope: transaction flag 6` (the origin length was read as the flag) | append the `00` flag |
| same file `:1016` | worker: "preserves a server Commands batch with a maximum-u64 HLC…" | same | append the `00` flag |

**Verification.** `cd 💻️os/📦️packages/🟦️typescript && bun ./📜️script.ts test ../../🟦️.ts ../../🔨️modules/🏪️store/👷️worker/🟦️.ts` gives **382 passed, 0 failed** (245 + 137). Before the fix: 244 + 1 failed and 135 + 2 failed.

The full framework-os suite does not complete: it exceeds the `quick` 300 s budget and ran more than 10 min at `long`. Before I stopped it, the only other failure was `🧪️tests/🏷️schema-vocabulary`, which rejects `x-semio-fixture`. That annotation comes from `🌎️hub/🧫️fixtures/🐳️docker-image-v1/🔣️.json` and is unrelated to W1-A.

**Tree sweep for pre-`transaction` envelope bytes.** Scope: tracked and untracked files, excluding `.🧬semio`. Each scanner decodes with both the old and the new grammar and flags bytes that parse only under the old one; the scanners are in `🗑️generated/w1-a/`.

| Carrier | Scanned | Envelope-bearing (new grammar) | Stale after fix |
|---|---|---|---|
| hex strings ≥ 40 chars in `*.ts *.tsx *.json *.rs *.py` | 27 519 | 21 (the backbone corpus plus the 3 fixed test batches) | **0** (3 before the fix) |
| base64 strings ≥ 40 chars in `*.ts *.tsx *.json *.rs *.py *.ops *.semio` | 5 910 | 0 | **0** |
| `.bin` files | 24 | 3 wire frames (updated in W1-A, verified by the Rust and TS wire fixture tests) | **0** |
| binary logs `.spr`, `.spr.semio`, `.wal`, `.bin`, scanned for length-prefixed or embedded envelopes (`.spr` conflict records embed `encode_envelope` blobs) | 55 | 0 (the `.spr.semio` examples carry no conflicts) | **0** |
| TS decimal byte arrays building `Commands` frames (`Uint8Array.from([0, 3, …])` etc.) | 2 sites | both spread the fixed `batch` | **0** |
| TS/Rust object-literal producers (`WireMutationEnvelope`, `ExactWireMutationEnvelope`, `MutationEnvelope`) | fixed in the main pass (§4) | | 0 left (`tsc` shows no `transaction` errors) |
| other-language envelope codecs (Go: 195 files) | none exist | | n/a |

Limitation: the hex and base64 scanners only detect a batch that ends the string. A frame whose batch is followed by other fields (origin, frontier) is covered by the byte-array grep and the wire `.bin` tests instead. I checked that the scanner flags the old `maximum-u64-hlc` batch.

## Follow-up 2 (coordinator decision: the transition content address excludes the actor string)

**Change.** The new derivation is `history_transition_id(timestamp, payload)` = `transition-{hex16(blake3(hlc.actor varint | hlc.physical_ms varint | hlc.logical varint | payload bytes))}`.
- The free-form actor string is authentication metadata that the hub rebinds to the socket subject. It no longer enters the address; the numeric HLC actor still does.
- `history_transition_envelope` keeps its signature. The `actor` parameter still sets `envelope.actor` and no longer feeds the id.
- `TransactionRef::mint` is unchanged, as decided.

**Derivation and verification sites** (repo-wide grep of `history_transition_id`, `history_transition_envelope(` and `transition-` id strings):

| Kind | Count | Action |
|---|---|---|
| Rust derivation (`REPL/🔗️causal/🔀️transition/🦀️.rs::history_transition_id`) | 1 | actor dropped |
| Rust production callers of the builder | 2, both in `OS/🏪️store/🦀️.rs`: `transition_from_ops_line` verifies an `.ops` line id (~line 12050); `commit_transition` authors ids (~line 18744) | none needed: both derive through the builder, so authoring and verification agree by construction |
| Hub or db derivation/verification | 0 | the hub treats transition ids as opaque |
| TS twin | 0 before, now 1: `historyTransitionId(hlc, payload)` in `REPL/🟦️.ts` (first-party blake3) | added |
| Python | now 1 independent derivation, with a pure reference BLAKE3 in `REPL/🧪️tests/🧪️history-transition/🐍️.py` | added. Checked against the published vectors (`""`, `"abc"`) and against the TS blake3 over 14 lengths (0 to 31 744 bytes, multi-chunk trees) |
| Test call sites of the builder | 23, in 13 files | unchanged signature, no edits |
| Committed artifacts with old-derivation ids | 1 file, `✏️s/🔌️plugins/✒️writer/…/🧫️fixtures/🔁️hub-tail-after-check-in/🔣️.json`: 2 ids, 3 occurrences | re-derived. Its description notes the re-derivation |
| Go or other twins | 0 | – |

The writer capture's two Check In ids did **not** match the old derivation under the capture's own (hub-rebound) actor string. They had been hashed with the authoring replica's actor string: the exact disagreement the decision removes. Their HLC and payload are unchanged, so the new ids are what a fresh capture would carry.

**Corpus.** `🧫️history-transition/🔣️.json` gains a top-level `idClock` (`{actor: 2557761449, physicalMs: 1790622765027, logical: 3}`) and a `transitionId` on each of its 12 accepted cases. The schema requires both (pattern `^transition-[0-9a-f]{16}$`). The Python generator regenerates the corpus byte for byte (`python3 🐍️.py` exits 0). The ids are pinned by four implementations: Python pure BLAKE3, the Rust first-party hasher, the third-party `blake3` crate in the Rust test, and the TS twin.

**Tests (run by me).**
- New `a_transition_id_ignores_the_actor_string`. For every transition variant, the actor strings `local` and `hub.v1.<hex>` give the same id, which equals `history_transition_id(hlc, payload)`. A different numeric HLC actor changes the id. The two-replica plus hub test belongs to W1-G.
- `cargo test -p semio-framework-replication --lib`: **307 passed, 0 failed**.
- `bun nx run @semio-tech/framework-replication:test`: **17 passed, 0 failed**.
- `cargo test -p semio-framework-os-kernel --features sync --lib`: **1303 passed, 0 failed**. This includes the `.ops` round trips through the verification site.
- `cargo test -p semio-s-artifact-writer-writer --lib -- hub_tail`: 1 passed (`a_document_folded_from_the_hub_tail_initializes_again`, which uses the re-derived capture).
- `cargo test -p semio-framework-os-kernel-db --lib`: 717 passed, 3 failed. None involve transition ids:
  - the known 50k `db_sync` WAL-close failure;
  - `db_storage::…backend_opened_while_every_backend_control_is_taken…`, which passes when run alone;
  - `db_engine::throughput_tests::fs_commits_and_reopen_storms…`, a storm/serial parallelism ratio (0.55) measured at load average 52.
