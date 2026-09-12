# Contributed manifest: loud failure + registry-generation evaluation baseline (2026-09-12)

Acts on `📓️audit-unknown-kind-2026-09-12.md` §2 (item 2 of §5) and §3. Every result below was run
in this session, in the foreground; raw output is under `🗑️generated/manifest-loud/`.

---

## 1. `register_contributed_manifest` now faults instead of silently registering zero operators

`📔️registry/🦀️.rs:125` was

```rust
fn register_contributed_manifest(registry: &mut neural::Registry, plugin_id: &str, manifest_json: &str) {
    let Ok(manifest) = crate::os_pack::json::from_json_str::<FlowExtensionManifest>(manifest_json) else { return };
```

so ONE contributor's malformed `manifestJson` emptied its whole operator pack out of the composed
registry while `sync_host_flow_extension_contributions[_page]` still answered `Ok` and the guest
`setContributions` command still reported success. The only symptom reached the surface a tick later
as `unknown kind: <operator>`, nowhere near the contributor that caused it.

### What changed (`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs`)

- New typed fault, shaped exactly like the existing `FlowExtensionAddressMiss` next to it — a public
  struct naming the contributor, a single `CODE` const, and `labels()` answering English **and**
  German with no default language:

  ```rust
  pub struct FlowExtensionManifestRejection { pub plugin_id: String, pub reason: String }
  impl FlowExtensionManifestRejection {
      pub const CODE: &'static str = "flow.extension-manifest-invalid";
      pub fn labels(&self) -> (String, String) { … }
  }
  ```

  `flow.extension-manifest-invalid` is the code the contribution fold and
  `install_flow_extension_manifest` ALREADY raised for unreadable manifest *metadata*; both now read
  `FlowExtensionManifestRejection::CODE` instead of repeating the literal, so the taxonomy has one
  declaration.
- `register_contributed_manifest -> Result<(), FlowExtensionManifestRejection>`.
- `build_flow_extension_registry -> Result<neural::Registry, FlowExtensionManifestRejection>`; the
  half-built registry is retired by its `neural::ColdOwner` on the `?`, so a refusal publishes no
  registry, changes no contribution table and burns **no** replacement generation.
- Every boundary (`sync_host_flow_extension_contributions`, `…_page` through it,
  `install_flow_extension_manifest`, `install_flow_extension`, `uninstall_flow_extension`) maps it to
  `FlowExtensionManifestRejection::CODE` and keeps its `Result<(), &'static str>` signature — which is
  what the generation3d editor/viewer and generation2d `set-contributions` commands already turn into
  a `Fault` via `.map_err(Fault::from)?`. **No command module was edited** (the generation3d
  `set-contributions` / `flow-eval-tick` lane was left untouched as instructed).

### The row the old metadata probe could never catch

`fold_host_flow_extension_contributions` only decodes `FlowExtensionMetadata { id, name, version }`.
A manifest whose id/name/version read perfectly but whose `contributes` is missing/renamed/not a
record passed that probe and then silently contributed nothing. That is the fixture's
`metadata-only` row, and it is now a `Fault`.

---

## 2. Language-agnostic fixture + Rust law + TypeScript twin

**Fixture** — `📔️registry/🧫️fixtures/🔣️manifest-admission.json` (`schema:
"flow.extension-manifest-admission"`). Declares the fault code, the required key lists
(`requiredManifestTextKeys` / `…ListKeys` / `…RecordKeys`, `requiredContributesListKeys`,
`requiredOperatorTextKeys`, `requiredSchemaTextKeys`), a `generationBump` contributor, and eight rows,
each carrying the exact `manifestJson` bytes plus its expected decision:

| row | accepted | why |
|---|---|---|
| `valid-full` | ✅ | a complete manifest registers every operator and schema it declares |
| `valid-empty-operators` | ✅ | declaring nothing is legal — an empty contribution is not a decode failure |
| `truncated-mid-object` | ❌ | a payload cut mid-object is not JSON at all |
| `metadata-only` | ❌ | id/name/version pass the fold's metadata probe; only the full decode catches it |
| `contributes-renamed-operators` | ❌ | a producer writing `nodes` where the contract says `operators` |
| `contributes-not-an-object` | ❌ | `contributes` is a record, never a list |
| `missing-activation-events` | ❌ | every declared manifest field is required, not defaulted |
| `operator-missing-abbreviation` | ❌ | strictness reaches into every contributed operator |

