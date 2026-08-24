@capability-en1997-1-mutate
@no-oracle-en1997-1-mutation-semantics
@comparison-ordered-json-v1
@mutations-en1997-1-any
Feature: Apply every typed EN 1997 mutation to its committed specification fixtures
  `s.norm.en1997` is a semio-NATIVE artifact — no third party reads or writes its
  `.dsl.semio`/`.pack.semio` envelope — so there is no reference implementation to register as an
  oracle. That is recorded as the `en1997-1-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, and it means the runner
  executes NO oracle role for this case: every assertion below lives inside the subject handler,
  which compares the applied document against the committed after-snapshot and the undone document
  against the committed before-snapshot, and fails with both documents printed. A handler that
  merely ran the mutation and returned would report a pass having checked nothing.

  Twenty-two document-root scalars and enums, one `change-<field>` each, covering two independent
  check families that share one ground model. The shallow-footing family carries the actions V_Ed
  and H_Ed, the footing area, the ground parameters phi, c and gamma, the footing width B, the
  embedment depth D_f, the stiffness E_s and Poisson's ratio nu, the settlement limit, and the
  design approach. The pile family carries N_Ed, the shaft factor alpha_s, the pile diameter and
  length, the shaft and base resistances q_s and q_b, the base area, the profile count and the
  investigated depth. `change-annex` and `change-design-approach` sit above both.

  EN 1997's Design Approaches 1, 2 and 3 apply partial factors at DIFFERENT points — to actions,
  to resistances, or to ground properties — so `change-design-approach` is the one mutation in
  this vocabulary that changes the meaning of every other field without touching any of them. It
  is exercised as a whole-document comparison here for exactly that reason. The second thing this
  vocabulary is exposed to is family confusion: `change-b-m` (footing width) beside
  `change-pile-d-m` (pile diameter), `change-footing-area-m2` beside `change-pile-base-area-m2`,
  and `change-v-ed-kn` beside `change-n-pile-ed-kn` are pairs where a diff builder wired to the
  sibling still produces a number the check will happily consume.

  Each of the 22 kinds carries its own independently handcrafted `(before, mutation, after, diff,
  outcome)` quintet under
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`, and this
  feature re-exercises those SAME committed bytes end to end through `apply_en1997_mutation`
  rather than calling `Mutation::diff`/`inverse` directly the way the in-crate fixture tests do.
  The committed `🎯️outcome` decides which contract a row is held to: `applied` demands the
  observability law (the document must MOVE), `rejected` demands the opposite and stricter one —
  the mutation must be refused and the document must come back bit-identical. All 22 committed
  vectors declare `applied`, so every row below is held to the observability law: a kind that left
  the document bit-for-bit unchanged would fail rather than pass silently.

  The identity scenario reads the real committed EN 1997 document at `📚️examples/🎬️demo`, not a
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
    When <id> is applied through apply_en1997_mutation
    Then the resulting document matches the committed after-snapshot fixture for <id> and honours the committed outcome status
    Examples:
      | id |
      | change-v-ed-kn |
      | change-h-ed-kn |
      | change-footing-area-m2 |
      | change-phi-deg |
      | change-c-kpa |
      | change-gamma-kn-m3 |
      | change-bm |
      | change-dfm |
      | change-es-mpa |
      | change-nu |
      | change-design-approach |
      | change-annex |
      | change-settlement-limit-mm |
      | change-n-pile-ed-kn |
      | change-alpha-s |
      | change-pile-dm |
      | change-qs-kpa |
      | change-pile-lm |
      | change-qb-kpa |
      | change-pile-base-area-m2 |
      | change-pile-n-profiles |
      | change-z-investigated-m |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_en1997_mutation
    And the mutation's own computed inverse is applied through apply_en1997_mutation
    Then the document matches the committed before-snapshot fixture again
    Examples:
      | id |
      | change-v-ed-kn |
      | change-h-ed-kn |
      | change-footing-area-m2 |
      | change-phi-deg |
      | change-c-kpa |
      | change-gamma-kn-m3 |
      | change-bm |
      | change-dfm |
      | change-es-mpa |
      | change-nu |
      | change-design-approach |
      | change-annex |
      | change-settlement-limit-mm |
      | change-n-pile-ed-kn |
      | change-alpha-s |
      | change-pile-dm |
      | change-qs-kpa |
      | change-pile-lm |
      | change-qb-kpa |
      | change-pile-base-area-m2 |
      | change-pile-n-profiles |
      | change-z-investigated-m |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real committed EN 1997 document through every encoding it has
    Given the real committed text artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the text artifact is parsed, printed back to DSL and parsed again, and the same document is round-tripped through the binary pack protocol and the JSON projection
    Then the canonical DSL rendering is reproduced byte for byte and every decoding agrees on one EN 1997 document
