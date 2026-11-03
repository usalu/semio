@capability-en1993-1-evaluate
@oracle-en1993-1-compliance-independent
@comparison-numerical-v1
Feature: EN 1993 catalogue-gap checks reproduce the independent oracle values
  The independent oracle `🔮️oracles/🧾️vids/🐍️.py` (numpy closed forms, plus the third-party library `sectionproperties`, which recomputes the torsion constant and the warping constant of the HEB 240 outline by finite elements) and the Rust checks of `🧱️member-checks/🧾️vid-checks` agree.
  The subject is the compliant HEB 240 beam-column of 4.0 m (S355, fu 510 MPa, bolted joint M20 8.8 in a 2 x 2 group with p 60 mm, e 40 mm, t 10 mm) and, for the flange curling, the cold-formed member cf-1 of the special-parts showcase (flange 60 mm, web 150 mm, t 3 mm, flange stress 150 MPa).

  Further specification assertions, asserted by the Rust unit tests of the artifact and not by the platform adapters:
  - Every V-id of the EN 1993 catalogue maps to a check family produced by an example document, or is marked not modelled with a reason (test `every_catalogue_id_is_traced_to_a_produced_check_family`).
  - A joint plate that is too thin fails en1993.1-8.3.10.block with a remedy on the plate thickness.

  @id-clause-vector
  @level-quick
  @mode-differential
  Scenario Outline: <quantity> equals the oracle value
    Given the clause vector "<vector>" of "EN 1993-1-1"
    When the quantity "<quantity>" is evaluated
    Then it equals the independent oracle value within relative tolerance <tolerance>
    Examples:
      | id                       | vector  | quantity                         | tolerance |
      | ncr-t-catalogue          | heb240  | ncrTCatalogue                    | 1e-9      |
      | ncr-t-sectionproperties  | heb240  | ncrTSectionproperties            | 5e-3      |
      | torsion-constant-fe      | heb240  | torsionConstantSectionproperties | 1e-3      |
      | warping-constant-fe      | heb240  | warpingConstantSectionproperties | 1e-3      |
      | block-tearing            | joint   | blockTearing                     | 1e-9      |
      | flange-curling           | cf      | flangeCurling                    | 1e-9      |
      | fatigue-limit-cat-71     | detail  | fatigueLimitCategory71           | 1e-12     |
      | ltb-fire-560             | fire    | ltbFire560                       | 1e-9      |
