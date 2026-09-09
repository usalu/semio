# Example Geometry Tests — Procedural 3D (2026-09-09)

Closes gap #2 of `📓️kernel-and-preview-audit-2026-09-09.md` ("Zero automated regression coverage
links any example to real kernel output") and TL;DR #5 of `📓️feature-inventory-2026-09-09.md`
("No example has a test that proves it actually tessellates").

Before this pass, all sixteen example-level tests (8 Rust + 8 TypeScript) asserted exactly one
thing: that the example's `.dsl.semio` text was longer than eight bytes. Nothing anywhere proved
that any of the eight bundled examples produced a single triangle, let alone the right solid.

---

## 1. Harness design

Three lanes meet on **one committed file per example** and never on each other's code.

```
🖼️assets/<name>/🗣️.dsl.semio  ──┬── Rust  ── parse_dsl → FlowHost::evaluate → tessellate_geometry → measure
                                 │           └── parry3d recomputes volume / COM / AABB from the same soup
                                 ├── TypeScript ── recomputes every closed-form number + both quadratures
                                 └── Python  ── scipy/numpy re-derive every number from the DSL + descriptor
                                       ▲
🧪️tests/🧩️example/🔣️.json  ────────┴── the single expected-stats statement all three assert against
```

### 1.1 Rust subject — `[[test]] example-geometry`

`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧩️geometry/🦀️.rs`

Declared in `📦️packages/🦀️rust/Cargo.toml` as `[[test]] name = "example-geometry"` with an
explicit package-root-relative `path` (an emoji file name never compiles as an implicit `tests/`
target — see the repository note on that).

Per example the harness:

1. `install_flow_extension(FlowExtensionSpec { id: "example-geometry", install })` publishes one
   composed operator registry containing `semio_s_plugin_flow_extension_brep::register` (polled to
   completion — it is `async` by convention but never suspends) and
   `semio_s_plugin_flow_extension_math::register`. `math.vector`/`math.point` are **not** written
   out by the math extension: `neural::Registry::finalize` auto-derives a schema-component operator
   `<module>.<schema id>` for every registered schema, which is where `math.vector` comes from. This
   was worth confirming — a repo-wide grep for the literal `"math.vector"` finds it in no
   `register()` body at all, which reads like a missing operator until the auto-registration path is
   traced (`🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs:1424-1437`).
2. `parse_dsl(text)` — the artifact's own text codec — yields the `Generation3dSnapshot`, whose
   `fixture` is the `FlowFixture`.
3. `FlowHost::from_fixture` + `set_neuron_kind_infos_json(&flow_neuron_kind_infos_json())` +
   `evaluate()` — the same host the editor's `⏱️flow-eval-tick` command drives, just with the whole
   graph solved in one shot instead of across budgeted ticks.
4. The preview node's geometry handle is read out of the evaluation JSON (`{node}.out.{channel}`),
   asserting `$schema == "geometry"` and the declared `kind`. A node that failed upstream reports
   its own error message rather than a missing-key panic.
5. `tessellate_geometry(handle, tolerance)` — the same bridge the `brep` extension's `tessellate`
   capability calls — returns `MeshData`.
6. The triangle soup is measured: triangle/vertex counts; closed-and-manifold status by undirected
   edge incidence over position-welded vertices (every edge exactly twice, none more); volume by the
   divergence theorem; edge-polyline length from `edge_positions`; axis-aligned bounding box.
7. `parry3d` recomputes volume and centre of mass (`MassProperties::from_trimesh`) and the bounding
   box (`TriMesh::local_aabb`) from the identical vertex/index arrays.

The whole binary serialises on one process mutex: the brep kernel, its mesh cache and the flow
extension registry are all process-global statics.

### 1.2 Committed expected-stats fixture

`📚️examples/<example>/🧪️tests/🧩️example/🔣️.json`, schema
`s.procedural.generation3d.example-geometry/v1`:

| field | meaning |
|---|---|
| `opChain` | every `neuron-kind` the graph declares — asserted both ways against the DSL |
| `preview.{node,channel,kind}` | which evaluated output carries the geometry the preview renders |
| `tessellationTolerance` | deflection handed to `tessellate_geometry` |
| `expect.minTriangles` / `closed` | soup-level shape assertions |
| `expect.volume` + `volumeTolerance` + `volumeSource` | the number **and its written provenance** |
| `expect.edgePerimeter` + tolerance | wire-only previews |
| `expect.boundingBox{Min,Max}` + tolerance | extent |
| `expect.kernelVolume{Node,Channel}` | where the graph's own `brep.measure.volume` result lives, when the chain has one |
| `kernelStatus` | `green`, `blocked-on-boolean-kernel`, `blocked-on-extrude-orientation` or `blocked-on-fillet-kernel` — machine-readable, not prose. A `blocked-*` standing names one located kernel defect and relaxes NOTHING; see §7.2/§7.3 |

