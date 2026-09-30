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
  declared `shared://` fixture, so neither side holds a transcription that could drift. The 28 vectors cover
  every kind of the current vocabulary (23 `change`, 2 `insert`, 2 `remove`, 1 `reorder`) on a liquid-retaining RC wall with a post-installed anchor; each vector's after-snapshot and diff
  were written by production dispatch and its mutation is the canonical Rust wire. Each side asserts the same laws
  in role — the applied document must BE the committed after-snapshot, an `applied` vector must move the document,
  and the mutation followed by its OWN computed inverse must restore the before-snapshot exactly, list position
  included. `parity` adds that two implementations, in two languages, reach the same document.

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
      | id                            | dir                             | fixture            |
      | change-annex                  | 🌍️change-annex                  | ✏️to-en            |
      | change-title                  | 🏷️change-title                  | ✏️to-liquid        |
      | change-design-working-life    | 📅️change-design-working-life    | ✏️to-100           |
      | change-delta-c-dev            | 📏️change-delta-c-dev            | ✏️to-0-015         |
      | change-cement-type            | 🧪change-cement-type             | ✏️to-s             |
      | change-concrete-f-ck          | 🧱change-concrete-f-ck           | ✏️to-45000000      |
      | change-reinforcement-f-yk     | 🔩change-reinforcement-f-yk      | ✏️to-550000000     |
      | insert-member                 | ➕️insert-member                 | ➕️inserts-member   |
      | remove-member                 | ➖️remove-member                 | ➖️removes-member   |
      | reorder-members               | 🔀️reorder-members               | 🔀️reorders-members |
      | change-member-width           | ↔️change-member-width           | ✏️to-0-42          |
      | change-member-height          | ↕️change-member-height          | ✏️to-0-6           |
      | change-member-effective-depth | 📐️change-member-effective-depth | ✏️to-0             |
      | change-member-cover           | 🛡️change-member-cover           | ✏️to-0-05          |
      | change-member-exposure        | 🌦change-member-exposure         | ✏️to-xd1           |
      | change-member-span            | 🌉️change-member-span            | ✏️to-8-25          |
      | change-member-stirrup-spacing | 🪢change-member-stirrup-spacing  | ✏️to-0-2           |
      | change-member-axis-distance   | 🔥change-member-axis-distance    | ✏️to-0-045         |
      | change-member-fire-rating     | 🔥️change-member-fire-rating     | ✏️to-r120          |
      | change-bar-layer-count        | #️⃣change-bar-layer-count       | ✏️to-8             |
      | change-bar-layer-diameter     | ⭕change-bar-layer-diameter      | ✏️to-0-02          |
      | change-action-mk              | ⤴️change-action-mk              | ✏️to-125000        |
      | change-action-nk              | 🏋️change-action-nk              | ✏️to-37500         |
      | change-action-vk              | ↘️change-action-vk              | ✏️to-62500         |
      | insert-anchor                 | ⚓️insert-anchor                 | ➕️inserts-anchor   |
      | remove-anchor                 | 🗑️remove-anchor                 | ➖️removes-anchor   |
      | change-anchor-h-ef            | 📍change-anchor-h-ef             | ✏️to-0-125         |
      | change-anchor-as              | 🧷change-anchor-a-s              | ✏️to-0-00015       |

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
      | id                            | dir                             | fixture            |
      | change-annex                  | 🌍️change-annex                  | ✏️to-en            |
      | change-title                  | 🏷️change-title                  | ✏️to-liquid        |
      | change-design-working-life    | 📅️change-design-working-life    | ✏️to-100           |
      | change-delta-c-dev            | 📏️change-delta-c-dev            | ✏️to-0-015         |
      | change-cement-type            | 🧪change-cement-type             | ✏️to-s             |
      | change-concrete-f-ck          | 🧱change-concrete-f-ck           | ✏️to-45000000      |
      | change-reinforcement-f-yk     | 🔩change-reinforcement-f-yk      | ✏️to-550000000     |
      | insert-member                 | ➕️insert-member                 | ➕️inserts-member   |
      | remove-member                 | ➖️remove-member                 | ➖️removes-member   |
      | reorder-members               | 🔀️reorder-members               | 🔀️reorders-members |
      | change-member-width           | ↔️change-member-width           | ✏️to-0-42          |
      | change-member-height          | ↕️change-member-height          | ✏️to-0-6           |
      | change-member-effective-depth | 📐️change-member-effective-depth | ✏️to-0             |
      | change-member-cover           | 🛡️change-member-cover           | ✏️to-0-05          |
      | change-member-exposure        | 🌦change-member-exposure         | ✏️to-xd1           |
      | change-member-span            | 🌉️change-member-span            | ✏️to-8-25          |
      | change-member-stirrup-spacing | 🪢change-member-stirrup-spacing  | ✏️to-0-2           |
      | change-member-axis-distance   | 🔥change-member-axis-distance    | ✏️to-0-045         |
      | change-member-fire-rating     | 🔥️change-member-fire-rating     | ✏️to-r120          |
      | change-bar-layer-count        | #️⃣change-bar-layer-count       | ✏️to-8             |
      | change-bar-layer-diameter     | ⭕change-bar-layer-diameter      | ✏️to-0-02          |
      | change-action-mk              | ⤴️change-action-mk              | ✏️to-125000        |
      | change-action-nk              | 🏋️change-action-nk              | ✏️to-37500         |
      | change-action-vk              | ↘️change-action-vk              | ✏️to-62500         |
      | insert-anchor                 | ⚓️insert-anchor                 | ➕️inserts-anchor   |
      | remove-anchor                 | 🗑️remove-anchor                 | ➖️removes-anchor   |
      | change-anchor-h-ef            | 📍change-anchor-h-ef             | ✏️to-0-125         |
      | change-anchor-as              | 🧷change-anchor-a-s              | ✏️to-0-00015       |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed EN 1992 document from the parsed carrier
    Given the real committed text artifact asset://🛢️liquid-retaining-fem-anchor/🛢️liquid-retaining-fem-anchor/🗣️.dsl.semio
    And its committed binary twin asset://🛢️liquid-retaining-fem-anchor/🎒️.pack.semio
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
