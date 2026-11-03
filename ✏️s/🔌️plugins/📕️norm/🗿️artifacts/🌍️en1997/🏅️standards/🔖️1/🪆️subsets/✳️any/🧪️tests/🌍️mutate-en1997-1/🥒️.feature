@capability-en1997-1-mutate
@oracle-en1997-1-python-independent
@comparison-ordered-json-v1
@mutations-en1997-1-any
Feature: Apply every typed EN 1997 mutation against an independent Python implementation
  `s.norm.en1997` is a semio-NATIVE artifact and no third party reads or writes it, so the second producer a
  differential comparison needs is a second IMPLEMENTATION: the shared norm reference engine
  (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`), which `🐍️.py` beside this file feeds with this subset's
  kind list, vectors and carrier. It is written from the repository's own specification of what a semantic
  mutation means (the verb table, the `new<Field>` naming mechanic, the addressing convention and the derivation
  rules) and imports nothing from the Rust it judges.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below is a
  declared `shared://` fixture, so neither side holds a transcription that could drift. The 147 `✅apply` vectors
  cover every kind of the current vocabulary (117 typed `change`, 15 `insert`, 15 `remove`; every editable leaf has its own kind and there is no whole-row update) on a spread foundation, piles, a retaining wall and a slope on layered soil; each vector's after-snapshot and
  diff were written by production dispatch and its mutation is the canonical Rust wire. Each side asserts the same
  laws in role — the applied document must BE the committed after-snapshot, an `applied` vector must move the
  document, and the mutation followed by its OWN computed inverse must restore the before-snapshot exactly, list
  position included. `parity` adds that two implementations, in two languages, reach the same document.

  The 3 refusal rows (`⛔dupe`) re-apply a kind's applied mutation to the
  after-snapshot it produced: re-inserting an id the collection now holds must be refused `mutation.duplicate-id`
  (Fatal), re-removing a member that is gone `mutation.target-missing` (Error) and re-setting a value the document
  already has must report `mutation.no-op` (Warning). Both sides must refuse under the committed code and leave the
  document bit-identical; a refusal has nothing to undo, so these rows are `mutate-` only.

  `inverse-` projects BOTH the mutated and the restored document, so the mutated half distinguishes the rows.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `asset://🎬️demo/🗣️.dsl.semio`. The carrier has no published grammar: the committed
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
      | id                              | dir                               | fixture |
      | change-annex                    | 🌍️change-annex                    | ✅apply  |
      | change-geotechnical-category    | 🗂️change-geotechnical-category    | ✅apply  |
      | change-design-situation         | 📅️change-design-situation         | ✅apply  |
      | change-design-approach          | 🧭️change-design-approach          | ✅apply  |
      | change-groundwater-level        | 💧change-groundwater-level         | ✅apply  |
      | change-investigation-depth      | 🔎️change-investigation-depth      | ✅apply  |
      | change-footing-width            | ↔️change-footing-width            | ✅apply  |
      | change-footing-embedment        | ⬇️change-footing-embedment        | ✅apply  |
      | change-pile-length              | 📏️change-pile-length              | ✅apply  |
      | change-pile-count               | 🔢change-pile-count                | ✅apply  |
      | change-wall-base-width          | 🧱change-wall-base-width           | ✅apply  |
      | change-slope-angle              | ⛰️change-slope-angle              | ✅apply  |
      | change-placement-x | 📍️change-placement-x | ✅apply  |
      | change-placement-y | 📍️change-placement-y | ✅apply  |
      | change-placement-z | 📍️change-placement-z | ✅apply  |
      | change-placement-yaw | 📍️change-placement-yaw | ✅apply  |
      | insert-placement | 📍️insert-placement | ✅apply  |
      | remove-placement | 📍️remove-placement | ✅apply  |
      | change-layer-phi-prime          | 📐️change-layer-phi-prime          | ✅apply  |
      | change-layer-oedometric-modulus | 🌀️change-layer-oedometric-modulus | ✅apply  |
      | insert-layer                    | ➕️insert-layer                    | ✅apply  |
      | insert-layer-dupe               | ➕️insert-layer                    | ⛔dupe   |
      | remove-layer                    | ➖️remove-layer                    | ✅apply  |
      | insert-footing                  | ➕insert-footing                   | ✅apply  |
      | insert-footing-dupe             | ➕insert-footing                   | ⛔dupe   |
      | remove-footing                  | ➖remove-footing                   | ✅apply  |
      | insert-pile                     | 📥insert-pile                      | ✅apply  |
      | insert-pile-dupe                | 📥insert-pile                      | ⛔dupe   |
      | remove-pile                     | 📤remove-pile                      | ✅apply  |
      | remove-footing-middle           | ➖remove-footing                   | 🧭️middle |
      | remove-layer-middle             | ➖️remove-layer                    | 🧭️middle |
      | remove-pile-middle              | 📤remove-pile                      | 🧭️middle |
      | insert-retaining-wall           | ➕insert-retaining-wall            | ✅apply  |
      | insert-retaining-wall-dupe      | ➕insert-retaining-wall            | ⛔dupe   |
      | remove-retaining-wall           | ➖remove-retaining-wall            | ✅apply  |
      | insert-slope                    | ➕insert-slope                     | ✅apply  |
      | insert-slope-dupe               | ➕insert-slope                     | ⛔dupe   |
      | remove-slope                    | ➖remove-slope                     | ✅apply  |
      | insert-uplift-case              | ➕insert-uplift-case               | ✅apply  |
      | insert-uplift-case-dupe         | ➕insert-uplift-case               | ⛔dupe   |
      | remove-uplift-case              | ➖remove-uplift-case               | ✅apply  |
      | insert-anchor                   | ➕insert-anchor                    | ✅apply  |
      | insert-anchor-dupe              | ➕insert-anchor                    | ⛔dupe   |
      | remove-anchor                   | ➖remove-anchor                    | ✅apply  |
      | insert-embedded-wall            | ➕insert-embedded-wall             | ✅apply  |
      | insert-embedded-wall-dupe       | ➕insert-embedded-wall             | ⛔dupe   |
      | remove-embedded-wall            | ➖remove-embedded-wall             | ✅apply  |
      | insert-pile-group               | ➕insert-pile-group                | ✅apply  |
      | insert-pile-group-dupe          | ➕insert-pile-group                | ⛔dupe   |
      | remove-pile-group               | ➖remove-pile-group                | ✅apply  |
      | insert-lab-test                 | ➕insert-lab-test                  | ✅apply  |
      | insert-lab-test-dupe            | ➕insert-lab-test                  | ⛔dupe   |
      | remove-lab-test                 | ➖remove-lab-test                  | ✅apply  |
      | insert-layer-water              | ➕insert-layer-water               | ✅apply  |
      | insert-layer-water-dupe         | ➕insert-layer-water               | ⛔dupe   |
      | remove-layer-water              | ➖remove-layer-water               | ✅apply  |
      | insert-characteristic-basis              | ➕insert-characteristic-basis               | ✅apply  |
      | insert-characteristic-basis-dupe         | ➕insert-characteristic-basis               | ⛔dupe   |
      | remove-characteristic-basis              | ➖remove-characteristic-basis               | ✅apply  |
      | insert-ground-investigation              | ➕insert-ground-investigation               | ✅apply  |
      | insert-ground-investigation-dupe         | ➕insert-ground-investigation               | ⛔dupe   |
      | remove-ground-investigation              | ➖remove-ground-investigation               | ✅apply  |
      | change-layer-soil-type | 🥞️change-layer-soil-type | ✅apply  |
      | change-layer-depth-top | 🥞️change-layer-depth-top | ✅apply  |
      | change-layer-depth-bottom | 🥞️change-layer-depth-bottom | ✅apply  |
      | change-layer-gamma | 🥞️change-layer-gamma | ✅apply  |
      | change-layer-gamma-prime | 🥞️change-layer-gamma-prime | ✅apply  |
      | change-layer-cohesion-effective | 🥞️change-layer-cohesion-effective | ✅apply  |
      | change-layer-cohesion-undrained | 🥞️change-layer-cohesion-undrained | ✅apply  |
      | change-layer-poisson-ratio | 🥞️change-layer-poisson-ratio | ✅apply  |
      | change-layer-cpt-qc | 🥞️change-layer-cpt-qc | ✅apply  |
      | change-layer-spt-n | 🥞️change-layer-spt-n | ✅apply  |
      | change-footing-length | 🏛️change-footing-length | ✅apply  |
      | change-footing-base-incl-deg | 🏛️change-footing-base-incl-deg | ✅apply  |
      | change-footing-settlement-limit | 🏛️change-footing-settlement-limit | ✅apply  |
      | change-load-case-design-situation | 🏛️change-load-case-design-situation | ✅apply  |
      | change-load-case-vertical-perm | 🏛️change-load-case-vertical-perm | ✅apply  |
      | change-load-case-vertical-var | 🏛️change-load-case-vertical-var | ✅apply  |
      | change-load-case-horiz-permanent | 🏛️change-load-case-horiz-permanent | ✅apply  |
      | change-load-case-horiz-variable | 🏛️change-load-case-horiz-variable | ✅apply  |
      | change-load-case-moment-permanent | 🏛️change-load-case-moment-permanent | ✅apply  |
      | change-load-case-moment-variable | 🏛️change-load-case-moment-variable | ✅apply  |
      | change-pile-pile-type | 🔄️change-pile-pile-type | ✅apply  |
      | change-pile-diameter | 🔄️change-pile-diameter | ✅apply  |
      | change-pile-alpha-s | 🔄️change-pile-alpha-s | ✅apply  |
      | change-pile-unit-shaft-resistance | 🔄️change-pile-unit-shaft-resistance | ✅apply  |
      | change-pile-unit-base-resistance | 🔄️change-pile-unit-base-resistance | ✅apply  |
      | change-pile-compression-permanent | 🔄️change-pile-compression-permanent | ✅apply  |
      | change-pile-compression-variable | 🔄️change-pile-compression-variable | ✅apply  |
      | change-pile-tension-permanent | 🔄️change-pile-tension-permanent | ✅apply  |
      | change-pile-tension-variable | 🔄️change-pile-tension-variable | ✅apply  |
      | change-profile-shaft-resistance | 🔄️change-profile-shaft-resistance | ✅apply  |
      | change-profile-base-resistance | 🔄️change-profile-base-resistance | ✅apply  |
      | change-wall-height | 🗄️change-wall-height | ✅apply  |
      | change-wall-embedment | 🗄️change-wall-embedment | ✅apply  |
      | change-wall-stem-thickness | 🗄️change-wall-stem-thickness | ✅apply  |
      | change-wall-backfill-phi-deg | 🗄️change-wall-backfill-phi-deg | ✅apply  |
      | change-wall-backfill-gamma | 🗄️change-wall-backfill-gamma | ✅apply  |
      | change-wall-wall-friction-deg | 🗄️change-wall-wall-friction-deg | ✅apply  |
      | change-wall-earth-pressure-mode | 🗄️change-wall-earth-pressure-mode | ✅apply  |
      | change-wall-wall-movement | 🗄️change-wall-wall-movement | ✅apply  |
      | change-wall-ocr | 🗄️change-wall-ocr | ✅apply  |
      | change-wall-concrete-gamma | 🗄️change-wall-concrete-gamma | ✅apply  |
      | change-wall-surcharge | 🗄️change-wall-surcharge | ✅apply  |
      | change-wall-vertical-permanent | 🗄️change-wall-vertical-permanent | ✅apply  |
      | change-wall-horizontal-permanent | 🗄️change-wall-horizontal-permanent | ✅apply  |
      | change-slope-height | ⛰️change-slope-height | ✅apply  |
      | change-slope-length | ⛰️change-slope-length | ✅apply  |
      | change-slope-governing-layer-id | ⛰️change-slope-governing-layer-id | ✅apply  |
      | change-slope-circle-centre-x | ⛰️change-slope-circle-centre-x | ✅apply  |
      | change-slope-circle-centre-z | ⛰️change-slope-circle-centre-z | ✅apply  |
      | change-slope-circle-radius | ⛰️change-slope-circle-radius | ✅apply  |
      | change-uplift-perm-stabilizing | ⬆️change-uplift-perm-stabilizing | ✅apply  |
      | change-uplift-perm-destabilizing | ⬆️change-uplift-perm-destabilizing | ✅apply  |
      | change-uplift-var-destabilizing | ⬆️change-uplift-var-destabilizing | ✅apply  |
      | change-uplift-pore-pressure | ⬆️change-uplift-pore-pressure | ✅apply  |
      | change-uplift-total-stress | ⬆️change-uplift-total-stress | ✅apply  |
      | change-anchor-wall-id | ⚓️change-anchor-wall-id | ✅apply  |
      | change-anchor-tendon | ⚓️change-anchor-tendon | ✅apply  |
      | change-anchor-head-depth | ⚓️change-anchor-head-depth | ✅apply  |
      | change-anchor-inclination-deg | ⚓️change-anchor-inclination-deg | ✅apply  |
      | change-anchor-free-length | ⚓️change-anchor-free-length | ✅apply  |
      | change-anchor-fixed-length | ⚓️change-anchor-fixed-length | ✅apply  |
      | change-anchor-fixed-diameter | ⚓️change-anchor-fixed-diameter | ✅apply  |
      | change-anchor-bond-stress | ⚓️change-anchor-bond-stress | ✅apply  |
      | change-anchor-design-force | ⚓️change-anchor-design-force | ✅apply  |
      | change-anchor-tendon-yield-force | ⚓️change-anchor-tendon-yield-force | ✅apply  |
      | change-anchor-min-free-length | ⚓️change-anchor-min-free-length | ✅apply  |
      | change-anchor-test-factor | ⚓️change-anchor-test-factor | ✅apply  |
      | change-anchor-test-pull-out-res | ⚓️change-anchor-test-pull-out-res | ✅apply  |
      | change-emb-wall-kind | 🧱️change-emb-wall-kind | ✅apply  |
      | change-emb-wall-retained-height | 🧱️change-emb-wall-retained-height | ✅apply  |
      | change-emb-wall-embedment | 🧱️change-emb-wall-embedment | ✅apply  |
      | change-emb-wall-prop-depth | 🧱️change-emb-wall-prop-depth | ✅apply  |
      | change-emb-wall-thickness | 🧱️change-emb-wall-thickness | ✅apply  |
      | change-emb-wall-backfill-gamma | 🧱️change-emb-wall-backfill-gamma | ✅apply  |
      | change-emb-wall-phi-deg | 🧱️change-emb-wall-phi-deg | ✅apply  |
      | change-emb-wall-delta-active-deg | 🧱️change-emb-wall-delta-active-deg | ✅apply  |
      | change-emb-wall-delta-passive-deg | 🧱️change-emb-wall-delta-passive-deg | ✅apply  |
      | change-emb-wall-surcharge | 🧱️change-emb-wall-surcharge | ✅apply  |
      | change-emb-wall-undr-strength | 🧱️change-emb-wall-undr-strength | ✅apply  |
      | change-pile-group-pile-id | 🏗️change-pile-group-pile-id | ✅apply  |
      | change-pile-group-rows | 🏗️change-pile-group-rows | ✅apply  |
      | change-pile-group-columns | 🏗️change-pile-group-columns | ✅apply  |
      | change-pile-group-spacing | 🏗️change-pile-group-spacing | ✅apply  |
      | change-pile-group-cap-vertical | 🏗️change-pile-group-cap-vertical | ✅apply  |
      | change-pile-group-cap-moment-x | 🏗️change-pile-group-cap-moment-x | ✅apply  |
      | change-pile-group-cap-moment-y | 🏗️change-pile-group-cap-moment-y | ✅apply  |
      | change-pile-group-single-res | 🏗️change-pile-group-single-res | ✅apply  |
      | change-pile-group-shaft-coh-sum | 🏗️change-pile-group-shaft-coh-sum | ✅apply  |
      | change-pile-group-base-cohesion | 🏗️change-pile-group-base-cohesion | ✅apply  |
      | change-lab-test-layer-id | 🧪️change-lab-test-layer-id | ✅apply  |
      | change-lab-test-kind | 🧪️change-lab-test-kind | ✅apply  |
      | change-lab-test-void-ratio | 🧪️change-lab-test-void-ratio | ✅apply  |
      | change-lab-test-preload-stress | 🧪️change-lab-test-preload-stress | ✅apply  |
      | change-lab-test-drainage-height | 🧪️change-lab-test-drainage-height | ✅apply  |
      | change-lab-test-consol-time | 🧪️change-lab-test-consol-time | ✅apply  |
      | change-lab-point-normal-stress | 🧪️change-lab-point-normal-stress | ✅apply  |
      | change-lab-point-shear-stress | 🧪️change-lab-point-shear-stress | ✅apply  |
      | change-water-layer-id | 💧️change-water-layer-id | ✅apply  |
      | change-water-piezometric-depth | 💧️change-water-piezometric-depth | ✅apply  |
      | change-water-permeability | 💧️change-water-permeability | ✅apply  |
      | change-water-head-difference | 💧️change-water-head-difference | ✅apply  |
      | change-water-flow-length | 💧️change-water-flow-length | ✅apply  |
      | insert-case | ➕️insert-case | ✅apply  |
      | remove-case | ➖️remove-case | ✅apply  |
      | change-case-number | 🔢️change-case-number | ✅apply  |
      | change-case-text | 🔤️change-case-text | ✅apply  |
      | change-case-label-en | 🏷️change-case-label-en | ✅apply  |
      | change-case-label-de | 🏷️change-case-label-de | ✅apply  |
      | change-case-entry-id | 📇️change-case-entry-id | ✅apply  |
      | change-slope-width | ⛰️change-slope-width | ✅apply  |
      | change-pile-bending-stiffness-ei | 🔄️change-pile-bending-stiffness-ei | ✅apply  |
      | change-pile-horiz-bed-modulus | 🔄️change-pile-horiz-bed-modulus | ✅apply  |
      | change-pile-lateral-head-load | 🔄️change-pile-lateral-head-load | ✅apply  |
      | change-wall-seismic-kh | 🗄️change-wall-seismic-kh | ✅apply  |
      | change-wall-seismic-kv | 🗄️change-wall-seismic-kv | ✅apply  |
      | change-pile-elastic-modulus | 🔄️change-pile-elastic-modulus | ✅apply  |
      | change-pile-settling-thickness | 🔄️change-pile-settling-thickness | ✅apply  |
      | change-pile-neg-friction-beta | 🔄️change-pile-neg-friction-beta | ✅apply  |
      | change-pile-dynamic-test-count | 🔄️change-pile-dynamic-test-count | ✅apply  |
      | change-pile-dynamic-res-mean | 🔄️change-pile-dynamic-res-mean | ✅apply  |
      | change-pile-dynamic-res-min | 🔄️change-pile-dynamic-res-min | ✅apply  |
      | change-pile-hammer-weight | 🔄️change-pile-hammer-weight | ✅apply  |
      | change-pile-drop-height | 🔄️change-pile-drop-height | ✅apply  |
      | change-pile-set-per-blow | 🔄️change-pile-set-per-blow | ✅apply  |
      | change-pile-elastic-compression | 🔄️change-pile-elastic-compression | ✅apply  |
      | change-pile-hammer-efficiency | 🔄️change-pile-hammer-efficiency | ✅apply  |
      | change-wall-reinforcement-spacing | 🗄️change-wall-reinforcement-spacing | ✅apply  |
      | change-wall-reinforcement-width | 🗄️change-wall-reinforcement-width | ✅apply  |
      | change-wall-rebar-friction | 🗄️change-wall-rebar-friction | ✅apply  |
      | change-lab-test-compression-index | 🧪️change-lab-test-compression-index | ✅apply  |
      | change-lab-test-swelling-index | 🧪️change-lab-test-swelling-index | ✅apply  |
      | change-lab-test-stress-increment | 🧪️change-lab-test-stress-increment | ✅apply  |
      | change-lab-test-sec-compr-index | 🧪️change-lab-test-sec-compr-index | ✅apply  |
      | change-lab-test-design-life | 🧪️change-lab-test-design-life | ✅apply  |
      | change-water-well-radius | 💧️change-water-well-radius | ✅apply  |
      | change-water-well-water-depth | 💧️change-water-well-water-depth | ✅apply  |
      | change-water-influence-radius | 💧️change-water-influence-radius | ✅apply  |
      | change-water-well-capacity | 💧️change-water-well-capacity | ✅apply  |
      | change-pile-concrete-class | 🔄️change-pile-concrete-class | ✅apply  |
      | change-ground-investigation-position-x | 🕳️change-ground-investigation-position-x | ✅apply  |
      | change-ground-investigation-position-y | 🕳️change-ground-investigation-position-y | ✅apply  |
      | change-ground-investigation-level | 🕳️change-ground-investigation-level | ✅apply  |
      | change-ground-investigation-depth | 🕳️change-ground-investigation-depth | ✅apply  |
      | change-ground-investigation-kind | 🕳️change-ground-investigation-kind | ✅apply  |
      | change-ground-reading-depth | 📈️change-ground-reading-depth | ✅apply  |
      | change-ground-reading-measurement | 📈️change-ground-reading-measurement | ✅apply  |
      | change-characteristic-basis-layer-id | 🧮️change-characteristic-basis-layer-id | ✅apply  |
      | change-characteristic-basis-parameter | 🧮️change-characteristic-basis-parameter | ✅apply  |
      | change-characteristic-basis-basis | 🧮️change-characteristic-basis-basis | ✅apply  |
      | change-layer-cpt-nkt | 🥞️change-layer-cpt-nkt | ✅apply  |
      | change-structure-id | 🏗️change-structure-id | ✅apply  |
      | insert-load-case | ➕️insert-load-case | ✅apply  |
      | remove-load-case | ➖️remove-load-case | ✅apply  |
      | insert-test-profile | ➕️insert-test-profile | ✅apply  |
      | remove-test-profile | ➖️remove-test-profile | ✅apply  |
      | insert-anchor-test | ➕️insert-anchor-test | ✅apply  |
      | remove-anchor-test | ➖️remove-anchor-test | ✅apply  |
      | insert-lab-point | ➕️insert-lab-point | ✅apply  |
      | remove-lab-point | ➖️remove-lab-point | ✅apply  |
      | insert-ground-investigation-reading | ➕️insert-ground-investigation-reading | ✅apply  |
      | remove-ground-investigation-reading | ➖️remove-ground-investigation-reading | ✅apply  |

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
      | id                              | dir                               | fixture |
      | change-annex                    | 🌍️change-annex                    | ✅apply  |
      | change-geotechnical-category    | 🗂️change-geotechnical-category    | ✅apply  |
      | change-design-situation         | 📅️change-design-situation         | ✅apply  |
      | change-design-approach          | 🧭️change-design-approach          | ✅apply  |
      | change-groundwater-level        | 💧change-groundwater-level         | ✅apply  |
      | change-investigation-depth      | 🔎️change-investigation-depth      | ✅apply  |
      | change-footing-width            | ↔️change-footing-width            | ✅apply  |
      | change-footing-embedment        | ⬇️change-footing-embedment        | ✅apply  |
      | change-pile-length              | 📏️change-pile-length              | ✅apply  |
      | change-pile-count               | 🔢change-pile-count                | ✅apply  |
      | change-wall-base-width          | 🧱change-wall-base-width           | ✅apply  |
      | change-slope-angle              | ⛰️change-slope-angle              | ✅apply  |
      | change-placement-x | 📍️change-placement-x | ✅apply  |
      | change-placement-y | 📍️change-placement-y | ✅apply  |
      | change-placement-z | 📍️change-placement-z | ✅apply  |
      | change-placement-yaw | 📍️change-placement-yaw | ✅apply  |
      | insert-placement | 📍️insert-placement | ✅apply  |
      | remove-placement | 📍️remove-placement | ✅apply  |
      | change-layer-phi-prime          | 📐️change-layer-phi-prime          | ✅apply  |
      | change-layer-oedometric-modulus | 🌀️change-layer-oedometric-modulus | ✅apply  |
      | insert-layer                    | ➕️insert-layer                    | ✅apply  |
      | remove-layer                    | ➖️remove-layer                    | ✅apply  |
      | insert-footing                  | ➕insert-footing                   | ✅apply  |
      | remove-footing                  | ➖remove-footing                   | ✅apply  |
      | insert-pile                     | 📥insert-pile                      | ✅apply  |
      | remove-pile                     | 📤remove-pile                      | ✅apply  |
      | insert-retaining-wall           | ➕insert-retaining-wall            | ✅apply  |
      | remove-retaining-wall           | ➖remove-retaining-wall            | ✅apply  |
      | insert-slope                    | ➕insert-slope                     | ✅apply  |
      | remove-slope                    | ➖remove-slope                     | ✅apply  |
      | insert-uplift-case              | ➕insert-uplift-case               | ✅apply  |
      | remove-uplift-case              | ➖remove-uplift-case               | ✅apply  |
      | insert-anchor                   | ➕insert-anchor                    | ✅apply  |
      | remove-anchor                   | ➖remove-anchor                    | ✅apply  |
      | insert-embedded-wall            | ➕insert-embedded-wall             | ✅apply  |
      | remove-embedded-wall            | ➖remove-embedded-wall             | ✅apply  |
      | insert-pile-group               | ➕insert-pile-group                | ✅apply  |
      | remove-pile-group               | ➖remove-pile-group                | ✅apply  |
      | insert-lab-test                 | ➕insert-lab-test                  | ✅apply  |
      | remove-lab-test                 | ➖remove-lab-test                  | ✅apply  |
      | insert-layer-water              | ➕insert-layer-water               | ✅apply  |
      | remove-layer-water              | ➖remove-layer-water               | ✅apply  |
      | insert-characteristic-basis              | ➕insert-characteristic-basis               | ✅apply  |
      | remove-characteristic-basis              | ➖remove-characteristic-basis               | ✅apply  |
      | insert-ground-investigation              | ➕insert-ground-investigation               | ✅apply  |
      | remove-ground-investigation              | ➖remove-ground-investigation               | ✅apply  |
      | change-layer-soil-type | 🥞️change-layer-soil-type | ✅apply  |
      | change-layer-depth-top | 🥞️change-layer-depth-top | ✅apply  |
      | change-layer-depth-bottom | 🥞️change-layer-depth-bottom | ✅apply  |
      | change-layer-gamma | 🥞️change-layer-gamma | ✅apply  |
      | change-layer-gamma-prime | 🥞️change-layer-gamma-prime | ✅apply  |
      | change-layer-cohesion-effective | 🥞️change-layer-cohesion-effective | ✅apply  |
      | change-layer-cohesion-undrained | 🥞️change-layer-cohesion-undrained | ✅apply  |
      | change-layer-poisson-ratio | 🥞️change-layer-poisson-ratio | ✅apply  |
      | change-layer-cpt-qc | 🥞️change-layer-cpt-qc | ✅apply  |
      | change-layer-spt-n | 🥞️change-layer-spt-n | ✅apply  |
      | change-footing-length | 🏛️change-footing-length | ✅apply  |
      | change-footing-base-incl-deg | 🏛️change-footing-base-incl-deg | ✅apply  |
      | change-footing-settlement-limit | 🏛️change-footing-settlement-limit | ✅apply  |
      | change-load-case-design-situation | 🏛️change-load-case-design-situation | ✅apply  |
      | change-load-case-vertical-perm | 🏛️change-load-case-vertical-perm | ✅apply  |
      | change-load-case-vertical-var | 🏛️change-load-case-vertical-var | ✅apply  |
      | change-load-case-horiz-permanent | 🏛️change-load-case-horiz-permanent | ✅apply  |
      | change-load-case-horiz-variable | 🏛️change-load-case-horiz-variable | ✅apply  |
      | change-load-case-moment-permanent | 🏛️change-load-case-moment-permanent | ✅apply  |
      | change-load-case-moment-variable | 🏛️change-load-case-moment-variable | ✅apply  |
      | change-pile-pile-type | 🔄️change-pile-pile-type | ✅apply  |
      | change-pile-diameter | 🔄️change-pile-diameter | ✅apply  |
      | change-pile-alpha-s | 🔄️change-pile-alpha-s | ✅apply  |
      | change-pile-unit-shaft-resistance | 🔄️change-pile-unit-shaft-resistance | ✅apply  |
      | change-pile-unit-base-resistance | 🔄️change-pile-unit-base-resistance | ✅apply  |
      | change-pile-compression-permanent | 🔄️change-pile-compression-permanent | ✅apply  |
      | change-pile-compression-variable | 🔄️change-pile-compression-variable | ✅apply  |
      | change-pile-tension-permanent | 🔄️change-pile-tension-permanent | ✅apply  |
      | change-pile-tension-variable | 🔄️change-pile-tension-variable | ✅apply  |
      | change-profile-shaft-resistance | 🔄️change-profile-shaft-resistance | ✅apply  |
      | change-profile-base-resistance | 🔄️change-profile-base-resistance | ✅apply  |
      | change-wall-height | 🗄️change-wall-height | ✅apply  |
      | change-wall-embedment | 🗄️change-wall-embedment | ✅apply  |
      | change-wall-stem-thickness | 🗄️change-wall-stem-thickness | ✅apply  |
      | change-wall-backfill-phi-deg | 🗄️change-wall-backfill-phi-deg | ✅apply  |
      | change-wall-backfill-gamma | 🗄️change-wall-backfill-gamma | ✅apply  |
      | change-wall-wall-friction-deg | 🗄️change-wall-wall-friction-deg | ✅apply  |
      | change-wall-earth-pressure-mode | 🗄️change-wall-earth-pressure-mode | ✅apply  |
      | change-wall-wall-movement | 🗄️change-wall-wall-movement | ✅apply  |
      | change-wall-ocr | 🗄️change-wall-ocr | ✅apply  |
      | change-wall-concrete-gamma | 🗄️change-wall-concrete-gamma | ✅apply  |
      | change-wall-surcharge | 🗄️change-wall-surcharge | ✅apply  |
      | change-wall-vertical-permanent | 🗄️change-wall-vertical-permanent | ✅apply  |
      | change-wall-horizontal-permanent | 🗄️change-wall-horizontal-permanent | ✅apply  |
      | change-slope-height | ⛰️change-slope-height | ✅apply  |
      | change-slope-length | ⛰️change-slope-length | ✅apply  |
      | change-slope-governing-layer-id | ⛰️change-slope-governing-layer-id | ✅apply  |
      | change-slope-circle-centre-x | ⛰️change-slope-circle-centre-x | ✅apply  |
      | change-slope-circle-centre-z | ⛰️change-slope-circle-centre-z | ✅apply  |
      | change-slope-circle-radius | ⛰️change-slope-circle-radius | ✅apply  |
      | change-uplift-perm-stabilizing | ⬆️change-uplift-perm-stabilizing | ✅apply  |
      | change-uplift-perm-destabilizing | ⬆️change-uplift-perm-destabilizing | ✅apply  |
      | change-uplift-var-destabilizing | ⬆️change-uplift-var-destabilizing | ✅apply  |
      | change-uplift-pore-pressure | ⬆️change-uplift-pore-pressure | ✅apply  |
      | change-uplift-total-stress | ⬆️change-uplift-total-stress | ✅apply  |
      | change-anchor-wall-id | ⚓️change-anchor-wall-id | ✅apply  |
      | change-anchor-tendon | ⚓️change-anchor-tendon | ✅apply  |
      | change-anchor-head-depth | ⚓️change-anchor-head-depth | ✅apply  |
      | change-anchor-inclination-deg | ⚓️change-anchor-inclination-deg | ✅apply  |
      | change-anchor-free-length | ⚓️change-anchor-free-length | ✅apply  |
      | change-anchor-fixed-length | ⚓️change-anchor-fixed-length | ✅apply  |
      | change-anchor-fixed-diameter | ⚓️change-anchor-fixed-diameter | ✅apply  |
      | change-anchor-bond-stress | ⚓️change-anchor-bond-stress | ✅apply  |
      | change-anchor-design-force | ⚓️change-anchor-design-force | ✅apply  |
      | change-anchor-tendon-yield-force | ⚓️change-anchor-tendon-yield-force | ✅apply  |
      | change-anchor-min-free-length | ⚓️change-anchor-min-free-length | ✅apply  |
      | change-anchor-test-factor | ⚓️change-anchor-test-factor | ✅apply  |
      | change-anchor-test-pull-out-res | ⚓️change-anchor-test-pull-out-res | ✅apply  |
      | change-emb-wall-kind | 🧱️change-emb-wall-kind | ✅apply  |
      | change-emb-wall-retained-height | 🧱️change-emb-wall-retained-height | ✅apply  |
      | change-emb-wall-embedment | 🧱️change-emb-wall-embedment | ✅apply  |
      | change-emb-wall-prop-depth | 🧱️change-emb-wall-prop-depth | ✅apply  |
      | change-emb-wall-thickness | 🧱️change-emb-wall-thickness | ✅apply  |
      | change-emb-wall-backfill-gamma | 🧱️change-emb-wall-backfill-gamma | ✅apply  |
      | change-emb-wall-phi-deg | 🧱️change-emb-wall-phi-deg | ✅apply  |
      | change-emb-wall-delta-active-deg | 🧱️change-emb-wall-delta-active-deg | ✅apply  |
      | change-emb-wall-delta-passive-deg | 🧱️change-emb-wall-delta-passive-deg | ✅apply  |
      | change-emb-wall-surcharge | 🧱️change-emb-wall-surcharge | ✅apply  |
      | change-emb-wall-undr-strength | 🧱️change-emb-wall-undr-strength | ✅apply  |
      | change-pile-group-pile-id | 🏗️change-pile-group-pile-id | ✅apply  |
      | change-pile-group-rows | 🏗️change-pile-group-rows | ✅apply  |
      | change-pile-group-columns | 🏗️change-pile-group-columns | ✅apply  |
      | change-pile-group-spacing | 🏗️change-pile-group-spacing | ✅apply  |
      | change-pile-group-cap-vertical | 🏗️change-pile-group-cap-vertical | ✅apply  |
      | change-pile-group-cap-moment-x | 🏗️change-pile-group-cap-moment-x | ✅apply  |
      | change-pile-group-cap-moment-y | 🏗️change-pile-group-cap-moment-y | ✅apply  |
      | change-pile-group-single-res | 🏗️change-pile-group-single-res | ✅apply  |
      | change-pile-group-shaft-coh-sum | 🏗️change-pile-group-shaft-coh-sum | ✅apply  |
      | change-pile-group-base-cohesion | 🏗️change-pile-group-base-cohesion | ✅apply  |
      | change-lab-test-layer-id | 🧪️change-lab-test-layer-id | ✅apply  |
      | change-lab-test-kind | 🧪️change-lab-test-kind | ✅apply  |
      | change-lab-test-void-ratio | 🧪️change-lab-test-void-ratio | ✅apply  |
      | change-lab-test-preload-stress | 🧪️change-lab-test-preload-stress | ✅apply  |
      | change-lab-test-drainage-height | 🧪️change-lab-test-drainage-height | ✅apply  |
      | change-lab-test-consol-time | 🧪️change-lab-test-consol-time | ✅apply  |
      | change-lab-point-normal-stress | 🧪️change-lab-point-normal-stress | ✅apply  |
      | change-lab-point-shear-stress | 🧪️change-lab-point-shear-stress | ✅apply  |
      | change-water-layer-id | 💧️change-water-layer-id | ✅apply  |
      | change-water-piezometric-depth | 💧️change-water-piezometric-depth | ✅apply  |
      | change-water-permeability | 💧️change-water-permeability | ✅apply  |
      | change-water-head-difference | 💧️change-water-head-difference | ✅apply  |
      | change-water-flow-length | 💧️change-water-flow-length | ✅apply  |
      | insert-case | ➕️insert-case | ✅apply  |
      | remove-case | ➖️remove-case | ✅apply  |
      | change-case-number | 🔢️change-case-number | ✅apply  |
      | change-case-text | 🔤️change-case-text | ✅apply  |
      | change-case-label-en | 🏷️change-case-label-en | ✅apply  |
      | change-case-label-de | 🏷️change-case-label-de | ✅apply  |
      | change-case-entry-id | 📇️change-case-entry-id | ✅apply  |
      | change-slope-width | ⛰️change-slope-width | ✅apply  |
      | change-pile-bending-stiffness-ei | 🔄️change-pile-bending-stiffness-ei | ✅apply  |
      | change-pile-horiz-bed-modulus | 🔄️change-pile-horiz-bed-modulus | ✅apply  |
      | change-pile-lateral-head-load | 🔄️change-pile-lateral-head-load | ✅apply  |
      | change-wall-seismic-kh | 🗄️change-wall-seismic-kh | ✅apply  |
      | change-wall-seismic-kv | 🗄️change-wall-seismic-kv | ✅apply  |
      | change-pile-elastic-modulus | 🔄️change-pile-elastic-modulus | ✅apply  |
      | change-pile-settling-thickness | 🔄️change-pile-settling-thickness | ✅apply  |
      | change-pile-neg-friction-beta | 🔄️change-pile-neg-friction-beta | ✅apply  |
      | change-pile-dynamic-test-count | 🔄️change-pile-dynamic-test-count | ✅apply  |
      | change-pile-dynamic-res-mean | 🔄️change-pile-dynamic-res-mean | ✅apply  |
      | change-pile-dynamic-res-min | 🔄️change-pile-dynamic-res-min | ✅apply  |
      | change-pile-hammer-weight | 🔄️change-pile-hammer-weight | ✅apply  |
      | change-pile-drop-height | 🔄️change-pile-drop-height | ✅apply  |
      | change-pile-set-per-blow | 🔄️change-pile-set-per-blow | ✅apply  |
      | change-pile-elastic-compression | 🔄️change-pile-elastic-compression | ✅apply  |
      | change-pile-hammer-efficiency | 🔄️change-pile-hammer-efficiency | ✅apply  |
      | change-wall-reinforcement-spacing | 🗄️change-wall-reinforcement-spacing | ✅apply  |
      | change-wall-reinforcement-width | 🗄️change-wall-reinforcement-width | ✅apply  |
      | change-wall-rebar-friction | 🗄️change-wall-rebar-friction | ✅apply  |
      | change-lab-test-compression-index | 🧪️change-lab-test-compression-index | ✅apply  |
      | change-lab-test-swelling-index | 🧪️change-lab-test-swelling-index | ✅apply  |
      | change-lab-test-stress-increment | 🧪️change-lab-test-stress-increment | ✅apply  |
      | change-lab-test-sec-compr-index | 🧪️change-lab-test-sec-compr-index | ✅apply  |
      | change-lab-test-design-life | 🧪️change-lab-test-design-life | ✅apply  |
      | change-water-well-radius | 💧️change-water-well-radius | ✅apply  |
      | change-water-well-water-depth | 💧️change-water-well-water-depth | ✅apply  |
      | change-water-influence-radius | 💧️change-water-influence-radius | ✅apply  |
      | change-water-well-capacity | 💧️change-water-well-capacity | ✅apply  |
      | change-pile-concrete-class | 🔄️change-pile-concrete-class | ✅apply  |
      | change-ground-investigation-position-x | 🕳️change-ground-investigation-position-x | ✅apply  |
      | change-ground-investigation-position-y | 🕳️change-ground-investigation-position-y | ✅apply  |
      | change-ground-investigation-level | 🕳️change-ground-investigation-level | ✅apply  |
      | change-ground-investigation-depth | 🕳️change-ground-investigation-depth | ✅apply  |
      | change-ground-investigation-kind | 🕳️change-ground-investigation-kind | ✅apply  |
      | change-ground-reading-depth | 📈️change-ground-reading-depth | ✅apply  |
      | change-ground-reading-measurement | 📈️change-ground-reading-measurement | ✅apply  |
      | change-characteristic-basis-layer-id | 🧮️change-characteristic-basis-layer-id | ✅apply  |
      | change-characteristic-basis-parameter | 🧮️change-characteristic-basis-parameter | ✅apply  |
      | change-characteristic-basis-basis | 🧮️change-characteristic-basis-basis | ✅apply  |
      | change-layer-cpt-nkt | 🥞️change-layer-cpt-nkt | ✅apply  |
      | change-structure-id | 🏗️change-structure-id | ✅apply  |
      | insert-load-case | ➕️insert-load-case | ✅apply  |
      | remove-load-case | ➖️remove-load-case | ✅apply  |
      | insert-test-profile | ➕️insert-test-profile | ✅apply  |
      | remove-test-profile | ➖️remove-test-profile | ✅apply  |
      | insert-anchor-test | ➕️insert-anchor-test | ✅apply  |
      | remove-anchor-test | ➖️remove-anchor-test | ✅apply  |
      | insert-lab-point | ➕️insert-lab-point | ✅apply  |
      | remove-lab-point | ➖️remove-lab-point | ✅apply  |
      | insert-ground-investigation-reading | ➕️insert-ground-investigation-reading | ✅apply  |
      | remove-ground-investigation-reading | ➖️remove-ground-investigation-reading | ✅apply  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1997 document from the parsed carrier
    Given the real committed text artifact asset://🎬️demo/🗣️.dsl.semio
    And its committed binary twin asset://🎬️demo/📦️.pack.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
