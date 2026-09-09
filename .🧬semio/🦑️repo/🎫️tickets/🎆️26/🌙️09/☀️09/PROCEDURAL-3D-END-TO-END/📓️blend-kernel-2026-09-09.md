# Blend Kernel — Analytic Fillets and Chamfers (2026-09-09)

Scope: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/🎨️blend/**`,
the `↔️offset` call site that delegates its `Round` corners to it, and `⚙️engine/🔖️contract`'s
`OPERATION_QUALITY` tags for fillet/chamfer. Closes the one item
`📓️boolean-kernel-2026-09-09.md` §5.6 left open.

## 1. State found on disk

`git diff HEAD --stat` over the blend directory, at the start of this lane:

```
🧬️schema/🔺️diff/🎨️blend/🦀️.rs                | 1577 +++++++++++++++-----
🧬️schema/🔺️diff/🎨️blend/🧪️tests/🔬️unit/🦀️.rs |  229 ++-
2 files changed, 1412 insertions(+), 394 deletions(-)
```

`git log --date=iso -1` on the file returns `025ec86a42`, **2026-09-08 20:58:18 +0200** — HEAD
predates this ticket, so the 1382-line working-tree file and its 246-line test module are entirely
this ticket's uncommitted work (a previous lane's, last written 21:45). Read top to bottom, the
rewrite is **coherent and complete**, not a torso: every construction the goal names is present and
reachable (`cylinder_patch`, `cap_patch`'s torus, `corner_surface`'s sphere, `chamfer_plane_patch`,
`cone_fillet_patch`), the module compiles, and its own ten unit tests pass. **Decision: finish it,
do not restore HEAD.** The 491-line HEAD version is the NURBS-patch engine §5.6 diagnosed.

Its one defect was an unused `Curve2Id` import (`cargo check --lib`: 0 errors, 1 warning).

Two unrelated blockers had to be cleared before ANY test in this crate could run — the lib TEST
binary did not compile, from a peer's committed API changes landing under the `📦️object` subset
(three test-only call sites: `FaultCode` lost its `Display`, `encode_pack` stopped returning a
`Result`). Fixed in place, test-side only, no API change. See §7.

## 2. Design — what makes the patches exact

The whole point of the rewrite is that **no p-curve is ever fitted**. Every blend patch is carried
on an analytic support whose boundaries are its own ISO-LINES, so each p-curve is a straight,
affinely parametrized line in `(u, v)` derived in closed form and then VERIFIED against its own 3D
curve at 33 stations before it is accepted (`affine_uv_pcurve`, `🎨️blend/🦀️.rs:241`; the check is
`pcurve_deviation`, `:288`, at `BLEND_EPS = 1e-10`).

| Edge pair | Support | Tangency curves | End trims | Built by |
| --- | --- | --- | --- | --- |
| plane / plane, constant `r` | `Surface::Cylinder` about the line at distance `r` inside both | its two `u = const` rulings | cross-section arcs (`v = const`) | `cylinder_patch` `:626` |
| plane / plane, chamfer | `Surface::Plane` through the two cut lines | `u = 0` and `u = \|A₁−A₀\|` | straight cut lines | `chamfer_plane_patch` `:642` |
| plane / plane, variable `r` | `Surface::Cone` of revolution, `sin(half_angle) = dr/ds` | two of its rulings | the exact conic its oblique cap plane cuts (`conic_end_trim` `:919`) | `cone_fillet_patch` `:662` |
| planar cap / coaxial cylinder, constant `r` | **`Surface::Torus`**, tube radius `r`, major `R−r` | `v = 0` (cylinder) and `v = π/2` (cap) | one seam curve, used twice | `cap_patch` `:702` |
| planar cap / coaxial cylinder, chamfer | `Surface::Cone` frustum | two `v = const` circles | the seam | `cap_patch` `:702` |
| fully blended trihedral vertex, fillet | **`Surface::Sphere`** of radius `r` at the resting ball centre | — | three GREAT circles: one equator, two meridians | `corner_surface` `:878` |
| fully blended trihedral vertex, chamfer | `Surface::Plane` through the three corner points | — | three straight lines | `corner_surface` `:878` |

`Surface` does have a `Torus` variant, so **cylinder–plane edges are IN scope and implemented**, not
documented as out of it. The torus's frame is pinned with `z` on the cap normal and `x` on the seam
radial, which puts both tangency circles on `v = const` and the seam on `u = 0 / u = 2π`.

The sphere's frame is pinned with `z` on the one face normal perpendicular to the other two, which
puts that face's corner at the pole and makes all three boundaries iso-lines. The three end circles
are great circles *because* the resting ball touches all three faces from one centre — a fact the
code re-derives and refuses if it does not hold (`:889`, "the three corner points do not share one
rolling-ball centre").

Exactly ONE p-curve in the engine is interpolated rather than derived: a variable fillet's
elliptical end trim on its own cone, whose image in the cone's `(u, v)` is transcendental in the
conic's parameter. `blend_pcurve` `:347` reaches it only for the literal `(Cone, Nurbs)` pair, and
`interpolated_uv_pcurve` `:379` densifies until the deviation certifies, refusing above
`FIT_EPS = 1e-8`. Everything else is a hard `Err`, never an approximation.

Every precondition is an explicit refusal, not a silent fallback: a blended edge must be convex and
shared by exactly two faces (`:598`, `:548`); a vertex a blend reaches must be trihedral with either
exactly one or ALL of its edges blended (`:800`, `:875`); the face closing a partially blended vertex
must be planar (`:218`); the dihedral may not be too flat for a rolling ball (`:629`).

## 3. Closed forms

### 3.1 Rounded box (all 12 edges, one radius)

By Steiner's formula the filleted box IS the Minkowski sum of the INNER box
`x × y × z = (a−2r)(b−2r)(c−2r)` with a ball of radius `r`:

```
V = xyz + 2r(xy + yz + zx) + πr²(x + y + z) + (4/3)πr³
```

Stated as a subtraction from the sharp box — the form the ticket asked for — expand
`a³ = (a−2r)³ + 6r(a−2r)² + 12r²(a−2r) + 8r³` against it. The `6r(a−2r)²` slab term cancels
exactly, `12r²(a−2r) − 3πr²(a−2r)` is the twelve edge slivers, and `8r³ − (4/3)πr³` is the eight
corners:

```
V = a³ − 12(1 − π/4)·r²·(a − 2r) − 8(1 − π/6)·r³
```

**The ticket's stated corner term `8(1 − π/3)r³` is wrong.** The eight corner deficits total
`8r³ − (4/3)πr³ = 8r³(1 − π/6)`, because the eight sphere octants together are ONE ball
(`(4/3)πr³`), i.e. `(π/6)r³` each — not `(π/3)r³`. Both statements are asserted to agree to `1e-12`
in `the_two_rounded_cube_closed_forms_agree_and_the_kernel_meets_both`, and the kernel is then held
to the subtraction form, so neither can drift.

### 3.2 Chamfered box (all 12 edges, symmetric `d`)

```
V = abc − 2d²(a + b + c) + (16/3)d³
```

Twelve triangular prisms of section `d²/2` over the FULL edge lengths (`(d²/2)·4(a+b+c)`), corrected
at the eight corners where three of them overlap and the corner plane cuts beyond them.

### 3.3 Partial selections

* one edge, fillet: `abc − L·r²(1 − π/4)` — the sliver runs the full edge, since both end vertices
  stay unblended and trihedral (7 faces: 6 originals + 1 cylinder, no corner patch);
* one edge, asymmetric chamfer: `abc − ½·d₀·d₁·L` (7 faces);
* the four edges parallel to `Z`, fillet: a prism of height `c` on a rounded rectangle,
  `((a−2r)(b−2r) + 2r[(a−2r)+(b−2r)] + πr²)·c` — 10 faces, and the selection whose eight end
  vertices each keep exactly ONE blended edge, so every one of them takes the `cap`-face branch
  rather than a corner patch;
* cylinder cap, fillet: Pappus on the removed meridian section,
  `2π[r²(R − r/2) − (πr²/4)(R − r + 4r/3π)]`;
* cylinder cap, chamfer: `2π·(d²/2)(R − d/3)`.

## 4. Diff

| File | Change |
| --- | --- |
| `🧊️brep/🧬️schema/🔺️diff/🎨️blend/🦀️.rs:37` | dropped the unused `Curve2Id` import — the partial rewrite's only defect; the crate now checks warning-free. |
| `🧊️brep/🧬️schema/⚙️engine/🔖️contract/🦀️.rs:301-316` | `fillet`/`fillet_edges`/`chamfer`/`chamfer_asymmetric`/`chamfer_edges` → **`OpQuality::ExactAnalytic`** (was `ExactNumericalWithinTolerance`), with the rationale in place: analytic supports, iso-line trims, closed-form verified p-curves, no sampling anywhere in the construction. `fillet_variable` deliberately stays `ExactNumericalWithinTolerance` — its cone's oblique cap trim is the one certified-`1e-8` interpolation. |
| `🧊️brep/🧪️tests/🎨️analytic-blend/🦀️.rs` | **NEW**, 281 lines, 7 tests — the `[[test]]` lane the ticket asked for, driven from the crate's PUBLIC surface. |
| `🧿️semio/📦️packages/🦀️rust/Cargo.toml` (tail) | **NEW** `[[test]] name = "brep_analytic_blend"` with its explicit ASCII name and package-root-relative path (emoji file names never compile as an implicit `tests/` target). |
| `.vscode/🧩️launch.seed.jsonc` + `.vscode/launch.json` | **NEW** `🧪️test🎨️brep🧊️analytic-blend` entry, placed and named by the existing `🧪️test📦️brep🧊️extrude-orientation` / `🧪️test⏱️brep🧊️tessellation-jobs` grouping — devs run this lane from `launch.json`, never the CLI. |
| `🌀️procedural/…/📚️examples/🧪️tests/🧩️geometry/🦀️.rs:75-78` | `KERNEL_STATUSES` shrunk `[&str; 2] → [&str; 1]`: `blocked-on-fillet-kernel` is gone with the defect it named, as that constant's own docstring requires ("a standing nothing declares is dead vocabulary"). |
| `🌀️procedural/…/📚️examples/📐️box-fillet-preview/🧪️tests/🧩️example/🔣️.json` | `kernelStatus` `blocked-on-fillet-kernel → green`; `tessellationTolerance` `0.01 → 0.0005`; `volumeSource` extended with the reason. See §5. |

## 5. Why `📐️box-fillet-preview`'s chord deflection moved, and its expectation did not

With the analytic engine the example immediately produced a **closed, coherently wound** solid
(`closed=true boundary=0 nonManifold=0 orientationDefects=0`, bbox exactly `[0,0,0]–[2,2,2]`) — the
`blocked-on-fillet-kernel` defect is gone. At the fixture's original `tessellationTolerance = 0.01`
it still failed the volume check: 7.871361 against the analytic 7.888635, a 0.017273 deficit past
the 0.005 tolerance, with `parry3d` reading 7.871352 — i.e. **both integrators agreed the MESH is
7.8714**. That is not a kernel error, it is the mesh: the harness compares the TESSELLATED volume to
the analytic one, and an inscribed polygonisation of a curved solid always under-measures.

The deficit is predictable in closed form. A quarter-cylinder of radius `r` split into `n` chords of
`θ = (π/2)/n` loses `n·(r²/2)(θ − sin θ)` of area per unit length. At deflection `0.01` on `r = 0.15`
the sagitta bound `r(1 − cos(θ/2)) ≤ 0.01` allows `n = 3`, `θ = 0.5236`, giving
`3·0.01125·(0.5236 − 0.5) = 7.97e-4` per unit length; over 12 edges of length 1.7 that is **0.01625**,
plus the eight sphere octants — the measured 0.01727. At `0.0005`, `n = 10`, `θ = 0.15708`, the same
arithmetic gives ≈ 0.0015.

So the fixture's **expected volume is untouched** (still the exact analytic 7.888634923940582) and
its **tolerance is untouched** (still 0.005); only the deflection at which the mesh is asked to
represent that solid was tightened, to the one value at which a curved example can meet a 0.005
tolerance at all. `📐️box-fillet-preview` is the only curved example in the set — every other one is
polyhedral, which is why 0.01 was enough for them. Measured after: **7.887102** (ours) / **7.887084**
(parry3d) against 7.888635, Δ = 0.00153.

## 6. Results

### 6.1 Before

Baseline at HEAD's blend engine, as recorded in `📓️boolean-kernel-2026-09-09.md` §5.6: the offset
family's `offset_solid_box_round_matches_minkowski_closed_form` was a **runtime bomb** (> 15 min, no
verdict) because `fillet_edges` returned `Surface::Nurbs` patches whose boundaries went through
`fit_pcurve`, and `📐️box-fillet-preview` failed (**7 of 8** examples green). On arrival in the
working tree, the lib TEST binary did not build at all (§7), so no lane was runnable.

### 6.2 After

| Lane | Filter | Result |
| --- | --- | --- |
| blend | `brep::schema::diff::blend` | **10 passed / 0 failed** (37.4 s) |
| offset | `brep::schema::diff::offset` | **13 passed / 0 failed** (37.3 s) — including `offset_solid_box_round_matches_minkowski_closed_form`, §5.6's bomb, in seconds |
| euler | `brep::schema::diff::euler` | **10 passed / 0 failed** |
| validation | `brep::schema::inferences::validation` | **17 passed / 0 failed** |
| engine (incl. `OPERATION_QUALITY`) | `brep::schema::engine` | **27 passed / 0 failed** (25.3 s) |
| **new** analytic-blend | `--test brep_analytic_blend` | **7 passed / 0 failed** (70.0 s) |
| examples | `--test example-geometry` | **8 passed / 0 failed** (27.4 s) — `📐️box-fillet-preview` green |

A whole-subset run (`--lib -- brep::schema`) reads **543 passed / 32 failed**; all 139 tests of
blend + offset + euler + engine + inferences pass and **none of the 32 failures is in this lane** —
they are entirely `brep::schema::mutations::component` and `brep::schema::snapshot::body`, a peer's
in-flight JSON number-carrier change (committed values now `Float(0.0)` where the fixtures expect
`UInt(0)`) and a `curves2` count drift. Not caused by, and not fixable from, this lane.

### 6.3 Timing

`fillet_of_all_twelve_box_edges_stays_inside_the_interactive_budget`, native **debug** build, three
consecutive runs on a 2.6 × 3.6 × 2.1 box at `r = 0.3`, deflection `1e-3`:

```
attempt 0: fillet 1.094708ms, fillet + tessellation 61.331208ms, 5420 triangles
attempt 1: fillet 1.012250ms, fillet + tessellation 60.597250ms, 5420 triangles
attempt 2: fillet 1.024250ms, fillet + tessellation 61.262167ms, 5420 triangles
```

**The 12-edge fillet itself is ~1.05 ms**; with its tessellation ~63 ms, against the 200 ms budget
the test enforces on every attempt individually. The same call under the NURBS engine did not
return inside 15 minutes.

### 6.4 Compilation

Private `CARGO_TARGET_DIR`, `RUSTC_WRAPPER=""`, each log shows the crate genuinely recompiling
(`Checking semio-s-artifact-stdio-semio`), not a cache hit:

* `cargo check -p semio-s-artifact-stdio-semio --all-targets --keep-going` — **0 errors, 0 warnings**
  (1 m 02 s). `--all-targets` covers the new `[[test]]` target.
* `cargo check -p semio-s-artifact-stdio-semio --lib --keep-going --target wasm32-wasip2 --profile
  wasm-dev` with `CARGO_PROFILE_WASM_DEV_DEBUG=false` — **exit 0, 0 errors, 0 warnings** (1 m 02 s).
* `cargo test -p semio-s-artifact-procedural-generation3d --test example-geometry` builds and links
  the harness plus its `parry3d` oracle against the real crates.

## 7. Two peer blockers cleared on the way

Neither is this lane's code; both made every test in the crate unrunnable, so they are fixed
test-side with no API change:

* `📦️object/🧬️schema/🧪️tests/🪪️document-contract/🦀️.rs:20` — `encode_pack()` now returns `Vec<u8>`,
  not a `Result`; dropped the stale `.expect("Object Pack")`.
* same file `:67` and `📦️object/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:80` — `FaultCode` no longer
  implements `Display`; `.code.to_string()` → `.code.0.as_str()`.

A third, `semio-framework-plugin`'s `E0592 duplicate definitions with name with_children`, appeared
mid-run and the peer resolved it themselves within a few minutes (the second definition is now
`with_render_context`); the affected run was simply repeated.

## 8. Raw logs

`🗑️generated/blend-*.txt` — `blend-tests1`, `blend-offset`, `blend-euler`, `blend-validation`,
`blend-engine`, `blend-analytic` (the new `[[test]]`, with the timing lines), `blend-example`
(the 0.01-deflection failure that motivated §5), `blend-example3` (all eight green),
`blend-final-lib` (the whole-subset run of §6.2), `blend-native-check`, `blend-wasm-check`.
