# W7 Report — os-run Fix + Cross-Plugin IoRouter Test (Retroactive)

**Written by the W7 closer, not the original implementer.** Per the master plan's own Wave DAG,
W7 ("os-run fix if deferred; native routed cross-plugin compose test") is a distinct wave from
W6/V7. `w7-verify-report.md` confirmed real, substantial, uncommitted code matching this
description already sitting in the working tree with no `w7-report.md` ever filed for it — this
file closes that process gap by documenting, from the code itself, what the (unreported) W7
implementer actually did. No claims below are new; all are carried over verbatim from
`w7-verify-report.md`'s independently-verified findings.

## 1. os-run fix — `RunArtifact: store::ArtifactDsl + store::ArtifactPack`

W1 left `semio-framework-os-run` failing to compile: `RunArtifact` did not satisfy the
`ArtifactPack` bound (4 real errors, per `w1-verify-report.md`). W7 fixed this at
`🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs:1922–1978` with a full, real
handcrafted impl mirroring `WorkflowSnapshot`'s own established pattern in the same file:
- `parse_dsl`/`print_dsl` route through `dsl::parse`/`dsl::print` + `SemioEnvelope` wrap/unwrap.
- `encode_pack_with`/`decode_pack_with` route through `store::pack_rt::encode_document`/
  `decode_document` + envelope wrap/unwrap, with an explicit envelope-id mismatch check.

No `todo!()`/`unimplemented!()`/stub bodies. `cargo check -p semio-framework-os-run` now passes
with 0 errors (confirmed both by the verifier and independently re-confirmed by this closer).

## 2. Cross-plugin `IoRouter` compose test

A new test, `io_router_routes_a_real_cross_plugin_compose_between_two_loaded_wasm_plugins`, was
added at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` (region
`🔖️IoRouterE2e`, +93 lines). It:
- loads two separate `WasmPluginRuntime` instances from two different `.wasm` paths (stdio, cad)
  via real component loading (not mocked structs);
- registers both into one shared `IoRouter::new()`;
- asserts `router.stats()` reports 2 plugins and `keys > 0`;
- builds a real `IoKey` (`s.cad`, Export→`s.stdio.step`) that is only ever registered inside cad's
  own composer, so a successful route with `calling_plugin_id = "stdio"` can only happen by
  hopping into cad's separately-loaded instance;
- reads a real on-disk DSL fixture, calls `router.compose(...)`, decodes the resulting
  `WireComposedArtifact`, and asserts non-empty output containing `"cad.document"`;
- does a second round through the same routed key and asserts byte-identical output (a
  `print(parse(x))` fixpoint check).

The test follows the same silent-skip convention as the existing
`wasm_plugin_runtime_loads_real_plugin_component_if_present` test in the same file
(`if !stdio_path.exists() || !cad_path.exists() { return; }`) so the crate's own suite stays green
on a fresh clone / no-wasm-toolchain CI run.

Supporting changes: `🏃️run/🦀️component.rs` gained an `io_router_stats()` accessor (+13 lines,
real — reads `self.io_router.stats()` off the `IoRouter` actually populated by `runtime_for`'s
`register_plugin` calls, not a hardcoded tuple); `🏃️run/📦️bin.rs` gained a dev-boot smoke line
printing those stats (`[os run] io-router: {N} plugins / {N} keys`); `🔌️plugin/🦀️component.rs`
gained +38 lines of `IoRouter`/plugin-registration plumbing to support the above.

## 3. Known gap at W7 hand-off — cad wasm cannot currently be built

`stdio`'s wasm component builds cleanly. `cad`'s does not: `E0599: no method named
'projection_json' found for struct 'Ref<'_, ArtifactStore<CadSnapshot, CadMutation>>'` at
`✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🌉️wasm/🦀️component.rs:44`. `projection_json` has no definition
anywhere on `ArtifactStore` — every repo hit is a caller, across ~10+ plugins' wasm-binding files
(cad, jack, raster, process, writer, gis, shooting, puzzle, animate/present,
trinity/rewrite). This is a widespread, pre-existing gap, not introduced by or specific to W7 —
but it means the new cross-plugin test's silent-skip guard fires in this environment (cad.wasm
absent), so its real routing assertions have not yet been observed to execute here. See
`w7-close-report.md` for this closer's disposition of the gap (documented as a follow-up, not
fixed — it is a design-scope fix, not a cheap/safe one).

## Files touched (all uncommitted at W7 hand-off)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs` — `RunArtifact` codec impls.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs` — `io_router_stats()` accessor.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust/📦️glue.rs` — no functional change
  at W7 hand-off (later cleaned up by the closer, see `w7-close-report.md`).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️bin.rs` — dev-boot smoke line.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` — new cross-plugin test.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — `IoRouter` plumbing.
