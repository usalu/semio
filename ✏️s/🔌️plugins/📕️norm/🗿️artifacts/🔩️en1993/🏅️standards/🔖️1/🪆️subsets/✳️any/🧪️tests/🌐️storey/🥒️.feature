@capability-en1993-1-evaluate
@oracle-en1993-1-compliance-independent
@comparison-numerical-v1
Feature: EN 1993-1-1 section 5 storey stability quantities
  The independent oracle `🔮️oracles/🌐️storey/🐍️.py` (stdlib formulas plus a scipy generalised eigenproblem) and the Rust clause functions of `🧮️clauses/🌐️global` agree on the elastic critical load factor alpha_cr of a storey, the sway imperfection of a two-column row and the stabilising force of a bracing system.
  The storey of the vectors carries H_Ed 20000 N, V_Ed 400000 N, a height of 4.0 m and a drift of 0.002 m (stiff) or 0.1 m (soft); the bracing carries sum N_Ed 1000000 N, m 2, L 6.0 m and delta_q 0.02 m.

  Further specification assertions, asserted by the oracle script itself and not by the platform adapters:
  - Third-party validation of the lateral stiffness model: Given a fixed-base guided column and a Hermite beam-column FE with 16 elements Then the critical load by scipy.linalg.eigh equals pi^2 EI / h^2 within 1e-5 And the first-order alpha_cr (H/V)(h/delta) of a two-column portal exceeds the FE alpha_cr by 24 / pi^2 / 2 = 1.2158517 (documented first-order approximation).
  - The check en1993.1-1.5.2.alpha reports the utilisation 3 / alpha_cr (0.03 for the stiff, 1.5 for the soft storey) and the check en1993.1-1.5.3.3.bracing reports 1.6666666666666667 against L/500; both are asserted by the Rust unit tests of the artifact.

  @id-clause-vector
  @level-quick
  @mode-differential
  Scenario Outline: <quantity> of the storey equals the oracle value
    Given the clause vector "<vector>" of "<part>"
    When the quantity "<quantity>" is evaluated
    Then it equals the independent oracle value within relative tolerance <tolerance>
    Examples:
      | id                       | vector | part        | quantity                 | tolerance |
      | alpha-showcase           | storey | EN 1993-1-1 | alpha_showcase           | 1e-9      |
      | alpha-soft               | storey | EN 1993-1-1 | alpha_soft               | 1e-9      |
      | phi-h4-m2                | storey | EN 1993-1-1 | phi_h4_m2                | 1e-12     |
      | phi-h4-m2-force          | storey | EN 1993-1-1 | phi_h4_m2_force          | 1e-9      |
      | qd-6m                    | storey | EN 1993-1-1 | qd_6m                    | 1e-9      |
      | alpha-portal-first-order | storey | EN 1993-1-1 | alpha_portal_first_order | 1e-9      |
