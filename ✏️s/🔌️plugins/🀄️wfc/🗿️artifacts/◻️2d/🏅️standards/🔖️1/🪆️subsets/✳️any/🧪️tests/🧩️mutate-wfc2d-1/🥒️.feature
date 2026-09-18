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

  Scenario Outline: <case> agrees across Rust and Python
    Given the committed before-snapshot at <vector>
    When the <kind> mutation is applied in both implementations
    Then the produced diff, the raised diagnostics and the after-snapshot agree

    Examples: mutate
      | id                 | vector             | code                                                    |
      | change-seed        | mutate-change-seed        | asset://🧫️fixtures/🧬️mutations/🎲️change-seed/🎲️reseeds-the-solve-from-7-to-99/ |
      | create-slot        | mutate-create-slot        | asset://🧫️fixtures/🧬️mutations/🧩️create-slot/🧩️inserts-slot-d-in-canonical-order/ |
      | delete-slot        | mutate-delete-slot        | asset://🧫️fixtures/🧬️mutations/🕳️delete-slot/🚫️removes-slot-a-and-cascades-edge-ab/ |
      | move-slot          | mutate-move-slot          | asset://🧫️fixtures/🧬️mutations/↔️move-slot/↔️drags-slot-b-down/ |
      | resize-slot        | mutate-resize-slot        | asset://🧫️fixtures/🧬️mutations/📐️resize-slot/📐️widens-slot-b/ |
      | connect-slots      | mutate-connect-slots      | asset://🧫️fixtures/🧬️mutations/🔗️connect-slots/🔗️joins-slot-a-to-slot-c/ |
      | disconnect-slots   | mutate-disconnect-slots   | asset://🧫️fixtures/🧬️mutations/✂️disconnect-slots/✂️severs-edge-ab/ |
      | pin-slot           | mutate-pin-slot           | asset://🧫️fixtures/🧬️mutations/📌️pin-slot/📌️pins-slot-a-to-the-roof-tile/ |
      | unpin-slot         | mutate-unpin-slot         | asset://🧫️fixtures/🧬️mutations/🔓️unpin-slot/🔓️releases-the-slot-c-pin/ |
      | create-tile        | mutate-create-tile        | asset://🧫️fixtures/🧬️mutations/🀄️create-tile/🀄️adds-the-window-tile/ |
      | delete-tile        | mutate-delete-tile        | asset://🧫️fixtures/🧬️mutations/🗑️delete-tile/🚫️removes-the-wall-tile-and-cascades-rules-and-pins/ |
      | change-tile-weight | mutate-change-tile-weight | asset://🧫️fixtures/🧬️mutations/⚖️change-tile-weight/⚖️raises-the-wall-tile-bias/ |
      | change-tile-media  | mutate-change-tile-media  | asset://🧫️fixtures/🧬️mutations/🎨️change-tile-media/🎨️repaints-the-roof-tile-as-a-raster/ |
      | create-rule        | mutate-create-rule        | asset://🧫️fixtures/🧬️mutations/🚦️create-rule/⛔️forbids-roof-over-roof/ |
      | delete-rule        | mutate-delete-rule        | asset://🧫️fixtures/🧬️mutations/❌delete-rule/🚫️removes-the-wall-wall-rule/ |

  Scenario Outline: <case> inverts across Rust and Python
    Given the committed before-snapshot at <vector>
    When the <kind> mutation and then its inverse are applied in both implementations
    Then the before-snapshot is restored exactly, row values and row positions alike

    Examples: inverse
      | id                 | vector             | code                                                    |
      | change-seed        | inverse-change-seed        | asset://🧫️fixtures/🧬️mutations/🎲️change-seed/🎲️reseeds-the-solve-from-7-to-99/ |
      | create-slot        | inverse-create-slot        | asset://🧫️fixtures/🧬️mutations/🧩️create-slot/🧩️inserts-slot-d-in-canonical-order/ |
      | delete-slot        | inverse-delete-slot        | asset://🧫️fixtures/🧬️mutations/🕳️delete-slot/🚫️removes-slot-a-and-cascades-edge-ab/ |
      | move-slot          | inverse-move-slot          | asset://🧫️fixtures/🧬️mutations/↔️move-slot/↔️drags-slot-b-down/ |
      | resize-slot        | inverse-resize-slot        | asset://🧫️fixtures/🧬️mutations/📐️resize-slot/📐️widens-slot-b/ |
      | connect-slots      | inverse-connect-slots      | asset://🧫️fixtures/🧬️mutations/🔗️connect-slots/🔗️joins-slot-a-to-slot-c/ |
      | disconnect-slots   | inverse-disconnect-slots   | asset://🧫️fixtures/🧬️mutations/✂️disconnect-slots/✂️severs-edge-ab/ |
      | pin-slot           | inverse-pin-slot           | asset://🧫️fixtures/🧬️mutations/📌️pin-slot/📌️pins-slot-a-to-the-roof-tile/ |
      | unpin-slot         | inverse-unpin-slot         | asset://🧫️fixtures/🧬️mutations/🔓️unpin-slot/🔓️releases-the-slot-c-pin/ |
      | create-tile        | inverse-create-tile        | asset://🧫️fixtures/🧬️mutations/🀄️create-tile/🀄️adds-the-window-tile/ |
      | delete-tile        | inverse-delete-tile        | asset://🧫️fixtures/🧬️mutations/🗑️delete-tile/🚫️removes-the-wall-tile-and-cascades-rules-and-pins/ |
      | change-tile-weight | inverse-change-tile-weight | asset://🧫️fixtures/🧬️mutations/⚖️change-tile-weight/⚖️raises-the-wall-tile-bias/ |
      | change-tile-media  | inverse-change-tile-media  | asset://🧫️fixtures/🧬️mutations/🎨️change-tile-media/🎨️repaints-the-roof-tile-as-a-raster/ |
      | create-rule        | inverse-create-rule        | asset://🧫️fixtures/🧬️mutations/🚦️create-rule/⛔️forbids-roof-over-roof/ |
      | delete-rule        | inverse-delete-rule        | asset://🧫️fixtures/🧬️mutations/❌delete-rule/🚫️removes-the-wall-wall-rule/ |
