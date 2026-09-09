# ➡️ Sweep kernel — prism orientation, and the two boolean examples

Ticket `26/09/09/PROCEDURAL-3D-END-TO-END`, lane `🔺️diff/➡️sweep` + `🔺️diff/🔀️boolean` +
`🔺️diff/✂️intersect` + the `generation3d` example-geometry harness. 2026-09-09, resumed 20:00.

Predecessors this file answers:

* `📓️example-geometry-tests-2026-09-09.md` §7.2 — three examples measured `−V/3` as a triangle soup
  while the B-Rep's own `measure.volume` read `+V`, located at `➡️sweep/🧮️core/🦀️.rs:271`, reported
  and deliberately not edited by that lane.
* `📓️example-geometry-tests-2026-09-09.md` §7.8 — the two boolean examples had stopped refusing but
  had not yet produced a measured volume.
* `📓️boolean-kernel-2026-09-09.md` §5 — the exact-imprint work that made them stop refusing.

---

## 1. Root cause — the lateral loop was written backwards, and `flipped` was patching the symptom

`build_prism` builds one lateral face per profile edge from the profile ("bottom") cap's loop. Two
independent facts have to be stated about each of those faces:

1. **Which way its loop runs in its OWN surface's `(u, v)`.** That decides which side of the
   boundary the trimmed region is on: an outer loop must be counter-clockwise, or the face is the
   complement of itself.
2. **Whether `du × dv` is the outward normal** (`Face::flipped`). That decides the rendered normal,
   the tessellated winding, and — since the boolean lane's `signed_tetra_sum` fix — the sign of the
   face's mass-property contribution.

The old code got (1) right only by accident and then used (2) to paper over the consequences.

**(1)** The loop's circuit was keyed off `f_i`, the sense in which the CAP happens to traverse the
profile edge:

```rust
let members = vec![(b_edge, !f_i), (left_rail, true), (t_edge, f_i), (right_rail, false)];
```

Every lateral surface this file builds has `u` increasing along the profile edge's own curve
direction (`translate_lateral` anchors the plane frame at `x = dir̂`; the cylinder's `u` is the
circle's own angle; the NURBS extrusion's `u` is the curve's own parameter) and `v` increasing along
the travel. So that circuit walks `u₁ → u₀`, up, `u₀ → u₁`, down whenever `f_i` is true — **clockwise
in `(u, v)`**. It is counter-clockwise only for `f_i == false`. `🧱️primitives::make_box`, the
hand-verified reference this file cites, only ever exercises `f_i == false` (its bottom ring is
`[(eb0, false), (eb3, false), (eb2, false), (eb1, false)]`), and a wire authored by
`make_polyline_wire` only ever produces `f_i == true` — so the two never met and the defect shipped
in every extrusion the DSL can build.

**(2)** With the circuit backwards, `flipped = false` gave the plane's `frame.z = dir̂ × travel` —
which §7.2 hand-verified as the CORRECT outward direction — but the loop's own 3D winding then
disagreed with it, and mass-properties' straight-edge fast path (`signed_tetra_sum`, which derives
its sign from that winding) came out at `−16` instead of `+16` for a unit test's box. The response
was to set

```rust
let lateral_flipped = matches!(lat.surface, Surface::Plane { .. });
```

— a per-surface-kind sign patch. It bought the right volume magnitude at the price of inverting the
geometric normal of every planar side wall: the preview lit them from inside, and the tessellated
soup of a watertight prism integrated to `−V/3` (only the `z = c` cap survives `⅓∮x·n dA`; the two
inverted max-normal side faces cancel the rest). Both numbers §7.2 measured — `4.0` for a `2×2×3`
box, `1.299038` for the hexagonal prism — are exactly `V/3`.

The patch has since become actively wrong in the other direction too: the boolean lane taught
`signed_tetra_sum` to honour `flipped`, so on today's tree the same `flipped = true` also makes
`brep.measure.volume` report `−V/3`.

### The derivation that replaces it

