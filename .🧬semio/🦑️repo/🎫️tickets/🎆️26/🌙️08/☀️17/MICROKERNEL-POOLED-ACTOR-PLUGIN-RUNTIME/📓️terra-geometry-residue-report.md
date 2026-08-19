# 📓️ terra-geometry-residue-report — geometry-residue packet

Ticket: `MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`. Scope: `🧰️framework/🔨️modules/📐️geometry/**` only (3 `.rs`
files: `⚙️engine/🦀️component.rs`, `🎲️random/🦀️component.rs`, `📦️packages/🦀️rust/📦️glue.rs`), plus this crate's
own `📦️packages/🦀️rust/Cargo.toml`.

Handed off at **61 errors** (fresh re-count on my own run: **60**, `--lib` only — the handoff's 61 was a
slightly earlier snapshot; not a discrepancy worth chasing). Re-ran the shared fixpoint tool first as
instructed; it found the crate already at a genuine fixpoint (0 further mechanical edits, same 10
ambiguous, 8 "other" it can't touch because their diagnostic codes carry no `.await`-hint child — e.g.
"async functions cannot be used for tests" has no `code` field at all). Everything from there was hand
work, exactly as R10 predicts for shared-tool residue.

## The 10 ambiguous diagnostics — all one shape, one fix

Every one of the 10 clustered in `⚙️engine/🦀️component.rs` lines 581–631 (`cubic_split`,
`cubic_arc_length`, `distance_point_to_cubic_bezier`'s callees). Inspecting the actual candidate byte
spans (not just the line numbers) showed they are **not** the tool's guard-worthy case — the docstring's
own worked example (`Park.to_u8()` vs `Live.to_u8()`, genuinely different subexpressions, exactly one of
which is correct) does not apply here. In every one of these 10, rustc offered 2–4 candidates because
**multiple sibling arguments in the same call are simultaneously un-awaited futures**, e.g.
`lerp_point(p0, p1, t)` where both `p0` and `p1` are un-awaited `CubicBez::p0()`/`p1()` futures — rustc
proposes "await `p0`" and "await `p1`" as two *independent, both-necessary* fixes, not as alternatives.
Applying only one leaves the other argument still mismatched on the next pass.

**Decision procedure used for all 10, and recommended for any future packet hitting the same wall:**
1. Read the candidate byte spans, not just the line. If the candidates sit on *different*
   subexpressions within the same call (different argument positions), the diagnostic is a
   **both-needed**, not an either/or — the tool's ambiguity guard is a safety default, not evidence of a
   real fork in the road.
2. Trace each un-awaited candidate back to its producer (here: `CubicBez::p0()/p1()/p2()/p3()`,
   `CubicBez::eval()`, `Vec2::dot()`). If the producer is itself an un-awaited async call assigned to a
   `let`, the correct fix is almost always to **hoist the `.await` to the point of creation** (`let p0 =
   c.p0().await;`) rather than scatter `.await` at every downstream use site — this also fixes the
   companion R10-shape-2 bug (a future silently used more than once, e.g. `previous`/`next` re-used
   across loop iterations in `distance_point_to_cubic_bezier`, `cubic_arc_length`) in the same edit.
3. Re-run the fixpoint tool after the hoist. All 10 cleared to 0 remaining ambiguous in the next pass,
   confirming the diagnosis (the ambiguity was a symptom of the missing hoist, not a real fork).

No case in this crate needed "await the one whose value is actually consumed" (the guidance for a
genuine either/or) — every ambiguous diagnostic here was a both-needed multi-argument case.

## R10 residue shapes found (all four, plus the "field access via Deref is not async" trap)

1. **`.await` inside a sync closure** (shape 1): `sort_by`/`dedup_by` were already handled by the prior
   packet's R9 tags on `Point::x()`/`y()`. I hit fresh instances in `geom_sel`'s `.any()`/`.all()`
   predicates calling async `point_in_polygon`/`world_box_contains_point`/`segments_intersect` —
   converted each to an explicit `for` loop with early return (`segment_intersects_world_box`,
   `polygon_contains_world_box`, `polygon_intersects_world_box`, `segment_intersects_polygon`), and in
   `random`'s `powerlaw_sequence`/`discrete_sequence` (`.map(|_| rng.next_f64())` — `rng: &mut Rng`
   can't cross an async boundary through a sync closure either way).
2. **Awaiting one future repeatedly** (shape 2): `circle_line_intersections` awaited `d`/`f`
   (`Vec2::new(...)` futures) up to 4 times each across the function body; `polygon_centroid` awaited
   `area` once for a comparison then tried to reuse the *already-moved* future later; `Mat4::look_at`
   awaited `f`/`s`/`u` between 2–4 times each; `Mat4::mul`/`Mat4::translation` awaited `out`/`m`
   (`Self::identity()`) once per loop iteration. All fixed by hoisting to a single `.await` at
   creation — safe everywhere here because every producing type (`Point`, `Vec2`, `CubicBez`, `Vec3`,
   `Mat4`, `WorldBox`) derives `Copy`.
3. **Self/mutually-recursive async needing `Box::pin`**: none found. Geometry's recursion (De Casteljau
   subdivision in `cubic_split`) is iterative, not self-recursive at the fn level.
