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
  cover every kind of the current vocabulary (23 `change`, 2 `insert`, 2 `remove`, 1 `reorder`) on a liquid-retaining RC wall with a post-installed anchor; each vector's after-snapshot and
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
  `📖️component.grammar.semio` is the repository-wide `payload = OCTET+` placeholder, so the two sides are compared
  at the envelope preamble, the ordered lines and the digest and length of what each re-emitted. The Rust side additionally proves it PARSED the document: the committed binary
  twin must decode to the same document as the text, through a separately written codec.

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
      | id                            | dir                             | fixture |
      | change-annex                  | 🌍️change-annex                  | ✅apply  |
      | change-title                  | 🏷️change-title                  | ✅apply  |
      | change-design-working-life    | 📅️change-design-working-life    | ✅apply  |
      | change-delta-c-dev            | 📏️change-delta-c-dev            | ✅apply  |
      | change-cement-type            | 🧪change-cement-type             | ✅apply  |
      | change-concrete-f-ck          | 🧱change-concrete-f-ck           | ✅apply  |
      | change-reinforcement-f-yk     | 🔩change-reinforcement-f-yk      | ✅apply  |
      | insert-member                 | ➕️insert-member                 | ✅apply  |
      | remove-member                 | ➖️remove-member                 | ✅apply  |
      | reorder-members               | 🔀️reorder-members               | ✅apply  |
      | change-member-width           | ↔️change-member-width           | ✅apply  |
      | change-member-height          | ↕️change-member-height          | ✅apply  |
      | change-member-effective-depth | 📐️change-member-effective-depth | ✅apply  |
      | change-member-cover           | 🛡️change-member-cover           | ✅apply  |
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
      | change-anchor-as              | 🧷change-anchor-a-s              | ✅apply  |

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
      | id                            | dir                             | fixture |
      | change-annex                  | 🌍️change-annex                  | ✅apply  |
      | change-title                  | 🏷️change-title                  | ✅apply  |
      | change-design-working-life    | 📅️change-design-working-life    | ✅apply  |
      | change-delta-c-dev            | 📏️change-delta-c-dev            | ✅apply  |
      | change-cement-type            | 🧪change-cement-type             | ✅apply  |
      | change-concrete-f-ck          | 🧱change-concrete-f-ck           | ✅apply  |
      | change-reinforcement-f-yk     | 🔩change-reinforcement-f-yk      | ✅apply  |
      | insert-member                 | ➕️insert-member                 | ✅apply  |
      | remove-member                 | ➖️remove-member                 | ✅apply  |
      | reorder-members               | 🔀️reorder-members               | ✅apply  |
      | change-member-width           | ↔️change-member-width           | ✅apply  |
      | change-member-height          | ↕️change-member-height          | ✅apply  |
      | change-member-effective-depth | 📐️change-member-effective-depth | ✅apply  |
      | change-member-cover           | 🛡️change-member-cover           | ✅apply  |
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
      | change-anchor-as              | 🧷change-anchor-a-s              | ✅apply  |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1992 document from the parsed carrier
    Given the real committed text artifact asset://🛢️liquid-retaining-fem-anchor/🛢️liquid-retaining-fem-anchor/🗣️.dsl.semio
    And its committed binary twin asset://🛢️liquid-retaining-fem-anchor/🎒️.pack.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