### 1.3 TypeScript twin

`📚️examples/🧪️tests/🧩️geometry/🟦️.ts` (shared assertions) + each example's
`🧪️tests/🧩️example/🟦️.ts`.

**The brep kernel is Rust-only and this plugin has no reachable TypeScript evaluation path** —
`brepjs`/`spatial-kernel` stays scoped to the `cad` plugin, confirmed in
`📓️kernel-and-preview-audit-2026-09-09.md` §1. So the TS lane does **not** evaluate geometry and
does not claim to. It:

- validates the fixture's schema and every structural invariant of the format;
- holds the fixture's `opChain` to the example's own DSL **in both directions** (every declared
  `neuron-kind` must be in the chain and vice versa), and the preview node id to a declared widget;
- **recomputes every committed number** from the DSL's own slider values — the six closed-form
  volumes, the wire perimeter, the bounding boxes, and both no-closed-form volumes by its own
  composite-Simpson rule.

That is a second implementation of the *expectation*, not of the kernel, and the feature file says
so explicitly.

### 1.4 Python third-party oracle

`🧪️tests/📐️example-geometry-3d-1/{🥒️.feature,🐍️.py}`, mirroring `🧊️mutate-procedural-3d-1/`'s
shape. Reads exactly two committed, language-neutral inputs and nothing else from this repository:

- the example's own `🗣️.dsl.semio` (slider values, declared `neuron-kind`s);
- the **packaged brep extension descriptor** `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🔣️.json`,
  for the channel defaults an unwired port falls back to — which is how `🍩️sphere-cut-with-torus`'s
  torus gets `major = 2.0` / `minor = 0.5`, values that appear nowhere in that example's DSL.

It imports no Rust and evaluates no graph: it recognises each op chain as the classical solid it is
and integrates that solid with `scipy`/`numpy`. Collected by `pyproject.toml`'s
`python_files = ["🐍️.py"]`, and runnable directly.

---

## 2. Per-example expected stats

Sliders are read from each example's own DSL. `t` is the tessellation deflection the fixture pins.

| Example | Op chain | Sliders | Expected volume | Provenance | BBox | `t` | `kernelStatus` |
|---|---|---|---|---|---|---|---|
| 🪢️ rectangle-wire-preview | `brep.curve.rectangle` | w 2, h 1.5 | — (edge-only) | perimeter `2(w+h)` = **7.0** | 2 × 1.5 × 0 | 0.05 | green |
| 📦️ rectangle-extrude-volume | `rectangle → math.vector → solid.extrude → measure.volume` | w 2, h 2, d 3 | **12.0** | analytic `w·h·d` | 2 × 2 × 3 | 0.05 | **blocked-on-extrude-orientation** |
| 🧹️ face-sweep-extrude | `rectangle → surf.planarFaceWire → math.vector → sweep.extrude` | w 2, h 1.5, d 4 | **12.0** | analytic `w·h·d` | 2 × 1.5 × 4 | 0.05 | **blocked-on-extrude-orientation** |
| 🍄️ hexagonal-mushroom-column | `curve.polygon → math.vector → solid.extrude` | r 0.5, sides 6, h 6 | **3.897114317030** | analytic `(3√3/2)r²·h` | span z = 6, xy between inradius and circumradius | 0.05 | **blocked-on-extrude-orientation** |
| 🐚️ box-shell-preview | `prim3d.box → solid.shell` | size 2, thickness 0.2 | **3.904** | analytic `s³ − (s−2t)³` | 2 × 2 × 2 | 0.05 | green |
| 📐️ box-fillet-preview | `prim3d.box → solid.fillet` | size 2, radius 0.15 | **7.888634924** | analytic Minkowski `(s−2r)³ + 6r(s−2r)² + 3πr²(s−2r) + 4πr³/3` | 2 × 2 × 2 | 0.01 | **blocked-on-fillet-kernel** |
| 🧲️ sphere-box-fuse | `prim3d.sphere + prim3d.box → bool.fuse` | r 1.2, size 1.5 | **7.245424930456** | `scipy.integrate.quad` over clipped disc areas; ball 7.238229474 + cube 3.375 − overlap 3.367804543 | 2.7³ (`[-1.2,-1.2,-1.2]`–`[1.5,1.5,1.5]`) | 0.01 | **blocked-on-boolean-kernel** |
| 🍩️ sphere-cut-with-torus | `prim3d.sphere + prim3d.torus → bool.cut → measure.volume` | sphere r 2.2 (slider), torus R 2.0 / r 0.5 (channel defaults) | **37.841142613316** | `scipy.integrate.quad` over the tube angle; ball 44.602238101 − torus∩ball 6.761095487 | 4.4³ | 0.01 | **blocked-on-boolean-kernel** |

