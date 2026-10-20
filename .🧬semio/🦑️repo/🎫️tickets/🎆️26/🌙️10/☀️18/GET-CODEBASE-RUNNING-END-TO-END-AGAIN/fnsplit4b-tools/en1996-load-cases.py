import sys, json
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import phspec
F = "/Users/ueli/Documents/semio/" + sys.argv[2]
P = {"annex": "AnnexChoice", "masonry_class": "MasonryClass", "default_situation": "DesignSituation", "wall": "&MasonryWall", "out": "&mut Vec<CheckResult>", "path_t": "&str", "path_fb": "&str", "path_mu": "&str",
     "path_bearing": "&str", "path_as_h": "&str", "path_density": "&str", "path_phi_inf": "&str", "lambda": "f64", "a": "f64", "f_k": "f64", "t": "f64",
     "lc": "&WallLoadCase", "effects": "Vec<DesignEffects>", "sit": "DesignSituation", "params": "AnnexParams", "gamma_m": "f64", "f_d": "f64"}
modes = {k: "val" for k, v in P.items() if v in ("f64", "AnnexChoice", "MasonryClass", "DesignSituation", "AnnexParams")}
modes["effects"] = "ref"
s = phspec.build(F, "check_load_cases", [
 ("check_wall_compression", "Checks the compression, the eccentricities and the slenderness of the wall under one load case.", 31, 31),
 ("check_wall_shear", "Checks the shear resistance of the wall under one load case.", 183, 183),
 ("check_wall_flexure", "Checks the lateral flexure of the wall under one load case.", 227, 227),
 ("check_wall_concentrated_load", "Checks the concentrated loads on the wall under one load case.", 294, 294),
], types=P, modes=modes, keep="auto", nth=0, block_line=24, context=P, allow_reason="the extracted check keeps the wall values of its original function as explicit parameters")
json.dump(s, open(sys.argv[1], "w", encoding="utf-8"), ensure_ascii=False, indent=1)
