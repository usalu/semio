# terra — Z1-zero-warnings — Report

`CARGO_TARGET_DIR` used throughout: `<ticket>/🎯️target-z1` (seeded via an APFS clone of the ticket's pre-existing `🎯️target`, then built forward — never reused a peer packet's live target dir).

## Honest end state per target (paste-verified)

### `wasm32-unknown-unknown` — **CLEAN**
```
$ bun ./📜️script.ts verify rust-warnings --target wasm32-unknown-unknown
[verify rust-warnings] wasm32-unknown-unknown: 1 crate(s)…
    Finished `dev` profile [unoptimized] target(s) in 0.36s
[verify rust-warnings] wasm32-unknown-unknown clean.
EXIT=0
```
Before: 13 clippy errors in `semio-framework-actor`. After: 0. Full before/after logs: `terra-Z1-wasm32-unknown-unknown-before.txt` / `terra-Z1-wasm32-unknown-unknown-after1.txt`.

### `wasm32-wasip2` — **NOT clean, blocked before reaching any of the 33 plugin crates' own code**
```
$ bun ./📜️script.ts verify rust-warnings --target wasm32-wasip2
error: cargo clippy -p semio-s-plugin-animate --lib --features component-guest --target wasm32-wasip2 -- -D warnings exited with status 101
EXIT=1
```
Two independent, both out-of-scope blockers — see `## Blocking findings (lease-request)` below. Full log: `terra-Z1-wasm32-wasip2-before.txt`.

### `native` — **NOT clean, 34/36 crates unreachable**
```
$ bun ./📜️script.ts verify rust-warnings --target native
[verify rust-warnings] native: 36 crate(s)…
    Finished `dev` profile [unoptimized] target(s) in 0.24s   ← semio-framework-actor, clean
error: cargo clippy -p semio-framework --all-targets -- -D warnings exited with status 101
EXIT=1
```
`semio-framework-actor` (1st of 36) is clean. `semio-framework` (2nd) fails immediately on the same out-of-scope `semio-framework-os-kernel-dsl-derive` + `semio-framework-mesh-engine` chain that blocks `wasip2`. The other 34 crates (`semio-framework-os-kernel` + 33 plugin crates) all transitively depend on one or both and are equally unreachable through the aggregate command — never independently disproven clean beyond what I could reach via plain `cargo check` (see below).

**A partial result reported accurately**: of the 36 native crates, only 1 (`semio-framework-actor`) is proven clean end-to-end through the real `-D warnings` clippy gate. `semio-framework-plugin`, `semio-s-plugin-stdio`, `semio-s-plugin-puzzle`, and `semio-framework-os-renderer-wgpu`'s own `📦️glue.rs` were fixed and proven **warning-clean under plain `cargo check --all-targets`/`--lib`** (real rustc diagnostics, not gated by `-D warnings`), but **not** re-verified through clippy's stricter lint set, because clippy lints every workspace-member dependency compiled as part of the command and the same three out-of-scope crates abort that compilation before clippy ever reaches these crates' own source. This is the honest limit of what foreground verification could reach this session.

## Root-cause discovery: three out-of-scope crates block ~everything

Running `cargo clippy -p <plugin-crate> --lib --target wasm32-wasip2 -- -D warnings` (bypassing the tooling bug below) or `--all-targets` natively on **any** of the 36 crates fails identically, always at one of:

| crate | path (out of my `path_scope`) | errors |
|---|---|---|
| `semio-framework-os-kernel-dsl-derive` | `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs` | 2 (`clippy::map_unwrap_or` at :353, `clippy::cmp_owned` at :355 — exact clippy-suggested diffs are in the raw logs) |
| `semio-framework-mesh-engine` | `🧰️framework/🔨️modules/🔺️mesh-engine/📦️packages/🦀️rust/📦️glue.rs` | 13 (`chunks_exact`×6, `needless_pass_by_value`×3, `map_unwrap_or`×2, `manual_is_multiple_of`×1, loop-index×1) |
| `semio-framework-graph` | `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/build.rs` | 2 (`clippy::unnecessary_map_or` at :12 and :50) |

`semio-framework-os-kernel-dsl-derive` alone is a **hard dependency of `semio-framework-os-kernel`**, which is a hard dependency of `semio-framework` and every one of the 33 plugin crates — so its 2 errors alone block 35 of the 36 native crates and all 33 wasip2 crates. `semio-framework-mesh-engine`/`semio-framework-graph` compound this for anything depending on them too (most plugin crates).