Both quadratures are computed twice by independent implementations — `scipy.integrate.quad`
(adaptive) in the Python lane and composite Simpson in the TypeScript lane — and agree inside the
fixtures' own tolerances.

### 2.1 A note on the stale baked preview value

`🍩️sphere-cut-with-torus`'s DSL carries a baked `output-preview` value of `37.73359174067349` from
some past authoring session. The exact difference volume is **37.841142613316** — the baked number
is **0.28 % low**, consistent with a faceted sphere/torus approximation, and nothing in the test
suite ever read it. It is left in place (it is authored document content, not a fixture) but it is
not an expectation and should not be mistaken for one.

---

## 3. Oracle registration

Two entries added to
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracle/🔣️.json`,
both `kind: "third-party-library"`, both `testOnly`, both `productionReachable: false`, capability
`procedural-3d-1-example-geometry`, comparison profile `floating-point-v1`:

| id | ecosystem | package | what it establishes | what it cannot |
|---|---|---|---|---|
| `example-geometry-parry3d` | rust | `parry3d` 0.17 | recomputes volume, COM and AABB **from real kernel output** with foreign code | measures whatever soup it is handed — cannot tell a correct solid from a wrong one |
| `example-geometry-scipy` | python | `scipy` (+`numpy`) | states **what the number must be**, from the DSL and the packaged descriptor only | never touches the kernel — can convict a wrong fixture, never a wrong solid |

Selected against `🔒️dependencies.json`: `parry3d 0.17` is already carried at exactly this standing
(`test-runner`, `productionReachable: false`) by `🧰️framework/🔨️modules/🧊️3d`, and `numpy`/`scipy`
are already registered python `test-runner` deps. `trimesh` was **not** used — it is not in
`🔒️dependencies.json` and adding it would have been a new external dependency for no gain over
`parry3d` + `scipy`.

`parry3d` sits on `engine.family = "none"` (a mesh/collision library, not an exact-geometry kernel),
so its independence from the `semio-s-artifact-stdio-semio` B-Rep kernel under test is total.

---

## 4. `demo-session` — why it stays out of `examples()`

**It is not a complete example of this artifact and must not be added.** Its entire primary text is:

```
semio generation.3d.cmd v1
action=demo
```

That is a `generation.3d.cmd` **command envelope**, not a `procedural.generation3d.dsl`
`flow.fixture` document. `ExampleSource::source()`'s text is fed to the artifact's `parse_dsl`, which
parses `Generation3dSnapshot` (`FlowFixture` + `GenerationPlayRoot`); a two-line command header
carries no `widgets`, no `synapses`, no `camera` and a different schema id, so selecting it from the
navbar dropdown would fail to parse rather than show anything. It has an `ExampleSource` shape only
because every example directory in that tree does.

Recommendation: either give it a real `.dsl.semio` asset (at which point it becomes a ninth example
and gets a ninth expected-stats fixture), or move it out of `📚️examples/` entirely — it is a
command fixture, not an example. Left untouched by this pass; `examples()` still returns 8.

## 5. `brep.measure.validate` — why the DSLs stay untouched

The audit's "no validate gate" finding (§5, gap #5) is **not** best fixed at the example level, and
adding `brep.measure.validate` to the eight DSLs would have been theatre. `preview_tessellate_effects`
(`✏️editor/🦀️.rs:1850-1876`) walks the fixture's widgets, collects preview channel *handles*, and
emits `Effect::InvokeExtension{capability: "tessellate"}` for each. It reads no validity channel and
there is no path by which one would reach the renderer or the user. A `validate` node in the graph
would compute a report into a channel nothing consumes, and the invalid geometry would still be
tessellated and drawn — while every reader of the DSL would now believe there was a gate.

The correct fix is a host-level gate in `preview_tessellate_effects` (or inside the `tessellate`
handler), which is outside this pass's lane. **The eight DSLs are unmodified.** In the meantime the
new Rust lane supplies the missing signal from the other side: the closed-and-manifold assertion
fails on exactly the shell-not-closed / non-manifold-edge conditions `validate` would have reported.

---

## 6. Diff summary

### Added

| File | What |
|---|---|
| `📚️examples/🧪️tests/🧩️geometry/🦀️.rs` | the Rust harness + the 8 tests |
| `📚️examples/🧪️tests/🧩️geometry/🟦️.ts` | shared TypeScript assertions + Simpson quadrature |
| `📚️examples/<8 examples>/🧪️tests/🧩️example/🔣️.json` | 8 committed expected-stats fixtures |
| `🧪️tests/📐️example-geometry-3d-1/🥒️.feature` | the language-agnostic scenario set (8 scenarios) |
| `🧪️tests/📐️example-geometry-3d-1/🐍️.py` | the `scipy`/`numpy` third-party oracle + its `adapter()` registration |

### Modified

| File:line | Change |
|---|---|
| `📦️packages/🦀️rust/Cargo.toml:53-73` | `[dev-dependencies]` += `semio-framework-os-flow`, `semio-s-plugin-flow-extension-brep`, `semio-s-plugin-flow-extension-math` (all `default-features = false` where the default is a component-guest feature), `parry3d = "0.17"`; new `[[test]] name = "example-geometry"` with explicit path |
| `📚️examples/<8 examples>/🧪️tests/🧩️example/🟦️.ts` | replaced the `length > 8` one-liner with fixture-contract + op-chain + recomputed-number assertions (the `ships primary asset` check is kept) |
| `🔮️oracle/🔣️.json` | `oracles` += `example-geometry-parry3d`, `example-geometry-scipy` |

### Modified outside this artifact (unblocking only)

| File:line | Change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🏠️owner/🧬️schema/🦀️.rs:2` | `use crate::os_dsl::{FromValue, ToValue}` → `use semio_framework_value_derive::{FromValue, ToValue}` — the file imported the *traits* and used them as *derive macros*, so `semio-framework-os-kernel` did not compile at all and neither did anything downstream. Matches the convention of its own sibling `🧩️composition/🗄️durable-group/🦀️.rs:7`. A concurrent session landed the identical fix minutes later; the resulting duplicate `use` line was deduplicated. |

