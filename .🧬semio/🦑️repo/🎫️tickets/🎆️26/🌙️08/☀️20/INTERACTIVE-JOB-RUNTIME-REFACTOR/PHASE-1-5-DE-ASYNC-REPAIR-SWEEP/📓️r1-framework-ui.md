# R1 — `semio-framework-ui` De-Async Repair

## Reproduction command

The workspace-reported "557 errors in `semio-framework-ui`" figure is reproduced **exactly** (rustc
itself reports "557 previous errors") by:

```
cargo check -p semio-framework-ui --features wgpu,wgpu-engine,tui,tui-terminal --lib
```

Note `--lib`, not `--all-targets`. Workspace feature unification enables `wgpu`+`wgpu-engine` (pulled
in by `🛍️products/💻️os/🔨️modules/🌊️flow`, `♾️infinite`, `📺️renderer/🎯️targets/🧊️wgpu`, `🖥️host`,
`🌉️mcp`, `🧰️framework/📦️packages/🦀️rust`, and several `✏️s/🔌️plugins/*`) together with `tui`+
`tui-terminal` (pulled in solely by `🛍️products/🦑️repo/🔨️modules/⌨️cli`, `features = ["tui-terminal"]`).
No workspace member requests `typegen`.

`-p ... --all-targets` (which also builds the `#[cfg(test)]` unittest binary) overshoots to 1494,
because it recompiles the same source under the `test` cfg and additionally hits an **unrelated**
pre-existing bug (see "Known pre-existing, out-of-scope defect" below) that was never part of the 557
baseline — `cargo check --workspace --all-targets`, filtered to this crate's own path
(`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/`, as opposed to the sibling `🖼️render/🎯️targets/*`
backend crates or `🎬️scene`), matched exactly 557 hits before this packet's edits and 0 after.

## Bug class confirmed

Every one of the 557 errors was `async fn` called without `.await`, exactly as described. Verified
per-file before touching anything: `grep -c '\.await'` was **0** in every file that had errors, so
every affected function was non-suspending (census categories B/C) — none were genuine-suspension
call sites needing an added `.await`.

## Error count trajectory

| Step | Command | Errors |
|---|---|---|
| Baseline | `cargo check -p semio-framework-ui --features wgpu,wgpu-engine,tui,tui-terminal --lib` | 557 |
| After stripping `async` from `⌨️tui/🦀️component.rs` (374 fns) | same | 24 |
| After stripping `async` from 13 `🧱️elements/*/⌨️component.rs` files (26 fns) + fixing `math.rs`/`draw.rs`/`widgets.rs` call sites | same | 0 |
| Final | same | **0** |
| Default features (`--all-targets`, no explicit features) | `cargo check -p semio-framework-ui --all-targets` | 0 (unchanged, was already clean) |
| Workspace, filtered to this crate's path | `cargo check --workspace --all-targets` | 557 → **0** |
| clippy | `cargo clippy -p semio-framework-ui --features wgpu,wgpu-engine,tui,tui-terminal --lib` | 0 errors (56 pre-existing style warnings, unchanged in kind) |
| wasm32-unknown-unknown | `cargo check -p semio-framework-ui --target wasm32-unknown-unknown --features wgpu-engine,tui-bindgen --lib` | 0 |
| wasm32-wasip2 | `cargo check -p semio-framework-ui --target wasm32-wasip2 --features wgpu --lib` | 0 |
| dependency ratchet | `bun ./📜️script.ts verify dependencies` | clean, 238 = 238 |

## async fn removed vs .await added

- **400 `async fn` → `fn`** (git diff: `-async fn` × 400, `+.await` × **0**).
  - 374 in `🧰️framework/🔨️modules/🖱️ui/⌨️tui/🦀️component.rs` (the entire terminal-emulator engine:
    geometry, cell buffer, ANSI diff/emit, VT100 input parser, VT screen parser, layout/flex solver,
    window-stack/tab management, dispatch loop, PTY spawn/copy/paste). Census: 293 category B, 81
    category C, 0 A/A-shallow — matches the file's own 0 `.await` count exactly.
  - 26 across 13 `🧱️elements/*/⌨️component.rs` files (`Select`, `Input`, `Divider`, `Chip`, `Label`,
    `List`, `Table`, `Tabs`, `Footer`, `Navbar`, `Wizard`, `Window`, `Log` — every tui-target element's
    `*_on_key` and `paint_*` fn).
- **0 `.await` added.** No call site in this crate needed a genuine suspension — consistent with
  Phase 0's 88.28% non-suspending finding.
