@capability-en1992-1-mutate
@no-oracle-en1992-1-mutation-semantics
@comparison-ordered-json-v1
@mutations-en1992-1-any
Feature: Apply every typed EN 1992 mutation to its committed specification fixtures
  `s.norm.en1992` is a semio-NATIVE artifact — no third party reads or writes its
  `.dsl.semio`/`.pack.semio` envelope — so there is no reference implementation to register as an
  oracle. That is recorded as the `en1992-1-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, and it means the runner
  executes NO oracle role for this case: every assertion below lives inside the subject handler,
  which compares the applied document against the committed after-snapshot and the undone document
  against the committed before-snapshot, and fails with both documents printed. A handler that
  merely ran the mutation and returned would report a pass having checked nothing.

  Thirty-five document-root scalars, one `change-<field>` each, feeding five distinct EN 1992
  checks: bending and shear (M_Ed, V_Ed, f_ck, b, d, A_s, f_yk, rho_l, N_Ed, P, A_c, the FEM
  toggle, span and UDL), fire (rating and provided axis distance), bridge fatigue (concrete stress
  and steel stress range), the liquid-retaining crack-width check (tightness class, h_D/h ratio,
  sigma_s, rho_p,eff, f_ct,eff, E_s and s_r,max) and the anchor check (h_ef, the cracked-concrete
  flag, f_uk, f_yk, A_s, d, c_1, N_Ed and V_Ed).

  Three of the five families carry their OWN copy of a symbol that already exists at the document
  root — `change-liquid-sigma-s-mpa` beside `change-bridge-delta-sigma-s-mpa`,
  `change-anchor-as-mm2` beside `change-as-mm2`, `change-anchor-f-yk-mpa` beside `change-f-yk`,
  `change-anchor-n-ed-kn`/`change-anchor-v-ed-kn` beside `change-n-ed-kn`/`change-v-ed-kn`. They
  are different physical quantities in different clauses that happen to share a symbol, and the
  single most likely defect in this vocabulary is a diff builder wired to the wrong one of a pair.
  Every scenario below compares the whole document, so writing the sibling field is a failure
  rather than a plausible number.
    The committed example is a liquid-retaining structure WITH a FEM run and an anchor check, so
    the real asset actually exercises all three of the families that overlap.

  Each of the 35 kinds carries its own independently handcrafted `(before, mutation, after, diff,
  outcome)` quintet under
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`, and this
  feature re-exercises those SAME committed bytes end to end through `apply_en1992_mutation`
  rather than calling `Mutation::diff`/`inverse` directly the way the in-crate fixture tests do.
  The committed `🎯️outcome` decides which contract a row is held to: `applied` demands the
  observability law (the document must MOVE), `rejected` demands the opposite and stricter one —
  the mutation must be refused and the document must come back bit-identical. All 35 committed
  vectors declare `applied`, so every row below is held to the observability law: a kind that left
  the document bit-for-bit unchanged would fail rather than pass silently.

  The identity scenario reads the real committed EN 1992 document at
  `📚️examples/📕️liquid-retaining-fem-anchor`, not a fixture authored for this case. Its DSL
  carrier is deliberately byte-preserving — the committed file IS this codec's own canonical
  printer output, so reproducing it exactly is the correct answer and anything else is the defect
  — which is why that half of the identity law is asserted as `carrier_is_exact` rather than as
  the usual no-byte-pass-through inequality. The evidence that the document was genuinely PARSED
  rather than copied comes from the other half: the same snapshot is round-tripped through two
  further, independently written codecs — the binary `.pack.semio` protocol and the JSON
  projection — and all three must agree on one document. The committed binary twin
  `🎒️liquid-retaining-fem-anchor.pack.semio` is decoded and cross-checked against the text
  artifact as well, so two separately committed files written by two separate codecs have to
  describe the same EN 1992 document.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot, mutation and outcome fixture for the <id> kind
    When <id> is applied through apply_en1992_mutation
    Then the resulting document matches the committed after-snapshot fixture for <id> and honours the committed outcome status
    Examples:
      | id |
      | change-annex |
      | change-m-ed-knm |
      | change-v-ed-kn |
      | change-f-ck |
      | change-b-mm |
      | change-d-mm |
      | change-as-mm2 |
      | change-f-yk |
      | change-rho-l |
      | change-n-ed-kn |
      | change-p-kn |
      | change-ac-mm2 |
      | change-use-fem |
      | change-span-m |
      | change-udl-kn-m |
      | change-fire-rating |
      | change-provided-axis-distance-mm |
      | change-bridge-sigma-c-mpa |
      | change-bridge-delta-sigma-s-mpa |
      | change-tightness-class |
      | change-hd-over-h |
      | change-liquid-sigma-s-mpa |
      | change-liquid-rho-p-eff |
      | change-liquid-f-ct-eff-mpa |
      | change-liquid-es-mpa |
      | change-liquid-sr-max-mm |
      | change-anchor-h-ef-mm |
      | change-anchor-cracked |
      | change-anchor-f-uk-mpa |
      | change-anchor-f-yk-mpa |
      | change-anchor-as-mm2 |
      | change-anchor-d-mm |
      | change-anchor-c1-mm |
      | change-anchor-n-ed-kn |
      | change-anchor-v-ed-kn |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_en1992_mutation
    And the mutation's own computed inverse is applied through apply_en1992_mutation
    Then the document matches the committed before-snapshot fixture again
    Examples:
      | id |
      | change-annex |
      | change-m-ed-knm |
      | change-v-ed-kn |
      | change-f-ck |
      | change-b-mm |
      | change-d-mm |
      | change-as-mm2 |
      | change-f-yk |
      | change-rho-l |
      | change-n-ed-kn |
      | change-p-kn |
      | change-ac-mm2 |
      | change-use-fem |
      | change-span-m |
      | change-udl-kn-m |
      | change-fire-rating |
      | change-provided-axis-distance-mm |
      | change-bridge-sigma-c-mpa |
      | change-bridge-delta-sigma-s-mpa |
      | change-tightness-class |
      | change-hd-over-h |
      | change-liquid-sigma-s-mpa |
      | change-liquid-rho-p-eff |
      | change-liquid-f-ct-eff-mpa |
      | change-liquid-es-mpa |
      | change-liquid-sr-max-mm |
      | change-anchor-h-ef-mm |
      | change-anchor-cracked |
      | change-anchor-f-uk-mpa |
      | change-anchor-f-yk-mpa |
      | change-anchor-as-mm2 |
      | change-anchor-d-mm |
      | change-anchor-c1-mm |
      | change-anchor-n-ed-kn |
      | change-anchor-v-ed-kn |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real committed EN 1992 document through every encoding it has
    Given the real committed text artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️liquid-retaining-fem-anchor/🖼️assets/🗣️liquid-retaining-fem-anchor.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️liquid-retaining-fem-anchor/🖼️assets/🎒️liquid-retaining-fem-anchor.pack.semio
    When the text artifact is parsed, printed back to DSL and parsed again, and the same document is round-tripped through the binary pack protocol and the JSON projection
    Then the canonical DSL rendering is reproduced byte for byte and every decoding agrees on one EN 1992 document
