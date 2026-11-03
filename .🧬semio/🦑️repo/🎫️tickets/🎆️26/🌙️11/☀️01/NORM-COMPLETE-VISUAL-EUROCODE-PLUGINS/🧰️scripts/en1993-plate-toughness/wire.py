"""Wires the plate toughness check after the three platedPanels leaves (steel_grade, steel_subgrade, design_temperature) exist in the model. Run once: /home/deck/semio/.venv/bin/python -I wire.py"""
import pathlib
SUB = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any")
model = (SUB / "🧬️schema/📸️snapshot/🦀️.rs").read_text(encoding="utf-8")
for f in ("steel_grade", "steel_subgrade", "design_temperature"):
    assert f"pub {f}:" in model.split("pub struct PlatedPanel")[1].split("\n}\n")[0], f"leaf {f} not applied yet"
p = SUB / "🧬️schema/💡️inferences/⚖️checks/🔗️special-checks/🔭️special-parts-checks/🦀️.rs"
t = p.read_text(encoding="utf-8")
assert "fn plate_brittle_fracture" not in t
code = pathlib.Path(__file__).with_name("plate-brittle-fracture.rs.txt").read_text(encoding="utf-8")
anchor = "pub(super) fn check_plated("
assert t.count(anchor) == 1
t = t.replace(anchor, code + anchor)
call = "    let subject = |field: &str, en: &str, de: &str| subj(&p.id, \"platedPanels\", field, en, de);\n"
assert t.count(call) == 1
t = t.replace(call, call + "    if !p.steel_grade.is_empty() && !p.steel_subgrade.is_empty() && t > 0.0 {\n        plate_brittle_fracture(annex, report, p);\n    }\n")
p.write_text(t, encoding="utf-8")
print("wired")

tests = SUB / "🧬️schema/💡️inferences/⚖️checks/🧪️tests/🔭️special-wiring/🦀️.rs"
text = tests.read_text(encoding="utf-8")
text += '''
#[test]
fn a_plated_panel_with_a_grade_is_assessed_against_table_2_1_and_one_without_is_not() {
    let mut doc = En1993Snapshot::special_parts_showcase();
    let check_id = format!("en1993.1-10.t.{}", doc.plated_panels[0].id);
    assert!(check_full_steel_structure(&doc).checks.iter().all(|c| c.id != check_id), "no grade, no check");
    let panel = &mut doc.plated_panels[0];
    (panel.steel_grade, panel.steel_subgrade, panel.design_temperature, panel.thickness) = ("S355".into(), "J2".into(), -20.0, 0.017);
    close(find(&check_full_steel_structure(&doc), &check_id).utilization, 17.0 / 55.0, 1e-9, "S355 J2 at -20 degC");
    doc.plated_panels[0].steel_subgrade = "J9".into();
    assert_eq!(find(&check_full_steel_structure(&doc), &check_id).status, CheckStatus::NotApplicable);
}
'''
tests.write_text(text, encoding="utf-8")
