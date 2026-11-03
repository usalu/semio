#!/usr/bin/env python3
"""One-shot: hand edits for the en1991 `FireOccupancy` conversion (document-facing fire load functions take the enum, registry-facing ones keep their string keys). Each replacement asserts its match count."""
import pathlib, re
ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any")
M = "semio_s_artifact_norm_en1991_model::FireOccupancy"
n = 0

def sub(rel, old, new, count=1):
    global n
    p = ROOT / rel
    t = p.read_text(encoding="utf-8")
    if old not in t and new in t:
        return
    c = t.count(old)
    assert c == count, f"{rel}: expected {count} of {old[:90]!r}, found {c}"
    p.write_text(t.replace(old, new), encoding="utf-8")
    n += 1

CK = "🧬️schema/💡️inferences/⚖️checks/🦀️.rs"
sub(CK, '''    /// 🔖 Registry ids the design fire load rests on''', f'''    /// 🗂️ Registry key of a Table E.4 occupancy row of the document.
    pub fn occupancy_name(occupancy: {M}) -> &'static str {{
        use {M} as O;
        match occupancy {{
            O::Dwelling => "dwelling",
            O::Office => "office",
            O::Hospital => "hospital",
            O::School => "school",
            O::Hotel => "hotel",
            O::Library => "library",
            O::Shopping => "shopping",
            O::Theatre => "theatre",
            O::Transport => "transport",
        }}
    }}
    /// 🧯 Characteristic fire load density q_f,k (J/m²) of a registry occupancy key (aliases accepted), the 80 % fractile of Table E.4; `None` for an unknown key.
    pub fn characteristic_fire_load_j_m2(annex: AnnexChoice, occupancy: &str) -> Option<f64> {{
        occupancy_row(annex, occupancy).map(|row| row.0 * 1.0e6)
    }}
    /// 🔖 Registry ids the design fire load rests on''') if "pub fn characteristic_fire_load_j_m2" not in (ROOT / CK).read_text(encoding="utf-8") else None
sub(CK, "pub fn fire_load_ndp_ids(basis: FireLoadBasis, occupancy: &str) -> Vec<String> {", f"pub fn fire_load_ndp_ids(basis: FireLoadBasis, occupancy: {M}) -> Vec<String> {{")
sub(CK, 'ids.extend(occupancy_key(occupancy).map(|key| format!("ndp.en1991.1-2.fire-load-fractile.{key}")));', 'ids.push(format!("ndp.en1991.1-2.fire-load-fractile.{}", occupancy_name(occupancy)));')
sub(CK, "annex: AnnexChoice, occupancy: &str) -> Option<f64> {\n        match basis {\n            FireLoadBasis::OccupancyTable => characteristic_fire_load_j_m2(annex, occupancy),", f"annex: AnnexChoice, occupancy: {M}) -> Option<f64> {{\n        match basis {{\n            FireLoadBasis::OccupancyTable => characteristic_fire_load_j_m2(annex, occupancy_name(occupancy)),")
sub(CK, "annex: AnnexChoice, occupancy: &str, area_m2: f64, delta_n: f64) -> Option<f64> {", f"annex: AnnexChoice, occupancy: {M}, area_m2: f64, delta_n: f64) -> Option<f64> {{")
sub(CK, "let dq2 = occupancy_row(annex, occupancy).map_or(1.0, |row| row.1);", "let dq2 = occupancy_row(annex, occupancy_name(occupancy)).map_or(1.0, |row| row.1);")
sub(CK, "pub fn fire_load_source(basis: FireLoadBasis, user_qf_k_j_m2: f64, annex: AnnexChoice, occupancy: &str) -> Option<(String, String)> {", f"pub fn fire_load_source(basis: FireLoadBasis, user_qf_k_j_m2: f64, annex: AnnexChoice, occupancy: {M}) -> Option<(String, String)> {{")
sub(CK, 'Table E.4 row \\"{}\\", 80 % fractile q_f,k = {qf_k:.0} MJ/m²", occupancy_key(occupancy)?)', 'Table E.4 row \\"{}\\", 80 % fractile q_f,k = {qf_k:.0} MJ/m²", occupancy_name(occupancy))')
sub(CK, 'Tabelle E.4 Zeile \\"{}\\", 80-%-Fraktile q_f,k = {qf_k:.0} MJ/m²", occupancy_key(occupancy)?)', 'Tabelle E.4 Zeile \\"{}\\", 80-%-Fraktile q_f,k = {qf_k:.0} MJ/m²", occupancy_name(occupancy))')
for rel, old, c in [
    ("🎬️scene/📈️diagrams/🦀️.rs", "&d.fire_occupancy,", 2), ("🎬️scene-members/🔥️thermal-fire/🦀️.rs", "&d.fire_occupancy", 3),
    ("🎬️scene/🧪️tests/🔬️unit/🦀️.rs", "&d.fire_occupancy,", 1), ("📚️examples/🧬️subjects/🦀️.rs", "&s.fire_occupancy,", 1),
    ("🧬️schema/💡️inferences/🦀️.rs", "&document.fire_occupancy,", 2)]:
    sub(rel, old, old.replace("&", ""), c)
