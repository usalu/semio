Feature: Quantity Driven BIM Costs
  Scenario: One type contributes several independently authored cost items
    Given walls of one type have net side areas 8 and 4 and net volumes 2 and 1
    And finishing costs 12.5 EUR per net square metre with factor 1.1
    And structure costs 100 EUR per net cubic metre
    When the quantity takeoff is costed
    Then the project cost is 465 EUR
    And the ground and upper storeys cost 310 and 155 EUR

  Scenario: Currencies remain individually addressable
    Given two components cost 40 CHF each
    When they are costed alongside the walls
    Then the project retains separate EUR and CHF totals

  Scenario: Parametric quantities and inverse replay propagate to costs
    Given the walls and their authored cost links
    When a storey edit doubles one wall's inferred quantities
    Then its quantity based costs double without changing the authored cost records
    When the inverse restores the quantities
    Then every cost returns to the original result

  Scenario: Invalid or incomplete costs are visible
    Given a link to a missing item or nonfinite or negative pricing
    When costs are inferred
    Then a diagnostic identifies the link or item
    And no invalid monetary total is published
