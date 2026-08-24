@capability-en1996-1-mutate
@no-oracle-en1996-1-mutation-semantics
@comparison-ordered-json-v1
@mutations-en1996-1-any
Feature: Apply every typed EN 1996 mutation to its committed specification fixtures
  `s.norm.en1996` is a semio-NATIVE artifact — no third party reads or writes its
  `.dsl.semio`/`.pack.semio` envelope — so there is no reference implementation to register as an
  oracle. That is recorded as the `en1996-1-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, and it means the runner
  executes NO oracle role for this case: every assertion below lives inside the subject handler,
  which compares the applied document against the committed after-snapshot and the undone document
  against the committed before-snapshot, and fails with both documents printed. A handler that
  merely ran the mutation and returned would report a pass having checked nothing.

  Twenty-two document-root scalars and enums, one `change-<field>` each: the design actions M_Ed,
  N_Ed, V_Ed and H_Ed, the section properties Z, gross area and shear area, the characteristic
  strengths f_k and f_vk, the national annex, masonry class, design situation, the friction
  coefficient mu, wall thickness, required fire resistance, the unit and mortar classifications,
  bed-joint thickness, storey count and the effective height and thickness h_ef / t_ef.

  Masonry is characterised by CLASSIFICATIONS rather than by continuous properties, and this
  vocabulary keeps four of them side by side — `change-unit`, `change-mortar`,
  `change-masonry-class` and `change-exposure` — where the first two together determine f_k
  through a table lookup and the third and fourth select the partial factor. Four enum kinds in
  one document is the shape in which a lookup keyed on the wrong enum still returns a plausible
  strength, so each is committed with a whole-document after-snapshot rather than a spot check on
  the derived value. The effective-geometry pair `change-h-ef-mm` / `change-t-ef-mm` sits beside
  the physical `change-wall-thickness-mm`, and confusing effective with actual thickness is the
  classic EN 1996 slenderness defect — the fixtures keep all three separately addressable.

  Each of the 22 kinds carries its own independently handcrafted `(before, mutation, after, diff,
  outcome)` quintet under
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`, and this
  feature re-exercises those SAME committed bytes end to end through `apply_en1996_mutation`
  rather than calling `Mutation::diff`/`inverse` directly the way the in-crate fixture tests do.
  The committed `🎯️outcome` decides which contract a row is held to: `applied` demands the
  observability law (the document must MOVE), `rejected` demands the opposite and stricter one —
  the mutation must be refused and the document must come back bit-identical. All 22 committed
  vectors declare `applied`, so every row below is held to the observability law: a kind that left
  the document bit-for-bit unchanged would fail rather than pass silently.

  The identity scenario reads the real committed EN 1996 document at
  `📚️examples/📕️loadbearing-wall`, not a fixture authored for this case. Its DSL carrier is
  deliberately byte-preserving — the committed file IS this codec's own canonical printer output,
  so reproducing it exactly is the correct answer and anything else is the defect — which is why
  that half of the identity law is asserted as `carrier_is_exact` rather than as the usual
  no-byte-pass-through inequality. The evidence that the document was genuinely PARSED rather than
  copied comes from the other half: the same snapshot is round-tripped through two further,
  independently written codecs — the binary `.pack.semio` protocol and the JSON projection — and
  all three must agree on one document. The committed binary twin `🎒️loadbearing-wall.pack.semio`
  is decoded and cross-checked against the text artifact as well, so two separately committed
  files written by two separate codecs have to describe the same EN 1996 document.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot, mutation and outcome fixture for the <id> kind
    When <id> is applied through apply_en1996_mutation
    Then the resulting document matches the committed after-snapshot fixture for <id> and honours the committed outcome status
    Examples:
      | id |
      | change-m-ed-knm |
      | change-n-ed-kn |
      | change-v-ed-kn |
      | change-h-ed-kn |
      | change-z-mm3 |
      | change-area-mm2 |
      | change-shear-area-mm2 |
      | change-fk-mpa |
      | change-f-vk-mpa |
      | change-annex |
      | change-masonry-class |
      | change-design-situation |
      | change-mu |
      | change-wall-thickness-mm |
      | change-fire-resistance-min |
      | change-unit |
      | change-exposure |
      | change-mortar |
      | change-bed-joint-thickness-mm |
      | change-storeys |
      | change-h-ef-mm |
      | change-t-ef-mm |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_en1996_mutation
    And the mutation's own computed inverse is applied through apply_en1996_mutation
    Then the document matches the committed before-snapshot fixture again
    Examples:
      | id |
      | change-m-ed-knm |
      | change-n-ed-kn |
      | change-v-ed-kn |
      | change-h-ed-kn |
      | change-z-mm3 |
      | change-area-mm2 |
      | change-shear-area-mm2 |
      | change-fk-mpa |
      | change-f-vk-mpa |
      | change-annex |
      | change-masonry-class |
      | change-design-situation |
      | change-mu |
      | change-wall-thickness-mm |
      | change-fire-resistance-min |
      | change-unit |
      | change-exposure |
      | change-mortar |
      | change-bed-joint-thickness-mm |
      | change-storeys |
      | change-h-ef-mm |
      | change-t-ef-mm |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real committed EN 1996 document through every encoding it has
    Given the real committed text artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️loadbearing-wall/🖼️assets/🗣️loadbearing-wall.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️loadbearing-wall/🖼️assets/🎒️loadbearing-wall.pack.semio
    When the text artifact is parsed, printed back to DSL and parsed again, and the same document is round-tripped through the binary pack protocol and the JSON projection
    Then the canonical DSL rendering is reproduced byte for byte and every decoding agrees on one EN 1996 document
