@capability-en1990-1-mutate
@oracle-en1990-1-python-independent
@comparison-ordered-json-v1
@mutations-en1990-1-any
Feature: Apply every typed EN 1990 mutation against an independent Python implementation
  `s.norm.en1990` is a semio-NATIVE artifact and no third party reads or writes it — checked, not
  assumed: PyPI serves no `en1990` distribution, and none for `eurocode`, `vdi3805` or `iso16757`
  either, and the nearest real packages (`structuralcodes`, `concreteproperties`, `anastruct`)
  implement design-code FORMULAE and speak no interchange format at all, so not one of them could be
  authoritative over this subset's `En1990Mutation` vocabulary. The second producer a differential
  comparison needs is therefore a second IMPLEMENTATION: the norm plugin's one independent Python
  engine, which `🐍️.py` beside this file imports and feeds with this subset's own committed
  catalog. The engine is written from the repository's own written specification of what a semantic
  mutation means — `📓️taxonomy.md`'s verb table, naming mechanics ("New-value fields are
  `new_<field>`") and addressing convention ("Inverse always computed from `base`", "Missing target
  ⇒ `inverse` returns `Vec::new()`"), and `📓️derivation-rules.md`'s shape rules. It imports nothing
  from the Rust it judges: the document field a `new*` argument names is resolved by normalised
  spelling against the document's own keys, never from a table copied out of `🧬️mutations/**`.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path
  below is a declared `shared://` fixture, one `✅apply` vector per kind, and each vector is that leaf's
  committed wire witness. Its `🦠️mutation` payload is written by hand against the leaf schema; its
  before-snapshot is the committed `🏢️accidental-seismic-compliant` example — the one EN 1990 example
  that carries a member in every action collection, the accidental and the seismic one included — and
  its after-snapshot, diff and outcome are production dispatch's answer, held by the crate's own
  vector law and judged here by the second implementation. All thirty kinds APPLY and move the
  document. Each `insert-<kind>-clamp` row asks an insert for a position past its list's end: both
  sides must insert last, which is where the canonical append landed, and production reports it as a
  `mutation.clamped` warning. Each `insert-<kind>-dupe` row re-applies an insert's canonical mutation to
  its own after-snapshot, whose id is already held, so both sides must refuse with
  `mutation.duplicate-id` and leave the document bit-identical; it has no inverse row, because nothing
  moved. Member effects carry no id — one member may take the same action along several load paths —
  so `insert-effect` has a clamp row and no duplicate.

  The vocabulary has three shapes. Eleven root scalars (annex, project id, site altitude, consequence
  and reliability class, design working life, reference period, supervision and inspection level,
  computed β) take `change-`; seven whole collections (permanent, variable, accidental and seismic
  actions, members, bridge serviceability records, member effects) take a whole-list `change-`; and
  six ordered collections take `insert-`/`remove-` by position. Each side then asserts the same three
  laws in role — the applied document must BE the committed after-snapshot; an `applied` vector must
  move the document and a `rejected` one must leave it untouched under its committed code; and the
  mutation followed by its OWN computed inverse must restore the before-snapshot exactly. The two sides may undo differently — production undoes an `insert-` with a
  whole-list `change-`, the reference with the paired `remove-` — and both must still land on the same
  before-snapshot. What `parity` adds on top is the only thing a single implementation can never
  provide: that two implementations, in two languages, written from one written specification, reach
  the same document.

  `inverse-` projects BOTH the mutated and the restored document, so every row projects a value only
  its own kind produces.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `🖼️assets/🏢️high-consequence-office/🏢️high-consequence-office/🗣️.dsl.semio` — a named CC3 office
  case, not a generic demo — whose binary twin the Rust side also decodes. The carrier has no published
  grammar: the committed `📖️.grammar.semio` is the repository-wide `payload = OCTET+` rule, so
  identity is compared at the envelope preamble, the ordered `key=value` fields and the digest and
  length of the re-emitted bytes, never at an inferred mapping from carrier tokens onto the JSON
  snapshot's enum spellings.

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
      | id                                  | dir                                  | fixture |
      | change-annex                        | 🌍️change-annex                       | ✅apply  |
      | change-traffic-group-group | 🚦️change-traffic-group-group | ✅apply  |
      | change-traffic-group-variable-id | 🚦️change-traffic-group-variable-id | ✅apply  |
      | insert-traffic-group | 🚦️insert-traffic-group | ✅apply  |
      | remove-traffic-group | 🚦️remove-traffic-group | ✅apply  |
      | change-member-material-strength-k | 🏗️change-member-material-strength-k | ✅apply  |
      | change-member-material-conv-factor | 🏗️change-member-material-conv-factor | ✅apply  |
      | change-member-mat-partial-factor | 🏗️change-member-mat-partial-factor | ✅apply  |
      | change-member-mat-design-strength | 🏗️change-member-mat-design-strength | ✅apply  |
      | change-member-nominal-dimension | 🏗️change-member-nominal-dimension | ✅apply  |
      | change-member-dimension-tolerance | 🏗️change-member-dimension-tolerance | ✅apply  |
      | change-member-design-dimension | 🏗️change-member-design-dimension | ✅apply  |
      | change-member-member-alpha-cr | 🏗️change-member-member-alpha-cr | ✅apply  |
      | change-member-second-order-analysis | 🏗️change-member-second-order-analysis | ✅apply  |
      | change-bridge-damping-ratio | 🌉️change-bridge-damping-ratio | ✅apply  |
      | change-factor-override-factor | 🎚️change-factor-override-factor | ✅apply  |
      | change-factor-override-target | 🎚️change-factor-override-target | ✅apply  |
      | change-factor-override-value | 🎚️change-factor-override-value | ✅apply  |
      | change-factor-override-justif | 🎚️change-factor-override-justif | ✅apply  |
      | insert-factor-override | 🎚️insert-factor-override | ✅apply  |
      | remove-factor-override | 🎚️remove-factor-override | ✅apply  |
      | change-member-frame-height | 🏗️change-member-frame-height | ✅apply  |
      | change-member-frame-drift | 🏗️change-member-frame-drift | ✅apply  |
      | change-member-storey-height | 🏗️change-member-storey-height | ✅apply  |
      | change-member-storey-drift | 🏗️change-member-storey-drift | ✅apply  |
      | change-bridge-track-gauge | 🌉️change-bridge-track-gauge | ✅apply  |
      | change-bridge-line-speed | 🌉️change-bridge-line-speed | ✅apply  |
      | change-bridge-ballast-depth | 🌉️change-bridge-ballast-depth | ✅apply  |
      | change-bridge-deck-mass | 🌉️change-bridge-deck-mass | ✅apply  |
      | change-bridge-natural-frequency | 🌉️change-bridge-natural-frequency | ✅apply  |
      | change-bridge-end-rotation | 🌉️change-bridge-end-rotation | ✅apply  |
      | change-bridge-basic-wind-speed | 🌉️change-bridge-basic-wind-speed | ✅apply  |
      | change-permanent-kind | ⚓️change-permanent-kind | ✅apply  |
      | change-permanent-gk | ⚓️change-permanent-gk | ✅apply  |
      | change-variable-category | 🏋️change-variable-category | ✅apply  |
      | change-variable-qk | 🏋️change-variable-qk | ✅apply  |
      | change-variable-group | 🏋️change-variable-group | ✅apply  |
      | change-accidental-ad | 💥️change-accidental-ad | ✅apply  |
      | insert-case | ➕️insert-case | ✅apply  |
      | remove-case | ➖️remove-case | ✅apply  |
      | change-case-number | 🔢️change-case-number | ✅apply  |
      | change-case-text | 🔤️change-case-text | ✅apply  |
      | change-case-label-en | 🏷️change-case-label-en | ✅apply  |
      | change-case-label-de | 🏷️change-case-label-de | ✅apply  |
      | change-case-entry-id | 📇️change-case-entry-id | ✅apply  |
      | change-seismic-a-ek | 🌋️change-seismic-a-ek | ✅apply  |
      | change-seismic-importance-class | 🌋️change-seismic-importance-class | ✅apply  |
      | change-member-label-en | 🏗️change-member-label-en | ✅apply  |
      | change-member-label-de | 🏗️change-member-label-de | ✅apply  |
      | change-member-rd-str | 🏗️change-member-rd-str | ✅apply  |
      | change-member-rd-geo | 🏗️change-member-rd-geo | ✅apply  |
      | change-member-rd-equ-stab | 🏗️change-member-rd-equ-stab | ✅apply  |
      | change-member-rd-equ-destab | 🏗️change-member-rd-equ-destab | ✅apply  |
      | change-member-rd-fat | 🏗️change-member-rd-fat | ✅apply  |
      | change-member-span | 🏗️change-member-span | ✅apply  |
      | change-member-precamber | 🏗️change-member-precamber | ✅apply  |
      | change-member-deflection-w1 | 🏗️change-member-deflection-w1 | ✅apply  |
      | change-member-deflection-w2 | 🏗️change-member-deflection-w2 | ✅apply  |
      | change-member-deflection-w3 | 🏗️change-member-deflection-w3 | ✅apply  |
      | change-member-defl-limit-ratio | 🏗️change-member-defl-limit-ratio | ✅apply  |
      | change-member-defl-limit-ratio-w3 | 🏗️change-member-defl-limit-ratio-w3 | ✅apply  |
      | change-member-sls-limit-char | 🏗️change-member-sls-limit-char | ✅apply  |
      | change-member-sls-limit-freq | 🏗️change-member-sls-limit-freq | ✅apply  |
      | change-member-sls-limit-qp | 🏗️change-member-sls-limit-qp | ✅apply  |
      | change-member-vibration-frequency | 🏗️change-member-vibration-frequency | ✅apply  |
      | change-member-vib-freq-min | 🏗️change-member-vib-freq-min | ✅apply  |
      | change-bridge-member-id | 🌉️change-bridge-member-id | ✅apply  |
      | change-bridge-deck-acceleration | 🌉️change-bridge-deck-acceleration | ✅apply  |
      | change-bridge-deck-accel-limit | 🌉️change-bridge-deck-accel-limit | ✅apply  |
      | change-bridge-deck-twist | 🌉️change-bridge-deck-twist | ✅apply  |
      | change-bridge-deck-twist-limit | 🌉️change-bridge-deck-twist-limit | ✅apply  |
      | change-bridge-deflection | 🌉️change-bridge-deflection | ✅apply  |
      | change-bridge-deflection-limit | 🌉️change-bridge-deflection-limit | ✅apply  |
      | change-effect-member-id | 🔗️change-effect-member-id | ✅apply  |
      | change-effect-action-id | 🔗️change-effect-action-id | ✅apply  |
      | change-effect-influence | 🔗️change-effect-influence | ✅apply  |
      | change-fatigue-member-id | 🔄️change-fatigue-member-id | ✅apply  |
      | change-fatigue-detail-category | 🔄️change-fatigue-detail-category | ✅apply  |
      | change-fatigue-stress-range | 🔄️change-fatigue-stress-range | ✅apply  |
      | change-fatigue-cycles | 🔄️change-fatigue-cycles | ✅apply  |
      | change-reliability-resistance-dist | 🎲️change-reliability-resistance-dist | ✅apply  |
      | change-reliability-resistance-mean | 🎲️change-reliability-resistance-mean | ✅apply  |
      | change-reliability-resistance-sd | 🎲️change-reliability-resistance-sd | ✅apply  |
      | change-reliability-action-dist | 🎲️change-reliability-action-dist | ✅apply  |
      | change-reliability-action-mean | 🎲️change-reliability-action-mean | ✅apply  |
      | change-reliability-action-sd | 🎲️change-reliability-action-sd | ✅apply  |
      | change-test-quantity | 🧪️change-test-quantity | ✅apply  |
      | change-test-pairs | 🧪️change-test-pairs | ✅apply  |
      | change-test-distribution | 🧪️change-test-distribution | ✅apply  |
      | change-test-sample-size | 🧪️change-test-sample-size | ✅apply  |
      | change-test-mean | 🧪️change-test-mean | ✅apply  |
      | change-test-std-dev | 🧪️change-test-std-dev | ✅apply  |
      | change-test-variation-known | 🧪️change-test-variation-known | ✅apply  |
      | change-test-conversion-factor | 🧪️change-test-conversion-factor | ✅apply  |
      | change-test-required | 🧪️change-test-required | ✅apply  |
      | change-test-theory-variation | 🧪️change-test-theory-variation | ✅apply  |
      | change-placement-x | 📍️change-placement-x | ✅apply  |
      | change-placement-y | 📍️change-placement-y | ✅apply  |
      | change-placement-z | 📍️change-placement-z | ✅apply  |
      | change-placement-yaw | 📍️change-placement-yaw | ✅apply  |
      | change-structure-kind | 🏛️change-structure-kind | ✅apply  |
      | insert-bridge-sls | 🌉️insert-bridge-sls | ✅apply  |
      | remove-bridge-sls | 🌉️remove-bridge-sls | ✅apply  |
      | insert-fatigue-spectrum | 🔄️insert-fatigue-spectrum | ✅apply  |
      | remove-fatigue-spectrum | 🔄️remove-fatigue-spectrum | ✅apply  |
      | insert-reliability-analysis | 🎲️insert-reliability-analysis | ✅apply  |
      | remove-reliability-analysis | 🎲️remove-reliability-analysis | ✅apply  |
      | insert-test-series | 🧪️insert-test-series | ✅apply  |
      | remove-test-series | 🧪️remove-test-series | ✅apply  |
      | insert-placement | 📍️insert-placement | ✅apply  |
      | remove-placement | 📍️remove-placement | ✅apply  |
      | change-project-id                   | 🏷️change-project-id                  | ✅apply  |
      | change-altitude-m                   | ⛰️change-altitude-m                  | ✅apply  |
      | change-consequence-class            | ⚠️change-consequence-class           | ✅apply  |
      | change-reliability-class            | 🎯change-reliability-class            | ✅apply  |
      | change-design-working-life-category | 📅change-design-working-life-category | ✅apply  |
      | change-design-working-life-years    | 📆change-design-working-life-years    | ✅apply  |
      | change-reference-period-years       | ⏱️change-reference-period-years      | ✅apply  |
      | change-supervision-level            | 👁️change-supervision-level           | ✅apply  |
      | change-inspection-level             | 🔍change-inspection-level             | ✅apply  |
      | change-beta-computed                | 📐change-beta-computed                | ✅apply  |
      | remove-effect                       | ✂️remove-effect                      | ✅apply  |
      | remove-effect-middle-row            | ✂️remove-effect                      | 🔬️middle-row |
      | remove-member                       | 🪚remove-member                       | ✅apply  |
      | remove-member-middle-row            | 🪚remove-member                       | 🔬️middle-row |
      | remove-seismic                      | 🕳️remove-seismic                     | ✅apply  |
      | remove-seismic-middle-row           | 🕳️remove-seismic                     | 🔬️middle-row |
      | remove-accidental                   | 🧯remove-accidental                   | ✅apply  |
      | remove-accidental-middle-row        | 🧯remove-accidental                   | 🔬️middle-row |
      | remove-variable                     | 📤remove-variable                     | ✅apply  |
      | remove-variable-middle-row          | 📤remove-variable                     | 🔬️middle-row |
      | remove-permanent                    | ➖remove-permanent                    | ✅apply  |
      | remove-permanent-middle-row         | ➖remove-permanent                    | 🔬️middle-row |
      | insert-effect                       | 📎insert-effect                       | ✅apply  |
      | insert-effect-clamp                 | 📎insert-effect                       | 📏clamp  |
      | insert-member                       | 🔩insert-member                       | ✅apply  |
      | insert-member-dupe                  | 🔩insert-member                       | ⛔dupe   |
      | insert-member-clamp                 | 🔩insert-member                       | 📏clamp  |
      | insert-seismic                      | 🌋insert-seismic                      | ✅apply  |
      | insert-seismic-dupe                 | 🌋insert-seismic                      | ⛔dupe   |
      | insert-seismic-clamp                | 🌋insert-seismic                      | 📏clamp  |
      | insert-accidental                   | 💣insert-accidental                   | ✅apply  |
      | insert-accidental-dupe              | 💣insert-accidental                   | ⛔dupe   |
      | insert-accidental-clamp             | 💣insert-accidental                   | 📏clamp  |
      | insert-variable                     | 📥insert-variable                     | ✅apply  |
      | insert-variable-dupe                | 📥insert-variable                     | ⛔dupe   |
      | insert-variable-clamp               | 📥insert-variable                     | 📏clamp  |
      | insert-permanent                    | ➕insert-permanent                    | ✅apply  |
      | insert-permanent-dupe               | ➕insert-permanent                    | ⛔dupe   |
      | insert-permanent-clamp              | ➕insert-permanent                    | 📏clamp  |

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
      | id                                  | dir                                  | fixture |
      | change-annex                        | 🌍️change-annex                       | ✅apply  |
      | change-traffic-group-group | 🚦️change-traffic-group-group | ✅apply  |
      | change-traffic-group-variable-id | 🚦️change-traffic-group-variable-id | ✅apply  |
      | insert-traffic-group | 🚦️insert-traffic-group | ✅apply  |
      | remove-traffic-group | 🚦️remove-traffic-group | ✅apply  |
      | change-member-material-strength-k | 🏗️change-member-material-strength-k | ✅apply  |
      | change-member-material-conv-factor | 🏗️change-member-material-conv-factor | ✅apply  |
      | change-member-mat-partial-factor | 🏗️change-member-mat-partial-factor | ✅apply  |
      | change-member-mat-design-strength | 🏗️change-member-mat-design-strength | ✅apply  |
      | change-member-nominal-dimension | 🏗️change-member-nominal-dimension | ✅apply  |
      | change-member-dimension-tolerance | 🏗️change-member-dimension-tolerance | ✅apply  |
      | change-member-design-dimension | 🏗️change-member-design-dimension | ✅apply  |
      | change-member-member-alpha-cr | 🏗️change-member-member-alpha-cr | ✅apply  |
      | change-member-second-order-analysis | 🏗️change-member-second-order-analysis | ✅apply  |
      | change-bridge-damping-ratio | 🌉️change-bridge-damping-ratio | ✅apply  |
      | change-factor-override-factor | 🎚️change-factor-override-factor | ✅apply  |
      | change-factor-override-target | 🎚️change-factor-override-target | ✅apply  |
      | change-factor-override-value | 🎚️change-factor-override-value | ✅apply  |
      | change-factor-override-justif | 🎚️change-factor-override-justif | ✅apply  |
      | insert-factor-override | 🎚️insert-factor-override | ✅apply  |
      | remove-factor-override | 🎚️remove-factor-override | ✅apply  |
      | change-member-frame-height | 🏗️change-member-frame-height | ✅apply  |
      | change-member-frame-drift | 🏗️change-member-frame-drift | ✅apply  |
      | change-member-storey-height | 🏗️change-member-storey-height | ✅apply  |
      | change-member-storey-drift | 🏗️change-member-storey-drift | ✅apply  |
      | change-bridge-track-gauge | 🌉️change-bridge-track-gauge | ✅apply  |
      | change-bridge-line-speed | 🌉️change-bridge-line-speed | ✅apply  |
      | change-bridge-ballast-depth | 🌉️change-bridge-ballast-depth | ✅apply  |
      | change-bridge-deck-mass | 🌉️change-bridge-deck-mass | ✅apply  |
      | change-bridge-natural-frequency | 🌉️change-bridge-natural-frequency | ✅apply  |
      | change-bridge-end-rotation | 🌉️change-bridge-end-rotation | ✅apply  |
      | change-bridge-basic-wind-speed | 🌉️change-bridge-basic-wind-speed | ✅apply  |
      | change-permanent-kind | ⚓️change-permanent-kind | ✅apply  |
      | change-permanent-gk | ⚓️change-permanent-gk | ✅apply  |
      | change-variable-category | 🏋️change-variable-category | ✅apply  |
      | change-variable-qk | 🏋️change-variable-qk | ✅apply  |
      | change-variable-group | 🏋️change-variable-group | ✅apply  |
      | change-accidental-ad | 💥️change-accidental-ad | ✅apply  |
      | insert-case | ➕️insert-case | ✅apply  |
      | remove-case | ➖️remove-case | ✅apply  |
      | change-case-number | 🔢️change-case-number | ✅apply  |
      | change-case-text | 🔤️change-case-text | ✅apply  |
      | change-case-label-en | 🏷️change-case-label-en | ✅apply  |
      | change-case-label-de | 🏷️change-case-label-de | ✅apply  |
      | change-case-entry-id | 📇️change-case-entry-id | ✅apply  |
      | change-seismic-a-ek | 🌋️change-seismic-a-ek | ✅apply  |
      | change-seismic-importance-class | 🌋️change-seismic-importance-class | ✅apply  |
      | change-member-label-en | 🏗️change-member-label-en | ✅apply  |
      | change-member-label-de | 🏗️change-member-label-de | ✅apply  |
      | change-member-rd-str | 🏗️change-member-rd-str | ✅apply  |
      | change-member-rd-geo | 🏗️change-member-rd-geo | ✅apply  |
      | change-member-rd-equ-stab | 🏗️change-member-rd-equ-stab | ✅apply  |
      | change-member-rd-equ-destab | 🏗️change-member-rd-equ-destab | ✅apply  |
      | change-member-rd-fat | 🏗️change-member-rd-fat | ✅apply  |
      | change-member-span | 🏗️change-member-span | ✅apply  |
      | change-member-precamber | 🏗️change-member-precamber | ✅apply  |
      | change-member-deflection-w1 | 🏗️change-member-deflection-w1 | ✅apply  |
      | change-member-deflection-w2 | 🏗️change-member-deflection-w2 | ✅apply  |
      | change-member-deflection-w3 | 🏗️change-member-deflection-w3 | ✅apply  |
      | change-member-defl-limit-ratio | 🏗️change-member-defl-limit-ratio | ✅apply  |
      | change-member-defl-limit-ratio-w3 | 🏗️change-member-defl-limit-ratio-w3 | ✅apply  |
      | change-member-sls-limit-char | 🏗️change-member-sls-limit-char | ✅apply  |
      | change-member-sls-limit-freq | 🏗️change-member-sls-limit-freq | ✅apply  |
      | change-member-sls-limit-qp | 🏗️change-member-sls-limit-qp | ✅apply  |
      | change-member-vibration-frequency | 🏗️change-member-vibration-frequency | ✅apply  |
      | change-member-vib-freq-min | 🏗️change-member-vib-freq-min | ✅apply  |
      | change-bridge-member-id | 🌉️change-bridge-member-id | ✅apply  |
      | change-bridge-deck-acceleration | 🌉️change-bridge-deck-acceleration | ✅apply  |
      | change-bridge-deck-accel-limit | 🌉️change-bridge-deck-accel-limit | ✅apply  |
      | change-bridge-deck-twist | 🌉️change-bridge-deck-twist | ✅apply  |
      | change-bridge-deck-twist-limit | 🌉️change-bridge-deck-twist-limit | ✅apply  |
      | change-bridge-deflection | 🌉️change-bridge-deflection | ✅apply  |
      | change-bridge-deflection-limit | 🌉️change-bridge-deflection-limit | ✅apply  |
      | change-effect-member-id | 🔗️change-effect-member-id | ✅apply  |
      | change-effect-action-id | 🔗️change-effect-action-id | ✅apply  |
      | change-effect-influence | 🔗️change-effect-influence | ✅apply  |
      | change-fatigue-member-id | 🔄️change-fatigue-member-id | ✅apply  |
      | change-fatigue-detail-category | 🔄️change-fatigue-detail-category | ✅apply  |
      | change-fatigue-stress-range | 🔄️change-fatigue-stress-range | ✅apply  |
      | change-fatigue-cycles | 🔄️change-fatigue-cycles | ✅apply  |
      | change-reliability-resistance-dist | 🎲️change-reliability-resistance-dist | ✅apply  |
      | change-reliability-resistance-mean | 🎲️change-reliability-resistance-mean | ✅apply  |
      | change-reliability-resistance-sd | 🎲️change-reliability-resistance-sd | ✅apply  |
      | change-reliability-action-dist | 🎲️change-reliability-action-dist | ✅apply  |
      | change-reliability-action-mean | 🎲️change-reliability-action-mean | ✅apply  |
      | change-reliability-action-sd | 🎲️change-reliability-action-sd | ✅apply  |
      | change-test-quantity | 🧪️change-test-quantity | ✅apply  |
      | change-test-pairs | 🧪️change-test-pairs | ✅apply  |
      | change-test-distribution | 🧪️change-test-distribution | ✅apply  |
      | change-test-sample-size | 🧪️change-test-sample-size | ✅apply  |
      | change-test-mean | 🧪️change-test-mean | ✅apply  |
      | change-test-std-dev | 🧪️change-test-std-dev | ✅apply  |
      | change-test-variation-known | 🧪️change-test-variation-known | ✅apply  |
      | change-test-conversion-factor | 🧪️change-test-conversion-factor | ✅apply  |
      | change-test-required | 🧪️change-test-required | ✅apply  |
      | change-test-theory-variation | 🧪️change-test-theory-variation | ✅apply  |
      | change-placement-x | 📍️change-placement-x | ✅apply  |
      | change-placement-y | 📍️change-placement-y | ✅apply  |
      | change-placement-z | 📍️change-placement-z | ✅apply  |
      | change-placement-yaw | 📍️change-placement-yaw | ✅apply  |
      | change-structure-kind | 🏛️change-structure-kind | ✅apply  |
      | insert-bridge-sls | 🌉️insert-bridge-sls | ✅apply  |
      | remove-bridge-sls | 🌉️remove-bridge-sls | ✅apply  |
      | insert-fatigue-spectrum | 🔄️insert-fatigue-spectrum | ✅apply  |
      | remove-fatigue-spectrum | 🔄️remove-fatigue-spectrum | ✅apply  |
      | insert-reliability-analysis | 🎲️insert-reliability-analysis | ✅apply  |
      | remove-reliability-analysis | 🎲️remove-reliability-analysis | ✅apply  |
      | insert-test-series | 🧪️insert-test-series | ✅apply  |
      | remove-test-series | 🧪️remove-test-series | ✅apply  |
      | insert-placement | 📍️insert-placement | ✅apply  |
      | remove-placement | 📍️remove-placement | ✅apply  |
      | change-project-id                   | 🏷️change-project-id                  | ✅apply  |
      | change-altitude-m                   | ⛰️change-altitude-m                  | ✅apply  |
      | change-consequence-class            | ⚠️change-consequence-class           | ✅apply  |
      | change-reliability-class            | 🎯change-reliability-class            | ✅apply  |
      | change-design-working-life-category | 📅change-design-working-life-category | ✅apply  |
      | change-design-working-life-years    | 📆change-design-working-life-years    | ✅apply  |
      | change-reference-period-years       | ⏱️change-reference-period-years      | ✅apply  |
      | change-supervision-level            | 👁️change-supervision-level           | ✅apply  |
      | change-inspection-level             | 🔍change-inspection-level             | ✅apply  |
      | change-beta-computed                | 📐change-beta-computed                | ✅apply  |
      | remove-effect                       | ✂️remove-effect                      | ✅apply  |
      | remove-member                       | 🪚remove-member                       | ✅apply  |
      | remove-seismic                      | 🕳️remove-seismic                     | ✅apply  |
      | remove-accidental                   | 🧯remove-accidental                   | ✅apply  |
      | remove-variable                     | 📤remove-variable                     | ✅apply  |
      | remove-permanent                    | ➖remove-permanent                    | ✅apply  |
      | insert-effect                       | 📎insert-effect                       | ✅apply  |
      | insert-member                       | 🔩insert-member                       | ✅apply  |
      | insert-seismic                      | 🌋insert-seismic                      | ✅apply  |
      | insert-accidental                   | 💣insert-accidental                   | ✅apply  |
      | insert-variable                     | 📥insert-variable                     | ✅apply  |
      | insert-permanent                    | ➕insert-permanent                    | ✅apply  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1990 document from the parsed carrier
    Given the real committed text artifact asset://🏢️high-consequence-office/🏢️high-consequence-office/🗣️.dsl.semio
    And its committed binary twin asset://🏢️high-consequence-office/🎒️.pack.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