---

## 7. Test runs

Resumed 2026-09-09 18:00 after the machine reboot that killed the authoring lane at ~17:30. All
cargo work ran in a private, APFS-cloned `CARGO_TARGET_DIR`
(`…/scratchpad/target-extest`, seeded `cp -Rc` from `target/debug`) with `RUSTC_WRAPPER=""`, so the
shared `target/` the boot lane owns was never touched. Raw logs: `🗑️generated/resume-*.txt`.

### 7.1 Rust — `[[test]] example-geometry`, all eight examples

```
CARGO_TARGET_DIR=…/target-extest RUSTC_WRAPPER="" \
  cargo test -p semio-s-artifact-procedural-generation3d --test example-geometry \
  -- --nocapture --test-threads=1
```

Tail (`🗑️generated/resume-example-geometry-all8.txt`):

```
test result: FAILED. 2 passed; 6 failed; 0 ignored; 0 measured; 0 filtered out; finished in 821.98s
```

**Two pass, six fail — and not one of the six is the harness's or the fixture's own fault.** Every
committed number survived contact with the kernel: where a run disagrees, the harness's measurement
and `parry3d`'s independent measurement of the same soup agree with EACH OTHER and disagree with the
solid the DSL describes. The 821.98 s is almost entirely `box-fillet-preview` (820 s of unoptimized
`fillet_one_edge → set_face_pcurves → fit_pcurve → solve_interpolation_1d → solve_linear_system`,
confirmed by `sample(1)` on the live process); the other seven together run in 0.7 s.

| Example | Result | tri / vert | closed | boundary / non-manifold / orientation-defect edges | soup volume | `parry3d` volume | kernel `measure.volume` | expected | bbox |
|---|---|---|---|---|---|---|---|---|---|
| 🪢️ rectangle-wire-preview | **pass** | 0 / 0 | false (wire) | 0 / 0 / 0 | — (edge length **7.000000000**) | — | — | perimeter 7.0 | [0,0,0]–[2,1.5,0] ✔ |
| 📦️ rectangle-extrude-volume | **fail — blocked-on-extrude-orientation** | 12 / 24 | true | 0 / 0 / **8** | **4.000000000** | 3.999999523 | **12.0 ✔** | 12.0 | [0,0,0]–[2,2,3] ✔ |
| 🧹️ face-sweep-extrude | **fail — blocked-on-extrude-orientation** | 12 / 24 | true | 0 / 0 / **8** | **4.000000000** | 3.999999523 | — | 12.0 | [0,0,0]–[2,1.5,4] ✔ |
| 🍄️ hexagonal-mushroom-column | **fail — blocked-on-extrude-orientation** | 20 / 36 | true | 0 / 0 / **12** | **1.299038082** | 1.299038172 | — | 3.897114317030 | [-0.5,-0.4330127,0]–[0.5,0.4330127,6] ✔ |
| 🐚️ box-shell-preview | **pass** | 24 / 48 | true | 0 / 0 / 0 | 3.904000389 | 3.903999567 | — | 3.904 | [0,0,0]–[2,2,2] ✔ |
| 📐️ box-fillet-preview | **fail — blocked-on-fillet-kernel** | 996 / 552 | **false** | **40** / 0 / 6 | 6.000000133 | 6.799999714 | — | 7.888634924 | [0,0,0]–[2,2,2] ✔ |
| 🧲️ sphere-box-fuse | **fail — blocked-on-boolean-kernel** | — | — | — | — | — | — | 7.245424930456 | — |
| 🍩️ sphere-cut-with-torus | **fail — blocked-on-boolean-kernel** | — | — | — | — | — | — | 37.841142613316 | — |

