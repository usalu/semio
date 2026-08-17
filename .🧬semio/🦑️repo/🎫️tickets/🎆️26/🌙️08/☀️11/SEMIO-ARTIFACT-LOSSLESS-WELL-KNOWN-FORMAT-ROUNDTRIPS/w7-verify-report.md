# W7 Independent Verify Report — os-run fix + cross-plugin IoRouter test

**Verdict: FAIL**

## 0. The premise of this verification task is false in one respect

`w7-report.md` does **not exist**. Confirmed by direct read (error: file does not exist) and by
`find`/`ls` across the entire ticket folder (no `w7-*` file of any kind exists in
`SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT`). `STATUS.md`'s newest entry is
the **W6/V7 closer** ("V7 (MediaFormat/ArtifactCodec deletion) is DONE") — it never mentions a W7
wave running. Per the ticket's own `📋️master-plan.md` (§Wave DAG, line 135), **W7 is a distinct,
separate wave** from W6/V7 ("W7 (1 agent, serial): os-run fix if deferred; ... native routed
cross-plugin compose test ... Gate: os-run checks+tests green, wasm builds succeed, smoke boots").
W1's own verify report (`w1-verify-report.md`) confirms os-run still had 4 real errors at the end of
W1 and was explicitly deferred to W7.

So: no report was written, but (per §1–§3 below) **real, substantial, uncommitted code matching
W7's description does exist in the working tree** — this looks like in-progress/interrupted work
that was never wrapped up with a report, not a fabricated report I failed to find. I verified the
actual code and behavior directly rather than trusting any narrative.

## 1. `cargo check -p semio-framework-os-run` — PASS (0 errors)

```
cargo check -p semio-framework-os-run 2>&1 | tail -30
```
```
warning: `semio-framework-os` (lib) generated 10 warnings (run `cargo fix --lib -p semio-framework-os` to apply 8 suggestions)
    Checking semio-framework-os-run v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust)
warning: unused extern crate
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust/📦️glue.rs:7:1
  |
7 | extern crate semio_framework_os_kernel as dsl_core;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unused
  ...
warning: function `run_fault_bytes` is never used
   --> 🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust/../../🦀️component.rs:304:4
    |
304 | fn run_fault_bytes(code: impl Into<String>, message: impl Into<String>) -> Vec<u8> {
    |    ^^^^^^^^^^^^^^^
warning: `semio-framework-os-run` (lib) generated 2 warnings (run `cargo fix --lib -p semio-framework-os-run` to apply 1 suggestion)
    Finished `dev` profile [unoptimized] target(s) in 4.61s
```
Re-confirmed with `--lib`, `--bins`, and `--all-targets` explicitly (this crate has no separate
`[[bin]]` — wait, it does: `Cargo.toml` declares `[[bin]] name = "semio-framework-os-run"`;
`--all-targets` compiled lib+bin+test cleanly, 0 errors). **Genuinely zero errors, only warnings.**

## 2. `cargo test -p semio-framework-os-run --lib` — PASS but irrelevant to the claimed new test

```
running 15 tests
test run_lib::tests::convert_media_is_identity_when_contract_has_no_conversion ... ok
test run_lib::tests::media_converter_registry_applies_registered_converter ... ok
test run_lib::tests::validate_rejects_missing_required_input ... ok
test run_lib::tests::validate_rejects_multiplicity_one_input_with_two_incoming_edges ... ok
test run_lib::tests::rejects_incompatible_edge_media_types ... ok
test run_lib::tests::detects_cycles ... ok
test run_lib::tests::vector_to_raster_rejects_non_structured_payload ... ok
test run_lib::tests::topological_order_respects_edges ... ok
test run_lib::tests::first_run_recomputes_every_node_second_run_is_a_no_operation ... ok
test run_lib::tests::editing_upstream_document_dirties_downstream_only_through_the_wire ... ok
test run_lib::tests::parameter_overlay_alone_dirties_its_bound_node_without_changing_raw_config_bytes ... ok
test run_lib::tests::changing_a_nodes_config_alone_dirties_it_without_touching_document_or_inputs ... ok
test run_lib::tests::register_builtin_converters_wires_vector_to_raster_through_convert_media ... ok
test run_lib::tests::vector_to_raster_rasterizes_svg_to_a_2d_image_media ... ok
test run_lib::tests::validate_rejects_unregistered_conversion ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
15/15 pass, all pre-existing `run_lib` graph/media-conversion tests. **None of these is a
cross-plugin IoRouter test.**

## 3. `cargo test -p semio-framework --lib` — PASS but also irrelevant

120 passed, 0 failed (mesh/manifest/platform/workflow test modules). **No IoRouter test here
either.**

**Important process finding: the two commands the task specified to verify the "native cross-plugin
IoRouter integration test" do not and cannot cover it.** The actual test lives in a **third crate**,
`semio-framework-plugin-host` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`,
function `io_router_routes_a_real_cross_plugin_compose_between_two_loaded_wasm_plugins`), which is
neither `semio-framework-os-run` nor `semio-framework`. Anyone who ran only the two specified
commands — as the report presumably did, or as I was initially instructed to — would never execute
this test at all.

