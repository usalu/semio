@capability-wfc-grid2d-1-mutate
@oracle-wfc-grid2d-python-independent
@comparison-ordered-json-v1
@mutations-wfc-grid2d-1-any
Feature: Apply every typed grid2d mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️.py` in this directory: a second
  implementation of the `s.wfc.grid2d` document and all fourteen typed mutations' diff, apply and
  inverse, written in Python from `../../🧬️schema/📸️snapshot/🔣️.json` and the per-kind payload schemas
  under `../../🧬️schema/🧬️mutations/<kind>/🧬️schema/🔣️.json`. It imports nothing from this repository
  and replays every committed quintet on its own (`python3 🐍️.py`).

  Why a second implementation rather than a third-party library. The published WaveFunctionCollapse
  libraries solve a tiled model; none of them models an editable problem document with an invertible,
  canonically positioned mutation log, and none reads this carrier.

  The persisted document is the PROBLEM only. Every mutation carries its committed before-snapshot to
  its committed after-snapshot, and its inverse carries it back — value AND position, because every
  collection insert lands at its canonical sorted index.

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
      | id                 | vector                                                                  |
      | change-seed        | 🎲️change-seed/🎲️reseeds                          |
      | resize-grid        | 📐️resize-grid/📐️shrinks           |
      | change-cell-size   | 📏️change-cell-size/📏️widens-every-cell                                  |
      | change-periodicity | 🔁️change-periodicity/🔁️wraps-the-x-axis                                 |
      | create-tile        | 🌱️create-tile/🌱️inserts                 |
      | delete-tile        | 🗑️delete-tile/🗑️removes |
      | change-tile-weight | ⚖️change-tile-weight/⚖️biases                   |
      | change-tile-media  | 🎨️change-tile-media/🎨️redraws                |
      | create-rule        | 🚦️create-rule/🚦️lets                     |
      | delete-rule        | ❌delete-rule/❌️forbids                          |
      | pin-cell           | 📌️pin-cell/📌️fixes                  |
      | unpin-cell         | 📍️unpin-cell/📍️releases                        |
      | mask-cell          | 🕳️mask-cell/🕳️cuts                 |
      | unmask-cell        | 🔳️unmask-cell/🔳️puts                     |

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
      | id                 | vector                                                                  |
      | change-seed        | 🎲️change-seed/🎲️reseeds                          |
      | resize-grid        | 📐️resize-grid/📐️shrinks           |
      | change-cell-size   | 📏️change-cell-size/📏️widens-every-cell                                  |
      | change-periodicity | 🔁️change-periodicity/🔁️wraps-the-x-axis                                 |
      | create-tile        | 🌱️create-tile/🌱️inserts                 |
      | delete-tile        | 🗑️delete-tile/🗑️removes |
      | change-tile-weight | ⚖️change-tile-weight/⚖️biases                   |
      | change-tile-media  | 🎨️change-tile-media/🎨️redraws                |
      | create-rule        | 🚦️create-rule/🚦️lets                     |
      | delete-rule        | ❌delete-rule/❌️forbids                          |
      | pin-cell           | 📌️pin-cell/📌️fixes                  |
      | unpin-cell         | 📍️unpin-cell/📍️releases                        |
      | mask-cell          | 🕳️mask-cell/🕳️cuts                 |
      | unmask-cell        | 🔳️unmask-cell/🔳️puts                     |
