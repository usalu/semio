# FP12 — `semio-framework-plugin --lib` 822/2 → 0 red

Slice FP12, 2026-09-22 (session 8). Continues FP11 (`📓️fp11-plugin-lib-zero-red-and-lanes.md`).

Method (inherited from FP5–FP11): private `CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-fp12`
+ the shared build-dir, `CARGO_INCREMENTAL=0`, `CARGO_BUILD_JOBS=4`, `RUST_MIN_STACK=67108864`;
`cargo check … --profile test` first (never uplifts), `--no-run` link second, the lib unittests binary
copied out of the shared build-dir and driven directly with `--test-threads=1`. Every number below is
read from a capture in `🗑️generated/fp12-*`.

## 0. Starting state (inherited, FP11 §1)

822 passed / 2 failed. The two reds:

| law | cause (FP11) |
|---|---|
| `a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames` (FP11 §3.4) | `TestSnapshot` declares no child refs |
| `child_root_maintenance_requires_terminal_empty_before_reclaim` (FP11 §3.5) | the lying factory is structurally unreachable |

Both live in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`.

## 1. Round table — **825 passed / 0 failed**

| round | what landed | result | capture |
|---|---|---|---|
| FP11 final (inherited) | — | 822 / 2 | (FP11's captures were swept by a peer's `script.ts clean` at ~16:4x; the numbers are quoted from `📓️fp11-…md`) |
| check 1 (16:10→16:24) | §2 + §3 in ONE edit | **killed at 16:24** on the coordinator's load order (machine at load 267, swap 8.4/9.2 GB); the run had reached the lib-test unit's late lint pass with no `error` line | `fp12-check-1.txt` (also swept) |
| check 2 (17:02) | — | **1 error**: `E0004` — `TestMutation::SetSlotChildren(_)` not covered in `🧪️tests/🔬️tool-run/🦀️.rs:329` | `fp12-check-2.txt` |
| check 3 (17:05) | the `tool-run` match arm | `Finished test profile in 1m 32s`, 0 errors | `fp12-check-3.txt` |
| round 1 (17:08) | §2 + §3 | **823 / 2** — both reds' ORIGINAL causes gone; the two failures were TEARDOWN only (§2's app reached `Drop` without its shallow-shell witness; §3's close hit `artifact store cursor child reported Complete without terminal-empty authority`) | `fp12-round1-serial.txt` |
| round 2 (17:11) | the lie mints ONCE per factory; §2 closes both apps | **825 / 0** | `fp12-round2-serial.txt` |
| rounds 3–14 (17:11→17:15) | no change — twelve runs of the SAME binary | **825 / 0** in rounds 3, 8, 9, 11, 12, 13, 14 (7 more greens); rounds 4, 5, 6, 7, 10 each failed ONE unrelated contract law | `fp12-round{3…14}-serial.txt` |
| rounds 15–17 (17:21) | no change — three consecutive runs after the dependent checks | **825 / 0, 825 / 0, 825 / 0** | `fp12-round{15,16,17}-serial.txt` |

**Net: 822 / 2 → 825 / 0.** `+3` passing = the two reds fixed plus one new law
(`descriptor_has_set_slot_children_identity`). No law was `#[ignore]`d, deleted or loosened, no
ceiling constant moved, and every clause of both reds' own assertions is intact — §3's law still
demands a NAMED fault before any reclaim and a non-empty registry on every turn.

**The intermittent failure in rounds 4/5/6/7/10 is NOT this slice's.** It is one of two contract laws
— `retained_operation_continues_after_command_admission_until_publication_and_retirement`
(`plugin.internal: instance busy or poisoned: 7`, `…/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:1692`)
or `concurrent_typed_operations_hand_every_presentable_page_to_one_turn` — never both, never mine,
and exactly FP11 §3.7's class (a `cell.instance.try_lock()` refusal over process-wide
`component_persistent_local!` state). **Control measured**: six runs of the same binary with BOTH of
my laws `--skip`ped still produced one such failure (`fp12-flake-without-my-laws.txt`, 823/0 five
times, 822/1 once). So the flake reproduces with my laws not running at all. Observed rate this
slice: 5 of 16 full runs, 1 of 6 skipped runs, under peer load 21–60; the last three runs
(15–17) are consecutive greens.

