@capability-en1990-1-evaluate
@oracle-en1990-1-compliance-independent
@comparison-numerical-v1
Feature: Evaluate the EN 1990 verification catalogue against an independent Python formulation
  Every catalogue entry E90.01 to E90.30 of the EN 1990 target specification (clauses 2 to 6, Annexes A1, A2, B, C and D)
  is evaluated by the Rust kernel on a committed input case and by `🔮️oracles/⚖️compliance/🐍️.py catalogue`, which
  re-derives the same quantities with a different formulation: combinations by brute-force enumeration of every
  exclusive-group and leading-action choice with itertools, Phi and its inverse from scipy.stats, the Gumbel,
  lognormal and Weibull fractiles from scipy.stats, FORM as a constrained minimisation of the design point in standard
  normal space (cross-checked against the exact failure probability by quadrature), Palmgren-Miner point by point on
  the EN 1993-1-9 S-N curve. Both read the same committed cases at `shared://📇️catalogue/🔣️.json`; every
  named result must agree within 1e-7 relative (2e-4 for FORM).

  Further specification assertions, asserted by the Rust unit tests of the artifact and not by the platform adapters:
  - The worked examples of the specification are reproduced: Given actions G = 50, Q1 = 20 (office), W = 10 (wind), S = 8 (snow) and the NA-DE annex When the kernel evaluates E90.16 Then Eq. 6.10 gives 112.5 with the office action leading, and for the EN annex Eq. 6.10a gives 103.5 and Eq. 6.10b gives 102.375
  - NA-DE offers Eq. 6.10 only: Given the same actions and the NA-DE annex When the kernel evaluates E90.16 Then no result for Eq. 6.10a or Eq. 6.10b is produced and the simplified bound is reported
  - Exclusive traffic groups enter a bridge combination at most once: Given gr1a (tandem system and uniform load) and gr1b (single axle) as distinct exclusive groups and wind as an independent action When the kernel evaluates E90.26 Then the governing Eq. 6.10 contains exactly one traffic group and the group count is reported as 2
  - The snapshot report is reproduced by brute-force enumeration: Given the default, high-consequence office, bridge, accidental/seismic and fatigue examples in both annexes When the compliance report is evaluated and the Python oracle recomputes every combination check Then the computed effect of every STR, GEO, EQU, accidental, seismic, serviceability, deflection and fatigue check agrees within 1e-9
  - Annex A2 bridges use Eq. 6.10 only: Given a road bridge in either annex When the compliance report is evaluated Then no Eq. 6.10a or 6.10b check exists, while an EN building still reports both
  - Traffic load groups are mutually exclusive: Given gr1a and gr1b traffic actions without or with a shared group id When the report is evaluated Then the traffic-group check warns with the group remedy, and passes once each load group carries its own id
  - Execution actions act in the transient situation only: Given a construction load Q_c on a bridge When the persistent Eq. 6.10 and the transient Eq. 6.10 are evaluated Then Q_c never enters the persistent combination and the transient check reproduces 1.35 G - 1.0 G_inf + 1.5 Q_c
  - Declared A1.4 and A2.4 limits are compared with the NDP recommendations: Given L/150, a 2 Hz floor and a rail bridge with a 9 m/s2 acceleration limit When the report is evaluated Then each deviation is a warning with an at-least or at-most remedy
  - Annex C and D are complete: Given a FORM analysis and a test series with n = 2 When the report is evaluated Then the C.7 sensitivity factors and the partial factors gamma_R and gamma_E are reported (scipy.stats fractiles) and n below 3 warns
  - Annex C covers permanent and variable actions together: Given R lognormal, G normal and Q Gumbel When E90.29 runs FORM on g = R - G - Q Then beta, the sensitivity factors, the design points and the implied gamma_R, gamma_G, gamma_Q agree with the constrained scipy.optimize minimisation in standard normal space, and for three normal variables beta equals the closed form (mu_R - mu_G - mu_Q) / sqrt(s_R^2 + s_G^2 + s_Q^2)
  - Annex D standard evaluation (D.7.2): Given paired test and model resistances When E90.30 receives them as `pairs` Then b, V_delta, V_r, k_n and the characteristic and design resistance agree with the numpy least-squares and log-error computation
  - Table D1 is reproduced from the Student t distribution: Given scipy.stats.t.ppf(0.95, n-1) sqrt(1+1/n) and norm.ppf(0.95) sqrt(1+1/n) When compared with every printed node of k_n Then the deviation is below 0.02
  - NA-DE psi completeness: Given the categories settlement, other variable actions, rail wind, HSLM and the stair and access categories T and Z When the psi table is read Then each row exists, is ordered and follows the registry

  @id-catalogue-fixture-case
  @level-quick
  @mode-differential
  Scenario Outline: Evaluate <entry> case <index> and compare every named result
    Given the committed catalogue case shared://📇️catalogue/🔣️.json at index <index>
    When both implementations evaluate catalogue entry <entry> on that case
    Then every result named by the oracle agrees within the comparison profile
    Examples:
      | id        | index | entry  | case                   |
      | e90-01-0  | 0     | E90.01 | use-building           |
      | e90-01-1  | 1     | E90.01 | use-bridge             |
      | e90-02-2  | 2     | E90.02 | kfi-rc3                |
      | e90-02-3  | 3     | E90.02 | kfi-override           |
      | e90-03-4  | 4     | E90.03 | dwl-cat2               |
      | e90-03-5  | 5     | E90.03 | dwl-cat4               |
      | e90-04-6  | 6     | E90.04 | durability             |
      | e90-05-7  | 7     | E90.05 | dsl-il                 |
      | e90-06-8  | 8     | E90.06 | situations             |
      | e90-07-9  | 9     | E90.07 | classes                |
      | e90-08-10 | 10    | E90.08 | g-and-q                |
      | e90-08-11 | 11    | E90.08 | g-single               |
      | e90-09-12 | 12    | E90.09 | psi-office             |
      | e90-09-13 | 13    | E90.09 | psi-snow-high          |
      | e90-09-14 | 14    | E90.09 | psi-custom             |
      | e90-10-15 | 15    | E90.10 | xd-steel               |
      | e90-10-16 | 16    | E90.10 | xd-gm                  |
      | e90-11-17 | 17    | E90.11 | geometry               |
      | e90-12-18 | 18    | E90.12 | analysis               |
      | e90-13-19 | 19    | E90.13 | test-based             |
      | e90-14-20 | 20    | E90.14 | limit-states           |
      | e90-15-21 | 21    | E90.15 | equ-worked             |
      | e90-15-22 | 22    | E90.15 | equ-bridge             |
      | e90-16-23 | 23    | E90.16 | str-de                 |
      | e90-16-24 | 24    | E90.16 | str-en                 |
      | e90-16-25 | 25    | E90.16 | str-en-rc3             |
      | e90-16-26 | 26    | E90.16 | geo-set-c              |
      | e90-16-27 | 27    | E90.16 | str-altitude           |
      | e90-17-28 | 28    | E90.17 | accidental             |
      | e90-17-29 | 29    | E90.17 | accidental-psi2        |
      | e90-18-30 | 30    | E90.18 | seismic                |
      | e90-19-31 | 31    | E90.19 | sls                    |
      | e90-20-32 | 32    | E90.20 | partial                |
      | e90-21-33 | 33    | E90.21 | fatigue                |
      | e90-22-34 | 34    | E90.22 | psi-a1                 |
      | e90-22-35 | 35    | E90.22 | psi-a1-snow            |
      | e90-23-36 | 36    | E90.23 | sets-building          |
      | e90-23-37 | 37    | E90.23 | sets-bridge            |
      | e90-24-38 | 38    | E90.24 | sls-a14                |
      | e90-24-39 | 39    | E90.24 | sls-cantilever         |
      | e90-25-40 | 40    | E90.25 | psi-a2-temperature     |
      | e90-25-41 | 41    | E90.25 | psi-a2-temperature-en  |
      | e90-25-42 | 42    | E90.25 | psi-rail               |
      | e90-26-43 | 43    | E90.26 | bridge-groups          |
      | e90-26-44 | 44    | E90.26 | bridge-groups-en       |
      | e90-27-45 | 45    | E90.27 | footbridge             |
      | e90-27-46 | 46    | E90.27 | rail                   |
      | e90-28-47 | 47    | E90.28 | beta-rc2               |
      | e90-28-48 | 48    | E90.28 | beta-100a              |
      | e90-28-49 | 49    | E90.28 | beta-1a                |
      | e90-29-50 | 50    | E90.29 | form-normal            |
      | e90-29-51 | 51    | E90.29 | form-lognormal-gumbel  |
      | e90-29-52 | 52    | E90.29 | form-weibull-lognormal |
      | e90-30-53 | 53    | E90.30 | annex-d-unknown        |
      | e90-30-54 | 54    | E90.30 | annex-d-lognormal      |
      | e90-30-55 | 55    | E90.30 | annex-d-n3             |
      | e90-30-56 | 56    | E90.30 | annex-d-n2-known       |
      | e90-16-57 | 57    | E90.16 | bridge-610-only        |
      | e90-29-58 | 58    | E90.29 | form-two-actions-normal |
      | e90-29-59 | 59    | E90.29 | form-two-actions-gumbel |
      | e90-30-60 | 60    | E90.30 | annex-d-standard |
      | e90-30-61 | 61    | E90.30 | annex-d-standard-n4 |
      | e90-30-62 | 62    | E90.30 | annex-d2-known-n10 |
