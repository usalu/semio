# language: en
Feature: A deviation that equals its tolerance has one verdict in every unit and at every scale
  The dimension checks of the multiview matrix compare a drawn extent with the dimensions within D18 (the larger of 15 mm and 2 %).
  The same drawing is compared in metres and in paper millimetres at every denominator, so a deviation that equals the tolerance must not be decided by the last bit of a product.
  THE REFERENCE is exact rational arithmetic (CPython `fractions.Fraction` on the decimal strings of the vectors).

  Scenario Outline: The verdict equals the verdict of exact arithmetic
    Given a drawn extent of <measured> m against a dimension of <nominal> m
    When the deviation is compared with D18 in metres and in paper millimetres at 1:20, 1:50, 1:75, 1:100, 1:150, 1:200, 1:500 and 1:1000, at several positions on the sheet
    Then every comparison says "<verdict>", as exact arithmetic does

    Examples:
      | measured    | nominal | verdict |
      | 1.127       | 1.15    | within  |
      | 1.173       | 1.15    | within  |
      | 1.126999    | 1.15    | exceeds |
      | 1.126999999 | 1.15    | exceeds |
      | 1.127000001 | 1.15    | within  |
      | 0.485       | 0.5     | within  |
      | 0.484999    | 0.5     | exceeds |
      | 2.94        | 3       | within  |
      | 3.0600001   | 3       | exceeds |

  Scenario: A deviation that is not a number exceeds every tolerance
    Given an extent that is not a number
    Then the comparison says "exceeds"
