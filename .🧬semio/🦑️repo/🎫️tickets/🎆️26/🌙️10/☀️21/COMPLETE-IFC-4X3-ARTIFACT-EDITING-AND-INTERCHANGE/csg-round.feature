Feature: Declared circular CSG primitive geometry
  Scenario Outline: Explicit source geometry and original sampling policy
    Given the exact declared primitive, placements, SI prefix and mapping in a neutral row
    When the circular primitive is evaluated under its captured chord-error and segment policy
    Then the supported polygon has the stated quadrant segment count, bounds and polygon volume
    And reflected mappings preserve coupled surface orientation after one orientation correction
    And unsupported schema roles, nonpositive dimensions and inadequate sampling resources refuse
    And a positive subnormal schema value is distinguished from selected geometry precision admission
