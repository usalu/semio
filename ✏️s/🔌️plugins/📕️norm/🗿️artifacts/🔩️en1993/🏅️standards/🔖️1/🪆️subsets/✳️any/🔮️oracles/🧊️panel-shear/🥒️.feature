@oracle-en1993-1-5-panel-shear-independent
@comparison-numerical-v1
Feature: EN 1993-1-5 web panel shear buckling
  The Rayleigh-Ritz shear buckling coefficient of the simply supported panel is solved with scipy and agrees with the classical Southwell-Skan values; the resistance terms of 5.2 follow the closed forms.

  Scenario Outline: elastic shear buckling coefficient of the simply supported panel
    Given a simply supported web panel with the aspect ratio <alpha>
    When the Rayleigh-Ritz shear buckling coefficient is solved with 8 sine terms
    Then k_tau equals <k> within 1e-5 and stays within 2 % of the classical value <classical>

    Examples:
      | alpha | k      | classical |
      | 1.0   | 9.328349293898766 | 9.34      |
      | 1.5   | 7.0730549018388675 | 7.1       |
      | 2.0   | 6.548053338422909 | 6.6       |
      | 3.0   | 5.843302365885329 | 5.9       |

  Scenario: web only resistance of a 1000 x 8 web with panels of 2000 mm
    Given a web h_w 1000 mm, t 8 mm, f_y 355 MPa, a 2000 mm without longitudinal stiffener
    When V_b,Rd is evaluated with a non-rigid end post and gamma_M1 1.0
    Then k_tau equals 6.34 and V_bw,Rd equals 834183.8504981137 N and the cap equals 1967609.7173982447 N

  Scenario: flange contribution at half the flange moment
    Given the same web with flanges 300 x 20 mm, f_yf 355 MPa and M_Ed = 0.5 M_f,Rd
    When V_b,Rd is evaluated
    Then V_bf,Rd equals 58302.91970802919 N and V_b,Rd equals 892486.7702061429 N
