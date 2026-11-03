"""W3-EN1993-CORR wire step: after the leaves `fatigueDetails[].consequence` and `bridgeFatigue[].consequence` exist (`evolve.py add-leaf`, specs `en1993-fatigue-consequence.json` and `en1993-bridge-fatigue-consequence.json`), replaces the method-only partial factor gamma_Mf by the 2x2 table of EN 1993-1-9 Table 3.1 (assessment method x consequence class) in the Rust call sites, the oracle, the unit tests, the fixture generator and the check-outcome rows.
Usage: /home/deck/semio/.venv/bin/python -I wire.py [--check]   (--check only verifies that every anchor exists exactly once and that the leaves exist)"""
import glob, json, pathlib, re, subprocess, sys

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
ART = next(iter(glob.glob(str(ROOT / "*en1993") + "/🏅️standards/🔖️1/🪆️subsets/✳️any")))
ART = pathlib.Path(ART)
TICKET = pathlib.Path(__file__).resolve().parents[2]
CHECKS = ART / "🧬️schema/💡️inferences/⚖️checks"
PARTS = CHECKS / "📏️parts/🦀️.rs"
VID = CHECKS / "🧱️member-checks/🧾️vid-checks/🧪️fatigue-fire-vids/🦀️.rs"
THIN = CHECKS / "🔗️special-checks/⏳️fatigue-fire-and-thin-walled-checks/🦀️.rs"
SHELL = CHECKS / "🔗️special-checks/🏛️shell-and-structure-checks/🦀️.rs"
SPECIAL = CHECKS / "🔗️special-checks/🔭️special-parts-checks/🦀️.rs"
DESIGN = ART / "🎬️scene/📈️design-diagrams/🦀️.rs"
DIAGRAMS = ART / "🎬️scene/📈️diagrams/🦀️.rs"
VIEWS = ART / "🎬️scene-members/🧵️fatigue-views/🦀️.rs"
COMPLIANCE_TEST = ART / "🧬️schema/🧪️tests/⚖️compliance/🦀️.rs"
ORACLE = ART / "🔮️oracles/🚦️check-outcome/🧭️families/🐍️.py"
SNAPSHOT = ART / "🧬️schema/📸️snapshot/🦀️.rs"
SHOWCASE = ART / "🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json"
GAPS = TICKET / "🧰️scripts/oracle-gaps"
FEATURE = ART / "🧪️tests/🚦️check-outcome-en1993-1/🥒️.feature"

PARTS_OLD_ENUM = "    pub enum AssessmentMethod {\n        DamageTolerant,\n        SafeLife,\n        LowConsequence,\n    }\n\n"
PARTS_OLD_FNS = '''    pub fn parse_method(value: &str) -> AssessmentMethod {
        match value.to_ascii_lowercase().as_str() {
            "safe_life" => AssessmentMethod::SafeLife,
            "low_consequence" => AssessmentMethod::LowConsequence,
            _ => AssessmentMethod::DamageTolerant,
        }
    }

    pub fn gamma_mf(method: AssessmentMethod) -> f64 {
        match method {
            AssessmentMethod::DamageTolerant => 1.0,
            AssessmentMethod::SafeLife => 1.15,
            AssessmentMethod::LowConsequence => 1.35,
        }
    }
'''
PARTS_NEW_FNS = '''    /// 🔖️ Registry row of γ_Mf for an assessment method and a consequence class (EN 1993-1-9 Table 3.1: damage tolerant 1.00 / 1.15, safe life 1.15 / 1.35 for low / high consequence).
    pub fn gamma_mf_id(method: FatigueMethod, consequence: FatigueConsequence) -> &'static str {
        match (method, consequence) {
            (FatigueMethod::DamageTolerant, FatigueConsequence::Low) => "ndp.en1993.1-9.gamma-mf.damage-tolerant.low",
            (FatigueMethod::DamageTolerant, FatigueConsequence::High) => "ndp.en1993.1-9.gamma-mf.damage-tolerant.high",
            (FatigueMethod::SafeLife, FatigueConsequence::Low) => "ndp.en1993.1-9.gamma-mf.safe-life.low",
            (FatigueMethod::SafeLife, FatigueConsequence::High) => "ndp.en1993.1-9.gamma-mf.safe-life.high",
        }
    }

    /// 🔄 γ_Mf of a fatigue assessment from the registry row of its method and consequence class.
    pub fn gamma_mf(method: FatigueMethod, consequence: FatigueConsequence, annex: semio_s_plugin_norm_compliance::AnnexChoice) -> f64 {
        semio_s_plugin_norm_compliance::ndp_number(gamma_mf_id(method, consequence), annex)
    }
'''
BRIDGE_DESIGN_OLD = "    let params = AnnexParams::for_choice(d.annex);\n    let l2 = bridge_fatigue::lambda_2("