## 2. Item 1 — `TestSnapshot` child refs + the `TestMutation` variant

**The law that was red:** `a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames`
(`…/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`), failing at `load child pack`
with `plugin.internal: child restore is not declared by the loaded parent snapshot`
(`🔌️plugin/🦀️.rs:24462`, `validate_parent_child_restore`).

**The chain, re-measured this slice.** `PluginApp::open_child`/`load_child_pack` call
`validate_parent_child_restore`, which builds `store::ChildRestoreProjection::from_snapshot` over the
LOADED parent snapshot's own `ArtifactCompositionFields::visit_child_refs` and nothing else
(`🏪️store/🦀️.rs:3240`); `admits_member` then matches slot name, child id == artifact id, and the
three dialect segments. `TestSnapshot::visit_child_refs` was `Ok(())` and `child_slots()` was the
empty default, so NO member could ever be admitted back, however the child packs were loaded.

**Landed, three coupled changes (FP11 §3.4's own list):**

1. **The field.** `TestSnapshot` (`🧪️tests/🖥️test-app-mutations-document/🦀️.rs`) gains
   `slot: Vec<store::ArtifactChild<TestSnapshot>>` — the product type a schema-derived snapshot gets
   from `#[child(kind = "s.test.child")]` (`ComposedParentSnapshot` in `🧪️tests/🧩️composition/🦀️.rs`
   is the derived precedent). `Vec`, not `Option`: there is **no `impl<T: DslField> DslField for
   Option<T>` anywhere in the tree** (only `Vec<T>`, `🗣️dsl/🦀️.rs:183`), and `TestSnapshot` derives
   `dsl::DslArtifact`, so an `Option` field would not compile. The slot is therefore
   `ChildSlotSpec { name: "slot", kind: "s.test.child", many: true }`, matching the `"slot"` every
   composed contract law registers under.
   `ArtifactChild<S>` has `ToValue`/`FromValue`/`DslField`/`RetireOwned`/`PartialEq`/`Debug`/`Clone`
   for any `S` but **no serde impl** (`🏪️store/🦀️.rs:3052`; the struct's own doc claims a
   `#[serde(bound = "")]` derive that is not there — stale doc, recorded as a finding, not touched).
   The field is therefore `#[serde(skip)]` and the file's hand-written codecs carry it explicitly:
   `to_json`/`from_json` write the declared rows as canonical `ArtifactRef` uris
   (`"child-1!s.test.child@native/*"`), and an **undeclared slot writes no key at all**, so every
   pre-existing pack and DSL text is byte-identical. `RetireOwned`'s destructure, `Clone`,
   `TestDiff` (`slot: Option<Vec<String>>`, replaced wholesale) and `TestDiff::apply` (which now
   fails with `MutationApplyError("test-snapshot.declared-child-uri", …)` on an unparsable uri
   rather than dropping it) all carry it.
2. **The mutation that maintains it.** New leaf
   `🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/🧒️set-slot-children/` with its
   `🦀️.rs`, its `🔣️.json` descriptor (`semanticKind: set-slot-children`, `binaryTag: 2`,
   `invertibility: explicit-mutation`) and its `🧬️schema/🔣️.json`; roster entry + aggregate variant
   `TestMutation::SetSlotChildren`; re-export through `🖥️test-app-mutations/🦀️.rs`; unit law
   `🧪️tests/🧒️test-app-document-set-slot-children-unit/🦀️.rs`; and the registry row
   `"🧒️set-slot-children"` in `📚️library/🔣️taxonomy.json`'s `plugin-test-mutations`
   `memberNames`. No hand-written `OpText`/`OpBinary`: the aggregate's `dsl::DslOps` `DslVariants`
   and `dsl::variants_binary` own both surfaces, exactly as `SetCount`/`SetLabel` do.
   `inverse()` returns the base's whole declaration, so declare↔clear is a real explicit inverse.
   The verb is `set`, not `declare`: `protocol::APPROVED_VERBS`
   (`📡️spr/🎮️command/🦀️.rs:112`) is a CLOSED vocabulary and `declare` is not in it — a
   `declare-slot-children` kind compiles the derive and then fails const-eval. Recorded because the
   failure message points nowhere near the verb.
3. **The law reloads the PARENT.** It now settles the composite gesture
   (`artifact_app_laws::settle_registered_typed_operation` — a migrated `dispatch_typed` only
   ADMITS, FP11 §3.1), declares the member on the parent store through the real
   `store::ArtifactCommand::Apply` route, asserts the LIVE parent declares one member, persists
   `PluginApp::document_pack` **beside** the child packs, and loads the parent pack into the fresh
   app before the child packs — asserting the reloaded parent carries the declaration through the
   pack. Without that leg a fresh app's default `TestSnapshot` can never declare anything, whatever
   changes 1 and 2 do.

The roster unit law `direct_leaves_preserve_generic_document_codecs_and_laws` now runs its
text/binary round trip, diff-apply and inverse over a base that ALREADY declares a child and over
both a declaring and a clearing `SetSlotChildren`.

## 3. Item 2 — the reclamation law: product answer

**Decision: the law is driven onto the reachable guard — the fixture reaches the lying factory
through the real `space_members!` member, and no law clause was weakened.** FP11 §3.5 concluded the
lying factory was "structurally unreachable through any `space_members!` member". That is true only
while the member still holds the captured snapshot; the chain below shows the reachable state, and
the law now sets it up.

**The measured chain (all line numbers at HEAD this slice):**

- `ChildContentRetirement::close_step` (`🔌️plugin/🦀️.rs:9249`) takes the displaced entry and calls
  `member.retire_snapshot_read_erased(snapshot)`. `SpaceMember` has exactly ONE implementor
  repo-wide (its own doc, `🏪️store/🦀️.rs:19925`) — the `space_members!` expansion
  (`🏪️store/🦀️.rs:20750`) forwarding to `ArtifactStore` (`:20224`) — which always answers with a
  **`ReturnedSnapshotReadRetirement`** carrying the member's `initial_snapshot_retirement_factory`,
  i.e. the lying `ArtifactOwnedValueRetirementFactory`.
- `ReturnedSnapshotReadRetirement::close_step` (`🏪️store/🦀️.rs:1676`) consults that factory **only**
  inside `Arc::into_inner(alias).Some(unique)`. So the lie is reachable exactly when the retiring
  root holds the snapshot's LAST `Arc`.
- Two owners kept it from being the last: (a) the member's own `current` snapshot — the root was
  captured from it, so they are the same allocation; (b) after a dispatch,
  `replace_current_retained` (`🏪️store/🦀️.rs:15872`) pushes `snapshot_retirement_factory.retire(previous)`
  onto the member's displaced queue, which **pins that very alias**, and the fixture installed the
  LIE on that factory too — so it could never release it. This is why FP11's "dispatch between the
  capture and the maintenance turns" measured byte-identical.
- The lease registry's own clone is not a factor: `into_typed` (`🏪️store/🦀️.rs:428`) takes and drops
  the registry's slot owner before the retirement is built.

**Landed (fixture only — no product line moved for this item):**

- `install_test_snapshot_retirement` installs the lie on the **owned-value** factory only; the
  **displacement** factory is honest, with the docstring saying why (a lying displacement factory
  pins the alias for ever, so `Arc::into_inner` can never reach the owned-value factory).
- The law advances the member twice past the captured snapshot and drains the member's own
  `maintenance_retirements_step` until `maintenance_retirements_terminal_is_empty()`, asserting that
  drain, before it publishes the replacement root. Two dispatches, because one leaves the previous
  snapshot in the store's `tail_undo_cache`.
- `TestSnapshotRetirement`'s lie is **spent on its first answer**: the law's subject is that a
  `Complete` without a terminal-empty witness is caught BY NAME, and a retirement that lied for ever
  could not then be disposed, so the app could not be closed through the same honest ladder every
  other member uses. The law therefore ends with `drain_and_close_composed_fixture`, proving the
  refusal rather than leaking past it.

The law's own assertions are untouched: every turn must leave `child_content_retirements`
non-empty, the fault must arrive before any reclaim, and its code must be one of the two NAMED
guards (`interactive-job.child-snapshot-terminal-not-empty` /
`interactive-job.child-snapshot-disposer-refused`) — an unnamed `plugin.internal` still fails it.

## 4. Dependent-crate re-checks

| check | result | capture |
|---|---|---|
| `cargo check -p semio-s-plugin-note --lib` (native) | **green**, `Checking semio-s-plugin-note` + `Finished dev in 59.17s`, 17:18 | `fp12-dependent-native.txt` |
| `cargo check -p semio-framework-plugin --lib` (plain, no `cfg(test)`) | **green**, `Checking semio-framework-plugin` + `Finished dev in 14.35s`, 17:19 | `fp12-check-nontest.txt` |
| `cargo check -p semio-framework-plugin --lib --target wasm32-wasip2 --features component-guest` | `Finished dev in 0.30s`, **fresh — nothing recompiled**, 17:19 | `fp12-wasm-plugin.txt` |

`semio-s-plugin-note` has no `component-app-assembly` feature (`cargo` refuses it by name, 17:17), so
the native check is the plain `--lib` one.

**The wasm32 check is fresh, and that freshness is the proof, not a gap.** Before running it I
deliberately `touch`ed `🧪️tests/🖥️test-app-mutations-document/🦀️.rs` — the fixture file this slice
rewrote — and the wasm32 unit STILL answered `Finished` in 0.30 s without a `Checking` line. A file
that is part of a unit invalidates it when touched; this one does not. Every Rust file this slice
changed is mounted under `#[cfg(test)]` (`🔌️plugin/🦀️.rs:317` for the whole
`test_app_mutation_fixture` tree, `:6963` for `🔬️tool-run`, and the roster's own `:54` for the unit
law), so none of them is in the wasm32 or the plain-native unit at all. The plain native `--lib`
check DID recompile (14.35 s with a `Checking` line) and is green, which is the fresh-compile receipt
for the crate itself. **This slice changed no production line**, so the 11:16–11:34 fleet-wide break
class cannot repeat from it; the one compile break it did cause (`E0004`, check 2) was inside the
test binary and never reached any other crate.

## 5. Flow `retained::*` read-only re-run

`cargo test -p semio-framework-artifact-flow-flow --lib retained -- --test-threads=1`, 17:20:
**8 passed / 6 failed** — unchanged from FP10's and FP11's readings, with the same six names
(`retained::copy::tests::flow_selected_copy_{allocation_admission_is_separate_and_never_reallocates_payload_pages,
cancellation_and_invalid_projection_preserve_root_until_close, matches_serde_and_shares_unchanged_ordered_roots,
rejects_root_retirement_overgrant_and_closes_factory_owner}` and
`retained::tests::flow_physical_retirement_{every_direct_string_and_vec_releases_actual_capacity_once,
multi_root_ingress_records_fault_without_admission_then_admits_exactly}`).
Capture `fp12-flow-retained.txt`. **No file under `🌊️flow/**` or `✏️s/🔌️plugins/🌊️flow/**` was
touched by this slice** — this is a read-only re-run, and the lane stays the peer session's topic.

## 6. Files changed

Fixtures / laws (all `#[cfg(test)]`-reachable; **no production line changed this slice**):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-document/🦀️.rs` —
  `TestSnapshot.slot`, `to_json`/`from_json`, `test_child_handle`, `visit_child_refs` + `child_slots`,
  `Clone`, `RetireOwned`, `ArtifactDsl`, `ArtifactPack`, `TestDiff.slot` + `apply`/`absorb`
- `…/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/🧒️set-slot-children/🦀️.rs`
  + `🔣️.json` + `🧬️schema/🔣️.json` — NEW leaf
- `…/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/🦀️.rs` — roster + aggregate variant
- `…/🔌️plugin/🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/{📝️set-test-count,🏷️set-label}/🦀️.rs`
  — `TestDiff` literals gain `slot: None`
- `…/🔌️plugin/🧪️tests/🖥️test-app-mutations/🦀️.rs` — re-export `SetSlotChildren`
- `…/🔌️plugin/🧪️tests/🧒️test-app-document-set-slot-children-unit/🦀️.rs` — NEW descriptor-identity law
- `…/🔌️plugin/🧪️tests/🧬️test-app-document-mutation-roster-unit/🦀️.rs` — the codec/inverse law now runs
  over a declaring base and both `SetSlotChildren` forms
- `…/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` — §2's persist/reload law,
  §3's reclamation law, `install_test_snapshot_retirement`, `TestSnapshotRetirement::close_step`,
  three `TestSnapshot` literals
- `…/🔌️plugin/🧪️tests/{🧾️document-archive-load-legs,🧩️composition}/🦀️.rs` — six `TestSnapshot` literals
- `…/🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs` — `toy_run_job`'s provisional match covers the new variant
  (the one compile break this slice caused, `E0004`, found by check 2 and fixed in check 3)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` — `plugin-test-mutations`
  `memberNames` gains `"🧒️set-slot-children"`

## 7. Honest gaps

- **The gate is green but the suite is not deterministic.** 825/0 was measured in 8 of 13 full serial
  runs of the same binary; the other 5 each lost ONE unrelated contract law to the
  `instance busy or poisoned` flake (§1). The control run proves the flake is not this slice's, and
  no root fix for it is claimed or invented. A successor wanting a deterministic gate should start
  from FP11 §3.7's inventory of process-wide `component_persistent_local!` state.
- **Round 1 taught the real cost of both laws going green: teardown.** While a law failed early, its
  panic masked every `Drop` assert behind it. Making §2 pass made
  `artifact store reached Drop without its exact terminal-empty shallow-shell witness` fire, and
  making §3's refusal arrive made the fixture close hit
  `artifact store cursor child reported Complete without terminal-empty authority`. Both are recorded
  because the next author to turn a red law green in this file will meet the same wall.
- **The lie is minted exactly once per factory** (§3). That is a deliberate, documented narrowing of
  the FIXTURE, not of the law: the same `ArtifactOwnedValueRetirementFactory` is what the member's
  own close cursor, envelope retirement and history disposal draw from, so a factory that lied for
  ever would prove only that a lying store cannot be shut down. One lie — the first unique owner a
  child-content retirement hands back — is exactly the subject.
- **`ArtifactChild<S>`'s own docstring is stale** (`🏪️store/🦀️.rs:3015`): it explains how
  `Serialize`/`Deserialize` avoid the `PhantomData<S>` bound trap "via `#[serde(bound = "")]`", but
  the struct carries no serde derive at all. Found while sizing §2's field; not touched, because it
  is a product file three peers are editing this session.
- **`interactive-job.child-snapshot-terminal-not-empty` remains dead code** for every member
  `store::space_members!` generates (FP11's finding, re-confirmed here with the line numbers in §3):
  the store's `ReturnedSnapshotReadRetirement` turns exactly that lie into an `Err` first, which the
  plugin reports as `interactive-job.child-snapshot-disposer-refused`. The law accepts either NAMED
  code and refuses an unnamed one, so it is truthful; the second guard is defence in depth against a
  hand-written `SpaceMember`, which no caller in the tree has.
- **`declare` is not in `protocol::APPROVED_VERBS`** and the const-eval failure for a non-approved
  verb reports through the generic "Mutations leaf source must match its aggregate workspace and
  direct owner" message. Avoided by naming the leaf `set-slot-children`; recorded because the next
  author will hit the same wall.
- **There is no `impl<T: DslField> DslField for Option<T>` in the tree**, which is why the declared
  slot is a `Vec` and the slot spec is `many: true`. That is a real shape choice forced by the DSL
  surface, not a preference.
- The `🧒️set-slot-children` leaf was first written as `🧒️declare-slot-children` and renamed; the
  auto-commit had already staged the pre-rename unit-test path, so `git status` may show one stale
  `AD` row until the next auto-commit. No git-modifying command was run by this slice.
- Flow `retained::*` was NOT touched (peer session's topic); §5 is a read-only re-run, still 8/6.
- **FP11's `🗑️generated/fp11-*` captures no longer exist** — a peer's `📜️script.ts clean` ran at
  ~16:4x (it freed the disk from 13 GiB to 105 GiB) and swept them, along with this slice's own
  `fp12-check-1.txt`. FP11's numbers in §1 are therefore quoted from its report, not from a capture I
  can still read. Every OTHER number in this report is read from a surviving `🗑️generated/fp12-*`
  file.
- **The wasm32 receipt is a freshness argument, not a fresh compile** (§4). It is backed by a
  measurement (a `touch` of a changed file that did not invalidate the unit) plus the `cfg(test)`
  mount sites, and by the plain native `--lib` check that DID recompile green.
- The two `TestSnapshot` codec surfaces are hand-written JSON, as they already were; the declared
  slot rides as `ArtifactRef` uris rather than through `pack::`/`ToValue`. That keeps every existing
  pack and DSL text byte-identical, and it is a FIXTURE codec, not a product one.
