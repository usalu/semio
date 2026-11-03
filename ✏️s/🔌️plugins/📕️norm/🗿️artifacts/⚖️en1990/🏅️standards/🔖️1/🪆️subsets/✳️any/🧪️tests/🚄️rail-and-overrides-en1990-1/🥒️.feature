@capability-en1990-1-evaluate
@oracle-en1990-1-scipy
@comparison-numerical-v1
Feature: EN 1990 factor overrides, E90.10 to E90.12 and rail frequency limits
  Rows of the v4 features. Committed documents under `shared://🚄️rail-and-overrides/` are evaluated by the production report and by the independent Python restatement in `🐍️.py` (override policy of the annex, X_d = eta X_k / gamma_m, a_d = a_nom +- delta a, alpha_cr against the first-order limit); the Fig. 6.10 limits are read from the production checks crate and from the closed form restated in `🐍️.py`. Every normative coefficient behind them is an unverified registry value.
  Further specification assertions, asserted by the Rust unit tests of the compliance report and not by the platform adapters:
  - an applied override reaches the generator: Given a document with an applied psi0 override on "Q-wind" When the combinations are generated Then a term of "Q-wind" deviates from the National Annex and the explanation marks it, while a refused row leaves every term at the annex value
  - peak mid-span acceleration under HSLM-A1: Given a deck of span 20 m, mass 5000 kg/m and natural frequency 4 Hz When the train HSLM-A1 crosses at 72 m/s with damping ratio 0.01 Then the peak mid-span acceleration is 74.9358808371768 m/s2 within relative tolerance 0.06 of the scipy DOP853 modal integration (49.1885636247798 at damping 0.02, 3.8894305414716124 at 54 m/s); the 6 percent bound of Newmark and the 16-element kernel against the analytic sine modes is wider than the 0.5 percent of the platform profile, so the rows stay with the unit test

  @id-check-outcome
  @level-quick
  @mode-differential
  Scenario Outline: <check> reports <status> on <fixture>
    Given the committed document <fixture>
    When the check "<check>" is evaluated
    Then its status is "<status>" with utilisation <utilisation> within relative tolerance <tolerance>
    Examples:
      | id | fixture | check | status | utilisation | tolerance |
      | psi0-wind-de-applied | shared://🚄️rail-and-overrides/override-psi0-de/🔣️.json | en1990.override.O1 | warning | 0 | 1e-9 |
      | gamma-office-en-applied | shared://🚄️rail-and-overrides/override-gamma-en/🔣️.json | en1990.override.O1 | warning | 0 | 1e-9 |
      | gamma-office-de-refused | shared://🚄️rail-and-overrides/override-gamma-de/🔣️.json | en1990.override.O1 | fail | 0 | 1e-9 |
      | k-fi-en-refused | shared://🚄️rail-and-overrides/override-kfi-en/🔣️.json | en1990.override.O1 | fail | 0 | 1e-9 |
      | declared-300-mpa | shared://🚄️rail-and-overrides/material-fail/🔣️.json | en1990.e10.material.beam-B1 | fail | 1.0328638497652582 | 1e-9 |
      | declared-280-mpa | shared://🚄️rail-and-overrides/material-pass/🔣️.json | en1990.e10.material.beam-B1 | pass | 0.9640062597809077 | 1e-9 |
      | deviation-4-mm | shared://🚄️rail-and-overrides/geometry-fail/🔣️.json | en1990.e11.geometry.beam-B1 | fail | 1.25 | 1e-6 |
      | deviation-10-mm | shared://🚄️rail-and-overrides/geometry-pass/🔣️.json | en1990.e11.geometry.beam-B1 | pass | 0.5 | 1e-6 |
      | first-order-alpha-8 | shared://🚄️rail-and-overrides/analysis-fail/🔣️.json | en1990.e12.analysis.beam-B1 | fail | 1.25 | 1e-9 |
      | first-order-alpha-12 | shared://🚄️rail-and-overrides/analysis-pass/🔣️.json | en1990.e12.analysis.beam-B1 | pass | 0.8333333333333334 | 1e-9 |
      | second-order-alpha-8 | shared://🚄️rail-and-overrides/analysis-second-order/🔣️.json | en1990.e12.analysis.beam-B1 | info | 0 | 1e-9 |

  @id-clause-vector
  @level-quick
  @mode-differential
  Scenario Outline: Fig. 6.10 <quantity> of <vector>
    Given the clause vector "<vector>" of "en1990-a2-fig-6-10"
    When the quantity "<quantity>" is evaluated
    Then it equals the independent oracle value within relative tolerance <tolerance>
    Examples:
      | id | vector | quantity | tolerance |
      | lower-10 | span-10 | lower-limit-hz | 1e-9 |
      | upper-10 | span-10 | upper-limit-hz | 1e-9 |
      | lower-40 | span-40 | lower-limit-hz | 1e-9 |
      | upper-40 | span-40 | upper-limit-hz | 1e-9 |