EDITS = [
    (PARTS, PARTS_OLD_ENUM, "    use semio_s_artifact_norm_en1993_model::{FatigueConsequence, FatigueMethod};\n\n"),
    (PARTS, PARTS_OLD_FNS, PARTS_NEW_FNS),
    (VID, "    let gmf = part_1_9::gamma_mf(part_1_9::parse_method(fat.method.as_str()));", "    let gmf = part_1_9::gamma_mf(fat.method, fat.consequence, annex);"),
    (VID, '.ndps(&["ndp.en1993.v-502.1", AnnexParams::GAMMA_MF]);', '.ndps(&["ndp.en1993.v-502.1", part_1_9::gamma_mf_id(fat.method, fat.consequence)]);'),
    (THIN, "        let method = part_1_9::parse_method(fat.method.as_str());\n        let gmf = part_1_9::gamma_mf(method);", "        let gmf = part_1_9::gamma_mf(fat.method, fat.consequence, annex);"),
    (THIN, "        .ndp(AnnexParams::GAMMA_MF)\n        .utilization(dimensionless(damage), dimensionless(1.0))", "        .ndp(part_1_9::gamma_mf_id(fat.method, fat.consequence))\n        .utilization(dimensionless(damage), dimensionless(1.0))"),
    (SHELL, "        let method = part_1_9::parse_method(bf.method.as_str());\n        let gmf = part_1_9::gamma_mf(method);", "        let gmf = part_1_9::gamma_mf(bf.method, bf.consequence, annex);"),
    (SHELL, "        .ndp(AnnexParams::GAMMA_MF)\n        .utilization(stress_pa(delta_e2), stress_pa(limit))", "        .ndp(part_1_9::gamma_mf_id(bf.method, bf.consequence))\n        .utilization(stress_pa(delta_e2), stress_pa(limit))"),
    (SPECIAL, "pub(super) fn check_bridge(annex: AnnexChoice, params: AnnexParams, bf: &BridgeFatigue", "pub(super) fn check_bridge(annex: AnnexChoice, _params: AnnexParams, bf: &BridgeFatigue"),
    (SPECIAL, "    let u = bridge_fatigue::fatigue_utilisation(delta_e2, delta_c, 1.0, params.gamma_mf);", "    let gmf = part_1_9::gamma_mf(bf.method, bf.consequence, annex);\n    let u = bridge_fatigue::fatigue_utilisation(delta_e2, delta_c, 1.0, gmf);"),
    (SPECIAL, "bf.lambda_1, delta_c / params.gamma_mf), format!(\"λ1={:.2}, λ2={l2:.3}, λ3={l3:.3}, λ4={l4:.3} → λ={lam:.3}; Δσ_E,2={delta_e2:.1} MPa gegen Δσ_C/γMf={:.1} MPa (k_s={ks:.2}).\", bf.lambda_1, delta_c / params.gamma_mf)),", "bf.lambda_1, delta_c / gmf), format!(\"λ1={:.2}, λ2={l2:.3}, λ3={l3:.3}, λ4={l4:.3} → λ={lam:.3}; Δσ_E,2={delta_e2:.1} MPa gegen Δσ_C/γMf={:.1} MPa (k_s={ks:.2}).\", bf.lambda_1, delta_c / gmf)),"),
    (SPECIAL, "        gammas: &[AnnexParams::GAMMA_MF],\n    };\n    emit(report, annex, case, || {\n        let needed = delta_c * u / ks;", "        gammas: &[part_1_9::gamma_mf_id(bf.method, bf.consequence)],\n    };\n    emit(report, annex, case, || {\n        let needed = delta_c * u / ks;"),
    (DESIGN, "frame_eigen, joint_frame, part_1_1, part_3, toughness_point", "frame_eigen, joint_frame, part_1_1, part_1_9, part_3, toughness_point"),
    (DESIGN, BRIDGE_DESIGN_OLD, "    let gamma = part_1_9::gamma_mf(bf.method, bf.consequence, d.annex);\n    let l2 = bridge_fatigue::lambda_2("),
    (DESIGN, "bridge_fatigue::fatigue_utilisation(delta_e2, delta_c, 1.0, params.gamma_mf)", "bridge_fatigue::fatigue_utilisation(delta_e2, delta_c, 1.0, gamma)"),
    (DIAGRAMS, "fn fatigue(report: &CheckReport, detail: &FatigueDetail) -> Option<Diagram2d> {", "fn fatigue(report: &CheckReport, detail: &FatigueDetail, d: &En1993Snapshot) -> Option<Diagram2d> {"),
    (DIAGRAMS, "    let gamma = part_1_9::gamma_mf(part_1_9::parse_method(detail.method.as_str()));", "    let gamma = part_1_9::gamma_mf(detail.method, detail.consequence, d.annex);"),
    (DIAGRAMS, "            out.extend(fatigue(report, detail));", "            out.extend(fatigue(report, detail, document));"),
    (VIEWS, "pub fn damage(detail: &FatigueDetail) -> f64 {", "pub fn damage(detail: &FatigueDetail, d: &En1993Snapshot) -> f64 {"),
    (VIEWS, "part_1_9::gamma_mf(part_1_9::parse_method(detail.method.as_str()))", "part_1_9::gamma_mf(detail.method, detail.consequence, d.annex)"),
    (VIEWS, "        let total = damage(detail);\n        let offset = (CLASSES as f64 + 1.0)", "        let total = damage(detail, d);\n        let offset = (CLASSES as f64 + 1.0)"),
    (VIEWS, "                return Vec::new();\n            }\n            let total = damage(detail);", "                return Vec::new();\n            }\n            let total = damage(detail, document);"),
    (COMPLIANCE_TEST, "    assert_eq!(part_1_9::gamma_mf(part_1_9::AssessmentMethod::DamageTolerant), 1.0);\n}\n", '''}

#[test]
fn w3_gamma_mf_is_the_two_by_two_table_of_en_1993_1_9_table_3_1() {
    use semio_s_artifact_norm_en1993_model::{FatigueConsequence::{High, Low}, FatigueMethod::{DamageTolerant, SafeLife}};
    for annex in [AnnexParams::en().choice, AnnexParams::de().choice] {
        for (method, consequence, expected) in [(DamageTolerant, Low, 1.0), (DamageTolerant, High, 1.15), (SafeLife, Low, 1.15), (SafeLife, High, 1.35)] {
            assert_eq!(part_1_9::gamma_mf(method, consequence, annex), expected, "{method:?} {consequence:?}");
        }
    }
}

#[test]
fn w3_the_fatigue_damage_of_a_detail_follows_its_consequence_class() {
    use semio_s_artifact_norm_en1993_model::FatigueConsequence::{High, Low};
    let damage = |consequence| {
        let mut document = En1993Snapshot::example(AnnexParams::en().choice);
        document.fatigue_details[0].consequence = consequence;
        check_full_steel_structure(&document).checks.iter().find(|c| c.id == "en1993.1-9.fat.fat-1").map(|c| c.utilization).expect("the fatigue check of fat-1")
    };
    approx(damage(Low), 0.2117946739890245, 1e-6);
    approx(damage(High), 0.37072016487327836, 1e-6);
}
'''),
]


