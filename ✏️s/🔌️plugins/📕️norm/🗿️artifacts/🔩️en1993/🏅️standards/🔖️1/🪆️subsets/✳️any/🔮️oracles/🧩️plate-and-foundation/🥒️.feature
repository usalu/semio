@oracle-en1993-1-plates-independent
@comparison-numerical-v1
Feature: EN 1993 joint plates and foundation blocks
  The checks of the fin plate, the splice cover plates and the foundation block follow independent closed forms; the shear and bearing resistance of the bolts of Table 3.4 are also computed by the third-party blue-prints library.

  Scenario Outline: worked examples of the special parts showcase
    Given the joint plate or foundation block "<entity>" of the special parts showcase
    When the check "<check>" is evaluated
    Then its utilisation equals <utilisation> within 1e-9

    Examples:
      | entity       | check                                    | utilisation        |
      | plate-fin    | en1993.1-8.3.6.plate.plate-fin           | 0.1731315939472695 |
      | plate-fin    | en1993.1-8.3.10.plateblock.plate-fin     | 0.0965502026989369 |
      | plate-fin    | en1993.1-8.4.5.plateweld.plate-fin       | 0.0819398772863447 |
      | plate-flange | en1993.1-8.3.6.plate.plate-flange        | 0.6355307444352929 |
      | plate-flange | en1993.1-1.6.2.3.platenet.plate-flange   | 0.4854026416854391 |
      | block-bp     | en1993.1-8.6.2.grout.block-bp            | 0.5                |
      | block-bp     | en1993.1-8.6.2.bearing.block-bp          | 0.2426534843882751 |
      | block-bp     | en1993.1-8.6.2.anchort.block-bp          | 0.1229540446962543 |
      | block-bp     | en1993.1-8.6.2.anchorv.block-bp          | 0.2677225166773279 |
      | block-bp     | en1993.1-8.6.2.cbend.block-bp            | 0.16513007058341486 |
      | block-bp     | en1993.1-8.6.2.tbend.block-bp            | 0.1229540446962543 |
      | block-bp     | en1993.1-8.6.2.mjrd.block-bp             | 0.3592883216923917 |
      | block-bp     | en1993.1-8.6.2.cone.block-bp             | 0.1802984191225749 |

  Scenario: a thin base plate governs in T-stub mode 1 with prying
    Given the joint plate or foundation block "block-bp" of the special parts showcase with the base plate thickness 12 mm
    When the check "en1993.1-8.6.2.tbend.block-bp" is evaluated
    Then its utilisation equals 0.18630300320441165 within 1e-9

  Scenario: a plate wider than the flange T-stub footprint bears on b_fc + 2c and not on its own width
    Given the joint plate or foundation block "block-bp" of the special parts showcase with the plate width 450 mm
    When the check "en1993.1-8.6.2.bearing.block-bp" is evaluated
    Then its utilisation equals 0.21865450222195487 within 1e-9

  Scenario: the required c of a wide plate solves (t_fc + 2c)(b_fc + 2c) = C_Ed / f_jd
    Given the joint plate or foundation block "block-bp" of the special parts showcase with the plate width 450 mm
    When the check "en1993.1-8.6.2.cbend.block-bp" is evaluated
    Then its utilisation equals 0.1802761683667285 within 1e-9

  Scenario: a column base fully in compression splits the force over both flange T-stubs
    Given the joint plate or foundation block "block-bp" of the special parts showcase with the base moment 20 kNm
    When the check "en1993.1-8.6.2.bearing.block-bp" is evaluated
    Then its utilisation equals 0.1502140617641703 within 1e-9

  Scenario: a short anchor embedment fails the concrete cone breakout
    Given the joint plate or foundation block "block-bp" of the special parts showcase with the anchor embedment 50 mm
    When the check "en1993.1-8.6.2.cone.block-bp" is evaluated
    Then the check fails and carries one remedy
