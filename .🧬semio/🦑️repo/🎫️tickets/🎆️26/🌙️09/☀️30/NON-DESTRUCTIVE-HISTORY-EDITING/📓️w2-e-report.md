# 📓️ W2-E Report — Hub Rules for `Supersede`

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, executor W2-E, 2026-09-30. Contract: `📋️design.md` §2, §9.1, §9.2.

Path aliases used below:

| Alias | Path |
|---|---|
| `DBART` | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact` |
| `SYNC` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync` |
| `HUBB` | `🌎️hub/🏗️bootstrap/🦀️.rs` |
| `CHECKIN` | `🌎️hub/🗿️artifact-authority/📌️check-in` |

## 1. Admission (`DBART/🦀️.rs`)

`foreign_history_transition_refusal` is replaced by `history_transition_refusal(envelope, batch, earlier)`. It decodes every history transition and answers one typed refusal. The refusal comes before the WAL and applies whatever the merge policy.

- **A payload that does not decode** is refused with `MALFORMED_HISTORY_TRANSITION_CODE = "history.malformed-transition"`.
  - The old rule was "left to the replicas' decoder". That let the log hold a transition every replica refuses, and every Check In of such a document then failed.
- **`Revert`/`Reinstate`** keep the foreign-author rule, `history.foreign-transition`, with the same behaviour as before.
- **`Supersede`** has no ownership rule (design §9.1). It is never refused because of its author.
  - Its `dependencies` must equal `targets()` exactly. Otherwise it is refused as `history.malformed-transition`.
  - Every target must be committed, earlier in the same batch, or folded by a durable group decision. This is the same predicate `plan_one` uses. Otherwise it is refused as `UNKNOWN_SUPERSEDE_TARGET_CODE = "history.unknown-target"`, with the target in `.at([..])`.
  - `plan_one` still guards dependencies for every envelope.
- **`Commit`/`Branch`/`Checkout`/`Repin`** are admitted as before.
- **Size limits are unchanged.** `max_command_bytes` bounds the whole envelope. The codec enforces the per-input 256 KiB limit and the 256 B scope limit, and a violation lands in `malformed-transition`.

## 2. Grading (`DBART/🦀️.rs`)

- `command_touch` gives a Supersede the kind `SUPERSEDE_TOUCH_KIND = "db.supersede"` (see `supersedes_inputs`).
  - `touch_writes_fields` therefore counts a committed Supersede as a field write, so later unseen writes are graded against it.
  - The other transitions keep the transition schema kind and are still never graded.
- `unseen_concurrent_writes(recent, batch, written, envelope)` now asks whether the envelope's own touch writes fields. It no longer asks whether the envelope is a transition. A Supersede is graded like a write:
  - against unseen foreign writes after its `observed`;
  - by `targets_overlap` of the declared `target`.
- The WAL reopen replay and `preview_conflicts` use the same `command_touch`, so they classify a Supersede the same way.
- The hub only reads `envelope.target`. For a withdrawal, the authoring store puts the withdrawn operation's own target in that field, and the fixture vector `a-withdrawal-is-graded-by-the-withdrawn-operations-target` pins this.

## 3. Fixtures (schema-first, three consumers)

- **Schema:** `DBART/🧬️schema/⚔️concurrent-write/🔣️.json`.
  - A new `writes` shape: `{ "supersede": { inputs: [{target, replacement: input|withdrawn}], target: [..] } }`, defined in `$defs/Supersede`.
  - The `codes` enum adds `history.unknown-target`.
  - The description states the rule.
- **Corpus:** `DBART/🧫️fixtures/⚔️concurrent-write/🔣️.json` now has 19 vectors and 57 commits. The 9 new vectors are:
  - a foreign supersede is admitted;
  - vigilant refuses a supersede that did not see a same-target write;
  - normal accepts it and reports `mutation.clamped`;
  - a write that did not see a supersede is concurrent with it;
  - a supersede to a disjoint target is no conflict;
  - a withdrawal is graded by the withdrawn operation's target;
  - concurrent supersessions of one operation conflict;
  - a refused write is not in the log, so it cannot be superseded (`history.unknown-target`);
  - an unknown target is refused whatever the policy, even alongside a known one.
- **Consumers:**
  - Rust: `DBART/🧪️tests/⚔️concurrent-write/🦀️.rs`.
    - The builder now encodes real transitions: an empty `Revert` for `transition: true`, and a `Supersede` with `dependencies = targets` for `supersede`.
    - The replay law skips the two commits that admission refuses.
    - New law: `a_supersession_is_admitted_by_its_targets_never_by_its_author` covers a same-batch target, a target placed after its supersede, undeclared dependencies, and an undecodable payload.
  - TS twin with Ajv: `💻️os/🧪️tests/⚔️concurrent-write/🟦️.ts`. The reference rule gains unknown-target admission and grading by declared target. No Python consumer of this corpus exists.

## 4. Check-in (`CHECKIN/🧪️tests/🔬️unit/🦀️.rs`, feature `native-artifact-execution`)