def read(path):
    return pathlib.Path(path).read_text(encoding="utf-8")


def leaves_exist():
    text = read(SNAPSHOT)
    return len(re.findall(r"pub consequence: FatigueConsequence", text)) == 2 and '"consequence"' in read(SHOWCASE)


def apply_edits(check_only):
    pending = {}
    for path, old, new in EDITS:
        text = pending.get(path, read(path))
        if text.count(old) != 1:
            sys.exit(f"anchor not found exactly once ({text.count(old)}x) in {path}:\n{old[:160]}")
        pending[path] = text.replace(old, new)
    pending[DESIGN] = pending[DESIGN].replace("params.gamma_mf", "gamma")
    if not check_only:
        for path, text in pending.items():
            pathlib.Path(path).write_text(text, encoding="utf-8")
    return len(pending)


def approx_exists():
    return "fn approx(" in read(COMPLIANCE_TEST)


ORACLE_OLD = '''    gmf = {"DamageTolerant": 1.0, "SafeLife": 1.15}[fat["method"]]
    dc = fat["category"] * 1e6 / gmf
    dd = (2 / 5) ** (1 / 3) * dc
    dl = (5 / 100) ** (1 / 5) * dd
    if not fat["spectrum"]:
        return []
    damage = 0.0
    for band in fat["spectrum"]:
        ds, n = band["deltaSigma"], band["cycles"]
        if ds < dl:
            continue
        nr = 2e6 * (dc / ds) ** 3 if ds >= dd else 5e6 * (dd / ds) ** 5
        damage += n / nr
    return [(f"en1993.1-9.fat.{fat['id']}", damage)]
'''
ORACLE_NEW = '''    gmf, cell = gamma_mf(fat)
    dc = fat["category"] * 1e6 / gmf
    dd = (2 / 5) ** (1 / 3) * dc
    dl = (5 / 100) ** (1 / 5) * dd
    peak = max((b["deltaSigma"] for b in fat["spectrum"]), default=0.0)
    cafl = (f"en1993.1-9.6.cafl.{fat['id']}", peak * gmf / (fat["category"] * 1e6 * (2 / 5) ** (1 / 3)), [cell]) if len(fat["spectrum"]) == 1 else (f"en1993.1-9.6.cafl.{fat['id']}", 0.0, "info")
    if not fat["spectrum"]:
        return [cafl]
    damage = 0.0
    for band in fat["spectrum"]:
        ds, n = band["deltaSigma"], band["cycles"]
        if ds < dl:
            continue
        nr = 2e6 * (dc / ds) ** 3 if ds >= dd else 5e6 * (dd / ds) ** 5
        damage += n / nr
    return [(f"en1993.1-9.fat.{fat['id']}", damage, [cell]), cafl]


def bridge_fatigue_row(bf):
    """🌉 EN 1993-2 9.3 with EN 1993-1-9 Table 3.1: lambda phi_2 Delta sigma_p against Delta sigma_C / gamma_Mf (gamma_Mf of the method and the consequence class of the detail)."""
    gmf, cell = gamma_mf(bf)
    return [(f"en1993.2.fat.{bf['id']}", bf["lambda"] * bf["phi2"] * bf["deltaSigmaP"] / (bf["category"] * 1e6 / gmf), [cell])]
'''
ORACLE_HELPER_ANCHOR = "def fatigue_rows(fat):"
ORACLE_HELPER = '''def gamma_mf(item):
    """🔄 EN 1993-1-9 Table 3.1: gamma_Mf from the registry row of the assessment method and the consequence class (damage tolerant 1.00 / 1.15, safe life 1.15 / 1.35), with the row id."""
    cell = "ndp.en1993.1-9.gamma-mf." + {"DamageTolerant": "damage-tolerant", "SafeLife": "safe-life"}[item["method"]] + "." + {"Low": "low", "High": "high"}[item["consequence"]]
    return registry()[cell]["recommended"], cell


'''
ORACLE_DOC_OLD = "gamma_Mf = 1.0 (damage tolerant) or 1.15 (safe life) for low consequence."
ORACLE_DOC_NEW = "gamma_Mf from `gamma_mf` (method x consequence class), plus the constant amplitude limit of 6.3 (`cafl`: gamma_Mf Delta sigma_max / Delta sigma_D for one band, informative otherwise)."
ORACLE_EVAL_OLD = '''    for j in doc["joints"]:
        rows += joint_rows(doc, j, params, combos)'''
