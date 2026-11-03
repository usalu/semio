@oracle-en1993-1-3-fasteners-independent
@comparison-numerical-v1
Feature: EN 1993-1-3 8.3 screws and blind rivets of cold-formed sheets
  The resistances follow the registry coefficients (unverified) and are evaluated a second time with numpy interpolation.

  Scenario: showcase joint of four self-tapping screws
    Given d 5.5 mm, washer 16 mm, sheets 0.75 and 1.5 mm of 360 MPa, 4 screws with 2 across, width 100 mm and gamma_M2 1.25
    When the shear 4000 N and the tension 1200 N are checked
    Then the bearing resistance equals 3002.3446121070315 N, the pull-over 5184 N, the pull-out 1069.2 N and the net section 19224 N
    And the shear utilisation equals 0.3330730243182193, the tension 0.2805836139169473 and the interaction 0.6136566382351666

  Scenario: bearing by thickness ratio
    Given a screw of 5.5 mm in a 1.0 mm sheet of 360 MPa on a 2.5 mm support
    Then the bearing resistance equals 5068.8 N
    And a blind rivet of 4.8 mm in equal 1.0 mm sheets of 400 MPa has 2523.9055449838056 N
