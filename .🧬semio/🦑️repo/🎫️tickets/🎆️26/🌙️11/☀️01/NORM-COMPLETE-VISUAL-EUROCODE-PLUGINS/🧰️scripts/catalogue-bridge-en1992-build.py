"""Superseded by catalogue-bridge-en1992-units.py (hand-reviewed units from the evaluators); do not run, it would overwrite the reviewed TSV with name-rule guesses. Original: builds catalogue-bridge-en1992-inputs.tsv by naming rules. EN 1992 evaluators take N, mm, N/mm² (see the Outcome constructors demand_mpa, demand_nmm, mm²); every unit is inferred from the key name, so the confidence is medium at best and low for the fallback."""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import re, pathlib, collections
ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992")
decl, listed, textual, extras = {}, set(), set(), set()
for path in ROOT.rglob("*.rs"):
    if "target" in path.parts: continue
    t = path.read_text(encoding="utf-8")
    for m in re.finditer(r'entry_of\("([^"]+)", "[^"]*", "[^"]*", "([^"]*)", "[^"]*", &\[([^\]]*)\]', t):
        decl[m.group(1)] = (m.group(2), re.findall(r'"([A-Za-z0-9_]+)"', m.group(3)))
    listed |= set(re.findall(r'(?:\.l|\.list\.get|with_l)\("([a-z_0-9]+)"', t))
    textual |= set(re.findall(r'(?:\.so|\.s|with_s)\("([a-z_0-9]+)"', t))
    extras |= set(re.findall(r'\b(?:c|case)\.(?:n|o|opt|pos)\("([a-z_0-9]+)"', t))
keys = sorted({k for _, ks in decl.values() for k in ks} | extras)
RULES = [
 (r"^(fck|fcd|fctm|fctd|fctd_t|fct_eff|fck_t|fyk|fyd|fpk|fp01k|fuk|fbd|ecm|ecm_t|es|ep|e_p|sigma_.*|delta_sigma.*)$", "MPa", "high", "stress or modulus in N/mm² (Outcome::stress, demand_mpa)"),
 (r"^(f_ed|f_td|f_v_ed|strut_force|q_wheel|v_.*|n_ed.*|n_rd.*|n_rk.*|delta_n_.*|f_bt|f_cr|n_s|tie)$", "N", "medium", "force in N by name"),
 (r"^(m_ed.*|m_.*|mx|my|mxy|m0|m1|m2|bimoment|m_char|m_total|m_qp)$", "Nmm", "medium", "moment in N·mm (Outcome::moment, demand_nmm)"),
 (r"^(as.*|asw.*|asv|ap|ac|ac0|ac1|ac_per_m|a_c|a_t|a_links|bar_a|sum_ast.*|act|act_ext|as_.*|a_b)$", "mm2", "medium", "area in mm² (Outcome::area)"),
 (r"^(i|ic|ei|is_moment|iz|iy)$", "1", "low", "second moment or stiffness: no matching stored unit"),
 (r"^(alpha_deg|alpha_v_deg|angle|step_deg|tendon_angle|theta_pl|theta_m)$", "deg", "medium", "angle in degrees by name"),
 (r"^(delta_t)$", "K", "medium", "temperature difference"),
 (r"^(t_hours)$", "h", "high", "time in hours by name"),
 (r"^(t_min)$", "min", "high", "time in minutes by name"),
 (r"^(design_life|t_pre)$", "a", "low", "years assumed"),
 (r"^(t|t0|t_a|t_f|t_w|t_y|t_z|t_ed|ts|tw)$", "1", "low", "time (days) or thickness: not distinguishable by name"),
 (r"^(.*_m|l_m|wall_length_m|width_m|radius_m)$", "m", "high", "length in m by suffix"),
 (r"^(b|bw|bf|bi|b_e|b_i|bt|h|hf|hw|h0|h_.*|d|d2|d_.*|c|c0|c1|c2|c2_left|c2_right|c_provided|clear|clear_distance|cover|dim|dg|dh|dc_.*|hef|hef_eff|l|l0|l0_fi|l0t|l_.*|x|x_.*|y_.*|z|z0|z_.*|zcp|u|u1_reduced|width|sr|sr_max|s|s2|s_surf|sx|sy|span|radius|delta_x|w_el|w_before|wk|wk_lim|a|av|ax|ay|az|bar_y|bar_z|e_.*|e0|e1|e2|e_tot|amp_.*|hd_over_h|hcr|sag|slip|xu_d|xy_d|capital_l|point_.*|poly_.*|load_x|load_y|node_x|node_y|hole_.*|sr_y|sr_z)$", "mm", "medium", "length in mm by name (Outcome::length, mm)"),
]
TEXT = {"action", "angle_x", "bond", "bracing", "cement", "classes", "duration", "edge_reinforcement", "exposure", "faces", "ground", "kind", "loading", "method", "mode", "node", "part", "position", "release", "roughness", "shape", "stage", "steel_class", "structure", "sub", "system", "tendon", "tendon_kind", "tie", "type", "situation", "relax_class", "structural_class", "quality", "restraint", "moisture", "member_i", "member_j", "approval", "checklist", "class", "r_class", "catalogue", "checks", "crack", "flexure", "anchors", "acc"}
def label(key):
    sym = key.replace("_", " ")
    return f"Input {key}", f"Eingabe {key}"
out = ["# key|scope|kind|unit|example|flags|en|de|confidence|note   (EN 1992 evaluators: N, mm, N/mm²; units inferred from key names, review before relying on them)"]
counts = collections.Counter()
for key in keys:
    en, de = label(key)
    if key in listed and key not in TEXT:
        out.append(f"{key}||v|1|||{en} (vector)|{de} (Vektor)|low|list input (Case.list); the element unit is unknown")
        counts["vector"] += 1; continue
    if key in TEXT or key in textual:
        out.append(f"{key}||x|-|||{en}|{de}|medium|text read by the evaluator; accepted values not enumerated")
        counts["text"] += 1; continue
    for pat, unit, conf, note in RULES:
        if re.match(pat, key):
            out.append(f"{key}||n|{unit}|||{en}|{de}|{conf}|{note}"); counts[unit] += 1; break
    else:
        out.append(f"{key}||n|1|||{en}|{de}|low|dimensionless fallback (factor, strain, ratio, count or flag); verify against the evaluator")
        counts["fallback"] += 1
open("catalogue-bridge-en1992-inputs.tsv", "w", encoding="utf-8").write("\n".join(out) + "\n")
print(len(keys), "keys", dict(counts))