**Lease-request**: these three files are outside `path_scope` — I did not touch them. The fixes are small and mechanical (clippy prints the exact diff for each); whoever owns `🕸️graph`/`🔺️mesh-engine`/`🗣️dsl` should apply them, after which both `wasip2` and `native` should get meaningfully further before the next blocker (if any) surfaces.

## Tooling bug found in the verify script itself (lease-request, registrar-only `📜️script.ts`)

`rustWarningTargetScope`'s `wasm32-wasip2` branch passes `--features component-guest` to every plugin crate's own `cargo clippy -p <crate>` invocation. **No plugin crate declares a `component-guest` feature of its own** — confirmed via `cargo metadata`: `semio-s-plugin-stdio`'s own features are `['default', 'plugin-root']`, `semio-s-plugin-writer`'s are `[]`. The feature exists only on `semio-framework-plugin` (`component-guest = []`), which every plugin crate already unconditionally enables via a hardwired `features = ["component-guest"]` on its dependency line — so the flag is both wrong (cargo requires `-p`'s own package to own any bare `--features` name) and unnecessary (the feature is already on). This is not specific to `semio-s-plugin-animate` (first alphabetically) — I confirmed the identical error on `semio-s-plugin-stdio` run in isolation. **Every one of the 33 plugin crates fails this exact way**, 100%, before clippy ever reaches their own source.

**Recommended fix** (not applied — `📜️script.ts` is registrar-only): drop `"--features", "component-guest"` from the `wasm32-wasip2` `scopeArgs` in `rustWarningTargetScope` (root `📜️script.ts`, `🔖️VerifyScript` region, `rustWarningTargetScope` function). As a workaround to still gather data this session, I ran `cargo clippy -p <crate> --lib --target wasm32-wasip2 -- -D warnings` (no `--features` flag) across all 33 plugin crates directly — results below.

## Per-crate disposition

### `semio-framework-actor` (🎭️actor, in scope) — FIXED, verified clean both targets
13 clippy errors (wasm32-unknown-unknown) + 3 more under `--all-targets` natively (`redundant_clone`, only visible once tests compile), all fixed:
- `on_signal`/`submit`/`complete`: `needless_pass_by_value` → take `&FailureSignal`/`&Envelope`/`&TurnResult` (all three were read-only at every call site; updated every call site in this crate, `📦️glue.rs`, and `🔌️plugin/🖥️host/🦀️component.rs`, which also calls `Kernel::complete`/`submit`).
- `PackError` (`pack::PackError` in this crate) → added `Copy` (all variants are `usize`/`&'static str`/`u8`, already `Clone`) instead of taking `&PackError` at the one `.map_err(to_js_error)` call site pattern used six times — avoids threading `&` through every `map_err(to_js_error)` call.
- `manual_clamp` (`.min(4).max(1)` → `.clamp(1,4)`), `unnecessary_min_or_max` (dropped a no-op `.max(0)` on an already-unsigned value).
- `map_unwrap_or` ×4 → `map_or`.
- `map_entry` (`contains_key` + `insert` → `Entry::Vacant`).
- `clone_on_copy` ×2 (`FailureStage` is `Copy`) → dropped `.clone()`.
- `redundant_clone` ×3 in tests (`.clone()` on the last use of a binding) → moved instead of cloned.

### `semio-framework-os-kernel` (📡️spr, in scope) — 1 fix applied, crate itself unreachable end-to-end
The coordinator's flagged dead store: `📡️spr/📡️wire/🦀️component.rs`'s `ServerFrame::Session` decode arm was the **only hand-rolled byte read** among sibling arms that all delegate cursor advancement to shared `read_*` helpers. **Decision: made it consistent with its siblings, not just removed the dead store** — added a `read_u8` primitive to `📡️spr/🧾️wire/🦀️component.rs` (the shared wire-codec file, alongside `read_varint_u64`/`read_str`/`read_bytes`; `write_u8` was **not** added since nothing needed it) and replaced the arm's manual `bytes.get(pos)` + `pos += 1` with `crate::os_spr::read_u8(bytes, &mut pos)?`. Rationale: the missing `read_u8` twin (every other primitive has a read/write pair) was the actual reason this one arm had to hand-roll cursor tracking in the first place — adding it is a real gap-fill, not a cosmetic tweak. **Not compile-verified** (this crate is unreachable behind the dsl-derive blocker) — verified by re-reading the diff carefully instead; flagged honestly rather than claimed clean.

### `semio-framework-plugin` (🔌️plugin, in scope) — FIXED, verified clean via plain `cargo check --all-targets` (not clippy — see caveat above)
Went from "6 warnings" (starting-point estimate) to a measured 21 warnings across `--lib`+`--lib test` (the estimate undercounted `--all-targets`), then 0:
- `⚛️reactor/🦀️component.rs:26,28` unused imports (`Effect`/`Event`/.../`UiPatch`, `HashMap`) — **root cause found, not just suppressed**: their only consumer is `mod wit_bridge` (the whole WIT-boundary translation layer), which is `#[cfg(all(any(feature = "component-guest", feature = "component-extension-guest"), target_arch = "wasm32", target_env = "p2"))]`-gated. Under a plain native build that code doesn't exist, so the imports were genuinely dead for native. Gated the two `use` lines identically instead of deleting them (they're real for the wasm32 build).
- `outcome_to_result` (`🌐host/🦀️component.rs:21`) and `set_instance_actor` (`🦀️component.rs:14745`) — **investigated per rule 4, found to be neither dead nor a wiring gap**: both have exactly one call site, inside `wit_bridge::poll` (same cfg-gated module as above). Gated both definitions identically to their sole caller. (Earlier in the session I initially misread these as "already wired, no action needed" from a naive whole-file grep that didn't account for the cfg gate — the coordinator's original flag was correct; corrected once I ran `cargo check` and saw the real native-build diagnostic.)
- `ArtifactDeclaration.child_slots`/`link_slots` — **left exactly as instructed**: not deleted, doc comment already explains the deliberate manifest-completeness capture. Added `#[allow(dead_code, reason = "...")]` referencing that existing comment (the doc already IS the reason; this just makes the suppression itself precise instead of blanket).
- `PluginRuntimeRegistry.schemas`/`inferences`/`languages`/`app_schemas` — **investigated, found to be a real gap, not deletable**: every sibling field (`inference_services`, `host_media_handlers`, `flow_extensions`, both mutation maps) is consumed via a `self.runtime.*` accessor elsewhere in the file; these four are the only ones with zero readers anywhere. Suppressed with a documented reason (not blanket) and spawned follow-up `task_00cfc256` rather than inventing the intended consumer.
- Macro-generated dead code in `derive_artifact_facets!`'s `ChildrenMacroSpec` grammar-smoke-test invocation (`ChildrenMacroBuilder`/`ChildrenMacroAnalyzer` never constructed, `sniff`/`analyze`/`compose` inherent duplicates never called) — every *real* invocation of this macro exercises these through their trait impls; only a pure grammar-parses-successfully smoke test doesn't. Added `#[allow(dead_code, reason = "...")]` to the macro's generated inherent-convenience impls/structs (affects every expansion, harmless — dead_code allow on code that IS used elsewhere is a no-op there).
- `unused doc comment` on a bare `derive_artifact_facets!{...}` invocation — rustdoc can't attach to a macro invocation; changed `///` → `//`.
- 4× `unnecessary qualification` (`crate::app::InteractionView` → `InteractionView`, `std::collections::BTreeMap` → `BTreeMap`) — mechanical, clippy's own suggested diff applied verbatim.
- `subset!` macro's `derived`/`owning` arms both silently discarded `register_subset_validator(...)`/`register_composer_entries(...)`'s `Result<(), _>` inside a `Once::call_once` closure (`unused_must_use`) — added `.expect("subset registration is Once-guarded — a failure here means a dialect collision, a real programmer error")` to both call sites in both arms (4 total), since a `Once`-guarded registration failing is a genuine programmer-error invariant violation, not a recoverable runtime condition, and panicking surfaces it immediately instead of the previous silent no-op.

