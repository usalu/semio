@capability-en1997-1-ground-derivation
@oracle-en1997-1-ground-derivation
@comparison-numerical-v1
Feature: EN 1997 derived characteristic undrained strength from CPT readings
  The undrained strength c_u of a layer is derived from its CPT cone resistance readings (EN 1997-2 Annex D, GI-03):
  c_u = (q_t - sigma_v0) / N_kt per reading inside the layer, with sigma_v0 from the unit weights and the water table.
  The samples of the statistics are independent values, one per investigation point: the mean of the readings of one
  sounding inside the layer (spatial averaging; the readings of one CPT are strongly autocorrelated and must not be
  counted as samples). With n >= 3 soundings the characteristic value is the statistical X_m (1 - k_n V_x) of
  EN 1997-1 2.4.5.2 with the Student-t factor of n - 1 degrees of freedom; with n < 3 soundings it is the lowest reading
  (cautious estimate). The sample rule is the unverified registry row ndp.en1997.2.characteristic-samples-per-sounding.
  The independent Python oracle (`🔮️oracles/🎯️ground-derivation/🐍️.py`) derives the same values from the committed
  worked examples; the mean and the characteristic value of each layer must agree with the blessed numbers.

  @id-published-example
  @level-quick
  @mode-differential
  Scenario Outline: Derived <quantity> of worked example <example>
    Given the worked example "<example>" with its published inputs
    When the quantity "<quantity>" is evaluated
    Then it equals <expected> within relative tolerance <tolerance>
    Examples:
      | id                       | example                 | quantity                            | expected        | tolerance |
      | clay-3-soundings-samples | worked-clay-silt-sand   | clay-1.undrainedStrength.n          | 3               | 0.000001  |
      | clay-3-soundings-readings | worked-clay-silt-sand  | clay-1.undrainedStrength.readings   | 10              | 0.000001  |
      | clay-3-soundings-mean    | worked-clay-silt-sand   | clay-1.undrainedStrength.mean       | 42077.777777778 | 0.000001  |
      | clay-3-soundings-kn      | worked-clay-silt-sand   | clay-1.undrainedStrength.kn         | 1.685854461     | 0.000001  |
      | clay-3-soundings-characteristic | worked-clay-silt-sand | clay-1.undrainedStrength.characteristic | 39367.368991363 | 0.000001  |
      | silt-2-soundings-characteristic | worked-clay-silt-sand | silt-1.undrainedStrength.characteristic | 28000.000000001 | 0.000001  |
      | sand-3-soundings-characteristic | worked-clay-silt-sand | sand-1.frictionAngle.characteristic | 40.188778164    | 0.000001  |
      | clay-1-sounding-samples  | clay-two-cpt-readings   | clay-1.undrainedStrength.n          | 1               | 0.000001  |
      | clay-1-sounding-mean     | clay-two-cpt-readings   | clay-1.undrainedStrength.mean       | 41000.0         | 0.000001  |
      | clay-1-sounding-characteristic | clay-two-cpt-readings | clay-1.undrainedStrength.characteristic | 40000.0         | 0.000001  |
