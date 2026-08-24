@capability-fem3d-1-mutate
@no-oracle-fem3d-mutation-semantics
@comparison-ordered-json-v1
@mutations-fem3d-1-any
Feature: Apply every typed fem.fem3d mutation to its committed specification vector

  `fem.fem3d` is a semio-NATIVE artifact; `.fem3d.dsl.semio` is read by nothing outside this
  repository (recorded as the `fem3d-mutation-semantics` no-oracle decision in
  `🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, which surveys the same solver
  families as the 2D artifact and DECLINES them for the same structural reason, but is recorded
  separately because this vocabulary is genuinely a different one).

  What separates this catalog from `fem2d-1-any` is not dimensionality alone. The spatial noun here
  is a SOLID — an outline plus holes, extruded from a base elevation through a height, divided into
  layers, with a mesh size — where the 2D artifact has a planar region with a thickness. Nodes carry
  a third coordinate. Materials carry a shear modulus `g` alongside `e`, `nu` and `rho`, which the 2D
  materials do not have at all. Sections carry `iz` and a torsion constant `j` in addition to `area`
  and `iy`. Frames carry a `roll` angle about their own axis, which has no 2D counterpart. Supports
  fix six degrees of freedom (`Tx Ty Tz Rx Ry Rz`) rather than three. Loads address a solid rather
  than a region, and nodal loads name a DOF out of the full six. Combinations are written as a
  `terms:MAP` block of `case-id=factor` lines rather than the 2D `terms:LIST`.

  Twenty-five kinds in the same three shapes as the 2D artifact — eight id-keyed collections with
  `create`/`delete`, five of them with `replace`, loads nested inside a load case with
  `add-load`/`remove-load` plus `change-load-case-self-weight`, and one `update-analysis-settings`
  for the inseparable settings facet — but each vector is authored against a 3D-specific hazard.
  `replace-element` ROLLS a column about its own axis, a change that is invisible in every projection
  except the element record itself, so a diff that summarized rather than described would miss it.
  `replace-support` FREES the three rotations at a clamped base and must leave the three translations
  fixed, which a support codec that stored a boolean instead of a DOF list cannot express.
  `replace-material` softens only the shear modulus, the one material field the 2D artifact has no
  slot for. `replace-solid` thickens the slab AND adds a mesh layer, so an implementation that
  replaced geometry but kept the old layer count fails. `create-combination` is keyed by case id,
  matching this artifact's MAP-shaped combination terms rather than the 2D list.

  The identity round trip reads the artifact's own committed demo example: a real two-storey steel
  frame — four ground nodes at the corners of an 8 m by 10 m bay, columns in two lifts to 2.8 m and
  5.6 m, sixteen HEA 200 frames, a C30/37 extruded first-floor slab on its own four-node support
  patch, eight fully or partially clamped supports, a dead case with self-weight on, a live case with
  a nodal load at a column head, and a ULS combination.

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
    When <id> is applied to that vector's before-snapshot through fem3d_mutation_report_json
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
      | create-solid                 |
      | delete-solid                 |
      | replace-solid                |
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
    When <id> is applied and then its own computed inverse is applied through fem3d_mutation_report_json
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
      | create-solid                 |
      | delete-solid                 |
      | replace-solid                |
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
  Scenario: Parse the real committed 3D structural model document, print it back and cross it against its binary encoding
    Given the real committed document asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the document is parsed, printed back to canonical DSL, parsed again, and separately encoded to a pack and decoded back
    Then every decoding agrees on one snapshot, and printing the canonical text a second time reproduces it byte for byte as ArtifactDsl's own fixpoint law requires
