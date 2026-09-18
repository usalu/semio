# 🥒️ `s.wfc.grid2d@1/*` mutation replay — one scenario per mutation kind, every vector declared
# as an `asset://` fixture URI so a plan can pin it by digest instead of inlining a payload.
Feature: Mutating a 2D WFC grid
  The persisted document is the PROBLEM only. Every mutation below carries its committed
  before-snapshot to its committed after-snapshot, and its inverse carries it back — value AND
  position, because every collection insert lands at its canonical sorted index.

  Scenario Outline: <kind> replays its committed vector
    Given the committed before-snapshot of <vector>
    When the <kind> mutation of <vector> is applied
    Then the document equals the committed after-snapshot
    And the produced diff equals the committed diff
    And the declared outcome is <code>
    And applying the inverse restores the before-snapshot

    Examples:
      | kind | vector | asset | code |
      | change-seed | 🎲️reseeds-the-solve-from-7-to-99 | asset://wfc/grid2d/1/any/mutations/change-seed/🎲️reseeds-the-solve-from-7-to-99 | applied |
      | resize-grid | 📐️shrinks-the-board-and-drops-the-outside-cells | asset://wfc/grid2d/1/any/mutations/resize-grid/📐️shrinks-the-board-and-drops-the-outside-cells | applied |
      | change-cell-size | 📏️widens-every-cell | asset://wfc/grid2d/1/any/mutations/change-cell-size/📏️widens-every-cell | applied |
      | change-periodicity | 🔁️wraps-the-x-axis | asset://wfc/grid2d/1/any/mutations/change-periodicity/🔁️wraps-the-x-axis | applied |
      | create-tile | 🌱️inserts-the-corner-tile-in-sorted-order | asset://wfc/grid2d/1/any/mutations/create-tile/🌱️inserts-the-corner-tile-in-sorted-order | applied |
      | delete-tile | 🗑️removes-the-straight-tile-and-cascades-its-rule-and-pin | asset://wfc/grid2d/1/any/mutations/delete-tile/🗑️removes-the-straight-tile-and-cascades-its-rule-and-pin | applied |
      | change-tile-weight | ⚖️biases-the-solve-towards-empty | asset://wfc/grid2d/1/any/mutations/change-tile-weight/⚖️biases-the-solve-towards-empty | applied |
      | change-tile-media | 🎨️redraws-the-empty-tile-as-a-bitmap | asset://wfc/grid2d/1/any/mutations/change-tile-media/🎨️redraws-the-empty-tile-as-a-bitmap | applied |
      | create-rule | 🚦️lets-two-straights-stack-vertically | asset://wfc/grid2d/1/any/mutations/create-rule/🚦️lets-two-straights-stack-vertically | applied |
      | delete-rule | ❌️forbids-the-straight-pair-again | asset://wfc/grid2d/1/any/mutations/delete-rule/❌️forbids-the-straight-pair-again | applied |
      | pin-cell | 📌️fixes-the-right-cell-to-the-straight-tile | asset://wfc/grid2d/1/any/mutations/pin-cell/📌️fixes-the-right-cell-to-the-straight-tile | applied |
      | unpin-cell | 📍️releases-the-pinned-straight-cell | asset://wfc/grid2d/1/any/mutations/unpin-cell/📍️releases-the-pinned-straight-cell | applied |
      | mask-cell | 🕳️cuts-the-pinned-corner-out-of-the-problem | asset://wfc/grid2d/1/any/mutations/mask-cell/🕳️cuts-the-pinned-corner-out-of-the-problem | applied |
      | unmask-cell | 🔳️puts-the-hole-back-into-the-problem | asset://wfc/grid2d/1/any/mutations/unmask-cell/🔳️puts-the-hole-back-into-the-problem | applied |