**Rust laws** — `📔️registry/🧪️tests/📔️registry/🦀️.rs`:

- `a_contributed_manifest_is_admitted_or_faulted_per_fixture_row` decides every row **twice** — once
  through `register_contributed_manifest`, once through an independent `serde_json` oracle
  (`third_party_manifest_is_admissible`) — and requires the two to agree. Accepted rows must have
  registered exactly the operators/schemas the row names; refused rows must answer a rejection that
  names the contributor, carries a non-empty reason, and yields two distinct languages.
- `a_malformed_contributed_manifest_faults_the_whole_contributions_sync` pushes the `metadata-only`
  row through the real `sync_host_flow_extension_contributions` wire shape
  (`[{pluginId, topicContribution:{topic:"flow.extension", payload:{manifestJson}}}]`) and asserts
  `Err("flow.extension-manifest-invalid")`, an unmoved generation, and nothing installed.

**TypeScript twin** — `📔️registry/🧪️tests/🪪️manifest-admission/🟦️.ts`
(`flowExtensionManifestAdmissionSelfTests`). `JSON.parse` + `node:assert/strict`, same fixture, same
accept/reject decision per row, plus a source contract over `📔️registry/🦀️.rs` (Result-returning
signature, `CODE`, propagation `?`, the German label, and no `let Ok(manifest) = … else { return }`
statement) with four hostile mutants. Registered as the first step of the already-launchable
`semio-framework-os-flow-core:test-source` target (`🫀️core/📦️packages/🦀️rust/📜️script.ts`) — no new
`launch.json` entry needed, that target is at `.vscode/launch.json:5027`.

---

## 3. §3 verified at HEAD, and the real hole that was left

**§3's hypothesis is FALSE at HEAD.** `FlowEvalSession::invalidate_for_flow_extension_registry`
(`🖥️host/🦀️.rs`) calls `set_eval_json(String::new())`, and `set_eval_json` DOES
`state.previous_snapshot.take()` / `state.previous_channels.take()` (lines 2720-2725). The
incremental baseline is already cleared. Pinned by a new law rather than left implicit:
`an_invalidated_session_hands_an_ephemeral_host_a_re_dispatching_baseline`.

**The hole that WAS live** is one level down, in `FlowHost` itself: `compute_dirty_set` diffs the
TREE, and a contributed operator arriving in a registry replacement changes no tree at all. A host
whose baseline survived a registry replacement answered from that baseline forever. Fixed:

- `FlowHost` gained `baseline_registry_generation: u64`, advanced together with
  `previous_snapshot`/`previous_channels` and only on success, in `evaluate_step` and in
  `apply_eval_outputs_json`. The generation is read *before* `flow_registry()`, so a replacement
  landing between the two reads over-dirties the next step (extra work, never a stale answer).
- `install_eval_baseline(snapshot, channels, registry_generation)` — the generation travels WITH the
  baseline instead of being read live, because an ephemeral host is rebuilt *after* the install that
  bumped it. `FlowEvalSession::install_baseline_into` passes its own
  `flow_extension_generation`; `capture_baseline_from` carries the host's stamp back, monotonically
  and only when a snapshot was actually captured, so it can never cancel a pending invalidation while
  still keeping the fast path alive for sessions nobody invalidates.
- `baseline_is_current()` / `current_baseline_snapshot()` / `current_baseline_channels()` gate the
  snapshot diff AND the previous channels in `evaluate_step` and `pending_eval_widget_ids`, and
  `baseline_answers_everything(&dirty)` is now the ONE fast-path predicate both read.
  **Refusing only the skip is not enough** — that was the first attempt, and it failed the law: a
  refused skip that still diffs against the superseded snapshot computes an EMPTY dirty set and
  dispatches nothing. The baseline has to be dropped whole.

New law `a_superseded_registry_generation_re_dispatches_an_unchanged_tree` counts bridge dispatches
on an ephemeral host with its own neural cache (the shape `flow_host_with_session` rebuilds per
tick): current generation ⇒ 0 dispatches and no pending work; after a real
`install_flow_extension_manifest` bump ⇒ the unchanged tree is dispatched again and reaches the same
answer. It fails on the pre-fix code.

---

## 4. Collateral repairs (needed to make any of the above runnable)

