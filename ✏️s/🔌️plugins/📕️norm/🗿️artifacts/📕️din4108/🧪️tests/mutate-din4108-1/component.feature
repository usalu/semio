@capability-din4108-1-mutate
@no-oracle-din4108-1-mutation-semantics
@comparison-ordered-json-v1
@mutations-din4108-1-any
Feature: Apply every typed DIN 4108 mutation to its committed specification fixtures
  `s.norm.din4108` is a semio-NATIVE artifact — no third party reads or writes its
  `.dsl.semio`/`.pack.semio` envelope — so there is no reference implementation to register as an
  oracle. That is recorded as the `din4108-1-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, and it means the runner
  executes NO oracle role for this case: every assertion below lives inside the subject handler,
  which compares the applied document against the committed after-snapshot and the undone document
  against the committed before-snapshot, and fails with both documents printed. A handler that
  merely ran the mutation and returned would report a pass having checked nothing.

  `Din4108Snapshot` is seventeen document-root scalars — assembly category, climate zone,
  airtightness (n50 and class), thermal-bridge sum, indoor relative humidity and design
  temperature, catalogue and material ids, solar absorptance, design irradiance, interior and
  exterior vapour-diffusion mu values, envelope area, the Beiblatt-2 conformity flag, application
  type and declared application class — plus `layers`, an id-less ORDERED construction build-up.
  That gives seventeen `change-<field>` kinds and five collection kinds: `insert-layer` and
  `remove-layer` address by index (inserted = final-state index, removed = base-state index),
  `reorder-layers` moves one layer inside the build-up, and `change-layer-thickness` /
  `change-layer-lambda` edit one field of one layer by base-state index.

  Layer ORDER is physics here, not presentation: a construction is a sequence from inside to
  outside, and the interstitial-condensation check reads it in that sequence. The committed
  fixtures are chosen against that — `inserts-an-interior-plaster-layer-at-index-1` inserts in the
  MIDDLE rather than appending, `removes-the-load-bearing-masonry-layer` removes a non-terminal
  member, and `moves-the-insulation-in-front-of-the-masonry` is a reorder whose whole meaning is
  the position swap. An implementation that treats `layers` as an unordered set, or that appends
  on insert, matches none of the three committed after-snapshots. Together with `📘️en1990` this is
  one of only two norm vocabularies with an ordered collection at all.

  Each of the 22 kinds carries its own independently handcrafted `(before, mutation, after, diff,
  outcome)` quintet under
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`, and this
  feature re-exercises those SAME committed bytes end to end through `apply_din4108_mutation`
  rather than calling `Mutation::diff`/`inverse` directly the way the in-crate fixture tests do.
  The committed `🎯️outcome` decides which contract a row is held to: `applied` demands the
  observability law (the document must MOVE), `rejected` demands the opposite and stricter one —
  the mutation must be refused and the document must come back bit-identical. All 22 committed
  vectors declare `applied`, so every row below is held to the observability law: a kind that left
  the document bit-for-bit unchanged would fail rather than pass silently.

  The identity scenario reads the real committed DIN 4108 document at `📚️examples/🎬️demo`, not a
  fixture authored for this case. Its DSL carrier is deliberately byte-preserving — the committed
  file IS this codec's own canonical printer output, so reproducing it exactly is the correct
  answer and anything else is the defect — which is why that half of the identity law is asserted
  as `carrier_is_exact` rather than as the usual no-byte-pass-through inequality. The evidence
  that the document was genuinely PARSED rather than copied comes from the other half: the same
  snapshot is round-tripped through two further, independently written codecs — the binary
  `.pack.semio` protocol and the JSON projection — and all three must agree on one document. This
  artifact commits only the DSL encoding of its example, so the binary leg is encode-then-decode
  rather than a committed twin; that is stated here rather than papered over.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot, mutation and outcome fixture for the <id> kind
    When <id> is applied through apply_din4108_mutation
    Then the resulting document matches the committed after-snapshot fixture for <id> and honours the committed outcome status
    Examples:
      | id |
      | change-category |
      | change-climate |
      | change-airtightness-n50 |
      | change-psi-times-l-sum |
      | change-rh-int |
      | change-catalog-id |
      | change-material-id |
      | change-airtightness-class |
      | change-t-int-c |
      | change-solar-absorptance |
      | change-irradiance-wm2 |
      | change-moisture-mu-exterior |
      | change-moisture-mu-interior |
      | change-envelope-area-m2 |
      | change-bb2-details-conform |
      | change-application-type |
      | change-declared-application-class |
      | insert-layer |
      | remove-layer |
      | reorder-layers |
      | change-layer-thickness |
      | change-layer-lambda |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_din4108_mutation
    And the mutation's own computed inverse is applied through apply_din4108_mutation
    Then the document matches the committed before-snapshot fixture again
    Examples:
      | id |
      | change-category |
      | change-climate |
      | change-airtightness-n50 |
      | change-psi-times-l-sum |
      | change-rh-int |
      | change-catalog-id |
      | change-material-id |
      | change-airtightness-class |
      | change-t-int-c |
      | change-solar-absorptance |
      | change-irradiance-wm2 |
      | change-moisture-mu-exterior |
      | change-moisture-mu-interior |
      | change-envelope-area-m2 |
      | change-bb2-details-conform |
      | change-application-type |
      | change-declared-application-class |
      | insert-layer |
      | remove-layer |
      | reorder-layers |
      | change-layer-thickness |
      | change-layer-lambda |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real committed DIN 4108 document through every encoding it has
    Given the real committed text artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the text artifact is parsed, printed back to DSL and parsed again, and the same document is round-tripped through the binary pack protocol and the JSON projection
    Then the canonical DSL rendering is reproduced byte for byte and every decoding agrees on one DIN 4108 document
