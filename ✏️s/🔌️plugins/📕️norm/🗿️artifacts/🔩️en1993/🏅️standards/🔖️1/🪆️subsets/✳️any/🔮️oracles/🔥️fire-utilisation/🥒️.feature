@oracle-en1993-1-2-fire-utilisation-independent
@comparison-numerical-v1
Feature: EN 1993-1-2 utilisation over time in fire
  The unprotected steel temperature is integrated with scipy solve_ivp (RK45) against the standard fire; the degree of utilisation is mu_0 divided by k_y,theta of Table 3.1.

  Scenario Outline: unprotected section with A_m/V = 200 1/m and mu_0 = 0.5
    Given the standard fire and an unprotected steel section with A_m/V 200 1/m
    When the steel temperature is integrated for <minutes> minutes
    Then theta_a equals <theta> within 3 percent and k_y,theta follows Table 3.1

    Examples:
      | minutes | theta             | mu                 |
      | 5       | 290.9477752347565 | 0.5                |
      | 10      | 552.4103696512241 | 0.8096800762840564 |
      | 15      | 681.7811228181926 | 1.8266487987593756 |
      | 30      | 828.1725515208364 | 5.213018303314503  |