- The 24 residual errors after the bulk strip were all **call sites**, not fresh async-fn definitions:
  cascading fixes once the callees above lost `async`, plus one pre-existing gap (below) where two
  `wgpu`-target files (`🎯️targets/🧊️wgpu/🦀️widgets.rs`, `🦀️draw.rs`) were still calling the *foreign*
  `semio_framework_geometry::Vec3`/`Mat4` crate's own `pub async fn` inherent methods (`.sub()`,
  `.scale()`, `.dot()`, `.cross()`, `.length()`, `.normalize()`, `Vec3::new()`, `.to_cols_array()`)
  instead of the sync mirror API (`🎬️scene/📦️packages/🦀️rust/🦀️math.rs`'s `Vec3Math`/`Mat4Math`
  traits, suffixed `_m`) that an earlier packet had already built for exactly this reason (see that
  file's own `🚫️async: E6` docstring — `semio-framework-geometry` is outside this packet's path scope,
  so the fix is call-site-only, not touching the foreign crate).
  - Made `Vec3Math`/`Mat4Math` `pub` (they were crate-private) and re-exported them from
    `🎯️targets/🧊️wgpu/📦️glue.rs`'s `kernel_3d_scene` re-export list.
  - Added `Mat4Math::to_cols_array_m` (was missing from the mirror trait) — same column-major flatten
    as the foreign crate's `Mat4::to_cols_array`, verified against that crate's own
    `mat4_to_cols_array_matches_column_major_layout` test.
  - Rewrote `widgets.rs`'s `orbit_view_gizmo_tips` (`sub`→`sub_m`, `scale`→`scale_m`, `dot`→`dot_m`,
    `cross`→`cross_m`, `length`→`length_m`, `normalize`→`normalize_m`, `Vec3::new(x,y,z)` → struct
    literal `Vec3 { x, y, z }`, legal since the foreign crate's `Vec3` fields are `pub`) and
    `draw.rs`'s two `World3dGpuInstance::from_instance(instance.model.to_cols_array(), …)` call sites
    (`to_cols_array` → `to_cols_array_m`).

## Files touched

- `🧰️framework/🔨️modules/🖱️ui/⌨️tui/🦀️component.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🦀️math.rs`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️draw.rs`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️widgets.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/{☑️Select,✏️Input,➖️Divider,🏷️Chip,🏷️Label,📃️List,📊️Table,📑️Tabs,🔚️Footer,🔝️Navbar,🧙️Wizard,🪟️Window,🪵️Log}/⌨️component.rs`

No `.await` was inserted anywhere; no new third-party dependency was introduced (ratchet: 238 = 238).

## Category-C functions touched (Phase 5 seed notes)

Phase 5 turns the run-to-completion frame transaction into a resumable, cursor-based, 8ms-ceiling
worker-owned state machine. These now-sync functions are exactly the CPU-bound work that transaction
will need to chunk. From the census (`bodyLineCount`/`loopKeywords` shown), the largest/most relevant
in `⌨️tui/🦀️component.rs`:

- `dispatch` L3727, 51 lines, `while`+`loop` — the tui target's central event-dispatch loop.
- `window_chip_layout` L3370, 107 lines, `for`+`map()` — window-chrome tab/chip layout.
- `mount_window_layout` L3517, 106 lines — window-stack (re)mount.
- `spawn` L4602, 113 lines, `map()` — PTY process spawn (cmdline/env construction, no actual `.await`
  suspension; the real PTY I/O is elsewhere).
- `layout_corner_tabs` L3292, 68 lines, `for`+`map()`.
- `apply_sgr` L1625, 65 lines, `while` — VT100 SGR attribute parser.
- `distribute` L2063, 46 lines, `for` — flex/stack size distribution solver.
- `native_copy` L3903, 45 lines, `for`+`map()` / `finish_csi` L1022, 45 lines, `for`.
- `layout_node` L2115, 39 lines, `for`+`map()`.
- `split_in_axis` L2422, 32 lines / `window_hit` L3137, 34 lines / `erase_display` L1433, 35 lines.

Every `paint_*` fn in the 13 element files is category C (this is literally the presentation-packet
work Phase 3/5 target): notably `paint_window` (88 lines), `paint_table` (74 lines), `paint_wizard`
(37 lines), `paint_navbar`/`paint_items`, `paint_log`, `paint_footer`, `paint_divider`, `paint_label`.
None of these currently suspend or block — they are plain synchronous CPU passes over already-resolved
state, which is exactly the shape Phase 5 needs to slice into resumable steps.

## Known pre-existing, out-of-scope defect (NOT touched)

