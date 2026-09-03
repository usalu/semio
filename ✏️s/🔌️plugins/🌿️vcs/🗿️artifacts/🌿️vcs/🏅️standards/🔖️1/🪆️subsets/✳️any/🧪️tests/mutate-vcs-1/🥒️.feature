@capability-vcs-1-mutate
@oracle-vcs-1-python-independent
@comparison-ordered-json-v1
@mutations-vcs-1-any
Feature: Apply every typed VCS checkpoint mutation to its committed specification vectors and against an independent Python implementation
  `s.vcs.vcs` is a semio-NATIVE review-checkpoint document: its two wire forms are
  `.vcs.dsl.semio` and `.vcs.pack.semio`, grammars this repository defines and nobody else reads, so
  no reference LIBRARY exists. The second producer a differential comparison needs is therefore a
  second IMPLEMENTATION, and `🐍️component.py` beside this file is it: all six kinds of this
  vocabulary, written in Python from this subset's own committed
  `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json` and each mutation's own payload
  schema, and from
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
  `change`/`rename`/`add`/`remove` verb entries. It imports nothing from the Rust it judges and
  transliterates none of it. Each of the six kinds carries an independently handcrafted `(before,
  mutation, after, diff, outcome)` specification fixture under its own triad leaf's `🧪️tests/`
  directory, and this feature re-exercises those SAME committed bytes end to end through BOTH
  `apply_vcs_mutation_reporting` and the Python reference. The no-oracle decision this replaces
  (`vcs-1-checkpoint-mutation-semantics`) is narrowed to an empty `capabilities` list rather than
  deleted, because its own investigation remains the honest record of what was checked.

  Both implementations now read the SAME committed bytes: every `(before, mutation, after, diff,
  outcome)` path is a declared `asset://` fixture rather than an `include_str!`-only literal, so the
  plan pins its digest and a Python reference can resolve it.

  What distinguishes this vocabulary from every 🗄️stdio one is what it does NOT declare. There is
  no `no-mutation` and no `set-snapshot`: whole-document replace is banned vocabulary in this
  taxonomy and reaches the store through its non-history `reset` path instead, so the two kinds
  every raster, document and archive catalog carries genuinely do not exist here and the catalog
  says six, not eight. What it does declare is one kind per artifact-lane field of a checkpoint —
  the title, the integer counter, the freeform notes and the workflow status word are scalar and
  take a `change`/`rename` verb, while `tags` is the one COLLECTION and takes the `add`/`remove`
  pair. The `moves` column below names, per kind, the single projection member that kind is
  allowed to touch; the subject handler asserts both halves of that — the named member moved AND
  every other member is byte-identical — so a mutation that quietly rewrote a neighbouring field
  fails on the neighbour rather than passing on its own target.

  `tags` is set-like in meaning but ORDERED on the wire, which is why `add-tag`'s fixture appends
  `urgent` AFTER the `review` the base already carried and `remove-tag`'s fixture detaches that
  same pre-existing member: an implementation that re-sorted, de-duplicated or rebuilt the list
  fails both, and an inverse that re-appended rather than restoring position fails the second.

  `mutate-<kind>`/`inverse-<kind>` now dispatch BOTH an oracle role (the Python implementation,
  reached through this plugin's `oracleHostPackages` entry) and a subject role (this repository's own
  `apply_vcs_mutation_reporting`, unaffected by this change): `mutate-<kind>` compares the applied
  snapshot with the committed after-snapshot and the reported diagnostic codes with the committed
  `🎯️outcome/🔣️.json`, and `inverse-<kind>` compares the undone snapshot with the committed
  before-snapshot — each side in role, then the two compared. A handler that merely ran the mutation
  and returned would report a pass having checked nothing.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed diff asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🔺️diff/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    And the committed before-snapshot, mutation and outcome fixtures for the <id> kind
    When <id> is applied through apply_vcs_mutation_reporting
      """
      {"kind": "<id>", "moves": "<moves>"}
      """
    Then the resulting snapshot matches the committed after-snapshot, only <moves> moved, the reported diagnostics match the committed outcome, and the two implementations agree
    Examples:
      | id             | dir              | fixture                    | moves   |
      | rename-vcs     | ✏️rename-vcs      | retitles-the-document       | title   |
      | change-counter | 🔢change-counter  | sets-counter-to-seven       | counter |
      | change-notes   | 📝change-notes    | rewrites-the-notes          | notes   |
      | change-status  | 🚦change-status   | draft-to-review             | status  |
      | add-tag        | 🏷️add-tag         | appends-urgent-tag          | tags    |
      | remove-tag     | 🗑️remove-tag      | detaches-the-review-tag     | tags    |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🎯️outcome/🔣️.json
    And the committed before-snapshot and mutation fixtures for the <id> kind
    When <id> is applied and then its own computed inverse steps are applied
      """
      {"kind": "<id>", "moves": "<moves>"}
      """
    Then the snapshot equals the committed before-snapshot again, member for member, and both implementations agree
    Examples:
      | id             | dir              | fixture                    | moves   |
      | rename-vcs     | ✏️rename-vcs      | retitles-the-document       | title   |
      | change-counter | 🔢change-counter  | sets-counter-to-seven       | counter |
      | change-notes   | 📝change-notes    | rewrites-the-notes          | notes   |
      | change-status  | 🚦change-status   | draft-to-review             | status  |
      | add-tag        | 🏷️add-tag         | appends-urgent-tag          | tags    |
      | remove-tag     | 🗑️remove-tag      | detaches-the-review-tag     | tags    |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Read the real committed checkpoint through its own DSL carrier and print it back
    Given the real committed text artifact asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When the artifact is parsed, printed back to `.vcs.dsl.semio` and parsed again
    Then every decoding agrees on the same checkpoint — "VCS Demo" at counter 2, status draft, tags alpha then beta — and the printed text reproduces the committed file byte for byte
