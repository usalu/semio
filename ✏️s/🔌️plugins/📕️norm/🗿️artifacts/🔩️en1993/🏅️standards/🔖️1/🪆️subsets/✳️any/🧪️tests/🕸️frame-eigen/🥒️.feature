@capability-en1993-1-evaluate
@oracle-en1993-1-compliance-independent
@comparison-numerical-v1
Feature: EN 1993-1-1 5.2.1 elastic critical load factor of a general planar frame
  The independent oracle `🔮️oracles/🕸️frame-eigen/🐍️.py` (numpy and scipy: Hermite beam-column assembly with consistent geometric stiffness, solved with scipy.linalg.eigh, plus the stability-function closed form solved with scipy.optimize.brentq) and the mechanics-kernel eigenvalue of `🕸️frame-eigen` agree on alpha_cr.
  Frame nodes lie in the x-z plane (z up). The portal has fixed bases, a height of 4.0 m, a span of 6.0 m, HEB 240 (I_y 1.126e-4 m4, A 0.0106 m2, E 210 GPa) for all bars and a design load of -200000 N in z on each eaves node. The cantilever column is 4.0 m high with a design load of -100000 N in z on its free top. The loaded portal carries all design loads multiplied by 20.
  The member rows compare the Euler load of one prismatic member (length 5 m, E 210 GPa, I 8e-5 m4, so E I 16.8e6 N m2) from the mechanics `member_modes::buckling` with pi squared E I over the square of the effective length (pinned 1.0, cantilever 2.0, fixed-guided 0.5).
  Out-of-plane degrees of freedom are restrained; every bar is solved with four elements.

  Further specification assertions, asserted by the Rust unit tests of the artifact and not by the platform adapters:
  - A slender frame fails the check en1993.1-1.5.2.frame.bar-1 (the portal with all design loads multiplied by 20, alpha_cr 2.438) and the failing check carries one remedy on "frameNodes[id=node-b].loadZ".

  @id-clause-vector
  @level-quick
  @mode-differential
  Scenario Outline: <quantity> of the frame equals the oracle value
    Given the clause vector "<vector>" of "EN 1993-1-1"
    When the quantity "<quantity>" is evaluated
    Then it equals the independent oracle value within relative tolerance <tolerance>
    Examples:
      | id                 | vector     | quantity         | tolerance |
      | portal-sway        | portal     | portalLowest     | 1e-2      |
      | portal-loaded-20   | portal     | portalLoaded20   | 1e-2      |
      | portal-closed-form | portal     | portalClosedForm | 2e-2      |
      | cantilever-euler   | cantilever | cantileverEuler  | 5e-3      |
      | cantilever-lowest  | cantilever | cantileverLowest | 5e-3      |
      | member-pinned      | member-pinned     | ncrPinned     | 5e-3 |
      | member-cantilever  | member-cantilever | ncrCantilever | 5e-3 |
      | member-fixed       | member-fixed      | ncrFixed      | 5e-3 |

  Internal forces along the bars. The demo portal carries 20 kN in x on the left eaves and 200 kN down on both eaves; first-order peak moment, shear and axial force of the left column (bar-1) and the beam (bar-2) from the mechanics kernel equal the same quantities from the third-party solvers PyNite (linear analysis) and anastruct (the oracle names carry the solver).

  @id-clause-vector
  @level-quick
  @mode-differential
  Scenario Outline: <quantity> of the demo portal equals the third-party solver value
    Given the clause vector "<vector>" of "EN 1993-1-1"
    When the quantity "<quantity>" is evaluated
    Then it equals the independent oracle value within relative tolerance <tolerance>
    Examples:
      | id                     | vector       | quantity               | tolerance |
      | column-moment-pynite   | portal-forces | columnMomentPyNite     | 5e-3      |
      | column-shear-pynite    | portal-forces | columnShearPyNite      | 5e-3      |
      | column-axial-pynite    | portal-forces | columnAxialPyNite      | 5e-3      |
      | beam-moment-pynite     | portal-forces | beamMomentPyNite       | 5e-3      |
      | beam-shear-pynite      | portal-forces | beamShearPyNite        | 5e-3      |
      | column-moment-anastruct | portal-forces | columnMomentAnastruct | 5e-3      |
      | column-axial-anastruct | portal-forces | columnAxialAnastruct   | 5e-3      |
      | beam-moment-anastruct  | portal-forces | beamMomentAnastruct    | 5e-3      |

  Internal forces per load case and per combination. The demo portal of `frame_cases_showcase` (HEB 240, fixed bases) is loaded by the frame actions of three load cases: permanent 20 kN/m and imposed 15 kN/m uniform on the beam (bar-2) and wind 12 kN at the left eaves node; the fourth vector combines them with the TEST coefficients 1.0, 0.5 and 0.25 (multipliers that exercise the solver on a combined vector, not EN 1990 values; the EN 1990 factors are asserted by the Rust unit tests against `combine_scalar_forces`). The peak first-order bending moment of the left column and of the beam from PyNite (linear `analyze`) and anastruct must equal the Rust `frame_envelope::second_order` first-order course of the same load vector.

  @id-clause-vector
  @level-quick
  @mode-differential
  Scenario Outline: <quantity> of the load-case portal equals the third-party solver value
    Given the clause vector "<vector>" of "EN 1993-1-1"
    When the quantity "<quantity>" is evaluated
    Then it equals the independent oracle value within relative tolerance <tolerance>
    Examples:
      | id | vector | quantity | tolerance |
      | g-column-moment-pynite | load-cases | caseGColumnMomentPyNite | 5e-3 |
      | g-column-moment-anastruct | load-cases | caseGColumnMomentAnastruct | 5e-3 |
      | g-beam-moment-pynite | load-cases | caseGBeamMomentPyNite | 5e-3 |
      | g-beam-moment-anastruct | load-cases | caseGBeamMomentAnastruct | 5e-3 |
      | q-column-moment-pynite | load-cases | caseQColumnMomentPyNite | 5e-3 |
      | q-column-moment-anastruct | load-cases | caseQColumnMomentAnastruct | 5e-3 |
      | q-beam-moment-pynite | load-cases | caseQBeamMomentPyNite | 5e-3 |
      | q-beam-moment-anastruct | load-cases | caseQBeamMomentAnastruct | 5e-3 |
      | w-column-moment-pynite | load-cases | caseWColumnMomentPyNite | 5e-3 |
      | w-column-moment-anastruct | load-cases | caseWColumnMomentAnastruct | 5e-3 |
      | w-beam-moment-pynite | load-cases | caseWBeamMomentPyNite | 5e-3 |
      | w-beam-moment-anastruct | load-cases | caseWBeamMomentAnastruct | 5e-3 |
      | combo-column-moment-pynite | load-cases | caseComboColumnMomentPyNite | 5e-3 |
      | combo-column-moment-anastruct | load-cases | caseComboColumnMomentAnastruct | 5e-3 |
      | combo-beam-moment-pynite | load-cases | caseComboBeamMomentPyNite | 5e-3 |
      | combo-beam-moment-anastruct | load-cases | caseComboBeamMomentAnastruct | 5e-3 |

  @id-clause-vector
  @level-quick
  @mode-differential
  Scenario Outline: <quantity> of the sway imperfection of EN 1993-1-1 5.3.2 equals the third-party value
    Given the clause vector "<vector>" of "EN 1993-1-1"
    When the quantity "<quantity>" is evaluated
    Then it equals the independent oracle value within relative tolerance <tolerance>
    Examples:
      | id | vector | quantity | tolerance |
      | sway-phi | sway-imperfection | swayPhi | 1e-9 |
      | sway-force | sway-imperfection | swayForce | 1e-9 |
      | sway-nodes-h20 | sway-imperfection | swayNodesH20 | 1e-9 |
      | sway-nodes-h59 | sway-imperfection | swayNodesH59 | 1e-9 |
      | sway-nodes-h61 | sway-imperfection | swayNodesH61 | 1e-9 |
      | sway-column-moment-pynite | sway-imperfection | swayColumnMomentPyNite | 5e-3 |
      | sway-column-moment-anastruct | sway-imperfection | swayColumnMomentAnastruct | 5e-3 |
      | sway-beam-moment-pynite | sway-imperfection | swayBeamMomentPyNite | 5e-3 |
      | sway-beam-moment-anastruct | sway-imperfection | swayBeamMomentAnastruct | 5e-3 |