ORACLE_EVAL_NEW = '''    for bf in doc.get("bridgeFatigue", []):
        rows += bridge_fatigue_row(bf)
    for j in doc["joints"]:
        rows += joint_rows(doc, j, params, combos)'''


def patch_oracle(check_only):
    text = read(ORACLE)
    for old in (ORACLE_OLD, ORACLE_DOC_OLD, ORACLE_EVAL_OLD, ORACLE_HELPER_ANCHOR):
        if text.count(old) != 1:
            sys.exit(f"oracle anchor not found exactly once: {old[:80]}")
    if check_only:
        return
    text = text.replace(ORACLE_OLD, ORACLE_NEW).replace(ORACLE_DOC_OLD, ORACLE_DOC_NEW).replace(ORACLE_EVAL_OLD, ORACLE_EVAL_NEW).replace(ORACLE_HELPER_ANCHOR, ORACLE_HELPER + ORACLE_HELPER_ANCHOR)
    pathlib.Path(ORACLE).write_text(text, encoding="utf-8")


FIXTURE_OLD = 'd["hollowJoints"][0].update('
FIXTURE_NEW = '''high_consequence = copy.deepcopy(d["fatigueDetails"][0])
high_consequence.update(id="fat-hc", consequence="High")
d["fatigueDetails"].append(high_consequence)
safe_life_bridge = copy.deepcopy(d["bridgeFatigue"][0])
safe_life_bridge.update(id="br-sl-hc", method="SafeLife", consequence="High")
d["bridgeFatigue"].append(safe_life_bridge)
d["hollowJoints"][0].update('''