The profile loop is counter-clockwise about its own surface frame `Z_b` (the repository-wide `(u, v)`
convention, which `flipped` does not touch). For a counter-clockwise loop about `Z_b`, the
material-outward in-plane direction at a profile edge traversed in direction `d` is `d × Z_b`. The
cap correction immediately above makes `outward_bottom = −travel̂`, so `Z_b = ±travel̂` with the sign
given by `bottom.flipped`. Every lateral surface has `du × dv = dir̂ × travel̂`. Substituting
`d = f_i ? dir̂ : −dir̂` collapses all four cases to one line:

```rust
let lateral_flipped = f_i != bottom_flipped;
```

and the circuit becomes unconditional — the profile edge always walked `u₀ → u₁`, the rail at the
edge's END vertex up, the top edge back, the rail at its START vertex down:

```rust
let members = vec![(b_edge, true), (end_rail, true), (t_edge, false), (start_rail, false)];
```

Two independent checks confirm the pair rather than each other:

* **Against `make_box`.** For `f_i == false, bottom.flipped == false` the new code emits exactly
  `make_box`'s own `front_members` — same edges, same senses, same rails, same `flipped = false`.
* **Against shell coherence.** `check_shell_closure_and_orientation` requires the two coedges of a
  shared edge to differ in `forward XOR flipped`. Bottom cap: `f_i XOR flipped_b`. Lateral on that
  same edge: `true XOR (f_i XOR flipped_b)` — always its negation. Top cap and the top edge, and
  each rail against the neighbouring lateral (all four `f_i` combinations at a shared vertex, both
  the ordinary chain and the "two edges both end here" / "both start here" cases a mixed-sense
  profile ring produces) come out opposed as well. The topological rule and the geometric one now
  agree; before, they demanded different values of `flipped` and only the geometric one was met.

---

## 2. The diff

| File | Change |
|---|---|
| `…/🧊️brep/🧬️schema/🔺️diff/➡️sweep/🧮️core/🦀️.rs:241` | `let bottom_flipped = body.faces.get(bottom).unwrap().flipped;` — read AFTER the cap correction, before `transform_face`. |
| `…/➡️sweep/🧮️core/🦀️.rs:265` | rails are now keyed by the EDGE's start/end vertex (`if f_i { (s_bot, …) } else { (e_bot, …) }`), not by the cap coedge's traversal endpoints, so `start_rail`/`end_rail` name the `u₀`/`u₁` sides of the lateral surface. |
| `…/➡️sweep/🧮️core/🦀️.rs:281` | `let lateral_flipped = f_i != bottom_flipped;` replaces `matches!(lat.surface, Surface::Plane { .. })`. |
| `…/➡️sweep/🧮️core/🦀️.rs:292-293` | the unconditional counter-clockwise circuit and its p-curves (`start_pc` at `u₀`, `end_pc` at `u₁`); the `(u_s, u_e) = if f_i {…} else {…}` swap is gone. |
| `…/➡️sweep/🧮️core/🦀️.rs:186-215` | `build_prism`'s docstring now carries the derivation; the three inline comment blocks that recorded the sign patch and the old circuit are deleted (they documented a rule that no longer holds, and `CLAUDE.md` forbids comments inside definitions). |
| `…/➡️sweep/🌀️revolve/🦀️.rs:49` | cross-reference reworded — `revolve`'s own per-edge `flip` reads the same fact off the profile winding; its construction is untouched. |
| `…/🧊️brep/🧬️schema/💡️inferences/📏mass-properties/🦀️.rs` | new `pub fn loop_uv_signed_area` — the shoelace area of `loop_uv_polygon`'s own output. |
| `…/🧊️brep/🧬️schema/💡️inferences/✅validation-report/🧪️body/🦀️.rs` | new `check_face_loop_winding`, run from `validate_body`: outer loop `(u, v)` area must be positive, hole loops negative. Codes `outer-loop-winding-inverted` / `hole-loop-winding-inverted`. |
| `…/🧊️brep/🧬️schema/📸️snapshot/➰️curve/✂️curve-ops/🦀️.rs:37` | `arc_length` orders its endpoints. See §4. |
| `…/🧊️brep/🧪️tests/📦️extrude-orientation/🦀️.rs` (new) + `📦️packages/🦀️rust/Cargo.toml` | the `[[test]] brep_extrude_orientation` regression target, §3. |