The flow host test module was 9 passed / 90 failed at HEAD. All of it was fail-closed retirement
debt, not logic:

| repair | file | effect |
|---|---|---|
| `kind_infos_json()` helper retires the law-local operator catalogue (`ChannelSpec::default` carries a `Value`) | `🖥️host/🧪️tests/🔬️unit/🦀️.rs` | unblocked ~80 laws stuck on `final Dictionary ownership must be explicitly retired` |
| `FlowHost::install_kind_infos` retires the catalogue it displaces when this host was its last owner | `🖥️host/🦀️.rs` | **production leak** — a plain `self.kind_infos = …` aborted the process on the second `set_neuron_kind_infos_json` |
| `explode_cluster` borrows `tree`/`flow` from ONE retired clone and retires the removed cluster widget | `🖥️host/🦀️.rs` | **production leak** — `OrderedMap<FlowNodeGui>` dropped bare; was double-panicking into `SIGABRT` and killing the whole test binary |
| `host.retire_cold()` added to 77 laws that dropped a populated `FlowFixture` | `🖥️host/🧪️tests/🔬️unit/🦀️.rs` | `ordered-map root must be explicitly retired before drop` |
| fixture completed with `label: null`, `operators: []`, `variadicInput/Output: null`, `group: []` | `📔️registry/🧫️fixtures/🔣️.json` | `FieldSpec::label` / `OperatorInfo` gained fields that `ToValue` always emits; the fixture had gone stale |
| `registry_maintenance_does_not_initialize_a_registry` measures presence *across* the step | `📔️registry/🧪️tests/📔️registry/🦀️.rs` | was a statement about libtest scheduling, not about maintenance |
| `FLOW_EXTENSION_REGISTRY_TEST_LOCK` + `drain_flow_extension_registry_retirements()` | `📔️registry/🦀️.rs` (`#[cfg(test)]`) | the registry is ONE process-wide singleton and libtest is parallel; every law that mutates it now serializes and drains leftovers first |

---

## 5. Results — every command run in this session, foreground

```
$ RUST_MIN_STACK=33554432 cargo test -p semio-framework-os-flow --lib -- registry:: --test-threads=1
test registry::tests::a_contributed_manifest_is_admitted_or_faulted_per_fixture_row ... ok
test registry::tests::a_malformed_contributed_manifest_faults_the_whole_contributions_sync ... ok
test registry::tests::contributed_operators_are_addressed_by_their_contributing_plugin_id ... ok
test registry::tests::contributed_registry_replacement_preserves_readers_and_drains_old_versions ... ok
test registry::tests::registry_maintenance_does_not_initialize_a_registry ... ok
test registry::tests::registry_maintenance_retains_cursor_outside_a_faulted_worker ... ok
test registry::tests::registry_replacement_admission_preserves_roots_on_capacity_and_generation_exhaustion ... ok
test result: ok. 7 passed; 0 failed          (at session start: 3 passed / 2 failed)
```

```
$ RUST_MIN_STACK=33554432 cargo test -p semio-framework-os-flow --lib \
    -- host::tests::an_invalidated_session host::tests::a_superseded_registry --test-threads=1
test host::tests::a_superseded_registry_generation_re_dispatches_an_unchanged_tree ... ok
test host::tests::an_invalidated_session_hands_an_ephemeral_host_a_re_dispatching_baseline ... ok
test result: ok. 2 passed; 0 failed
```

```
$ RUST_MIN_STACK=33554432 cargo test -p semio-framework-os-flow --lib -- host::tests:: --test-threads=1
test result: FAILED. 78 passed; 21 failed    (at session start: 9 passed / 90 failed, and after the
                                              Dictionary-gate repair it aborted with SIGABRT before
                                              finishing — it now runs to completion)
```

```
$ RUST_MIN_STACK=33554432 cargo test -p semio-framework-os-flow --test flow_mesh_pack_wire
test result: ok. 7 passed; 0 failed
```

