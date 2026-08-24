@capability-en1998-1-mutate
@no-oracle-en1998-1-mutation-semantics
@comparison-ordered-json-v1
@mutations-en1998-1-any
Feature: Apply every typed EN 1998 mutation to its committed specification fixtures
  `s.norm.en1998` is a semio-NATIVE artifact — no third party reads or writes its
  `.dsl.semio`/`.pack.semio` envelope — so there is no reference implementation to register as an
  oracle. That is recorded as the `en1998-1-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, and it means the runner
  executes NO oracle role for this case: every assertion below lives inside the subject handler,
  which compares the applied document against the committed after-snapshot and the undone document
  against the committed before-snapshot, and fails with both documents printed. A handler that
  merely ran the mutation and returned would report a pass having checked nothing.

  Forty-nine document-root scalars and booleans, one `change-<field>` each — the second-largest
  vocabulary in the plugin — spanning seven of EN 1998's own structure classes in ONE document:
  buildings (seismic zone, ground type, importance class, structural system, T_1, mass, V_Rd,
  drift, height, the multiple-resisting-systems flag), the EN-annex spectrum (a_gR, ground type,
  spectrum type, period ratio), bridges (V_Rd, bearing displacement demand and capacity), retrofit
  assessment (knowledge level, limit state, E_d, R_k, gamma_el), silos and tanks (height, radius,
  N_Rd, V_Ed, V_Rd, behaviour factor q, plus the tank mass and V_Rd), towers and chimneys (M_Ed,
  M_Rd, the chimney flag, q, mass), foundations (area, p_Rd, H_Ed, H_Rd, the two stiffness factors
  k) and retaining walls (height, phi, soil gamma, the ductility factor r, H_Rd).

  Seven structure classes in one flat namespace means the SAME symbol appears up to five times
  under different prefixes: V_Rd exists as `change-v-rd-kn` (building), `change-bridge-v-rd-kn`,
  `change-silo-v-rd-kn` and `change-tank-v-rd-kn`; the behaviour factor q exists as
  `change-silo-q-nominal` and `change-tower-q-nominal`; ground type exists twice, as
  `change-ground-type` and `change-en-ground-type`, because the national and EN spectra classify
  soil differently. That last pair is the sharpest: the two fields must be able to disagree, so an
  implementation that keeps them in sync as a convenience fails the committed after-snapshot for
  either kind. Forty-nine whole-document comparisons is what makes a prefix mix-up visible.

  Each of the 49 kinds carries its own independently handcrafted `(before, mutation, after, diff,
  outcome)` quintet under
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`, and this
  feature re-exercises those SAME committed bytes end to end through `apply_en1998_mutation`
  rather than calling `Mutation::diff`/`inverse` directly the way the in-crate fixture tests do.
  The committed `🎯️outcome` decides which contract a row is held to: `applied` demands the
  observability law (the document must MOVE), `rejected` demands the opposite and stricter one —
  the mutation must be refused and the document must come back bit-identical. All 49 committed
  vectors declare `applied`, so every row below is held to the observability law: a kind that left
  the document bit-for-bit unchanged would fail rather than pass silently.

  The identity scenario reads the real committed EN 1998 document at
  `📚️examples/📕️seismic-rc-frame`, not a fixture authored for this case. Its DSL carrier is
  deliberately byte-preserving — the committed file IS this codec's own canonical printer output,
  so reproducing it exactly is the correct answer and anything else is the defect — which is why
  that half of the identity law is asserted as `carrier_is_exact` rather than as the usual
  no-byte-pass-through inequality. The evidence that the document was genuinely PARSED rather than
  copied comes from the other half: the same snapshot is round-tripped through two further,
  independently written codecs — the binary `.pack.semio` protocol and the JSON projection — and
  all three must agree on one document. The committed binary twin `🎒️seismic-rc-frame.pack.semio`
  is decoded and cross-checked against the text artifact as well, so two separately committed
  files written by two separate codecs have to describe the same EN 1998 document.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot, mutation and outcome fixture for the <id> kind
    When <id> is applied through apply_en1998_mutation
    Then the resulting document matches the committed after-snapshot fixture for <id> and honours the committed outcome status
    Examples:
      | id |
      | change-seismic-zone |
      | change-ground-type |
      | change-importance-class |
      | change-structural-system |
      | change-t1-s |
      | change-mass-t |
      | change-v-rd-kn |
      | change-drift-mm |
      | change-height-m |
      | change-multiple-resisting-systems |
      | change-annex |
      | change-en-a-gr |
      | change-en-ground-type |
      | change-en-spectrum-type |
      | change-period-ratio |
      | change-bridge-v-rd-kn |
      | change-bearing-d-ed-mm |
      | change-bearing-d-rd-mm |
      | change-retrofit-knowledge-level |
      | change-retrofit-limit-state |
      | change-retrofit-ed-kn |
      | change-retrofit-rk-kn |
      | change-retrofit-gamma-el |
      | change-silo-height-m |
      | change-silo-radius-m |
      | change-silo-n-rd-kn |
      | change-silo-v-ed-kn |
      | change-silo-v-rd-kn |
      | change-silo-q-nominal |
      | change-tank-height-m |
      | change-tank-radius-m |
      | change-tank-mass-t |
      | change-tank-v-rd-kn |
      | change-tower-m-ed-knm |
      | change-tower-m-rd-knm |
      | change-tower-is-chimney |
      | change-tower-q-nominal |
      | change-tower-mass-t |
      | change-foundation-area-m2 |
      | change-foundation-p-rd-kpa |
      | change-foundation-h-ed-kn |
      | change-foundation-h-rd-kn |
      | change-k-foundation |
      | change-k-soil |
      | change-wall-height-m |
      | change-wall-phi-deg |
      | change-wall-soil-gamma-kn-m3 |
      | change-wall-r |
      | change-wall-h-rd-kn |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_en1998_mutation
    And the mutation's own computed inverse is applied through apply_en1998_mutation
    Then the document matches the committed before-snapshot fixture again
    Examples:
      | id |
      | change-seismic-zone |
      | change-ground-type |
      | change-importance-class |
      | change-structural-system |
      | change-t1-s |
      | change-mass-t |
      | change-v-rd-kn |
      | change-drift-mm |
      | change-height-m |
      | change-multiple-resisting-systems |
      | change-annex |
      | change-en-a-gr |
      | change-en-ground-type |
      | change-en-spectrum-type |
      | change-period-ratio |
      | change-bridge-v-rd-kn |
      | change-bearing-d-ed-mm |
      | change-bearing-d-rd-mm |
      | change-retrofit-knowledge-level |
      | change-retrofit-limit-state |
      | change-retrofit-ed-kn |
      | change-retrofit-rk-kn |
      | change-retrofit-gamma-el |
      | change-silo-height-m |
      | change-silo-radius-m |
      | change-silo-n-rd-kn |
      | change-silo-v-ed-kn |
      | change-silo-v-rd-kn |
      | change-silo-q-nominal |
      | change-tank-height-m |
      | change-tank-radius-m |
      | change-tank-mass-t |
      | change-tank-v-rd-kn |
      | change-tower-m-ed-knm |
      | change-tower-m-rd-knm |
      | change-tower-is-chimney |
      | change-tower-q-nominal |
      | change-tower-mass-t |
      | change-foundation-area-m2 |
      | change-foundation-p-rd-kpa |
      | change-foundation-h-ed-kn |
      | change-foundation-h-rd-kn |
      | change-k-foundation |
      | change-k-soil |
      | change-wall-height-m |
      | change-wall-phi-deg |
      | change-wall-soil-gamma-kn-m3 |
      | change-wall-r |
      | change-wall-h-rd-kn |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real committed EN 1998 document through every encoding it has
    Given the real committed text artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️seismic-rc-frame/🖼️assets/🗣️seismic-rc-frame.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️seismic-rc-frame/🖼️assets/🎒️seismic-rc-frame.pack.semio
    When the text artifact is parsed, printed back to DSL and parsed again, and the same document is round-tripped through the binary pack protocol and the JSON projection
    Then the canonical DSL rendering is reproduced byte for byte and every decoding agrees on one EN 1998 document
