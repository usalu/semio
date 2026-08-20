# 📓️ terra-gate-3d-report — gate-3d packet

Ticket: `MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`. Scope: `🧰️framework/🔨️modules/🧊️3d/**` — three
`.rs` files (`🥽️mesh/🦀️component.rs` 2769→2807 lines, `⚙️engine/🦀️component.rs` 68 lines — pure data
types, no fns, untouched, `📦️packages/🦀️rust/📦️glue.rs` 28 lines — module wiring, untouched) plus this
crate's own `📦️packages/🦀️rust/Cargo.toml`. All errors and all fixes were in `🥽️mesh/🦀️component.rs`.

## Trajectory (every count from a fresh `cargo check`, never carried over from memory)

| stage | `--lib` errors | mechanism |
|---|---:|---|
| handoff / fresh count | 295→296¹ | — |
| `insert-await.py` pass 1 | 170 | 209 mechanical `.await` edits |
| hand: 2× `.await` inside sync closure (E0728) | — | R10 shape 1 |
| `insert-await.py` to fixpoint | 150 | 41+11+5+1+2 more mechanical edits |
| hand batch A (`Vec3::add/sub/scale/dot/cross/length/normalize`, `from_faces` ctor, `face_normal`) | 117 | R10 shape 6 + missing-await |
| hand batch B (transform fns, inset/bevel/loop_cut/knife_cut/merge_vertices/dissolve_edges/subdivide, `newell_normal`/`plane_basis`/`point_in_triangle`/`triangulate_polygon` internals) | 30 | R10 shape 2 (repeated `.await`) + missing-await, dominant pattern |
| hand batch C (`decimate`/`mirror`/`drop_unreferenced_vertices`/`segment_plane_intersect`/`cot_angle`/`pack_island_uvs`/`unwrap_uv`/`collinear_cleanup`/`faces_coplanar`/`tessellate`) | **0** | same patterns |

¹ handoff brief said 295; my own fresh `cargo check -p semio-framework-3d --lib` (first command run
this packet, pasted below) measured 296 `error[E` lines by grep but cargo's own summary line said
"296 previous errors" — not a discrepancy worth chasing per the geometry-residue precedent.

`--all-targets` was RED even after `--lib` hit 0 (rule 26 payoff, textbook case, same as
`geometry-residue`): 77 `#[test] async fn` sites had never compiled (no diagnostic code, `--lib`
never sees `#[cfg(test)]`). Fixed with `async-test-attr.py --apply` (62 sites in one file, one
`[dev-dependencies]` line added to `Cargo.toml`), which then surfaced 140 real type errors in the
test bodies (missing `.await` inside `assert_eq!`/`assert!` macro args — rustc emits these E0369/
E0277/E0600/E0605 with **no suggestion children at all**, confirmed by inspecting the JSON
diagnostics before doing any hand work, so this was never in reach of the mechanical tool). Fixed by
hand across ~50 test functions. Final: `--lib` **EXIT 0**, `--all-targets` **EXIT 0**, both zero
warnings from this crate's own code (the 57 warnings reported alongside are `async_fn_in_trait` from
the `semio-framework-os-kernel` dependency, allowed crate-wide per R7, not this crate).

## R10 residue shapes found — all four, one dominant

1. **`.await` inside a sync closure**: `subdivide` midpoint cache's `.entry().or_insert_with(closure)`
   (De Casteljau icosphere subdivision) and `merge_coplanar_faces`'s `.iter().find(closure)` — both
   converted to explicit loops (HashMap manual get/insert; manual `for` with early break).
2. **Awaiting one future repeatedly — by far the dominant shape in this crate**, tens of instances:
   `rotate`/`rotate_vertices`'s `ax`/`p` axis components read 3× each; `bevel_edges`'s `mid`/`offset`
   each used twice; `inset_faces`'s `to_center` used twice per iteration (×2 loops); `knife_cut`'s
   `cut_dir` re-awaited **inside a `for` loop** (moves on iteration 1, E0382 on iteration 2 — genuine
   bug the conversion exposed, not caused: the plane normal is loop-invariant and is now hoisted
   above the loop, which is also a correctness improvement, not just a compile fix);
   `triangulate_polygon`'s `normal` used 4×; `plane_basis`'s `axis_u` used 3×; `point_in_triangle`'s
   `cross2` results; `faces_coplanar`'s `na`/`na_n`/`nb`/`nb_n` each used 2–3× (the worst offender —
   `na_n` was read 3 times across the function). All fixed by hoisting to a single `.await` at
   creation, safe throughout since every producing type (`Vec3`, tuple `Vec3f64`) is `Copy`.
3. **Self/mutually-recursive async needing `Box::pin`**: none found (same as `geometry-residue` —
   this crate's recursion, ear-clipping in `triangulate_polygon`, is iterative at the fn level).
4. **Futures stored in structs / chains over futures**: `find_closest_bridge`'s `d` (chained into an
   `Option` tuple then compared inside a sync `.map()` closure) — awaited once up front instead of
   compared-as-future-then-awaited-on-store.

## R9 check — both halves shown; only one candidate, and it was E4 not E9

Swept every pure-looking free/assoc fn touched this packet (`Vec3::{x,y,z,add,sub,scale,dot,cross,
length,normalize,lerp}`, `newell_normal`, `sub3/dot3/cross3/normalize3/plane_basis/cross2/
point_in_triangle`, `find_edge_position`, `merge_face_loops`, `faces_coplanar`, `collinear_cleanup`,
`triangulate_polygon`). **Every one has only async-capable consumers** (other async fns in this
file, or now-`#[async_test]` test bodies) — R9 §3 applies: correct move was hoisting `.await` at the
consumer, not detagging the producer. Zero new R9 tags added.