```
$ RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-procedural-generation3d \
    --features component-app-assembly --lib -- a_late_contributions_install set_contributions --test-threads=1
test editor::…::flow_eval_tick::tests::a_later_set_contributions_re_arms_the_chain_the_gate_stopped ... ok
test editor::…::set_contributions::tests::a_one_page_host_shaped_run_indexes_contributed_operators ... ok
test editor::…::set_contributions::tests::a_paged_run_installs_the_contributed_registry ... ok
test editor::…::set_contributions::tests::an_invalid_page_address_is_refused ... ok
test editor::…::set_contributions::tests::an_out_of_order_run_is_refused_and_discarded ... ok
test editor::…::set_contributions::tests::the_packaged_brep_manifest_parses_as_a_flow_extension_manifest ... ok
test editor::generation3d::component::tests::a_late_contributions_install_re_arms_the_evaluation_the_empty_registry_faulted ... ok
test viewer::generation3d::component::eval_chain_tests::a_late_contributions_install_re_arms_the_viewer_evaluation_the_empty_registry_faulted ... ok
test result: ok. 8 passed; 0 failed
```

```
$ bun 🧰️framework/…/🌊️flow/📔️registry/🧪️tests/🪪️manifest-admission/🟦️.ts
[twin] flow extension manifest admission: 29 assertions

$ bun nx run semio-framework-os-flow-core:test-source --skip-nx-cache
semio-framework-os-flow-core: [DEBUG] flow extension manifest admission twin: 29 assertions
… then FAILS on the pre-existing red below.
```

```
$ cargo check -p semio-s-artifact-procedural-generation2d --features component-app-assembly --lib
(clean — the only other consumer of the changed sync surface)
```

The unlinked law named in the brief is green and was run twice (once alone, once with the
`set_contributions` family). No new compiler warning mentions any symbol added here.

---

## 6. Pre-existing reds I did NOT cause and did NOT fix

Each was confirmed pre-existing by a neutralisation probe (revert the suspect clause, re-run, still
red) or by reading the offending source.

1. **`semio-framework-os-flow-core:test-source`** fails at
   `🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts:36`,
   `assert(sceneConsumers[1][1].includes("assert!(turns > 1_600)"))`. That string no longer exists in
   `♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs` (last touched 2026-09-08). My twin is
   deliberately the FIRST step of that target so it still runs and reports.
2. **21 remaining `host::tests` reds** — the same fail-closed retirement class, but each needs a real
   owner decision rather than a mechanical `retire_cold()`: `explode_cluster`-adjacent cluster laws,
   `collect_live_*` handle laws holding loose `Dictionary`s, the geometry fixtures
   (`rectangle_extrude_*`, `hexagonal_mushroom_*`), and three genuine assertion failures
   (`flow_eval_session_seeds_its_retained_neural_cache`,
   `delete_selection_removes_edge_selected_by_synapse_id_domain`,
   `hexagonal_mushroom_fixture_reports_extruded_solid_output`).
3. **`semio-framework-os-flow --lib` whole-binary run: 153 passed / 57 failed.** Beyond the 21 above,
   `vcs::flow_vcs_tests` (24), `wasm_session::domain_laws` (7), `drawing` (1), `extensions::wasm` (1),
   and 3 `registry::tests` that are green under their own filter but contaminated in the full binary
   by `install_first_party_light_flow_extensions_for_tests` (a `Once` that permanently installs nine
   manifests) and by `registry_replacement_admission_…`'s own closing
   `sync_host_flow_extension_contributions("[]")`, which wipes them again. Making those two
   cooperate is a separate piece of work on the process-wide singleton.
4. **6 `semio-s-artifact-procedural-generation3d` reds** — `add_generation_records_an_undoable_generation_operation`,
   `undo_redo_round_trips_flow_graph_edits`, `two_instances_converge_disjoint_widget_moves`,
   `generation_preview_is_one_app_transient_shared_by_two_generation_windows`,
   `hex_column_boot_stays_inside_the_interactive_turn_budget`,
   `vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed`. All fail
   inside `semio-framework-plugin`'s undo/redo conformance testkit
   (`🔌️plugin/🦀️.rs:6843`/`:6866`, "undo did not revert to the expected snapshot"). **Probed twice**:
   neutralising the registry-generation clause, and then neutralising `install_kind_infos`'
   retirement, left both laws red — this is somebody else's in-flight undo/VCS work, not this change.

---

## Files touched

Production:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs`

Fixtures / tests / wiring:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🧫️fixtures/🔣️manifest-admission.json` (new)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🧪️tests/📔️registry/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🧪️tests/🪪️manifest-admission/🟦️.ts` (new)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/📜️script.ts`
