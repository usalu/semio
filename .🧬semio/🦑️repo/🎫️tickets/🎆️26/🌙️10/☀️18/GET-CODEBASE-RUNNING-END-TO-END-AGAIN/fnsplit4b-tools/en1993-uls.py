import sys, json
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import phspec
F = "/Users/ueli/Documents/semio/" + sys.argv[2]
P = {"annex": "AnnexChoice", "report": "&mut CheckReport", "params": "AnnexParams", "member": "&SteelMember", "section": "&SteelSection", "material": "&SteelMaterial", "kind": "&str", "class": "u8",
     "a_res": "f64", "w_y": "f64", "w_z": "f64", "iy": "f64", "iz": "f64", "curve_y": "part_1_1::BucklingCurve", "curve_z": "part_1_1::BucklingCurve", "lambda_y": "f64", "lambda_z": "f64",
     "chi_y": "f64", "chi_z": "f64", "n_b_rd_y": "f64", "n_b_rd_z": "f64", "n_b_rd": "f64", "m_cr": "f64", "lambda_lt": "f64", "curve_lt": "part_1_1::BucklingCurve", "chi_lt": "f64", "m_b_rd": "f64",
     "combinations": "&[CombinedEffects]", "action_row": "&CombinedEffects", "a": "&semio_s_artifact_norm_en1993_model::DesignAction", "n_rd": "f64", "m_y_rd": "f64", "m_z_rd": "f64", "v_z_rd": "f64", "v_y_rd": "f64", "m_y_rd_red": "f64"}
modes = {k: "val" for k, v in P.items() if v in ("f64", "u8", "AnnexChoice", "AnnexParams", "part_1_1::BucklingCurve")}
s = phspec.build(F, "check_uls_actions", [
 ("check_uls_tension_and_compression", "Checks the tension and the compression resistance of one governing ultimate action row.", 46, 118),
 ("check_uls_shear_and_axial_bending", "Checks the shear resistance and the bending with axial force of one governing ultimate action row.", 161, 192),
 ("check_uls_buckling", "Checks the flexural and the lateral torsional buckling resistance of one governing ultimate action row.", 244, 316),
 ("check_uls_interaction", "Checks the combined bending and axial interaction of one governing ultimate action row.", 387, 387),
], types=P, modes=modes, keep="auto", nth=0, block_line=37, context=P, allow_reason="the extracted check keeps the member resistances of its original function as explicit parameters")
json.dump(s, open(sys.argv[1], "w", encoding="utf-8"), ensure_ascii=False, indent=1)
