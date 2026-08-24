@capability-gisterrain-1-mutate
@no-oracle-gisterrain-mutation-semantics
@comparison-ordered-json-v1
@mutations-gisterrain-1-any
Feature: Apply both typed gis.gisterrain mutations to their committed specification vectors

  `s.gis.gisterrain` is a semio-NATIVE artifact and nothing outside this repository reads
  `.dsl.semio` (recorded as the `gisterrain-mutation-semantics` no-oracle decision in
  `🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, which also records why `geo`,
  `geojson` and `gdal` were surveyed and DECLINED).

  What distinguishes this subset is how narrow its persisted state is and how indirect its mutations
  are. Exactly two fields survive a save: an `f64` vertical `exaggeration` and a raw
  `imported_features_json` string that is the `map:in` port's insertion point and that the artifact
  never interprets. A third slot, `mesh`, is not persisted content at all but a CONTENT-ADDRESSED
  child handle whose `childId` is derived from those two fields alone. So both declared kinds are
  root-scalar setters whose visible effect is second-order: changing either field must re-derive the
  mesh handle, and every constructor in the subset — `Default`, `apply_gis_terrain_mutation` and
  `GisTerrainDiff::apply` alike — re-derives it for exactly that reason. An implementation that wrote
  the field but forgot the handle leaves a stale digest behind, and the inverse law catches it in the
  other direction too: undoing must converge back onto the original handle, not merely the original
  field.

  The two committed vectors are chosen to move the two fields independently.
  `raises-the-exaggeration-from-one-to-two-and-a-half` moves the numeric field and leaves the JSON
  string alone; `imports-a-single-harbor-position-descriptor` moves the string and leaves the number
  alone. Neither is a round number pair, so a setter that clamped or defaulted shows up.

  The identity round trip reads the artifact's own committed demo example, a real Liège survey
  fragment: exaggeration 1.5, an origin at 5.5818/50.603, and two named positions — the Institut de
  Botanique and Lycée Block 3000 — carried as `position` lines. Note that those `origin`/`position`
  lines are NOT fields of `GisTerrainSnapshot`, which persists only the two scalars and the mesh
  handle; if this subset's `parse_dsl` rejects them the identity scenario fails, and that failure is
  a genuine finding about a stale committed example rather than something to route around.

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
    When <id> is applied to that vector's before-snapshot through gis_terrain_mutation_report_json
    Then the applied snapshot, the produced diff and the outcome's diagnostics are exactly what the vector commits, and a kind the vector declares observable really moved the projection
    Examples:
      | id                       |
      | change-exaggeration      |
      | change-imported-features |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed <id> specification vector under 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations
    When <id> is applied and then its own computed inverse is applied through gis_terrain_mutation_report_json
    Then the snapshot's projection is the before-snapshot's projection again, and any divergence is reported by JSON path
    Examples:
      | id                       |
      | change-exaggeration      |
      | change-imported-features |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed terrain document, print it back and cross it against its binary encoding
    Given the real committed document asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the document is parsed, printed back to canonical DSL, parsed again, and separately encoded to a pack and decoded back
    Then every decoding agrees on one snapshot, and printing the canonical text a second time reproduces it byte for byte as ArtifactDsl's own fixpoint law requires