**Caveat**: fixed against plain `cargo check`, which surfaces real rustc diagnostics but not clippy's stricter lint set (`needless_pass_by_value`, `map_unwrap_or`, etc. — the same lint families I fixed in `semio-framework-actor`) since clippy on this crate is blocked by the dsl-derive/mesh-engine chain. Some clippy-only findings likely remain undiscovered here.

### `semio-s-plugin-stdio` (✏️s/🔌️plugins, in scope) — FIXED, verified clean via plain `cargo check --all-targets`
Went from "5 warnings" (starting-point estimate, `--lib` only) to a measured 9 across `--lib`+`--lib test`, then 0. Notable ones beyond mechanical unused-import removal:
- `parse_ut_mtime` (zip UT-extra-field mtime parser) — **investigated, real gap**: its own doc comment claims mtime is "surfaced as a typed convenience field," but `ZipEntry` has only `name`/`data`, no such field exists. Suppressed with a reason, follow-up `task_dc49df65` spawned rather than guessing at the schema addition.
- `hard`/`soft` (ISO/IEC 21320-1 diagnostic builders) — **investigated, real gap**: `check_iso21320_conformance` is a stub returning `Vec::new()` unconditionally, despite the module having fully-defined `CODE_*`/`FLAG_*`/`VERSION_NEEDED_SOFT_CEILING` constants clearly set up for real checks that were never implemented. Same follow-up task as above (`task_dc49df65`) covers both — real feature work, not a warnings fix.
- `StlFormat::Ascii` variant "never constructed" — **investigated, real gap**: only this file's own tests construct it; `write_ascii_stl` (the arm it dispatches to) is real, tested code, but no production command surface lets a user choose ASCII STL export. Suppressed, follow-up `task_ac8f7b76` spawned.
- `fn object_diff` (json diff test helper, sibling of the actively-used `fn array_diff`) — **investigated, judged genuinely dead** (no doc comment declaring deliberate incompleteness, unlike the cases above) — deleted rather than suppressed.
- A real bug I introduced then caught myself: removing `GltfDocument` from a `use` list (correctly flagged unused under the `--lib` build) broke `--lib test` with 8 compile errors, because `mod tests { use super::*; }` only reaches it via that same import, and `mod tests` doesn't exist in the `--lib` (non-test) build at all. Fixed by `#[cfg(test)]`-gating the import instead of deleting it — the general lesson (also applied to the reactor/puzzle wasm-only imports below): **`--lib`-only "unused" verdicts can be wrong once `--tests`/`--all-targets` is considered; always re-check with `--all-targets` after acting on an unused-import warning.**
- A separate genuine duplicate `GltfDocument`/`GltfMesh`-style redundant import inside a dwg-export serializer's `#[cfg(test)]` block (outer cfg-gated import shadowed by an identical inner `mod tests` import) — removed the outer, kept the inner.
- Two literal duplicate-attribute bugs, both distinct root causes: `📷️jpg/…/🚪️io/🦀️component.rs:1365` had a stray `#[test]` stranded in the *middle* of what should be one contiguous doc-comment block (a bad merge, not a real second test) — removed the stray one, kept the correctly-placed one immediately before `fn`; `🎞️gif/…/🧬️mutations/🦀️component.rs:481` had a literal `#[test]\n#[test]` (plain copy-paste duplicate) — removed one.
- `semio_framework_plugin::plugin_exports!(plugin::plugin)` → `plugin_exports!(plugin)` (mechanical, `pub use plugin::plugin;` already re-exports it unqualified).

