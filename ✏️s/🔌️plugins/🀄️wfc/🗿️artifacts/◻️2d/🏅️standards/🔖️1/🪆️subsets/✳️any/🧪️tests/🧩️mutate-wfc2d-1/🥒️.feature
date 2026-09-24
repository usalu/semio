@capability-wfc2d-1-mutate
@oracle-wfc2d-python-independent
@comparison-ordered-json-v1
@mutations-wfc2d-1-any
Feature: Apply every typed wfc2d mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️.py` in this directory: a second
  implementation of the `s.wfc.wfc2d` document and all fifteen typed mutations, written in Python
  from `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`, from the per-kind payload
  schemas under `🧬️schema/🧬️mutations/<kind>/🧬️schema/🔣️.json`, and from the fifteen committed
  quintets. It imports nothing from this repository's Rust and can be replayed on its own
  (`python3 🐍️.py`), which is how it is run when the repo test host is not available.

  Why a second implementation rather than a third-party library. A `wfc2d` document is the INPUT to a
  wave-function-collapse solve, not its output: free rectangles, an explicit adjacency edge list with
  NAMED relation classes, a tile alphabet carrying inline 2D media, and a rule set, all under one
  seed. A WFC library computes a collapse; none of them carries the problem statement as a document,
  and none of them reads this carrier. The ancestor case `mutate-assembly-1` settled that this
  algebra is adjudicable by a Python twin over exactly this carrier shape.

  Four things only the committed vectors state, and both implementations take them from there. This
  subset tags its mutations EXTERNALLY — a payload is `{"CreateSlot": {…}}`, a PascalCase variant
  name as the single key. Every create inserts at the CANONICAL ASCENDING-`id` position, never at the
  end, which is what makes a create/delete pair restore a row's POSITION as well as its value.
  `delete-slot` cascades into the edges naming the slot, and `delete-tile` cascades into BOTH the
  rules naming the tile and the slot pins holding it — each announcing itself with one `info`-level
  `mutation.cascade`. And a diff is TOTAL: every lane is an explicit key, an untouched one carrying
  `null` or `[]`, never an omitted key.

  ✅️ ALL FIFTEEN KINDS ARE ADJUDICATED AND NONE IS REFUSED: the tile media is only ever carried here,
  never re-derived, so nothing depends on a rendering function no specification states.

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
      | id                 | vector                                                            |
      | change-seed        | 🎲️change-seed/🎲️reseeds-the-solve-from-7-to-99                    |
      | create-slot        | 🧩️create-slot/🧩️inserts-slot-d-in-canonical-order                 |
      | delete-slot        | 🕳️delete-slot/🚫️removes-slot-a-and-cascades-edge-ab               |
      | move-slot          | ↔️move-slot/↔️drags-slot-b-down                                   |
      | resize-slot        | 📐️resize-slot/📐️widens-slot-b                                     |
      | connect-slots      | 🔗️connect-slots/🔗️joins-slot-a-to-slot-c                          |
      | disconnect-slots   | ✂️disconnect-slots/✂️severs-edge-ab                               |
      | pin-slot           | 📌️pin-slot/📌️pins-slot-a-to-the-roof-tile                         |
      | unpin-slot         | 🔓️unpin-slot/🔓️releases-the-slot-c-pin                            |
      | create-tile        | 🀄️create-tile/🀄️adds-the-window-tile                              |
      | delete-tile        | 🗑️delete-tile/🚫️removes-the-wall-tile-and-cascades-rules-and-pins |
      | change-tile-weight | ⚖️change-tile-weight/⚖️raises-the-wall-tile-bias                  |
      | change-tile-media  | 🎨️change-tile-media/🎨️repaints-the-roof-tile-as-a-raster          |
      | create-rule        | 🚦️create-rule/⛔️forbids-roof-over-roof                            |
      | delete-rule        | ❌delete-rule/🚫️removes-the-wall-wall-rule                         |

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
      | id                 | vector                                                            |
      | change-seed        | 🎲️change-seed/🎲️reseeds-the-solve-from-7-to-99                    |
      | create-slot        | 🧩️create-slot/🧩️inserts-slot-d-in-canonical-order                 |
      | delete-slot        | 🕳️delete-slot/🚫️removes-slot-a-and-cascades-edge-ab               |
      | move-slot          | ↔️move-slot/↔️drags-slot-b-down                                   |
      | resize-slot        | 📐️resize-slot/📐️widens-slot-b                                     |
      | connect-slots      | 🔗️connect-slots/🔗️joins-slot-a-to-slot-c                          |
      | disconnect-slots   | ✂️disconnect-slots/✂️severs-edge-ab                               |
      | pin-slot           | 📌️pin-slot/📌️pins-slot-a-to-the-roof-tile                         |
      | unpin-slot         | 🔓️unpin-slot/🔓️releases-the-slot-c-pin                            |
      | create-tile        | 🀄️create-tile/🀄️adds-the-window-tile                              |
      | delete-tile        | 🗑️delete-tile/🚫️removes-the-wall-tile-and-cascades-rules-and-pins |
      | change-tile-weight | ⚖️change-tile-weight/⚖️raises-the-wall-tile-bias                  |
      | change-tile-media  | 🎨️change-tile-media/🎨️repaints-the-roof-tile-as-a-raster          |
      | create-rule        | 🚦️create-rule/⛔️forbids-roof-over-roof                            |
      | delete-rule        | ❌delete-rule/🚫️removes-the-wall-wall-rule                         |
