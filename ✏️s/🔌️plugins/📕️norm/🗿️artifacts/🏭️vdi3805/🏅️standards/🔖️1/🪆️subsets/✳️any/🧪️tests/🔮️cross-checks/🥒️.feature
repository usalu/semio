@capability-vdi3805-1-evaluate
@oracle-vdi3805-1-cross-check
@comparison-numerical-v1
Feature: VDI 3805 radiator and valve laws reproduce the third-party cross-checks
  The independent oracle `🔮️oracles/🌐️cross-checks/🐍️.py` (scipy brentq for the operating point, the EN 442 logarithmic mean excess temperature, the IAPWS-95 water density table) writes the vectors in `🧫️fixtures/🔮️cross-checks/🔣️.json`; every implementation must reproduce the rows tagged @agree.
  The nominal excess temperature is the exact logarithmic mean 49.8329 K and the valve law carries the water density, so no row is a known divergence.

  @id-radiator
  @level-quick
  @mode-differential
  @agree
  Scenario Outline: the EN 442 power law at the logarithmic mean excess temperature
    Given the vector "<vector>" of the section "radiators"
    When the radiator output is evaluated at the excess temperature deltaTLogK
    Then it equals outputW within the relative tolerance <tolerance>
    Examples:
      | vector | tolerance |
      | 0 | 1e-9 |
      | 1 | 1e-9 |
      | 2 | 1e-9 |
      | 3 | 1e-9 |
      | 4 | 1e-9 |
  @id-valve
  @level-quick
  @mode-differential
  @agree
  Scenario Outline: the valve pressure drop follows the square law of the kvs value scaled by the water density
    Given the vector "<vector>" of the section "valves"
    When the pressure drop at the flow is evaluated
    Then it equals dropBar within the relative tolerance <tolerance>
    Examples:
      | vector | tolerance |
      | 0 | 3e-5 |
      | 1 | 3e-5 |
      | 2 | 3e-5 |
      | 3 | 3e-5 |
      | 4 | 3e-5 |
