@capability-procedural-3d-1-example-geometry
@oracle-example-geometry-parry3d
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

  `scipy`/`numpy` (Python) re-derive what each op chain's solid must measure, from the example's own
  DSL and from the packaged brep extension descriptor's declared channel defaults, and from nothing
  else in this repository. Six of the eight are closed form (rectangular prism, regular-polygon
  prism, hollow box, rounded box as the Minkowski sum of a shrunk cube with a ball); two have no
  closed form and are integrated: the ball-cube union by quadrature over clipped disc areas, and the
  ball-minus-torus difference by quadrature over the torus tube angle. The TypeScript lane repeats
  both integrals with its own composite-Simpson rule, so the two numbers rest on two independent
  quadratures rather than on one library.

  ⚠️ TWO EXAMPLES ARE KNOWN-BLOCKED, and their fixtures say so in a machine-readable field rather
  than in prose. `🧲️sphere-box-fuse` and `🍩️sphere-cut-with-torus` sit on `brep.bool.fuse`/
  `brep.bool.cut`, the exact-imprint boolean path that ticket
  `26/09/08/COMPOSABLE-STDIO-ARTIFACT-PACKAGES` records as carrying real failures. Their fixtures
  carry `kernelStatus: "blocked-on-boolean-kernel"`; the expected volumes are nonetheless stated to
  full precision, so the day the kernel lands the gate is already written.

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
    When the flow host evaluates its "brep.curve.rectangle -> math.vector -> brep.solid.extrude -> brep.measure.volume" chain
    Then the tessellated solid is closed and manifold
    And its divergence-theorem volume, the parry3d volume and the kernel's own brep.measure.volume all equal width*height*distance within the fixture tolerance
    And its bounding box spans width by height by distance

  @id-face-sweep-extrude
  @level-fundamental
  @mode-differential
  Scenario: A planar face swept along a wired vector has the analytic prism volume
    Given the example "face-sweep-extrude" and its committed expected-geometry fixture
    When the flow host evaluates its "brep.curve.rectangle -> brep.surf.planarFaceWire -> math.vector -> brep.sweep.extrude" chain
    Then the tessellated solid is closed and manifold
    And its volume equals width*height*distance within the fixture tolerance

  @id-hexagonal-mushroom-column
  @level-fundamental
  @mode-differential
  Scenario: A regular hexagon extruded to the wired height has the analytic prism volume
    Given the example "hexagonal-mushroom-column" and its committed expected-geometry fixture
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
    When the flow host evaluates its "brep.prim3d.box -> brep.solid.fillet" chain
    Then the tessellated solid is closed and manifold
    And its volume equals the Minkowski sum of a (size-2r)^3 cube with a ball of radius r within the fixture tolerance
    And its bounding box is still the unfilleted cube

  @id-sphere-box-fuse
  @level-fundamental
  @mode-differential
  Scenario: A sphere fused with a box measures the union of the two solids
    Given the example "sphere-box-fuse" and its committed expected-geometry fixture
    And the fixture declares kernelStatus "blocked-on-boolean-kernel"
    When the flow host evaluates its "brep.prim3d.sphere + brep.prim3d.box -> brep.bool.fuse" chain
    Then the tessellated solid is closed and manifold
    And its volume equals the quadrature union volume within the fixture tolerance

  @id-sphere-cut-with-torus
  @level-fundamental
  @mode-differential
  Scenario: A sphere cut by a torus measures the difference of the two solids
    Given the example "sphere-cut-with-torus" and its committed expected-geometry fixture
    And the fixture declares kernelStatus "blocked-on-boolean-kernel"
    And the torus takes its major and minor radii from the packaged brep extension's declared channel defaults
    When the flow host evaluates its "brep.prim3d.sphere + brep.prim3d.torus -> brep.bool.cut -> brep.measure.volume" chain
    Then the tessellated solid is closed and manifold
    And its volume and the kernel's own brep.measure.volume both equal the quadrature difference volume within the fixture tolerance