### Why `validate_body` now catches a repeat

The old configuration was topologically coherent (`forward XOR flipped` opposed on every shared
edge) and integrated to the right magnitude, so neither `check_shell_closure_and_orientation` nor
`solid_volume` could see it. `check_face_loop_winding` sees it directly: the lateral loops were
clockwise in their own `(u, v)`, which is a statement about the trimmed REGION and cannot be
compensated by any choice of `flipped`.

Measured, not asserted — the old circuit and the old `flipped` were put back for one run
(`🗑️generated/sweep-negative-control.txt`, restored immediately afterwards) and the new regression
test failed exactly as intended:

```
rectangle ccw +Z: validate_body reported 4 issue(s): [
  "face-2:outer-loop-winding-inverted:outer loop's (u, v) signed area is negative (-6) …",
  "face-3:… negative (-4.5) …", "face-4:… negative (-6) …", "face-5:… negative (-4.5) …"]
```

— the four side walls of the `2 × 1.5 × 3` prism, by their own areas. On the fixed tree the check
finds **zero** issues across the 434 kernel tests of §6.3, both kernel-level booleans and all eight
examples, so it is not a tightening that had to be paid for elsewhere.

---

## 3. The regression target — `[[test]] brep_extrude_orientation`

`…/🧊️brep/🧪️tests/📦️extrude-orientation/🦀️.rs`, declared in the crate's `Cargo.toml` (emoji file
names never compile as an implicit `tests/` target). Twelve cases: **rectangle** and **regular
hexagon** profiles × **both windings** (which mirrors the profile plane's frame, moving the cap's
`flipped` to the other branch) × **both sweep directions** (which moves it back) — plus, per shape,
a profile whose loop traverses every one of its edges BACKWARDS, the `f_i == false` half of the
derivation that `make_box`'s side faces exercise and a freshly authored wire never does.

Each case asserts five things, none derivable from another:

1. `validate_body` silent — closed, 2-manifold, coherently oriented, non-degenerate, and now
   correctly wound in `(u, v)`.
2. every face's outer loop is counter-clockwise in its own surface frame (`Newell · frame.z > 0`);
3. every face's outward normal (`frame.z`, negated for `flipped`) points AWAY from the solid's
   centroid — measured from the geometry, never from the loop, and exact for these convex prisms;
4. the tessellated soup is closed, has **0** edges traversed the same way by both their triangles,
   and its SIGNED divergence volume equals the closed form within `1e-6` — positive;
5. `parry3d`'s `trimesh_signed_volume_and_center_of_mass` over that same soup agrees within `1e-4`
   (the soup is `f32`), so no number rests on our arithmetic alone.

```
running 12 tests
test hexagon_profile_with_backward_coedges_extruded_down_faces_outward ... ok
test hexagon_profile_with_backward_coedges_extruded_up_faces_outward ... ok
test hexagon_profile_wound_ccw_extruded_down_faces_outward ... ok
test hexagon_profile_wound_ccw_extruded_up_faces_outward ... ok
test hexagon_profile_wound_cw_extruded_down_faces_outward ... ok
test hexagon_profile_wound_cw_extruded_up_faces_outward ... ok
test rectangle_profile_with_backward_coedges_extruded_down_faces_outward ... ok
test rectangle_profile_with_backward_coedges_extruded_up_faces_outward ... ok
test rectangle_profile_wound_ccw_extruded_down_faces_outward ... ok
test rectangle_profile_wound_ccw_extruded_up_faces_outward ... ok
test rectangle_profile_wound_cw_extruded_down_faces_outward ... ok
test rectangle_profile_wound_cw_extruded_up_faces_outward ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

---

## 4. The two booleans — the kernel was already exact, the FIXTURES were wrong

`📓️example-geometry-tests-2026-09-09.md` §7.8 read the timestamps right: the harness's
`imprint point does not lie on the face's boundary loop` run predates the boolean lane's landing
(`🗑️generated/resume-example-geometry-all8.txt` 19:00 vs `🗑️generated/resumed-boolean-after.txt`
19:45). Re-run on the current tree in a private target, neither example refuses, and neither hangs:
`sphere-box-fuse` tessellates in **1 s**, `sphere-cut-with-torus` in **29 s** — §7.8's 61-minute
`refine_adaptive` is gone with the resumable tessellator that landed in the meantime. Both come out
`closed=true`, `boundary=0`, `nonManifold=0`, **`orientationDefects=0`**.