The two boolean examples never reach a mesh; the chain refuses at the operator:

```
node "fuse" failed to evaluate: invalid input: operation failed: operation failed:
  imprint point does not lie on the face's boundary loop
node "brep_bool_cut_5" failed to evaluate: invalid input: operation failed: operation failed:
  imprint point does not lie on the face's boundary loop
```

That is verbatim the failure `📓️boolean-kernel-2026-09-09.md` owns (and the same string that lane's
own `🧲️procedural-example-booleans` reproduces). **Neither assertion was loosened and neither
expected volume was touched.**

### 7.2 The extrude failure, diagnosed — every planar lateral face is oriented inward

Three failures share one cause, and it is neither the fixture's arithmetic nor the harness's. A
temporary `[DEBUG]` probe in the harness (added, run, and removed again; raw output
`🗑️generated/resume-extrude-orientation-probe.txt`) printed, per tessellated triangle of
`rectangle-extrude-volume`, its winding normal, the stored shading normal at its first vertex, and
the sign of `winding · (centroid − solid centre)`:

| triangles | face | winding normal | stored normal | `winding·stored` | outwardness |
|---|---|---|---|---|---|
| 0–1 | cap z=0 | (0, 0, −4) | (0, 0, −1) | +4 | **+6 outward** |
| 2–3 | cap z=3 | (0, 0, +4) | (0, 0, +1) | +4 | **+6 outward** |
| 4–5 | side y=0 | (0, +6, 0) | (0, +1, 0) | +6 | **−6 INWARD** |
| 6–7 | side x=2 | (−6, 0, 0) | (−1, 0, 0) | +6 | **−6 INWARD** |
| 8–9 | side y=2 | (0, −6, 0) | (0, −1, 0) | +6 | **−6 INWARD** |
| 10–11 | side x=0 | (+6, 0, 0) | (+1, 0, 0) | +6 | **−6 INWARD** |

All four lateral faces point into the solid; both caps point out; and `winding · stored > 0`
everywhere, so the SHADING normals are inverted with the winding rather than disagreeing with it —
the preview's side walls are lit from the inside. Arithmetic that follows from exactly that: only the
`z = c` cap contributes to `⅓∮x·n dA`, the two inverted max-normal side faces cancel the rest, and
the soup's signed volume lands on `−abc/3` — `|−12/3| = 4.0` for both boxes, and
`3.897114/3 = 1.299038` for the hexagonal prism, precisely the three numbers observed. The
orientation-defect counts follow too: the mis-wound edges are exactly the cap↔lateral ones, 4+4 = **8**
for a rectangular prism and 6+6 = **12** for a hexagonal one.

The kernel's own `brep.measure.volume` says **12.0** — the B-Rep body is right; only the mesh handed
to the renderer and to any mass-property integrator is wrong.

**Located at**
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/🔺️diff/➡️sweep/🧮️core/🦀️.rs:271`:

```rust
let lateral_flipped = matches!(lat.surface, Surface::Plane { .. });
```

Its own 15-line comment records why: with `flipped = false` "every one of the 4 side faces'
independently-computed `frame.z` was the CORRECT physical outward direction (hand-verified against
the box's own geometry), yet their combined `signed_tetra_sum` contribution was `-16` instead of
`+16`". So `flipped` — a *geometric orientation* flag that
`💡️inferences/🧩tessellation/🦀️.rs:386-394` feeds straight into `fix_winding_per_triangle` and
`vertex_normal` — was set to `true` to patch a sign convention in the mass-property integrator. The
volume gauge reads right and the geometry is inverted. **The fix belongs in `signed_tetra_sum`'s loop
winding, not in the `flipped` flag, and it is not this lane's to make: it sits inside the same
`🧊️brep` subset the boolean-kernel lane owns and would move every volume that lane's tests assert.**
Reported, not edited.

### 7.3 The fillet failure, diagnosed — an unclosed shell, holes at the corners

Same method (`🗑️generated/resume-fillet-boundary-probe.txt`): a temporary `[DEBUG]` pass printed the
midpoint of every free (incidence-1) edge of `box-fillet-preview`'s 996-triangle soup. All **40** of
them, on a 2×2×2 box filleted at r=0.15:

- **24** ring the eight corners — three per corner, at `(0, 0.075, 0.075)`, `(0.075, 0, 0.075)`,
  `(0.075, 0.075, 0)` and the seven mirror images. `0.075 = r/2`: these are the mouths of eight
  missing corner patches.
- **12** lie on blend tangency lines at the middle of a box edge, offset by exactly `r = 0.15`
  (`(0, 1, 0.15)`, `(1, 0, 0.15)`, `(1, 0.15, 0)`, …).
- **4** lie on the ORIGINAL sharp edge line itself (`(0, 1, 0)`, `(0, 1, 2)`, `(2, 1, 0)`,
  `(2, 1, 2)`) — the four box edges parallel to y still carry an unblended sharp edge.

`brep.solid.fillet` therefore returns blend faces that were never stitched into a closed shell, so
there is no enclosed volume to compare: the soup measures 6.000000133 (and `parry3d` 6.799999714 on
its own vertex merge — two different numbers, which is itself the signature of an open surface)
against the analytic 7.888634924. Also inside the `🧊️brep` subset; reported, not edited.

### 7.4 TypeScript twin

```
bun nx run @semio-tech/procedural-js:test --skip-nx-cache
```

```
 35 pass
 0 fail
 238 expect() calls