4. **Futures stored in structs / `map`/`and_then` chains over futures**: the `for j in (n-k)..n { let t =
   self.next_range(...).await as usize; ... }` shapes in `Rng::shuffle`/`choose`/
   `sample_without_replacement` are the "chain" variant — fixed by awaiting before the cast rather than
   casting a future.
5. **Not in R10's list, found here**: `impl From<(f64, f64)> for Vec2` (`engine/🦀️component.rs:162`)
   called the now-async `Vec2::new(x, y)` from a `From::from` body, which the trait forces to stay sync
   (E1). This is **not** the R9 "detag the async fn" pattern (`Vec2::new` has many legitimate async
   callers elsewhere) — the fix is narrower: inline the pure sync body (`Self(kurbo::Vec2::new(x, y))`)
   directly in the trait impl instead of routing through the async wrapper, leaving `Vec2::new` itself
   untouched. Tagged `// 🚫️async: E1 ... inlined sync rather than routed through the async Vec2::new`.
6. **A trap worth naming for the next packet**: `Point`/`Vec2`/`CubicBez` all `Deref` to their `kurbo`
   inner type, and `geom_sel`'s functions read `.x`/`.y`/`.p0`/`.p1` etc as **plain field access through
   that Deref**, never as method calls — these are permanently sync (no fn call, no future, ever) and
   need no `.await` no matter how much the surrounding code is async. I verified this once
   (`impl Deref for Point`/`Vec2` target `kurbo::Point`/`kurbo::Vec2`, both have public `x`/`y` fields)
   rather than assuming from the dotted syntax that every `.x` was `Point::x()` the R9-tagged method.

## R9 check — both halves, shown

Only one R9 candidate this packet: `Point::x()`/`Point::y()` were already tagged sync by the prior
packet's pass (not mine to re-litigate), with evidence recorded there. I did **not** add any new R9 tags
— every other pure-looking helper I touched (`CubicBez::eval`, `Vec2::dot`, `Point::new`, `Rng::next_u64`,
etc.) has *only* async-capable consumers (other crate-internal async fns, or test bodies that are
themselves now `#[async_test]` async fns), so per R9 §3 the correct move was **hoisting `.await` at the
consumer**, not detagging the producer. The one exception (`From<(f64,f64)> for Vec2`, above) is E1 by
the trait signature itself, not by R9's transitive-consumer argument — I did not tag it `R9`, I tagged it
`E1` directly since `From::from`'s signature is externally fixed regardless of what its body calls.

## `#[test]` → `#[async_test]`

