@capability-gisterrain-1-mutate
@oracle-gisterrain-python-independent
@comparison-ordered-json-v1
@mutations-gisterrain-1-any
Feature: Apply both typed gis.gisterrain mutations twice — once in Rust, once in Python — and require the same answer

  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` in this directory: a
  second implementation of the `s.gis.gisterrain` document and both typed mutations, written in
  Python from `🧬️schema/📸️snapshot/🔣️.json` (the document is an `f64` `exaggeration` and a
  raw `importedFeaturesJson` string, `additionalProperties: false`),
  `🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` (the two verbs) and the two committed
  specification vectors (the externally tagged wire form of each, and the demonstration that the two
  setters move their fields independently). It imports nothing from this repository's Rust.

  Why a second implementation rather than a third-party library. Two fields survive a save here: a
  scalar and a string the artifact never parses. `geo`, `geojson` and `gdal` were surveyed by an
  earlier wave and declined — none reads `.dsl.semio`, and none is authoritative over an
  exaggeration factor or an opaque payload. What a reference can genuinely adjudicate is the two
  setters, their independence and the inverse of each.

  The real artifact, and the honest limit on it. The artifact's committed demo example is a real
  Liège survey fragment — exaggeration 1.5, an origin at 5.5818/50.603 and two named positions — but
  its `importedFeaturesJson` is EMPTY, so `change-imported-features` would replace nothing with
  something and its inverse would restore emptiness. The mutation scenarios therefore read
  local://🔣️.snapshot.json, derived ONCE by
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w16-cross-language/🐍️derive-gisterrain-imports.py`
  from committed real content only: the exaggeration and the `mesh` composed-child handle come from
  that same terrain example, and the imported payload carries the two REAL Liège positions with
  their true WGS84 coordinates, taken from the committed `gismap` demo document in this same plugin
  and rendered in the `{id, lat, lon}` descriptor shape the committed `change-imported-features`
  vector demonstrates. Nothing in it is invented.

  Why the Python reference does not read the carrier. Its `gismap` sibling's `.dsl.semio` members
  are plainly hex-encoded JSON, a layout that can be derived from the committed bytes and then
  pinned by byte-exact re-encoding. This document's only committed example carries an EMPTY
  `importedFeaturesJson`, so the encoding of a non-empty string value cannot be read off it, and no
  prose document specifies it. Guessing and calling the guess a specification is exactly what this
  exercise forbids, so the carrier's own laws stay where they can honestly be asserted: in role, on
  the Rust side, in `identity-round-trip`, against the committed example — the ArtifactDsl fixpoint
  law and agreement with the separate binary pack codec, both unchanged from before the conversion.

  The committed specification vectors were KEPT, not replaced: `spec-vector-<kind>` replays each
  handcrafted triple through both implementations, and the Rust side additionally holds the applied
  document to the committed `🔺️diff` and `🎯️outcome` and to the re-derivation of the `mesh` handle.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to the real derived Liège terrain document
    Given the real derived terrain document local://🔣️.snapshot.json
    When the <id> mutation is applied with the parameters the feature states
      """
      <mutation>
      """
    Then both implementations produce the same exaggeration and the same imported-features payload
    Examples:
      | id                       | mutation                                                                                                                                                                       |
      | change-exaggeration      | {"ChangeExaggeration":{"newExaggeration":2.75}}                                                                                                                                |
      | change-imported-features | {"ChangeImportedFeatures":{"newImportedFeaturesJson":"{\"positions\":[{\"id\":\"p_val_benoit_campus\",\"lat\":50.6231,\"lon\":5.5674}],\"routes\":[],\"regions\":[]}"}}         |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undo <id> on the real derived Liège terrain document and land back on it
    Given the real derived terrain document local://🔣️.snapshot.json
    When the <id> mutation is applied and then its own computed inverse is applied
      """
      <mutation>
      """
    Then both implementations agree on the mutated document AND on the restored one
    Examples:
      | id                       | mutation                                                                                                                                                                       |
      | change-exaggeration      | {"ChangeExaggeration":{"newExaggeration":2.75}}                                                                                                                                |
      | change-imported-features | {"ChangeImportedFeatures":{"newImportedFeaturesJson":"{\"positions\":[{\"id\":\"p_val_benoit_campus\",\"lat\":50.6231,\"lon\":5.5674}],\"routes\":[],\"regions\":[]}"}}         |

  @id-spec-vector
  @level-exhaustive
  @mode-differential
  Scenario Outline: Replay the committed <id> specification vector through both implementations
    Given the committed before-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    When the committed mutation is applied to the committed before-snapshot
    Then each implementation lands on the committed after-snapshot in role, and the two agree
    Examples:
      | id                       | dir                        | fixture                            |
      | change-exaggeration      | 🎚change-exaggeration      | raises-exaggeration-from-1-to-2-5  |
      | change-imported-features | 📥change-imported-features | imports-harbor-position-descriptor |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Read the real derived terrain document in both languages, and hold the committed carrier to its own laws in Rust
    Given the real derived terrain document local://🔣️.snapshot.json
    And the artifact's own committed carrier asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When each implementation reads the derived document, and the Rust additionally parses the committed carrier, prints it back, parses it again and cross-checks the pack codec
      """
      {"ChangeExaggeration":{"newExaggeration":1.5}}
      """
    Then both languages read the same exaggeration and imported-features payload, and the Rust printing is an ArtifactDsl fixpoint that agrees with the binary decoding