Ran 35 tests across 11 files. [93.00ms]
```

Green, before and after this pass's edits. A third confirmation run at 19:5x could not go through
`nx` at all — the project graph itself failed repo-wide on a peer's in-flight edit
(`Cannot find module './🧪️tests/🧊️mesh-pack-decode/🟦️.ts'`, referenced from
`🧰️framework/🛍️products/💻️os/🟦️.ts:2393` against a directory that does not exist yet; also
`Source project does not exist: npm:@asamuzakjp/css-color`). Nothing to do with this plugin, and it
still failed after waiting, so the final confirmation ran the target's own command directly in the
cwd `📋️project.json` declares — `bun ./📜️script.ts test` from
`✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript` — with the identical result, exit 0
(`🗑️generated/resume-ts-lane.txt`). The pre-existing
baseline for this lane was **11 tests / 11 `expect()` calls** — one length check per example; it is
now **35 tests / 238 assertions**, and every one of the eight examples' committed numbers is
recomputed on this side from the DSL's own sliders.

### 7.5 Python third-party oracle, run directly

```
./.venv/bin/python3 "…/🧪️tests/📐️example-geometry-3d-1/🐍️.py"
```

```
PASS: numpy 2.5.0 + scipy agree with all 23 committed expectations across 8 examples
```

Exit 0 (`🗑️generated/resume-python-oracle.txt`). **All 23 committed expectations across all eight
examples are re-derived independently and agree — including the two whose kernel cannot yet produce
them.** So every fixture number in §2 is confirmed correct by third-party code, which is precisely
what makes §7.2/§7.3's verdict "the kernel is wrong, not the fixture" a measurement rather than an
opinion.

### 7.6 The language-agnostic lane — it runs, and the audit's blocker was misdiagnosed

`📓️test-harness-audit-2026-09-09.md` §2 recorded this lane as blocked on building a PyO3 extension.
That is not what `semio_repo_test` is: it is a pure-Python module at
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🐍️python/🐍️.py` that installs itself as
`sys.modules["semio_repo_test"]` when the coordinator's host script loads an adapter. Nothing needs
compiling, and the bare `import semio_repo_test` the audit tried can never succeed by design. Run
through the coordinator instead:

```
bun nx run @semio-tech/repo-test-domain:test-discover
bun nx run @semio-tech/repo-test-domain:test-oracle -- --id test-…-📐️example-geometry-3d-1
```

Discovery sees the case (`250 test case(s)` total):

```
test-s-plugins-procedural-artifacts-generation3d-standards-1-subsets-any-cdccaf-📐️example-geometry-3d-1
  ✏️s/…/🧪️tests/📐️example-geometry-3d-1	[python]
```

and after the three registration repairs in §8 the oracle phase runs **all eight scenarios green**
(`.🧬semio/🦑️repo/⚡️cache/tests/results/…-oracle-python/📤️results.jsonl`):

| scenario | status | projection |
|---|---|---|
| rectangle-wire-preview | passed | closed false, perimeter 7.0, extent [2, 1.5, 0] |
| rectangle-extrude-volume | passed | volume 12.0, extent [2, 2, 3] |
| face-sweep-extrude | passed | volume 12.0, extent [2, 1.5, 4] |
| hexagonal-mushroom-column | passed | volume 3.897114317029974 |
| box-shell-preview | passed | volume 3.903999999999999, extent [2, 2, 2] |
| box-fillet-preview | passed | volume 7.888634923940582, extent [2, 2, 2] |
| sphere-box-fuse | passed | volume 7.245424930456152, extent [2.7, 2.7, 2.7] |
| sphere-cut-with-torus | passed | volume 37.84114261331554, extent [4.4, 4.4, 4.4] |