What was left was two authoring errors in the committed fixtures — numbers written while the
examples were blocked and therefore never once measured. Neither is a loosened assertion; both are
corrections of a stated expectation to what the geometry actually is, each confirmed by a
third-party quadrature and by the kernel's own exact `measure.volume`.

### 4.1 `🧲️sphere-box-fuse` — the union volume was derived for a CONCENTRIC cube

`brep.prim3d.box` spans `[0, size]³` from its own CORNER (`make_box`'s `corners[0]` is the origin),
and the DSL centres the ball on that corner. The committed `7.245424930456` is the union of a ball
with a cube CENTRED on it — and the same fixture's own committed bounding box,
`[-1.2, -1.2, -1.2] … [1.5, 1.5, 1.5]`, describes the corner placement. The fixture contradicted
itself; the bounding box was the half that was right.

With `radius = 1.2 < size = 1.5` the whole positive octant of the ball is inside the cube, so

`V = V_ball + size³ − V_ball/8 = 7.238229473870883 + 3.375 − 0.904778684233860 = 9.70845078963702`

confirmed to `4e-14` by an independent quadrature over the corner-clipped octant, and matched by the
kernel's `solid_volume` in `[[test]] brep_procedural_example_booleans`, which has asserted this same
closed form (to `5e-3` relative, cross-checked by `parry3d`) since the boolean lane landed. Fixture
`volume` and `volumeSource` corrected.

### 4.2 `🍩️sphere-cut-with-torus` — the bounding box ignored the groove the torus cuts

The tube reaches `major + minor = 2.5 > 2.2`, so the torus does not sit inside the ball: it carves a
groove right around the ball's equator. The difference's widest surviving circle is where the ball's
surface is exactly `minor` from the tube's centre circle,

`ρ_max = (ball² + major² − minor²) / (2·major) = (4.84 + 4 − 0.25) / 4 = 2.1475`

with only the poles still reaching `±2.2`. The committed box said `±2.2` on all three axes. The
kernel's tessellated extent is `±2.14750004` in `x`/`y` and `±2.2000000` in `z` — the analytic value
to `4e-8`, and identical at two different deflections, which is what identifies it as the solid's
real extent rather than a sampling artefact. Fixture bounding box corrected on `x` and `y`.

The committed VOLUME needed no change: `brep.measure.volume` returns `37.84114575338816` against the
committed `37.841142613316`, agreeing to **3.1e-6**.

### 4.3 Deflection, not tolerance

Both boolean fixtures asked for a `0.01` tessellation deflection, at which a chordal mesh of a
`r = 2.2` sphere is `0.18` short in volume — outside the committed `0.05`. Rather than widen a
tolerance, the fixtures now ask for `0.0025`, at which the same committed tolerances hold with room
to spare (fuse: volume short by `0.0155` of `0.02`, box off by `0.001` of `0.005`; cut: volume short
by `0.041` of `0.05`, box off by `0.0024` of `0.005`). Cost: 6 128 and 19 208 triangles, 1 s and
29 s. **No tolerance in any of the eight fixtures was widened.**

### 4.4 One assertion was tightened

`kernelVolumeTolerance` is new in the fixture schema (`Expectation`, the TypeScript twin's type and
its `assertFixtureContract`, and all eight `🔣️.json`). The harness used to hold
`brep.measure.volume` — an EXACT analytic number — to the same tolerance as the tessellated soup, a
discretisation-bounded one. They are different physical quantities and now carry different
tolerances: `1e-9` for `📦️rectangle-extrude-volume` and `1e-5` for `🍩️sphere-cut-with-torus`, both
met; `null` for the six examples whose DSL exposes no measure node, which the contract requires to
match `kernelVolumeNode`'s own nullity in both lanes.

### 4.5 `🐚️box-shell-preview` — a negative arc length, found on the way

Not a boolean, and not previously failing: the tessellation lane's new validate-before-tessellate
gate turned a latent `curve_ops::arc_length` defect into a hard refusal
(`[degenerate-edge] edge-15: edge length -1.6000000000004002 is below its own tolerance`). The
adaptive quadrature returns the NEGATIVE of the length when read with `t0 > t1`, which the shell's
inner cavity edges are, and every caller compares it against a positive tolerance — so those edges
were both flagged degenerate AND accepted by `is_point_edge`, which then exempted them from the
shell-closure rule entirely. `arc_length` now orders its endpoints; a length is a magnitude. The
example is green again and the closure rule now actually covers those six edges.

---

## 5. Per-example results

`cargo test -p semio-s-artifact-procedural-generation3d --test example-geometry` in a private
`CARGO_TARGET_DIR` (`…/scratchpad/target-sweep`, APFS-cloned from `target/debug`), `RUSTC_WRAPPER=""`,
`--test-threads=1`. Raw logs: `🗑️generated/sweep-example-*.txt`.

| Example | Result | tri | closed | bnd / n-mf / **orient** | signed soup volume | `parry3d` | kernel `measure.volume` | expected | fixture `kernelStatus` |
|---|---|---|---|---|---|---|---|---|---|
| 🪢️ rectangle-wire-preview | **pass** | 0 | false (wire) | 0 / 0 / 0 | — (edge length `7.000000000`) | — | — | perimeter 7.0 | `green` |
| 📦️ rectangle-extrude-volume | **pass** | 12 | true | 0 / 0 / **0** | **+12.000000000** | 12.000000000 | **12.0** | 12.0 | `green` (was `blocked-on-extrude-orientation`) |
| 🧹️ face-sweep-extrude | **pass** | 12 | true | 0 / 0 / **0** | **+12.000000000** | 12.000000000 | — | 12.0 | `green` (was `blocked-on-extrude-orientation`) |
| 🍄️ hexagonal-mushroom-column | **pass** | 20 | true | 0 / 0 / **0** | **+3.897114247** | 3.897113562 | — | 3.897114317030 | `green` (was `blocked-on-extrude-orientation`) |
| 🐚️ box-shell-preview | **pass** | 24 | true | 0 / 0 / 0 | +3.904000389 | 3.903999567 | — | 3.904 | `green` |
| 📐️ box-fillet-preview | **fail — blocked-on-fillet-kernel** (blend lane) | — | — | — | — | — | — | 7.888634924 | `blocked-on-fillet-kernel` |
| 🧲️ sphere-box-fuse | **pass** | 6128 | true | 0 / 0 / **0** | **+9.692918877** | 9.692929268 | — | 9.708450790 | `green` (was `blocked-on-boolean-kernel`) |
| 🍩️ sphere-cut-with-torus | **pass** | 19208 | true | 0 / 0 / **0** | **+37.800080999** | 37.800052643 | **37.84114575** | 37.841142613 | `green` (was `blocked-on-boolean-kernel`) |

Every signed volume is POSITIVE; every closed soup has zero orientation defects. The three extrude
rows previously read `−4.000000000`, `−4.000000000` and `−1.299038082` with 8, 8 and 12 defects.

`📐️box-fillet-preview` is the blend lane's, skipped per this lane's brief and re-run once at the
end. It no longer spends 820 s in `fit_pcurve`; it now refuses in 12.6 s inside that lane's
in-flight rewrite:

```
node "fillet" failed to evaluate: invalid input: operation failed: operation failed:
  blend: no p-curve within tolerance exists for this surface/curve pair
```

Its fixture keeps `kernelStatus: "blocked-on-fillet-kernel"` and its committed numbers are untouched.
`blocked-on-extrude-orientation` and `blocked-on-boolean-kernel` are removed from the `KERNEL_STATUSES`
vocabulary in both lanes and from the `🥒️.feature` prose — a standing nothing declares is dead
vocabulary, and this repository keeps no legacy values.

---

## 6. Runs

### 6.1 Kernel `[[test]]` targets

```
cargo test -p semio-s-artifact-stdio-semio --test brep_extrude_orientation \
                                           --test brep_procedural_example_booleans
```

```
running 12 tests … test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
running 2 tests
test sphere_box_fuse_example_is_a_closed_oriented_solid ... ok
test sphere_cut_with_torus_example_is_a_closed_oriented_solid ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 61.67s
```

### 6.2 The example lane

```
running 7 tests … test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 36.03s
running 1 test  … test result: FAILED. 0 passed; 1 failed … finished in 12.64s   (box-fillet-preview, §5)
```

### 6.3 Whole-kernel suite

The ticket's `🔬️harness` mounts every kernel source at its real module path and runs each subset's
own `#[cfg(test)]` suite. It is the only way to run them at all today: the crate's `--lib` test
target does not compile, on three errors in a PEER's in-flight `📦️object` subset
(`FaultCode: Display`, `Vec<u8>::expect`) that no file in this lane touches.

Baseline before this lane's edits: `441 passed; 0 failed; 1 ignored` in 1158.77 s (one runtime bomb
skipped, `offset_solid_box_round_matches_minkowski`, the open item `📓️boolean-kernel` §5.6 owns).

