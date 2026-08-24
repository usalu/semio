@capability-fem2d-1-mutate
@no-oracle-fem2d-mutation-semantics
@comparison-ordered-json-v1
@mutations-fem2d-1-any
Feature: Apply every typed fem.fem2d mutation to its committed specification vector

  `fem.fem2d` is a semio-NATIVE artifact; `.fem2d.dsl.semio` is read by nothing outside this
  repository (recorded as the `fem2d-mutation-semantics` no-oracle decision in
  `🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, which also records why `frame3dd`,
  `calculix` and `code_aster` were surveyed and DECLINED — they are solvers with no editing
  vocabulary — and why IFC's structural-analysis view does not map onto these 25 kinds).

  This vocabulary is NOT the 3D artifact's vocabulary with the z-coordinate removed, and the catalog
  is recorded separately for that reason. The 2D artifact's planar noun is a REGION: an outline plus
  holes, a thickness, a material and a mesh size, meshed in plane. Its nodes are `(x, y)`. Its
  materials carry `e`, `nu` and `rho` and no shear modulus. Its sections carry `area` and `iy` and
  neither `iz` nor a torsion constant. Its member loads are a `(wx, wy)` pair rather than a
  six-degree-of-freedom nodal vector, and its combinations are written as a `terms:LIST` of
  `case-id`/`factor` pairs. The committed demo document even writes quantities WITH UNITS —
  `210000000000Pa`, `7850kg/m3`, `0.001m2`, `0m` — where the 3D grammar writes bare numbers.

  Twenty-five kinds fall into three shapes. Eight id-keyed collections get `create`/`delete`, five of
  them additionally get `replace` (an in-place restatement that must keep the element in its original
  slot rather than removing and appending it). Loads are the exception: they live INSIDE a load case,
  not in a top-level collection, so they get `add-load`/`remove-load` addressed through the owning
  case, and the case itself gets `change-load-case-self-weight` for the one boolean that changes what
  the solver adds without changing what the document lists. Finally `update-analysis-settings` writes
  the single inseparable settings facet — modal count, buckling count and deformation scale together.

  The committed vectors are chosen against cascade hazards rather than against the easy cases.
  `delete-load-case` removes the live case TOGETHER with its loads, so an implementation that
  detached the case and orphaned the loads fails. `delete-node` removes node n3 WITHOUT cascading to
  its support, which is the opposite rule, and an implementation that applied one policy everywhere
  fails one of the two. `delete-material` removes the unreferenced timber material only, so a
  reference check that never ran shows up. `replace-region` punches a stair opening through the slab,
  which changes the holes list without touching the outline. `replace-element` converts beam e1 into a
  bar IN PLACE, so an implementation that removed and re-appended lands with the right set and the
  wrong order.

  The identity round trip reads the artifact's own committed demo example: a real timber-frame house
  — a steel foundation column, timber posts, floor beams and rafters over an 8 m span, three
  materials, four sections, four supports, a dead case with self-weight on carrying an area pressure
  over the first-floor slab, a live case, and a ULS combination at 1.35/1.5.

  Where the assertions live. This case records a no-oracle decision, so the runner dispatches NO
  oracle role at all: `oracleDecision` resolves an oracle implementation from an `@oracle-` tag, this
  feature has none, and the comparison profile therefore never receives two sides to compare. Every
  law below is asserted INSIDE the adapter's handler, through the shared law module
  `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️component.rs` that the stdio subsets use — `divergence` for
  a path-named first difference, `mutation_is_observable` for the forward law, `inverse_restores` for
  the inverse law, `round_trip_preserves` and `carrier_is_exact` for the identity law. A handler that
  applied the mutation and returned would report a pass having checked nothing, which is exactly the
  failure this platform exists to prevent.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> and land on the committed after-snapshot, diff and outcome
    Given the committed <id> specification vector under 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations
    When <id> is applied to that vector's before-snapshot through fem2d_mutation_report_json
    Then the applied snapshot, the produced diff and the outcome's diagnostics are exactly what the vector commits, and a kind the vector declares observable really moved the projection
    Examples:
      | id                           |
      | create-node                  |
      | delete-node                  |
      | create-element               |
      | delete-element               |
      | replace-element              |
      | create-material              |
      | delete-material              |
      | replace-material             |
      | create-section               |
      | delete-section               |
      | replace-section              |
      | create-support               |
      | delete-support               |
      | replace-support              |
      | create-region                |
      | delete-region                |
      | replace-region               |
      | create-load-case             |
      | delete-load-case             |
      | add-load                     |
      | remove-load                  |
      | change-load-case-self-weight |
      | create-combination           |
      | delete-combination           |
      | update-analysis-settings     |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed <id> specification vector under 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations
    When <id> is applied and then its own computed inverse is applied through fem2d_mutation_report_json
    Then the snapshot's projection is the before-snapshot's projection again, and any divergence is reported by JSON path
    Examples:
      | id                           |
      | create-node                  |
      | delete-node                  |
      | create-element               |
      | delete-element               |
      | replace-element              |
      | create-material              |
      | delete-material              |
      | replace-material             |
      | create-section               |
      | delete-section               |
      | replace-section              |
      | create-support               |
      | delete-support               |
      | replace-support              |
      | create-region                |
      | delete-region                |
      | replace-region               |
      | create-load-case             |
      | delete-load-case             |
      | add-load                     |
      | remove-load                  |
      | change-load-case-self-weight |
      | create-combination           |
      | delete-combination           |
      | update-analysis-settings     |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed 2D structural model document, print it back and cross it against its binary encoding
    Given the real committed document asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the document is parsed, printed back to canonical DSL, parsed again, and separately encoded to a pack and decoded back
    Then every decoding agrees on one snapshot, and printing the canonical text a second time reproduces it byte for byte as ArtifactDsl's own fixpoint law requires