def patch_fixture_script(check_only):
    path = GAPS / "fixtures_en1993.py"
    text = read(path)
    if FIXTURE_OLD not in text:
        sys.exit("fixture script anchor missing")
    if "fat-hc" in text or check_only:
        return
    pathlib.Path(path).write_text(text.replace(FIXTURE_OLD, FIXTURE_NEW, 1), encoding="utf-8")


def regenerate_rows():
    py = "/home/deck/semio/.venv/bin/python"
    text = read(FEATURE)
    kept = [line for line in text.splitlines() if not re.search(r"\|\s*(?:pass|fail)-v30-\d+\s*\|[^|]*\|\s*en1993\.1-9\.fat\.", line)]
    pathlib.Path(FEATURE).write_text("\n".join(kept) + "\n", encoding="utf-8")
    subprocess.run([py, "-I", str(GAPS / "fixtures_en1993.py")], check=True, cwd=GAPS)
    spec = GAPS / "spec_fatigue_w3corr.tsv"
    subprocess.run([py, "-I", str(GAPS / "spec_from_oracle.py"), "en1993", str(spec), "en1993.1-9.fat.,en1993.1-9.6.cafl.,en1993.2.fat.", "shared://🎪️showcase/🔣️.json", "shared://📄️oracle-documents/families-fail/🔣️.json"], check=True, cwd=GAPS)
    subprocess.run([py, "-I", str(GAPS / "rows.py"), "en1993", str(spec)], check=True, cwd=GAPS)


def main():
    check_only = "--check" in sys.argv
    if not check_only and not leaves_exist():
        sys.exit("the leaves fatigueDetails[].consequence and bridgeFatigue[].consequence are not applied yet (evolve.py add-leaf); nothing changed")
    if not approx_exists():
        sys.exit("the compliance test file has no `approx` helper")
    edits = apply_edits(check_only)
    patch_oracle(check_only)
    patch_fixture_script(check_only)
    if not check_only:
        regenerate_rows()
    print(("anchors ok: " if check_only else "wired: ") + f"{edits} Rust files, oracle, fixture script" + ("" if check_only else ", rows regenerated"))


main()
