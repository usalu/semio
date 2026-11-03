@oracle-en1993-1-9-stress-range-independent
@comparison-numerical-v1
Feature: EN 1993-1-9 5.1 to 5.4 and 7.1 fatigue stress range of the load model
  The fibre stresses of two load cases, the compressive reduction, the size effect and the endurance on the S-N curve are evaluated closed form and with numpy.

  Scenario: showcase HEB 240 detail category 71
    Given the permanent case N 100000 N, M_y 40000 N m and the imposed case N 43333 N, M_y 17333 N m at z 0.12 m
    When the range is derived for a stress-relieved detail with 1e7 cycles
    Then sigma_g equals 52062736.686886296 Pa, sigma_q equals 22560132.54465632 Pa and the range equals 29502604.142229974 Pa
    And the endurance equals 87644897.97151105 cycles and the damage equals 0.11409677267523886

  Scenario: compressive part of a cycle
    Given sigma_max 60 MPa and sigma_min -20 MPa
    Then the range of a stress-relieved detail equals 72 MPa and the range of an as-welded detail equals 80 MPa
    And the range between -10 MPa and -50 MPa of a stress-relieved detail equals 24 MPa

  Scenario Outline: endurance of category 71
    Given the design range <range> MPa
    Then the endurance equals <n>

    Examples:
      | range | n                  |
      | 100   | 715821.9999999999  |
      | 40    | 19130593.49504685  |
      | 10    | none               |

  Scenario: size effect
    Given a plate of 40 mm
    Then k_s equals 0.9102821015130401
