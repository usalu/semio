@capability-en1993-1-mutate
@no-oracle-en1993-1-mutation-semantics
@comparison-ordered-json-v1
@mutations-en1993-1-any
Feature: Apply every typed EN 1993 mutation to its committed specification fixtures
  `s.norm.en1993` is a semio-NATIVE artifact — no third party reads or writes its
  `.dsl.semio`/`.pack.semio` envelope — so there is no reference implementation to register as an
  oracle. That is recorded as the `en1993-1-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, and it means the runner
  executes NO oracle role for this case: every assertion below lives inside the subject handler,
  which compares the applied document against the committed after-snapshot and the undone document
  against the committed before-snapshot, and fails with both documents printed. A handler that
  merely ran the mutation and returned would report a pass having checked nothing.

  The ONE norm vocabulary that is not a parameter form. `En1993Snapshot` carries 74 scalar fields,
  yet declares only seventeen mutations: `change-annex` for the lone document-identity scalar, and
  sixteen `update-<family>-inputs` kinds — member properties, fire, cold-formed, stainless,
  plated, silo shell, bolt, weld, fatigue, through-thickness, tension component, HSS, bridge,
  tower, pile and crane. The grouping is not editorial: `⚙️engine`'s `check_full_steel_member` has
  one region per EN 1993 part, each calling exactly one check function with exactly that part's
  fields, and the mutation families are those argument sets.

  This is the only place in the plugin where the derivation rules' `update-<facet>` exception is
  applied at scale, and the fixtures are written to prove the grouping is real rather than
  convenient. `moves-the-connection-to-four-m24-grade-10-9-bolts` changes bolt count, diameter and
  grade in ONE mutation, because `bolt_e1_mm` alone means nothing without `bolt_e2_mm` and
  `bolt_d0_mm`; `thickens-the-cold-formed-flange-and-reverses-its-stress-gradient` and
  `upsizes-the-stainless-section-to-a-duplex-grade` do the same for their parts.
  `update-silo-shell-inputs` is the deliberate exception to the one-part-one-mutation rule:
  `silo_t_mm` and `silo_r_mm` are read by both the part 1-6 shell-buckling check and the part 4
  silo-wall check because they describe ONE physical silo, so they live in one group rather than
  being duplicated into two. Seventeen whole-document comparisons is therefore also a check that
  no group has quietly grown a field belonging to another part.

  Each of the 17 kinds carries its own independently handcrafted `(before, mutation, after, diff,
  outcome)` quintet under
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`, and this
  feature re-exercises those SAME committed bytes end to end through `apply_en1993_mutation`
  rather than calling `Mutation::diff`/`inverse` directly the way the in-crate fixture tests do.
  The committed `🎯️outcome` decides which contract a row is held to: `applied` demands the
  observability law (the document must MOVE), `rejected` demands the opposite and stricter one —
  the mutation must be refused and the document must come back bit-identical. All 17 committed
  vectors declare `applied`, so every row below is held to the observability law: a kind that left
  the document bit-for-bit unchanged would fail rather than pass silently.

  The identity scenario reads the real committed EN 1993 document at
  `📚️examples/📕️high-strength-connection`, not a fixture authored for this case. Its DSL carrier
  is deliberately byte-preserving — the committed file IS this codec's own canonical printer
  output, so reproducing it exactly is the correct answer and anything else is the defect — which
  is why that half of the identity law is asserted as `carrier_is_exact` rather than as the usual
  no-byte-pass-through inequality. The evidence that the document was genuinely PARSED rather than
  copied comes from the other half: the same snapshot is round-tripped through two further,
  independently written codecs — the binary `.pack.semio` protocol and the JSON projection — and
  all three must agree on one document. The committed binary twin
  `🎒️high-strength-connection.pack.semio` is decoded and cross-checked against the text artifact
  as well, so two separately committed files written by two separate codecs have to describe the
  same EN 1993 document.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot, mutation and outcome fixture for the <id> kind
    When <id> is applied through apply_en1993_mutation
    Then the resulting document matches the committed after-snapshot fixture for <id> and honours the committed outcome status
    Examples:
      | id |
      | change-annex |
      | update-member-properties |
      | update-fire-inputs |
      | update-cold-formed-inputs |
      | update-stainless-inputs |
      | update-plated-inputs |
      | update-silo-shell-inputs |
      | update-bolt-inputs |
      | update-weld-inputs |
      | update-fatigue-inputs |
      | update-through-thickness-inputs |
      | update-tension-component-inputs |
      | update-hss-inputs |
      | update-bridge-inputs |
      | update-tower-inputs |
      | update-pile-inputs |
      | update-crane-inputs |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_en1993_mutation
    And the mutation's own computed inverse is applied through apply_en1993_mutation
    Then the document matches the committed before-snapshot fixture again
    Examples:
      | id |
      | change-annex |
      | update-member-properties |
      | update-fire-inputs |
      | update-cold-formed-inputs |
      | update-stainless-inputs |
      | update-plated-inputs |
      | update-silo-shell-inputs |
      | update-bolt-inputs |
      | update-weld-inputs |
      | update-fatigue-inputs |
      | update-through-thickness-inputs |
      | update-tension-component-inputs |
      | update-hss-inputs |
      | update-bridge-inputs |
      | update-tower-inputs |
      | update-pile-inputs |
      | update-crane-inputs |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real committed EN 1993 document through every encoding it has
    Given the real committed text artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️high-strength-connection/🖼️assets/🗣️high-strength-connection.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️high-strength-connection/🖼️assets/🎒️high-strength-connection.pack.semio
    When the text artifact is parsed, printed back to DSL and parsed again, and the same document is round-tripped through the binary pack protocol and the JSON projection
    Then the canonical DSL rendering is reproduced byte for byte and every decoding agrees on one EN 1993 document
