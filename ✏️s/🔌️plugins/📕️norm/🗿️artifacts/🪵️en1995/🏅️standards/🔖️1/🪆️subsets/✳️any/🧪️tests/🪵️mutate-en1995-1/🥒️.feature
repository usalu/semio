@capability-en1995-1-mutate
@oracle-en1995-1-python-independent
@comparison-ordered-json-v1
@mutations-en1995-1-any
Feature: Apply every typed EN 1995 mutation against an independent Python implementation
  `s.norm.en1995` is a semio-NATIVE artifact and no third party reads or writes it — checked, not
  assumed: PyPI serves no `en1995` distribution, and the nearest real packages (`structuralcodes`,
  `anastruct`) implement design-code FORMULAE and speak no interchange format, so none of them could be
  authoritative over this subset's `En1995Mutation` vocabulary. The second producer a differential
  comparison needs is therefore a second IMPLEMENTATION: the shared Python norm vocabulary that
  `🐍️.py` beside this file parameterises with this subset's catalog, vectors and example document.

  The vocabulary is HIERARCHICAL: the document holds id-addressed `members`, each with an id-addressed
  `actions` table, and id-addressed `connections`, each with its own `actions` table. All 66 kinds are
  covered — `change-annex`; `insert-`/`remove-member`; one `change-member-<field>` per member scalar
  (labels, role, strength and service class, support, section, lengths, notch, M_crit, masses,
  damping, fire duration and the EN 1995-2 bridge inputs); `insert-`/`remove-member-action` and one
  `change-member-action-<field>` per action scalar; and the same three shapes for connections and
  their fastener actions. Member, connection and action edits address their target by id
  (`memberId`, `connectionId`, `actionId`); inserts and removals address by index.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path
  below is a committed vector under `🧫️fixtures/🧬️mutations`, written by the production JSON codec
  from the default floor-beam document (regenerated with `EN1995_REGEN_MUTATION_VECTORS=1` and replayed
  by `committed_mutation_vectors_replay_for_every_kind`). Each side asserts the same laws in role — the
  applied document must BE the committed after-snapshot, an `applied` vector must move the document,
  and the mutation followed by its OWN computed inverse must restore the before-snapshot exactly
  (position included for inserts and removals).

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `🖼️assets/🏠️glulam-floor-beam/🏠️glulam-floor-beam/🗣️.dsl.semio`, the default document every vector
  starts from. Its nested `members`/`connections` tables use the table notation the grammar-less
  Python carrier refuses, so that scenario is compared at the envelope preamble, the body lines and the
  digest and length of what each side re-emitted.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed specification vector
    Given the committed before-snapshot shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload shared://🧬️mutations/<dir>/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome shared://🧬️mutations/<dir>/<fixture>/🎯️outcome/🔣️.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot under the committed outcome status and the two agree
    Examples:
      | id                                      | dir                                       | fixture                                                   |
      | change-annex                            | 🌍️change-annex                            | 🌍️switches-from-the-german-na-to-the-recommended-en-annex |
      | insert-member                           | ➕️insert-member                           | ➕️inserts-a-member-at-end                                 |
      | remove-member                           | ➖️remove-member                           | ➖️removes-the-first-member                                |
      | change-member-label-en                  | 🏷️change-member-label-en                  | ✏️sets-labelEn                                            |
      | change-member-label-de                  | 🏷️change-member-label-de                  | ✏️sets-labelDe                                            |
      | change-member-role                      | 🎯️change-member-role                      | ✏️sets-role                                               |
      | change-member-strength-class            | 🛡️change-member-strength-class            | ✏️sets-strengthClass                                      |
      | change-member-service-class             | 🌧️change-member-service-class             | ✏️sets-serviceClass                                       |
      | change-member-support                   | 📍️change-member-support                   | ✏️sets-support                                            |
      | change-member-b                         | ↔️change-member-b                         | ✏️sets-bM                                                 |
      | change-member-h                         | ↕️change-member-h                         | ✏️sets-hM                                                 |
      | change-member-span                      | ↔️change-member-span                      | ✏️sets-spanM                                              |
      | change-member-support-length            | ↔️change-member-support-length            | ✏️sets-supportLengthM                                     |
      | change-member-bearing-length            | ↔️change-member-bearing-length            | ✏️sets-bearingLengthM                                     |
      | change-member-buckling-y                | ↔️change-member-buckling-y                | ✏️sets-bucklingLengthYM                                   |
      | change-member-buckling-z                | ↔️change-member-buckling-z                | ✏️sets-bucklingLengthZM                                   |
      | change-member-lateral-restraint         | ↔️change-member-lateral-restraint         | ✏️sets-lateralRestraintSpacingM                           |
      | change-member-notch-depth               | ↔️change-member-notch-depth               | ✏️sets-notchDepthM                                        |
      | change-member-notch-distance            | ↔️change-member-notch-distance            | ✏️sets-notchDistanceM                                     |
      | change-member-m-crit                    | ⚠️change-member-m-crit                    | ✏️sets-mCritNm                                            |
      | change-member-mass-per-m                | ⚖️change-member-mass-per-m                | ✏️sets-massKgPerM                                         |
      | change-member-mass-per-m2               | ⚖️change-member-mass-per-m2               | ✏️sets-massKgPerM2                                        |
      | change-member-damping                   | 🌊️change-member-damping                   | ✏️sets-dampingXi                                          |
      | change-member-fire-duration             | 🔥️change-member-fire-duration             | ✏️sets-fireDurationS                                      |
      | change-member-bridge-n-obs              | 🌉️change-member-bridge-n-obs              | ✏️sets-bridgeNObs                                         |
      | change-member-bridge-tl-years           | 🌉️change-member-bridge-tl-years           | ✏️sets-bridgeTLYears                                      |
      | change-member-bridge-beta               | 🌉️change-member-bridge-beta               | ✏️sets-bridgeBeta                                         |
      | change-member-bridge-a                  | 🌉️change-member-bridge-a                  | ✏️sets-bridgeA                                            |
      | change-member-bridge-b                  | 🌉️change-member-bridge-b                  | ✏️sets-bridgeB                                            |
      | change-member-bridge-crowd              | 🚶️change-member-bridge-crowd              | ✏️sets-bridgeCrowdPerM2                                   |
      | insert-member-action                    | ➕️insert-member-action                    | ➕️inserts-an-action-at-end-of-member                      |
      | remove-member-action                    | ➖️remove-member-action                    | ➖️removes-the-first-action-of-member                      |
      | change-member-action-kind               | ⚖️change-member-action-kind               | ✏️sets-kind                                               |
      | change-member-action-category           | 🏢️change-member-action-category           | ✏️sets-category                                           |
      | change-member-action-load-duration      | ⏳️change-member-action-load-duration      | ✏️sets-loadDuration                                       |
      | change-member-action-q-line             | ⬇️change-member-action-q-line             | ✏️sets-qLineNPerM                                         |
      | change-member-action-f-point            | ⬇️change-member-action-f-point            | ✏️sets-fPointN                                            |
      | change-member-action-mk                 | ⤴️change-member-action-mk                 | ✏️sets-mKNm                                               |
      | change-member-action-vk                 | ↕️change-member-action-vk                 | ✏️sets-vKN                                                |
      | change-member-action-nk                 | 🏋️change-member-action-nk                 | ✏️sets-nKN                                                |
      | change-member-action-ntk                | 🏋️change-member-action-ntk                | ✏️sets-nTKN                                               |
      | change-member-action-fc90-k             | 🏋️change-member-action-fc90-k             | ✏️sets-fC90KN                                             |
      | insert-connection                       | ➕️insert-connection                       | ➕️inserts-a-connection-at-end                             |
      | remove-connection                       | ➖️remove-connection                       | ➖️removes-the-first-connection                            |
      | change-connection-label-en              | 🏷️change-connection-label-en              | ✏️sets-labelEn                                            |
      | change-connection-label-de              | 🏷️change-connection-label-de              | ✏️sets-labelDe                                            |
      | change-connection-fastener-type         | 🔩️change-connection-fastener-type         | ✏️sets-fastenerType                                       |
      | change-connection-strength-class        | 🛡️change-connection-strength-class        | ✏️sets-strengthClass                                      |
      | change-connection-service-class         | 🌧️change-connection-service-class         | ✏️sets-serviceClass                                       |
      | change-connection-diameter              | ↔️change-connection-diameter              | ✏️sets-diameterM                                          |
      | change-connection-number                | 🔢️change-connection-number                | ✏️sets-number                                             |
      | change-connection-rows                  | 🔢️change-connection-rows                  | ✏️sets-rows                                               |
      | change-connection-spacing               | ↔️change-connection-spacing               | ✏️sets-spacingM                                           |
      | change-connection-edge-distance         | ↔️change-connection-edge-distance         | ✏️sets-edgeDistanceM                                      |
      | change-connection-end-distance          | ↔️change-connection-end-distance          | ✏️sets-endDistanceM                                       |
      | change-connection-t1                    | ↔️change-connection-t1                    | ✏️sets-t1M                                                |
      | change-connection-t2                    | ↔️change-connection-t2                    | ✏️sets-t2M                                                |
      | change-connection-steel-plate           | 🔩️change-connection-steel-plate           | ✏️sets-steelPlate                                         |
      | change-connection-steel-plate-thickness | ↔️change-connection-steel-plate-thickness | ✏️sets-steelPlateThicknessM                               |
      | change-connection-shear-planes          | 🔢️change-connection-shear-planes          | ✏️sets-shearPlanes                                        |
      | change-connection-fuk                   | 🛡️change-connection-fuk                   | ✏️sets-fUK                                                |
      | insert-connection-action                | ➕️insert-connection-action                | ➕️inserts-an-action-at-end-of-connection                  |
      | remove-connection-action                | ➖️remove-connection-action                | ➖️removes-the-first-action-of-connection                  |
      | change-connection-action-kind           | ⚖️change-connection-action-kind           | ✏️sets-kind                                               |
      | change-connection-action-load-duration  | ⏳️change-connection-action-load-duration  | ✏️sets-loadDuration                                       |
      | change-connection-action-fk             | 🔩️change-connection-action-fk             | ✏️sets-fKN                                                |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores its committed before-snapshot
    Given the committed before-snapshot shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload shared://🧬️mutations/<dir>/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome shared://🧬️mutations/<dir>/<fixture>/🎯️outcome/🔣️.json
    When each implementation applies the committed mutation and then its OWN computed inverse
    Then both restore the before-snapshot and agree on the mutated and the restored document
    Examples:
      | id                                      | dir                                       | fixture                                                   |
      | change-annex                            | 🌍️change-annex                            | 🌍️switches-from-the-german-na-to-the-recommended-en-annex |
      | insert-member                           | ➕️insert-member                           | ➕️inserts-a-member-at-end                                 |
      | remove-member                           | ➖️remove-member                           | ➖️removes-the-first-member                                |
      | change-member-label-en                  | 🏷️change-member-label-en                  | ✏️sets-labelEn                                            |
      | change-member-label-de                  | 🏷️change-member-label-de                  | ✏️sets-labelDe                                            |
      | change-member-role                      | 🎯️change-member-role                      | ✏️sets-role                                               |
      | change-member-strength-class            | 🛡️change-member-strength-class            | ✏️sets-strengthClass                                      |
      | change-member-service-class             | 🌧️change-member-service-class             | ✏️sets-serviceClass                                       |
      | change-member-support                   | 📍️change-member-support                   | ✏️sets-support                                            |
      | change-member-b                         | ↔️change-member-b                         | ✏️sets-bM                                                 |
      | change-member-h                         | ↕️change-member-h                         | ✏️sets-hM                                                 |
      | change-member-span                      | ↔️change-member-span                      | ✏️sets-spanM                                              |
      | change-member-support-length            | ↔️change-member-support-length            | ✏️sets-supportLengthM                                     |
      | change-member-bearing-length            | ↔️change-member-bearing-length            | ✏️sets-bearingLengthM                                     |
      | change-member-buckling-y                | ↔️change-member-buckling-y                | ✏️sets-bucklingLengthYM                                   |
      | change-member-buckling-z                | ↔️change-member-buckling-z                | ✏️sets-bucklingLengthZM                                   |
      | change-member-lateral-restraint         | ↔️change-member-lateral-restraint         | ✏️sets-lateralRestraintSpacingM                           |
      | change-member-notch-depth               | ↔️change-member-notch-depth               | ✏️sets-notchDepthM                                        |
      | change-member-notch-distance            | ↔️change-member-notch-distance            | ✏️sets-notchDistanceM                                     |
      | change-member-m-crit                    | ⚠️change-member-m-crit                    | ✏️sets-mCritNm                                            |
      | change-member-mass-per-m                | ⚖️change-member-mass-per-m                | ✏️sets-massKgPerM                                         |
      | change-member-mass-per-m2               | ⚖️change-member-mass-per-m2               | ✏️sets-massKgPerM2                                        |
      | change-member-damping                   | 🌊️change-member-damping                   | ✏️sets-dampingXi                                          |
      | change-member-fire-duration             | 🔥️change-member-fire-duration             | ✏️sets-fireDurationS                                      |
      | change-member-bridge-n-obs              | 🌉️change-member-bridge-n-obs              | ✏️sets-bridgeNObs                                         |
      | change-member-bridge-tl-years           | 🌉️change-member-bridge-tl-years           | ✏️sets-bridgeTLYears                                      |
      | change-member-bridge-beta               | 🌉️change-member-bridge-beta               | ✏️sets-bridgeBeta                                         |
      | change-member-bridge-a                  | 🌉️change-member-bridge-a                  | ✏️sets-bridgeA                                            |
      | change-member-bridge-b                  | 🌉️change-member-bridge-b                  | ✏️sets-bridgeB                                            |
      | change-member-bridge-crowd              | 🚶️change-member-bridge-crowd              | ✏️sets-bridgeCrowdPerM2                                   |
      | insert-member-action                    | ➕️insert-member-action                    | ➕️inserts-an-action-at-end-of-member                      |
      | remove-member-action                    | ➖️remove-member-action                    | ➖️removes-the-first-action-of-member                      |
      | change-member-action-kind               | ⚖️change-member-action-kind               | ✏️sets-kind                                               |
      | change-member-action-category           | 🏢️change-member-action-category           | ✏️sets-category                                           |
      | change-member-action-load-duration      | ⏳️change-member-action-load-duration      | ✏️sets-loadDuration                                       |
      | change-member-action-q-line             | ⬇️change-member-action-q-line             | ✏️sets-qLineNPerM                                         |
      | change-member-action-f-point            | ⬇️change-member-action-f-point            | ✏️sets-fPointN                                            |
      | change-member-action-mk                 | ⤴️change-member-action-mk                 | ✏️sets-mKNm                                               |
      | change-member-action-vk                 | ↕️change-member-action-vk                 | ✏️sets-vKN                                                |
      | change-member-action-nk                 | 🏋️change-member-action-nk                 | ✏️sets-nKN                                                |
      | change-member-action-ntk                | 🏋️change-member-action-ntk                | ✏️sets-nTKN                                               |
      | change-member-action-fc90-k             | 🏋️change-member-action-fc90-k             | ✏️sets-fC90KN                                             |
      | insert-connection                       | ➕️insert-connection                       | ➕️inserts-a-connection-at-end                             |
      | remove-connection                       | ➖️remove-connection                       | ➖️removes-the-first-connection                            |
      | change-connection-label-en              | 🏷️change-connection-label-en              | ✏️sets-labelEn                                            |
      | change-connection-label-de              | 🏷️change-connection-label-de              | ✏️sets-labelDe                                            |
      | change-connection-fastener-type         | 🔩️change-connection-fastener-type         | ✏️sets-fastenerType                                       |
      | change-connection-strength-class        | 🛡️change-connection-strength-class        | ✏️sets-strengthClass                                      |
      | change-connection-service-class         | 🌧️change-connection-service-class         | ✏️sets-serviceClass                                       |
      | change-connection-diameter              | ↔️change-connection-diameter              | ✏️sets-diameterM                                          |
      | change-connection-number                | 🔢️change-connection-number                | ✏️sets-number                                             |
      | change-connection-rows                  | 🔢️change-connection-rows                  | ✏️sets-rows                                               |
      | change-connection-spacing               | ↔️change-connection-spacing               | ✏️sets-spacingM                                           |
      | change-connection-edge-distance         | ↔️change-connection-edge-distance         | ✏️sets-edgeDistanceM                                      |
      | change-connection-end-distance          | ↔️change-connection-end-distance          | ✏️sets-endDistanceM                                       |
      | change-connection-t1                    | ↔️change-connection-t1                    | ✏️sets-t1M                                                |
      | change-connection-t2                    | ↔️change-connection-t2                    | ✏️sets-t2M                                                |
      | change-connection-steel-plate           | 🔩️change-connection-steel-plate           | ✏️sets-steelPlate                                         |
      | change-connection-steel-plate-thickness | ↔️change-connection-steel-plate-thickness | ✏️sets-steelPlateThicknessM                               |
      | change-connection-shear-planes          | 🔢️change-connection-shear-planes          | ✏️sets-shearPlanes                                        |
      | change-connection-fuk                   | 🛡️change-connection-fuk                   | ✏️sets-fUK                                                |
      | insert-connection-action                | ➕️insert-connection-action                | ➕️inserts-an-action-at-end-of-connection                  |
      | remove-connection-action                | ➖️remove-connection-action                | ➖️removes-the-first-action-of-connection                  |
      | change-connection-action-kind           | ⚖️change-connection-action-kind           | ✏️sets-kind                                               |
      | change-connection-action-load-duration  | ⏳️change-connection-action-load-duration  | ✏️sets-loadDuration                                       |
      | change-connection-action-fk             | 🔩️change-connection-action-fk             | ✏️sets-fKN                                                |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1995 document from the parsed carrier
    Given the real committed text artifact asset://🏠️glulam-floor-beam/🏠️glulam-floor-beam/🗣️.dsl.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
