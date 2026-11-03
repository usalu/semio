"""Writes the v3 gap file from the spec-trace JSON, the unreached-function TSVs and the added catalogues. Usage: python3 -I gen_gap.py OUT_MD"""
import json, pathlib, re, sys
ART = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
S = "🏅️standards/🔖️1/🪆️subsets/✳️any/"
TICKET = pathlib.Path(__file__).resolve().parent.parent.parent
GEN = TICKET / "🗑️generated/catalogue-bridge-v3"
CODES = {"1993": ("🔩️en1993", "mm, N, MPa"), "1994": ("🧩️en1994", "SI: m, Pa, N, N·m"), "1998": ("🫨️en1998", "SI: kg, m, s, N, Pa"), "1999": ("🪶️en1999", "SI: m, Pa, N")}
CAT = {"1993": "🧬️schema/💡️inferences/⚖️checks/📇️catalogue", "1994": "🧬️schema/💡️inferences/⚖️checks/📇️catalogue", "1998": "🧬️schema/💡️inferences/📇️catalogue", "1999": None}
ROWS = {
 "1993": {
  "V-001": ("AnnexParams::GAMMA_M0..GAMMA_MF", "γM0, γM1, γM2, γMf (1)", "registry lookup of partial factors, no demand or capacity: nothing to evaluate"),
  "V-014": ("joints::bolt_fub, joints::bolt_fyb", "bolt class (choice 4.6 to 10.9) -> fyb, fub (N/mm²)", "table lookup behind a joint field, informative result only; a case needs an informational result row"),
  "V-015": ("cold_stainless::stainless_grade", "grade (choice) -> fy, fu (N/mm²)", "built-in grade table behind ndp.en1993.v-300.2 (unverified), lookup without verification"),
  "V-034": ("-", "fy,f, fy,w, hybrid ratio", "not-modelled and no pure function: hybrid girder reduction f_y,red of 5.5.2(11) needs a new formula"),
  "V-066": ("resistance::warping_torsion_fork (exists, unreached)", "l (mm), E·Iw (N·mm⁴/mm²), G·It (N·mm²), Mt (N·mm), xa (mm), udl flag", "returns θ, Tt, Tw and bimoment of a fork-supported member; the warping stress σw = B/Wω and the 6.2.7(5)-(9) combination have no function, so no verification can be assembled without new formulas"),
  "V-501": ("-", "σmax, σmin (N/mm²)", "not-modelled and no function: Δσ from σmax/σmin with the 0.6 compression reduction needs a new formula"),
  "V-504": ("fatigue_fire::rainflow (exists, unreached)", "stress history (vector, N/mm²) -> (range, count) blocks", "counting only, no utilisation; vector inputs are not seeded by design_case::seed, so a case cannot start from example values (shared bridge follow-up)"),
  "V-505": ("-", "Tables 8.1 to 8.10 detail categories", "needs the detail tables as new data"),
  "V-506": ("-", "Paris law constants, crack geometry", "needs new formulas (fracture mechanics assessment)"),
  "V-601": ("fatigue_fire::specific_heat_steel, k_p_theta, gas_*", "θ (°C), t (min)", "thermal property functions, informative; the heating model reaches them through the unprotected/protected temperature checks"),
  "V-609": ("-", "-", "advanced calculation methods need a finite element solver; out of scope for closed forms"),
  "V-366": ("effective_width::k_sigma_internal, k_sigma_outstand", "ψ (1)", "table lookup (kσ), informative; reached by the plated panel checks"),
  "V-805": ("misc::plate_lateral_pressure_stress (exists, unreached)", "k (1), p (N/mm²), b (mm), t (mm)", "plated panel entity carries no lateral pressure, so the function is never called; case C93-01 added"),
 },
 "1994": {
  "C-11": ("catalogue::plastic_sagging, plastic_full, plastic_hogging", "ISection (h, b, tf, tw, A, Iy), fyd, ys, beff, hc, fcc / As, fsd, yr", "support functions of C-07, C-08, C-12 with no check id of their own; cases C94-07 and C94-08 added"),
  "C-17": ("catalogue::stud_resistance_n, sheeting_reduction", "d, hsc, fu, fck, γV, hp, b0, t, nr, rib direction", "kt/kl of profiled sheeting only inside the stud check of a beam entity; case C94-01 added"),
  "C-45": ("catalogue::ky_theta, steel_specific_heat", "θa (°C), fy (Pa), σEd (Pa)", "steel strength reduction exposed only through column fire checks; case C94-06 added; concrete and reinforcement tables of F:3.2 are not modelled and need new data"),
 },
 "1998": {
  "SEI-03": ("part_1::elastic_response_spectrum, damping_correction", "ag (m/s²), S, TB, TC, TD (s), T (s), ξ (%)", "drawn by scene diagrams and blessed by the oracle only; case C98-02 added"),
  "SEI-04": ("part_1::design_spectrum_horizontal", "ag, S, TB, TC, TD, q, T", "kernel function; case C98-01 added (base shear Fb = Sd(T1) m λ)"),
  "SEI-05": ("part_1::vertical_elastic_spectrum", "ag (m/s²), avg/ag (1), Tv (s)", "drawn by the vertical spectrum diagram only; case C98-03 added"),
  "SEI-42": ("part_1::shear_building_modes, effective_modal_masses, srss_combine, cqc_combine", "masses (kg), storey stiffness (N/m), spectrum, ξ", "kernel functions blessed by the oracle only; case C98-04 added (three storeys, scalar inputs)"),
 },
 "1999": {
  "A-01": ("part_1_1::ALLOY_BANDS, resolve_alloy*, max_thickness", "alloy designation, wall thickness (m)", "thickness-band lookup of f_o, f_u, ρ_haz: a table, reached by every member check, no verification of its own"),
  "A-28": ("-", "stiffener spacing (m), second moment of the transverse web stiffener (m⁴), web geometry", "now realised as a check for the edge stiffener of an outstand and the ring stiffener of a shell (owner update); the stiffness requirement of the transverse web stiffeners of plate girders (6.7.6) has no function and the model carries only their spacing, so it needs a new formula and section data"),
  "A-81": ("-", "α = 23e-6, creep", "excluded by the spec (constant coefficient, creep negligible below 100 °C): nothing to evaluate"),
 },
}
MOD = {  # 1993 module -> (spec ids, hint)
 "🌉️bridge-fatigue": "V-880", "🏷️classification": "V-030..V-033", "🥫️cold-formed": "V-250..V-254", "❄️cold-stainless": "V-015, V-300", "🚜️crane-runway": "V-890", "🪟️effective-width": "V-362, V-366",
 "⏳️fatigue-fire": "V-500..V-504, V-600..V-611", "🔗️joint-components": "V-420..V-423", "🔩️joints": "V-400..V-407, V-410, V-431, V-433", "🌀️mcr-fe": "V-113", "🧰️misc": "V-090, V-132, V-805 (pipelines: EN 1993-4-3 has no spec id)",
 "📐️parametric-sections": "V-050", "🏗️parts-2-6": "V-440, V-850, V-860, V-870, V-880, V-890", "🧩️plated-ext": "V-074, V-360", "🧊️plates-shells": "V-800, V-801", "🧮️resistance": "V-040..V-075", "🧷️sheet-piling": "V-870",
 "🧱️stability": "V-100..V-127", "🔬️stainless": "V-300", "🪢️tension-components": "V-860", "🌡️toughness": "V-012", "🧵️weld-group": "V-412",
}
FNSPEC = {  # 1994 / 1998 function -> spec id
 "1994": {"patch_interaction": "C-24", "introduction_studs": "C-31", "deflection_simple": "C-40", "deflection_two_span": "C-40", "confinement": "C-26", "hogging_zones": "C-05", "max_shear_own": "C-05", "moment_per_load": "C-05", "shear_per_load": "C-05", "beta_end_support": "C-48", "kc_theta": "C-32", "ke_theta": "C-32", "eps_c1_theta": "C-32", "concrete_secant_modulus_pa": "C-32", "sample": "C-32", "fctk005_pa": "C-42", "area_conc": "C-26", "area_steel": "C-26"},
 "1998": {"cantilever_beta_l": "EN 1998-6 4.3 (no spec id)", "cantilever_mode_shape": "EN 1998-6 4.3", "cantilever_participation": "EN 1998-6 4.3", "cantilever_period_ratio": "EN 1998-6 4.3", "chord_force_at_n": "SEI-48", "shear_at_n": "SEI-48", "shear_flow_n_per_m": "SEI-48", "cqc_combine": "SEI-42", "cqc_correlation": "SEI-42", "srss_combine": "SEI-42", "damping_correction": "SEI-03", "effective_modal_masses": "SEI-42", "horizontal_limit": "EN 1998-5 Annex F (no spec id)", "moment_limit": "EN 1998-5 Annex F", "liquefaction_safety_factor": "EN 1998-5 4.1.4 (no spec id)", "pile_inertial_moment_at": "EN 1998-5 5.4.2 (no spec id)", "vertical_elastic_spectrum": "SEI-05", "vs30_from_layers": "SEI-01", "from_snapshot": "-", "to_snapshot": "-", "grid": "-", "ordinate": "SEI-06"},
 "1999": {"category_ids": "A-40", "sn_samples": "A-40", "max_thickness": "A-01", "haz_extent_of": "A-02", "haz_width_for": "A-02"},
}
REASON = {"1999": {"category_ids": "list of Annex J categories for the catalogue table, no verification", "sn_samples": "samples the S-N curve for the diagram only", "max_thickness": "thickness of a section for the scene facts only, no verification", "haz_extent_of": "false positive of the call-graph scan: reached through the private element_factors of the effective section", "haz_width_for": "false positive: reached through haz_extent_of"},
          "1998": {"from_snapshot": "snapshot conversion, not a formula", "to_snapshot": "snapshot conversion, not a formula", "grid": "diagram sampling helper", "ordinate": "accelerogram spectrum sampling helper"}}