After (`🗑️generated/sweep-harness-after.txt`):

```
test result: ok. 434 passed; 0 failed; 1 ignored; 0 measured; 16 filtered out; finished in 256.48s
```

`➡️sweep`'s own 13 tests all pass — including `extrude_rectangle_with_hole_has_ten_faces` (inner
loops get the same derivation and their tube faces come out facing into the hole),
`sweep_circle_along_line_is_a_cylinder`, the three `revolve` cases and `loft`/`pipe`/`helical`. **No
test anywhere reports `outer-loop-winding-inverted` or `hole-loop-winding-inverted`.**

The 16 filtered are the `🎨️blend` family plus the minkowski bomb and the probes. Blend is skipped
deliberately: its own new tests fail and one of them (`fillet_cylinder_cap_matches_pappus_closed_form`)
did not return in 18 minutes, all inside the blend lane's in-flight rewrite — the same rewrite that
left the crate uncompilable for four minutes at 20:38 and produces §5's fillet refusal. Its run is
kept at `🗑️generated/sweep-harness-with-blend.txt` for that lane; nothing in it names this one.

### 6.4 Compilation

```
CARGO_TARGET_DIR=…/target-sweep RUSTC_WRAPPER="" cargo check -p semio-s-artifact-stdio-semio --lib --keep-going
    Checking semio-s-artifact-stdio-semio v0.1.0 (…)
    Finished `dev` profile [unoptimized] target(s) in 22.93s
```