The command still exits 1 — on a repo-wide baseline classification sweep that names ~25 unrelated
cases (`unknown oracle id zip/png/tobj/stl-io/…`, `needs a typescript adapter`, and the tiled-map
python oracle crashing). **No line of that failure names this case.**

### 7.7 Repo-wide gates this pass's files touch

| Gate | Result |
|---|---|
| `bun ./📜️script.ts verify taxonomy enforce` | **aborts repo-wide, not ours.** `Normalization requires an explicit repository-boundary decision before authored classification: .🧬semio/…/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/📓️rust-join-provenance/🧪️runs/🧪️path-ambiguous-Izy9Ww` — a nested gitlink inside a different ticket's folder, exactly as `📓️viewer-2026-09-09.md` found. (Note for whoever runs it next: the verb needs `report`/`enforce`; bare `verify taxonomy` throws on the missing mode.) |
| `bun 🐍️viewer-policy-check.ts` (the pure policy functions, scoped to `🌀️procedural`) | ran; **one breach was ours and is fixed** — `🧪️tests/🧊️example-geometry-3d-1` collided with sibling `🧪️tests/🧊️mutate-procedural-3d-1` under `folder/name/emoji-not-unique`. Renamed to `📐️example-geometry-3d-1`; procedural emoji-prefix breaches 13 → 12, and no example-geometry line remains. The other 18 are pre-existing (generation2d/assembly `app-schema` facets, peer-owned editor/viewer test directories). |
| `bun ./📜️script.ts verify dependencies literal-external` | `target=0, current=193, oracle-conflicts=23, toolchain-owner-conflicts=2, toolchain-failures=0` — **byte-identical to the pre-existing baseline** in `🗑️generated/lane4-verify-dependencies.txt`. Neither `parry3d` nor `scipy`/`numpy` appears in any conflict line, so the two new oracle registrations added nothing to the ratchet. |

### 7.8 A concurrent kernel change, observed mid-pass

A verification re-run started at 18:5x recompiled `semio-s-artifact-stdio-semio` (the boolean lane
landing work) and **the two boolean examples no longer refuse** — `bool.fuse` now runs, and was still
running after **61 minutes** of a debug build without returning, at which point this pass stopped
waiting and killed it (`🗑️generated/resume-example-geometry-rerun.txt`). Two `sample(1)` captures 20
minutes apart put it in the SAME place both times, and it is not the boolean:

```
BrepKernel::tessellate → tessellate_solid_with_report → append_face_mesh
  → refine_adaptive → insert_point_into
```

So the fuse itself now SUCCEEDS and returns a solid; what does not finish is the preview
tessellation of that solid at the fixture's 0.01 deflection — Bowyer–Watson point insertion into the
fused sphere's face, unoptimized, against a box already at load 51 from peer lanes. RSS stayed at
76 MB and `MAX_INTERIOR_POINTS`/`MAX_REFINE_ITERS` are bounded, so this is slowness, not a hang, and
not a memory problem. `blocked-on-boolean-kernel` is still the right standing — no boolean example
has yet produced a measured volume — but the blocker has moved one stage downstream and the next
lane should expect a tessellation cost, not an `imprint point…` refusal. The numbers in §7.1 are from the
18:0x run, which is the one this section reports; the five non-fillet, non-boolean examples came out
bit-identical in both runs, now with the new `signedVolume` field making §7.2's defect readable at a
glance:

```
box-shell-preview            volume=3.904000389  signedVolume=+3.904000389  orientationDefects=0
face-sweep-extrude           volume=4.000000000  signedVolume=-4.000000000  orientationDefects=8
hexagonal-mushroom-column    volume=1.299038082  signedVolume=-1.299038082  orientationDefects=12
rectangle-extrude-volume     volume=4.000000000  signedVolume=-4.000000000  orientationDefects=8
rectangle-wire-preview       volume=0.000000000  signedVolume=+0.000000000  orientationDefects=0
```

A negative signed volume on a soup the same run calls `closed=true` is the whole of §7.2 in one
number: watertight, and inside out.

---

## 8. Resumed 18:00 — what changed

Nothing in `✏️editor/**`, nothing in the `🧊️brep` kernel, nothing in the DSL assets. §7.2 and §7.3
locate two kernel defects precisely and deliberately stop there.

