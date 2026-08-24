@capability-din16798-1-mutate
@no-oracle-din16798-1-mutation-semantics
@comparison-ordered-json-v1
@mutations-din16798-1-any
Feature: Apply every typed DIN EN 16798-1 mutation to its committed specification fixtures
  `s.norm.din16798` is a semio-NATIVE artifact — no third party reads or writes its
  `.dsl.semio`/`.pack.semio` envelope — so there is no reference implementation to register as an
  oracle. That is recorded as the `din16798-1-mutation-semantics` no-oracle decision in
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`, and it means the runner
  executes NO oracle role for this case: every assertion below lives inside the subject handler,
  which compares the applied document against the committed after-snapshot and the undone document
  against the committed before-snapshot, and fails with both documents printed. A handler that
  merely ran the mutation and returned would report a pass having checked nothing.

  Sixty-two document-root scalars and not one collection: this is the largest flat mutation
  vocabulary in the repository, and every kind is a `change-<field>`. The fields group into the
  standard's own clause families — thermal comfort (operative temperature, humidity, draught air
  speed, running-mean outdoor temperature), air quality (CO2, IDA class, supply airflow), daylight
  and acoustics, three separate occupancy models (non-residential persons, dwelling bedrooms,
  residential occupants) each with their own airflow field, specific fan power, heat recovery
  (achieved and required efficiency, mass flow, specific heat, temperature lift, operating hours,
  savings reference), infiltration and blower-door, cellar ventilation, transmission and
  ventilation heat transfer, cooling (set point, period, gains, utilization factor, reference,
  chiller type, EER, annual demand), storage and DHW, and duct leakage.

  Sixty-two independent scalars is precisely the shape in which a vocabulary rots silently:
  adjacent fields differ by one word (`change-heat-recovery-eta` versus
  `change-heat-recovery-eta-min`, `change-humidification-required-kg-h` versus
  `change-humidification-provided-kg-h`, `change-hr-th` versus `change-storage-th`), so a diff
  builder that writes the neighbouring field is invisible to any check weaker than a full snapshot
  comparison. That is what this case runs: every one of the sixty-two committed vectors is applied
  and compared as a WHOLE document, not field-by-field, so a mutation that also moves a field it
  was not asked to move fails. The committed fixtures also pin the DIRECTION of each edit —
  `tightens-the-comfort-category-to-i`, `relaxes-the-indoor-air-class-to-ida-3`,
  `halves-the-measured-duct-leakage-to-0-point-0625` — so a sign error is a red scenario rather
  than a different-but-plausible number.

  Each of the 62 kinds carries its own independently handcrafted `(before, mutation, after, diff,
  outcome)` quintet under
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`, and this
  feature re-exercises those SAME committed bytes end to end through `apply_din16798_mutation`
  rather than calling `Mutation::diff`/`inverse` directly the way the in-crate fixture tests do.
  The committed `🎯️outcome` decides which contract a row is held to: `applied` demands the
  observability law (the document must MOVE), `rejected` demands the opposite and stricter one —
  the mutation must be refused and the document must come back bit-identical. All 62 committed
  vectors declare `applied`, so every row below is held to the observability law: a kind that left
  the document bit-for-bit unchanged would fail rather than pass silently.

  The identity scenario reads the real committed DIN EN 16798-1 document at `📚️examples/🎬️demo`,
  not a fixture authored for this case. Its DSL carrier is deliberately byte-preserving — the
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
    When <id> is applied through apply_din16798_mutation
    Then the resulting document matches the committed after-snapshot fixture for <id> and honours the committed outcome status
    Examples:
      | id |
      | change-annex |
      | change-occupancy |
      | change-comfort-category |
      | change-t-op-c |
      | change-rh-percent |
      | change-air-speed-ms |
      | change-theta-rm-c |
      | change-co2-ppm |
      | change-df-percent |
      | change-l-aeq-db |
      | change-persons |
      | change-ida-class |
      | change-ventilation-m3-h |
      | change-floor-area-m2 |
      | change-bedrooms |
      | change-dwelling-ventilation-m3-h |
      | change-occupants |
      | change-residential-ventilation-m3-h |
      | change-sfp-wm3-s |
      | change-sfp-required-class |
      | change-heat-recovery-eta |
      | change-heat-recovery-eta-min |
      | change-system-type |
      | change-years-since-inspection |
      | change-humidification-required-kg-h |
      | change-humidification-provided-kg-h |
      | change-fan-qvm3-s |
      | change-fan-t-run-h |
      | change-fan-energy-reference-kwh |
      | change-night-setback-k |
      | change-hr-m-dot-kg-s |
      | change-hr-cp-j-kgk |
      | change-hr-delta-tc |
      | change-hr-th |
      | change-hr-savings-reference-kwh |
      | change-n50-h-inv |
      | change-volume-m3 |
      | change-infiltration-allowance-m3-h |
      | change-cellar-area-m2 |
      | change-cellar-ventilation-m3-h |
      | change-h-tr-wk |
      | change-h-ve-wk |
      | change-theta-ec |
      | change-theta-set-c |
      | change-cooling-delta-th |
      | change-cooling-gains-kwh |
      | change-cooling-utilization-factor |
      | change-cooling-reference-kwh |
      | change-chiller-type |
      | change-eer-actual |
      | change-qc-kwh |
      | change-generation-reference-kwh |
      | change-data-center-supply-c |
      | change-h-st-wk |
      | change-theta-st-c |
      | change-theta-amb-c |
      | change-storage-th |
      | change-storage-allowance-kwh |
      | change-dhw-delivery-c |
      | change-duct-class |
      | change-duct-test-pressure-pa |
      | change-duct-leakage-m3-sm2 |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixture for the <id> kind
    When <id> is applied through apply_din16798_mutation
    And the mutation's own computed inverse is applied through apply_din16798_mutation
    Then the document matches the committed before-snapshot fixture again
    Examples:
      | id |
      | change-annex |
      | change-occupancy |
      | change-comfort-category |
      | change-t-op-c |
      | change-rh-percent |
      | change-air-speed-ms |
      | change-theta-rm-c |
      | change-co2-ppm |
      | change-df-percent |
      | change-l-aeq-db |
      | change-persons |
      | change-ida-class |
      | change-ventilation-m3-h |
      | change-floor-area-m2 |
      | change-bedrooms |
      | change-dwelling-ventilation-m3-h |
      | change-occupants |
      | change-residential-ventilation-m3-h |
      | change-sfp-wm3-s |
      | change-sfp-required-class |
      | change-heat-recovery-eta |
      | change-heat-recovery-eta-min |
      | change-system-type |
      | change-years-since-inspection |
      | change-humidification-required-kg-h |
      | change-humidification-provided-kg-h |
      | change-fan-qvm3-s |
      | change-fan-t-run-h |
      | change-fan-energy-reference-kwh |
      | change-night-setback-k |
      | change-hr-m-dot-kg-s |
      | change-hr-cp-j-kgk |
      | change-hr-delta-tc |
      | change-hr-th |
      | change-hr-savings-reference-kwh |
      | change-n50-h-inv |
      | change-volume-m3 |
      | change-infiltration-allowance-m3-h |
      | change-cellar-area-m2 |
      | change-cellar-ventilation-m3-h |
      | change-h-tr-wk |
      | change-h-ve-wk |
      | change-theta-ec |
      | change-theta-set-c |
      | change-cooling-delta-th |
      | change-cooling-gains-kwh |
      | change-cooling-utilization-factor |
      | change-cooling-reference-kwh |
      | change-chiller-type |
      | change-eer-actual |
      | change-qc-kwh |
      | change-generation-reference-kwh |
      | change-data-center-supply-c |
      | change-h-st-wk |
      | change-theta-st-c |
      | change-theta-amb-c |
      | change-storage-th |
      | change-storage-allowance-kwh |
      | change-dhw-delivery-c |
      | change-duct-class |
      | change-duct-test-pressure-pa |
      | change-duct-leakage-m3-sm2 |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode the real committed DIN EN 16798-1 document through every encoding it has
    Given the real committed text artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio
    When the text artifact is parsed, printed back to DSL and parsed again, and the same document is round-tripped through the binary pack protocol and the JSON projection
    Then the canonical DSL rendering is reproduced byte for byte and every decoding agrees on one DIN EN 16798-1 document
