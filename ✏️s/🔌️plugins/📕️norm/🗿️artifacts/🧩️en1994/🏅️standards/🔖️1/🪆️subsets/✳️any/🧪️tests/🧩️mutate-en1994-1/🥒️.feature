@capability-en1994-1-mutate
@oracle-en1994-1-python-independent
@comparison-ordered-json-v1
@mutations-en1994-1-any
Feature: Apply every typed EN 1994 mutation against an independent Python implementation
  `s.norm.en1994` is a semio-NATIVE artifact and no third party reads or writes it, so the second producer a
  differential comparison needs is a second IMPLEMENTATION: the shared norm reference engine
  (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`), which `🐍️.py` beside this file feeds with this subset's
  kind list, vectors and carrier. It is written from the repository's own specification of what a semantic
  mutation means (the verb table, the `new<Field>` naming mechanic, the addressing convention and the derivation
  rules) and imports nothing from the Rust it judges.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below is a
  declared `shared://` fixture, so neither side holds a transcription that could drift. The 31 `✅apply` vectors
  cover every kind of the current vocabulary (19 `change`, 3 `insert`, 3 `remove`) on a steel-concrete composite bridge girder; each vector's after-snapshot and
  diff were written by production dispatch and its mutation is the canonical Rust wire. Each side asserts the same
  laws in role — the applied document must BE the committed after-snapshot, an `applied` vector must move the
  document, and the mutation followed by its OWN computed inverse must restore the before-snapshot exactly, list
  position included. `parity` adds that two implementations, in two languages, reach the same document.

  `inverse-` projects BOTH the mutated and the restored document, so the mutated half distinguishes the rows.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `asset://🌉️composite-bridge-girder/🌉️composite-bridge-girder/🗣️.dsl.semio`. The carrier has no published grammar: the committed
  `📖️component.grammar.semio` is the repository-wide `payload = OCTET+` rule, so the two sides are compared
  at the envelope preamble, the ordered lines and the digest and length of what each re-emitted. The Rust side additionally proves it PARSED the document: the committed binary
  twin must decode to the same document as the text, through a separately written codec.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed specification vector
    Given the committed before-snapshot shared://🧬️mutations/<dir>/<fixture>/🔣️.before.json
    And the committed mutation payload shared://🧬️mutations/<dir>/<fixture>/🔣️.mutation.json
    And the committed after-snapshot shared://🧬️mutations/<dir>/<fixture>/🔣️.after.json
    And the committed outcome shared://🧬️mutations/<dir>/<fixture>/🔣️.outcome.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot under the committed outcome status and the two agree
    Examples:
      | id                            | dir                             | fixture |
      | change-annex                  | 🌍️change-annex                  | ✅apply  |
      | change-reliability-class                  | 🎯change-reliability-class                  | ✅apply  |
      | change-structure-kind         | 🏗️change-structure-kind         | ✅apply  |
      | change-steel-fy-pa            | 🏋️change-steel-fy-pa            | ✅apply  |
      | change-fire-rating            | 🔥️change-fire-rating            | ✅apply  |
      | change-insulation-thickness-m | 🧯️change-insulation-thickness-m | ✅apply  |
      | change-fatigue-detail         | 🔁️change-fatigue-detail         | ✅apply  |
      | insert-beam                   | ➕️insert-beam                   | ✅apply  |
      | remove-beam                   | ➖️remove-beam                   | ✅apply  |
      | change-beam-action-q-area-pa  | 🌀️change-beam-action-q-area-pa  | ✅apply  |
      | change-beam-span-m            | 📏️change-beam-span-m            | ✅apply  |
      | change-beam-slab-thickness-m  | 🧱change-beam-slab-thickness-m   | ✅apply  |
      | change-beam-action-xm | 🏋️change-beam-action-xm | ✅apply  |
      | change-slab-action-xm | 🧱️change-slab-action-xm | ✅apply  |
      | change-slab-action-ym | 🧱️change-slab-action-ym | ✅apply  |
      | insert-beam-action | 🏋️insert-beam-action | ✅apply  |
      | remove-beam-action | 🏋️remove-beam-action | ✅apply  |
      | insert-column-action | 🏛️insert-column-action | ✅apply  |
      | remove-column-action | 🏛️remove-column-action | ✅apply  |
      | insert-slab-action | 🧱️insert-slab-action | ✅apply  |
      | remove-slab-action | 🧱️remove-slab-action | ✅apply  |
      | change-placement-x | 📍️change-placement-x | ✅apply  |
      | change-placement-y | 📍️change-placement-y | ✅apply  |
      | change-placement-z | 📍️change-placement-z | ✅apply  |
      | change-placement-yaw | 📍️change-placement-yaw | ✅apply  |
      | insert-placement | 📍️insert-placement | ✅apply  |
      | remove-placement | 📍️remove-placement | ✅apply  |
      | insert-stud | 📍️insert-stud | ✅apply  |
      | remove-stud | 📍️remove-stud | ✅apply  |
      | insert-case | ➕️insert-case | ✅apply  |
      | remove-case | ➖️remove-case | ✅apply  |
      | change-case-number | 🔢️change-case-number | ✅apply  |
      | change-case-text | 🔤️change-case-text | ✅apply  |
      | change-case-label-en | 🏷️change-case-label-en | ✅apply  |
      | change-case-label-de | 🏷️change-case-label-de | ✅apply  |
      | change-case-entry-id | 📇️change-case-entry-id | ✅apply  |
      | change-beam-transverse-as     | ↔️change-beam-transverse-as     | ✅apply  |
      | change-beam-construction      | 🛠️change-beam-construction      | ✅apply  |
      | insert-column                 | ➗️insert-column                 | ✅apply  |
      | remove-column                 | ⛔️remove-column                 | ✅apply  |
      | change-column-action-force-n  | ⬇️change-column-action-force-n  | ✅apply  |
      | change-column-kind            | ↪️change-column-kind            | ✅apply  |
      | insert-slab                   | ➕insert-slab                    | ✅apply  |
      | remove-slab                   | ➖remove-slab                    | ✅apply  |
      | change-slab-action-q-area-pa  | 📐️change-slab-action-q-area-pa  | ✅apply  |
      | change-slab-thickness-m       | 📏change-slab-thickness-m        | ✅apply  |
      | change-beam-spacing-m | ↔️change-beam-spacing-m | ✅apply  |
      | change-beam-support | 🪢️change-beam-support | ✅apply  |
      | change-beam-rebar-product | 🧵️change-beam-rebar-product | ✅apply  |
      | change-beam-concrete-f-ck-pa | 🪨️change-beam-concrete-f-ck-pa | ✅apply  |
      | change-beam-concrete-e-cm-pa | 🧱️change-beam-concrete-e-cm-pa | ✅apply  |
      | change-beam-ltb-length-m | 🌀️change-beam-ltb-length-m | ✅apply  |
      | change-beam-hogging-rebar-m2-per-m | 🕸️change-beam-hogging-rebar-m2-per-m | ✅apply  |
      | change-beam-bar-spacing-m | 📏️change-beam-bar-spacing-m | ✅apply  |
      | change-beam-crack-width-limit-m | 🧬️change-beam-crack-width-limit-m | ✅apply  |
      | change-beam-fatigue-cycles | 🔁️change-beam-fatigue-cycles | ✅apply  |
      | change-beam-steel-designation | 🔩️change-beam-steel-designation | ✅apply  |
      | change-beam-steel-height-m | ↕️change-beam-steel-height-m | ✅apply  |
      | change-beam-steel-width-m | ↔️change-beam-steel-width-m | ✅apply  |
      | change-beam-steel-web-thickness-m | 🧱️change-beam-steel-web-thickness-m | ✅apply  |
      | change-beam-steel-tf-m | 🧱️change-beam-steel-tf-m | ✅apply  |
      | change-beam-steel-area-m2 | ⬛️change-beam-steel-area-m2 | ✅apply  |
      | change-beam-steel-w-pl-ym3 | 📐️change-beam-steel-w-pl-ym3 | ✅apply  |
      | change-beam-steel-second-moment-m4 | 📐️change-beam-steel-second-moment-m4 | ✅apply  |
      | change-beam-steel-shear-area-m2 | ✂️change-beam-steel-shear-area-m2 | ✅apply  |
      | change-beam-steel-root-radius-m | ⭕️change-beam-steel-root-radius-m | ✅apply  |
      | change-beam-sheeting-profile | 🏗️change-beam-sheeting-profile | ✅apply  |
      | change-beam-sheeting-height-m | 🏗️change-beam-sheeting-height-m | ✅apply  |
      | change-beam-sheeting-rib-width-m | 🏗️change-beam-sheeting-rib-width-m | ✅apply  |
      | change-beam-sheeting-thickness-m | 🏗️change-beam-sheeting-thickness-m | ✅apply  |
      | change-beam-sheeting-ribs-parallel | 🧭️change-beam-sheeting-ribs-parallel | ✅apply  |
      | change-beam-action-kind | 🏋️change-beam-action-kind | ✅apply  |
      | change-beam-action-category | 🏷️change-beam-action-category | ✅apply  |
      | change-beam-action-stage | 🧭️change-beam-action-stage | ✅apply  |
      | change-beam-action-point-force-n | 📍️change-beam-action-point-force-n | ✅apply  |
      | change-beam-action-delta-sigma-pa | 🔁️change-beam-action-delta-sigma-pa | ✅apply  |
      | change-beam-action-delta-tau-pa | 🔁️change-beam-action-delta-tau-pa | ✅apply  |
      | change-column-length-m | 📏️change-column-length-m | ✅apply  |
      | change-column-outer-size-m | ⬜️change-column-outer-size-m | ✅apply  |
      | change-column-wall-thickness-m | 🧱️change-column-wall-thickness-m | ✅apply  |
      | change-column-steel-area-m2 | ⬛️change-column-steel-area-m2 | ✅apply  |
      | change-column-steel-fy-pa | 🔩️change-column-steel-fy-pa | ✅apply  |
      | change-column-concrete-area-m2 | 🪨️change-column-concrete-area-m2 | ✅apply  |
      | change-column-concrete-f-ck-pa | 🪨️change-column-concrete-f-ck-pa | ✅apply  |
      | change-column-rebar-area-m2 | 🕸️change-column-rebar-area-m2 | ✅apply  |
      | change-column-rebar-f-yk-pa | 🕸️change-column-rebar-f-yk-pa | ✅apply  |
      | change-column-second-moment-m4 | 📐️change-column-second-moment-m4 | ✅apply  |
      | change-column-buckling-curve | 📉️change-column-buckling-curve | ✅apply  |
      | change-column-rebar-product | 🧵️change-column-rebar-product | ✅apply  |
      | change-column-action-kind | 🏋️change-column-action-kind | ✅apply  |
      | change-column-action-category | 🏷️change-column-action-category | ✅apply  |
      | change-column-action-stage | 🧭️change-column-action-stage | ✅apply  |
      | change-column-action-moment-nm | 🔄️change-column-action-moment-nm | ✅apply  |
      | change-slab-span-m | 📏️change-slab-span-m | ✅apply  |
      | change-slab-support | 🪢️change-slab-support | ✅apply  |
      | change-slab-f-ck-pa | 🪨️change-slab-f-ck-pa | ✅apply  |
      | change-slab-m-factor | 🧮️change-slab-m-factor | ✅apply  |
      | change-slab-k-factor | 🧮️change-slab-k-factor | ✅apply  |
      | change-slab-reinforcement-m2-per-m | 🕸️change-slab-reinforcement-m2-per-m | ✅apply  |
      | change-slab-sheeting-profile | 🏗️change-slab-sheeting-profile | ✅apply  |
      | change-slab-sheeting-height-m | 🏗️change-slab-sheeting-height-m | ✅apply  |
      | change-slab-sheeting-rib-width-m | 🏗️change-slab-sheeting-rib-width-m | ✅apply  |
      | change-slab-sheeting-thickness-m | 🏗️change-slab-sheeting-thickness-m | ✅apply  |
      | change-slab-sheeting-ribs-parallel | 🧭️change-slab-sheeting-ribs-parallel | ✅apply  |
      | change-slab-action-kind | 🏋️change-slab-action-kind | ✅apply  |
      | change-slab-action-category | 🏷️change-slab-action-category | ✅apply  |
      | change-slab-action-stage | 🧭️change-slab-action-stage | ✅apply  |
      | change-slab-action-point-force-n | 📍️change-slab-action-point-force-n | ✅apply  |
      | change-slab-action-delta-sigma-pa | 🔁️change-slab-action-delta-sigma-pa | ✅apply  |
      | change-slab-action-delta-tau-pa | 🔁️change-slab-action-delta-tau-pa | ✅apply  |
      | change-beam-adjacent-span-m | ↔️change-beam-adjacent-span-m | ✅apply  |
      | change-beam-patch-bearing-length-m | 📍️change-beam-patch-bearing-length-m | ✅apply  |
      | change-beam-bridge-lanes | 🛣️change-beam-bridge-lanes | ✅apply  |
      | change-column-introduced-force-n | ⬇️change-column-introduced-force-n | ✅apply  |
      | change-beam-cracked-length-fraction | 〰️change-beam-cracked-length-fraction | ✅apply  |
      | change-column-section-height-m | ↕️change-column-section-height-m | ✅apply  |
      | change-column-section-width-m | ↔️change-column-section-width-m | ✅apply  |
      | change-column-section-tw-m | ▮change-column-section-tw-m | ✅apply  |
      | change-column-section-tf-m | ▬change-column-section-tf-m | ✅apply  |
      | change-column-rebar-cover-m | 🔘change-column-rebar-cover-m | ✅apply  |
      | change-beam-stud-cross-spacing-m | ↔️change-beam-stud-cross-spacing-m | ✅apply  |
      | change-beam-stud-edge-distance-m | 📏️change-beam-stud-edge-distance-m | ✅apply  |
      | change-beam-transverse-as-top | 🕸️change-beam-transverse-as-top | ✅apply  |
      | change-beam-end-plate-width-m | ⬜️change-beam-end-plate-width-m | ✅apply  |
      | change-beam-end-plate-height-m | ↕️change-beam-end-plate-height-m | ✅apply  |
      | change-beam-end-zone-length-m | ➡️change-beam-end-zone-length-m | ✅apply  |
      | change-stud-fu-pa | ➡️change-stud-fu-pa | ✅apply  |
      | change-stud-height-m | ➡️change-stud-height-m | ✅apply  |
      | change-stud-diameter-m | ➡️change-stud-diameter-m | ✅apply  |
      | change-stud-xm | ➡️change-stud-xm | ✅apply  |
      | remove-column-middle          | ⛔️remove-column                 | 🧭️middle |
      | remove-slab-middle            | ➖remove-slab                    | 🧭️middle |
      | remove-beam-middle            | ➖️remove-beam                   | 🧭️middle |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores its committed before-snapshot
    Given the committed before-snapshot shared://🧬️mutations/<dir>/<fixture>/🔣️.before.json
    And the committed mutation payload shared://🧬️mutations/<dir>/<fixture>/🔣️.mutation.json
    And the committed after-snapshot shared://🧬️mutations/<dir>/<fixture>/🔣️.after.json
    And the committed outcome shared://🧬️mutations/<dir>/<fixture>/🔣️.outcome.json
    When each implementation applies the committed mutation and then its OWN computed inverse
    Then both restore the before-snapshot and agree on the mutated and the restored document
    Examples:
      | id                            | dir                             | fixture |
      | change-annex                  | 🌍️change-annex                  | ✅apply  |
      | change-reliability-class                  | 🎯change-reliability-class                  | ✅apply  |
      | change-structure-kind         | 🏗️change-structure-kind         | ✅apply  |
      | change-steel-fy-pa            | 🏋️change-steel-fy-pa            | ✅apply  |
      | change-fire-rating            | 🔥️change-fire-rating            | ✅apply  |
      | change-insulation-thickness-m | 🧯️change-insulation-thickness-m | ✅apply  |
      | change-fatigue-detail         | 🔁️change-fatigue-detail         | ✅apply  |
      | insert-beam                   | ➕️insert-beam                   | ✅apply  |
      | remove-beam                   | ➖️remove-beam                   | ✅apply  |
      | change-beam-action-q-area-pa  | 🌀️change-beam-action-q-area-pa  | ✅apply  |
      | change-beam-span-m            | 📏️change-beam-span-m            | ✅apply  |
      | change-beam-slab-thickness-m  | 🧱change-beam-slab-thickness-m   | ✅apply  |
      | change-beam-action-xm | 🏋️change-beam-action-xm | ✅apply  |
      | change-slab-action-xm | 🧱️change-slab-action-xm | ✅apply  |
      | change-slab-action-ym | 🧱️change-slab-action-ym | ✅apply  |
      | insert-beam-action | 🏋️insert-beam-action | ✅apply  |
      | remove-beam-action | 🏋️remove-beam-action | ✅apply  |
      | insert-column-action | 🏛️insert-column-action | ✅apply  |
      | remove-column-action | 🏛️remove-column-action | ✅apply  |
      | insert-slab-action | 🧱️insert-slab-action | ✅apply  |
      | remove-slab-action | 🧱️remove-slab-action | ✅apply  |
      | change-placement-x | 📍️change-placement-x | ✅apply  |
      | change-placement-y | 📍️change-placement-y | ✅apply  |
      | change-placement-z | 📍️change-placement-z | ✅apply  |
      | change-placement-yaw | 📍️change-placement-yaw | ✅apply  |
      | insert-placement | 📍️insert-placement | ✅apply  |
      | remove-placement | 📍️remove-placement | ✅apply  |
      | insert-stud | 📍️insert-stud | ✅apply  |
      | remove-stud | 📍️remove-stud | ✅apply  |
      | insert-case | ➕️insert-case | ✅apply  |
      | remove-case | ➖️remove-case | ✅apply  |
      | change-case-number | 🔢️change-case-number | ✅apply  |
      | change-case-text | 🔤️change-case-text | ✅apply  |
      | change-case-label-en | 🏷️change-case-label-en | ✅apply  |
      | change-case-label-de | 🏷️change-case-label-de | ✅apply  |
      | change-case-entry-id | 📇️change-case-entry-id | ✅apply  |
      | change-beam-transverse-as     | ↔️change-beam-transverse-as     | ✅apply  |
      | change-beam-construction      | 🛠️change-beam-construction      | ✅apply  |
      | insert-column                 | ➗️insert-column                 | ✅apply  |
      | remove-column                 | ⛔️remove-column                 | ✅apply  |
      | change-column-action-force-n  | ⬇️change-column-action-force-n  | ✅apply  |
      | change-column-kind            | ↪️change-column-kind            | ✅apply  |
      | insert-slab                   | ➕insert-slab                    | ✅apply  |
      | remove-slab                   | ➖remove-slab                    | ✅apply  |
      | change-slab-action-q-area-pa  | 📐️change-slab-action-q-area-pa  | ✅apply  |
      | change-slab-thickness-m       | 📏change-slab-thickness-m        | ✅apply  |
      | change-beam-spacing-m | ↔️change-beam-spacing-m | ✅apply  |
      | change-beam-support | 🪢️change-beam-support | ✅apply  |
      | change-beam-rebar-product | 🧵️change-beam-rebar-product | ✅apply  |
      | change-beam-concrete-f-ck-pa | 🪨️change-beam-concrete-f-ck-pa | ✅apply  |
      | change-beam-concrete-e-cm-pa | 🧱️change-beam-concrete-e-cm-pa | ✅apply  |
      | change-beam-ltb-length-m | 🌀️change-beam-ltb-length-m | ✅apply  |
      | change-beam-hogging-rebar-m2-per-m | 🕸️change-beam-hogging-rebar-m2-per-m | ✅apply  |
      | change-beam-bar-spacing-m | 📏️change-beam-bar-spacing-m | ✅apply  |
      | change-beam-crack-width-limit-m | 🧬️change-beam-crack-width-limit-m | ✅apply  |
      | change-beam-fatigue-cycles | 🔁️change-beam-fatigue-cycles | ✅apply  |
      | change-beam-steel-designation | 🔩️change-beam-steel-designation | ✅apply  |
      | change-beam-steel-height-m | ↕️change-beam-steel-height-m | ✅apply  |
      | change-beam-steel-width-m | ↔️change-beam-steel-width-m | ✅apply  |
      | change-beam-steel-web-thickness-m | 🧱️change-beam-steel-web-thickness-m | ✅apply  |
      | change-beam-steel-tf-m | 🧱️change-beam-steel-tf-m | ✅apply  |
      | change-beam-steel-area-m2 | ⬛️change-beam-steel-area-m2 | ✅apply  |
      | change-beam-steel-w-pl-ym3 | 📐️change-beam-steel-w-pl-ym3 | ✅apply  |
      | change-beam-steel-second-moment-m4 | 📐️change-beam-steel-second-moment-m4 | ✅apply  |
      | change-beam-steel-shear-area-m2 | ✂️change-beam-steel-shear-area-m2 | ✅apply  |
      | change-beam-steel-root-radius-m | ⭕️change-beam-steel-root-radius-m | ✅apply  |
      | change-beam-sheeting-profile | 🏗️change-beam-sheeting-profile | ✅apply  |
      | change-beam-sheeting-height-m | 🏗️change-beam-sheeting-height-m | ✅apply  |
      | change-beam-sheeting-rib-width-m | 🏗️change-beam-sheeting-rib-width-m | ✅apply  |
      | change-beam-sheeting-thickness-m | 🏗️change-beam-sheeting-thickness-m | ✅apply  |
      | change-beam-sheeting-ribs-parallel | 🧭️change-beam-sheeting-ribs-parallel | ✅apply  |
      | change-beam-action-kind | 🏋️change-beam-action-kind | ✅apply  |
      | change-beam-action-category | 🏷️change-beam-action-category | ✅apply  |
      | change-beam-action-stage | 🧭️change-beam-action-stage | ✅apply  |
      | change-beam-action-point-force-n | 📍️change-beam-action-point-force-n | ✅apply  |
      | change-beam-action-delta-sigma-pa | 🔁️change-beam-action-delta-sigma-pa | ✅apply  |
      | change-beam-action-delta-tau-pa | 🔁️change-beam-action-delta-tau-pa | ✅apply  |
      | change-column-length-m | 📏️change-column-length-m | ✅apply  |
      | change-column-outer-size-m | ⬜️change-column-outer-size-m | ✅apply  |
      | change-column-wall-thickness-m | 🧱️change-column-wall-thickness-m | ✅apply  |
      | change-column-steel-area-m2 | ⬛️change-column-steel-area-m2 | ✅apply  |
      | change-column-steel-fy-pa | 🔩️change-column-steel-fy-pa | ✅apply  |
      | change-column-concrete-area-m2 | 🪨️change-column-concrete-area-m2 | ✅apply  |
      | change-column-concrete-f-ck-pa | 🪨️change-column-concrete-f-ck-pa | ✅apply  |
      | change-column-rebar-area-m2 | 🕸️change-column-rebar-area-m2 | ✅apply  |
      | change-column-rebar-f-yk-pa | 🕸️change-column-rebar-f-yk-pa | ✅apply  |
      | change-column-second-moment-m4 | 📐️change-column-second-moment-m4 | ✅apply  |
      | change-column-buckling-curve | 📉️change-column-buckling-curve | ✅apply  |
      | change-column-rebar-product | 🧵️change-column-rebar-product | ✅apply  |
      | change-column-action-kind | 🏋️change-column-action-kind | ✅apply  |
      | change-column-action-category | 🏷️change-column-action-category | ✅apply  |
      | change-column-action-stage | 🧭️change-column-action-stage | ✅apply  |
      | change-column-action-moment-nm | 🔄️change-column-action-moment-nm | ✅apply  |
      | change-slab-span-m | 📏️change-slab-span-m | ✅apply  |
      | change-slab-support | 🪢️change-slab-support | ✅apply  |
      | change-slab-f-ck-pa | 🪨️change-slab-f-ck-pa | ✅apply  |
      | change-slab-m-factor | 🧮️change-slab-m-factor | ✅apply  |
      | change-slab-k-factor | 🧮️change-slab-k-factor | ✅apply  |
      | change-slab-reinforcement-m2-per-m | 🕸️change-slab-reinforcement-m2-per-m | ✅apply  |
      | change-slab-sheeting-profile | 🏗️change-slab-sheeting-profile | ✅apply  |
      | change-slab-sheeting-height-m | 🏗️change-slab-sheeting-height-m | ✅apply  |
      | change-slab-sheeting-rib-width-m | 🏗️change-slab-sheeting-rib-width-m | ✅apply  |
      | change-slab-sheeting-thickness-m | 🏗️change-slab-sheeting-thickness-m | ✅apply  |
      | change-slab-sheeting-ribs-parallel | 🧭️change-slab-sheeting-ribs-parallel | ✅apply  |
      | change-slab-action-kind | 🏋️change-slab-action-kind | ✅apply  |
      | change-slab-action-category | 🏷️change-slab-action-category | ✅apply  |
      | change-slab-action-stage | 🧭️change-slab-action-stage | ✅apply  |
      | change-slab-action-point-force-n | 📍️change-slab-action-point-force-n | ✅apply  |
      | change-slab-action-delta-sigma-pa | 🔁️change-slab-action-delta-sigma-pa | ✅apply  |
      | change-slab-action-delta-tau-pa | 🔁️change-slab-action-delta-tau-pa | ✅apply  |
      | change-beam-adjacent-span-m | ↔️change-beam-adjacent-span-m | ✅apply  |
      | change-beam-patch-bearing-length-m | 📍️change-beam-patch-bearing-length-m | ✅apply  |
      | change-beam-bridge-lanes | 🛣️change-beam-bridge-lanes | ✅apply  |
      | change-column-introduced-force-n | ⬇️change-column-introduced-force-n | ✅apply  |
      | change-beam-cracked-length-fraction | 〰️change-beam-cracked-length-fraction | ✅apply  |
      | change-column-section-height-m | ↕️change-column-section-height-m | ✅apply  |
      | change-column-section-width-m | ↔️change-column-section-width-m | ✅apply  |
      | change-column-section-tw-m | ▮change-column-section-tw-m | ✅apply  |
      | change-column-section-tf-m | ▬change-column-section-tf-m | ✅apply  |
      | change-column-rebar-cover-m | 🔘change-column-rebar-cover-m | ✅apply  |
      | change-beam-stud-cross-spacing-m | ↔️change-beam-stud-cross-spacing-m | ✅apply  |
      | change-beam-stud-edge-distance-m | 📏️change-beam-stud-edge-distance-m | ✅apply  |
      | change-beam-transverse-as-top | 🕸️change-beam-transverse-as-top | ✅apply  |
      | change-beam-end-plate-width-m | ⬜️change-beam-end-plate-width-m | ✅apply  |
      | change-beam-end-plate-height-m | ↕️change-beam-end-plate-height-m | ✅apply  |
      | change-beam-end-zone-length-m | ➡️change-beam-end-zone-length-m | ✅apply  |
      | change-stud-fu-pa | ➡️change-stud-fu-pa | ✅apply  |
      | change-stud-height-m | ➡️change-stud-height-m | ✅apply  |
      | change-stud-diameter-m | ➡️change-stud-diameter-m | ✅apply  |
      | change-stud-xm | ➡️change-stud-xm | ✅apply  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1994 document from the parsed carrier
    Given the real committed text artifact asset://🌉️composite-bridge-girder/🌉️composite-bridge-girder/🗣️.dsl.semio
    And its committed binary twin asset://🌉️composite-bridge-girder/🎒️.pack.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