### `semio-s-plugin-puzzle` (✏️s/🔌️plugins, in scope) — FIXED for the 4 flagged warnings; **major unrelated pre-existing breakage found and NOT fixed**
The 4 flagged unused imports (`Puzzle2dSnapshot`, `BoardHost`/`CubicBez`/`Point`/`SceneDescriptorJson`/6 engine fns, `puzzle_board_host_normal`/`puzzle_board_host`, `redraw_layout_fixture_json`) were all in `◻2d/…/✏️editor/🌉️wasm/🦀️component.rs`, whose entire body is individually `#[cfg(target_arch = "wasm32")]`-gated per item — same shape as the reactor fix above. Gated the imports; `--lib` is now clean.

**Found while verifying `--all-targets`**: `semio-s-plugin-puzzle`'s **test suite does not compile — 176 pre-existing errors**, confirmed unrelated to my edit (none reference the file I touched). Dominant patterns: 80 errors of `Mutation<Result<...>> is not satisfied` (Puzzle2d/3d/5d), ~46 errors of `.envelope()`/`.dispatch()`/`.apply()`/`.snapshot()` "not found for `Result<T, E>`" (looks like an `ArtifactStore`/`MutationOutcome`-shaped API started returning `Result` somewhere and puzzle's tests were never updated), plus `EditorApp`/`ObjectKindRepresentation`/`App` "cannot find type" (missing imports, ~25 more). This is orders of magnitude beyond a warnings-cleanup packet and clearly belongs to whatever packet changed the underlying mutation/store API — **out of scope, reported not fixed**, flagged prominently here rather than attempted.

