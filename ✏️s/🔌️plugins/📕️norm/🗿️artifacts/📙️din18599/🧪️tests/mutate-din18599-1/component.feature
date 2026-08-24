@capability-din18599-1-mutate
@no-oracle-din18599-1-mutation-semantics
@comparison-ordered-json-v1
@mutations-din18599-1-any
Feature: Apply every typed DIN V 18599 mutation to its committed specification fixtures
  `s.norm.din18599` is a semio-NATIVE artifact — no third party reads or writes its
  `.dsl.semio`/`.pack.semio` envelope — so there is no reference implementation to register as an
  oracle. That is recorded as the `din18599-1-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, and it means the runner
  executes NO oracle role for this case: every assertion below lives inside the subject handler,
  which compares the applied document against the committed after-snapshot and the undone document
  against the committed before-snapshot, and fails with both documents printed. A handler that
  merely ran the mutation and returned would report a pass having checked nothing.

  Twelve document-root scalars — use class, heated area, occupants, the transmission and
  ventilation heat-transfer coefficients H_T and H_V, internal and solar gains, system losses,
  renewable yield, the annual primary-energy limit, the energy carrier and the reference Q_p —
  each with its own `change-<field>` kind, plus ONE `update-climate`.

  `update-climate` is the only `update-<facet>` mutation in this artifact and the reason this
  vocabulary is thirteen kinds rather than fourteen: `climate: MonthlyClimate` is two parallel
  twelve-month arrays, `theta_e_c` and `g_h_w_m2`, which are always entered as one dataset —
  typically loaded whole from `MonthlyClimate::german_reference` for a climate zone — and never
  one month or one array at a time from this app's own input surface. Splitting it into two
  `change-*` mutations would let a document exist with outdoor temperatures from one zone and
  irradiation from another, which is not a state the standard admits.
    ⚠️ Its committed vector carries `{"status": "rejected"}`: the fixture offers a climate dataset
    the artifact refuses, so the scenario requires the mutation to be REFUSED and the document to
    come back bit-identical. That is a stricter contract than the observability law the other
    twelve kinds are held to — a partial application of a twenty-four-value facet is exactly the
    defect it catches — and it is the one kind in this vocabulary whose forward effect is a
    rejection rather than a change.

  Each of the 13 kinds carries its own independently handcrafted `(before, mutation, after, diff,
  outcome)` quintet under
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`, and this
  feature re-exercises those SAME committed bytes end to end through `apply_din18599_mutation`
  rather than calling `Mutation::diff`/`inverse` directly the way the in-crate fixture tests do.
  The committed `🎯️outcome` decides which contract a row is held to: `applied` demands the
  observability law (the document must MOVE), `rejected` demands the opposite and stricter one —
  the mutation must be refused and the document must come back bit-identical.

  The identity scenario reads the real committed DIN V 18599 document at `📚️examples/🎬️demo`, not
  a fixture authored for this case. Its DSL carrier is deliberately byte-preserving — the
  committed file IS this codec's own canonical printer output, so reproducing it exactly is the
  correct answer and anything else is the defect — which is why that half of the identity law is
  asserted as `carrier_is_exact` rather than as the usual no-byte-pass-through inequality. The
  evidence that the document was genuinely PARSED rather than copied comes from the other half:
  the same snapshot is round-tripped through two further, independently written codecs — the
  binary `.pack.semio` protocol and the JSON projection — and all three must agree on one
  document. This artifact commits only the DSL encoding of its example, so the binary leg is
  encode-then-decode rather than a committed twin; that is stated here rather than papered over.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot, mutation and outcome fixture for the <id> kind
    When <id> is applied through apply_din18599_mutation
    Then the resulting document matches the committed after-snapshot fixture for <id> and honours the committed outcome status
    Examples:
      | id |
      | change-use-class |
      | change-heated-area-m2 |
      | change-occupants |
      | change-ht |
      | change-hv |
      | change-internal-gains-wm2 |
      | change-solar-gains-kwh |
      | change-system-losses-kwh |
      | change-renewable-kwh |
      | change-annual-limit-kwh |
      | change-energy-carrier |
      | change-reference-qp-kwh |
      | update-climate |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_din18599_mutation
    And the mutation's own computed inverse is applied through apply_din18599_mutation
    Then the document matches the committed before-snapshot fixture again
    Examples:
      | id |
      | change-use-class |
      | change-heated-area-m2 |
      | change-occupants |
      | change-ht |
      | change-hv |
      | change-internal-gains-wm2 |
      | change-solar-gains-kwh |
      | change-system-losses-kwh |
      | change-renewable-kwh |
      | change-annual-limit-kwh |
      | change-energy-carrier |
      | change-reference-qp-kwh |
      | update-climate |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real committed DIN V 18599 document through every encoding it has
    Given the real committed text artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the text artifact is parsed, printed back to DSL and parsed again, and the same document is round-tripped through the binary pack protocol and the JSON projection
    Then the canonical DSL rendering is reproduced byte for byte and every decoding agrees on one DIN V 18599 document
