@capability-gismap-1-mutate
@no-oracle-gismap-mutation-semantics
@comparison-ordered-json-v1
@mutations-gismap-1-any
Feature: Apply every typed gis.gismap mutation to its committed specification vector

  `s.gis.gismap` is a semio-NATIVE artifact and no third party reads `.dsl.semio` (recorded as the
  `gismap-mutation-semantics` no-oracle decision in
  `🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, which also records why `geojson`,
  `geo` and `gdal` were surveyed and DECLINED — the third of those additionally for being a system
  library with a C dependency).

  What distinguishes this subset is that its twelve kinds are four verbs over three PARALLEL
  collections, and that the parallelism is a specification, not a copy: `positions`, `routes` and
  `regions` are all `Vec<MapFeature>` and all receive the identical
  create/delete/replace-data/reorder recipe from `📓️derivation-rules.md`'s per-id-keyed-collection
  rule. A `MapFeature` is two things only — a stable `id` and an opaque `dsl::DslValue` payload the
  artifact never looks inside — so `replace-<noun>-data` swaps a whole untyped value rather than
  editing a typed geometry, and nothing here can be adjudicated by a geometry library.

  Order is meaningful. The three `reorder-` kinds address a feature by id and a target index, and
  their committed vectors deliberately exercise three different displacements: positions moves the
  LEADING feature to the end, routes moves the bus route to the FRONT, and regions moves the park
  BETWEEN the two districts — so an implementation that reordered by identity, or that only ever
  handled append and prepend, fails on one of the three.

  The sharpest structural check here is composition. `drawing` and `value` are DERIVED composed
  children whose `childId` is a digest of `(positions, routes, regions)` taken together, so editing
  any one collection must re-mint BOTH handles, and undoing must converge back onto the original two.
  The committed snapshots deliberately carry readable placeholders in those slots rather than a
  frozen digest, because `std`'s `DefaultHasher` leaves its output unspecified; the adapter therefore
  funnels every committed snapshot through the artifact's own `gis_map_snapshot_with_derived_children`
  before comparing, exactly as the in-crate fixture tests do, so the comparison stays exact rather
  than exempting the field.

  The identity round trip reads the artifact's own committed demo example — a real Liège fragment
  carrying the Institut de Botanique and Lycée Block 3000 positions with their true coordinates, two
  named routes with polyline payloads, and an empty regions collection, which is the case where a
  codec that confuses an empty collection with an absent one still parses.

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
    When <id> is applied to that vector's before-snapshot through gis_map_mutation_report_json
    Then the applied snapshot, the produced diff and the outcome's diagnostics are exactly what the vector commits, and a kind the vector declares observable really moved the projection
    Examples:
      | id                    |
      | create-position       |
      | delete-position       |
      | replace-position-data |
      | reorder-positions     |
      | create-route          |
      | delete-route          |
      | replace-route-data    |
      | reorder-routes        |
      | create-region         |
      | delete-region         |
      | replace-region-data   |
      | reorder-regions       |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot
    Given the committed <id> specification vector under 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations
    When <id> is applied and then its own computed inverse is applied through gis_map_mutation_report_json
    Then the snapshot's projection is the before-snapshot's projection again, and any divergence is reported by JSON path
    Examples:
      | id                    |
      | create-position       |
      | delete-position       |
      | replace-position-data |
      | reorder-positions     |
      | create-route          |
      | delete-route          |
      | replace-route-data    |
      | reorder-routes        |
      | create-region         |
      | delete-region         |
      | replace-region-data   |
      | reorder-regions       |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Parse the real committed map document, print it back and cross it against its binary encoding
    Given the real committed document asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the document is parsed, printed back to canonical DSL, parsed again, and separately encoded to a pack and decoded back
    Then every decoding agrees on one snapshot, and printing the canonical text a second time reproduces it byte for byte as ArtifactDsl's own fixpoint law requires