def spec_clause(code, ids):
    d = json.loads((ART / CODES[code][0] / S / "🧭️spec-trace/🔣️.json").read_text(encoding="utf-8"))
    return {i["id"]: i for i in d["items"]}
out = ["# 🧩 Catalogue bridge v3: gaps of the entity-driven codes (EN 1993, 1994, 1998, 1999)", "",
 "Status: UNCOMPILED, written by reading. Generated by `🧰️scripts/catalogue-bridge-v3/gen_gap.py` from the spec-trace JSON of each artifact and a call-graph scan (`🧰️scripts/catalogue-bridge-v3/unreached_fns.py`, name based: every `pub fn` of the pure modules whose name is not called, directly or through other called pure functions, from a non-test, non-scene file; private callers are ignored, so a few false positives remain and are marked).", "",
 "Reasons used below: a pure function is *not entity-realised* when no check iterating a snapshot collection calls it (only tests, scene diagrams or other unreached functions do), or when the spec row is a lookup (`data`) or has no formula at all (`not-modelled`).", ""]
out += ["## Outcome", "", "| code | unreached pure functions | catalogue entries added | location of the evaluators |", "|---|---:|---:|---|", "| EN 1993 | 128 | 15 (C93-01..C93-15) | `⚖️checks/📇️catalogue` |", "| EN 1994 | 18 | 8 (C94-01..C94-08) | `⚖️checks/📇️catalogue` |", "| EN 1998 | 21 | 8 (C98-01..C98-08) | `💡️inferences/📇️catalogue` |", "| EN 1999 | 5 | 0 | none: only diagram and scene helpers and two call-graph false positives are unreached; A-28 meanwhile became a check and only the transverse web stiffener requirement needs a new formula |", "",
 "Notes: (1) vocabulary units are the units of the existing evaluators (EN 1993 mm, N, N/mm²; the others SI), so a case value is passed through unchanged; (2) vector and series inputs are not seeded by `design_case::seed`, so all entries use scalar inputs (the three-storey shear building of C98-04 has m_1..m_3, k_1..k_3); (3) functions that only look up a table (V-014, V-015, V-366, V-601, A-01) or only count/sample (V-504 rainflow, S-N samples) have no demand/capacity and were not turned into cases; (4) the call-graph scan is name based and ran on the sources as of this pass, concurrent owner changes can move rows.", ""]
