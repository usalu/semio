@capability-en1992-1-mutate
@oracle-en1992-1-python-independent
@comparison-ordered-json-v1
@mutations-en1992-1-any
Feature: Apply every typed EN 1992 mutation against an independent Python implementation
  `s.norm.en1992` is a semio-NATIVE artifact and no third party reads or writes it, so the second producer a
  differential comparison needs is a second IMPLEMENTATION: the shared norm reference engine
  (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`), which `🐍️.py` beside this file feeds with this subset's
  kind list, vectors and carrier. It is written from the repository's own specification of what a semantic
  mutation means (the verb table, the `new<Field>` naming mechanic, the addressing convention and the derivation
  rules) and imports nothing from the Rust it judges.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below is a
  declared `shared://` fixture, so neither side holds a transcription that could drift. The 28 `✅apply` vectors
  cover every kind of the current vocabulary (24 `change`, 2 `insert`, 2 `remove`) on a liquid-retaining RC wall with a post-installed anchor; each vector's after-snapshot and
  diff were written by production dispatch and its mutation is the canonical Rust wire. Each side asserts the same
  laws in role — the applied document must BE the committed after-snapshot, an `applied` vector must move the
  document, and the mutation followed by its OWN computed inverse must restore the before-snapshot exactly, list
  position included. `parity` adds that two implementations, in two languages, reach the same document.

  The 2 refusal rows (`⛔dupe`, `❓gone`) re-apply a kind's applied mutation to the
  after-snapshot it produced: re-inserting an id the collection now holds must be refused `mutation.duplicate-id`
  (Fatal), re-removing a member that is gone `mutation.target-missing` (Error) and re-setting a value the document
  already has must report `mutation.no-op` (Warning). Both sides must refuse under the committed code and leave the
  document bit-identical; a refusal has nothing to undo, so these rows are `mutate-` only.

  `inverse-` projects BOTH the mutated and the restored document, so the mutated half distinguishes the rows.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `asset://🛢️liquid-retaining-fem-anchor/🛢️liquid-retaining-fem-anchor/🗣️.dsl.semio`. The carrier has no published grammar: the committed
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
      | change-title                  | 🏷️change-title                  | ✅apply  |
      | change-design-working-life    | 📅️change-design-working-life    | ✅apply  |
      | change-delta-c-dev            | 📏️change-delta-c-dev            | ✅apply  |
      | change-cement-type            | 🧪change-cement-type             | ✅apply  |
      | change-concrete-f-ck          | 🧱change-concrete-f-ck           | ✅apply  |
      | change-reinforcement-f-yk     | 🔩change-reinforcement-f-yk      | ✅apply  |
      | insert-member                 | ➕️insert-member                 | ✅apply  |
      | remove-member                 | ➖️remove-member                 | ✅apply  |
      | change-member-order               | 🔀️change-member-order               | ✅apply  |
      | change-member-width           | ↔️change-member-width           | ✅apply  |
      | change-member-height          | ↕️change-member-height          | ✅apply  |
      | change-member-effective-depth | 📐️change-member-effective-depth | ✅apply  |
      | change-member-cover           | 🛡️change-member-cover           | ✅apply  |
      | change-stud-row-angle | 🔩️change-stud-row-angle | ✅apply  |
      | change-stud-row-count | 🔩️change-stud-row-count | ✅apply  |
      | change-stud-row-spacing-first | 🔩️change-stud-row-spacing-first | ✅apply  |
      | change-stud-row-spacing-radial | 🔩️change-stud-row-spacing-radial | ✅apply  |
      | change-stud-row-diameter | 🔩️change-stud-row-diameter | ✅apply  |
      | insert-stud-row | 🔩️insert-stud-row | ✅apply  |
      | remove-stud-row | 🔩️remove-stud-row | ✅apply  |
      | change-opening-x | 🕳️change-opening-x | ✅apply  |
      | change-opening-y | 🕳️change-opening-y | ✅apply  |
      | change-opening-width | 🕳️change-opening-width | ✅apply  |
      | change-opening-depth | 🕳️change-opening-depth | ✅apply  |
      | insert-opening | 🕳️insert-opening | ✅apply  |
      | remove-opening | 🕳️remove-opening | ✅apply  |
      | change-member-punching-beta-method | 🏛️change-member-punching-beta-method | ✅apply  |
      | change-member-punching-moment-c1 | 🏛️change-member-punching-moment-c1 | ✅apply  |
      | change-member-punching-moment-c2 | 🏛️change-member-punching-moment-c2 | ✅apply  |
      | change-bar-point-length | 📍️change-bar-point-length | ✅apply  |
      | change-bar-point-bend | 📍️change-bar-point-bend | ✅apply  |
      | change-placement-x | 📍️change-placement-x | ✅apply  |
      | change-placement-y | 📍️change-placement-y | ✅apply  |
      | change-placement-z | 📍️change-placement-z | ✅apply  |
      | change-placement-yaw | 📍️change-placement-yaw | ✅apply  |
      | insert-placement | 📍️insert-placement | ✅apply  |
      | remove-placement | 📍️remove-placement | ✅apply  |
      | change-concrete-name | 🧱️change-concrete-name | ✅apply  |
      | change-reinforcement-name | 🔩️change-reinforcement-name | ✅apply  |
      | change-reinforcement-es | 🔩️change-reinforcement-es | ✅apply  |
      | change-reinforcement-ductility | 🔩️change-reinforcement-ductility | ✅apply  |
      | change-reinforcement-k | 🔩️change-reinforcement-k | ✅apply  |
      | change-reinforcement-eps-uk | 🔩️change-reinforcement-eps-uk | ✅apply  |
      | change-prestress-steel-name | 🧵️change-prestress-steel-name | ✅apply  |
      | change-prestress-steel-f-pk | 🧵️change-prestress-steel-f-pk | ✅apply  |
      | change-prestress-steel-fp01k | 🧵️change-prestress-steel-fp01k | ✅apply  |
      | change-member-label-en | 🏛️change-member-label-en | ✅apply  |
      | change-member-label-de | 🏛️change-member-label-de | ✅apply  |
      | change-member-kind | 🏛️change-member-kind | ✅apply  |
      | change-member-concrete-grade-id | 🏛️change-member-concrete-grade-id | ✅apply  |
      | change-member-rebar-grade-id | 🏛️change-member-rebar-grade-id | ✅apply  |
      | change-member-prestress-steel-id | 🏛️change-member-prestress-steel-id | ✅apply  |
      | change-member-support | 🏛️change-member-support | ✅apply  |
      | change-member-stirrups-diameter | 🏛️change-member-stirrups-diameter | ✅apply  |
      | change-member-stirrups-legs | 🏛️change-member-stirrups-legs | ✅apply  |
      | change-member-punch-column-width | 🏛️change-member-punch-column-width | ✅apply  |
      | change-member-punch-column-depth | 🏛️change-member-punch-column-depth | ✅apply  |
      | change-member-punch-col-position | 🏛️change-member-punch-col-position | ✅apply  |
      | change-member-punching-asw | 🏛️change-member-punching-asw | ✅apply  |
      | change-member-prestress-force | 🏛️change-member-prestress-force | ✅apply  |
      | change-member-prestress-area | 🏛️change-member-prestress-area | ✅apply  |
      | change-member-prestress-ecc | 🏛️change-member-prestress-ecc | ✅apply  |
      | change-member-prestress-loss-ratio | 🏛️change-member-prestress-loss-ratio | ✅apply  |
      | change-member-fire-slab-system | 🏛️change-member-fire-slab-system | ✅apply  |
      | change-member-use-fem | 🏛️change-member-use-fem | ✅apply  |
      | change-member-deflection-sensitive | 🏛️change-member-deflection-sensitive | ✅apply  |
      | change-bar-layer-position | ⭕️change-bar-layer-position | ✅apply  |
      | change-bar-layer-anchorage-length | ⭕️change-bar-layer-anchorage-length | ✅apply  |
      | change-bar-layer-lap-length | ⭕️change-bar-layer-lap-length | ✅apply  |
      | change-bar-layer-bond-condition | ⭕️change-bar-layer-bond-condition | ✅apply  |
      | change-bar-layer-aggregate-size | ⭕️change-bar-layer-aggregate-size | ✅apply  |
      | change-bar-layer-lap-share | ⭕️change-bar-layer-lap-share | ✅apply  |
      | change-action-kind | 🏋️change-action-kind | ✅apply  |
      | change-action-category | 🏋️change-action-category | ✅apply  |
      | change-action-source | 🏋️change-action-source | ✅apply  |
      | change-action-gk-line | 🏋️change-action-gk-line | ✅apply  |
      | change-action-qk-line | 🏋️change-action-qk-line | ✅apply  |
      | change-action-point-force | 🏋️change-action-point-force | ✅apply  |
      | change-action-tk | 🏋️change-action-tk | ✅apply  |
      | change-action-vk-punch | 🏋️change-action-vk-punch | ✅apply  |
      | change-section-point-y | 📐️change-section-point-y | ✅apply  |
      | change-section-point-z | 📐️change-section-point-z | ✅apply  |
      | change-bar-point-layer-id | 📍️change-bar-point-layer-id | ✅apply  |
      | change-bar-point-y | 📍️change-bar-point-y | ✅apply  |
      | change-bar-point-z | 📍️change-bar-point-z | ✅apply  |
      | change-bar-point-diameter | 📍️change-bar-point-diameter | ✅apply  |
      | change-tendon-point-x | 🧬️change-tendon-point-x | ✅apply  |
      | change-tendon-point-y | 🧬️change-tendon-point-y | ✅apply  |
      | change-tendon-point-z | 🧬️change-tendon-point-z | ✅apply  |
      | change-anchor-cracked | ⚓️change-anchor-cracked | ✅apply  |
      | change-anchor-f-uk | ⚓️change-anchor-f-uk | ✅apply  |
      | change-anchor-f-yk | ⚓️change-anchor-f-yk | ✅apply  |
      | change-anchor-d | ⚓️change-anchor-d | ✅apply  |
      | change-anchor-c1 | ⚓️change-anchor-c1 | ✅apply  |
      | change-anchor-f-ck | ⚓️change-anchor-f-ck | ✅apply  |
      | change-anchor-head-diameter | ⚓️change-anchor-head-diameter | ✅apply  |
      | change-anchor-action-kind | 🏋️change-anchor-action-kind | ✅apply  |
      | change-anchor-action-category | 🏋️change-anchor-action-category | ✅apply  |
      | change-anchor-action-source | 🏋️change-anchor-action-source | ✅apply  |
      | change-anchor-action-gk-line | 🏋️change-anchor-action-gk-line | ✅apply  |
      | change-anchor-action-qk-line | 🏋️change-anchor-action-qk-line | ✅apply  |
      | change-anchor-action-point-force | 🏋️change-anchor-action-point-force | ✅apply  |
      | change-anchor-action-mk | 🏋️change-anchor-action-mk | ✅apply  |
      | change-anchor-action-nk | 🏋️change-anchor-action-nk | ✅apply  |
      | change-anchor-action-vk | 🏋️change-anchor-action-vk | ✅apply  |
      | change-anchor-action-tk | 🏋️change-anchor-action-tk | ✅apply  |
      | change-anchor-action-vk-punch | 🏋️change-anchor-action-vk-punch | ✅apply  |
      | insert-concrete-grade | 🧱️insert-concrete-grade | ✅apply  |
      | remove-concrete-grade | 🧱️remove-concrete-grade | ✅apply  |
      | insert-reinforcement-grade | 🔩️insert-reinforcement-grade | ✅apply  |
      | remove-reinforcement-grade | 🔩️remove-reinforcement-grade | ✅apply  |
      | insert-prestress-steel | 🧵️insert-prestress-steel | ✅apply  |
      | remove-prestress-steel | 🧵️remove-prestress-steel | ✅apply  |
      | insert-bar-layer | ⭕️insert-bar-layer | ✅apply  |
      | remove-bar-layer | ⭕️remove-bar-layer | ✅apply  |
      | insert-action | 🏋️insert-action | ✅apply  |
      | remove-action | 🏋️remove-action | ✅apply  |
      | insert-section-point | 📐️insert-section-point | ✅apply  |
      | remove-section-point | 📐️remove-section-point | ✅apply  |
      | insert-bar-point | 📍️insert-bar-point | ✅apply  |
      | remove-bar-point | 📍️remove-bar-point | ✅apply  |
      | insert-tendon-point | 🧬️insert-tendon-point | ✅apply  |
      | remove-tendon-point | 🧬️remove-tendon-point | ✅apply  |
      | insert-anchor-action | 🏋️insert-anchor-action | ✅apply  |
      | remove-anchor-action | 🏋️remove-anchor-action | ✅apply  |
      | insert-case | ➕️insert-case | ✅apply  |
      | remove-case | ➖️remove-case | ✅apply  |
      | change-case-number | 🔢️change-case-number | ✅apply  |
      | change-case-text | 🔤️change-case-text | ✅apply  |
      | change-case-label-en | 🏷️change-case-label-en | ✅apply  |
      | change-case-label-de | 🏷️change-case-label-de | ✅apply  |
      | change-case-entry-id | 📇️change-case-entry-id | ✅apply  |
      | change-member-tightness | 🚿change-member-tightness | ✅apply  |
      | clear-member-tightness | 🧽clear-member-tightness | ✅apply  |
      | change-member-exposure        | 🌦️change-member-exposure        | ✅apply  |
      | change-member-span            | 🌉️change-member-span            | ✅apply  |
      | change-member-stirrup-spacing | 🪢change-member-stirrup-spacing  | ✅apply  |
      | change-member-axis-distance   | 🔥change-member-axis-distance    | ✅apply  |
      | change-member-fire-rating     | 🔥️change-member-fire-rating     | ✅apply  |
      | change-bar-layer-count        | #️⃣change-bar-layer-count       | ✅apply  |
      | change-bar-layer-diameter     | ⭕change-bar-layer-diameter      | ✅apply  |
      | change-action-mk              | ⤴️change-action-mk              | ✅apply  |
      | change-action-nk              | 🏋️change-action-nk              | ✅apply  |
      | change-action-vk              | ↘️change-action-vk              | ✅apply  |
      | insert-anchor                 | ⚓️insert-anchor                 | ✅apply  |
      | insert-anchor-dupe            | ⚓️insert-anchor                 | ⛔dupe   |
      | remove-anchor                 | 🗑️remove-anchor                 | ✅apply  |
      | remove-anchor-gone            | 🗑️remove-anchor                 | ❓gone   |
      | change-anchor-h-ef            | 📍change-anchor-h-ef             | ✅apply  |
      | change-anchor-as              | 🧷change-anchor-as               | ✅apply  |
      | change-member-bridge-delta-sigma-s | 🎢change-member-bridge-delta-sigma-s | ✅apply  |
      | change-member-bridge-sigma-c  | 🌁change-member-bridge-sigma-c    | ✅apply  |
      | change-member-liquid-sr-max  | 🌊change-member-liquid-sr-max    | ✅apply  |
      | change-member-liquid-f-ct-eff | 🧊change-member-liquid-f-ct-eff   | ✅apply  |
      | change-member-liquid-rho-p-eff | 🫧change-member-liquid-rho-p-eff  | ✅apply  |
      | change-member-liquid-sigma-s  | 💦change-member-liquid-sigma-s    | ✅apply  |
      | change-member-hd-over-h       | 🚰change-member-hd-over-h         | ✅apply  |
      | change-member-udl             | 🪜change-member-udl               | ✅apply  |
      | change-member-buckling-length | 🧵change-member-buckling-length   | ✅apply  |
      | change-member-relative-humidity | 💧change-member-relative-humidity | ✅apply  |
      | change-member-bearing-width | 💧change-member-bearing-width | ✅apply  |
      | remove-member-middle          | ➖️remove-member                 | 🧭️middle |
      | change-member-order-middle        | 🔀️change-member-order               | 🧭️middle |
      | remove-anchor-middle          | 🗑️remove-anchor                 | 🧭️middle |

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
      | change-title                  | 🏷️change-title                  | ✅apply  |
      | change-design-working-life    | 📅️change-design-working-life    | ✅apply  |
      | change-delta-c-dev            | 📏️change-delta-c-dev            | ✅apply  |
      | change-cement-type            | 🧪change-cement-type             | ✅apply  |
      | change-concrete-f-ck          | 🧱change-concrete-f-ck           | ✅apply  |
      | change-reinforcement-f-yk     | 🔩change-reinforcement-f-yk      | ✅apply  |
      | insert-member                 | ➕️insert-member                 | ✅apply  |
      | remove-member                 | ➖️remove-member                 | ✅apply  |
      | change-member-order               | 🔀️change-member-order               | ✅apply  |
      | change-member-width           | ↔️change-member-width           | ✅apply  |
      | change-member-height          | ↕️change-member-height          | ✅apply  |
      | change-member-effective-depth | 📐️change-member-effective-depth | ✅apply  |
      | change-member-cover           | 🛡️change-member-cover           | ✅apply  |
      | change-stud-row-angle | 🔩️change-stud-row-angle | ✅apply  |
      | change-stud-row-count | 🔩️change-stud-row-count | ✅apply  |
      | change-stud-row-spacing-first | 🔩️change-stud-row-spacing-first | ✅apply  |
      | change-stud-row-spacing-radial | 🔩️change-stud-row-spacing-radial | ✅apply  |
      | change-stud-row-diameter | 🔩️change-stud-row-diameter | ✅apply  |
      | insert-stud-row | 🔩️insert-stud-row | ✅apply  |
      | remove-stud-row | 🔩️remove-stud-row | ✅apply  |
      | change-opening-x | 🕳️change-opening-x | ✅apply  |
      | change-opening-y | 🕳️change-opening-y | ✅apply  |
      | change-opening-width | 🕳️change-opening-width | ✅apply  |
      | change-opening-depth | 🕳️change-opening-depth | ✅apply  |
      | insert-opening | 🕳️insert-opening | ✅apply  |
      | remove-opening | 🕳️remove-opening | ✅apply  |
      | change-member-punching-beta-method | 🏛️change-member-punching-beta-method | ✅apply  |
      | change-member-punching-moment-c1 | 🏛️change-member-punching-moment-c1 | ✅apply  |
      | change-member-punching-moment-c2 | 🏛️change-member-punching-moment-c2 | ✅apply  |
      | change-bar-point-length | 📍️change-bar-point-length | ✅apply  |
      | change-bar-point-bend | 📍️change-bar-point-bend | ✅apply  |
      | change-placement-x | 📍️change-placement-x | ✅apply  |
      | change-placement-y | 📍️change-placement-y | ✅apply  |
      | change-placement-z | 📍️change-placement-z | ✅apply  |
      | change-placement-yaw | 📍️change-placement-yaw | ✅apply  |
      | insert-placement | 📍️insert-placement | ✅apply  |
      | remove-placement | 📍️remove-placement | ✅apply  |
      | change-concrete-name | 🧱️change-concrete-name | ✅apply  |
      | change-reinforcement-name | 🔩️change-reinforcement-name | ✅apply  |
      | change-reinforcement-es | 🔩️change-reinforcement-es | ✅apply  |
      | change-reinforcement-ductility | 🔩️change-reinforcement-ductility | ✅apply  |
      | change-reinforcement-k | 🔩️change-reinforcement-k | ✅apply  |
      | change-reinforcement-eps-uk | 🔩️change-reinforcement-eps-uk | ✅apply  |
      | change-prestress-steel-name | 🧵️change-prestress-steel-name | ✅apply  |
      | change-prestress-steel-f-pk | 🧵️change-prestress-steel-f-pk | ✅apply  |
      | change-prestress-steel-fp01k | 🧵️change-prestress-steel-fp01k | ✅apply  |
      | change-member-label-en | 🏛️change-member-label-en | ✅apply  |
      | change-member-label-de | 🏛️change-member-label-de | ✅apply  |
      | change-member-kind | 🏛️change-member-kind | ✅apply  |
      | change-member-concrete-grade-id | 🏛️change-member-concrete-grade-id | ✅apply  |
      | change-member-rebar-grade-id | 🏛️change-member-rebar-grade-id | ✅apply  |
      | change-member-prestress-steel-id | 🏛️change-member-prestress-steel-id | ✅apply  |
      | change-member-support | 🏛️change-member-support | ✅apply  |
      | change-member-stirrups-diameter | 🏛️change-member-stirrups-diameter | ✅apply  |
      | change-member-stirrups-legs | 🏛️change-member-stirrups-legs | ✅apply  |
      | change-member-punch-column-width | 🏛️change-member-punch-column-width | ✅apply  |
      | change-member-punch-column-depth | 🏛️change-member-punch-column-depth | ✅apply  |
      | change-member-punch-col-position | 🏛️change-member-punch-col-position | ✅apply  |
      | change-member-punching-asw | 🏛️change-member-punching-asw | ✅apply  |
      | change-member-prestress-force | 🏛️change-member-prestress-force | ✅apply  |
      | change-member-prestress-area | 🏛️change-member-prestress-area | ✅apply  |
      | change-member-prestress-ecc | 🏛️change-member-prestress-ecc | ✅apply  |
      | change-member-prestress-loss-ratio | 🏛️change-member-prestress-loss-ratio | ✅apply  |
      | change-member-fire-slab-system | 🏛️change-member-fire-slab-system | ✅apply  |
      | change-member-use-fem | 🏛️change-member-use-fem | ✅apply  |
      | change-member-deflection-sensitive | 🏛️change-member-deflection-sensitive | ✅apply  |
      | change-bar-layer-position | ⭕️change-bar-layer-position | ✅apply  |
      | change-bar-layer-anchorage-length | ⭕️change-bar-layer-anchorage-length | ✅apply  |
      | change-bar-layer-lap-length | ⭕️change-bar-layer-lap-length | ✅apply  |
      | change-bar-layer-bond-condition | ⭕️change-bar-layer-bond-condition | ✅apply  |
      | change-bar-layer-aggregate-size | ⭕️change-bar-layer-aggregate-size | ✅apply  |
      | change-bar-layer-lap-share | ⭕️change-bar-layer-lap-share | ✅apply  |
      | change-action-kind | 🏋️change-action-kind | ✅apply  |
      | change-action-category | 🏋️change-action-category | ✅apply  |
      | change-action-source | 🏋️change-action-source | ✅apply  |
      | change-action-gk-line | 🏋️change-action-gk-line | ✅apply  |
      | change-action-qk-line | 🏋️change-action-qk-line | ✅apply  |
      | change-action-point-force | 🏋️change-action-point-force | ✅apply  |
      | change-action-tk | 🏋️change-action-tk | ✅apply  |
      | change-action-vk-punch | 🏋️change-action-vk-punch | ✅apply  |
      | change-section-point-y | 📐️change-section-point-y | ✅apply  |
      | change-section-point-z | 📐️change-section-point-z | ✅apply  |
      | change-bar-point-layer-id | 📍️change-bar-point-layer-id | ✅apply  |
      | change-bar-point-y | 📍️change-bar-point-y | ✅apply  |
      | change-bar-point-z | 📍️change-bar-point-z | ✅apply  |
      | change-bar-point-diameter | 📍️change-bar-point-diameter | ✅apply  |
      | change-tendon-point-x | 🧬️change-tendon-point-x | ✅apply  |
      | change-tendon-point-y | 🧬️change-tendon-point-y | ✅apply  |
      | change-tendon-point-z | 🧬️change-tendon-point-z | ✅apply  |
      | change-anchor-cracked | ⚓️change-anchor-cracked | ✅apply  |
      | change-anchor-f-uk | ⚓️change-anchor-f-uk | ✅apply  |
      | change-anchor-f-yk | ⚓️change-anchor-f-yk | ✅apply  |
      | change-anchor-d | ⚓️change-anchor-d | ✅apply  |
      | change-anchor-c1 | ⚓️change-anchor-c1 | ✅apply  |
      | change-anchor-f-ck | ⚓️change-anchor-f-ck | ✅apply  |
      | change-anchor-head-diameter | ⚓️change-anchor-head-diameter | ✅apply  |
      | change-anchor-action-kind | 🏋️change-anchor-action-kind | ✅apply  |
      | change-anchor-action-category | 🏋️change-anchor-action-category | ✅apply  |
      | change-anchor-action-source | 🏋️change-anchor-action-source | ✅apply  |
      | change-anchor-action-gk-line | 🏋️change-anchor-action-gk-line | ✅apply  |
      | change-anchor-action-qk-line | 🏋️change-anchor-action-qk-line | ✅apply  |
      | change-anchor-action-point-force | 🏋️change-anchor-action-point-force | ✅apply  |
      | change-anchor-action-mk | 🏋️change-anchor-action-mk | ✅apply  |
      | change-anchor-action-nk | 🏋️change-anchor-action-nk | ✅apply  |
      | change-anchor-action-vk | 🏋️change-anchor-action-vk | ✅apply  |
      | change-anchor-action-tk | 🏋️change-anchor-action-tk | ✅apply  |
      | change-anchor-action-vk-punch | 🏋️change-anchor-action-vk-punch | ✅apply  |
      | insert-concrete-grade | 🧱️insert-concrete-grade | ✅apply  |
      | remove-concrete-grade | 🧱️remove-concrete-grade | ✅apply  |
      | insert-reinforcement-grade | 🔩️insert-reinforcement-grade | ✅apply  |
      | remove-reinforcement-grade | 🔩️remove-reinforcement-grade | ✅apply  |
      | insert-prestress-steel | 🧵️insert-prestress-steel | ✅apply  |
      | remove-prestress-steel | 🧵️remove-prestress-steel | ✅apply  |
      | insert-bar-layer | ⭕️insert-bar-layer | ✅apply  |
      | remove-bar-layer | ⭕️remove-bar-layer | ✅apply  |
      | insert-action | 🏋️insert-action | ✅apply  |
      | remove-action | 🏋️remove-action | ✅apply  |
      | insert-section-point | 📐️insert-section-point | ✅apply  |
      | remove-section-point | 📐️remove-section-point | ✅apply  |
      | insert-bar-point | 📍️insert-bar-point | ✅apply  |
      | remove-bar-point | 📍️remove-bar-point | ✅apply  |
      | insert-tendon-point | 🧬️insert-tendon-point | ✅apply  |
      | remove-tendon-point | 🧬️remove-tendon-point | ✅apply  |
      | insert-anchor-action | 🏋️insert-anchor-action | ✅apply  |
      | remove-anchor-action | 🏋️remove-anchor-action | ✅apply  |
      | insert-case | ➕️insert-case | ✅apply  |
      | remove-case | ➖️remove-case | ✅apply  |
      | change-case-number | 🔢️change-case-number | ✅apply  |
      | change-case-text | 🔤️change-case-text | ✅apply  |
      | change-case-label-en | 🏷️change-case-label-en | ✅apply  |
      | change-case-label-de | 🏷️change-case-label-de | ✅apply  |
      | change-case-entry-id | 📇️change-case-entry-id | ✅apply  |
      | change-member-tightness | 🚿change-member-tightness | ✅apply  |
      | clear-member-tightness | 🧽clear-member-tightness | ✅apply  |
      | change-member-exposure        | 🌦️change-member-exposure        | ✅apply  |
      | change-member-span            | 🌉️change-member-span            | ✅apply  |
      | change-member-stirrup-spacing | 🪢change-member-stirrup-spacing  | ✅apply  |
      | change-member-axis-distance   | 🔥change-member-axis-distance    | ✅apply  |
      | change-member-fire-rating     | 🔥️change-member-fire-rating     | ✅apply  |
      | change-bar-layer-count        | #️⃣change-bar-layer-count       | ✅apply  |
      | change-bar-layer-diameter     | ⭕change-bar-layer-diameter      | ✅apply  |
      | change-action-mk              | ⤴️change-action-mk              | ✅apply  |
      | change-action-nk              | 🏋️change-action-nk              | ✅apply  |
      | change-action-vk              | ↘️change-action-vk              | ✅apply  |
      | insert-anchor                 | ⚓️insert-anchor                 | ✅apply  |
      | remove-anchor                 | 🗑️remove-anchor                 | ✅apply  |
      | change-anchor-h-ef            | 📍change-anchor-h-ef             | ✅apply  |
      | change-anchor-as              | 🧷change-anchor-as               | ✅apply  |
      | change-member-bridge-delta-sigma-s | 🎢change-member-bridge-delta-sigma-s | ✅apply  |
      | change-member-bridge-sigma-c  | 🌁change-member-bridge-sigma-c    | ✅apply  |
      | change-member-liquid-sr-max  | 🌊change-member-liquid-sr-max    | ✅apply  |
      | change-member-liquid-f-ct-eff | 🧊change-member-liquid-f-ct-eff   | ✅apply  |
      | change-member-liquid-rho-p-eff | 🫧change-member-liquid-rho-p-eff  | ✅apply  |
      | change-member-liquid-sigma-s  | 💦change-member-liquid-sigma-s    | ✅apply  |
      | change-member-hd-over-h       | 🚰change-member-hd-over-h         | ✅apply  |
      | change-member-udl             | 🪜change-member-udl               | ✅apply  |
      | change-member-buckling-length | 🧵change-member-buckling-length   | ✅apply  |
      | change-member-relative-humidity | 💧change-member-relative-humidity | ✅apply  |
      | change-member-bearing-width | 💧change-member-bearing-width | ✅apply  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1992 document from the parsed carrier
    Given the real committed text artifact asset://🛢️liquid-retaining-fem-anchor/🛢️liquid-retaining-fem-anchor/🗣️.dsl.semio
    And its committed binary twin asset://🛢️liquid-retaining-fem-anchor/🎒️.pack.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
