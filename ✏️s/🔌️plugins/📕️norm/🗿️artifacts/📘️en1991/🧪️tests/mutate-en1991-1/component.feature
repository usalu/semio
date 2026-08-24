@capability-en1991-1-mutate
@no-oracle-en1991-1-mutation-semantics
@comparison-ordered-json-v1
@mutations-en1991-1-any
Feature: Apply every typed EN 1991 mutation to its committed specification fixtures
  `s.norm.en1991` is a semio-NATIVE artifact — no third party reads or writes its
  `.dsl.semio`/`.pack.semio` envelope — so there is no reference implementation to register as an
  oracle. That is recorded as the `en1991-1-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, and it means the runner
  executes NO oracle role for this case: every assertion below lives inside the subject handler,
  which compares the applied document against the committed after-snapshot and the undone document
  against the committed before-snapshot, and fails with both documents printed. A handler that
  merely ran the mutation and returned would report a pass having checked nothing.

  Thirty-two document-root scalars, one `change-<field>` each, spanning the whole of Eurocode 1:
  loaded area and imposed-load category, national annex, self-weight (material and layer
  thickness, plus an assumed characteristic value), fire (curve, required resistance, member
  capacity), snow (zone, altitude, characteristic load), wind (zone, basic speed), thermal delta
  T, construction activity, accidental impact (vehicle mass and speed), bridge traffic (notional
  lanes, span, lane width, moment resistance), crane and hoist classes with hoisting speed, silo
  bulk material (density, height, hydraulic radius, wall friction mu, lateral pressure ratio K)
  and the size and dynamic factors c_s and c_d.

  This vocabulary is the one whose SPELLING is load-bearing, and its own module header says why:
  the derive's `to_kebab` merges adjacent all-caps runs when no lowercase letter anchors a word
  boundary, so `ChangeEnVBMS` becomes `change-en-vbms` and not `change-en-v-b-m-s`,
  `ChangeEnSKKnM2` becomes `change-en-sk-kn-m2`, `ChangeCS` becomes `change-cs` and `ChangeCD`
  becomes `change-cd` — while the payload's own Rust field still addresses `en_v_b_m_s`. The
  catalog beside this feature is generated from each leaf's own `SemanticDescriptor.kind`, not
  from a hand-transliteration of the variant name, and `kinds_match_the_enum_and_the_catalog`
  fails the moment those two spellings part company.
    The committed fixture chosen for the identity scenario is a REAL retail hydrocarbon-fire case,
    not a default document: `switches-fire-curve-to-hydrocarbon` and
    `extends-fire-resistance-to-120-min` are the same design decision the example asset records.

  Each of the 32 kinds carries its own independently handcrafted `(before, mutation, after, diff,
  outcome)` quintet under
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`, and this
  feature re-exercises those SAME committed bytes end to end through `apply_en1991_mutation`
  rather than calling `Mutation::diff`/`inverse` directly the way the in-crate fixture tests do.
  The committed `🎯️outcome` decides which contract a row is held to: `applied` demands the
  observability law (the document must MOVE), `rejected` demands the opposite and stricter one —
  the mutation must be refused and the document must come back bit-identical. All 32 committed
  vectors declare `applied`, so every row below is held to the observability law: a kind that left
  the document bit-for-bit unchanged would fail rather than pass silently.

  The identity scenario reads the real committed EN 1991 document at
  `📚️examples/📕️retail-hydrocarbon-fire`, not a fixture authored for this case. Its DSL carrier is
  deliberately byte-preserving — the committed file IS this codec's own canonical printer output,
  so reproducing it exactly is the correct answer and anything else is the defect — which is why
  that half of the identity law is asserted as `carrier_is_exact` rather than as the usual
  no-byte-pass-through inequality. The evidence that the document was genuinely PARSED rather than
  copied comes from the other half: the same snapshot is round-tripped through two further,
  independently written codecs — the binary `.pack.semio` protocol and the JSON projection — and
  all three must agree on one document. The committed binary twin
  `🎒️retail-hydrocarbon-fire.pack.semio` is decoded and cross-checked against the text artifact as
  well, so two separately committed files written by two separate codecs have to describe the same
  EN 1991 document.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot, mutation and outcome fixture for the <id> kind
    When <id> is applied through apply_en1991_mutation
    Then the resulting document matches the committed after-snapshot fixture for <id> and honours the committed outcome status
    Examples:
      | id |
      | change-area-m2 |
      | change-category |
      | change-annex |
      | change-self-weight-material |
      | change-self-weight-thickness-m |
      | change-assumed-gk-kn-m2 |
      | change-fire-curve |
      | change-fire-resistance-min |
      | change-fire-member-capacity-c |
      | change-snow-zone |
      | change-snow-altitude-m |
      | change-en-sk-kn-m2 |
      | change-wind-zone |
      | change-en-vbms |
      | change-delta-tk |
      | change-construction-activity |
      | change-accidental-mass-t |
      | change-accidental-speed-km-h |
      | change-bridge-lane |
      | change-bridge-span-m |
      | change-bridge-lane-width-m |
      | change-bridge-moment-resistance-knm |
      | change-crane-class |
      | change-hoist-class |
      | change-hoisting-speed-ms |
      | change-silo-bulk-density-kn-m3 |
      | change-silo-height-m |
      | change-silo-hydraulic-radius-m |
      | change-silo-mu |
      | change-silo-k |
      | change-cs |
      | change-cd |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_en1991_mutation
    And the mutation's own computed inverse is applied through apply_en1991_mutation
    Then the document matches the committed before-snapshot fixture again
    Examples:
      | id |
      | change-area-m2 |
      | change-category |
      | change-annex |
      | change-self-weight-material |
      | change-self-weight-thickness-m |
      | change-assumed-gk-kn-m2 |
      | change-fire-curve |
      | change-fire-resistance-min |
      | change-fire-member-capacity-c |
      | change-snow-zone |
      | change-snow-altitude-m |
      | change-en-sk-kn-m2 |
      | change-wind-zone |
      | change-en-vbms |
      | change-delta-tk |
      | change-construction-activity |
      | change-accidental-mass-t |
      | change-accidental-speed-km-h |
      | change-bridge-lane |
      | change-bridge-span-m |
      | change-bridge-lane-width-m |
      | change-bridge-moment-resistance-knm |
      | change-crane-class |
      | change-hoist-class |
      | change-hoisting-speed-ms |
      | change-silo-bulk-density-kn-m3 |
      | change-silo-height-m |
      | change-silo-hydraulic-radius-m |
      | change-silo-mu |
      | change-silo-k |
      | change-cs |
      | change-cd |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real committed EN 1991 document through every encoding it has
    Given the real committed text artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️retail-hydrocarbon-fire/🖼️assets/🗣️retail-hydrocarbon-fire.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️retail-hydrocarbon-fire/🖼️assets/🎒️retail-hydrocarbon-fire.pack.semio
    When the text artifact is parsed, printed back to DSL and parsed again, and the same document is round-tripped through the binary pack protocol and the JSON projection
    Then the canonical DSL rendering is reproduced byte for byte and every decoding agrees on one EN 1991 document