sub("🎬️scene-members/🔥️thermal-fire/🦀️.rs", 'format!("{} {} ({}), q_f,k = {} MJ/m², q_f,d = {} MJ/m²"', 'format!("{} {:?} ({}), q_f,k = {} MJ/m², q_f,d = {} MJ/m²"')
INF = "🧬️schema/💡️inferences/🦀️.rs"
sub(INF, "&document.fire_occupancy, part_1_2::OCCUPANCIES", "&format!(\"{:?}\", document.fire_occupancy), part_1_2::OCCUPANCIES") if False else None
p = ROOT / INF
t = p.read_text(encoding="utf-8")
if "(\"occupancy class of the fire load table\", \"Nutzungsklasse der Brandlasttabelle\"), &document.fire_occupancy," in t:
    p.write_text(t.replace("(\"occupancy class of the fire load table\", \"Nutzungsklasse der Brandlasttabelle\"), &document.fire_occupancy,", "(\"occupancy class of the fire load table\", \"Nutzungsklasse der Brandlasttabelle\"), &format!(\"{:?}\", document.fire_occupancy),"), encoding="utf-8")
    n += 1
T = "🧬️schema/🧪️tests/⚖️compliance/🦀️.rs"
sub(T, "AnnexChoice::De, \"office\", 100.0, 1.0)", f"AnnexChoice::De, {M}::Office, 100.0, 1.0)", 4)
sub(T, "AnnexChoice::En, \"office\", 100.0, 1.0)", f"AnnexChoice::En, {M}::Office, 100.0, 1.0)")
sub(T, 'AnnexChoice::De, "spaceport", 100.0, 1.0)', f"AnnexChoice::De, {M}::Office, 100.0, 1.0)", 2)
sub(T, 'AnnexChoice::De, "residential", 25.0, 1.0)', f"AnnexChoice::De, {M}::Dwelling, 25.0, 1.0)")
sub(T, 'fire_load_ndp_ids(OccupancyTable, "mall")', f"fire_load_ndp_ids(OccupancyTable, {M}::Shopping)")
sub(T, 'fire_load_ndp_ids(UserSpecified, "mall")', f"fire_load_ndp_ids(UserSpecified, {M}::Shopping)")
sub(T, 'fire_load_source(OccupancyTable, 420e6, AnnexChoice::De, "office")', f"fire_load_source(OccupancyTable, 420e6, AnnexChoice::De, {M}::Office)")
sub(T, 'fire_load_source(UserSpecified, 420e6, AnnexChoice::De, "office")', f"fire_load_source(UserSpecified, 420e6, AnnexChoice::De, {M}::Office)")
R = "🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs"
p = ROOT / R
t = p.read_text(encoding="utf-8")
o = '    let unknown = En1991Snapshot { fire_occupancy: "spaceport".into(), ..table };\n    assert!(evaluate(&unknown).checks.iter().any(|c| c.id == "en1991.input.fireOccupancy"));\n'
if o in t:
    p.write_text(t.replace(o, ""), encoding="utf-8")
    n += 1
sub(R, '"fireOccupancy" => if s == "office" { "dwelling" } else { "office" }.into(),', '"fireOccupancy" => if s == "Office" { "Dwelling" } else { "Office" }.into(),')
print("edits", n)
