@capability-din16798-1-mutate
@oracle-din16798-1-python-independent
@comparison-ordered-json-v1
@mutations-din16798-1-any
Feature: Apply every typed DIN EN 16798 mutation against an independent Python implementation
  `s.norm.din16798` is a semio-NATIVE artifact and no third party reads or writes it — checked, not
  assumed: PyPI serves no `din16798` distribution, and none for `eurocode`, `vdi3805` or `iso16757`
  either, and the nearest real packages (`structuralcodes`, `concreteproperties`, `anastruct`)
  implement design-code FORMULAE and speak no interchange format at all, so not one of them could be
  authoritative over this subset's `Din16798Mutation` vocabulary. The second producer a differential
  comparison needs is therefore a second IMPLEMENTATION, and `🐍️component.py` beside this file is
  it: all 62 kinds of this vocabulary, written in Python from the repository's own written
  specification of what a semantic mutation means — `📓️taxonomy.md`'s verb table, naming mechanics
  ("New-value fields are `new_<field>`") and addressing convention ("Inverse always computed from
  `base`", "Missing target ⇒ `inverse` returns `Vec::new()`"), and `📓️derivation-rules.md`'s shape
  rules — plus this subset's committed catalog for the closed list of kinds. It imports nothing from
  the Rust it judges and transliterates none of it: the document field a `new*` argument names is
  resolved by normalised spelling against the document's own keys, which is what the naming mechanic
  states, never from a table copied out of `🧬️mutations/**` — and the paragraph below names the
  spellings in THIS subset where that resolution can genuinely go wrong. The recorded no-oracle
  decision it replaces is gone from
  `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`, because there is now a
  reference to compare against.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)`
  path below is a declared `asset://` fixture, so neither side holds a transcription that could
  drift. This subset is the widest FLAT vocabulary in the plugin — all sixty-two kinds are
  `change-<field>` on one indoor-climate record, with no collection, no composed child and no
  positional addressing anywhere. That makes it the purest test of the naming mechanic itself:
  sixty-two `new_<field>` arguments must each resolve to exactly one of the document's own keys by
  normalised spelling, and near-collisions the domain really contains (the five-way
  `change-theta-rm-c` / `change-theta-set-c` / `change-theta-st-c` / `change-theta-ec` /
  `change-theta-amb-c` family, `change-heat-recovery-eta` versus `change-heat-recovery-eta-min`,
  `change-hr-th` versus `change-storage-th`) are where a second reading written from the spelling
  rule alone can genuinely land on the wrong field. Each side then asserts the same three laws in
  role — the applied document must BE the committed after-snapshot; an `applied` vector must move
  the document and a `rejected` one must leave it bit-identical; and the mutation followed by its
  OWN computed inverse must restore the before-snapshot exactly. What `parity` adds on top is the
  only thing a single implementation can never provide: that two implementations, in two languages,
  written from one written specification, reach the same document.

  `inverse-` projects BOTH the mutated and the restored document. With sixty-two scalar kinds the
  restored document is always the before-document, so projecting only it would make every one of the
  sixty-two rows report the identical value and the differential would be vacuous — the mutated
  projection is the only half of the pair that distinguishes one row from the next.

  ⚠️ Honest boundary — the CARRIER and the INPUT. `identity-round-trip` reads
  `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`, 1,224 bytes of hand-authored residential demo
  data (`annex=de occupancy=residential comfort-category=II …`). Every one of the sixty-two fields
  is present, which is exactly what this case needs, but the values are illustrative rather than a
  measured building, so this is grammar and field-coverage evidence, not evidence about DIN EN 16798
  conformity. The carrier has no published grammar: the committed `📖️component.grammar.semio` is the
  repository-wide `payload = OCTET+` placeholder, so identity is compared at the envelope preamble,
  the ordered `key=value` fields and the digest and length of the re-emitted bytes — deliberately
  not at a carrier-token-to-enum-spelling mapping, which no document in this repository states.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed specification vector
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot under the committed outcome status and the two agree
    Examples:
      | id                                  | dir                                  | fixture                                                        |
      | change-annex                        | 🏷️change-annex                       | switches-the-check-to-the-en-annex                             |
      | change-occupancy                    | 🍂change-occupancy                    | reclassifies-the-space-as-office                               |
      | change-comfort-category             | 🪛change-comfort-category             | tightens-the-comfort-category-to-i                             |
      | change-t-op-c                       | 🌊change-t-op-c                       | raises-the-operative-temperature-to-24-point-5-c               |
      | change-rh-percent                   | 🌹change-rh-percent                   | drops-indoor-humidity-to-42-point-5-percent                    |
      | change-air-speed-ms                 | 🔀change-air-speed-ms                 | doubles-the-draught-air-speed-to-0-point-25-ms                 |
      | change-theta-rm-c                   | 🌍️change-theta-rm-c                  | raises-the-running-mean-outdoor-temperature-to-18-point-5-c    |
      | change-co2-ppm                      | 🛠️change-co2-ppm                     | raises-the-measured-co2-to-950-ppm                             |
      | change-df-percent                   | 🧵change-df-percent                   | raises-the-daylight-factor-to-3-point-75-percent               |
      | change-l-aeq-db                     | 🌳change-l-aeq-db                     | raises-the-equivalent-sound-level-to-30-db                     |
      | change-persons                      | 🌱change-persons                      | raises-the-design-occupancy-to-16-people                       |
      | change-ida-class                    | 🌵change-ida-class                    | relaxes-the-indoor-air-class-to-ida-3                          |
      | change-ventilation-m3-h             | 🌐change-ventilation-m3-h             | raises-the-supply-airflow-to-360-m3-per-hour                   |
      | change-floor-area-m2                | 🧼change-floor-area-m2                | grows-the-conditioned-floor-area-to-120-m2                     |
      | change-bedrooms                     | 🔢change-bedrooms                     | adds-a-fourth-bedroom                                          |
      | change-dwelling-ventilation-m3-h    | 🧲change-dwelling-ventilation-m3-h    | raises-the-dwelling-airflow-to-96-m3-per-hour                  |
      | change-occupants                    | 🍃change-occupants                    | raises-the-household-to-five-occupants                         |
      | change-residential-ventilation-m3-h | 🌸change-residential-ventilation-m3-h | raises-the-residential-airflow-to-110-m3-per-hour              |
      | change-sfp-wm3-s                    | 🌻change-sfp-wm3-s                    | improves-the-specific-fan-power-to-1250-w-per-m3-s             |
      | change-sfp-required-class           | 🌺change-sfp-required-class           | tightens-the-required-sfp-class-to-3                           |
      | change-heat-recovery-eta            | 🪑change-heat-recovery-eta            | raises-the-achieved-heat-recovery-to-0-point-875               |
      | change-heat-recovery-eta-min        | 🪞change-heat-recovery-eta-min        | raises-the-required-heat-recovery-minimum-to-0-point-625       |
      | change-system-type                  | 🌰change-system-type                  | switches-to-a-decentral-mechanical-system                      |
      | change-years-since-inspection       | 🏔️change-years-since-inspection      | ages-the-last-inspection-to-six-years                          |
      | change-humidification-required-kg-h | 🌾change-humidification-required-kg-h | raises-the-required-humidification-to-3-point-5-kg-per-hour    |
      | change-humidification-provided-kg-h | 🍀change-humidification-provided-kg-h | drops-the-provided-humidification-to-1-point-25-kg-per-hour    |
      | change-fan-qvm3-s                   | 🪥change-fan-qvm3-s                   | raises-the-fan-volume-flow-to-1-point-5-m3-per-second          |
      | change-fan-t-run-h                  | 🧴change-fan-t-run-h                  | extends-the-daily-fan-runtime-to-12-hours                      |
      | change-fan-energy-reference-kwh     | 🪒change-fan-energy-reference-kwh     | raises-the-fan-energy-reference-to-18-kwh                      |
      | change-night-setback-k              | 🍁change-night-setback-k              | deepens-the-night-setback-to-5-kelvin                          |
      | change-hr-m-dot-kg-s                | 🚿change-hr-m-dot-kg-s                | raises-the-heat-recovery-mass-flow-to-0-point-75-kg-per-second |
      | change-hr-cp-j-kgk                  | 🛋️change-hr-cp-j-kgk                 | corrects-the-air-specific-heat-to-1010-j-per-kgk               |
      | change-hr-delta-tc                  | 🛏️change-hr-delta-tc                 | drops-the-heat-recovery-temperature-lift-to-12-point-5-c       |
      | change-hr-th                        | 🌿change-hr-th                        | extends-the-heat-recovery-operating-hours-to-14                |
      | change-hr-savings-reference-kwh     | 🛁change-hr-savings-reference-kwh     | raises-the-heat-recovery-savings-reference-to-65-kwh           |
      | change-n50-h-inv                    | 🌲change-n50-h-inv                    | loosens-the-blower-door-result-to-2-point-5-per-hour           |
      | change-volume-m3                    | 🗻change-volume-m3                    | grows-the-air-volume-to-640-m3                                 |
      | change-infiltration-allowance-m3-h  | 🌴change-infiltration-allowance-m3-h  | raises-the-infiltration-allowance-to-52-point-5-m3-per-hour    |
      | change-cellar-area-m2               | 🛡️change-cellar-area-m2              | grows-the-cellar-floor-area-to-62-point-5-m2                   |
      | change-cellar-ventilation-m3-h      | 🧯change-cellar-ventilation-m3-h      | raises-the-cellar-airflow-to-22-point-5-m3-per-hour            |
      | change-h-tr-wk                      | 🧹change-h-tr-wk                      | improves-the-transmission-heat-transfer-to-175-w-per-k         |
      | change-h-ve-wk                      | 🧺change-h-ve-wk                      | raises-the-ventilation-heat-transfer-to-125-w-per-k            |
      | change-theta-ec                     | 🪨change-theta-ec                     | raises-the-external-design-temperature-to-34-point-5-c         |
      | change-theta-set-c                  | 🌎️change-theta-set-c                 | lowers-the-cooling-set-point-to-25-c                           |
      | change-cooling-delta-th             | 🪚change-cooling-delta-th             | extends-the-cooling-period-to-12-point-5-hours                 |
      | change-cooling-gains-kwh            | 🪜change-cooling-gains-kwh            | raises-the-internal-cooling-gains-to-7-point-5-kwh             |
      | change-cooling-utilization-factor   | 🪣change-cooling-utilization-factor   | raises-the-cooling-utilization-factor-to-0-point-875           |
      | change-cooling-reference-kwh        | 🪝change-cooling-reference-kwh        | raises-the-cooling-reference-to-25-kwh                         |
      | change-chiller-type                 | 🚨change-chiller-type                 | switches-to-a-water-cooled-chiller                             |
      | change-eer-actual                   | 🪤change-eer-actual                   | raises-the-achieved-eer-to-3-point-5                           |
      | change-qc-kwh                       | 🌷change-qc-kwh                       | raises-the-annual-cooling-demand-to-1250-kwh                   |
      | change-generation-reference-kwh     | 🧽change-generation-reference-kwh     | raises-the-generation-reference-to-450-kwh                     |
      | change-data-center-supply-c         | 🧰change-data-center-supply-c         | raises-the-data-centre-supply-air-to-27-c                      |
      | change-h-st-wk                      | 🪠change-h-st-wk                      | raises-the-storage-loss-coefficient-to-6-point-5-w-per-k       |
      | change-theta-st-c                   | 🌏️change-theta-st-c                  | lowers-the-storage-temperature-to-55-c                         |
      | change-theta-amb-c                  | 🐚change-theta-amb-c                  | lowers-the-storage-room-ambient-to-18-c                        |
      | change-storage-th                   | 🍄change-storage-th                   | shortens-the-storage-standby-period-to-18-hours                |
      | change-storage-allowance-kwh        | 🌼change-storage-allowance-kwh        | tightens-the-storage-loss-allowance-to-4-point-5-kwh           |
      | change-dhw-delivery-c               | 🧶change-dhw-delivery-c               | raises-the-dhw-delivery-temperature-to-60-c                    |
      | change-duct-class                   | 🪡change-duct-class                   | upgrades-the-duct-tightness-class-to-d                         |
      | change-duct-test-pressure-pa        | 🧷change-duct-test-pressure-pa        | raises-the-duct-test-pressure-to-500-pa                        |
      | change-duct-leakage-m3-sm2          | 🪢change-duct-leakage-m3-sm2          | halves-the-measured-duct-leakage-to-0-point-0625               |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores its committed before-snapshot
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    When each implementation applies the committed mutation and then its OWN computed inverse
    Then both restore the before-snapshot and agree on the mutated and the restored document
    Examples:
      | id                                  | dir                                  | fixture                                                        |
      | change-annex                        | 🏷️change-annex                       | switches-the-check-to-the-en-annex                             |
      | change-occupancy                    | 🍂change-occupancy                    | reclassifies-the-space-as-office                               |
      | change-comfort-category             | 🪛change-comfort-category             | tightens-the-comfort-category-to-i                             |
      | change-t-op-c                       | 🌊change-t-op-c                       | raises-the-operative-temperature-to-24-point-5-c               |
      | change-rh-percent                   | 🌹change-rh-percent                   | drops-indoor-humidity-to-42-point-5-percent                    |
      | change-air-speed-ms                 | 🔀change-air-speed-ms                 | doubles-the-draught-air-speed-to-0-point-25-ms                 |
      | change-theta-rm-c                   | 🌍️change-theta-rm-c                  | raises-the-running-mean-outdoor-temperature-to-18-point-5-c    |
      | change-co2-ppm                      | 🛠️change-co2-ppm                     | raises-the-measured-co2-to-950-ppm                             |
      | change-df-percent                   | 🧵change-df-percent                   | raises-the-daylight-factor-to-3-point-75-percent               |
      | change-l-aeq-db                     | 🌳change-l-aeq-db                     | raises-the-equivalent-sound-level-to-30-db                     |
      | change-persons                      | 🌱change-persons                      | raises-the-design-occupancy-to-16-people                       |
      | change-ida-class                    | 🌵change-ida-class                    | relaxes-the-indoor-air-class-to-ida-3                          |
      | change-ventilation-m3-h             | 🌐change-ventilation-m3-h             | raises-the-supply-airflow-to-360-m3-per-hour                   |
      | change-floor-area-m2                | 🧼change-floor-area-m2                | grows-the-conditioned-floor-area-to-120-m2                     |
      | change-bedrooms                     | 🔢change-bedrooms                     | adds-a-fourth-bedroom                                          |
      | change-dwelling-ventilation-m3-h    | 🧲change-dwelling-ventilation-m3-h    | raises-the-dwelling-airflow-to-96-m3-per-hour                  |
      | change-occupants                    | 🍃change-occupants                    | raises-the-household-to-five-occupants                         |
      | change-residential-ventilation-m3-h | 🌸change-residential-ventilation-m3-h | raises-the-residential-airflow-to-110-m3-per-hour              |
      | change-sfp-wm3-s                    | 🌻change-sfp-wm3-s                    | improves-the-specific-fan-power-to-1250-w-per-m3-s             |
      | change-sfp-required-class           | 🌺change-sfp-required-class           | tightens-the-required-sfp-class-to-3                           |
      | change-heat-recovery-eta            | 🪑change-heat-recovery-eta            | raises-the-achieved-heat-recovery-to-0-point-875               |
      | change-heat-recovery-eta-min        | 🪞change-heat-recovery-eta-min        | raises-the-required-heat-recovery-minimum-to-0-point-625       |
      | change-system-type                  | 🌰change-system-type                  | switches-to-a-decentral-mechanical-system                      |
      | change-years-since-inspection       | 🏔️change-years-since-inspection      | ages-the-last-inspection-to-six-years                          |
      | change-humidification-required-kg-h | 🌾change-humidification-required-kg-h | raises-the-required-humidification-to-3-point-5-kg-per-hour    |
      | change-humidification-provided-kg-h | 🍀change-humidification-provided-kg-h | drops-the-provided-humidification-to-1-point-25-kg-per-hour    |
      | change-fan-qvm3-s                   | 🪥change-fan-qvm3-s                   | raises-the-fan-volume-flow-to-1-point-5-m3-per-second          |
      | change-fan-t-run-h                  | 🧴change-fan-t-run-h                  | extends-the-daily-fan-runtime-to-12-hours                      |
      | change-fan-energy-reference-kwh     | 🪒change-fan-energy-reference-kwh     | raises-the-fan-energy-reference-to-18-kwh                      |
      | change-night-setback-k              | 🍁change-night-setback-k              | deepens-the-night-setback-to-5-kelvin                          |
      | change-hr-m-dot-kg-s                | 🚿change-hr-m-dot-kg-s                | raises-the-heat-recovery-mass-flow-to-0-point-75-kg-per-second |
      | change-hr-cp-j-kgk                  | 🛋️change-hr-cp-j-kgk                 | corrects-the-air-specific-heat-to-1010-j-per-kgk               |
      | change-hr-delta-tc                  | 🛏️change-hr-delta-tc                 | drops-the-heat-recovery-temperature-lift-to-12-point-5-c       |
      | change-hr-th                        | 🌿change-hr-th                        | extends-the-heat-recovery-operating-hours-to-14                |
      | change-hr-savings-reference-kwh     | 🛁change-hr-savings-reference-kwh     | raises-the-heat-recovery-savings-reference-to-65-kwh           |
      | change-n50-h-inv                    | 🌲change-n50-h-inv                    | loosens-the-blower-door-result-to-2-point-5-per-hour           |
      | change-volume-m3                    | 🗻change-volume-m3                    | grows-the-air-volume-to-640-m3                                 |
      | change-infiltration-allowance-m3-h  | 🌴change-infiltration-allowance-m3-h  | raises-the-infiltration-allowance-to-52-point-5-m3-per-hour    |
      | change-cellar-area-m2               | 🛡️change-cellar-area-m2              | grows-the-cellar-floor-area-to-62-point-5-m2                   |
      | change-cellar-ventilation-m3-h      | 🧯change-cellar-ventilation-m3-h      | raises-the-cellar-airflow-to-22-point-5-m3-per-hour            |
      | change-h-tr-wk                      | 🧹change-h-tr-wk                      | improves-the-transmission-heat-transfer-to-175-w-per-k         |
      | change-h-ve-wk                      | 🧺change-h-ve-wk                      | raises-the-ventilation-heat-transfer-to-125-w-per-k            |
      | change-theta-ec                     | 🪨change-theta-ec                     | raises-the-external-design-temperature-to-34-point-5-c         |
      | change-theta-set-c                  | 🌎️change-theta-set-c                 | lowers-the-cooling-set-point-to-25-c                           |
      | change-cooling-delta-th             | 🪚change-cooling-delta-th             | extends-the-cooling-period-to-12-point-5-hours                 |
      | change-cooling-gains-kwh            | 🪜change-cooling-gains-kwh            | raises-the-internal-cooling-gains-to-7-point-5-kwh             |
      | change-cooling-utilization-factor   | 🪣change-cooling-utilization-factor   | raises-the-cooling-utilization-factor-to-0-point-875           |
      | change-cooling-reference-kwh        | 🪝change-cooling-reference-kwh        | raises-the-cooling-reference-to-25-kwh                         |
      | change-chiller-type                 | 🚨change-chiller-type                 | switches-to-a-water-cooled-chiller                             |
      | change-eer-actual                   | 🪤change-eer-actual                   | raises-the-achieved-eer-to-3-point-5                           |
      | change-qc-kwh                       | 🌷change-qc-kwh                       | raises-the-annual-cooling-demand-to-1250-kwh                   |
      | change-generation-reference-kwh     | 🧽change-generation-reference-kwh     | raises-the-generation-reference-to-450-kwh                     |
      | change-data-center-supply-c         | 🧰change-data-center-supply-c         | raises-the-data-centre-supply-air-to-27-c                      |
      | change-h-st-wk                      | 🪠change-h-st-wk                      | raises-the-storage-loss-coefficient-to-6-point-5-w-per-k       |
      | change-theta-st-c                   | 🌏️change-theta-st-c                  | lowers-the-storage-temperature-to-55-c                         |
      | change-theta-amb-c                  | 🐚change-theta-amb-c                  | lowers-the-storage-room-ambient-to-18-c                        |
      | change-storage-th                   | 🍄change-storage-th                   | shortens-the-storage-standby-period-to-18-hours                |
      | change-storage-allowance-kwh        | 🌼change-storage-allowance-kwh        | tightens-the-storage-loss-allowance-to-4-point-5-kwh           |
      | change-dhw-delivery-c               | 🧶change-dhw-delivery-c               | raises-the-dhw-delivery-temperature-to-60-c                    |
      | change-duct-class                   | 🪡change-duct-class                   | upgrades-the-duct-tightness-class-to-d                         |
      | change-duct-test-pressure-pa        | 🧷change-duct-test-pressure-pa        | raises-the-duct-test-pressure-to-500-pa                        |
      | change-duct-leakage-m3-sm2          | 🪢change-duct-leakage-m3-sm2          | halves-the-measured-duct-leakage-to-0-point-0625               |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed DIN EN 16798 document from the parsed carrier
    Given the real committed text artifact asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
