@oracle-en1993-1-8-bolt-group-ic-independent
@comparison-numerical-v1
Feature: EN 1993-1-8 3.12 bolt group by the instantaneous centre of rotation
  The plastic distribution about the instantaneous centre with the load-deformation law R = R_ult (1 - exp(-10 delta))^0.55, delta_max 8.64 mm, is solved as a three-unknown nonlinear system with scipy fsolve and agrees with the bracketed search used by the implementation; the elastic vector method follows the polar moment.

  Scenario Outline: load factor of an eccentric bolt group
    Given a bolt group of <rows> rows and 2 bolts per row with pitch <pitch> mm and gauge <gauge> mm and R_ult 100000 N
    When the load (<fx> N, 1000 N) acts at the eccentricity <e> mm
    Then the elastic largest bolt force equals <elastic> N and the instantaneous centre load factor equals <factor> within 1e-6

    Examples:
      | rows | pitch | gauge | fx    | e     | elastic            | factor             |
      | 3    | 75.0  | 100.0 | 0.0   | 150.0 | 473.75568011839647 | 249.99999612305191 |
      | 4    | 80.0  | 90.0  | 0.0   | 100.0 | 234.92265367799646 | 536.1776768649606  |

  Scenario: inclined load on a 3 x 2 group
    Given a bolt group of 3 rows and 2 bolts per row with pitch 75 mm and gauge 100 mm and R_ult 100000 N
    When the load (300 N, 1000 N) acts with the moment 120000 N mm about the centroid
    Then the elastic largest bolt force equals 436.81931174240805 N and the instantaneous centre load factor equals 294.0790129777767 within 1e-6

  Scenario: showcase fin plate with three rows of two M20 8.8 bolts
    Given pitch 70 mm, gauge 100 mm, R_ult min(F_v,Rd 94080 N, F_b,Rd) = 94080 N and the shear 30000 N at 150 mm
    When the group is solved
    Then the elastic largest bolt force equals 14669.701439774408 N and the load factor equals 7.591173736544717 within 1e-6 and the plastic gain over the elastic factor is 1.1836761510689595

  Scenario: pure moment on two bolts
    Given two bolts 100 mm apart with R_ult 100000 N
    When only a moment acts
    Then the ultimate moment equals 10000000 N mm