| File | Change |
|---|---|
| `📚️examples/🧪️tests/🧩️geometry/🦀️.rs` | `divergence_volume` → `signed_divergence_volume`, keeping the magnitude the expectations are stated in but ALSO reporting the sign as a new `signedVolume` `[STATS]` field — the compact statement of whether a closed soup faces outward, and what makes §7.2's class of defect self-diagnosing on the next run. Kernel-status vocabulary lifted out of an inline `\|\|` into a documented `KERNEL_STATUSES` constant. **No assertion changed, no tolerance widened.** |
| `📚️examples/🧪️tests/🧩️geometry/🟦️.ts` | the same vocabulary as `EXAMPLE_GEOMETRY_KERNEL_STATUSES`, replacing the inline two-element literal, so the two lanes cannot drift. |
| `📚️examples/📦️rectangle-extrude-volume/…/🔣️.json`, `🧹️face-sweep-extrude`, `🍄️hexagonal-mushroom-column` | `kernelStatus` `green` → `blocked-on-extrude-orientation`. Expectations untouched. |
| `📚️examples/📐️box-fillet-preview/…/🔣️.json` | `kernelStatus` `green` → `blocked-on-fillet-kernel`. Expectations untouched. |
| `🧪️tests/🧊️example-geometry-3d-1/` → `🧪️tests/📐️example-geometry-3d-1/` | renamed: `🧊️` collided with sibling `🧊️mutate-procedural-3d-1` under the `folder/name/emoji-not-unique` policy (§7.7). |
| `🧪️tests/📐️example-geometry-3d-1/🐍️.py` | **added `def adapter()`** — eight `Adapter("python").oracle(<example>, …)` registrations returning this file's own derivation. Without it the coordinator refused the case with "must define `def adapter() -> Adapter`", i.e. the language-agnostic lane had never actually been reachable. The harness import stays inside the function so the file remains runnable standalone and collectable by `pytest`. Oracle role only — never subject. |
| `🧪️tests/📐️example-geometry-3d-1/🥒️.feature` | the `⚠️` block now names all three located defects and all six blocked examples instead of two; `And the fixture declares kernelStatus …` steps added to the four newly-blocked scenarios; **`@oracle-example-geometry-parry3d` removed from the feature tags** — a generated repository-test host may take no Cargo dependency on a plugin crate or a third-party one, so tagging it made the coordinator demand a rust adapter this directory is not allowed to have (`oracle example-geometry-parry3d needs a rust adapter to run in`). Sibling `🚪️io-procedural-3d-1` leaves `io-round-trip-parry3d` untagged for exactly this reason. The registration in `🔮️oracle/🔣️.json` stays; the lane it runs in (`[[test]] example-geometry`) is now named in the feature. |
| `🔮️oracle/🔣️.json` | `example-geometry-scipy.packages` was `["scipy", "numpy"]` — a list of STRINGS where `OracleLinkedPackage` objects are required, which crashed the whole coordinator (`TypeError: undefined is not an object (evaluating 'linked.package.length')` in `oracleLinkedPackages`) for every case in the repository, not just this one. Now one object for `numpy` (`scipy` is already the primary), with its own version/license/homepage/role. Case-folder path updated for the rename. |

### 8.1 Corrections to §2 of this document

- `🧲️sphere-box-fuse`'s bbox is **2.7³** (`[-1.2,-1.2,-1.2]`–`[1.5,1.5,1.5]`, the ball's −1.2 face
  against the cube's +1.5 face), not the `2.4³` §2 printed. The committed fixture always carried the
  correct extent — the table's prose was wrong, and `🐍️.py` agrees with the fixture (§7.6).
- §2's `kernelStatus` column now reads `blocked-on-extrude-orientation` for the two extrude rows and
  the sweep row, and `blocked-on-fillet-kernel` for the fillet row.

### 8.2 What is owed, and by whom

1. **`🧊️brep` subset owner** — `signed_tetra_sum`'s loop-winding sign convention, so
   `➡️sweep/🧮️core/🦀️.rs:271` can drop the compensating `flipped = true` and every extruded solid's
   side walls face out. Three examples turn green on that one change.
2. **`🧊️brep` subset owner** — `solid.fillet`'s corner patches and the four unstitched y-parallel
   edge blends (§7.3).
3. **`📓️boolean-kernel-2026-09-09.md`** — the two booleans, whose symptom moved from refusal to
   non-termination during this pass (§7.8).
4. **Anyone touching `box-fillet-preview` or the boolean examples** — this lane is now
   tessellation-cost-bound in a debug build: 820 s for one fillet (hot path `solve_linear_system`
   under `fit_pcurve`, once per blend pcurve) and >61 min for the fused sphere's `refine_adaptive`
   at deflection 0.01 (§7.8). The other five examples together run in 0.7 s. Whoever wires this into
   a routine gate should either run it `--release` or split the two expensive examples out of the
   default filter; the fixtures' 0.01 deflections are deliberate (they are what makes the fillet and
   boolean volumes meaningful) and were not loosened to buy time.

