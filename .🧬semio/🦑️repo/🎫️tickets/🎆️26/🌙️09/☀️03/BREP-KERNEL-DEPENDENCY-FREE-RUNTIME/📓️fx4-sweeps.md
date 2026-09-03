# 📓️ FX-4 — `diff::sweep::tests::*` fixes — ALL GREEN

Fixer FX-4 on `BREP-KERNEL-DEPENDENCY-FREE-RUNTIME`. Files owned (per instruction): `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/➡️sweep/**` ONLY.

## Target: 5 failing tests

`extrude_rectangle_matches_box_topology_and_volume`, `loft_two_rectangles_is_ruled`, `revolve_annulus_full_turn_is_analytic_and_exact_volume`, `revolve_circle_makes_a_torus`, `sweep_circle_along_line_is_a_cylinder`.

## Final result

**All 5 pass.** Final verbatim run (`cd harness && bun ./📜️script.ts sync && RUSTC_WRAPPER="" cargo test -- sweep`), reproduced twice:

```
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 437 filtered out; finished in 3.71s
```

(14 = every test in the `sweep` module, including 2 new regression tests I added, plus one unrelated `boolean::` test that happens to match the `sweep` filter substring.)

**6 real bugs found and fixed, all in `➡️sweep/**`.** Two more (mass-properties bugs A/B/C) were out of my file ownership and were fixed by FX-5 concurrently (`📓️fx5-inferences.md`); their fix made `loft_two_rectangles_is_ruled` pass and got the others most of the way there, but 3 of the 5 target tests still failed after FX-5's landing — those pointed back INSIDE `➡️sweep/🦀️.rs`/`🧮️core/🦀️.rs`/`🌀️revolve/🦀️.rs` themselves, and the coordinator asked me to fix them rather than defer. All were genuine, previously-undiscovered bugs in code this wave (W2-C) wrote blind, never compiled before this ticket.

## Progression this session

