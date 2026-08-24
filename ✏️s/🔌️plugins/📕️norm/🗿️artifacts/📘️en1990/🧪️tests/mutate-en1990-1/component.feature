@capability-en1990-1-mutate
@no-oracle-en1990-1-mutation-semantics
@comparison-ordered-json-v1
@mutations-en1990-1-any
Feature: Apply every typed EN 1990 mutation to its committed specification fixtures
  `s.norm.en1990` is a semio-NATIVE artifact — no third party reads or writes its
  `.dsl.semio`/`.pack.semio` envelope — so there is no reference implementation to register as an
  oracle. That is recorded as the `en1990-1-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, and it means the runner
  executes NO oracle role for this case: every assertion below lives inside the subject handler,
  which compares the applied document against the committed after-snapshot and the undone document
  against the committed before-snapshot, and fails with both documents printed. A handler that
  merely ran the mutation and returned would report a pass having checked nothing.

  The SMALLEST vocabulary in the plugin, and the only one whose collection is a composed CHILD
  artifact. `En1990Snapshot` is five document-root scalars (`g_k`, `resistance_kn`,
  `consequence_class`, `annex`, `seismic_a_ed_kn`) plus `q_k`, which is not an inline `Vec` at all
  but a fixed `s.stdio.semio.table` child slot holding a handle — a `child_id` and an
  `ArtifactRef` — to a separate table artifact. Five `change-<field>` kinds cover the scalars;
  `insert-variable-action`, `remove-variable-action`, `reorder-variable-actions`,
  `change-variable-action-category` and `change-variable-action-value` reach through the handle
  into the composed table.

  Composition is what this case exists to protect. The committed before/after snapshots carry the
  literal child handle (`"childId": "en1990-qk-7904dd65836c8ff4"` plus its dialect-qualified
  `ArtifactRef`), and `switches-the-national-annex-from-de-to-en` asserts that a scalar edit
  leaves that handle byte-identical rather than re-minting it — an implementation that rebuilds
  the child on every write would produce a plausible-looking document that no longer resolves to
  the same table.
    ⚠️ Four of the ten committed vectors — `remove-variable-action`, `reorder-variable-actions`,
    `change-variable-action-category` and `change-variable-action-value` — carry `{"status":
    "rejected"}` in their committed `🎯️outcome`, because the fixture's child slot is unseeded and
    index 0 does not exist. Those four are not weaker rows, they are a STRICTER contract: the
    scenario requires the mutation to be refused AND the document to come back bit-identical, so
    an implementation that silently clamps an out-of-range index, or that half-applies before
    noticing, fails where a plain "the projection moved" check would have passed it. The remaining
    six carry `{"status": "applied"}` and are held to the observability law instead.

  Each of the 10 kinds carries its own independently handcrafted `(before, mutation, after, diff,
  outcome)` quintet under
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`, and this
  feature re-exercises those SAME committed bytes end to end through `apply_en1990_mutation`
  rather than calling `Mutation::diff`/`inverse` directly the way the in-crate fixture tests do.
  The committed `🎯️outcome` decides which contract a row is held to: `applied` demands the
  observability law (the document must MOVE), `rejected` demands the opposite and stricter one —
  the mutation must be refused and the document must come back bit-identical.

  The identity scenario reads the real committed EN 1990 document at
  `📚️examples/📕️high-consequence-office`, not a fixture authored for this case. Its DSL carrier is
  deliberately byte-preserving — the committed file IS this codec's own canonical printer output,
  so reproducing it exactly is the correct answer and anything else is the defect — which is why
  that half of the identity law is asserted as `carrier_is_exact` rather than as the usual
  no-byte-pass-through inequality. The evidence that the document was genuinely PARSED rather than
  copied comes from the other half: the same snapshot is round-tripped through two further,
  independently written codecs — the binary `.pack.semio` protocol and the JSON projection — and
  all three must agree on one document. The committed binary twin
  `🎒️high-consequence-office.pack.semio` is decoded and cross-checked against the text artifact as
  well, so two separately committed files written by two separate codecs have to describe the same
  EN 1990 document.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot, mutation and outcome fixture for the <id> kind
    When <id> is applied through apply_en1990_mutation
    Then the resulting document matches the committed after-snapshot fixture for <id> and honours the committed outcome status
    Examples:
      | id |
      | change-annex |
      | change-permanent-action |
      | change-resistance |
      | change-consequence-class |
      | change-seismic-action |
      | insert-variable-action |
      | remove-variable-action |
      | change-variable-action-category |
      | change-variable-action-value |
      | reorder-variable-actions |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_en1990_mutation
    And the mutation's own computed inverse is applied through apply_en1990_mutation
    Then the document matches the committed before-snapshot fixture again
    Examples:
      | id |
      | change-annex |
      | change-permanent-action |
      | change-resistance |
      | change-consequence-class |
      | change-seismic-action |
      | insert-variable-action |
      | remove-variable-action |
      | change-variable-action-category |
      | change-variable-action-value |
      | reorder-variable-actions |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real committed EN 1990 document through every encoding it has
    Given the real committed text artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️high-consequence-office/🖼️assets/🗣️high-consequence-office.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️high-consequence-office/🖼️assets/🎒️high-consequence-office.pack.semio
    When the text artifact is parsed, printed back to DSL and parsed again, and the same document is round-tripped through the binary pack protocol and the JSON projection
    Then the canonical DSL rendering is reproduced byte for byte and every decoding agrees on one EN 1990 document