Both files' `#[cfg(test)] mod tests` (and `engine`'s second `mod algebra_tests`) had ~57 `#[test] async
fn` items — illegal since `#[test]` cannot run an async fn. `--lib` never compiles `#[cfg(test)]`, so
these were invisible until `--all-targets` (rule 26 payoff — this crate is the textbook case for "run
both"). Adopted the shared `semio_framework_async_macros::async_test` proc-macro (already used by
`semio-framework-math`, `semio-framework-async`, `semio-framework-replication`, etc. — same pattern,
fully-qualified attribute, no `use` needed). Added `semio-framework-async-macros` as a
`[dev-dependencies]` entry in this crate's own `📦️packages/🦀️rust/Cargo.toml` (owned path, not a
registrar-only file). Every test body then needed the same hand-hoisting as the lib code — rewrote both
test modules in full rather than patching line-by-line, since the mechanical tool cannot even see
`#[test]`-shaped errors (no diagnostic code) and had already produced a few internally-inconsistent
partial edits (e.g. one future awaited twice, once via the tool's edit and once left over) that were
easier to discard and rewrite clean than to patch.

One more bug the mechanical tool's partial edits could have hidden if trusted as-is: it left
`geom_sel::point_in_polygon`'s test calling `world_box_edges(box_)` un-awaited inside `.iter().any(...)`
in one intermediate state — caught by re-running the fixpoint tool after every hand batch, not by
assuming my own edit was already right.

## Files touched (all within owned scope)

- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📐️geometry/⚙️engine/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📐️geometry/🎲️random/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust/Cargo.toml`
  (added `[dev-dependencies] semio-framework-async-macros`)

No tool built (the shared `insert-await.py` sufficed for the mechanical layer; residue was all hand
edits, no new script needed — nothing new to save into the ticket folder on that front).

## Acceptance — every command run in the foreground this turn, `CARGO_TARGET_DIR` in scratchpad

```
$ CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/target-geom \
  cargo check -p semio-framework-geometry --lib
    Checking semio-framework-geometry v0.1.0 (.../📐️geometry/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 0.24s
EXIT_LIB:0
```

```
$ CARGO_TARGET_DIR=.../scratchpad/target-geom cargo check -p semio-framework-geometry --all-targets
    Checking semio-framework-geometry v0.1.0 (.../📐️geometry/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 0.32s
EXIT_ALL:0
```
Zero warnings on both (the one warning mid-session, `unused implementer of Future` on
`append_shape_to_path`'s `path.push(el.into())`, was fixed by adding the missing `.await` — that call
site is a plain `for` loop inside the async fn body, not behind the `with_shape_ref!` macro's closure-
looking-but-not-actually-a-closure syntax; verified by reading the macro's expansion, which substitutes
`$body` directly into a `match` arm with no closure boundary).

```
$ CARGO_TARGET_DIR=.../scratchpad/target-geom cargo test -p semio-framework-geometry
running 57 tests
... (57 lines, all "ok")
test result: ok. 57 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
Doc-tests semio_framework_geometry: 0 passed; 0 failed
EXIT_TEST:0
```
Named set: all 57 are new-to-this-packet passes (the crate had 0 runnable tests before — `#[test] async
fn` never compiled under `--all-targets`, so there is no prior baseline to diff against; this **is** the
baseline going forward). Split, re-verified by grouping the actual `cargo test` output by module path:
**12** in `engine::tests`, **15** in `engine::algebra_tests`, **30** in `random::tests` — 12 + 15 + 30 =
57, matching the harness's own "running 57 tests" / "57 passed" lines, and matching the 27 `#[test]`
sites counted in `engine/🦀️component.rs` (12 + 15) and 30 in `random/🦀️component.rs` before the
`#[test]` → `#[async_test]` swap.

```
$ CARGO_TARGET_DIR=.../scratchpad/target-geom cargo check -p semio-framework-math --lib
    Checking semio-framework-math v0.1.0 (.../🧮️math/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 3.14s
EXIT_MATH_LIB:0
```
(2 pre-existing `elided_lifetimes_in_paths` warnings from the `semio-framework-dispatch-macros`
*dependency* crate — not `semio-framework-math` itself, not touched by this packet, out of scope.)

```
$ CARGO_TARGET_DIR=.../scratchpad/target-geom cargo check -p semio-framework-math --all-targets
    Finished `dev` profile [unoptimized] target(s) in 2.17s
EXIT_MATH_ALL:0
```

```
$ CARGO_TARGET_DIR=.../scratchpad/target-geom cargo test -p semio-framework-math
test result: ok. 191 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
Doc-tests semio_framework_math: 0 passed; 0 failed
EXIT_MATH_TEST:0
```
**191 passed / 0 failed** — matches the packet's stated expected baseline exactly, now verified against
the **real** `semio-framework-geometry` crate rather than `math`'s hand-written stand-in. This is the
point of the packet: `math` is unblocked and independently confirmed.

## Summary

`semio-framework-geometry`: 61 (handoff) / 60 (fresh recount) `--lib` errors → 0. `--all-targets` (not
separately tracked at handoff, discovered here) went 201 → 0 once the `#[test]`/`#[async_test]` layer was
included. 57/57 tests pass. `semio-framework-math` verified green end-to-end against the real crate:
`--lib` 0, `--all-targets` 0, tests 191/0 matching the stated baseline exactly.
