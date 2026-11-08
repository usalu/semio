@capability-din16798-1-evaluate
@oracle-din16798-1-comfort-independent
@comparison-numerical-v1
Feature: DIN EN 16798-1 comfort and ventilation closed forms reproduce the independent oracle
  The Python oracle (ISO 7730 Annex D fixed point cross-checked by a bisection of the clothing heat balance, the adaptive comfort bands of Annex B.2, the
  outdoor-air rates of Tables B.6 and B.7 and the CO2 limits of Table B.5 with the national absolute limits) writes the committed vectors in
  `🧫️fixtures/🌡️comfort-vectors/🔣️.json`; the Rust check kernels and every other implementation must reproduce them.

  @id-pmv-vector
  @level-quick
  @mode-differential
  Scenario Outline: the predicted mean vote equals the oracle value
    Given the vector "<vector>" of the section "pmv"
    When the predicted mean vote of the operative temperature, humidity, air speed, metabolic rate and clothing is evaluated
    Then it equals the oracle value within the absolute tolerance <tolerance>
    Examples:
      | vector | tolerance |
      | 0      | 1e-6      |
      | 1      | 1e-6      |
      | 2      | 1e-6      |
      | 3      | 1e-6      |
      | 4      | 1e-6      |
      | 5      | 1e-6      |
      | 6      | 1e-6      |

  @id-pmv-published
  @level-quick
  @mode-differential
  Scenario: the predicted mean vote reproduces the published table D.1 of ISO 7730
    Given the section "iso7730_table_d1"
    When the predicted mean vote of every row is evaluated
    Then it equals the published value within the absolute tolerance 0.011

  @id-adaptive-band
  @level-quick
  @mode-differential
  Scenario: the adaptive comfort bands follow theta_c = 0.33 theta_rm + 18.8 and the category half widths 2, 3, 4 and 5 K
    Given the section "adaptive"
    When the lower limit, the comfort temperature and the upper limit are evaluated
    Then every value equals the oracle value within the absolute tolerance 1e-9

  @id-ventilation-components
  @level-quick
  @mode-differential
  Scenario: the outdoor air of persons and of the building follow the litres per second times 3.6
    Given the section "ventilation"
    When the persons and the building component are evaluated
    Then every value equals the oracle value within the absolute tolerance 1e-9

  @id-co2-limit
  @level-quick
  @mode-differential
  Scenario: the CO2 excess limit follows the category and, under the German annex, the absolute limit of the usage
    Given the section "co2"
    When the permitted excess above the outdoor concentration is evaluated
    Then every value equals the oracle value within the absolute tolerance 1e-9

  @id-pmv-checks
  @level-quick
  @mode-differential
  Scenario: the PMV checks of the example documents report the oracle vote
    Given the example documents compliant office, residential method 3 and non-compliant office
    When the summer and winter PMV checks of every zone are evaluated
    Then each check utilisation equals the absolute oracle vote of its conditions over the category limit within the absolute tolerance 1e-6
