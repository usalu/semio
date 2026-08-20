# R2 — De-Async Repair: `semio-framework-2d` + `semio-framework-graph`

## Scope

Packet R2 of Phase 1.5. Boundary: `🧰️framework/🔨️modules/◻2d/**` (crate `semio-framework-2d`,
manifest at `🧰️framework/🔨️modules/◻2d/📦️packages/🦀️rust/Cargo.toml`) and
`🧰️framework/🔨️modules/🕸️graph/**` (crate `semio-framework-graph`, manifest at
`🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/Cargo.toml`). `./compose` and `semio-compose-rs`
untouched, out of scope.

## Reproduction

Contrary to the packet brief's warning about needing workspace feature unification (true for
`semio-framework-ui`), both crates reproduce their exact reported error counts with a plain
per-package check — no `--workspace`, no non-default features required:

```
cargo check -p semio-framework-2d       # → 28 errors (lib target) — matches packet estimate exactly
cargo check -p semio-framework-graph    # → 8 errors  (lib target) — matches packet estimate exactly
```

`--all-targets` on `semio-framework-2d` alone pulls in 66 (28 lib + 38 more from `#[test] async fn`
bodies using the same broken pattern, since the lib errors were masking them from being reached).
`--no-default-features` on `semio-framework-2d` (i.e. `booleans` off) was 0 errors — the entire bug
lived in the `booleans` and `trace` feature-gated modules, both on by default.

Both crates also compile clean under full workspace-unified `cargo check --workspace --all-targets`
(verified below) — nothing about our two crates needed the wider feature set to expose or resolve.

## Error trajectory

| Step | 2d errors (lib) | graph errors (lib) |
|---|---|---|
| Baseline | 28 | 8 |
| After fix | 0 | 0 |

Workspace-wide `cargo check --workspace --all-targets` after the fix: 931 errors remain, all
attributable to sibling packets' crates (`semio-s-plugin-draw-fsm`, `semio-framework-machine-derive`,
`semio-hub`, `semio-framework-ui-backend-d3d12`, `semio-framework-ui-backend-webgpu`,
`semio-framework-ui`, `semio-compose-rs`) — grepping the error log for `◻2d` or `🕸️graph` paths
returns nothing.

## What changed

### `semio-framework-2d` — two files, no callers outside the crate needed changes

- `🧰️framework/🔨️modules/◻2d/🔍️trace/🦀️component.rs` — bitmap marching-squares tracer +
  Douglas-Peucker simplifier. All 8 functions (`pixel_on`, `marching_squares_contours` and its
  nested `direction`, `perpendicular_distance`, `douglas_peucker`, `contour_to_segments`, public
  `trace_bitmap_paths`) plus all 7 `#[test]` bodies lost `async`. Pure geometry/bitmap math, zero
  I/O, zero channels, zero timers — no suspension point existed anywhere in the file.
- `🧰️framework/🔨️modules/◻2d/🔀️booleans/🦀️component.rs` — planar boolean ops on `geo` polygons.
  All 6 functions (`close_polygon`, `segments_to_multipolygon`, `ring_to_segments`,
  `polygon_to_segments`, public `boolean_paths`, public `boolean_paths_many`) plus the `square` test
  helper and all 7 `#[test]` bodies lost `async`. Same story: pure `geo`-crate polygon math.
- Verified the two public entry points (`boolean_paths`/`boolean_paths_many`/`trace_bitmap_paths`)
  against their only outside callers — `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️component.rs`
  and `✏️s/🔌️plugins/🖍️draw/…/🧬️schema/🦀️component.rs` — both already call them as plain synchronous
  `Result`-returning functions (`?` / `match` directly on the return value, no `.await` anywhere).
  Removing `async` here doesn't just avoid breaking those call sites, it makes them correct — they
  were presumably already broken by the same bug class, just outside packet R2's boundary.
- Net: **14 functions de-asynced, 0 call sites needed a new `.await`.**

### `semio-framework-graph` — two files

- `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️component.rs` — `impl dsl_core::DslField for
  PropertyValue { to_value, from_value }` were still `async fn` (E0053: incompatible with the
  externally-declared `dsl_core::DslField` trait, whose methods are sync — a sibling method,
  `shape()`, already carried a code comment recording that exact E1/E4 classification, but
  `to_value`/`from_value` were missed). De-asynced both, plus their only two private callees
  `property_value_to_dsl_value`/`dsl_value_to_property_value` (self-recursive pure `PropertyValue`
  ⇄ `DslValue` converters — the recursive calls previously needed `Box::pin(...).await` purely
  because `async fn` self-recursion requires boxing; as plain `fn` the recursion needs no pinning at
  all). Updated the one test that called the trait methods directly to drop its two `.await`s.
  Left the rest of this file's large, self-consistent `async`/`.await` web (the file already runs
  its tests through a hand-rolled single-poll `block_on_test` bridge, documented inline as an
  "E5-class executor bridge, sanctioned per R4 clause 5") untouched — it wasn't broken and touching
  it was outside what the 2 reported errors required.