7 passed/5 failed (start) → 8/4 (my `revolve.rs` classify()+pcurve-slope fixes, bugs 1-2) → 10/2 → 12/2 (FX-5 landing mass-properties bugs A and C) → 13/1 (my `core.rs` top-cap-orientation fix, bug 3) → 14/1→ then regressed to 13/1 when I ALSO fixed the lateral-Plane-flipped bug (bug 4, this uncovered `extrude_rectangle`'s pre-existing double-bug-cancellation) → **14/0** (bug 4 landed correctly) → then `revolve_annulus` still failed after FX-5's Bug B fix landed fully → **all 14/14 green** after my `revolve.rs` annulus-rail-winding fix (bug 5).

## Bugs fixed (all in `➡️sweep/**`, in scope)

### 1 — `🌀️revolve/🦀️.rs` `classify()`: wrong operator for "circle's plane contains the axis"

Used `frame.z.cross(axis)` (tests PARALLEL) where `frame.z.dot(axis)` (tests PERPENDICULAR) was needed — rejected every valid torus-producing circle profile with a false `Operation` error.

### 2 — `🌀️revolve/🦀️.rs` `lateral_pcurves()`: pcurve slope assumed a `(0,1)` domain

`start_pc`/`end_pc` (both `RevSurface` branches) used the endpoint delta `v1 - v0` as `dir`, combined with the edge's RAW (non-`(0,1)`) `t`-domain as `prange` — correct only when that domain happened to be `(0,1)`. Fixed by deriving the true per-unit-`t` slope from samples at `t=0`/`t=1` (both closures are affine by construction).

### 3 — `🧮️core/🦀️.rs` `build_prism`: `top` cap inherited `bottom`'s orientation verbatim

`transform_face` (`copy_face`) correctly preserves `flipped` under any non-reflecting map (every rigid placement here) — that's right for a plain copy, but `top` caps the solid on the OPPOSITE side from `bottom`, so its outward direction must be `bottom`'s opposite, even though both share the identical local frame. Fixed: `top.flipped = !top.flipped` right after `transform_face`. Invisible before because `solid_volume` takes `.abs()`, masking a globally-consistent-but-inverted sign whenever every OTHER face happened to share the same accidental inversion (see bug 4).

### 4 — `🧮️core/🦀️.rs` `build_prism`: `Curve3::Line` laterals need `flipped = true`, not hardcoded `false`

Every OTHER lateral surface kind (`Cylinder`/`Cone`/`Torus`/NURBS) is independently correct with `flipped = false` (`du × dv` naturally outward for this file's coedge order) — but a straight-edge `Surface::Plane` lateral routes through mass-properties' `signed_tetra_sum` fast path instead, whose sign depends on LOOP VERTEX WINDING, not `frame.z`/`du × dv` at all, and empirically comes out backward for this same coedge order (confirmed: hand-verified all 4 of `extrude_rectangle`'s side faces' `frame.z` was independently correct, yet their combined `signed_tetra_sum` was `-16` instead of `+16`). Fixed: `flipped = matches!(lat.surface, Surface::Plane{..})`.

Bugs 3+4 combined are what actually made `extrude_rectangle_matches_box_topology_and_volume` correct (24) — before either fix the test happened to "pass" only by relying on TWO compounding sign errors (top-cap-inherits-bottom, straight-lateral-backward) that, by coincidence, produced a globally-inverted-but-magnitude-correct `-24`; fixing bug 3 alone broke it (exposed bug 4), and both together fixed it for real.

### 5 — `🌀️revolve/🦀️.rs` `lateral_pcurves()` PlanarAnnulus branch: inner/outer rails traced the SAME angular sense

`revolve_full`'s annulus-cap construction represents an annulus (multiply-connected: outer circle + zero-width bridge + inner circle, per this module's single-seam pattern) as ONE simple polygon for ear-clipping — a standard technique, but it requires the outer boundary CCW and the inner (hole) boundary CW so the net shoelace area comes out `outer − inner`. The coedge `forward` flags (topology-load-bearing, shared via `orbit_cache` with adjacent lateral faces — never safe to touch) gave `left_pc` (inner) CCW and `right_pc` (outer) CW — backward, and NOT simply opposite. Confirmed: each annulus cap's own `face_area` came out `~10.1`/`~10.4` instead of `π·(3²−2²) ≈ 15.71`. Fixed purely in the PCURVE's own `prange` ordering (reversed both `left_pc`'s and `right_pc`'s domain, `(base+angle, base)` instead of `(base, base+angle)`) — no topology touched. Area corrected to `~15.6`/`~15.63`; solid volume from `~69.6` to the exact `78.54`.

### 6 — `➡️sweep/🦀️.rs`: regression tests added (per coordinator instruction)

`extrude_rectangle_pcurves_are_consistent` and `extrude_circle_pcurves_are_consistent`, plus a reusable `assert_pcurves_consistent(body, solid)` helper checking `surface.eval(pcurve.eval(p)) ≈ curve3.eval(t)` (the same invariant `validate_body`'s own `check_same_parameter` enforces) for every coedge of every face — both pass, and this is what originally surfaced bugs 3/4 (adding these checks to the 5 target tests, before I'd found bugs 3/4, immediately broke the previously-"passing"-by-luck `extrude_rectangle`/`extrude_circle` tests with `same-parameter-violated`/wrong-sign findings).

## Mass-properties bugs (💡️inferences/📏mass-properties/🦀️.rs — NOT my file, fixed by FX-5 concurrently, not touched by me)

For completeness/traceability (see `📓️fx5-inferences.md` for FX-5's own account): Bug A (`loop_volume_moments`'s `Surface::Plane` fast path ignored `face.flipped`), Bug B (`loop_positions` sampled only each coedge's start vertex, degenerate for curved-boundary `Plane` loops — fixed via a `loop_has_only_straight_edges` guard routing curved loops to the general `loop_uv_polygon` path instead), Bug C (`coedge_uv_sample`'s pcurve branch never reversed sampling for `forward == false`, unlike its own 3D-curve fallback three lines below it).

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/➡️sweep/🌀️revolve/🦀️.rs` — bugs 1, 2, 5.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/➡️sweep/🧮️core/🦀️.rs` — bugs 3, 4 (also carries the earlier `translate_lateral`/`bottom_pc`/`top_pc`/`left_pc`/`right_pc` slope generalization from the same session, needed for bug 2's sibling case).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/➡️sweep/🦀️.rs` — bug 6 (2 new regression tests + helper).

No file outside `➡️sweep/**` was touched. `💡️inferences/📏mass-properties/🦀️.rs` (FX-5's file) was read-only for diagnosis (temporary `eprintln!` instrumentation added to MY OWN test bodies, always removed before the next commit-worthy state — none remain in the final files).

## Verification commands actually run (foreground, this session)

```
cd harness && bun ./📜️script.ts sync && RUSTC_WRAPPER="" cargo test -- sweep 2>&1 | tail -N   # run to green, repeated ~15 times through the session
cd harness && bun ./📜️script.ts sync && RUSTC_WRAPPER="" cargo test -- <single_test> --nocapture 2>&1 | tail -N   # per-bug diagnosis, temporary eprintln! instrumentation
```

One `-- sweep` run hit a transient, unrelated foreign-session directory-rename race (`semio-framework-mesh-engine`'s `Cargo.toml` `path=` pointing at a mid-rename directory) — resolved itself after a few retries with the `sync` step, per this ticket's own documented pattern (`📓️w2c-sweeps.md`/`📓️h0-harness.md`), not touched by me. A full unfiltered `cargo test` was attempted once for a broader regression check but abandoned (explicitly stopped, never left running) — it queued behind heavy, unrelated concurrent cargo/rustc activity from other tickets' sessions on this shared machine; the filtered `-- sweep` run above is the relevant, complete regression signal for the module I own and was run to green multiple times.
