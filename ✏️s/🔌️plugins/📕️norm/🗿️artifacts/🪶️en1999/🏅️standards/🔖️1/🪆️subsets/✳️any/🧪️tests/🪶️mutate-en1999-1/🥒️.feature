@capability-en1999-1-mutate
@oracle-en1999-1-python-independent
@comparison-ordered-json-v1
@mutations-en1999-1-any
Feature: Apply every typed EN 1999 mutation against an independent Python implementation
  `s.norm.en1999` is a semio-NATIVE artifact and no third party reads or writes it, so the second producer a
  differential comparison needs is a second IMPLEMENTATION: the shared norm reference engine
  (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`), which `🐍️.py` beside this file feeds with this subset's
  kind list, vectors and carrier. It is written from the repository's own specification of what a semantic
  mutation means (the verb table, the `new<Field>` naming mechanic, the addressing convention and the derivation
  rules) and imports nothing from the Rust it judges.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below is a
  declared `shared://` fixture, so neither side holds a transcription that could drift. The 156 `✅apply` vectors
  cover every kind of the current vocabulary (133 `change`, 4 `add`, 4 `remove`, 1 annex) on an aluminium roof purlin with its connections; each vector's after-snapshot and
  diff were written by production dispatch and its mutation is the canonical Rust wire. Each side asserts the same
  laws in role — the applied document must BE the committed after-snapshot, an `applied` vector must move the
  document, and the mutation followed by its OWN computed inverse must restore the before-snapshot exactly, list
  position included. `parity` adds that two implementations, in two languages, reach the same document.

  The 4 refusal rows (`⛔dupe`) re-apply a kind's applied mutation to the
  after-snapshot it produced: re-inserting an id the collection now holds must be refused `mutation.duplicate-id`
  (Fatal), re-removing a member that is gone `mutation.target-missing` (Error) and re-setting a value the document
  already has must report `mutation.no-op` (Warning). Both sides must refuse under the committed code and leave the
  document bit-identical; a refusal has nothing to undo, so these rows are `mutate-` only.

  `inverse-` projects BOTH the mutated and the restored document, so the mutated half distinguishes the rows.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `asset://🏠️aluminium-roof-purlin/🏠️aluminium-roof-purlin/🗣️.dsl.semio`. The carrier has no published grammar: the committed
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
      | insert-member                    | ➕insert-member                     | ✅apply  |
      | insert-member-dupe               | ➕insert-member                     | ⛔dupe   |
      | insert-transverse-stiffener-dupe | ➕insert-transverse-stiffener | ⛔dupe  |
      | insert-member-action-dupe | ➕insert-member-action | ⛔dupe  |
      | insert-connection-action-dupe | ➕insert-connection-action | ⛔dupe  |
      | insert-section-element-dupe | ➕insert-section-element | ⛔dupe  |
      | insert-sheet-action-dupe | ➕insert-sheet-action | ⛔dupe  |
      | insert-shell-action-dupe | ➕insert-shell-action | ⛔dupe  |
      | insert-fatigue-category-dupe | ➕insert-fatigue-category | ⛔dupe  |
      | insert-patch-load-dupe | ➕insert-patch-load | ⛔dupe  |
      | insert-ring-stiffener-dupe | ➕insert-ring-stiffener | ⛔dupe  |
      | insert-sheet-profile-dupe | ➕insert-sheet-profile | ⛔dupe  |
      | insert-fire-insulation-dupe | ➕insert-fire-insulation | ⛔dupe  |
      | insert-material-dupe | ➕insert-material | ⛔dupe  |
      | insert-section-dupe | ➕insert-section | ⛔dupe  |
      | insert-connection-dupe | ➕insert-connection | ⛔dupe  |
      | insert-fire-scenario-dupe | ➕insert-fire-scenario | ⛔dupe  |
      | insert-fatigue-detail-dupe | ➕insert-fatigue-detail | ⛔dupe  |
      | insert-cold-formed-sheet-dupe | ➕insert-cold-formed-sheet | ⛔dupe  |
      | insert-shell-dupe | ➕insert-shell | ⛔dupe  |
      | insert-weld-run-dupe | ➕insert-weld-run | ⛔dupe  |
      | insert-outline-vertex-dupe | ➕insert-outline-vertex | ⛔dupe  |
      | insert-placement-dupe | ➕insert-placement | ⛔dupe  |
      | remove-member                 | ➖remove-member                  | ✅apply  |
      | change-member-n-ed            | 🏋️change-member-n-ed            | ✅apply  |
      | change-member-my-ed           | ⤴️change-member-my-ed           | ✅apply  |
      | change-member-buckling-length | 📏️change-member-buckling-length | ✅apply  |
      | change-material-designation   | ⚗️change-material-designation   | ✅apply  |
      | change-plate-thickness        | 🧱change-plate-thickness         | ✅apply  |
      | change-weld-throat            | 🔥️change-weld-throat            | ✅apply  |
      | change-bolt-count             | 🔩change-bolt-count              | ✅apply  |
      | insert-transverse-stiffener | ➕insert-transverse-stiffener | ✅apply  |
      | remove-transverse-stiffener | ➖remove-transverse-stiffener | ✅apply  |
      | change-transverse-girder-id | 🦴️change-transverse-girder-id | ✅apply  |
      | change-sheet-profile-sheet-id | 🦴️change-sheet-profile-sheet-id | ✅apply  |
      | change-fire-insulation-fire-id | 🦴️change-fire-insulation-fire-id | ✅apply  |
      | change-transverse-position | 🦴️change-transverse-position | ✅apply  |
      | change-transverse-width | 🦴️change-transverse-width | ✅apply  |
      | change-transverse-thickness | 🦴️change-transverse-thickness | ✅apply  |
      | change-conn-load-eccentricity | 🦴️change-conn-load-eccentricity | ✅apply  |
      | change-conn-hole-stagger | 🦴️change-conn-hole-stagger | ✅apply  |
      | change-sheet-profile-web-height | 🦴️change-sheet-profile-web-height | ✅apply  |
      | change-sheet-profile-bottom-flange | 🦴️change-sheet-profile-bottom-flange | ✅apply  |
      | change-sheet-profile-top-flange | 🦴️change-sheet-profile-top-flange | ✅apply  |
      | change-sheet-profile-pitch | 🦴️change-sheet-profile-pitch | ✅apply  |
      | change-fire-insulation-density | 🦴️change-fire-insulation-density | ✅apply  |
      | change-fire-insulation-specific-heat | 🦴️change-fire-insulation-specific-heat | ✅apply  |
      | change-fire-insulation-conductivity | 🦴️change-fire-insulation-conductivity | ✅apply  |
      | change-fire-insulation-thickness | 🦴️change-fire-insulation-thickness | ✅apply  |
      | change-transverse-sides | 🦴️change-transverse-sides | ✅apply  |
      | insert-member-action | ➕insert-member-action | ✅apply  |
      | remove-member-action | ➖remove-member-action | ✅apply  |
      | insert-connection-action | ➕insert-connection-action | ✅apply  |
      | remove-connection-action | ➖remove-connection-action | ✅apply  |
      | insert-section-element | ➕insert-section-element | ✅apply  |
      | remove-section-element | ➖remove-section-element | ✅apply  |
      | insert-sheet-action | ➕insert-sheet-action | ✅apply  |
      | remove-sheet-action | ➖remove-sheet-action | ✅apply  |
      | insert-shell-action | ➕insert-shell-action | ✅apply  |
      | remove-shell-action | ➖remove-shell-action | ✅apply  |
      | insert-fatigue-category | ➕insert-fatigue-category | ✅apply  |
      | remove-fatigue-category | ➖remove-fatigue-category | ✅apply  |
      | insert-patch-load | ➕insert-patch-load | ✅apply  |
      | remove-patch-load | ➖remove-patch-load | ✅apply  |
      | insert-ring-stiffener | ➕insert-ring-stiffener | ✅apply  |
      | remove-ring-stiffener | ➖remove-ring-stiffener | ✅apply  |
      | insert-sheet-profile | ➕insert-sheet-profile | ✅apply  |
      | remove-sheet-profile | ➖remove-sheet-profile | ✅apply  |
      | insert-fire-insulation | ➕insert-fire-insulation | ✅apply  |
      | remove-fire-insulation | ➖remove-fire-insulation | ✅apply  |
      | change-category-member-id | 🎯️change-category-member-id | ✅apply  |
      | change-category-element-id | 🎯️change-category-element-id | ✅apply  |
      | change-category-detail-category | 🎯️change-category-detail-category | ✅apply  |
      | change-category-detail-type | 🎯️change-category-detail-type | ✅apply  |
      | change-category-location-x | 🎯️change-category-location-x | ✅apply  |
      | change-category-m1 | 🎯️change-category-m1 | ✅apply  |
      | change-category-m2 | 🎯️change-category-m2 | ✅apply  |
      | change-patch-subject-id | ⬇️change-patch-subject-id | ✅apply  |
      | change-patch-position | ⬇️change-patch-position | ✅apply  |
      | change-patch-x | ⬇️change-patch-x | ✅apply  |
      | change-patch-bearing-length | ⬇️change-patch-bearing-length | ✅apply  |
      | change-patch-web-height | ⬇️change-patch-web-height | ✅apply  |
      | change-patch-web-angle | ⬇️change-patch-web-angle | ✅apply  |
      | change-patch-corner-radius | ⬇️change-patch-corner-radius | ✅apply  |
      | change-patch-load | ⬇️change-patch-load | ✅apply  |
      | change-ring-shell-id | ⭕️change-ring-shell-id | ✅apply  |
      | change-ring-x | ⭕️change-ring-x | ✅apply  |
      | change-ring-depth | ⭕️change-ring-depth | ✅apply  |
      | change-ring-thickness | ⭕️change-ring-thickness | ✅apply  |
      | change-ring-internal | ⭕️change-ring-internal | ✅apply  |
      | change-member-buckling-length-y | 🏗️change-member-buckling-length-y | ✅apply  |
      | change-member-buckling-length-t | 🏗️change-member-buckling-length-t | ✅apply  |
      | change-conn-bolts-bolts-per-row | 🔗️change-conn-bolts-bolts-per-row | ✅apply  |
      | insert-material | ➕insert-material | ✅apply  |
      | remove-material | ➖remove-material | ✅apply  |
      | insert-section | ➕insert-section | ✅apply  |
      | remove-section | ➖remove-section | ✅apply  |
      | insert-connection | ➕insert-connection | ✅apply  |
      | remove-connection | ➖remove-connection | ✅apply  |
      | insert-fire-scenario | ➕insert-fire-scenario | ✅apply  |
      | remove-fire-scenario | ➖remove-fire-scenario | ✅apply  |
      | insert-fatigue-detail | ➕insert-fatigue-detail | ✅apply  |
      | remove-fatigue-detail | ➖remove-fatigue-detail | ✅apply  |
      | insert-cold-formed-sheet | ➕insert-cold-formed-sheet | ✅apply  |
      | remove-cold-formed-sheet | ➖remove-cold-formed-sheet | ✅apply  |
      | insert-shell | ➕insert-shell | ✅apply  |
      | remove-shell | ➖remove-shell | ✅apply  |
      | change-weld-run-member-id | 🧵️change-weld-run-member-id | ✅apply  |
      | change-weld-run-element-id | 🧵️change-weld-run-element-id | ✅apply  |
      | change-weld-run-start | 🧵️change-weld-run-start | ✅apply  |
      | change-weld-run-length | 🧵️change-weld-run-length | ✅apply  |
      | change-weld-run-offset | 🧵️change-weld-run-offset | ✅apply  |
      | change-weld-run-throat | 🧵️change-weld-run-throat | ✅apply  |
      | change-weld-run-process | 🧵️change-weld-run-process | ✅apply  |
      | change-weld-run-t-junction | 🧵️change-weld-run-t-junction | ✅apply  |
      | change-weld-run-direction | 🧵️change-weld-run-direction | ✅apply  |
      | change-weld-run-haz-extent | 🧵️change-weld-run-haz-extent | ✅apply  |
      | change-vertex-section-id | 🔷️change-vertex-section-id | ✅apply  |
      | change-vertex-ring | 🔷️change-vertex-ring | ✅apply  |
      | change-vertex-element-id | 🔷️change-vertex-element-id | ✅apply  |
      | change-vertex-y | 🔷️change-vertex-y | ✅apply  |
      | change-vertex-z | 🔷️change-vertex-z | ✅apply  |
      | change-placement-x | 📍️change-placement-x | ✅apply  |
      | change-placement-y | 📍️change-placement-y | ✅apply  |
      | change-placement-z | 📍️change-placement-z | ✅apply  |
      | change-placement-yaw | 📍️change-placement-yaw | ✅apply  |
      | insert-weld-run | ➕insert-weld-run | ✅apply  |
      | remove-weld-run | ➖remove-weld-run | ✅apply  |
      | insert-outline-vertex | ➕insert-outline-vertex | ✅apply  |
      | remove-outline-vertex | ➖remove-outline-vertex | ✅apply  |
      | insert-placement | ➕insert-placement | ✅apply  |
      | remove-placement | ➖remove-placement | ✅apply  |
      | change-material-product-form | ⚗️change-material-product-form | ✅apply  |
      | change-section-kind | 📐️change-section-kind | ✅apply  |
      | change-section-height | 📐️change-section-height | ✅apply  |
      | change-section-width | 📐️change-section-width | ✅apply  |
      | change-section-flange-thickness | 📐️change-section-flange-thickness | ✅apply  |
      | change-section-web-thickness | 📐️change-section-web-thickness | ✅apply  |
      | change-section-outer-diameter | 📐️change-section-outer-diameter | ✅apply  |
      | change-element-width | 🧱️change-element-width | ✅apply  |
      | change-element-outstand | 🧱️change-element-outstand | ✅apply  |
      | change-element-welded | 🧱️change-element-welded | ✅apply  |
      | change-element-weld-position | 🧱️change-element-weld-position | ✅apply  |
      | change-element-weld-process | 🧱️change-element-weld-process | ✅apply  |
      | change-element-weld-t-junction | 🧱️change-element-weld-t-junction | ✅apply  |
      | change-element-weld-direction | 🧱️change-element-weld-direction | ✅apply  |
      | change-element-stiffener-depth | 🧱️change-element-stiffener-depth | ✅apply  |
      | change-element-stiffener-thk | 🧱️change-element-stiffener-thk | ✅apply  |
      | change-member-section-id | 🏗️change-member-section-id | ✅apply  |
      | change-member-material-id | 🏗️change-member-material-id | ✅apply  |
      | change-member-length | 🏗️change-member-length | ✅apply  |
      | change-member-support | 🏗️change-member-support | ✅apply  |
      | change-member-ltb-length | 🏗️change-member-ltb-length | ✅apply  |
      | change-member-c1 | 🏗️change-member-c1 | ✅apply  |
      | change-member-restrained-ltb | 🏗️change-member-restrained-ltb | ✅apply  |
      | change-member-bearing-length | 🏗️change-member-bearing-length | ✅apply  |
      | change-member-transverse-weld | 🏗️change-member-transverse-weld | ✅apply  |
      | change-member-t-weld-offset | 🏗️change-member-t-weld-offset | ✅apply  |
      | change-action-kind | 🏋️change-action-kind | ✅apply  |
      | change-action-category | 🏋️change-action-category | ✅apply  |
      | change-action-source | 🏋️change-action-source | ✅apply  |
      | change-action-gk-line | 🏋️change-action-gk-line | ✅apply  |
      | change-action-qk-line | 🏋️change-action-qk-line | ✅apply  |
      | change-action-vyk | 🏋️change-action-vyk | ✅apply  |
      | change-action-vzk | 🏋️change-action-vzk | ✅apply  |
      | change-action-mzk | 🏋️change-action-mzk | ✅apply  |
      | change-action-torsion-k | 🏋️change-action-torsion-k | ✅apply  |
      | change-action-local-load-k | 🏋️change-action-local-load-k | ✅apply  |
      | change-conn-member-id | 🔗️change-conn-member-id | ✅apply  |
      | change-conn-material-id | 🔗️change-conn-material-id | ✅apply  |
      | change-conn-kind | 🔗️change-conn-kind | ✅apply  |
      | change-conn-force-method | 🔗️change-conn-force-method | ✅apply  |
      | change-conn-bolts-material | 🔗️change-conn-bolts-material | ✅apply  |
      | change-conn-bolts-diameter | 🔗️change-conn-bolts-diameter | ✅apply  |
      | change-conn-bolts-edge-distance | 🔗️change-conn-bolts-edge-distance | ✅apply  |
      | change-conn-bolts-pitch | 🔗️change-conn-bolts-pitch | ✅apply  |
      | change-conn-bolts-gauge | 🔗️change-conn-bolts-gauge | ✅apply  |
      | change-conn-bolts-plate-thickness | 🔗️change-conn-bolts-plate-thickness | ✅apply  |
      | change-conn-welds-filler-alloy | 🔗️change-conn-welds-filler-alloy | ✅apply  |
      | change-conn-welds-length | 🔗️change-conn-welds-length | ✅apply  |
      | change-conn-welds-beta-w | 🔗️change-conn-welds-beta-w | ✅apply  |
      | change-conn-welds-haz-extent | 🔗️change-conn-welds-haz-extent | ✅apply  |
      | change-conn-action-kind | 🔗️change-conn-action-kind | ✅apply  |
      | change-conn-action-category | 🔗️change-conn-action-category | ✅apply  |
      | change-conn-action-source | 🔗️change-conn-action-source | ✅apply  |
      | change-conn-action-gk-line | 🔗️change-conn-action-gk-line | ✅apply  |
      | change-conn-action-qk-line | 🔗️change-conn-action-qk-line | ✅apply  |
      | change-conn-action-nk | 🔗️change-conn-action-nk | ✅apply  |
      | change-conn-action-vyk | 🔗️change-conn-action-vyk | ✅apply  |
      | change-conn-action-vzk | 🔗️change-conn-action-vzk | ✅apply  |
      | change-conn-action-myk | 🔗️change-conn-action-myk | ✅apply  |
      | change-conn-action-mzk | 🔗️change-conn-action-mzk | ✅apply  |
      | change-conn-action-torsion-k | 🔗️change-conn-action-torsion-k | ✅apply  |
      | change-conn-action-local-load-k | 🔗️change-conn-action-local-load-k | ✅apply  |
      | change-fire-member-id | 🔥️change-fire-member-id | ✅apply  |
      | change-fire-theta-a | 🔥️change-fire-theta-a | ✅apply  |
      | change-fire-duration-s | 🔥️change-fire-duration-s | ✅apply  |
      | change-fatigue-member-id | 🔄️change-fatigue-member-id | ✅apply  |
      | change-fatigue-detail-category | 🔄️change-fatigue-detail-category | ✅apply  |
      | change-fatigue-delta-sigma-c | 🔄️change-fatigue-delta-sigma-c | ✅apply  |
      | change-fatigue-delta-sigma-ed | 🔄️change-fatigue-delta-sigma-ed | ✅apply  |
      | change-fatigue-n-cycles | 🔄️change-fatigue-n-cycles | ✅apply  |
      | change-fatigue-m1 | 🔄️change-fatigue-m1 | ✅apply  |
      | change-fatigue-m2 | 🔄️change-fatigue-m2 | ✅apply  |
      | change-sheet-material-id | ❄️change-sheet-material-id | ✅apply  |
      | change-sheet-thickness | ❄️change-sheet-thickness | ✅apply  |
      | change-sheet-width | ❄️change-sheet-width | ✅apply  |
      | change-sheet-span | ❄️change-sheet-span | ✅apply  |
      | change-sheet-welded | ❄️change-sheet-welded | ✅apply  |
      | change-sheet-action-kind | ❄️change-sheet-action-kind | ✅apply  |
      | change-sheet-action-category | ❄️change-sheet-action-category | ✅apply  |
      | change-sheet-action-source | ❄️change-sheet-action-source | ✅apply  |
      | change-sheet-action-gk-line | ❄️change-sheet-action-gk-line | ✅apply  |
      | change-sheet-action-qk-line | ❄️change-sheet-action-qk-line | ✅apply  |
      | change-sheet-action-nk | ❄️change-sheet-action-nk | ✅apply  |
      | change-sheet-action-vyk | ❄️change-sheet-action-vyk | ✅apply  |
      | change-sheet-action-vzk | ❄️change-sheet-action-vzk | ✅apply  |
      | change-sheet-action-myk | ❄️change-sheet-action-myk | ✅apply  |
      | change-sheet-action-mzk | ❄️change-sheet-action-mzk | ✅apply  |
      | change-sheet-action-torsion-k | ❄️change-sheet-action-torsion-k | ✅apply  |
      | change-sheet-action-local-load-k | ❄️change-sheet-action-local-load-k | ✅apply  |
      | change-shell-material-id | 🫙️change-shell-material-id | ✅apply  |
      | change-shell-radius | 🫙️change-shell-radius | ✅apply  |
      | change-shell-thickness | 🫙️change-shell-thickness | ✅apply  |
      | change-shell-length | 🫙️change-shell-length | ✅apply  |
      | change-shell-welded | 🫙️change-shell-welded | ✅apply  |
      | change-shell-fabrication-class | 🫙️change-shell-fabrication-class | ✅apply  |
      | change-shell-action-kind | 🫙️change-shell-action-kind | ✅apply  |
      | change-shell-action-category | 🫙️change-shell-action-category | ✅apply  |
      | change-shell-action-source | 🫙️change-shell-action-source | ✅apply  |
      | change-shell-action-gk-line | 🫙️change-shell-action-gk-line | ✅apply  |
      | change-shell-action-qk-line | 🫙️change-shell-action-qk-line | ✅apply  |
      | change-shell-action-nk | 🫙️change-shell-action-nk | ✅apply  |
      | change-shell-action-vyk | 🫙️change-shell-action-vyk | ✅apply  |
      | change-shell-action-vzk | 🫙️change-shell-action-vzk | ✅apply  |
      | change-shell-action-myk | 🫙️change-shell-action-myk | ✅apply  |
      | change-shell-action-mzk | 🫙️change-shell-action-mzk | ✅apply  |
      | change-shell-action-torsion-k | 🫙️change-shell-action-torsion-k | ✅apply  |
      | change-shell-action-local-load-k | 🫙️change-shell-action-local-load-k | ✅apply  |
      | remove-member-middle          | ➖remove-member                  | 🧭️middle |

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
      | insert-member                    | ➕insert-member                     | ✅apply  |
      | remove-member                 | ➖remove-member                  | ✅apply  |
      | change-member-n-ed            | 🏋️change-member-n-ed            | ✅apply  |
      | change-member-my-ed           | ⤴️change-member-my-ed           | ✅apply  |
      | change-member-buckling-length | 📏️change-member-buckling-length | ✅apply  |
      | change-material-designation   | ⚗️change-material-designation   | ✅apply  |
      | change-plate-thickness        | 🧱change-plate-thickness         | ✅apply  |
      | change-weld-throat            | 🔥️change-weld-throat            | ✅apply  |
      | change-bolt-count             | 🔩change-bolt-count              | ✅apply  |
      | insert-transverse-stiffener | ➕insert-transverse-stiffener | ✅apply  |
      | remove-transverse-stiffener | ➖remove-transverse-stiffener | ✅apply  |
      | change-transverse-girder-id | 🦴️change-transverse-girder-id | ✅apply  |
      | change-sheet-profile-sheet-id | 🦴️change-sheet-profile-sheet-id | ✅apply  |
      | change-fire-insulation-fire-id | 🦴️change-fire-insulation-fire-id | ✅apply  |
      | change-transverse-position | 🦴️change-transverse-position | ✅apply  |
      | change-transverse-width | 🦴️change-transverse-width | ✅apply  |
      | change-transverse-thickness | 🦴️change-transverse-thickness | ✅apply  |
      | change-conn-load-eccentricity | 🦴️change-conn-load-eccentricity | ✅apply  |
      | change-conn-hole-stagger | 🦴️change-conn-hole-stagger | ✅apply  |
      | change-sheet-profile-web-height | 🦴️change-sheet-profile-web-height | ✅apply  |
      | change-sheet-profile-bottom-flange | 🦴️change-sheet-profile-bottom-flange | ✅apply  |
      | change-sheet-profile-top-flange | 🦴️change-sheet-profile-top-flange | ✅apply  |
      | change-sheet-profile-pitch | 🦴️change-sheet-profile-pitch | ✅apply  |
      | change-fire-insulation-density | 🦴️change-fire-insulation-density | ✅apply  |
      | change-fire-insulation-specific-heat | 🦴️change-fire-insulation-specific-heat | ✅apply  |
      | change-fire-insulation-conductivity | 🦴️change-fire-insulation-conductivity | ✅apply  |
      | change-fire-insulation-thickness | 🦴️change-fire-insulation-thickness | ✅apply  |
      | change-transverse-sides | 🦴️change-transverse-sides | ✅apply  |
      | insert-member-action | ➕insert-member-action | ✅apply  |
      | remove-member-action | ➖remove-member-action | ✅apply  |
      | insert-connection-action | ➕insert-connection-action | ✅apply  |
      | remove-connection-action | ➖remove-connection-action | ✅apply  |
      | insert-section-element | ➕insert-section-element | ✅apply  |
      | remove-section-element | ➖remove-section-element | ✅apply  |
      | insert-sheet-action | ➕insert-sheet-action | ✅apply  |
      | remove-sheet-action | ➖remove-sheet-action | ✅apply  |
      | insert-shell-action | ➕insert-shell-action | ✅apply  |
      | remove-shell-action | ➖remove-shell-action | ✅apply  |
      | insert-fatigue-category | ➕insert-fatigue-category | ✅apply  |
      | remove-fatigue-category | ➖remove-fatigue-category | ✅apply  |
      | insert-patch-load | ➕insert-patch-load | ✅apply  |
      | remove-patch-load | ➖remove-patch-load | ✅apply  |
      | insert-ring-stiffener | ➕insert-ring-stiffener | ✅apply  |
      | remove-ring-stiffener | ➖remove-ring-stiffener | ✅apply  |
      | insert-sheet-profile | ➕insert-sheet-profile | ✅apply  |
      | remove-sheet-profile | ➖remove-sheet-profile | ✅apply  |
      | insert-fire-insulation | ➕insert-fire-insulation | ✅apply  |
      | remove-fire-insulation | ➖remove-fire-insulation | ✅apply  |
      | change-category-member-id | 🎯️change-category-member-id | ✅apply  |
      | change-category-element-id | 🎯️change-category-element-id | ✅apply  |
      | change-category-detail-category | 🎯️change-category-detail-category | ✅apply  |
      | change-category-detail-type | 🎯️change-category-detail-type | ✅apply  |
      | change-category-location-x | 🎯️change-category-location-x | ✅apply  |
      | change-category-m1 | 🎯️change-category-m1 | ✅apply  |
      | change-category-m2 | 🎯️change-category-m2 | ✅apply  |
      | change-patch-subject-id | ⬇️change-patch-subject-id | ✅apply  |
      | change-patch-position | ⬇️change-patch-position | ✅apply  |
      | change-patch-x | ⬇️change-patch-x | ✅apply  |
      | change-patch-bearing-length | ⬇️change-patch-bearing-length | ✅apply  |
      | change-patch-web-height | ⬇️change-patch-web-height | ✅apply  |
      | change-patch-web-angle | ⬇️change-patch-web-angle | ✅apply  |
      | change-patch-corner-radius | ⬇️change-patch-corner-radius | ✅apply  |
      | change-patch-load | ⬇️change-patch-load | ✅apply  |
      | change-ring-shell-id | ⭕️change-ring-shell-id | ✅apply  |
      | change-ring-x | ⭕️change-ring-x | ✅apply  |
      | change-ring-depth | ⭕️change-ring-depth | ✅apply  |
      | change-ring-thickness | ⭕️change-ring-thickness | ✅apply  |
      | change-ring-internal | ⭕️change-ring-internal | ✅apply  |
      | change-member-buckling-length-y | 🏗️change-member-buckling-length-y | ✅apply  |
      | change-member-buckling-length-t | 🏗️change-member-buckling-length-t | ✅apply  |
      | change-conn-bolts-bolts-per-row | 🔗️change-conn-bolts-bolts-per-row | ✅apply  |
      | insert-material | ➕insert-material | ✅apply  |
      | remove-material | ➖remove-material | ✅apply  |
      | insert-section | ➕insert-section | ✅apply  |
      | remove-section | ➖remove-section | ✅apply  |
      | insert-connection | ➕insert-connection | ✅apply  |
      | remove-connection | ➖remove-connection | ✅apply  |
      | insert-fire-scenario | ➕insert-fire-scenario | ✅apply  |
      | remove-fire-scenario | ➖remove-fire-scenario | ✅apply  |
      | insert-fatigue-detail | ➕insert-fatigue-detail | ✅apply  |
      | remove-fatigue-detail | ➖remove-fatigue-detail | ✅apply  |
      | insert-cold-formed-sheet | ➕insert-cold-formed-sheet | ✅apply  |
      | remove-cold-formed-sheet | ➖remove-cold-formed-sheet | ✅apply  |
      | insert-shell | ➕insert-shell | ✅apply  |
      | remove-shell | ➖remove-shell | ✅apply  |
      | change-weld-run-member-id | 🧵️change-weld-run-member-id | ✅apply  |
      | change-weld-run-element-id | 🧵️change-weld-run-element-id | ✅apply  |
      | change-weld-run-start | 🧵️change-weld-run-start | ✅apply  |
      | change-weld-run-length | 🧵️change-weld-run-length | ✅apply  |
      | change-weld-run-offset | 🧵️change-weld-run-offset | ✅apply  |
      | change-weld-run-throat | 🧵️change-weld-run-throat | ✅apply  |
      | change-weld-run-process | 🧵️change-weld-run-process | ✅apply  |
      | change-weld-run-t-junction | 🧵️change-weld-run-t-junction | ✅apply  |
      | change-weld-run-direction | 🧵️change-weld-run-direction | ✅apply  |
      | change-weld-run-haz-extent | 🧵️change-weld-run-haz-extent | ✅apply  |
      | change-vertex-section-id | 🔷️change-vertex-section-id | ✅apply  |
      | change-vertex-ring | 🔷️change-vertex-ring | ✅apply  |
      | change-vertex-element-id | 🔷️change-vertex-element-id | ✅apply  |
      | change-vertex-y | 🔷️change-vertex-y | ✅apply  |
      | change-vertex-z | 🔷️change-vertex-z | ✅apply  |
      | change-placement-x | 📍️change-placement-x | ✅apply  |
      | change-placement-y | 📍️change-placement-y | ✅apply  |
      | change-placement-z | 📍️change-placement-z | ✅apply  |
      | change-placement-yaw | 📍️change-placement-yaw | ✅apply  |
      | insert-weld-run | ➕insert-weld-run | ✅apply  |
      | remove-weld-run | ➖remove-weld-run | ✅apply  |
      | insert-outline-vertex | ➕insert-outline-vertex | ✅apply  |
      | remove-outline-vertex | ➖remove-outline-vertex | ✅apply  |
      | insert-placement | ➕insert-placement | ✅apply  |
      | remove-placement | ➖remove-placement | ✅apply  |
      | change-material-product-form | ⚗️change-material-product-form | ✅apply  |
      | change-section-kind | 📐️change-section-kind | ✅apply  |
      | change-section-height | 📐️change-section-height | ✅apply  |
      | change-section-width | 📐️change-section-width | ✅apply  |
      | change-section-flange-thickness | 📐️change-section-flange-thickness | ✅apply  |
      | change-section-web-thickness | 📐️change-section-web-thickness | ✅apply  |
      | change-section-outer-diameter | 📐️change-section-outer-diameter | ✅apply  |
      | change-element-width | 🧱️change-element-width | ✅apply  |
      | change-element-outstand | 🧱️change-element-outstand | ✅apply  |
      | change-element-welded | 🧱️change-element-welded | ✅apply  |
      | change-element-weld-position | 🧱️change-element-weld-position | ✅apply  |
      | change-element-weld-process | 🧱️change-element-weld-process | ✅apply  |
      | change-element-weld-t-junction | 🧱️change-element-weld-t-junction | ✅apply  |
      | change-element-weld-direction | 🧱️change-element-weld-direction | ✅apply  |
      | change-element-stiffener-depth | 🧱️change-element-stiffener-depth | ✅apply  |
      | change-element-stiffener-thk | 🧱️change-element-stiffener-thk | ✅apply  |
      | change-member-section-id | 🏗️change-member-section-id | ✅apply  |
      | change-member-material-id | 🏗️change-member-material-id | ✅apply  |
      | change-member-length | 🏗️change-member-length | ✅apply  |
      | change-member-support | 🏗️change-member-support | ✅apply  |
      | change-member-ltb-length | 🏗️change-member-ltb-length | ✅apply  |
      | change-member-c1 | 🏗️change-member-c1 | ✅apply  |
      | change-member-restrained-ltb | 🏗️change-member-restrained-ltb | ✅apply  |
      | change-member-bearing-length | 🏗️change-member-bearing-length | ✅apply  |
      | change-member-transverse-weld | 🏗️change-member-transverse-weld | ✅apply  |
      | change-member-t-weld-offset | 🏗️change-member-t-weld-offset | ✅apply  |
      | change-action-kind | 🏋️change-action-kind | ✅apply  |
      | change-action-category | 🏋️change-action-category | ✅apply  |
      | change-action-source | 🏋️change-action-source | ✅apply  |
      | change-action-gk-line | 🏋️change-action-gk-line | ✅apply  |
      | change-action-qk-line | 🏋️change-action-qk-line | ✅apply  |
      | change-action-vyk | 🏋️change-action-vyk | ✅apply  |
      | change-action-vzk | 🏋️change-action-vzk | ✅apply  |
      | change-action-mzk | 🏋️change-action-mzk | ✅apply  |
      | change-action-torsion-k | 🏋️change-action-torsion-k | ✅apply  |
      | change-action-local-load-k | 🏋️change-action-local-load-k | ✅apply  |
      | change-conn-member-id | 🔗️change-conn-member-id | ✅apply  |
      | change-conn-material-id | 🔗️change-conn-material-id | ✅apply  |
      | change-conn-kind | 🔗️change-conn-kind | ✅apply  |
      | change-conn-force-method | 🔗️change-conn-force-method | ✅apply  |
      | change-conn-bolts-material | 🔗️change-conn-bolts-material | ✅apply  |
      | change-conn-bolts-diameter | 🔗️change-conn-bolts-diameter | ✅apply  |
      | change-conn-bolts-edge-distance | 🔗️change-conn-bolts-edge-distance | ✅apply  |
      | change-conn-bolts-pitch | 🔗️change-conn-bolts-pitch | ✅apply  |
      | change-conn-bolts-gauge | 🔗️change-conn-bolts-gauge | ✅apply  |
      | change-conn-bolts-plate-thickness | 🔗️change-conn-bolts-plate-thickness | ✅apply  |
      | change-conn-welds-filler-alloy | 🔗️change-conn-welds-filler-alloy | ✅apply  |
      | change-conn-welds-length | 🔗️change-conn-welds-length | ✅apply  |
      | change-conn-welds-beta-w | 🔗️change-conn-welds-beta-w | ✅apply  |
      | change-conn-welds-haz-extent | 🔗️change-conn-welds-haz-extent | ✅apply  |
      | change-conn-action-kind | 🔗️change-conn-action-kind | ✅apply  |
      | change-conn-action-category | 🔗️change-conn-action-category | ✅apply  |
      | change-conn-action-source | 🔗️change-conn-action-source | ✅apply  |
      | change-conn-action-gk-line | 🔗️change-conn-action-gk-line | ✅apply  |
      | change-conn-action-qk-line | 🔗️change-conn-action-qk-line | ✅apply  |
      | change-conn-action-nk | 🔗️change-conn-action-nk | ✅apply  |
      | change-conn-action-vyk | 🔗️change-conn-action-vyk | ✅apply  |
      | change-conn-action-vzk | 🔗️change-conn-action-vzk | ✅apply  |
      | change-conn-action-myk | 🔗️change-conn-action-myk | ✅apply  |
      | change-conn-action-mzk | 🔗️change-conn-action-mzk | ✅apply  |
      | change-conn-action-torsion-k | 🔗️change-conn-action-torsion-k | ✅apply  |
      | change-conn-action-local-load-k | 🔗️change-conn-action-local-load-k | ✅apply  |
      | change-fire-member-id | 🔥️change-fire-member-id | ✅apply  |
      | change-fire-theta-a | 🔥️change-fire-theta-a | ✅apply  |
      | change-fire-duration-s | 🔥️change-fire-duration-s | ✅apply  |
      | change-fatigue-member-id | 🔄️change-fatigue-member-id | ✅apply  |
      | change-fatigue-detail-category | 🔄️change-fatigue-detail-category | ✅apply  |
      | change-fatigue-delta-sigma-c | 🔄️change-fatigue-delta-sigma-c | ✅apply  |
      | change-fatigue-delta-sigma-ed | 🔄️change-fatigue-delta-sigma-ed | ✅apply  |
      | change-fatigue-n-cycles | 🔄️change-fatigue-n-cycles | ✅apply  |
      | change-fatigue-m1 | 🔄️change-fatigue-m1 | ✅apply  |
      | change-fatigue-m2 | 🔄️change-fatigue-m2 | ✅apply  |
      | change-sheet-material-id | ❄️change-sheet-material-id | ✅apply  |
      | change-sheet-thickness | ❄️change-sheet-thickness | ✅apply  |
      | change-sheet-width | ❄️change-sheet-width | ✅apply  |
      | change-sheet-span | ❄️change-sheet-span | ✅apply  |
      | change-sheet-welded | ❄️change-sheet-welded | ✅apply  |
      | change-sheet-action-kind | ❄️change-sheet-action-kind | ✅apply  |
      | change-sheet-action-category | ❄️change-sheet-action-category | ✅apply  |
      | change-sheet-action-source | ❄️change-sheet-action-source | ✅apply  |
      | change-sheet-action-gk-line | ❄️change-sheet-action-gk-line | ✅apply  |
      | change-sheet-action-qk-line | ❄️change-sheet-action-qk-line | ✅apply  |
      | change-sheet-action-nk | ❄️change-sheet-action-nk | ✅apply  |
      | change-sheet-action-vyk | ❄️change-sheet-action-vyk | ✅apply  |
      | change-sheet-action-vzk | ❄️change-sheet-action-vzk | ✅apply  |
      | change-sheet-action-myk | ❄️change-sheet-action-myk | ✅apply  |
      | change-sheet-action-mzk | ❄️change-sheet-action-mzk | ✅apply  |
      | change-sheet-action-torsion-k | ❄️change-sheet-action-torsion-k | ✅apply  |
      | change-sheet-action-local-load-k | ❄️change-sheet-action-local-load-k | ✅apply  |
      | change-shell-material-id | 🫙️change-shell-material-id | ✅apply  |
      | change-shell-radius | 🫙️change-shell-radius | ✅apply  |
      | change-shell-thickness | 🫙️change-shell-thickness | ✅apply  |
      | change-shell-length | 🫙️change-shell-length | ✅apply  |
      | change-shell-welded | 🫙️change-shell-welded | ✅apply  |
      | change-shell-fabrication-class | 🫙️change-shell-fabrication-class | ✅apply  |
      | change-shell-action-kind | 🫙️change-shell-action-kind | ✅apply  |
      | change-shell-action-category | 🫙️change-shell-action-category | ✅apply  |
      | change-shell-action-source | 🫙️change-shell-action-source | ✅apply  |
      | change-shell-action-gk-line | 🫙️change-shell-action-gk-line | ✅apply  |
      | change-shell-action-qk-line | 🫙️change-shell-action-qk-line | ✅apply  |
      | change-shell-action-nk | 🫙️change-shell-action-nk | ✅apply  |
      | change-shell-action-vyk | 🫙️change-shell-action-vyk | ✅apply  |
      | change-shell-action-vzk | 🫙️change-shell-action-vzk | ✅apply  |
      | change-shell-action-myk | 🫙️change-shell-action-myk | ✅apply  |
      | change-shell-action-mzk | 🫙️change-shell-action-mzk | ✅apply  |
      | change-shell-action-torsion-k | 🫙️change-shell-action-torsion-k | ✅apply  |
      | change-shell-action-local-load-k | 🫙️change-shell-action-local-load-k | ✅apply  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1999 document from the parsed carrier
    Given the real committed text artifact asset://🏠️aluminium-roof-purlin/🏠️aluminium-roof-purlin/🗣️.dsl.semio
    And its committed binary twin asset://🏠️aluminium-roof-purlin/🎒️.pack.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