- `🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🦀️component.rs` — inverse bug shape from everywhere else in
  this packet: **stray `.await` on already-synchronous `dsl_core` (= `semio_framework_os_kernel`,
  aliased via `extern crate`) calls**, not a missing one. Four call sites fixed by deleting the
  erroneous `.await`:
  - `render_wire_line`: `dsl_core::Writer::new().await` → `dsl_core::Writer::new()`,
    `dsl_core::print_shape(...).await` → `dsl_core::print_shape(...)`,
    `writer.render(...).await` → `writer.render(...)`. A pre-existing code comment on this function
    got the direction of the original bug backwards (claimed `Writer::new()` "was never awaited");
    replaced it with an accurate one. Left `render_wire_line` itself `async fn` — it has callers
    (`wire_literal_from_dag`) that already correctly `.await` it, and its body has zero suspension
    points now, so it's a legal (if non-suspending) async fn; changing its signature would have
    cascaded into a ~2000-line query-engine call graph nowhere near the 8 reported errors, well
    outside a compiler-driven bounded edit.
  - `dag_from_wire_literal`: `dsl_core::parse_wire_text(line).await?` → `...(line)?`.
  - `push_dsl_core_segment`: `dsl_core::os_dsl::lex(...).await.map_err(...)` → `...(...).map_err(...)`.
  - `lex_spanned`: `dsl_core::os_dsl::unescape_text(...).await.unwrap_or(raw)` →
    `...(...).unwrap_or(raw)`.
  - No caller of any of these four fixed sites needed a signature change — every enclosing function
    (`render_wire_line`, `dag_from_wire_literal`, `push_dsl_core_segment`, `lex_spanned`) stayed
    `async fn` and kept compiling because each still has other genuine internal `.await`s on this
    crate's own (still-async, unbroken) helpers.
- Net: **2 methods + 2 helpers de-asynced (manifest), 4 stray `.await`s deleted (dsl), 0 function
  signatures changed in the dsl file.**

## Category-C seed notes (long-running CPU work, for later resumable-job conversion)

Per the packet brief's architectural-awareness ask — functions touched here that are genuine
CPU-bound traversal/algorithm work, flagged for whoever does the Phase 3/5 resumable-job conversion:

- `marching_squares_contours` (`◻2d/🔍️trace`) — O(width×height) scanline pass building an edge set,
  then a contour-walking loop over all edges. Unbounded by caller-supplied bitmap size.
- `douglas_peucker` (`◻2d/🔍️trace`) — recursive polyline simplification, worst-case O(n²) on
  pathological point sets.
- `boolean_paths`/`boolean_paths_many` (`◻2d/🔀️booleans`) — delegates the real work to `geo`'s
  polygon-clipping algorithms; the surrounding segment⇄polygon conversion loops are linear but the
  underlying boolean-op itself is not bounded by this crate.
- `property_value_to_dsl_value`/`dsl_value_to_property_value` (`🕸️graph/🛂️manifest`) — unbounded
  recursive tree walk over arbitrarily nested `PropertyValue`/`DslValue`.
- None of the four `dsl/🦀️component.rs` call sites touched in this packet are themselves the
  CPU-heavy part — `render_wire_line` is O(1) per wire, `push_dsl_core_segment`/`lex_spanned` are
  per-token lexer work. The DSL query engine's `match_pattern`/`execute` family (untouched, not
  broken) is the actual long-running part of that file and is worth a closer look in a later phase.

## Cross-boundary observations (not touched, reported only)

- `🧰️framework/🔨️modules/◻2d/⚙️engine/🦀️component.rs` has an unrelated `pub async fn
  block_on<F>(future: F) -> F::Output { pollster::block_on(future) }` — itself async but never
  `.await`s internally (delegates to sync `pollster::block_on`), and it's dead code: nothing in the
  workspace calls `compute::block_on`/`engine::block_on`. Not a compile error, so left untouched
  per the compiler-driven edit discipline; flagging in case a later phase wants to delete it or fix
  its self-inconsistent async-ness.
- The two external callers of `boolean_paths`/`boolean_paths_many`/`trace_bitmap_paths` mentioned
  above (`💻️os/🔨️modules/🌊️flow/🖍️drawing` and the `🖍️draw` plugin's schema component) are outside
  packet R2's boundary and were not edited, but both already called these functions synchronously
  before this fix — meaning removing `async` here is a strict compatibility improvement for them,
  not a break. Neither crate showed up in the post-fix workspace error list, so this needs no
  follow-up.

## Verification actually run (all passed)

```
cargo check -p semio-framework-2d                              # 0 errors (was 28)
cargo check -p semio-framework-2d --all-targets                 # 0 errors
cargo check -p semio-framework-graph                             # 0 errors (was 8)
cargo check -p semio-framework-graph --all-targets               # 0 errors
cargo clippy -p semio-framework-2d --all-targets                 # 0 errors, 0 warnings in our files
cargo clippy -p semio-framework-graph --all-targets               # 0 errors, 0 warnings in our files
cargo test -p semio-framework-2d                                  # 21 passed; 0 failed
cargo test -p semio-framework-2d --release                        # 21 passed; 0 failed
cargo test -p semio-framework-graph                                # 174 passed; 0 failed
cargo test -p semio-framework-graph --release                      # 174 passed; 0 failed
cargo check --workspace --all-targets                              # 931 errors, none in our crates
bun ./📜️script.ts verify dependencies                              # clean, 238 == 238 (baseline unchanged)
```

No `wasm32-unknown-unknown`/`wasm32-wasip2` build was run: neither crate has any
`wasm32`/`wasm_bindgen`-gated code (`grep -rl "wasm32\|wasm_bindgen"` over both module trees is
empty), so there is nothing wasm-gated to build.

## Files touched

- `🧰️framework/🔨️modules/◻2d/🔍️trace/🦀️component.rs`
- `🧰️framework/🔨️modules/◻2d/🔀️booleans/🦀️component.rs`
- `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️component.rs`
- `🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🦀️component.rs`

No files created or removed. No `Cargo.toml`/`project.json`/`📜️script.ts` changes needed in either
crate.