## 4. Is the RunArtifact `ArtifactDsl`/`ArtifactPack` impl real? — YES, and it is uncommitted

Read directly at `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs:1922–1978`. It is a
full, real handcrafted impl mirroring `WorkflowSnapshot`'s own pattern (same file, comment says so
explicitly): `parse_dsl`/`print_dsl` route through `dsl::parse`/`dsl::print` + `SemioEnvelope`
wrap/unwrap; `encode_pack_with`/`decode_pack_with` route through `store::pack_rt::encode_document` /
`decode_document` + envelope wrap/unwrap with an explicit envelope-id mismatch check. **No
`todo!()`/`unimplemented!()`/stub bodies — genuinely real code**, not a placeholder.

Cross-checked history: `git log -S "impl store::ArtifactDsl for RunArtifact"` returns **zero
commits** anywhere in the file's history, and `git show HEAD:<file> | grep "impl store::ArtifactDsl
for RunArtifact"` returns **nothing** — the committed HEAD version of this file does not have these
impls at all. `git status` confirms the file is `M` (modified, uncommitted). This matches W1's
verify report, which found exactly this (`RunArtifact: ArtifactPack` bound not satisfied, 4 errors)
as of W1's close. **The os-run fix is real, substantial, and currently sitting uncommitted in the
working tree** — consistent with "an implementer just fixed" it, even though no report documents it.

Other uncommitted, os-run-relevant files: `🏃️run/📦️bin.rs` (+8/-… incl. the `io_router_stats` smoke
line, §6), `🏃️run/🦀️component.rs` (+13, incl. `io_router_stats()` accessor), `🔌️plugin/🦀️component.rs`
(+38/-…).

## 5. The cross-plugin compose test itself — real code, but its real assertions are UNPROVEN in this environment

Read the full diff at `🔌️plugin/🖥️host/🦀️component.rs` (+93 lines,
`io_router_routes_a_real_cross_plugin_compose_between_two_loaded_wasm_plugins`, region
`🔖️IoRouterE2e`). It genuinely:
- loads two **separate** `WasmPluginRuntime` instances from two **different** `.wasm` paths
  (stdio, cad), each via `WasmPluginRuntime::load(path)` — real component loading, not mocked structs.
- registers both into one shared `IoRouter::new()` via `register_host_io_router` +
  `router.register_plugin`.
- asserts `router.stats()` reports 2 plugins and `keys > 0`.
- builds a real `IoKey` naming `s.cad` (Export→`s.stdio.step`) — a key the comment demonstrates,
  by inspection, is **only ever registered inside cad's own composer**, so routing it with
  `calling_plugin_id = "stdio"` can only succeed by hopping into cad's separately-loaded instance.
- reads a real on-disk fixture (`✏️s/🔌️plugins/📐️cad/…/example.dsl.semio`), calls
  `router.compose(...)`, decodes `WireComposedArtifact`, asserts the composed text is non-empty and
  contains `"cad.document"`.
- does a **second** round through the same routed key on the first result and asserts byte-identical
  output — a real `print(parse(x))` fixpoint check, non-trivial evidence real parse/print logic ran
  on both hops.

**However**, the test has a guard at the top:
```rust
if !stdio_path.exists() || !cad_path.exists() { return; }
```
If either `.wasm` file is missing, the test **silently passes having asserted nothing**. This is
exactly what happened in my own targeted run (§7): `cad`'s wasm component **cannot currently be
built** (see §6), so my "passing" run of this test only proved the guard fired, not that the routed
compose logic actually works. **The report's implicit claim that this test's real assertions have
been observed to pass cannot be confirmed** — worse, it is currently unconfirmable in this repo
state, because the precondition (2 real wasm components on disk) is unmet and cannot be
mechanically satisfied right now (§6).

## 6. Re-running the wasm builds — stdio PASS, cad FAIL (real compile error)

`bun nx run @semio-tech/framework-os-dev:build -- stdio` (and `-- cad`) as literally instructed
**failed for both** via nx's dependency graph — `build` depends on `@semio-tech/assets:build`, which
transitively drags in a `storybook build` that fails on a pre-existing, unrelated, **committed** (not
concurrent-churn) broken import: `.storybook/stories/ui/✅ValidationTree.stories.tsx` imports
`@semio-tech/coda-desktop/renderer`, a package that does not exist anywhere in the repo (confirmed:
`find … -iname coda-desktop` → nothing; last touch to that stories file was commit `55fef8334f`,
several commits back, not live churn). This is an nx-wiring/infra problem orthogonal to W7.

