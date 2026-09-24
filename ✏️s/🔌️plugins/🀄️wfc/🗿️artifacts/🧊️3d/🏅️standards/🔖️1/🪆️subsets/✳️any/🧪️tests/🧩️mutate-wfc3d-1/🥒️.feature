@capability-wfc3d-1-mutate
@oracle-wfc3d-python-independent
@comparison-ordered-json-v1
@mutations-wfc3d-1-any
Feature: Apply every typed wfc3d mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️.py` in this directory: a second
  implementation of the `s.wfc.wfc3d` document and all fifteen typed mutations, written in Python from
  `../../🧬️schema/📸️snapshot/🔣️.json`, `../../🧬️schema/🧬️mutations/🔣️.json` and each
  `../../🧬️schema/🧬️mutations/<kind>/🧬️schema/🔣️.json`, plus the fifteen committed quintets. It
  imports nothing from this repository's Rust, so a bug that lives in the Rust diff builder cannot
  also live in the reference.

  Why a second implementation rather than a third-party library. This document is the INPUT to a
  wave-function-collapse solve, not its output. A WFC library computes a collapse; none of them
  carries the problem statement as a document and none of them reads this carrier, so none can answer
  what `delete-tile` does to a rule set or where `create-slot` inserts. The SOLVE itself is
  deliberately out of scope here: it is an inference, pinned by this artifact's own deterministic-seed
  and contradiction tests and by the committed example outcomes, not by this editing algebra.

  Four facts only the committed vectors state, and both implementations take them from there. The
  payload is EXTERNALLY tagged — `{"DeleteSlot": {…}}`, a PascalCase variant name as the single key.
  A `create-*` carries the collection's CANONICAL SORTED insertion index, which is what makes a delete
  followed by its inverse restore the row's position rather than append it. There are TWO cascades and
  they are asymmetric: `delete-slot` drops every edge incident to the slot and says so with an
  `info`-level `wfc3d.slot.edges-cascaded`, while `delete-tile` drops every rule NAMING the tile AND
  releases every pin ON it, under `wfc3d.tile.references-cascaded`. And a refusal produces an EMPTY
  delta with a diagnostic rather than an error, so applying a refused mutation is a document no-op —
  which is why the reference asserts the outcome's messages, not an error type.

  📌️ ONE CEILING ON WHAT THIS ESTABLISHES, stated rather than implied. The reference replays the
  committed vectors and computes its own answer for each; it does not drive this subset's Rust codec.
  What it establishes is that an independent implementation of the specification computes the same
  deltas, the same after-snapshots, the same diagnostics AND the same inverses — a real check of the
  vectors and of the algebra, but not yet our codec against a second producer.

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: The committed <id> vector declares its own kind and moves the document
    Given the committed specification vector for the <id> kind
      """
      {
        "kind": "<id>",
        "before": "shared://🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "shared://🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "diff": "shared://🧬️mutations/<vector>/🔺️diff/🔣️.json",
        "outcome": "shared://🧬️mutations/<vector>/🎯️outcome/🔣️.json",
        "after": "shared://🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json"
      }
      """
    Then the committed mutation payload declares the <id> kind
    And the after-snapshot differs from the before-snapshot, or the committed outcome declares the vector a no-op
    Examples:
      | id                 | vector                                                     |
      | change-seed        | 🎲️change-seed/🎲️reseeds-the-solve-from-7-to-99             |
      | create-slot        | 🧩️create-slot/🧩️inserts-room-c-at-the-sorted-position      |
      | delete-slot        | 🕳️delete-slot/🕳️removes-room-a-and-cascades-its-edge       |
      | move-slot          | 🚚️move-slot/🚚️lifts-room-b-one-storey                      |
      | resize-slot        | 📐️resize-slot/📐️widens-room-a-to-a-double-bay              |
      | connect-slots      | 🔗️connect-slots/🔗️joins-room-a-to-room-b-beside            |
      | disconnect-slots   | ✂️disconnect-slots/✂️severs-the-corridor-b-edge            |
      | pin-slot           | 📌️pin-slot/📌️pins-room-a-to-the-room-tile                  |
      | unpin-slot         | 📍️unpin-slot/📍️releases-the-pin-on-room-a                  |
      | create-tile        | 🀄️create-tile/🀄️adds-a-stair-tile-to-the-catalogue         |
      | delete-tile        | 🗑️delete-tile/🗑️drops-the-corridor-tile-and-its-rule       |
      | change-tile-weight | ⚖️change-tile-weight/⚖️raises-the-room-tile-selection-bias |
      | change-tile-media  | 🖼️change-tile-media/🖼️swaps-the-corridor-box-for-a-wedge   |
      | create-rule        | 🚦️create-rule/🚦️forbids-two-rooms-side-by-side           |
      | delete-rule        | 🚫️delete-rule/🚫️drops-the-room-corridor-pairing            |

  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: The committed <id> vector changes only what its diff declares, and inverts exactly
    Given the committed specification vector for the <id> kind
      """
      {
        "kind": "<id>",
        "before": "shared://🧬️mutations/<vector>/📸️snapshot/⬅️before/🔣️.json",
        "mutation": "shared://🧬️mutations/<vector>/🦠️mutation/🔣️.json",
        "diff": "shared://🧬️mutations/<vector>/🔺️diff/🔣️.json",
        "outcome": "shared://🧬️mutations/<vector>/🎯️outcome/🔣️.json",
        "after": "shared://🧬️mutations/<vector>/📸️snapshot/➡️after/🔣️.json"
      }
      """
    Then every field where the after-snapshot differs from the before-snapshot is declared by the committed diff
    And every field the committed diff declares actually differs
    And the reference's own inverse of the committed mutation restores the before-snapshot exactly
    Examples:
      | id                 | vector                                                     |
      | change-seed        | 🎲️change-seed/🎲️reseeds-the-solve-from-7-to-99             |
      | create-slot        | 🧩️create-slot/🧩️inserts-room-c-at-the-sorted-position      |
      | delete-slot        | 🕳️delete-slot/🕳️removes-room-a-and-cascades-its-edge       |
      | move-slot          | 🚚️move-slot/🚚️lifts-room-b-one-storey                      |
      | resize-slot        | 📐️resize-slot/📐️widens-room-a-to-a-double-bay              |
      | connect-slots      | 🔗️connect-slots/🔗️joins-room-a-to-room-b-beside            |
      | disconnect-slots   | ✂️disconnect-slots/✂️severs-the-corridor-b-edge            |
      | pin-slot           | 📌️pin-slot/📌️pins-room-a-to-the-room-tile                  |
      | unpin-slot         | 📍️unpin-slot/📍️releases-the-pin-on-room-a                  |
      | create-tile        | 🀄️create-tile/🀄️adds-a-stair-tile-to-the-catalogue         |
      | delete-tile        | 🗑️delete-tile/🗑️drops-the-corridor-tile-and-its-rule       |
      | change-tile-weight | ⚖️change-tile-weight/⚖️raises-the-room-tile-selection-bias |
      | change-tile-media  | 🖼️change-tile-media/🖼️swaps-the-corridor-box-for-a-wedge   |
      | create-rule        | 🚦️create-rule/🚦️forbids-two-rooms-side-by-side           |
      | delete-rule        | 🚫️delete-rule/🚫️drops-the-room-corridor-pairing            |

  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Decode and re-encode the three-slot corridor problem
    Given the committed before-snapshot shared://🧬️mutations/🎲️change-seed/🎲️reseeds-the-solve-from-7-to-99/📸️snapshot/⬅️before/🔣️.json
    When it is parsed by the platform's own dependency-free JSON reader, re-serialized and parsed again
    Then the document is unchanged and every tile keeps its inline mesh buffers