ADDED = {}
for code in CODES:
    cd = ART / CODES[code][0] / S
    trace = spec_clause(code, None)
    out += [f"## EN {code}", ""]
    out += ["### Spec rows that are `data`, `not-modelled` or have no entity", "", "| spec id | clause | realisation | function | inputs (unit) | reason it is not entity-realised |", "|---|---|---|---|---|---|"]
    for sid, (fn, ins, why) in ROWS[code].items():
        it = trace.get(sid, {"clause": "-", "realisation": {"kind": "-"}})
        out.append(f"| {sid} | {it['clause']} | {it['realisation']['kind']} | {fn} | {ins} | {why} |")
    out.append("")
    tsv = [l.split("\t") for l in (GEN / f"en{code}.tsv").read_text(encoding="utf-8").splitlines() if l]
    catdir = (cd / CAT[code]) if CAT[code] else None
    used = set()
    if catdir and (catdir / "🧮️evaluators/🦀️.rs").exists():
        used = set(re.findall(r"(?:::|\b)([a-z_0-9]+)\(", (catdir / "🧮️evaluators/🦀️.rs").read_text(encoding="utf-8")))
    out += [f"### Pure functions not reachable from any snapshot entity ({len(tsv)}; units {CODES[code][1]})", "", "| nearest spec id | function | inputs | case |", "|---|---|---|---|"]
    for rel, name, line, ncalls in tsv:
        text = list(open(cd / rel, encoding="utf8"))
        sig = re.sub(r"\s+", " ", text[int(line) - 1]).strip()
        m = re.search(r"fn " + re.escape(name) + r"(?:<[^>]*>)?\((.*?)\)\s*(->|\{|$)", sig)
        params = m.group(1) if m else sig
        mod = rel.split("/")[-2] if code == "1993" else ""
        spec = MOD.get(mod, "-") if code == "1993" else FNSPEC[code].get(name, "-")
        why = REASON.get(code, {}).get(name, "")
        label = (mod.lstrip("🌉️🏷️🥫️❄️🚜️🪟️⏳️🔗️🔩️🌀️🧰️📐️🏗️🧩️🧊️🧮️🧷️🧱️🔬️🪢️🌡️🧵️") + "::" if mod else "") + name
        case = "added" if name in used else ("-" + (f" ({why})" if why else ""))
        out.append(f"| {spec} | {label} | `{params.replace('|', '/')}` | {case} |")
    out.append("")
    entries = []
    if catdir and (catdir / "🦀️.rs").exists():
        txt = (catdir / "🦀️.rs").read_text(encoding="utf-8")
        entries = re.findall(r'Entry \{ id: "([^"]+)", part: "([^"]+)", clause: "([^"]+)", en: "([^"]+)"', txt)
    out += ["### Catalogue added", ""]
    if entries:
        out += [f"Location `{CODES[code][0]}/{S}{CAT[code]}/` (vocabulary `📚️inputs/{{🔣️.json,🦀️.rs}}`, evaluators `🧮️evaluators`, `En{code}Cases: CaseCatalogue` in `🦀️.rs`, tests `🧪️tests/🔬️unit`), mounted by one `#[path]` pair `pub mod case_catalogue;` at the end of the checks/inferences root.", "", "| entry | part | clause | title |", "|---|---|---|---|"]
        out += [f"| {i} | {p} | {c} | {e} |" for i, p, c, e in entries]
    else:
        out.append("None: every verification formula of this code is already called by a check that iterates a snapshot collection; the remaining rows have no existing evaluator, so no evaluator was invented. New formulas needed: A-28 transverse web stiffener stiffness requirement of 6.7.6 for plate girders (needs section data of the stiffener in the model).")
    out.append("")
pathlib.Path(sys.argv[1]).write_text("\n".join(out) + "\n", encoding="utf-8")