The one sync fn in scope, `default_uv` (`⚠️` shown both halves): **I/O-free** (returns a literal
`[0.0, 0.0]`, no `std::fs`/`tokio`/`File::`/etc) **and** its sole consumer is
`#[serde(default = "default_uv")]` on `HalfEdge::uv` — a **fn-pointer slot** (E4, not E1/R9: serde's
`default = "path"` attribute macro-generates a call expecting a plain `fn() -> T`, and an `async fn`
item's pointer type is unnameable there, language-fixed same as E3). Tagged:
```rust
// 🚫️async: E4 fn-pointer slot — serde's `#[serde(default = "...")]` calls this by path as a plain
// `fn() -> T`; an `async fn` item's pointer type is unnameable there.
fn default_uv() -> [f32; 2] {
```
No other E1/E4/E5 cases in this crate — no manual `From`/`Display`/`Debug`/`Default`/`Iterator` impls
(only derives), no `const fn`, no `extern`/`main`/proc-macro, no `block_on`.

## 🚨 Dropped-future bug found — silent no-op, exactly the flagged highest-value class

`pub async fn mark_uv_seam(&mut self, edges: &[EdgeId], seam: bool)` returns `()`. Its **only** three
call sites (all in `#[cfg(test)]`, no production callers repo-wide — grepped) called it at statement
position without `.await`:
```rust
mesh.mark_uv_seam(&[EdgeId(0)], true);   // future created, dropped, never polled — no-op
```
This compiled clean at every stage (a `()`-returning future satisfies "expression statement" with no
coercion needed, exactly as the packet brief warned) and was caught only by running the test suite:
`mark_uv_seam_toggles_and_is_uv_seam_reports_state` failed at
`assert!(mesh.is_uv_seam(EdgeId(0)).await)` because the preceding `mark_uv_seam(..., true)` had never
actually run. Fixed all 3 call sites (2 in the failing test, 1 in
`unwrap_uv_splits_islands_across_seam`, which was passing "by accident" — its assertion only checked
`!transfer.uvs.is_empty()`, not that the seam split actually happened). Verified no other bare
`()`-returning async fn exists in this crate (`grep 'async fn .*) {$'` minus `-> ` hits, cross-checked
against every production — non-test — signature: only `mark_uv_seam`).

## Files touched (all within owned scope)

- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/Cargo.toml` (added
  `[dev-dependencies] semio-framework-async-macros`, via `async-test-attr.py --apply`)

No new tool built. Used the shared `insert-await.py` (mechanical layer) and shared
`async-test-attr.py` (`#[test]` → `#[async_test]` + dev-dependency, scoped to
`🧰️framework/🔨️modules/🧊️3d` only, not repo-wide). All hand residue was diagnostic-driven — every
edit traced to a real `cargo check` line:col, re-verified from a fresh read before each edit, no
name/regex-keyed pass.

## Acceptance — every command run in the foreground this turn, `CARGO_TARGET_DIR` in scratchpad

```
$ CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/target-gate-3d \
  cargo check -p semio-framework-3d --lib --message-format=short
    Checking semio-framework-3d v0.1.0 (.../🧊️3d/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 1.85s
EXIT_LIB:0
```

```
$ CARGO_TARGET_DIR=.../scratchpad/target-gate-3d cargo check -p semio-framework-3d --all-targets --message-format=short
    Finished `dev` profile [unoptimized] target(s) in 0.43s
EXIT_ALL:0
```
Zero warnings from `semio-framework-3d` itself on either command (the 57 lines printed are
`async_fn_in_trait` warnings from the `semio-framework-os-kernel` dependency crate, R7-allowed).

```
$ CARGO_TARGET_DIR=.../scratchpad/target-gate-3d cargo test -p semio-framework-3d
running 62 tests
... (62 lines)
test result: ok. 62 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
Doc-tests semio_framework_3d: 0 passed; 0 failed
EXIT:0
```
Named set (rule 11): all 62 are new-to-this-packet passes — `#[test] async fn` never compiled under
`--all-targets` before this packet (no `#[async_test]`), so there is no prior baseline to diff
against; this **is** the baseline going forward. One test (`mark_uv_seam_toggles_and_
is_uv_seam_reports_state`) failed on the first run for the dropped-future reason above; fixed; full
suite green on re-run, 62/62.

```
$ CARGO_TARGET_DIR=.../scratchpad/target-gate-3d cargo check -p semio-framework-os-kernel --lib --message-format=short
    Finished `dev` profile [unoptimized] target(s) in 0.16s
EXIT_OSKERNEL:0
```

```
$ CARGO_TARGET_DIR=.../scratchpad/target-gate-3d cargo check -p semio-framework --lib --message-format=short
    Finished `dev` profile [unoptimized] target(s) in 12.94s
EXIT_FRAMEWORK:0
```
Both re-verified at the end per rule 26 / this packet's acceptance #3, still green (I never touched
`🏪️store`/`🗣️dsl`/`💡️inference` — confirmed by `git log`/scope discipline, not by inference from files
I didn't write).

## Summary

`semio-framework-3d`: `--lib` 295/296 → **0**. `--all-targets` (not separately tracked at handoff,
discovered here exactly as rule 26 predicts) 77 (missing `#[async_test]`) → 140 (real residue,
diagnostic-confirmed unfixable by the mechanical tool) → **0**. `cargo test`: **62 passed / 0
failed** (one genuine dropped-future logic bug found and fixed along the way, not just a compile
fix). `os-kernel --lib` **EXIT 0**, `semio-framework --lib` **EXIT 0**. No `lease-request` needed —
never touched a peer-owned file. No R9 detagging performed; one E4 tag added (`default_uv`, serde
fn-pointer slot).