Bypassing nx and calling the underlying script directly (`cd .../🧑️‍💻️dev/📦️packages/🟦️typescript &&
bun ./📜️script.ts build stdio` / `... build cad`, same env vars the `project.json` target sets):

- **stdio: PASS.** Built cleanly, `semio_s_plugin_stdio_component.core.wasm` rewritten (38537 bytes,
  fresh mtime).
- **cad: FAIL — real compile error**, not an infra flake:
  ```
  error[E0599]: no method named `projection_json` found for struct
  `Ref<'_, ArtifactStore<CadSnapshot, CadMutation>>` in the current scope
    --> ✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/././../../🎛️apps/📐️cad/🌉️wasm/🦀️component.rs:44:33
     |
  44 |             self.store.borrow().projection_json().map_err(|e| JsValue::from_str(&e.to_string()))
     |                                 ^^^^^^^^^^^^^^^ method not found in `Ref<'_, ArtifactStore<CadSnapshot, CadMutation>>`
  ...
  error: plugin build failed: cad
  ```
  Confirmed `projection_json` has **no real definition anywhere** as a method on `ArtifactStore<P,
  Mutation>` (`grep -rn "fn projection_json"` across the repo hits zero definitions on `ArtifactStore`
  itself — every hit is a *caller*, in ~10+ plugins' wasm-binding files: cad, jack, raster, process,
  writer, gis, shooting, puzzle, animate/present, trinity/rewrite). This is a **widespread,
  pre-existing gap affecting essentially every plugin's wasm bindings**, not something introduced by
  or specific to the W7 work — but it does mean: **"build 2 real wasm components" is currently
  impossible for any second plugin that has this common wasm-bindings shape**, cad included. Whoever
  wrote this test either never actually built cad's wasm locally, or built it before this gap
  appeared/was noticed, or built a different second plugin and the report (never written) would have
  said so. Either way, the checked-in state of the test today cannot be exercised end-to-end.

## 7. Running the specific new test with a targeted filter — passes, but proves nothing here

```
cargo test -p semio-framework-plugin-host --lib io_router_routes_a_real_cross_plugin_compose_between_two_loaded_wasm_plugins -- --nocapture
```
```
running 1 test
test component::tests::io_router_routes_a_real_cross_plugin_compose_between_two_loaded_wasm_plugins ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s
```
Given §6 (cad.wasm absent/unbuildable in this environment), this "ok" is the **guard-triggered
no-op path**, not a real exercise of the routed compose logic. I cannot currently produce a run of
this test where its real assertions execute.

## 8. `io_router_stats` smoke log — real, not hardcoded

`🏃️run/📦️bin.rs:234-238`:
```rust
// 📊️ Dev-boot smoke line (W7): real `io_router_stats()` numbers, not hardcoded — a zero-plugin or
// zero-key router (the shared cross-plugin `IoRouter` silently doing nothing) is visible right
// here, regardless of whether the run itself succeeded or failed partway through.
let (io_router_plugins, io_router_keys) = runner.into_host().io_router_stats();
eprintln!("[os run] io-router: {io_router_plugins} plugins / {io_router_keys} keys");
```
`io_router_stats` (`🏃️run/🦀️component.rs:1201`) is a real accessor: `self.io_router.stats()` on the
`IoRouter` actually populated by `runtime_for`'s real `register_plugin` calls during the run — not a
hardcoded/fake tuple. **This claim checks out.**

## Verdict and reasoning

**FAIL.** Summary:
- No `w7-report.md` exists — the process requirement (write a report, refer to it in chat) was not
  met by whoever did this work, if anyone did file-level closure at all.
- os-run genuinely compiles clean (§1) and its `RunArtifact` codec impls are genuinely real (§4) —
  these two checks pass on their own merits.
- The two test commands named in the task (`-p semio-framework-os-run --lib`, `-p semio-framework
  --lib`) both pass but **neither one runs the claimed new test** — it lives in a third crate,
  `semio-framework-plugin-host`.
- The new cross-plugin IoRouter test (§5) is real, well-constructed code with genuine assertions —
  but it has a silent-skip guard, and in this environment **the second wasm component (cad) cannot
  currently be built** (§6, a real, reproducible `E0599` on a repo-wide missing
  `ArtifactStore::projection_json` method, affecting ~10+ plugins, not W7-specific). This means the
  test's real assertions are **currently unverifiable** — every "pass" anyone gets right now,
  including mine, is the empty guard path, not proof the routed compose logic works.
- Master plan's own W7 gate ("os-run checks+tests green, **wasm builds succeed**, smoke boots") is
  therefore **not met**: one of the two required wasm builds fails.

## Files/paths referenced
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs` (uncommitted, RunArtifact codecs)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs`, `📦️bin.rs` (uncommitted)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` (uncommitted, new test)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (uncommitted)
- `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🌉️wasm/🦀️component.rs:44` (cad wasm build failure site)
- `.storybook/stories/ui/✅ValidationTree.stories.tsx` (unrelated broken nx build dependency)