`cargo check -p semio-framework-ui --features wgpu,wgpu-engine,tui,tui-terminal --all-targets` (i.e.
adding the `#[cfg(test)]` unittest target on top of the now-clean `--lib`) still shows **84 errors**,
all of a *different* bug class: `label_impl::Label: From<&str>` unsatisfied (in
`🎯️targets/🧊️wgpu/{scene_slots.rs,events.rs,engine.rs,…}`), cascading into a handful of `E0308`
`Option<String>` vs `Option<Label>` mismatches. This is **not** an async/`.await` bug — `Label` is a
deliberate compile-time-checked-label gate (see `🎯️targets/🧊️wgpu/🦀️label.rs`'s own doc comment: "No
`From<&str>`/`From<String>` on purpose", ticket
`26/08/03/COMPILE-TIME-CHECKED-UI-LABELS-ACROSS-LOCALE-TERMINOLOGY-AND-BRAND`) that this pre-existing
test/production code violates by passing raw string literals where `Label::data(...)` or an
`app_labels!`-produced `LabelText` is required. Confirmed pre-existing (present in the very first
`--all-targets` probe of this crate, before any edit in this packet, and **not** among the 557 hits the
orchestrator's workspace-path filter attributed to this crate — that filter only saw the `--lib`
target). Left untouched: it's a different ticket's gate, not this packet's mechanical async-fn
class, and fixing it correctly requires per-call-site judgment about real vs. placeholder UI copy
that's out of R1's remit. `cargo check -p semio-framework-ui --all-targets` **with default (empty)
features** stays clean throughout (0 errors), since `default = []` never touches this code path.

## Cross-boundary observations (not touched)

- `🧰️framework/🔨️modules/📐️geometry` (crate `semio-framework-geometry`) is genuinely outside
  `🧰️framework/🔨️modules/🖱️ui/**` and was not touched. Its `Vec3`/`Mat4` inherent methods are `pub
  async fn` by that crate's own (out-of-scope) design; the in-boundary fix was entirely on this
  packet's side (sync mirror methods + call-site rewrites), per the pattern an earlier packet already
  established in `🎬️scene/📦️packages/🦀️rust/🦀️math.rs`.
- `🧰️framework/🔨️modules/🖱️ui/🖼️render/🎯️targets/{🧊️webgpu,🪟️d3d12,🌋️vulkan,🍎️metal}` are separate
  crates (`semio-framework-ui-backend-*`) nested under the `🖱️ui/**` path but not named in this
  ticket's own crate-attribution list (only `semio-framework-ui` and
  `semio-framework-ui-backend-vulkan (1)`, the latter explicitly the vulkan sibling packet's). Two
  full `cargo check --workspace --all-targets` runs taken before and after this packet's edits (which
  never touched any file under `🖼️render/`) showed wildly different, mutually-exclusive error counts
  between the `webgpu` and `d3d12` backend crates (476→0 and 0→332 respectively) with no corresponding
  git changes on this packet's side — strong evidence of concurrent sibling-packet churn on shared
  render-target/feature-selection code, matching the "expect sibling crates to still show errors"
  guidance. Not investigated further; flagged here rather than assumed to be this packet's doing.

## Verification actually run

```
cargo check -p semio-framework-ui --features wgpu,wgpu-engine,tui,tui-terminal --lib   # 557 → 0
cargo check -p semio-framework-ui --all-targets                                        # 0 (default features, unchanged)
cargo check -p semio-framework-ui --features wgpu,wgpu-engine,tui,tui-terminal --all-targets  # 0 lib + 84 pre-existing Label-gate errors in unittest target (see above)
cargo clippy -p semio-framework-ui --features wgpu,wgpu-engine,tui,tui-terminal --lib   # 0 errors, 56 pre-existing style warnings
cargo check -p semio-framework-ui --target wasm32-unknown-unknown --features wgpu-engine,tui-bindgen --lib  # 0
cargo check -p semio-framework-ui --target wasm32-wasip2 --features wgpu --lib         # 0
cargo check --workspace --all-targets                                                  # this crate's path: 557 → 0
bun ./📜️script.ts verify dependencies                                                  # clean, 238 = 238
```

`cargo test -p semio-framework-ui` was not run to completion: the unittest target doesn't compile,
blocked by the pre-existing, out-of-scope `Label` gate violations above (not an async regression from
this packet — the `--lib` target those same tests live behind, minus `#[cfg(test)]` code, is clean).

Formatting: `cargo fmt --check --` on explicit paths ignored the path filter and swept unrelated files
(including `compose/`, out of scope) — did not use it. Plain `rustfmt --check --config-path
./rustfmt.toml` on the exact edited files showed pre-existing formatting drift unrelated to this
packet's edits (struct-literal/if-expr line wrapping in code this packet never touched). Left
unformatted rather than take a wholesale-reformat diff on a live, concurrently-edited tree; only the
literal `async fn` → `fn` token removal and the handful of call-site rewrites above were applied,
each still valid under the existing style.