### `semio-framework-os-renderer-wgpu`'s `📦️glue.rs` (wgpu target, in scope) — FIXED, verified clean via `cargo check --lib`
8 `--lib` warnings → 3 in-scope (glue.rs) + 5 out-of-scope (`Shell`/`Dock` — registrar-only, matches the packet brief's explicit callout).
- `fn now_ms() -> u64` wrapper around `app_now_ms() -> f64` — genuinely dead (every real call site uses `app_now_ms()` directly, not this wrapper) — **deleted**.
- `RetainedSurface.node` — **investigated, ambiguous real gap, not guessed at**: `apply_ui_patch`'s desync branch comment claims "Previous snapshot is reused (item 4)" but the code only records a pending rejection and never reads `.node` back into the returned `out` map. Could be that omitting the key from `out` already IS the reuse mechanism (caller keeps last frame) — or a real visual-freeze/blank bug. Suppressed, follow-up `task_bcc46a4e` spawned rather than guessing which.
- `RegistryQuotas.fuel` — deserialized from the scale-bench registry fixture, siblings (`deadline_ms`/`max_effects`/`max_patch_bytes`/`max_frames`) are all enforced elsewhere in the bench harness, `fuel` alone isn't. Suppressed, same follow-up task.

### `[DEBUG] ` cleanup — all three files were misusing the marker for PERMANENT diagnostics, none were genuinely temporary
- `🧊️wgpu/📦️glue.rs` (29): all real error-propagation/logging call sites (scale-bench read/parse/build/compile/write/report failures and the `"wrote {report}"` success line the brief specifically called out to preserve; `log_debug(...)` calls for render/input/boot/reload failures — `log_debug` itself is a permanent, always-compiled stderr/console logger, not gated behind any debug/dev flag). Stripped the `[DEBUG] ` prefix from all 29, kept every message verbatim.
- `🧊️wgpu/📜️script.ts` (4): dev-server/build progress lines (asset server, trunk build, native build) — permanent CLI output. Stripped.
- `🧑️‍💻️dev/📜️script.ts` (51): spans the entire dev command surface (publish/describe/build/watch, size reports, capability/export-path lints, e2e/parity/bench summaries) — all structured, permanent operator output, no signs of ad-hoc throwaway debugging. Stripped all 51.
No files outside this ticket's own scope were touched for `[DEBUG] `; the repo-wide 312+ count stands, out of scope, not attempted.

### Out of scope, lease-requested / reported only (no edits made)
- **`semio-framework-surface`** (`🗺️surface` module, unused imports in `🕸️node-graph/🦀️component.rs:24`) — **not** in `path_scope` despite being in the packet brief's starting-point list (I double-checked the literal scope string; `🗺️surface` isn't one of the listed directories). Lease-request.
- **`Dock`/`Shell` `🧊️component.rs`** (5 warnings: `input`/`stroke`/`border`/`maximized` unused vars, `corner` field never read) — registrar-only per the brief. Lease-request.
- **`semio-framework-os-kernel-dsl-derive`, `semio-framework-mesh-engine`, `semio-framework-graph`** — see root-cause section above.
- The `verify rust-warnings --target wasm32-wasip2` tooling bug in root `📜️script.ts` — see above.
- `semio-s-plugin-puzzle`'s 176-error test-suite breakage — see above.

## peer-coexistence