**0 errors, 0 warnings** (the crate genuinely recompiled — `Checking semio-s-artifact-stdio-semio`
is in the log, not a cache hit).

```
CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check -p semio-s-artifact-stdio-semio --lib --keep-going \
    --target wasm32-wasip2 --profile wasm-dev
```

```
    Checking semio-s-artifact-stdio-semio v0.1.0 (…)
    Finished `wasm-dev` profile [unoptimized] target(s) in 2m 01s
```

**0 errors, 0 warnings**, again with the crate genuinely re-checked. Its `wasm32-wasip2` artifacts
were seeded into the private target by APFS clone from the shared `target/wasm32-wasip2`; the shared
`target/` was never written to by this lane. Raw: `🗑️generated/sweep-wasm-check.txt`,
`🗑️generated/sweep-native-check.txt`.

### 6.5 The other two lanes of this example family

```
bun ./📜️script.ts test        (✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript)
 35 pass / 0 fail / 255 expect() calls across 11 files
```

The TypeScript twin recomputes both boolean volumes from the DSL's own sliders with its own
composite-Simpson rule; its `sphere-box-fuse` quadrature carried the same concentric-cube error as
the fixture and is corrected to the corner placement, and its `sphere-cut-with-torus` bounding box
now derives `ρ_max` rather than assuming the whole ball. Assertion count 238 → 255.

