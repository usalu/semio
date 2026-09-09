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
| `kernelStatus` | `green` or `blocked-on-boolean-kernel` — machine-readable, not prose |

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

`🧪️tests/🧊️example-geometry-3d-1/{🥒️.feature,🐍️.py}`, mirroring `🧊️mutate-procedural-3d-1/`'s
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
| 📦️ rectangle-extrude-volume | `rectangle → math.vector → solid.extrude → measure.volume` | w 2, h 2, d 3 | **12.0** | analytic `w·h·d` | 2 × 2 × 3 | 0.05 | green |
| 🧹️ face-sweep-extrude | `rectangle → surf.planarFaceWire → math.vector → sweep.extrude` | w 2, h 1.5, d 4 | **12.0** | analytic `w·h·d` | 2 × 1.5 × 4 | 0.05 | green |
| 🍄️ hexagonal-mushroom-column | `curve.polygon → math.vector → solid.extrude` | r 0.5, sides 6, h 6 | **3.897114317030** | analytic `(3√3/2)r²·h` | span z = 6, xy between inradius and circumradius | 0.05 | green |
| 🐚️ box-shell-preview | `prim3d.box → solid.shell` | size 2, thickness 0.2 | **3.904** | analytic `s³ − (s−2t)³` | 2 × 2 × 2 | 0.05 | green |
| 📐️ box-fillet-preview | `prim3d.box → solid.fillet` | size 2, radius 0.15 | **7.888634924** | analytic Minkowski `(s−2r)³ + 6r(s−2r)² + 3πr²(s−2r) + 4πr³/3` | 2 × 2 × 2 | 0.01 | green |
| 🧲️ sphere-box-fuse | `prim3d.sphere + prim3d.box → bool.fuse` | r 1.2, size 1.5 | **7.245424930456** | `scipy.integrate.quad` over clipped disc areas; ball 7.238229474 + cube 3.375 − overlap 3.367804543 | 2.4³ | 0.01 | **blocked-on-boolean-kernel** |
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
| `🧪️tests/🧊️example-geometry-3d-1/🥒️.feature` | the language-agnostic scenario set (8 scenarios) |
| `🧪️tests/🧊️example-geometry-3d-1/🐍️.py` | the `scipy`/`numpy` third-party oracle |

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

_(filled in below — see §7.1-§7.4)_
