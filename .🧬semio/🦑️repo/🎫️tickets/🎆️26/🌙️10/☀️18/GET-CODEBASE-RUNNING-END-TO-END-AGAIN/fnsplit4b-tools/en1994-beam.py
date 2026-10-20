import sys, json
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import phspec
F = "/Users/ueli/Documents/semio/" + sys.argv[2]
types = {"annex": "AnnexChoice", "f_y": "f64", "beam": "CompositeBeam", "b0": "f64", "b_eff": "f64", "b_uncapped": "f64", "uls": "part_en1990::DesignEffects", "uls_c": "part_en1990::DesignEffects",
         "sls_char": "part_en1990::DesignEffects", "sls_freq": "part_en1990::DesignEffects", "sls_qp": "part_en1990::DesignEffects", "m_ed": "f64", "v_ed": "f64", "v_l_ed": "f64"}
modes = {"annex": "val", "f_y": "val", "b0": "val", "b_eff": "val", "b_uncapped": "val", "m_ed": "val", "v_ed": "val", "v_l_ed": "val", "beam": "ref"}
s = phspec.build(F, "evaluate_beam", [
 ("check_beam_effective_width_and_class", "Checks the effective width and the steel section class of a composite beam.", 114, 162),
 ("check_beam_flexure_and_shear_connection", "Checks the plastic flexure, the degree of shear connection and the longitudinal shear of a composite beam.", 164, 296, {"allow_control": [201]}),
 ("check_beam_vertical_shear_and_stud_resistance", "Checks the vertical shear and the stud resistance of a composite beam.", 298, 387, {"allow_control": [316, 430]}),
 ("check_beam_serviceability_stresses", "Checks the elastic stresses and the deflections of a composite beam.", 460, 501),
 ("check_beam_cracking_and_vibration", "Checks the cracking and the vibration of a composite beam.", 503, 592),
], types=types, modes=modes, keep="auto", nth=0, out=None, allow_reason="the extracted check keeps the composite beam values of its original function as explicit parameters")
json.dump(s, open(sys.argv[1], "w", encoding="utf-8"), ensure_ascii=False, indent=1)
