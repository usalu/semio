@capability-bim-1-infer
@oracle-bim-1-shapely-geometry
@comparison-floating-point-v1
Feature: Infer the table of every user-defined schedule from the quantities and the authored facts of the model and audit it with shapely
  `s.bim.model@1` stores a schedule as an authored definition only: a category, the columns (a built-in field or a property of a property set, each summed or not), the sort keys, the filters,
  the grouping levels, whether every instance is listed, and the storeys and phases it covers. `📋️schedules` derives the table: one row per element (per layer with a material for a material
  take-off, per floor, wall and ceiling finish of a resolved room for the finish schedule) read from the quantities and the authored fields, kept when every filter holds, ordered by the
  group keys and then the sort keys (texts in natural order with digit runs as integers, empty cells first, equal rows in element id order), closed by a subtotal row per group, collapsed
  into one row per group when the schedule does not list every instance, and ended by a grand total row that sums the flagged columns. The oracle is `🐍️.py` in this directory. It reproduces
  every table from the committed snapshot: `shapely` 2 gives the quantities through the sibling oracle `../🧮️infer-bim-1-quantities` and the finish areas through `../🪣️infer-bim-1-finishes`,
  the rows, filters, order, grouping and totals are restated from the documented rules alone, and the sums are exactly rounded `math.fsum`. It audits that every grand total is the sum of the
  rows the table lists and that a plain schedule lists every candidate exactly once. The quantity law is proved too: a case directory with a `🦠️mutation` is the model before a `set-wall-top`;
  the oracle applies that one authored field itself and requires that a quantity edit updates the schedules: the rows of every other element are unchanged, the edited wall's row is the only one
  that moves and every summed total moves by exactly what that row moved. The committed expectation is written by that file, never by hand.

  @id-schedules-house
  @level-quick
  @mode-differential
  Scenario: Door, window, room, finish, wall and material schedules with filters, grouping, scope and a property column resolve to their rows and totals
    Given the committed house model shared://💡️inferences/📋️schedules/🏡️house/📸️snapshot/🔣️.json with nine schedules
    When 📋️schedules is inferred for it
    Then every schedule's rows, subtotals and totals equal the table shared://💡️inferences/📋️schedules/🏡️house/💡️inference/📋️schedules/🔣️.json

  @id-schedules-wall-edit
  @level-quick
  @mode-differential
  Scenario: A quantity edit updates the schedule: freeing the height of a wall changes its rows and totals and nothing else
    Given the committed model shared://💡️inferences/📋️schedules/✂️wall-edit/📸️snapshot/🔣️.json with nine schedules
    And the committed mutation shared://💡️inferences/📋️schedules/✂️wall-edit/🦠️mutation/🔣️.json that frees the height of one wall
    When the mutation is applied and 📋️schedules is inferred for the edited model
    Then only that wall's rows and the totals they feed differ and every schedule equals the table shared://💡️inferences/📋️schedules/✂️wall-edit/💡️inference/📋️schedules/🔣️.json