The test-local `LinkedGisMapCodec` has the shape of `PluginHostArtifactCodec` and a linked `VerifiedNativeArtifactCodec`: `codec.replay_envelopes` is `store::replay_envelopes_onto_pair`. It runs over a real GIS Map ledger that the editor store emits.

- `materialize_check_in_folds_a_supersession_of_the_ledger`: a create of `p` at lon 7, then a collaborator's Supersede to lon 9, goes through `materialize_check_in`. The resulting pair keeps 1 edit and 1 transition, and its reload (`parse_document_pack`) shows lon 9.
- `materialize_check_in_refuses_a_supersession_that_blocks_the_replay`: the ledger is create `p`, delete `p`, then withdraw the create, which makes the delete report `mutation.target-missing` (an `Error`).
  - Check In fails with `AuthorityError::Codec { stage: Output }`, whose message contains "rejected by merge policy Normal".
  - The error maps to `DocumentCheckInRefusalV1::CodecRefused`.
  - A second attempt gives an identical error.
  - The same ledger without the supersession checks in.
- **Dependency on W1-G, now in place.** W1-G added this at my request:
  - `ingest_remote` reports the worst outcome of the supersession replay;
  - `replay_envelopes_onto_pair` refuses `Normal.rejects(worst)` as `VcsError::Rejected { policy: Normal }`.
  - W1-G's own law is `check_in_refuses_a_supersession_whose_replay_blocks_under_normal`.
- **Why "typed" is `Codec`/`CodecRefused`, not a new variant.** A guest codec crosses the WIT boundary as a string `TurnFault`. A dedicated `AuthorityError` variant would therefore differ between linked and guest codecs for the same ledger. `codec-refused` is also what a quarantined ledger envelope already answers. A dedicated `ledger-not-replayable` mapping would need a typed guest fault, which belongs to the plugin host (W2-A), and is flagged below.

## 5. Refused supersede in the sync actor and the hub (`SYNC/🦀️.rs` refusal region, `HUBB`)

A history transition has no inverse; `rollback_envelope` returns `None`. A refused Supersede, or a foreign undo, therefore leaves its author ahead of the hub. The behaviour is split between the hub and the actor.

- **Hub, `HUBB`.**
  - `ClientFrameStepV1::Rebootstrap` and `after_refusal(sent, irreversible)`: after it answers a Rejected Ack for a batch that `holds_history_transition` and that was refused for good (anything except `db::DbError::Unavailable`), the hub runs `rebootstrap_document_socket`. That sends the `RebootstrapRequired` control for the active checkpoint and closes the socket `1013 rebootstrap-required`. This applies to:
    - the db refusal in the commit path (`submit_commands` now answers `for_good`);
    - socket actor or document mismatch;
    - an undeclared batch limit;
    - a permanent security refusal.
  - A document with no canonical checkpoint has nothing to rebuild from, so its socket stays open. That keeps `a_foreign_history_transition_is_refused_at_the_socket_and_never_relayed` valid.
  - The lag path uses the same helper with `lagged = true`, which always closes. `send_socket_document_rebootstrap` now answers `Result<bool, SocketBindingValidityV1>`.
- **Actor, `SYNC`, native and browser-wasm.**
  - `irreversible_refusal(refused, reason)` raises the typed `HISTORY_TRANSITION_REFUSED_CODE = "history.transition-refused"`. It is an `ArtifactEvent::Conflict` at `Error` level whose `target` lists the refused transition ids.
  - It is emitted after `CommandOutcome::Rejected`, and only the operation envelopes roll back.
  - The hub's following control goes through the existing `require_artifact_rebootstrap`, then the host's reseed, then `reseed_hub_document`. The refused batch was already removed from `pending_batches`, so it is never requeued, resent or handed back to the re-seeded guest.
  - Hosts already surface `Conflict` events: the wgpu sync card and `shell_sync_link_terminal`, which does not treat this code as terminal.

## 6. Verification (all foreground and gated, run by me)

