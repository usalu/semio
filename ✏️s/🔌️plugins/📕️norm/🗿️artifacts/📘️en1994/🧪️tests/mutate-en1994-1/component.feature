@capability-en1994-1-mutate
@no-oracle-en1994-1-mutation-semantics
@comparison-ordered-json-v1
@mutations-en1994-1-any
Feature: Apply every typed EN 1994 mutation to its committed specification fixtures
  `s.norm.en1994` is a semio-NATIVE artifact — no third party reads or writes its
  `.dsl.semio`/`.pack.semio` envelope — so there is no reference implementation to register as an
  oracle. That is recorded as the `en1994-1-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, and it means the runner
  executes NO oracle role for this case: every assertion below lives inside the subject handler,
  which compares the applied document against the committed after-snapshot and the undone document
  against the committed before-snapshot, and fails with both documents printed. A handler that
  merely ran the mutation and returned would report a pass having checked nothing.

  Twenty-two document-root scalars, one `change-<field>` each: national annex, the design actions
  M_Ed and V_Ed, the plastic-resistance pair M_pl,a / M_pl,Rd with the degree-of-connection eta
  and the longitudinal shear resistance V_L,Rd, the fire inputs (insulation thickness, rating,
  deck type), the fatigue inputs (stress range and detail category), and the stud-connector set —
  shank diameter, stud height, f_ck, f_u, E_cm, the per-stud design shear, span, f_y, cycle count
  and the stud stress range.

  Composite design is where a steel quantity and a concrete quantity sit next to each other under
  similar names, and this vocabulary keeps both: `change-f-ck-mpa` (concrete) beside
  `change-f-y-mpa` and `change-f-u-mpa` (steel), `change-e-cm-mpa` (concrete secant modulus)
  beside them, and `change-d-mm` (stud shank) beside `change-h-sc-mm` (stud height) — the two
  geometric inputs to the same push-out resistance formula, where swapping them still yields a
  number. Unlike its `📘️en1993` sibling this artifact takes NO `update-<facet>` grouping: its own
  module header records that none of the twenty-two fields forms a set that is never meaningfully
  set one field at a time, so the exception was not invented for it. The committed example is a
  composite bridge girder, which is why the fatigue and stud-connector fields carry real values
  rather than defaults.

  Each of the 22 kinds carries its own independently handcrafted `(before, mutation, after, diff,
  outcome)` quintet under
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`, and this
  feature re-exercises those SAME committed bytes end to end through `apply_en1994_mutation`
  rather than calling `Mutation::diff`/`inverse` directly the way the in-crate fixture tests do.
  The committed `🎯️outcome` decides which contract a row is held to: `applied` demands the
  observability law (the document must MOVE), `rejected` demands the opposite and stricter one —
  the mutation must be refused and the document must come back bit-identical. All 22 committed
  vectors declare `applied`, so every row below is held to the observability law: a kind that left
  the document bit-for-bit unchanged would fail rather than pass silently.

  The identity scenario reads the real committed EN 1994 document at
  `📚️examples/📕️composite-bridge-girder`, not a fixture authored for this case. Its DSL carrier is
  deliberately byte-preserving — the committed file IS this codec's own canonical printer output,
  so reproducing it exactly is the correct answer and anything else is the defect — which is why
  that half of the identity law is asserted as `carrier_is_exact` rather than as the usual
  no-byte-pass-through inequality. The evidence that the document was genuinely PARSED rather than
  copied comes from the other half: the same snapshot is round-tripped through two further,
  independently written codecs — the binary `.pack.semio` protocol and the JSON projection — and
  all three must agree on one document. The committed binary twin
  `🎒️composite-bridge-girder.pack.semio` is decoded and cross-checked against the text artifact as
  well, so two separately committed files written by two separate codecs have to describe the same
  EN 1994 document.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot, mutation and outcome fixture for the <id> kind
    When <id> is applied through apply_en1994_mutation
    Then the resulting document matches the committed after-snapshot fixture for <id> and honours the committed outcome status
    Examples:
      | id |
      | change-annex |
      | change-m-ed-knm |
      | change-v-ed-kn |
      | change-m-pla |
      | change-m-pl-rd |
      | change-eta |
      | change-vl-rd |
      | change-insulation-thickness-mm |
      | change-fire-rating |
      | change-deck-type |
      | change-delta-sigma-mpa |
      | change-fatigue-detail |
      | change-d-mm |
      | change-h-sc-mm |
      | change-f-ck-mpa |
      | change-fu-mpa |
      | change-e-cm-mpa |
      | change-v-ed-per-stud-kn |
      | change-span-m |
      | change-fy-mpa |
      | change-n-cycles-stud |
      | change-delta-tau-stud-mpa |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_en1994_mutation
    And the mutation's own computed inverse is applied through apply_en1994_mutation
    Then the document matches the committed before-snapshot fixture again
    Examples:
      | id |
      | change-annex |
      | change-m-ed-knm |
      | change-v-ed-kn |
      | change-m-pla |
      | change-m-pl-rd |
      | change-eta |
      | change-vl-rd |
      | change-insulation-thickness-mm |
      | change-fire-rating |
      | change-deck-type |
      | change-delta-sigma-mpa |
      | change-fatigue-detail |
      | change-d-mm |
      | change-h-sc-mm |
      | change-f-ck-mpa |
      | change-fu-mpa |
      | change-e-cm-mpa |
      | change-v-ed-per-stud-kn |
      | change-span-m |
      | change-fy-mpa |
      | change-n-cycles-stud |
      | change-delta-tau-stud-mpa |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real committed EN 1994 document through every encoding it has
    Given the real committed text artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️composite-bridge-girder/🖼️assets/🗣️composite-bridge-girder.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️composite-bridge-girder/🖼️assets/🎒️composite-bridge-girder.pack.semio
    When the text artifact is parsed, printed back to DSL and parsed again, and the same document is round-tripped through the binary pack protocol and the JSON projection
    Then the canonical DSL rendering is reproduced byte for byte and every decoding agrees on one EN 1994 document
