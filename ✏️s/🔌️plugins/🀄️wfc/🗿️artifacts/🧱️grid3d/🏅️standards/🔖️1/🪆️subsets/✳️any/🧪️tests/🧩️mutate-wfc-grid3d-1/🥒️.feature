@capability-wfc-grid3d-1-mutate
@oracle-wfc-grid3d-python-independent
@comparison-ordered-json-v1
@mutations-wfc-grid3d-1-any
Feature: Apply every typed grid3d mutation twice — once in Rust, once in Python — and require the same answer
  This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️.py` in this directory: a second
  implementation of the `s.wfc.grid3d` document and all fourteen typed mutations, written in Python
  against the normative JSON Schema rather than ported from the Rust. It re-derives every sparse diff
  and re-applies every committed one, and replays every committed quintet on its own
  (`python3 🐍️.py`).

  Why a second implementation rather than a third-party library. The surveyed WaveFunctionCollapse
  implementations solve a tiled model; none carries a persisted 3D problem document with per-axis cell
  sizes, an authored allow-list and a mutation vocabulary, so none can adjudicate an edit to it.

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
      | id                 | vector                                             |
      | change-seed        | 🎲️change-seed/🎲️reseeds-the-solve-from-7-to-99     |
      | resize-grid        | 📐️resize-grid/📐️grows-the-grid-to-3x2x2            |
      | change-cell-sizes  | 📏️change-cell-sizes/📏️stretches-the-x-axis-columns |
      | change-periodicity | 🔁️change-periodicity/🔁️wraps-the-x-axis            |
      | create-tile        | 🧱️create-tile/🧱️adds-the-roof-tile                 |
      | delete-tile        | 🕳️delete-tile/🕳️removes-the-air-tile-and-cascades  |
      | change-tile-weight | ⚖️change-tile-weight/⚖️raises-the-wall-tile-bias   |
      | change-tile-media  | 🖼️change-tile-media/🖼️replaces-the-wall-tile-mesh  |
      | create-rule        | 🚦️create-rule/🚦️allows-air-above-air               |
      | delete-rule        | ❌️delete-rule/❌️removes-the-floor-wall-rule        |
      | pin-cell           | 📌️pin-cell/📌️pins-the-far-cell-to-wall             |
      | unpin-cell         | 📍️unpin-cell/📍️releases-the-origin-cell            |
      | mask-cell          | 🚫️mask-cell/🚫️carves-out-the-far-edge-cell         |
      | unmask-cell        | 🔓️unmask-cell/🔓️restores-the-masked-corner         |

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
      | id                 | vector                                             |
      | change-seed        | 🎲️change-seed/🎲️reseeds-the-solve-from-7-to-99     |
      | resize-grid        | 📐️resize-grid/📐️grows-the-grid-to-3x2x2            |
      | change-cell-sizes  | 📏️change-cell-sizes/📏️stretches-the-x-axis-columns |
      | change-periodicity | 🔁️change-periodicity/🔁️wraps-the-x-axis            |
      | create-tile        | 🧱️create-tile/🧱️adds-the-roof-tile                 |
      | delete-tile        | 🕳️delete-tile/🕳️removes-the-air-tile-and-cascades  |
      | change-tile-weight | ⚖️change-tile-weight/⚖️raises-the-wall-tile-bias   |
      | change-tile-media  | 🖼️change-tile-media/🖼️replaces-the-wall-tile-mesh  |
      | create-rule        | 🚦️create-rule/🚦️allows-air-above-air               |
      | delete-rule        | ❌️delete-rule/❌️removes-the-floor-wall-rule        |
      | pin-cell           | 📌️pin-cell/📌️pins-the-far-cell-to-wall             |
      | unpin-cell         | 📍️unpin-cell/📍️releases-the-origin-cell            |
      | mask-cell          | 🚫️mask-cell/🚫️carves-out-the-far-edge-cell         |
      | unmask-cell        | 🔓️unmask-cell/🔓️restores-the-masked-corner         |
