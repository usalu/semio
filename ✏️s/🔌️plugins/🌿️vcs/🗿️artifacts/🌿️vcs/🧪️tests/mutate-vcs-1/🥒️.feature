@capability-vcs-1-mutate
@no-oracle-vcs-1-checkpoint-mutation-semantics
@comparison-ordered-json-v1
@mutations-vcs-1-any
Feature: Apply every typed VCS checkpoint mutation to its committed specification vectors
  `s.vcs.vcs` is a semio-NATIVE review-checkpoint document: its two wire forms are
  `.vcs.dsl.semio` and `.vcs.pack.semio`, grammars this repository defines and nobody else reads, so
  no reference LIBRARY is registered — recorded as the `vcs-1-checkpoint-mutation-semantics`
  no-oracle decision in `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`. Each of
  the six kinds carries an independently handcrafted `(before, mutation, after, diff, outcome)`
  specification fixture under its own triad leaf's `🧪️tests/` directory, and this feature re-exercises
  those SAME committed bytes end to end through `apply_vcs_mutation_reporting`.

  ⚠️ THIS NO-ORACLE DECISION IS A DEBT, NOT A VERDICT, and is recorded as one. Declining a third-party
  LIBRARY is a different judgement from declining a SECOND IMPLEMENTATION, and only the first was ever
  made here. `mutate-writer-1` and `mutate-playbook-1` took Python second
  implementations over this same `.dsl.semio` carrier in this wave, so the same is writable for this
  subset from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`, the rules of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` and the
  committed vectors this feature already replays. What blocks it TODAY is stated in the decision and
  is one edit: this case's vectors are not declared as `asset://` fixtures — the `Examples` table
  carries the payloads inline and the adapter reads the committed files through `include_str!` — so
  the plan pins none of their digests and a Python reference cannot read them at all. Separately, `identity-round-trip` would still be refused: this subset's committed
  snapshot text grammar is the repository-wide placeholder `payload = OCTET+`, whose header production
  declares `"schema" SP "stdio.json"` against an artifact whose own first line says otherwise.
  Until that is done, every assertion below still lives in the SUBJECT role, and the ceiling is the
  one this decision has always had: no second producer runs beside it, so a mistake shared by the
  handcrafted vector and the production code passes unseen.

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

  Because this case records a no-oracle decision the runner executes NO oracle role, so every
  assertion lives inside the subject handler: `mutate-<kind>` compares the applied snapshot with
  the committed after-snapshot and the reported diagnostic codes with the committed
  `🎯️outcome/🔣️.json`, and `inverse-<kind>` compares the undone snapshot with the
  committed before-snapshot. A handler that merely ran the mutation and returned would report a
  pass having checked nothing.

  @id-mutate
  @level-exhaustive
  @mode-conformance
  Scenario Outline: Apply <id> to its committed before-snapshot fixture
    Given the committed before-snapshot, mutation and outcome fixtures for the <id> kind
    When <id> is applied through apply_vcs_mutation_reporting
      """
      {"kind": "<id>", "moves": "<moves>"}
      """
    Then the resulting snapshot matches the committed after-snapshot, only <moves> moved, and the reported diagnostics match the committed outcome
    Examples:
      | id             | moves   |
      | rename-vcs     | title   |
      | change-counter | counter |
      | change-notes   | notes   |
      | change-status  | status  |
      | add-tag        | tags    |
      | remove-tag     | tags    |

  @id-inverse
  @level-exhaustive
  @mode-property
  Scenario Outline: Undoing <id> restores the committed before-snapshot fixture
    Given the committed before-snapshot and mutation fixtures for the <id> kind
    When <id> is applied and then its own computed inverse steps are applied
      """
      {"kind": "<id>", "moves": "<moves>"}
      """
    Then the snapshot equals the committed before-snapshot again, member for member
    Examples:
      | id             | moves   |
      | rename-vcs     | title   |
      | change-counter | counter |
      | change-notes   | notes   |
      | change-status  | status  |
      | add-tag        | tags    |
      | remove-tag     | tags    |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Read the real committed checkpoint through its own DSL carrier and print it back
    Given the real committed text artifact asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio
    When the artifact is parsed, printed back to `.vcs.dsl.semio` and parsed again
    Then every decoding agrees on the same checkpoint — "VCS Demo" at counter 2, status draft, tags alpha then beta — and the printed text reproduces the committed file byte for byte
