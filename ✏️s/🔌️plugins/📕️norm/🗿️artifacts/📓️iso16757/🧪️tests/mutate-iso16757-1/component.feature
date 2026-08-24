@capability-iso16757-1-mutate
@no-oracle-iso16757-1-mutation-semantics
@comparison-ordered-json-v1
@mutations-iso16757-1-any
Feature: Apply every typed ISO 16757 mutation to its committed specification fixtures
  `s.norm.iso16757` is a semio-NATIVE artifact — no third party reads or writes its
  `.dsl.semio`/`.pack.semio` envelope — so there is no reference implementation to register as an
  oracle. That is recorded as the `iso16757-1-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, and it means the runner
  executes NO oracle role for this case: every assertion below lives inside the subject handler,
  which compares the applied document against the committed after-snapshot and the undone document
  against the committed before-snapshot, and fails with both documents printed. A handler that
  merely ran the mutation and returned would report a pass having checked nothing.

  This is the RICHEST document shape in the plugin and the only one besides `📔️vdi3805` whose
  vocabulary is a lifecycle rather than a parameter form. `Iso16757Snapshot` is a multi-collection
  document — catalogue, dictionary, geometry, selection, part-number rule, part-number inputs,
  script limits and exchange process — and the twenty-one kinds split into three genuinely
  different families: document-root scalars (`change-exchange-process`, `update-script-limits`,
  `replace-part-number-rule`, `change`/`remove-part-number-input`, `change-selection-class`,
  `change-selection-series`), ordered constraint edits on the selection facet
  (`add`/`remove-selection-constraint`), and full create/delete(+rename) lifecycles over four
  id-keyed collections — the catalogue's `product_groups` and `products`, its
  `property_definitions`, and the dictionary's `subjects`.

  What the fixtures are chosen to expose is REFERENTIAL: `delete-product-group` is committed as
  `removes-the-radiators-group-and-strands-its-class`, i.e. the vector deliberately leaves a
  dangling class reference behind, so an implementation that silently cascades — or silently
  repairs — the reference fails against the committed after-snapshot. `create-product` appends
  into an EXISTING series rather than a fresh one, and `create-subject` appends under an existing
  parent, so an implementation that ignores the parent/series address and appends at the document
  root cannot pass either. `rename-catalogue` and `rename-manufacturer` are the only two identity
  mutations in this vocabulary; every other collection member is addressed by id.

  The vocabulary is deliberately partial and says so in its own module header: `product_classes`,
  `product_series`, `product_indexes`, `descriptive_objects`, the `accessories`/`compositions`
  edges, the dictionary's `relationships`/`properties`/`controlled_lists`/`meta_subjects` and the
  whole `geometry` pool carry no mutation yet. This case measures the twenty-one kinds that DO
  exist; it makes no claim about the deferred surface, which is tracked in that header rather than
  hidden behind a `deferredKinds` entry here.

  Each of the 21 kinds carries its own independently handcrafted `(before, mutation, after, diff,
  outcome)` quintet under
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`, and this
  feature re-exercises those SAME committed bytes end to end through `apply_iso16757_mutation`
  rather than calling `Mutation::diff`/`inverse` directly the way the in-crate fixture tests do.
  The committed `🎯️outcome` decides which contract a row is held to: `applied` demands the
  observability law (the document must MOVE), `rejected` demands the opposite and stricter one —
  the mutation must be refused and the document must come back bit-identical. All 21 committed
  vectors declare `applied`, so every row below is held to the observability law: a kind that left
  the document bit-for-bit unchanged would fail rather than pass silently.

  The identity scenario reads the real committed ISO 16757 document at `📚️examples/🎬️demo`, not a
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
    When <id> is applied through apply_iso16757_mutation
    Then the resulting document matches the committed after-snapshot fixture for <id> and honours the committed outcome status
    Examples:
      | id |
      | change-exchange-process |
      | update-script-limits |
      | replace-part-number-rule |
      | change-part-number-input |
      | remove-part-number-input |
      | change-selection-class |
      | change-selection-series |
      | add-selection-constraint |
      | remove-selection-constraint |
      | rename-catalogue |
      | rename-manufacturer |
      | create-product-group |
      | delete-product-group |
      | rename-product-group |
      | create-product |
      | delete-product |
      | rename-product |
      | create-property-definition |
      | delete-property-definition |
      | create-subject |
      | delete-subject |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_iso16757_mutation
    And the mutation's own computed inverse is applied through apply_iso16757_mutation
    Then the document matches the committed before-snapshot fixture again
    Examples:
      | id |
      | change-exchange-process |
      | update-script-limits |
      | replace-part-number-rule |
      | change-part-number-input |
      | remove-part-number-input |
      | change-selection-class |
      | change-selection-series |
      | add-selection-constraint |
      | remove-selection-constraint |
      | rename-catalogue |
      | rename-manufacturer |
      | create-product-group |
      | delete-product-group |
      | rename-product-group |
      | create-product |
      | delete-product |
      | rename-product |
      | create-property-definition |
      | delete-property-definition |
      | create-subject |
      | delete-subject |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real committed ISO 16757 document through every encoding it has
    Given the real committed text artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the text artifact is parsed, printed back to DSL and parsed again, and the same document is round-tripped through the binary pack protocol and the JSON projection
    Then the canonical DSL rendering is reproduced byte for byte and every decoding agrees on one ISO 16757 document