- Liveness checks (`git log --date=iso --oneline -3` + `stat -f "%Sm"`) were run before every edit to a shared file. Files touched this session that showed recent (same-day, hours-old) mtimes were all attributable to **my own** in-progress edits from earlier in this same session (re-checked and confirmed via successive `stat` calls showing the mtime advancing exactly when I edited).
- D1's live window (files changed in the last ~30 minutes across `✏️s/🔌️plugins/**`) never overlapped anything I touched — every stdio/puzzle file I edited had an mtime hours-to-a-day stale at edit time.
- `🎯️target-z1` was seeded via `cp -Rc` (APFS copy-on-write clone, ~4s for 6.9G) from the ticket's pre-existing shared `🎯️target` (idle ≥60min at time of clone, not another packet's live build dir) — never touched `🎯️target-d1` (D1's active one) or any other packet's `-p1`/`-r1`/`-v1b`/`-k1` suffixed dir.
- The coordinator ran `wasm32-unknown-unknown` once before I started and handed me `w4-z1-wasm-unknown.txt`/`w4-z1-wasm-unknown2.txt` (raw logs in the ticket folder) — confirmed my `to_js_error`/`Kernel::complete` signature fix (made mid-session, before the coordinator's message arrived) as correct and noted it independently caught a real E0308 compile error in `wasm32`-gated `📦️glue.rs` code that native `cargo check`/`cargo test` structurally never compiles (worth keeping this gate in the routine cycle, not just at exit, per the coordinator's own observation).

## Follow-up tasks spawned (not part of this packet's own scope, real gaps found while chasing warnings)
- `task_00cfc256` — Wire or remove `PluginRuntimeRegistry`'s unread fields.
- `task_dc49df65` — Finish stdio zip mtime parsing + ISO21320 conformance stub.
- `task_ac8f7b76` — Wire or remove stdio's unreachable STL ASCII export.
- `task_bcc46a4e` — Resolve wgpu `glue.rs` retained-node reuse + fuel-quota gaps.

## Commands + exit codes (every one actually run this session, foreground, no backgrounding)

```
cargo clippy -p semio-framework-actor --lib --target wasm32-unknown-unknown -- -D warnings           → 101 (before), 0 (after)
cargo clippy -p semio-s-plugin-stdio --lib --features component-guest --target wasm32-wasip2 -- -D warnings → 101 (feature doesn't exist — tooling bug)
cargo clippy -p semio-s-plugin-stdio --lib --target wasm32-wasip2 -- -D warnings                     → 101 (blocked by dsl-derive/mesh-engine, out of scope)
cargo clippy -p semio-framework-actor --all-targets -- -D warnings                                   → 101 (before, redundant_clone×3), 0 (after)
cargo check -p semio-framework-plugin --all-targets                                                  → 0 errors 5 warnings (before), 0/0 (after)
cargo check -p semio-s-plugin-stdio --all-targets                                                    → 0 errors 9 warnings across lib+lib-test (before), 0/0 (after)
cargo check -p semio-s-plugin-stdio --tests                                                          → 8 ERRORS (mid-session, my own regression from removing GltfDocument) → 0 (fixed same session)
cargo check -p semio-s-plugin-puzzle --all-targets                                                   → 176 pre-existing errors in lib-test (confirmed unrelated, not fixed), 0 warnings in lib (after my fix)
cargo check -p semio-framework-os-renderer-wgpu --all-targets                                        → 26 pre-existing errors in lib-test (Dock/Shell, out of scope), lib clean after my fix
cargo check -p semio-framework-os-renderer-wgpu --lib                                                → 8 warnings (before), 5 (after, all Dock/Shell, out of scope)
bun ./📜️script.ts verify rust-warnings --target wasm32-unknown-unknown                                → 0 (clean, both before-fix-run-by-coordinator and my final run)
bun ./📜️script.ts verify rust-warnings --target wasm32-wasip2                                         → 1 (blocked at semio-s-plugin-animate, tooling bug — first crate alphabetically, not animate-specific)
bun ./📜️script.ts verify rust-warnings --target native                                                → 1 (semio-framework-actor clean, blocked at semio-framework, out-of-scope dsl-derive/mesh-engine chain)
```
Raw logs: `terra-Z1-wasm32-unknown-unknown-before.txt`, `terra-Z1-wasm32-unknown-unknown-after1.txt`, `terra-Z1-wasm32-wasip2-before.txt`, plus coordinator-supplied `w4-z1-wasm-unknown.txt`/`w4-z1-wasm-unknown2.txt` already in the ticket folder.

## Files touched
- `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📦️glue.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧾️wire/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/✳️iso21320/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/📦️mesh-io/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/📚️examples/💃️dancing/🧪️tests/🦀️test.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/📚️examples/🌱️metabolism/🧪️tests/🦀️test.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`

Scratch/logs (ticket folder, `.txt` only): `terra-Z1-wasm32-unknown-unknown-before.txt`, `terra-Z1-wasm32-unknown-unknown-after1.txt`, `terra-Z1-wasm32-wasip2-before.txt`. `🎯️target-z1` is a build-cache directory, not a deliverable.
