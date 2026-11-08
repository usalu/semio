@oracle-en1993-1-joint-rows-independent
@comparison-numerical-v1
Feature: EN 1993 end plate joint components and the compression zone
  The component method of the end plate joint of the special parts showcase (rows row-1 and row-2, M_j,Ed = 80 kNm) follows the independent closed forms of 🐍️.py: the column flange and the end plate in bending as T-stubs (EN 1993-1-8 6.2.6.4, 6.2.6.5), the column web in tension (6.2.6.3) per bolt row, and the compression zone F_c,Ed = M_j,Ed/z against the column web in compression (6.2.6.2), the web panel in shear (6.2.6.1) and the beam flange and web in compression F_c,fb,Rd = M_c,Rd/(h - t_fb) (6.2.6.7).
  The beam web in tension (6.2.6.8) is a kernel only: F_t,wb,Rd = 210 mm * 7.1 mm * 355 MPa / 1.0 = 529.305 kN.

  Scenario Outline: worked examples of the end plate joint
    Given the end plate joint of the special parts showcase
    When the check "<check>" is evaluated
    Then its utilisation equals <utilisation> within 1e-6

    Examples:
      | check                           | utilisation         |
      | en1993.1-8.6.2.row.row-1        | 0.77363079437377    |
      | en1993.1-8.6.2.row.row-2        | 0.4261645205053071  |
      | en1993.1-8.6.2.comp.joint-ep    | 0.8535036680184495  |

  Scenario: the plastic analysis gate follows the stable length of equation 6.67
    Given the special parts showcase with plastic analysis of the restrained member and a hinge to restraint distance of 14 m
    When the plastic analysis gate check of that member (EN 1993-1-1 5.6) is evaluated
    Then its utilisation is at least 14 / 11.529290821604922 within 1e-3

  Scenario: a 200 mm flange of S355 J2 fails the brittle fracture check
    Given the compliant HEB 240 frame with a flange thickness of 200 mm and the material S355 J2
    When the brittle fracture check of member-b1 (EN 1993-1-10 2.1) is evaluated
    Then its status is Fail
