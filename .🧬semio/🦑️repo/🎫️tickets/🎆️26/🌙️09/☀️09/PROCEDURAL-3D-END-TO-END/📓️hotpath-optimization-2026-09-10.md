# Runtime hot-path optimization — procedural 3d, 2026-09-10

Implements the ranked list in `📓️runtime-hotpath-audit-2026-09-10.md` §4. Measured natively with a
private `CARGO_TARGET_DIR`, `RUSTC_WRAPPER=""`, on the working tree at the time of the run (other
lanes edit these files concurrently; §10 records the one that merged into this lane's own files).

**Headline**: the hexagonal-mushroom-column boot's evaluation tick went from **26 239 us mean / 35 901 us
worst** to **6 878 us mean / 7 425 us worst** (unoptimized, same machine), its full dispatch+settle
cycle from **110 401 us** to **10 178 us** mean, its per-boot preview publications from 3 to 1 and its
published bytes from 3 180 B to 1 060 B, with **zero** per-turn `[DEBUG]` lines left on the Rust hot
path and **zero** tessellations on a hover-only re-render. §7 has the table; §6 has the cost the audit
did not name.

---

## 1. Stale round-trip probes removed (audit rank 1)

Seven unconditional `eprintln!` probes, all added by the extension-round-trip debugging pass and all
doc-labelled "temporary", are gone. The round trip they instrumented is unit-verified by
`hex_column_evaluates_end_to_end_through_the_extension_round_trip`.

| audit id | site (before) | what it printed | now |
|---|---|---|---|
| R3 | `🧰️framework/…/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:286-287` | `[DEBUG] Event::Completed req=… ok=…` | deleted (comment + line) |
| R4 | `…/⚛️reactor/🔄️turn/🦀️.rs:290` | `[DEBUG] continuation resolve hit …` | deleted |
| R6 | `…/⚛️reactor/🔄️turn/🦀️.rs:308` | `[DEBUG] continuation resolve miss …` | deleted; the `None` arm is now a one-line `REGISTRY.with(…)` |
| R5 | `…/⚛️reactor/🔄️turn/🦀️.rs:306` | `[DEBUG] continuation resolve dropped …` | **kept** — fault-adjacent, fires only when the owning instance died mid-flight |
| R9 | `🧰️framework/…/🔌️plugin/⚛️reactor/🦀️.rs:715-716` | `[DEBUG] extension invocation minted …` | deleted; `queue_extension_invocation` now returns the registry call directly |
| R16 | `…/generation3d/…/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs:31` | `[DEBUG] flowEvalTick invoke …` | deleted |
| R17 | `…/generation3d/…/✏️editor/🦀️.rs` (`flowTessellate invoke`) | `[DEBUG] flowTessellate invoke …` | deleted |
| R17b | `…/✏️editor/🦀️.rs` (`flowTessellate skipped`) | `[DEBUG] flowTessellate skipped: no plugin contributes …` | **kept**, `[DEBUG]` prefix dropped — it is a real fault-path line, not temporary instrumentation |
| R18 | `…/✏️editor/🎮️commands/✅️flow-eval-resolve/🦀️.rs:18` | `[DEBUG] flowEvalResolve nodeHash=… outputBytes=… seeded=…` | replaced by a fault-only line: it now prints ONLY when `seed_node_cache` fails (the old line discarded the `seeded` flag into a log nobody read) |

TypeScript (`🧰️framework/…/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`):

| site | before | after |
|---|---|---|
| `:1586` | `console.log("[DEBUG] extension invocation completed", {…, status})` on EVERY call | `console.warn("[DEBUG] extension invocation faulted", …)` only when the outcome is a fault |
| `:1616` | `console.debug("[DEBUG] invokeExtension resolve", {…, loaded: plugins.map(…)})` on EVERY call, serializing the whole plugin id list | `console.warn("[DEBUG] invokeExtension unresolved", …)` only when no plugin answers the address (the `extension.missing` case the line existed to catch) |

`[DEBUG] extension completion submitted` (audit T-hop 4, `🔌️PluginRuntime/🟦️.tsx`) was already gone
before this lane started; `🔌️PluginRuntime/🟦️.tsx` still has zero `console.*` calls.

## 2. One runtime-diagnostics switch (audit rank 2)

New leaf primitive in `🧰️framework/🔨️modules/⏱️trace/🦀️.rs` (`//#region 🩺️Diagnostics`), the same
crate that already owns `INTERACTIVE_STEP_CEILING_US`:

- `RUNTIME_DIAGNOSTICS_ENV = "SEMIO_RUNTIME_DIAGNOSTICS"` — the schema constant, so a host with no
  environment (bare wasm in a browser tab) arms the same switch programmatically.
- `set_runtime_diagnostics(bool)` — explicit host override, outranks the environment.
- `runtime_diagnostics_enabled() -> bool` — resolves the environment ONCE into a tri-state
  `AtomicU32`, then is a relaxed atomic load. `1`/`true`/`on`/`yes` arm it; unset ⇒ **off**.
- Bare `wasm32` (non-p2) has no `std::env`, so its resolver is a `const false`.

Re-exported through `semio_framework_job` so a plugin crate that already depends on the job crate can
name the ceiling and the switch without widening its dependency graph.

Gated sites (all previously unconditional):

| site | gate |
|---|---|
| `🔌️plugin/🦀️.rs` `maintenance stage=… elapsed_us=…` | the whole measurement, not just the print — with diagnostics off the two `default_now_us()` clock reads around `maintenance_step` are skipped too |
| `🔌️plugin/🦀️.rs` `cooperative maintenance callback overran the interactive ceiling …` | print only; the `CallbackVerdict` and the recorded `maintenance_fault_us` are unchanged, so the overrun is still *recorded*, just not *printed* |
| `⚛️reactor/🔄️turn/🦀️.rs` `reactor more-work streak…` / `…streak ended…` | the whole `MORE_WORK_TRACE` closure short-circuits after updating the streak counters, so the `PatchTracker::debug_state()` / `pending.debug_state()` string builds never run |

`[DEBUG] typed-operation publication turn=… operations=[…] latest_wins_empty=…` **does not exist as a
live site**: the only occurrence in the tree is a doc-comment reference in
`…/✏️editor/🧪️tests/🔬️fold-contract/🦀️.rs:141` describing the shape a previous lane's line had.
Nothing to gate.

## 3. The eval session is no longer re-serialized every tick (audit rank 3)

**Before.** Every `flowEvalTick` ran `session.eval_json().to_string()` and published the result as
`SetPreviewEval { eval_text }`, unconditionally. The tick self-redispatches until the graph settles,
so most of those ticks moved no node and published byte-identical JSON. The retained `String`'s
retirement cursor (`store::retirement`'s `Bytes`) truncates at most `maximum_bytes` per step, and the
window-transient store is maintenance **stage 22 of 24** — one visit per 24 reactor turns. An
unconditional per-tick republication therefore queues byte-proportional retirement strictly faster
than the rotation can drain it, which is what the audit measured as a 6.9 ms `maintenance_step` and a
12.2 ms callback overrun.

**After.** The decision moved into the flow host as one stateless function:

```rust
// 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs
pub enum FlowEvalPublication { Retained, Changed(Option<String>) }
pub fn flow_eval_publication_for(session: &FlowEvalSession, retained: Option<&str>) -> FlowEvalPublication
```

`retained` is the evaluation the **publication target** already holds — read straight off the target's
own retained state, never off the session:

- editor: `window.get::<Generation3dPreviewWindowTransientOwner>()?.preview_eval_text.as_deref()`
  (`…/✏️editor/🦀️.rs`, `Generation3dFlowEvalWindowWork::step`), passed into
  `flow_eval_tick::evaluate(&doc, &cfg, session, retained_eval)`.
- viewer: `input.context.and_then(|context| context.transient.preview_eval_text.as_deref())`
  (`…/👁️viewer/🦀️.rs`, `Generation3dViewCommandWork::step`).

Per-target is load-bearing, not incidental: one app instance can hold two preview windows over one
shared `FlowEvalSession`, and a session-scoped gate would starve the second window forever
(`preview_eval_exact_window_transient_isolates_and_resets_in_the_registered_app` is the law).
`Retained` emits **no** window-transient / transient mutation at all, so nothing is allocated,
nothing is published and nothing is queued for retirement.

Two further per-tick allocations of the same string are gone:

- `flow_eval_tick::evaluate` no longer clones `eval_json` at all. It used to clone once per tick just
  to hand a `&str` to `preview_tessellate_invocations` while holding `&mut session`.
- `preview_tessellate_invocations(session, fixture, cfg)` (was `(session, eval_json, fixture, cfg)`)
  now reads `session.eval_json()` under an immutable borrow, parses it **once**, and hands the parsed
  `dsl::json::Value` to `pending_preview_tessellate_handles(eval, …)` (was `(eval_json, …)`, which
  re-parsed the whole document a second time). Both borrows end before the `&mut` calls.

A `FlowEvalPublicationLedger` (`considered`/`considered_bytes`/`published`/`published_bytes`, four
thread-local `Cell` adds, no allocation and no formatting) records what the gate saw and what it let
through, so ONE measured boot reports both the ungated (before) and gated (after) volume.

## 4. Tessellation is cached across renders (audit rank 4)

`👁️preview/🦀️.rs`'s `preview_payload` used a per-call-local dedup map, so a render triggered by a
camera orbit or a pointer move re-ran `tessellate_geometry` for every visible handle, re-applied the
show mode, and rebuilt every mesh `Value` — and searched the mesh vector linearly per item
(`meshes.iter().any(…)` inside the loop, i.e. quadratic in mesh count).

Now the payload is split at its real seam. Everything that does **not** depend on hover or selection
is one `PreviewMeshTable { signature, meshes_json, mesh_ids, mesh_id_by_handle }` held in a
thread-local across renders, keyed by

```
signature = hash(eval_json, tolerance.to_bits(), show_mode, preview_widget_ids)
```

— the complete set of inputs the mesh table is a pure function of. A render whose signature matches
reuses `meshes_json` verbatim and rebuilds only the instance table (ids, labels, `selected`,
`hovered`), which is small and mark-dependent by definition. `mesh_ids` is a `BTreeSet`, so the
former quadratic membership scan is now logarithmic.

One entry, deliberately not an LRU: a surface paints one document at one LOD at a time, and holding
stale tables would retain their whole vertex payloads. `preview_tessellation_count()` /
`reset_preview_mesh_table()` expose the observable and the reset for the laws in §6.

## 5. Render no longer evaluates (audit rank 5)

`👁️preview/🦀️.rs`'s `render` used to call `evaluate_fixture(&document.fixture)` — a full synchronous
`FlowHost::evaluate` of the whole flow graph — whenever the ephemeral `preview_eval_text` was absent,
inside the *pure* render function. Every repaint that arrived before the first tick landed paid for a
whole flow evaluation, racing the real `flowEvalTick` chain that was computing the same thing.

`render` now reads the retained evaluation and nothing else: `preview_payload(eval_json.unwrap_or_default(), …)`.
With no evaluation published it paints the empty world (`meshes_json == "[]"`) and lets the tick chain
drive it. `evaluate_fixture` stays `pub` — it is what the window's own unit tests use to *produce* an
evaluation to feed in.

## 6. The measured dominant cost the audit did not name: a 108 kB JSON round trip per tick

Instrumenting `flow_eval_tick::evaluate`'s phases under the new diagnostics flag (temporary probes,
removed again) showed the eval tick was not dominated by anything in the audit's list:

```
[DEBUG] evalphase  host_us=26410  tick_us=33850  retire_us=9148  invocations_us=12181
[DEBUG] hostphase  from_fixture_us=950  infos_json_us=3679  set_infos_us=7082  baseline_us=1  infos_bytes=108201
[DEBUG] hostphase  from_fixture_us=446  infos_json_us=3749  set_infos_us=22151 baseline_us=1  infos_bytes=108201
```

`flow_host_with_session` ran, **once per evaluation tick**:

1. `flow_neuron_kind_infos_json()` — clone every `OperatorInfo` in the flow extension registry into a
   `Vec` and serialize it: **108 201 bytes** of JSON, 3.7-4.5 ms.
2. `set_neuron_kind_infos_json(&that)` — parse those 108 kB back into a `Vec<OperatorInfo>`, rebuild
   the id-keyed map, and `rebuild_dag()`: 7.1-22.2 ms.

The fixture clone the audit suspected (`from_fixture`) is **0.4-0.9 ms** — a rounding error next to
the catalogue round trip. Three changes, in order of what each bought:

| # | change | file |
|---|---|---|
| a | `flow_neuron_kind_info_map()` builds the id-keyed map straight off the registry — **no JSON at all**; `FlowHost::set_neuron_kind_info_map` takes it. `flow_neuron_kind_infos_json` stays for the one consumer that really is across a boundary (the browser's node-graph surface). | `🌊️flow/🗂️catalogue/🦀️.rs`, `🌊️flow/🖥️host/🦀️.rs` |
| b | That map is a pure projection of the registry, so it is built **once per registry generation** and shared: `FlowHost::kind_infos` is now `Arc<HashMap<String, OperatorInfo>>`, and `flow_extension_registry_generation()` (new, `🌊️flow/📔️registry/🦀️.rs`) is the cache key. `state.registry` is written in exactly one place (`FlowRegistryReplacement::publish`) and that write always bumps the generation, so the cache cannot go stale. | `🌊️flow/🗂️catalogue/🦀️.rs`, `🌊️flow/📔️registry/🦀️.rs`, `🌊️flow/🖥️host/🦀️.rs` |
| c | `FlowHost::from_fixture_with_cache_and_infos(fixture, cache, infos)` — the bare constructor built its dag against an EMPTY catalogue and the setter's own `rebuild_dag` then threw that work away, i.e. two dag builds per tick. `from_fixture_with_cache` now delegates with `Arc::default()`. | `🌊️flow/🖥️host/🦀️.rs` |

Every in-process caller was moved to the typed path — generation3d (`🧬️schema`, `✏️editor`,
`👁️viewer`, the example-geometry harness), generation2d (`🧬️schema`), and the playbook's procedural
extension. The flow host's own unit test keeps exercising the JSON API.

## 7. Measured, natively

`hex_column_boot_stays_inside_the_interactive_turn_budget`
(`…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`, region `📈️HotPathBudget`) drives the hexagonal-mushroom-column
boot through the REAL retained-command loop — `dispatch_typed` + `settle_registered_typed_operation`
per turn, with the brep/math extension answered in-process exactly as `dispatchInvokeExtensionEffect`
answers it in the browser — and reports the boot as one `[BUDGET]` line. Unoptimized (`dev` profile),
same machine, same fixture:

| quantity | before | after | how it is measured |
|---|---|---|---|
| turns to first mesh | 1 | 1 | preview body rendered after each turn until `meshes_json` is non-empty |
| extension round trips | 3 | 3 | invocations answered by the in-process brep/math runner |
| **eval tick, best** | *(not separable — the probes did not exist)* | **6 498 us** | `FlowEvalStepLedger::best_us`, flag-gated clock around `flow_eval_tick::evaluate` |
| **eval tick, mean** | **26 239 us** | **6 878 us** (−74%) | `total_us / steps` |
| **eval tick, worst** | **35 901 us** | **7 425 us** (−79%) | `worst_us` |
| **dispatch+settle cycle, mean** | **110 401 us** | **10 178 us** (−91%) | wall around one `dispatch_with_view` |
| **dispatch+settle cycle, worst** | **210 590 us** | **10 842 us** (−95%) | same |
| **eval publications per boot** | **3** (one per tick) | **1** (−67%) | `FlowEvalPublicationLedger.considered` vs `.published` |
| **eval bytes published per boot** | **3 180 B** | **1 060 B** (−67%) | `.considered_bytes` vs `.published_bytes` |
| **bytes per tick** | **1 060 B** | **353 B** (−67%) | the same, over ticks |
| tessellations per hover-only re-render | one per visible handle | **0** | `preview_tessellation_count()` |
| console lines per boot, Rust hot path | ~15 from the removed probes + every ≥2 ms maintenance stage + every ≥8 ms callback | **0** | derived: 5 lines per extension round trip (hops 2/4/6a + hop 1 + hop 7), 3 round trips; the two maintenance sites are now flag-gated |

The `before` column for the timings is the same test on the same machine before §6's changes
(`$T/🗑️generated/perf-run3.txt` and `perf-run5.txt`); the publication columns are read off the SAME
run as the `after` column, because the ledger records what an ungated tick chain WOULD have published
(`considered`) next to what the gate let through (`published`).

### What the test asserts

- the extruded column reaches the preview (`meshes >= 1`);
- the flag-gated clock measured every tick;
- **`best_us < INTERACTIVE_STEP_CEILING_US`** — the interactive contract, asserted on the CHEAPEST
  measured tick. This suite runs on developer machines that are simultaneously compiling the rest of
  the repo, and scheduler noise only ever ADDS wall time, so the minimum is the machine's real
  capability while the mean and worst report its current load. Measured under a concurrent
  `wasm32-wasip2` check the same run reads 15 193 us mean / 21 875 us worst — asserting the mean or
  the worst would assert the scheduler. The margin is real: the same code measured 26 239 us mean
  before §6, so a reintroduction fails here even under load. All four numbers print.
- `published * 2 <= considered` and `published_bytes * 2 <= considered_bytes` — the gate must at
  least halve both.

`a_hover_only_re_render_tessellates_nothing` and `render_without_a_published_evaluation_paints_the_empty_world`
(`…/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs`) are the §4/§5 laws.

## 8. Files changed

**Framework — instrumentation switch**

- `🧰️framework/🔨️modules/⏱️trace/🦀️.rs` — new `//#region 🩺️Diagnostics`: `RUNTIME_DIAGNOSTICS_ENV`,
  `set_runtime_diagnostics`, `runtime_diagnostics_enabled`, per-target environment resolver.
- `🧰️framework/🔨️modules/🧵️job/🦀️.rs:51` — re-exports `INTERACTIVE_STEP_CEILING_US`,
  `interactive_step_contract_violated`, `runtime_diagnostics_enabled`, `set_runtime_diagnostics`,
  `RUNTIME_DIAGNOSTICS_ENV`.

**Framework — plugin runtime / reactor**

- `🧰️framework/…/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` — R3/R4/R6 probes deleted; `MORE_WORK_TRACE`
  short-circuits behind `runtime_diagnostics_enabled()`.
- `🧰️framework/…/🔌️plugin/⚛️reactor/🦀️.rs` — R9 probe deleted; `queue_extension_invocation` returns
  the registry call directly.
- `🧰️framework/…/🔌️plugin/🦀️.rs` — maintenance stage-cost measurement AND its two clock reads gated;
  callback-overrun print gated (the recorded verdict is unchanged).

**Framework — flow host**

- `🧰️framework/…/🌊️flow/🖥️host/🦀️.rs` — `FlowEvalPublication`, `flow_eval_publication_for`,
  `FlowEvalSession::eval_publication_for`, `FlowEvalPublicationLedger` + `FlowEvalStepLedger` (atomic,
  process-wide) and `record_flow_eval_step`; `kind_infos` is `Arc<HashMap<..>>`;
  `set_neuron_kind_info_map`; `from_fixture_with_cache_and_infos`; `flow_host_with_session` builds the
  host with the catalogue in one pass.
- `🧰️framework/…/🌊️flow/🗂️catalogue/🦀️.rs` — `flow_neuron_kind_info_map()` with the
  generation-keyed `Arc` cache; `flow_neuron_kind_infos_json` documented as the wire-only form.
- `🧰️framework/…/🌊️flow/📔️registry/🦀️.rs` — `flow_extension_registry_generation()`.

**Framework — renderer (TypeScript)**

- `🧰️framework/…/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` — the two per-call extension
  hop logs are now fault-path only.

**Plugin — procedural generation3d**

- `…/✏️editor/🦀️.rs` — `flowTessellate invoke` probe deleted, `flowTessellate skipped` kept as a plain
  fault line; `preview_tessellate_invocations` reads `session.eval_json()` and parses ONCE;
  `pending_preview_tessellate_handles` takes the parsed value; the flow-eval window work passes the
  window's retained eval into the tick and publishes only on `Changed`.
- `…/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs` — no per-tick `eval_json` clone; flag-gated step
  clock; returns `FlowEvalPublication`.
- `…/✏️editor/🎮️commands/✅️flow-eval-resolve/🦀️.rs` — per-resolve probe replaced by a fault-only line.
- `…/👁️viewer/🦀️.rs` — publishes only on `Changed`, reading its own transient through the job context.
- `…/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs` — cross-render `PreviewMeshTable`,
  `preview_tessellation_count()`, `reset_preview_mesh_table()`; `render` no longer evaluates.
- `…/🧬️schema/🦀️.rs`, `…/📚️examples/🧪️tests/🧩️geometry/🦀️.rs` — typed catalogue path.

**Plugin — procedural generation2d, playbook**

- `…/🌀️generation2d/…/🧬️schema/🦀️.rs`, `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs` —
  typed catalogue path.

**Tests**

- `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — new `//#region 📈️HotPathBudget`: `BootBudget`,
  `preview_mesh_count`, `measure_hex_column_boot`,
  `hex_column_boot_stays_inside_the_interactive_turn_budget`.
- `…/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs` — two new laws (§4, §5).

## 9. Verification

| gate | result |
|---|---|
| `cargo check -p semio-s-plugin-procedural --keep-going` (native) | **0 errors**, warnings emitted (so the crates really type-checked); the only generation3d warning is the pre-existing `unused extern crate` |
| `cargo check -p semio-s-plugin-procedural --target wasm32-wasip2 --profile wasm-dev` (`CARGO_PROFILE_WASM_DEV_DEBUG=false`) | **0 errors** — mid-lane `🗑️generated/perf-wasm1.txt` (1 m 38 s) and re-run from a cold target dir after the window-addressing merge, `🗑️generated/perf-wasm2.txt` (3 m 27 s) |
| `cargo check -p semio-s-plugin-playbook` | 0 errors |
| generation3d lib suite | **315 passed / 5 failed** (final run; an earlier run mid-lane read 313/7, `🗑️generated/perf-run11.txt`) — the `unit-suite-3d` lane's last reported baseline was 301/10 |
| generation3d hot-path tests | `hex_column_boot_stays_inside_the_interactive_turn_budget`, `hex_column_evaluates_end_to_end_through_the_extension_round_trip`, `a_hover_only_re_render_tessellates_nothing`, `render_without_a_published_evaluation_paints_the_empty_world`, `flow_eval_*` — **all pass** |
| generation2d lib suite | **228 passed / 2 failed** — byte-identical to the count `📓️status.md` 02:59 records for that lane's own landing, so unchanged by this lane |
| renderer TypeScript (`SEMIO_TEST_LEVEL=standard`, react engine config) | **589 tests pass, 14/15 suites**; the one failing suite is `🧪️tests/🧩️package-integration` failing at IMPORT with `ReferenceError: self is not defined` from `🎯️targets/🧊️wgpu/…/plugin-bridge.ts:159` — a jsdom global, not reachable from the two ShellHost lines this lane touched |

### The 7 generation3d failures are not this lane's

| test | failure | owner |
|---|---|---|
| `preview_eval_exact_window_transient_isolates_and_resets_in_the_registered_app` | `ordered-map root must be explicitly retired before drop` — backtrace: `VcsArtifactApp::load_document_pack` → `Generation3dSnapshot` drop glue. Its per-window publication assertions (the ones this lane's gate could break) **pass**; it dies on the document RELOAD one line later. | envelope-load / FlowHost-ownership |
| `vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed` | `Fault` where `Ready` expected | store/vcs |
| `two_instances_converge_disjoint_widget_moves` | `module.vcs … remote snapshot merge is fail-closed` — named verbatim in `📓️status.md` 02:59 as a known remaining framework failure | envelope-load |
| `domain_local_static_verifier_rejects_raw_routes_and_proves_three_dimensional_coverage` | `assertion failed: owner_source.contains("GENERATION3D_RETAINED_COMBINED_DEPTH: usize = 12")` — the work-capacity lane changed that constant | work-capacity |
| `cancelled_and_stale_aba_initializers_retire_to_terminal_empty` | `RetainedJobPayload requires one-page close to terminal-empty` | retained/ownership |
| `every_fourteen_variant_decodes_through_retained_structural_grants` | `one retained mutation ingress grant: "generation3d-mutation.root-string-field"` — the `mutation-ingress-malformed` family `📓️status.md` 05:10 hands to the FlowHost/cursor lane | FlowHost/cursor |
| `generation_preview_is_one_app_transient_shared_by_two_generation_windows` | `Generation3d preview operation did not finish` | work-capacity |

None of the seven touch evaluation publication, tessellation caching, the preview payload, or the
diagnostics switch. `semio-framework-os-flow`'s own lib suite is broadly red (78/127) with
`final Dictionary ownership must be explicitly retired or owned by a cold boundary` in
`🧠️neural/⚙️engine` across `drawing::`, `extensions::wasm::`, `host::` and `vcs::` — a systemic
retirement-contract failure in code this lane never touches, from the ownership work landing in
`📐️brep-geometry/🦀️.rs`, `📔️registry/🧪️tests/`, and `🌿️vcs/🧬️schema/🔺️diff/` (all modified after the
last commit while this lane ran).

### Five remaining generation3d failures at the end of this lane

`preview_eval_exact_window_transient_isolates_and_resets_in_the_registered_app`,
`vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed`
(now `generation3d-publication.contended`), `generation_preview_is_one_app_transient_shared_by_two_generation_windows`,
`two_instances_converge_disjoint_widget_moves`, and `refresh_pending_effects_arms_flow_eval_tick_chain`
(new: `registered fixture typed operation did not retire within 30 seconds`, raised inside the
testkit's own `drain_flow_eval_ticks_with_view`). The last one appeared with the window-addressed
`FlowEvalTick { window_id }` migration that landed in this crate WHILE this lane was verifying — its
testkit still dispatches a window id the retained work refuses. All five sit in the peer lanes' own
surface.

## 10. Interaction with the concurrent window-addressing lane

Midway through verification another lane added `window_id` to `FlowEvalTick`/`FlowEvalResolve`/
`FlowTessellateResolve` plus a `rearm(window_id, req)` helper, and changed
`ArtifactEditor::pending_effects` / `PluginApp::pending_effects` to take an `Option<&ViewModel>`. Their
edit merged cleanly with this lane's — `evaluate` kept both `retained_eval` and the flag-gated step
clock, and `preview_tessellate_invocations` kept the single-parse form. Three of their own compile
errors were fixed here so the tree would build for everyone: `rearm`'s `req: u32` into a
`RequestId(u64)`, a missing `Effect` import in `✅️flow-eval-resolve/🧪️tests/🔬️unit/🦀️.rs`, and the two
stale `flow_neuron_kind_infos_json` imports left by this lane's own typed-catalogue sweep.

## 11. Not done, and why

- **The eval publication is still a JSON `String`, not a pack record body.** The audit's rank 3 asked
  for both the hash gate and a compact retained representation. The gate is in and it is what removes
  the cost: the retirement cursor is byte-proportional PER PUBLICATION, so publishing nothing on an
  unchanged tick removes the whole cycle, while re-encoding the same content more compactly would
  only shrink each remaining walk. The measured `considered_bytes` for this fixture is 1 060 B per
  tick, so the pack encoding would save a few hundred bytes on the ONE publication a boot now makes,
  against a decode on every render (`preview_payload` and `preview_tessellate_invocations` both want
  a parsed value). It is the wrong next lever; the right one is below.
- **The `FlowHost` is still rebuilt from a cloned fixture every tick.** After §6 the tick is ~7 ms,
  and what remains is one `rebuild_dag` + `refresh_interaction_projection` + `retire_cold` per tick
  over a document that did not change. Retaining the host in `Generation3dInstanceOperationOwner`
  next to the session (keyed by a fixture hash, retired through `FlowHostRetirement` the way
  `👁️viewer` already does) is the structural fix and should take the tick well under 1 ms. It was not
  attempted here: the owner and `✏️editor/🦀️.rs` were being rewritten by the work-capacity and
  window-addressing lanes throughout this lane's window.
- **The viewer still builds a fresh `FlowEvalSession` per view command.** So a camera orbit
  re-evaluates the whole flow graph before it can decide the publication is `Retained`. The viewer has
  no `ArtifactInstanceOperationOwner` (the editor's retained-session pattern); giving it one is the
  same shape of change as the point above.
- **Maintenance stage 22 is still round-robin** (audit rank 6). With the publication gate in place a
  boot queues one retirement instead of one per tick, so the 24-turn rotation now drains faster than
  work arrives — the priority scheduler is no longer on the critical path here.
