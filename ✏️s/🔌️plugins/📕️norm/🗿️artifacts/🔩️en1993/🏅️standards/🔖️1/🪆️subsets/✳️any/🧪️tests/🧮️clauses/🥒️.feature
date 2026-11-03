@capability-en1993-1-evaluate
@oracle-en1993-1-compliance-independent
@oracle-en1993-1-rainflow
@comparison-numerical-v1
Feature: EN 1993 clause functions reproduce the worked examples E01 to E15 of the target specification
  The Python oracle (quadratic-root buckling factor, sine-series Galerkin M_cr, Heun fire integration; rainflow from the third-party `rainflow` package)
  and the Rust clause functions in 🧮️clauses must agree for every vector, in any implementation language.

  @id-clause-vector
  @level-quick
  @mode-differential
  Scenario Outline: <vector> <quantity> equals the oracle value
    Given the clause vector "<vector>" of "<part>"
    When the quantity "<quantity>" is evaluated
    Then it equals the independent oracle value within relative tolerance <tolerance>
    Examples:
      | id                                   | vector | part        | quantity             | tolerance |
      | e01-chi-z-en-1993-1-1                | E01    | EN 1993-1-1 | chi_z                | 1e-9      |
      | e01-nb-rd-z-na-de-en-1993-1-1        | E01    | EN 1993-1-1 | Nb_Rd_z_na_de        | 1e-9      |
      | e02-mcr-closed-form-en-1993-1-1      | E02    | EN 1993-1-1 | Mcr_closed_form      | 1e-9      |
      | e02-chi-lt-6-3-2-2-en-1993-1-1       | E02    | EN 1993-1-1 | chi_LT_6.3.2.2       | 1e-9      |
      | e02-chi-lt-mod-6-3-2-3-en-1993-1-1   | E02    | EN 1993-1-1 | chi_LT_mod_6.3.2.3   | 1e-9      |
      | e02-mcr-fe-uniform-en-1993-1-1       | E02    | EN 1993-1-1 | Mcr_fe_uniform       | 5e-3      |
      | e02-mcr-fe-udl-en-1993-1-1           | E02    | EN 1993-1-1 | Mcr_fe_udl           | 5e-3      |
      | e04-b-eff-en-1993-1-1                | E04    | EN 1993-1-1 | b_eff                | 1e-9      |
      | e07-v-bw-rd-en-1993-1-1              | E07    | EN 1993-1-1 | V_bw_Rd              | 1e-9      |
      | e08-f-b-rd-en-1993-1-1               | E08    | EN 1993-1-1 | F_b_Rd               | 1e-9      |
      | e10-f-t2-rd-en-1993-1-1              | E10    | EN 1993-1-1 | F_T2_Rd              | 1e-9      |
      | e11-damage-en-1993-1-1               | E11    | EN 1993-1-1 | damage               | 1e-9      |
      | e11-rainflow-cycles-en-1993-1-1      | E11    | EN 1993-1-1 | rainflow_cycles      | 1e-12     |
      | e11-rainflow-detail-en-1993-1-1      | E11    | EN 1993-1-1 | rainflow_cycles_detail | 1e-12   |
      | e12-theta-a-300s-en-1993-1-1         | E12    | EN 1993-1-1 | theta_a_300s              | 5e-3      |
      | e12-theta-a-600s-en-1993-1-1         | E12    | EN 1993-1-1 | theta_a_600s              | 5e-3      |
      | e12-theta-a-900s-en-1993-1-1         | E12    | EN 1993-1-1 | theta_a_900s              | 5e-3      |
      | e13-phi-sway-en-1993-1-1             | E13    | EN 1993-1-1 | phi_sway             | 1e-12     |
