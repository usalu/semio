@capability-bim-1-infer
@oracle-bim-1-sympy-costs
@comparison-floating-point-v1
Feature: Price the quantity take-off with authored cost items and audit the exact decimal arithmetic with sympy
  `s.bim.model@1` stores authored cost items (a currency, a basis, a unit price and a rounding precision) and links of those items to element types (a factor). `💰️costs` prices
  every element of a linked type with `quantity * unit price * factor`, where the quantity is the field of the inferred take-off row that the basis names. The product is exact and
  rounded once, half up, to the precision of the item; the sums per element, type, storey, item and project are exact decimals per currency and are never converted. The oracle is
  `🐍️.py` in this directory: it recomputes every case from the same committed fixture with `sympy` rationals (exact arithmetic that has never seen this repository), floors
  `x * 10^precision + 1/2` for the half-up rounding and converts the exact sums to the nearest double. A missing item, an invalid item, link or quantity and an overflow are diagnostics
  and publish no amount.

  @id-costs-items
  @level-quick
  @mode-differential
  Scenario: One type contributes several independently authored cost items
    Given walls of one type have net side areas 8 and 4 and net volumes 2 and 1 in the committed costs shared://💡️inferences/💰️costs/🔣️.json
    And finishing costs 12.5 EUR per net square metre with factor 1.1
    And structure costs 100 EUR per net cubic metre
    When the quantity takeoff is costed
    Then the project cost is 465 EUR
    And the ground and upper storeys cost 310 and 155 EUR
    And every cost line and every total equals the exact decimal sums of the oracle

  @id-costs-currencies
  @level-quick
  @mode-differential
  Scenario: Currencies remain individually addressable
    Given two components cost 40 CHF each in the committed costs shared://💡️inferences/💰️costs/🔣️.json
    When they are costed alongside the walls
    Then the project retains separate EUR and CHF totals of 465 and 80

  @id-costs-replay
  @level-quick
  @mode-differential
  Scenario: Parametric quantities and inverse replay propagate to costs
    Given the walls and their authored cost links of shared://💡️inferences/💰️costs/🔣️.json
    When a storey edit doubles one wall's inferred quantities
    Then its quantity based costs double without changing the authored cost records
    When the inverse restores the quantities
    Then every cost returns to the original result

  @id-costs-diagnostics
  @level-quick
  @mode-differential
  Scenario: Invalid or incomplete costs are visible
    Given the committed costs shared://💡️inferences/💰️costs/🔣️.json with these further authored records
      """
      {
        "items": {"cheap": {"name": "Cheap", "currency": "EUR", "basis": "Count", "unit_cost": -1, "precision": 2}, "blank": {"name": "Blank", "currency": "  ", "basis": "Count", "unit_cost": 1, "precision": 2}, "fine": {"name": "Fine", "currency": "EUR", "basis": "Count", "unit_cost": 1, "precision": 12}},
        "links": {
          "missing": {"type_id": "wall-type", "item": "absent", "factor": 1},
          "negative-price": {"type_id": "wall-type", "item": "cheap", "factor": 1},
          "blank-currency": {"type_id": "wall-type", "item": "blank", "factor": 1},
          "fine-precision": {"type_id": "wall-type", "item": "fine", "factor": 1},
          "negative-factor": {"type_id": "wall-type", "item": "structure", "factor": -1},
          "blank-type": {"type_id": " ", "item": "structure", "factor": 1},
          "overflow": {"type_id": "wall-type", "item": "structure", "factor": 1.7976931348623157e308}
        }
      }
      """
    When costs are inferred
    Then a diagnostic identifies the link or item
    And no invalid monetary total is published
