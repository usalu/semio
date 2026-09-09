@capability-procedural-3d-1-example-geometry
@oracle-example-geometry-scipy
@comparison-floating-point-v1
Feature: Every bundled generation3d example evaluates to the geometry its committed fixture states
  Until this case existed, all sixteen example-level tests (eight Rust, eight TypeScript) asserted
  one thing: that the example's `.dsl.semio` text was longer than eight bytes. Nothing anywhere
  proved that any of the eight bundled examples produced a triangle, let alone the right solid, and
  the last "all eight green" record was a manual playground probe from 2026-08-07. This case is the
  regression gate that record never was.

  THE SUBJECT. `📚️examples/🧪️tests/🧩️geometry/🦀️.rs` (cargo `[[test]] example-geometry`) parses
  each example's real `🗣️.dsl.semio` asset into its `FlowFixture` through the artifact's own
  `parse_dsl`, installs the two packaged flow extensions the graphs' `neuron-kind` chains resolve to
  (`brep`, `math`) into the shared flow operator registry, evaluates the graph through the same
  `FlowHost` the editor's `flow-eval-tick` command drives, reads the preview node's geometry handle
  out of the evaluation JSON, and tessellates it through the same `tessellate_geometry` bridge the
  preview path calls. It then measures the triangle soup: triangle and vertex counts, closed/manifold
  status by undirected-edge incidence, volume by the divergence theorem, edge-polyline length, and
  the axis-aligned bounding box.

  THE COMMITTED STATEMENT. Each example gains one `🧪️tests/🧩️example/🔣️.json` expected-stats
  fixture, schema `s.procedural.generation3d.example-geometry/v1`, naming its op chain, the preview
  node and channel, the tessellation tolerance and every expected number with its own tolerance and
  a written provenance for the volume. Three lanes read that one file and nothing else of each
  other's: Rust (evaluates and measures), TypeScript (`🧩️example/🟦️.ts`, recomputes every
  closed-form number and both quadratures from the DSL's own slider values), and Python
  (`🐍️.py` beside this file).

  THE THIRD-PARTY HALVES, and what each one can and cannot establish.

  `parry3d` (Rust, `[dev-dependencies]`, the same test-only standing it already has in
  `semio-framework-3d`) recomputes volume, centre of mass and the bounding box from the exact
  triangle soup our kernel emitted, via `MassProperties::from_trimesh` and `TriMesh::local_aabb`.
  This is the only check in the case that touches real kernel output with foreign code. It cannot
  tell a correct solid from a wrong one on its own — it measures whatever soup it is handed — which
  is why it is paired with, not substituted for, the committed expectation.

  `example-geometry-parry3d` is registered in `🔮️oracle/🔣️.json` but is deliberately NOT an
  `@oracle-` tag on this feature, for the same reason `🚪️io-procedural-3d-1` leaves
  `io-round-trip-parry3d` untagged: a generated repository-test host may not gain a Cargo dependency
  on a plugin crate or on a third-party one, so the `parry3d` half cannot run as this case's rust
  adapter. It runs where it can reach both the kernel and the library — in-crate, as
  `[[test]] example-geometry`. Tagging it here would make the coordinator demand a rust adapter this
  directory is not allowed to have.

  `scipy`/`numpy` (Python) re-derive what each op chain's solid must measure, from the example's own
  DSL and from the packaged brep extension descriptor's declared channel defaults, and from nothing
  else in this repository. Six of the eight are closed form (rectangular prism, regular-polygon
  prism, hollow box, rounded box as the Minkowski sum of a shrunk cube with a ball); two have no
  closed form and are integrated: the ball-cube union by quadrature over clipped disc areas, and the
  ball-minus-torus difference by quadrature over the torus tube angle. The TypeScript lane repeats
  both integrals with its own composite-Simpson rule, so the two numbers rest on two independent
  quadratures rather than on one library.

  ⚠️ ONE OF THE EIGHT IS STILL KNOWN-BLOCKED, named in the fixture's own machine-readable
  `kernelStatus` rather than in prose. A `blocked-*` standing NEVER relaxes an expectation: every
  committed number stays exactly what the geometry must be and the run keeps failing until the named
  defect is fixed — and a fixed defect takes its `kernelStatus` value out of the vocabulary with it.

  - `blocked-on-fillet-kernel` — `📐️box-fillet-preview`. `brep.solid.fillet` returns an UNCLOSED
    shell (blend patches without their corner pieces), so the preview carries boundary edges and no
    enclosed volume.

  The first run of this lane (2026-09-09 18:00, ticket `26/09/09/PROCEDURAL-3D-END-TO-END` §7)
  located three defects; two are now fixed and their examples are `green`:

  - `blocked-on-extrude-orientation` (`📦️rectangle-extrude-volume`, `🧹️face-sweep-extrude`,
    `🍄️hexagonal-mushroom-column`) — every PLANAR lateral face `➡️sweep`'s prism builder emitted was
    oriented INWARD, so a watertight prism's soup integrated to `−V/3`. Fixed by deriving the lateral
    loop's own `(u, v)` winding and its `flipped` from the profile's winding and the sweep direction
    (`📓️sweep-kernel-2026-09-09.md`).
  - `blocked-on-boolean-kernel` (`🧲️sphere-box-fuse`, `🍩️sphere-cut-with-torus`) — `brep.bool.fuse`/
    `brep.bool.cut` refused with `imprint point does not lie on the face's boundary loop`. Fixed by
    the exact-imprint work in `📓️boolean-kernel-2026-09-09.md` §5. Both examples then exposed a
    FIXTURE error each, not a kernel one: the union volume had been derived for a CONCENTRIC cube
    though `brep.prim3d.box` grows from its corner, and the difference's bounding box had been stated
    as the whole ball though the torus tube (`major + minor = 2.5 > 2.2`) grooves the ball's equator
    away down to a cylindrical radius of `(ball² + major² − minor²) / (2·major) = 2.1475`.

  📌️ ONE CEILING. The TypeScript lane does not evaluate geometry and does not claim to: the brep
  kernel is Rust-only and this plugin has no reachable TypeScript evaluation path (`brepjs` stays
  scoped to the `cad` plugin). It validates the fixture's schema, holds the fixture's op chain to the
  example's own DSL node-for-node in both directions, and recomputes every committed number. That is
  a second implementation of the EXPECTATION, not of the kernel.

  @id-rectangle-wire-preview
  @level-fundamental
  @mode-differential
  Scenario: An edge-only rectangle wire previews as an open polyline of the wired perimeter
    Given the example "rectangle-wire-preview" and its committed expected-geometry fixture
    When the flow host evaluates its "brep.curve.rectangle" chain and the preview handle is tessellated
    Then the preview carries no triangles and is not a closed surface
    And the edge polyline length equals 2*(width+height) from the dsl sliders within the fixture tolerance

  @id-rectangle-extrude-volume
  @level-fundamental
  @mode-differential
  Scenario: A rectangle extruded along a wired vector has the analytic prism volume
    Given the example "rectangle-extrude-volume" and its committed expected-geometry fixture
    And the fixture declares kernelStatus "green"
    When the flow host evaluates its "brep.curve.rectangle -> math.vector -> brep.solid.extrude -> brep.measure.volume" chain
    Then the tessellated solid is closed and manifold
    And its divergence-theorem volume, the parry3d volume and the kernel's own brep.measure.volume all equal width*height*distance within the fixture tolerance
    And its bounding box spans width by height by distance

  @id-face-sweep-extrude
  @level-fundamental
  @mode-differential
  Scenario: A planar face swept along a wired vector has the analytic prism volume
    Given the example "face-sweep-extrude" and its committed expected-geometry fixture
    And the fixture declares kernelStatus "green"
    When the flow host evaluates its "brep.curve.rectangle -> brep.surf.planarFaceWire -> math.vector -> brep.sweep.extrude" chain
    Then the tessellated solid is closed and manifold
    And its volume equals width*height*distance within the fixture tolerance

  @id-hexagonal-mushroom-column
  @level-fundamental
  @mode-differential
  Scenario: A regular hexagon extruded to the wired height has the analytic prism volume
    Given the example "hexagonal-mushroom-column" and its committed expected-geometry fixture
    And the fixture declares kernelStatus "green"
    When the flow host evaluates its "brep.curve.polygon -> math.vector -> brep.solid.extrude" chain
    Then the tessellated solid is closed and manifold
    And its volume equals the regular-polygon area (sides/2 * radius^2 * sin(2*pi/sides)) times the height within the fixture tolerance

  @id-box-shell-preview
  @level-fundamental
  @mode-differential
  Scenario: A shelled box keeps only the material between its outer and inner cubes
    Given the example "box-shell-preview" and its committed expected-geometry fixture
    When the flow host evaluates its "brep.prim3d.box -> brep.solid.shell" chain
    Then the tessellated solid is closed and manifold
    And its volume equals size^3 minus (size - 2*thickness)^3 within the fixture tolerance
    And its bounding box is the outer cube

  @id-box-fillet-preview
  @level-fundamental
  @mode-differential
  Scenario: A box filleted on every edge measures its Minkowski rounded volume
    Given the example "box-fillet-preview" and its committed expected-geometry fixture
    And the fixture declares kernelStatus "blocked-on-fillet-kernel"
    When the flow host evaluates its "brep.prim3d.box -> brep.solid.fillet" chain
    Then the tessellated solid is closed and manifold
    And its volume equals the Minkowski sum of a (size-2r)^3 cube with a ball of radius r within the fixture tolerance
    And its bounding box is still the unfilleted cube

  @id-sphere-box-fuse
  @level-fundamental
  @mode-differential
  Scenario: A sphere fused with a box measures the union of the two solids
    Given the example "sphere-box-fuse" and its committed expected-geometry fixture
    And the fixture declares kernelStatus "green"
    When the flow host evaluates its "brep.prim3d.sphere + brep.prim3d.box -> brep.bool.fuse" chain
    Then the tessellated solid is closed and manifold
    And its volume equals the quadrature union volume within the fixture tolerance

  @id-sphere-cut-with-torus
  @level-fundamental
  @mode-differential
  Scenario: A sphere cut by a torus measures the difference of the two solids
    Given the example "sphere-cut-with-torus" and its committed expected-geometry fixture
    And the fixture declares kernelStatus "green"
    And the torus takes its major and minor radii from the packaged brep extension's declared channel defaults
    When the flow host evaluates its "brep.prim3d.sphere + brep.prim3d.torus -> brep.bool.cut -> brep.measure.volume" chain
    Then the tessellated solid is closed and manifold
    And its volume and the kernel's own brep.measure.volume both equal the quadrature difference volume within the fixture tolerance
