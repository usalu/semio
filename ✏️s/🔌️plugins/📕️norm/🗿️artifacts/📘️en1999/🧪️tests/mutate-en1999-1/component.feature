@capability-en1999-1-mutate
@no-oracle-en1999-1-mutation-semantics
@comparison-ordered-json-v1
@mutations-en1999-1-any
Feature: Apply every typed EN 1999 mutation to its committed specification fixtures
  `s.norm.en1999` is a semio-NATIVE artifact — no third party reads or writes its
  `.dsl.semio`/`.pack.semio` envelope — so there is no reference implementation to register as an
  oracle. That is recorded as the `en1999-1-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, and it means the runner
  executes NO oracle role for this case: every assertion below lives inside the subject handler,
  which compares the applied document against the committed after-snapshot and the undone document
  against the committed before-snapshot, and fails with both documents printed. A handler that
  merely ran the mutation and returned would report a pass having checked nothing.

  Twenty-six document-root scalars and enums, one `change-<field>` each: the design actions N_Ed
  and M_Ed, the section properties A and W_el, the alloy selection, the buckling reduction chi,
  the torsion constant I_T and the critical length L_cr, the elevated-temperature theta, the
  fatigue set (applied and detail stress ranges, the slope m, and the cycle count), the fillet
  weld set (V_Ed, throat, length and the correlation factor beta_w), the thin-sheet local-buckling
  set (b, t, k_sigma, W_el and M_Ed) and the shell set (t, r and the applied meridional stress).

  Aluminium is the Eurocode where the ALLOY is not a material constant but a branch:
  `change-alloy` re-selects f_o and f_u, the heat-affected-zone softening factors and the buckling
  class in one step, which is why it is committed as a whole-document vector rather than as a
  strength edit. The vocabulary also carries THREE separate copies of the same section symbols
  because EN 1999 treats them as different members: `change-w-el-mm3` and `change-m-ed-knm` at the
  document root, `change-sheet-w-el-mm3` and `change-sheet-m-ed-knm` for the thin-sheet
  local-buckling check, and `change-shell-t-mm`/`change-shell-r-mm` for the shell. A diff builder
  wired to the root copy instead of the sheet copy produces a document that still checks out
  numerically and is wrong — which is exactly what a full after-snapshot comparison catches and a
  spot check does not.

  Each of the 26 kinds carries its own independently handcrafted `(before, mutation, after, diff,
  outcome)` quintet under
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`, and this
  feature re-exercises those SAME committed bytes end to end through `apply_en1999_mutation`
  rather than calling `Mutation::diff`/`inverse` directly the way the in-crate fixture tests do.
  The committed `🎯️outcome` decides which contract a row is held to: `applied` demands the
  observability law (the document must MOVE), `rejected` demands the opposite and stricter one —
  the mutation must be refused and the document must come back bit-identical. All 26 committed
  vectors declare `applied`, so every row below is held to the observability law: a kind that left
  the document bit-for-bit unchanged would fail rather than pass silently.

  The identity scenario reads the real committed EN 1999 document at
  `📚️examples/📕️aluminium-roof-purlin`, not a fixture authored for this case. Its DSL carrier is
  deliberately byte-preserving — the committed file IS this codec's own canonical printer output,
  so reproducing it exactly is the correct answer and anything else is the defect — which is why
  that half of the identity law is asserted as `carrier_is_exact` rather than as the usual
  no-byte-pass-through inequality. The evidence that the document was genuinely PARSED rather than
  copied comes from the other half: the same snapshot is round-tripped through two further,
  independently written codecs — the binary `.pack.semio` protocol and the JSON projection — and
  all three must agree on one document. The committed binary twin
  `🎒️aluminium-roof-purlin.pack.semio` is decoded and cross-checked against the text artifact as
  well, so two separately committed files written by two separate codecs have to describe the same
  EN 1999 document.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot, mutation and outcome fixture for the <id> kind
    When <id> is applied through apply_en1999_mutation
    Then the resulting document matches the committed after-snapshot fixture for <id> and honours the committed outcome status
    Examples:
      | id |
      | change-n-ed-kn |
      | change-m-ed-knm |
      | change-a-mm2 |
      | change-w-el-mm3 |
      | change-alloy |
      | change-chi |
      | change-it-mm4 |
      | change-l-cr-mm |
      | change-theta-c |
      | change-delta-sigma-ed |
      | change-delta-sigma-c |
      | change-fatigue-m |
      | change-n-cycles |
      | change-v-weld-ed-kn |
      | change-weld-throat-mm |
      | change-weld-length-mm |
      | change-beta-w |
      | change-sheet-b-mm |
      | change-sheet-t-mm |
      | change-sheet-k-sigma |
      | change-sheet-w-el-mm3 |
      | change-sheet-m-ed-knm |
      | change-shell-t-mm |
      | change-shell-r-mm |
      | change-sigma-ed-shell-mpa |
      | change-annex |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_en1999_mutation
    And the mutation's own computed inverse is applied through apply_en1999_mutation
    Then the document matches the committed before-snapshot fixture again
    Examples:
      | id |
      | change-n-ed-kn |
      | change-m-ed-knm |
      | change-a-mm2 |
      | change-w-el-mm3 |
      | change-alloy |
      | change-chi |
      | change-it-mm4 |
      | change-l-cr-mm |
      | change-theta-c |
      | change-delta-sigma-ed |
      | change-delta-sigma-c |
      | change-fatigue-m |
      | change-n-cycles |
      | change-v-weld-ed-kn |
      | change-weld-throat-mm |
      | change-weld-length-mm |
      | change-beta-w |
      | change-sheet-b-mm |
      | change-sheet-t-mm |
      | change-sheet-k-sigma |
      | change-sheet-w-el-mm3 |
      | change-sheet-m-ed-knm |
      | change-shell-t-mm |
      | change-shell-r-mm |
      | change-sigma-ed-shell-mpa |
      | change-annex |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real committed EN 1999 document through every encoding it has
    Given the real committed text artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️aluminium-roof-purlin/🖼️assets/🗣️aluminium-roof-purlin.dsl.semio
    And its committed binary twin asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/📕️aluminium-roof-purlin/🖼️assets/🎒️aluminium-roof-purlin.pack.semio
    When the text artifact is parsed, printed back to DSL and parsed again, and the same document is round-tripped through the binary pack protocol and the JSON projection
    Then the canonical DSL rendering is reproduced byte for byte and every decoding agrees on one EN 1999 document
