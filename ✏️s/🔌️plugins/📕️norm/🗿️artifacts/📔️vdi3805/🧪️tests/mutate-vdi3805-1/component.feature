@capability-vdi3805-1-mutate
@no-oracle-vdi3805-1-mutation-semantics
@comparison-ordered-json-v1
@mutations-vdi3805-1-any
Feature: Apply every typed VDI 3805 mutation to its committed specification fixtures
  `s.norm.vdi3805` is a semio-NATIVE artifact — no third party reads or writes its
  `.dsl.semio`/`.pack.semio` envelope — so there is no reference implementation to register as an
  oracle. That is recorded as the `vdi3805-1-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, and it means the runner
  executes NO oracle role for this case: every assertion below lives inside the subject handler,
  which compares the applied document against the committed after-snapshot and the undone document
  against the committed before-snapshot, and fails with both documents printed. A handler that
  merely ran the mutation and returned would report a pass having checked nothing.

  `Vdi3805Snapshot` carries a manufacturer-file header, an id-keyed `catalog.products` pool,
  edition-profile overrides per VDI sheet, a correction cut-off date, a strict-mode flag,
  parametric geometry definitions with named connections, characteristic curves, and security
  limits on untrusted input. The nineteen kinds cover the header and policy scalars
  (`update-manufacturer-file`, `change-correction-as-of`, `change-strict-mode`, `update-limits`,
  `change`/`remove-edition-profile`), the product lifecycle (`create`/`delete`/`rename-product`,
  `replace-product-configuration`), the geometry lifecycle (`create`/`delete`/`resize-geometry`,
  `add`/`remove-geometry-connection`, `replace-geometry-parameters`) and the curve lifecycle
  (`create`/`delete-curve`, `replace-curve-points`).

  The one thing that genuinely separates this vocabulary from every other in the plugin is DERIVED
  PERSISTED STATE. `catalog.index` mirrors `catalog.products` one-to-one — it is written to the
  document, not recomputed on read — so every product mutation has to keep it in lockstep or the
  document is internally inconsistent the moment it is saved. The committed fixtures are named for
  exactly that obligation and are useless without it: `appends-vlv-80-002-and-its-index-entry`,
  `removes-vlv-50-001-and-its-index-entry`, `retitles-vlv-50-001-and-resyncs-its-index-tags`,
  `reparameterises-vlv-50-001-to-dn-80-and-resyncs-index-dn`. An implementation that edits the
  product and forgets the index passes nothing here, and neither does one that rebuilds the whole
  index from scratch and reorders it.

  Each of the 19 kinds carries its own independently handcrafted `(before, mutation, after, diff,
  outcome)` quintet under
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`, and this
  feature re-exercises those SAME committed bytes end to end through `apply_vdi3805_mutation`
  rather than calling `Mutation::diff`/`inverse` directly the way the in-crate fixture tests do.
  The committed `🎯️outcome` decides which contract a row is held to: `applied` demands the
  observability law (the document must MOVE), `rejected` demands the opposite and stricter one —
  the mutation must be refused and the document must come back bit-identical. All 19 committed
  vectors declare `applied`, so every row below is held to the observability law: a kind that left
  the document bit-for-bit unchanged would fail rather than pass silently.

  The identity scenario reads the real committed VDI 3805 document at `📚️examples/🎬️demo`, not a
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
    When <id> is applied through apply_vdi3805_mutation
    Then the resulting document matches the committed after-snapshot fixture for <id> and honours the committed outcome status
    Examples:
      | id |
      | update-manufacturer-file |
      | change-correction-as-of |
      | change-strict-mode |
      | update-limits |
      | change-edition-profile |
      | remove-edition-profile |
      | create-product |
      | delete-product |
      | rename-product |
      | replace-product-configuration |
      | create-geometry |
      | delete-geometry |
      | resize-geometry |
      | add-geometry-connection |
      | remove-geometry-connection |
      | replace-geometry-parameters |
      | create-curve |
      | delete-curve |
      | replace-curve-points |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_vdi3805_mutation
    And the mutation's own computed inverse is applied through apply_vdi3805_mutation
    Then the document matches the committed before-snapshot fixture again
    Examples:
      | id |
      | update-manufacturer-file |
      | change-correction-as-of |
      | change-strict-mode |
      | update-limits |
      | change-edition-profile |
      | remove-edition-profile |
      | create-product |
      | delete-product |
      | rename-product |
      | replace-product-configuration |
      | create-geometry |
      | delete-geometry |
      | resize-geometry |
      | add-geometry-connection |
      | remove-geometry-connection |
      | replace-geometry-parameters |
      | create-curve |
      | delete-curve |
      | replace-curve-points |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real committed VDI 3805 document through every encoding it has
    Given the real committed text artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the text artifact is parsed, printed back to DSL and parsed again, and the same document is round-tripped through the binary pack protocol and the JSON projection
    Then the canonical DSL rendering is reproduced byte for byte and every decoding agrees on one VDI 3805 document