| Command | Result |
|---|---|
| Baseline, before my change: `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-w2e cargo test -p semio-framework-os-kernel-db --lib` | 717 passed, 2 failed. The failures were `db_io_retained_fixtures::a_backend_cleanup_fault…` and `db_sync::a_fifty_thousand_edit…` |
| `cargo check -p semio-framework-os-kernel-db --tests` | green; my 4 new warnings fixed |
| Same `cargo test -p semio-framework-os-kernel-db --lib`, after the change | **718 passed**, 2 failed. The failures were `db_sync::a_fifty_thousand_edit…` (pre-existing, deterministic) and `db_engine::throughput_tests::fs_commits_and_reopen_storms…` (load: average about 25; **passes alone**, 1/1). All 3 concurrent-write laws and the foreign-transition law pass |
| `bun nx run @semio-tech/framework-os:test --skip-nx-cache -- -t ConcurrentWrite` | **2 passed** (Ajv admission plus the independent reference rule) |
| `cargo check -p semio-framework-os-kernel --features sync` | green (4 pre-existing store warnings) |
| `cargo check -p semio-framework-os-kernel --features sync --target wasm32-unknown-unknown` | green (this covers the browser `wasm_actor` edit) |
| `CARGO_INCREMENTAL=0 … cargo test -p semio-framework-os-kernel --features sync --lib -- os_store::sync` | **82 passed**, including `a_refused_supersession_is_a_typed_refusal_and_reseeds_from_the_canonical_pair` |
| `CARGO_INCREMENTAL=0 … cargo test -p semio-hub --lib -- check_in` (first run) | 4 passed, 1 failed. The failing law was `materialize_check_in_folds_a_supersession_of_the_ledger`, and **only** on DslValue key order: the fold was right (lon 9), and I fixed the expectation to the pack's canonical order. `materialize_check_in_refuses_a_supersession_that_blocks_the_replay` passed |
| Hub rerun after the key-order fix and the stronger message assertion; `os-hub` bin (`HUBB` edits plus the new socket law `a_supersession_is_admitted_whatever_its_author_and_its_refusal_rebootstraps_the_author`) | see §7 |

## 7. Hub rerun status

**WRITTEN BUT UNVERIFIED: every `semio-hub` build is blocked by peer WIP.** Since about 04:25, the untracked module `🔌️plugin/🧩️extension/🚪️retirement/🦀️.rs` fails to compile, with four errors:
- `semio_framework::CapabilityRequest` does not exist;
- `CapabilityId` does not exist;
- `MetadataOwner` is not implemented;
- a match on `SnapshotRetirementStep::Blocked` is non-exhaustive.

`semio-hub` depends on `semio-framework-plugin`, so both builds below are blocked. I tried twice about 15 minutes apart and did not poll further (fleet rule 14).

- **`cargo test -p semio-hub --lib -- check_in` rerun.** The only change since the run in §6 is test-side: the expected DslValue key order, and a stronger `message.contains("rejected by merge policy Normal")` assertion. W1-G confirmed that Display string.
- **`os-hub` binary (`HUBB` plus the new bin-unit socket law).** This code was never compiled, so reviewers should read it:
  - `ClientFrameStepV1::Rebootstrap` / `after_refusal`;
  - `holds_history_transition`;
  - `submit_commands`' `for_good`;
  - `commit_admitted_commands` → `ClientFrameStepV1`;
  - `rebootstrap_document_socket`;
  - `send_socket_document_rebootstrap` → `Result<bool, _>`;
  - the bin-unit law `a_supersession_is_admitted_whatever_its_author_and_its_refusal_rebootstraps_the_author`.

Rerun once the plugin compiles:
- `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-w2e cargo test -p semio-hub --lib -- check_in`
- `cargo test -p semio-hub --bin os-hub -- a_supersession_is_admitted_whatever_its_author a_foreign_history_transition_is_refused_at_the_socket`

## 8. Open items / flags

- **W2-B/W2-C (shells):** localize the new codes `history.unknown-target`, `history.malformed-transition` and `history.transition-refused`. Precedent: `🏛️ShellHost/🟦️.tsx` `mutationCodeLabelKey` for `history.foreign-transition`, plus the wgpu equivalent. Without labels they fall back to the generic rejected title.
- **W2-A (plugin host):** carry a typed guest fault for `VcsError::Rejected` through `codec.replay-envelopes`. Only then can Check In answer `ledger-not-replayable` instead of `codec-refused` uniformly for linked and guest codecs.
  - The directory schema description of `ledger-not-replayable` (approval decisions only) lives in `OS/📇️directory`, which is not mine.
- **Wire vocabulary is unchanged.** No new frame; the reseed reuses `RebootstrapRequired`.
- **TS browser worker.** The `👷️worker/🟦️.ts` refusal path already rebootstraps verified browser actors on any refusal, and its twin of `rollback_envelope` does not skip transitions. It belongs to the store owner, so I did not change it.
- **Pre-existing, not mine:**
  - `db_sync::a_fifty_thousand_edit…` fails with "WAL has pending records".
  - The db throughput and IO-cleanup laws are load-flaky.
  - A peer's in-flight `semio-framework-plugin` edit (`🧩️extension/🚪️retirement`) blocked every `semio-hub` build during my run.

## 9. Files

- `DBART/🦀️.rs`
- `DBART/🧪️tests/⚔️concurrent-write/🦀️.rs`
- `DBART/🧫️fixtures/⚔️concurrent-write/🔣️.json`
- `DBART/🧬️schema/⚔️concurrent-write/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🧪️tests/⚔️concurrent-write/🟦️.ts`
- `CHECKIN/🧪️tests/🔬️unit/🦀️.rs`
- `🌎️hub/🏗️bootstrap/🦀️.rs`
- `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`
- `SYNC/🦀️.rs` (WireBridge refusal helper and both actors' `Rejected` arms)
- `SYNC/🧪️tests/🔬️unit/🦀️.rs` (one appended law)

Scratch output is in `🗑️generated/w2-e/` and is left for the coordinator's sweep.