```
./.venv/bin/python3 "…/🧪️tests/📐️example-geometry-3d-1/🐍️.py"
PASS: numpy 2.5.0 + scipy agree with all 23 committed expectations across 8 examples
```

`ball_cube_union` rewritten for the corner placement (`scipy.integrate.quad` over the clipped
positive octant) and the cut example's extent derived from the groove. The oracle agreed with the
old fixture only because both encoded the same wrong model on the volume while both encoded the
right one on the extent — the internal contradiction §4.1 names.

---

## 7. Files changed

Kernel (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/`):

* `🧬️schema/🔺️diff/➡️sweep/🧮️core/🦀️.rs` — the fix (§1, §2).
* `🧬️schema/🔺️diff/➡️sweep/🌀️revolve/🦀️.rs` — cross-reference only.
* `🧬️schema/💡️inferences/📏mass-properties/🦀️.rs` — `loop_uv_signed_area`.
* `🧬️schema/💡️inferences/✅validation-report/🧪️body/🦀️.rs` — `check_face_loop_winding`.
* `🧬️schema/📸️snapshot/➰️curve/✂️curve-ops/🦀️.rs` — `arc_length` orders its endpoints.
* `🧪️tests/📦️extrude-orientation/🦀️.rs` — new `[[test]]` target.
* `📦️packages/🦀️rust/Cargo.toml` (crate root) — `[[test]] brep_extrude_orientation`.

Examples (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/`):

* `📚️examples/🧪️tests/🧩️geometry/🦀️.rs` — `kernel_volume_tolerance`, trimmed `KERNEL_STATUSES`.
* `📚️examples/🧪️tests/🧩️geometry/🟦️.ts` — the same, plus the contract check on the new field.
* all eight `📚️examples/<example>/🧪️tests/🧩️example/🔣️.json` — `kernelVolumeTolerance`;
  `kernelStatus` `green` on five; corrected `volume`/`volumeSource` on `🧲️sphere-box-fuse`,
  corrected bounding box on `🍩️sphere-cut-with-torus`, `tessellationTolerance` `0.01 → 0.0025` on
  both booleans.
* `📚️examples/🧲️sphere-box-fuse/🧪️tests/🧩️example/🟦️.ts` — union quadrature corrected to the corner
  placement, `kernelStatus` `green`.
* `📚️examples/🍩️sphere-cut-with-torus/🧪️tests/🧩️example/🟦️.ts` — bounding box derived from the
  groove, `kernelStatus` `green`, `kernelVolumeTolerance` asserted `≤ 1e-5`. (The other six
  per-example `🟦️.ts` files assert no `kernelStatus` and needed no edit.)
* `🧪️tests/📐️example-geometry-3d-1/🐍️.py` — corrected `ball_cube_union` and cut extent.
* `🧪️tests/📐️example-geometry-3d-1/🥒️.feature` — statuses and the ⚠️ block.

Launchers:

* `.vscode/🧩️launch.seed.jsonc` and the rendered `.vscode/launch.json` — `🧪️test📦️brep🧊️extrude-orientation`,
  placed beside the sibling `🧪️test⏱️brep🧊️tessellation-jobs` entry it follows in grouping and naming.
