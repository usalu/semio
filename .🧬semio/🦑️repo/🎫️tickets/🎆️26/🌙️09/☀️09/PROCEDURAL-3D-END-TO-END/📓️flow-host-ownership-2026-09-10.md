# FlowHost ownership, the finished-cursor admission law, and the contributed extension round trip (2026-09-10)

Lane: flow-host-ownership (Opus). Takes the three items
`📓️unit-suite-3d-2026-09-09.md` §5/§6.3 and `📓️eval-continuation-runtime-2026-09-10.md` §4.1 handed
over:

- **(a)** `FlowHost`'s coupled `Dictionary`/`FlowFixture` ownership — `history_store_from_baseline`,
  `evaluate_step`, `set_neuron_params`.
- **(b)** the mounted / record-body session admitting a document byte into a finished value cursor
  (3 law tests, shared with generation2d).
- **(c)** generation3d's `🧪️tests/🔬️flow-operators/🦀️.rs` contributed-extension install, whose round
  trip did not converge (13 tests).

Private target `$S/target-fh` (APFS clone of the eval lane's warm `target-eval/debug`),
`RUSTC_WRAPPER=""`, `RUST_MIN_STACK=536870912`, `--test-threads=2`, `--no-fail-fast`.
Raw logs: `🗑️generated/fh-*.txt`.

---

## 0. Status

**Done.** (a), (b) and (c) all landed and measured; two more leaks of the same family were found by
the suite and fixed (§4, §5). generation3d **289/23 → 315/5** (320), generation2d **179/46 → 228/2**
(230); both gates green (§7). Remaining reds all belong to sibling lanes (§6.1, §9).

### 0.1 Resume inventory (2026-09-10, second instance)

`git status --porcelain` + `git diff HEAD` over `🌊️flow` and `🌀️procedural` show the first instance
of this lane had landed the **ingress-predicate half of item (b)** and nothing of (a) or (c):

| file | state found |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:1203` | NEW `RetainedValueCursor::ingress_ready` |
| `…/🎒️pack/🌱️value/🦀️.rs:1410` | NEW `RetainedRecordBodyCursor::ingress_ready` (also consults the live value producer) |
| `…/🧊️generation3d/…/📸️snapshot/💾️binary/🦀️.rs` | NEW one-slot `document_byte: Option<(u64,u8)>` ingress between the catalog and the value cursor |
| `…/🌀️generation2d/…/📸️snapshot/💾️binary/🦀️.rs` | same, mirrored |
| `…/🧊️generation3d/…/🧬️mutations/💾️binary/🦀️.rs` Body phase | byte held in `self.pending` until `body.ingress_ready()` |
| `…/🌀️generation2d/…/🧬️mutations/💾️binary/🦀️.rs` Body phase | same, mirrored |
| `…/🌊️flow/🖥️host/🦀️.rs:1975` | only the ⚠️ docstring on `history_store_from_baseline` — the §5 fix itself still REVERTED |
| `…/🧊️generation3d/🧪️tests/🔬️flow-operators/🦀️.rs` | peer's contributed-manifest rewrite, unconverged |

Left behind by the stall and removed first: a `[DEBUG] begin_record …` `eprintln!` plus its
`stack_shape()` formatter in the generation3d mounted snapshot owner, and two in-definition `// 🚦️`
comments in the mutation sessions (CLAUDE.md forbids comments inside definitions; the explanation
lives on `ingress_ready`'s docstring).


---

## 1. Item (a) — `FlowHost`'s coupled `Dictionary`/`FlowFixture` ownership

### 1.1 What the three sites actually are

`FlowHost` displaces fail-closed roots in five places, not three. Two families:

| family | root | fail-closed because |
|---|---|---|
| `FlowFixture` | `layout: OrderedMap<WidgetLayout>` | `OrderedMap::drop` asserts `root.is_none()` (`🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81`) |
| neural `Dictionary` | `pairs: OrderedMap<Value>` | `Dictionary::drop` calls `OrderedMap::release_shared`, which is `Ok` for a NON-final share and `Err(retirement)` — i.e. an abort — for the FINAL owner (`🧠️neural/⚙️engine/🦀️.rs:97`) |

That asymmetry is the whole difficulty the previous instance recorded in
`📓️unit-suite-3d-2026-09-09.md` §5: a displaced `Dictionary` is usually still shared with
`NeuralCache` and with `previous_channels`, so it drops silently — until the cache sweeps its share
and the very same code path becomes a final owner and aborts the pool worker.

### 1.2 Design — a cold owner on the host, retired on session close

`FlowHost` gains ONE field (`🌊️flow/🖥️host/🦀️.rs:190`):

```rust
displaced: neural::ValueRetirement,
```

- every displacement site MOVES its displaced root into it instead of dropping it;
- `FlowHost::drain_displaced` (`🖥️host/🦀️.rs:390`) empties it through the bounded ladder
  (`close_step(64, 65_536)` until `Complete`), and is called at the **START** of `evaluate_step`
  (`:1071`) — so a root the CURRENT tick's cache or `previous_channels` still reads is never torn
  down under it, which is exactly the failure mode (`meshes empty; eval may have failed`) the
  previous instance measured when it retired `set_neuron_params`' bag inline;
- what is still on the frontier when the host closes is handed to `FlowHostRetirement::new`
  (`:2245`) as its initial `neural` frontier, so the existing close ladder drains it — nothing leaks
  and nothing is retired twice.

The invariant is therefore: **the frontier holds at most one evaluation tick's displacement, and it
never outlives the host.**

`FlowFixture` displacement is different and needs no deferral — a surplus baseline clone shares its
payload Arcs with the live fixture, so it is retired inline through the artifact's own frontier, the
shape `translate_ids` (`🖥️host/🦀️.rs:868`) already used.

### 1.3 The five sites, before → after

| site | before | after |
|---|---|---|
| `history_store_from_baseline` (`🖥️host/🦀️.rs:1987`) | `if self.history_store.is_none() { … }` — the `baseline` parameter was consumed on the FIRST call only and **dropped** on every later one | already-seeded branch retires `FlowOwner::Fixture(baseline)` through a local `FlowRetirement` and returns |
| `evaluate_step` (`:1099`, `:1119`) | `self.outputs = channels.outputs.clone();` / `self.previous_channels = Some(channels);` — both displaced values dropped | `std::mem::replace` → `displaced.push_dictionaries` / `previous_channels.replace` → `displaced.push_channels` |
| `set_neuron_params` (`:880`) | `*params = params.merge(&patch);` — the displaced bag AND `patch` itself dropped, `patch` also on both error paths | the widget lookup becomes a `match` producing `Result<Dictionary, FlowCoreError>`; `patch` is pushed to the frontier BEFORE the `?`, so no early return can drop it |
| `apply_eval_outputs_json` (`:374`) | same pair of assignments as `evaluate_step` | same replace/push pair |
| `install_eval_baseline` (`:399`) | `self.previous_channels = channels;` then `self.outputs = channels.outputs.clone();` over live values | `std::mem::replace` on both, displaced values pushed; the `outputs` write still happens only when a baseline was supplied |
| scene resync (`:272`) | `self.outputs.clear(); self.export_payloads.clear(); self.previous_snapshot = None; self.previous_channels = None;` — four bare drops, `BTreeMap::clear` being a bare drop of every `Dictionary` in it | one `displace_eval_state()` (`🖥️host/🦀️.rs:404`) that moves all four onto the frontier |

### 1.4 Collateral: `kind_infos` behind an `Arc`

A sibling lane (hot-path) moved `FlowHost.kind_infos` to `Arc<HashMap<String, OperatorInfo>>` while
this lane was working, and left `FlowHostRetirementState.kind_infos` typed as the bare map — the
whole `semio-framework-os-flow` crate did not compile (`E0308` at `🖥️host/🦀️.rs:2185`). Repaired in
place, since the retirement ladder is this lane's: the state field takes the `Arc`, and the ladder
step drains entries only while the host is the unique owner, otherwise releases the shared handle in
one charged step (`🖥️host/🦀️.rs:2263`). The shared case is the process-global catalogue
(`🗂️catalogue/🦀️.rs:218`), which outlives every host.

`semio-s-artifact-procedural-generation2d`'s `🧬️schema/🦀️.rs:13` had the same lane's half-landed
rename (`flow_neuron_kind_infos_json` imported, `flow_neuron_kind_info_map` called, `E0425`);
the import was corrected so the 2d suite could build.

---

## 2. Item (b) — the mounted / record-body session and the finished cursor

### 2.1 The predicate (landed by the first instance, kept)

`RetainedValueCursor::admit_byte` is fail-closed on FOUR conditions
(`🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:813`):
`closed || sealed || stack.is_empty() || pending.is_some()`. The first three mean **finished** — the
top-level value is done and no further document byte belongs to it — the fourth means **not ready
yet**. A producer that hands a byte down on a fixed cadence cannot tell those apart, and both come
back as the same untyped handback, which the sessions reported as
`generation3d-mounted.value-backpressure` / `generation3d-mutation.body-malformed`.

Two predicates now exist so a producer can ask before it admits:

- `RetainedValueCursor::ingress_ready` (`🎒️pack/🌱️value/🦀️.rs:1203`) — `!closed && !sealed &&
  !stack.is_empty() && pending.is_none()`;
- `RetainedRecordBodyCursor::ingress_ready` (`:1410`) — the same for its own slot AND, once a value
  producer is live, `value.is_none_or(RetainedValueCursor::ingress_ready)`, because every byte this
  cursor takes is handed straight down.

Both sessions were rewired to hold the byte rather than force it:

- `Generation3dMountedPackSession` / `Generation2dMountedPackSession` gained a one-slot
  `document_byte: Option<(u64, u8)>` ingress between the catalog and the value cursor; the catalog is
  only pumped again while that slot is empty, the slot is drained in `retire`, and
  `terminal_is_empty` accounts for it;
- both `Generation*MutationSession`'s `Body` phase admits from `self.pending` only under
  `body.ingress_ready()`.

A byte that arrives after the top-level value is finished therefore stays in the one-slot ingress and
is released by the session's own retirement, instead of being pushed into a finished cursor and
coming back as a generic backpressure fault.

### 2.2 What was still broken — generation3d had never received generation2d's decode repairs

With the ingress law in place, the two artifacts' three laws diverged: 2d passed, 3d moved on to a
different class of fault (`generation3d-mounted.widget-statements-owner`, then
`generation3d-mutation.field-owner`, then `generation3d-mutation.root-string-field`). Diffing the two
owners with the artifact prefixes normalised away showed 3d had simply never received a set of
repairs 2d already carried. Ported, verbatim in structure:

| repair | 3d snapshot owner | 3d mutation owner |
|---|---|---|
| structural depth `12 → 64` (a `Widget::Neuron`'s `params` is a `Dictionary` whose entries are themselves `Value::Dictionary`; each level costs several pack frames) | `GENERATION3D_MOUNTED_TYPED_DEPTH` (`📸️snapshot/💾️binary/🦀️.rs:54`) | `GENERATION3D_RETAINED_COMBINED_DEPTH` **deleted**, every use now `GENERATION3D_RETAINED_STACK_CAPACITY` (already 64) |
| a `Dictionary` also reaches the wire as a `List` of one-entry records, not only as a columnar `Table` | new `Generation3dMountedNeuralOwner { TableRow, EntryRow }`, new `DictionaryEntries`/`DictionaryEntry` container owners, `DictionaryEntryKey` string target, and the matching `begin_record`/`begin_container`/`end_container`/`Role::FieldId` arms | the same, as `Generation3dMutationNeuralOwner` + `DictionaryEntries`/`DictionaryEntry` frames |
| synapse wire roles are 0/2/3/5, not 0/1/3/4 | `from_port` `1 → 2`, `to_port` `4 → 5` | same |
| `move-widget`'s `id` is a root string field | — | `(2 \| 5 \| 7 \| 9 \| 11, 0)` → `(2 \| 5 \| 6 \| 7 \| 9 \| 11, 0)` |

`🔬️mounted-registry`'s source assertion was updated to the surviving constant
(`✏️editor/🌉️wasm/🧪️tests/🔬️mounted-registry/🦀️.rs:529`).

### 2.3 A third law was failing on a test-side ownership gap

`cancelled_and_stale_aba_initializers_retire_to_terminal_empty` asserted the stale initializer's
outcome discriminant and then let the value fall out of scope:

```
RetainedJobPayload requires one-page close to terminal-empty; ordinary Drop intentionally preserves page backing
  2: <RetainedJobPayload as Drop>::drop
  4: drop_glue::<JobFault>
  5: drop_glue::<StepOutcome>
  6: …cancelled_and_stale_aba_initializers_retire_to_terminal_empty
```

`StepOutcome::Fault(JobFault { detail: RetainedJobPayload })` owns preadmitted payload pages whose
`Drop` deliberately preserves its page backing (`🧵️job/🦀️.rs:658`). The law now drains both terminal
outcomes through `StepOutcome::close_step` via a new `close_outcome` helper
(`🧬️mutations/💾️binary/🧪️tests/🔬️retained-authority-laws/🦀️.rs:207`).

### 2.4 Result — the laws in BOTH artifacts

```
generation3d  (--features component-app-assembly --lib, filters retained_authority_laws retained_mounted_laws)
  before: 4 passed / 3 failed   generation3d-mounted.value-backpressure, generation3d-mutation.body-malformed,
                                RetainedJobPayload requires one-page close
  after : 7 passed / 0 failed

generation2d  (same filters)
  after : 4 passed / 0 failed
```

---

## 3. Item (c) — generation3d `🔬️flow-operators` contributed-install convergence

No code change was needed here in the end. A peer landed the two-half arrangement
(`🧪️tests/🔬️flow-operators/🦀️.rs`: `register_linked_flow_extension_installer` for the EVALUATE hop under
the flow-domain ids `brep`/`math`, `install_flow_extension_manifest` for the TESSELLATE hop under the
plugin ids `flow-extension-brep`/`flow-extension-math`) together with the registry's plugin-id
addressing (`🌊️flow/📔️registry/🦀️.rs` — `ContributedExtensionStub.invocation_address` is the
CONTRIBUTING PLUGIN's id, not the flow manifest's `id`). What kept the thirteen geometry tests red was
NOT that round trip: it was the `FlowHost` ownership aborts of item (a), which killed the pool worker
before any evaluation produced geometry. With (a) landed, every one of them converges:

```
editor::generation3d::commands::remove_widget::tests::patch_flow_widgets_recomputes_preview_geometry ... ok
editor::generation3d::commands::translate_selection::tests::translate_selection_persists_transform_into_flow_graph ... ok
editor::generation3d::commands::translate_selection::tests::rotate_and_scale_selection_persist_into_flow_graph ... ok
editor::generation3d::component::tests::preview_payload_has_meshes_and_instances ... ok
editor::generation3d::modes::edit::windows::preview::tests::renders_world_preview_scene ... ok
editor::generation3d::modes::edit::windows::preview::tests::switching_active_example_changes_preview_meshes ... ok
```

## 4. One more leak of the same family, found by the suite

`preview_eval_exact_window_transient_isolates_and_resets_in_the_registered_app` aborted with the same
`ordered-map root must be explicitly retired before drop`, this time inside the framework:

```
  2: <OrderedMap<WidgetLayout> as Drop>::drop
  4: drop_glue::<FlowFixture>
  5: drop_glue::<Generation3dSnapshot>
  6: <VcsArtifactApp<EditorApp<Generation3dPlayApp>> as PluginApp>::load_document_pack
```

`ParsedDocumentText::into_envelope` (`🏪️store/🦀️.rs:10625`) had been ADDED for exactly this — take the
envelope and retire the replayed projection beside it — and then wired at zero call sites. All five
`store::…::reset(parsed.envelope, …)` partial moves now go through it:
`🔌️plugin/🦀️.rs:25254` (draft store), `:25366` (config store), `:25408`, `:25428` (document store) and
`🔌️plugin/🪟️window/🎚️config/🦀️.rs:467`.

## 5. A leaked publication lease, hardened

With the laws green, the only remaining red inside this lane's own files in a FULL-suite run was
`cancelled_and_stale_aba_initializers_retire_to_terminal_empty` reporting
`generation3d-publication.saturated` — the cascade `📓️unit-suite-3d-2026-09-09.md` §6.3 predicted,
now measured directly:

- `vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed` panics at
  `assert_eq!(drive_production_envelope(…), Ready)` (it gets `Fault` — the envelope-load lane's
  defect, §6.3), which is BEFORE its `generation3d_release_publication_authority`;
- the lease table is a process-global 4-slot `FixedOperationRegistry`, so that one leaked slot is
  never reclaimed and every later law in the binary answers `saturated`.

The lease is now owned by an RAII guard (`✏️editor/🧪️tests/🔬️unit/🦀️.rs:147`,
`Generation3dProductionLease`): the explicit `assert!(lease.release())` still asserts the real
release, and `Drop` reclaims the slot on any panicking path. That makes the law suite independent of
whether a peer's law in the same binary fails.

---

## 6. Before → after

### 6.1 generation3d — `--features component-app-assembly --lib`, `--test-threads=2`

| run | result |
|---|---|
| lane hand-over baseline (`g3d-44`, `📓️unit-suite-3d-2026-09-09.md` §6.2) | **289 passed / 23 failed** (312) |
| best previously measured on an unchanged peer tree (`g3d-30`, §6.1) | **301 passed / 10 failed** (311) |
| **after this lane** (`🗑️generated/fh-18-generation3d-suite-final.txt`) | **315 passed / 5 failed** (320) |

The five reds, all attributed elsewhere and all unchanged in character:

| test | owner |
|---|---|
| `component::preview_eval_exact_window_transient_isolates_and_resets_in_the_registered_app` — now `registered fixture close blocked: transient read remains live` (it used to abort earlier, on the `ordered-map root` drop in `load_document_pack` fixed in §4) | window-transient close ladder |
| `component::generation_preview_is_one_app_transient_shared_by_two_generation_windows` | window-transient close ladder |
| `component::refresh_pending_effects_arms_flow_eval_tick_chain` — `registered fixture typed operation did not retire within 30 seconds` | hot-path lane (`📓️unit-suite-3d-2026-09-09.md` §2 class B.2: the first `flowEvalTick` costs 25 s) |
| `component::two_instances_converge_disjoint_widget_moves` — `module.vcs … remote snapshot merge is fail-closed` | excluded by this lane's brief |
| `component::vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed` — `left: Fault, right: Ready` | envelope-load lane (`📓️unit-suite-3d-2026-09-09.md` §6.3) |

Retained-law filter alone (`retained_authority_laws retained_mounted_laws`): **4 passed / 3 failed →
7 passed / 0 failed** (`🗑️generated/fh-08-3d-laws.txt`).

### 6.2 generation2d — same invocation

| run | result |
|---|---|
| lane hand-over baseline (`📓️unit-suite-2026-09-09.md` §1) | **179 passed / 46 failed** (225) |
| **after this lane** (`🗑️generated/fh-19-generation2d-suite-final.txt`) | **228 passed / 2 failed** (230) |

The two reds are the same two as 3d's, `two_instances_converge_disjoint_widget_moves` and
`vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed` — neither is
this lane's.

Retained-law filter alone: **4 passed / 0 failed** (`🗑️generated/fh-09-2d-laws.txt`).

## 7. Gates

### 7.1 `cargo check -p semio-s-plugin-procedural --keep-going` (native)

```
    Checking semio-s-plugin-procedural v0.1.0 (…/✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 2m 00s
```
0 errors (`🗑️generated/fh-20-native-check.txt`).

### 7.2 `cargo check -p semio-s-plugin-procedural --target wasm32-wasip2 --profile wasm-dev`

`CARGO_PROFILE_WASM_DEV_DEBUG=false`, `CARGO_INCREMENTAL=0`:

```
    Checking semio-s-plugin-procedural v0.1.0 (…/✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust)
    Finished `wasm-dev` profile [unoptimized] target(s) in 1m 52s
```
0 errors (`🗑️generated/fh-22-wasm-check.txt`). The first attempt died on `No space left on device`
(the shared scratchpad was at 94 % with several lanes' target dirs live) — not a code failure; the
retry with `CARGO_INCREMENTAL=0` went through.

---

## 8. Files changed

**Framework — `🌊️flow`**

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs`
  - `:197` new `displaced: neural::ValueRetirement` field (+ `:250` init);
  - `:414` `drain_displaced`, `:421` `displace_eval_state`;
  - `:272` scene resync, `:370`/`:375` `apply_eval_outputs_json`, `:401`/`:405`
    `install_eval_baseline`, `:930`/`:931` `set_neuron_params`, `:1075` tick-start drain,
    `:1103`/`:1121` `evaluate_step` — every displacement routed;
  - `:2040` `history_store_from_baseline` retires the surplus baseline;
  - `:2160` `FlowHostRetirementState.kind_infos: Arc<…>`, `:2248` takes the host's frontier over,
    `:2311` ladder step drains or releases the shared catalogue.

**Framework — `🔌️plugin` (the `ParsedDocumentText` load seams)**

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:25254`, `:25366`, `:25408`, `:25428`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs:467`

**generation3d**

- `…/🧬️schema/📸️snapshot/💾️binary/🦀️.rs` — depth 64, `Generation3dMountedNeuralOwner`,
  `DictionaryEntries`/`DictionaryEntry`, `DictionaryEntryKey`, synapse roles 2/5; the leftover
  `[DEBUG] begin_record` `eprintln!` and its `stack_shape` formatter removed.
- `…/🧬️schema/🧬️mutations/💾️binary/🦀️.rs` — the same dictionary-entry family as
  `Generation3dMutationNeuralOwner` + two frames, synapse roles 2/5, `move-widget`'s root string
  field, `GENERATION3D_RETAINED_COMBINED_DEPTH` deleted in favour of
  `GENERATION3D_RETAINED_STACK_CAPACITY`; the Body-phase in-definition comment removed.
- `…/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️retained-authority-laws/🦀️.rs` — `close_outcome` helper
  and its two call sites.
- `…/✏️editor/🌉️wasm/🧪️tests/🔬️mounted-registry/🦀️.rs:529` — the depth-constant source assertion.
- `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:147` — `Generation3dProductionLease` RAII guard, both call sites.

**generation2d**

- `…/🧬️schema/🦀️.rs:13` — `flow_neuron_kind_info_map` import (peer's half-landed rename).
- `…/🧬️schema/🧬️mutations/💾️binary/🦀️.rs` — Body-phase in-definition comment removed.

## 9. Owed follow-ups

- `preview_eval_exact_window_transient_isolates_and_resets_in_the_registered_app` and
  `generation_preview_is_one_app_transient_shared_by_two_generation_windows` need the window-transient
  close ladder (`registered fixture close blocked: transient read remains live`).
- `refresh_pending_effects_arms_flow_eval_tick_chain` needs the hot-path lane's first-tick budget
  (25 s against a 30 s liveness guard).
- `vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed` (both
  artifacts) needs `ArtifactEnvelopeDecodeOperationPoll::Fault` to carry its fault code before the
  ACCEPTED swap's failure can be diagnosed — envelope-load lane.
- `two_instances_converge_disjoint_widget_moves` (both artifacts) is the `module.vcs` fail-closed
  remote-merge, excluded by this lane's brief.
- The `kind_infos` ladder step releases the SHARED catalogue handle without draining its entries. That
  is correct today because the only sharer is the process-global `flow_neuron_kind_info_map`
  (`🗂️catalogue/🦀️.rs:218`), which outlives every host; if a second, host-scoped catalogue ever shares
  that `Arc`, its entries need their own retirement owner.

## 10. Concurrency notes

Two sibling lanes were editing the same files throughout and left the tree uncompilable three times:
`FlowHost.kind_infos` moved to `Arc<…>` without the retirement state (`E0308`, §1.4), generation2d's
`flow_neuron_kind_info_map` import (`E0425`), and an `ArtifactEditor::pending_effects` signature that
grew an `Option<&ViewModel>` parameter and reached the artifact's lib before its tests. The first two
were repaired here because they are in files this lane owns; the third was waited out (polling
`cargo check --lib --tests` until it went clean) rather than raced.
