#!/usr/bin/env python3
"""One-shot: hand edits of en1999 call sites that take the typed enums (`&str` parameters, Display uses, validations of unknown values that became unrepresentable). Every replacement asserts its match count."""
import pathlib, re
ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any")
M = "semio_s_artifact_norm_en1999_model"
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

def cut(rel, pattern, count=1, flags=re.S):
    global n
    p = ROOT / rel
    t = p.read_text(encoding="utf-8")
    new, c = re.subn(pattern, "", t, flags=flags)
    if c == 0 and not re.search(pattern, t, flags):
        return
    assert c == count, f"{rel}: expected {count} removals of {pattern[:80]!r}, got {c}"
    p.write_text(new, encoding="utf-8")
    n += 1

CK = "🧬️schema/💡️inferences/⚖️checks/"
SC = "🎬️scene/"
# -- section interaction exponents
ALLOY = CK + "📏️parts/🔩️alloys-and-sections/🦀️.rs"
sub(ALLOY, 'pub fn section_interaction_exponents(kind: &str) -> (f64, f64, f64) {\n    match kind {\n        "tube" | "chs" => (1.5, 1.0, 1.0),\n        "box" | "rhs" | "angle" => (1.0, 1.0, 1.0),\n        _ => (2.0, 1.0, 1.0),',
    f'pub fn section_interaction_exponents(kind: {M}::SectionKind) -> (f64, f64, f64) {{\n    use {M}::SectionKind as K;\n    match kind {{\n        K::Chs => (1.5, 1.0, 1.0),\n        K::Box | K::Rhs | K::Angle => (1.0, 1.0, 1.0),\n        _ => (2.0, 1.0, 1.0),')
sub(ALLOY, "static CACHE: OnceLock<Mutex<HashMap<(String, [u64; 4]), Option<mechanics::SectionConstants>>>>", "static CACHE: OnceLock<Mutex<HashMap<(u8, [u64; 4]), Option<mechanics::SectionConstants>>>>")
sub(ALLOY, "let key = (section.kind.clone(), [", "let key = (section.kind as u8, [")
sub(CK + "🧱️member-checks/🦀️.rs", "section_interaction_exponents(&section.kind)", "section_interaction_exponents(section.kind)", 2)
sub(SC + "📈️diagrams/🦀️.rs", "section_interaction_exponents(&c.section.kind)", "section_interaction_exponents(c.section.kind)")
# -- psi rows
PARTS = CK + "📏️parts/🦀️.rs"
sub(PARTS, 'pub fn psi_row(category: &str) -> PsiRow {\n        let key = match category {\n            "residential" | "A" | "office" | "B" | "self" => "ndp.en1990.psi.office",\n            "congregation" | "C" | "retail" | "D" => "ndp.en1990.psi.congregation",\n            "storage" | "E" => "ndp.en1990.psi.storage",\n            "snow" => "ndp.en1990.psi.snow",\n            "snow_high" => "ndp.en1990.psi.snow_high",\n            "wind" => "ndp.en1990.psi.wind",\n            "temperature" => "ndp.en1990.psi.temperature",\n            "fire" => "ndp.en1999.psi-fire",\n            _ => "ndp.en1999.psi-default",\n        };',
    f'pub fn psi_row(category: {M}::ActionCategory) -> PsiRow {{\n        use {M}::ActionCategory as C;\n        let key = match category {{\n            C::Office | C::SelfWeight => "ndp.en1990.psi.office",\n            C::Snow => "ndp.en1990.psi.snow",\n            C::Wind => "ndp.en1990.psi.wind",\n        }};')
for who in ("other", "lead", "a"):
    sub(PARTS, f"psi_row(&{who}.category)", f"psi_row({who}.category)", {"other": 2, "lead": 2, "a": 1}[who])
# -- shell quality
THIN = CK + "📏️parts/🫙️thin-wall/🦀️.rs"
sub(THIN, 'pub fn shell_quality_q(fabrication_class: &str) -> f64 {\n    semio_s_plugin_norm_compliance::ndp_common(match fabrication_class { "A" => "ndp.en1999.shell-quality-q-class-a", "C" => "ndp.en1999.shell-quality-q-class-c", _ => "ndp.en1999.shell-quality-q" })',
    f'pub fn shell_quality_q(fabrication_class: {M}::FabricationClass) -> f64 {{\n    semio_s_plugin_norm_compliance::ndp_common(match fabrication_class {{ {M}::FabricationClass::A => "ndp.en1999.shell-quality-q-class-a", {M}::FabricationClass::C => "ndp.en1999.shell-quality-q-class-c", {M}::FabricationClass::B => "ndp.en1999.shell-quality-q" }})')
sub(THIN, "an unknown class counts as B.", "")
sub(CK + "🥫️thin-wall-checks/🦀️.rs", "shell_quality_q(shell.fabrication_class.as_str())", "shell_quality_q(shell.fabrication_class)")
# -- product form
sub(ALLOY, 'pub const PRODUCT_FORMS: &[&str] = &["sheet", "extrudedProfile", "extrudedTube", "extrudedBar", "drawnTube", "forging"];\n\n', '')
sub(ALLOY, 'pub fn form_supplies_tube(form: &str) -> bool { matches!(form, "extrudedProfile" | "extrudedTube" | "drawnTube") }', f'pub fn form_supplies_tube(form: {M}::ProductForm) -> bool {{ matches!(form, {M}::ProductForm::ExtrudedProfile | {M}::ProductForm::ExtrudedTube | {M}::ProductForm::DrawnTube) }}')
EVAL = CK + "🧾️structure-evaluation/🦀️.rs"
sub(EVAL, "form_supplies_tube(&mat.product_form)", "form_supplies_tube(mat.product_form)")
sub(EVAL, "is '{}'; Table 3.2 only lists hollow sections as extruded profile, extruded tube or drawn tube.\", member.id, mat.id, mat.product_form)", "is '{:?}'; Table 3.2 only lists hollow sections as extruded profile, extruded tube or drawn tube.\", member.id, mat.id, mat.product_form)")
sub(EVAL, "aber '{}'; Tabelle 3.2", "aber '{:?}'; Tabelle 3.2")
# -- validations that became unrepresentable
cut(EVAL, r"            if !matches!\(el\.weld_process, [^\n]*\n(?:[^\n]*\n){2}            \}\n")
cut(EVAL, r"            if !matches!\(el\.weld_direction, [^\n]*\n(?:[^\n]*\n){2}            \}\n")
cut(EVAL, r"        if !part_1_1::PRODUCT_FORMS\.contains[^\n]*\n(?:[^\n]*\n){2}        \}\n")
cut(EVAL, r"        if !matches!\(shell\.fabrication_class, [^\n]*\n(?:[^\n]*\n){2}        \}\n")
sub(EVAL, "/// 🧪 Refuses unknown weld process, weld direction and fabrication class, and incomplete or misplaced edge stiffeners.", "/// 🧪 Refuses incomplete or misplaced edge stiffeners.")
CONN = CK + "🔗️connection-checks/🦀️.rs"
sub(CONN, "kind: &str, g: &GoverningEffects) -> Option<CheckResult> {", f"kind: {M}::ConnectionKind, g: &GoverningEffects) -> Option<CheckResult> {{\n    use {M}::ConnectionKind as K;")
sub(CONN, 'if kind == "bolted" || kind == "combined" {', "if matches!(kind, K::Bolted | K::Combined) {", 2)
sub(CONN, 'if kind == "pinned" {', "if kind == K::Pinned {", 2)
sub(CONN, 'if kind == "welded" || kind == "combined" {', "if matches!(kind, K::Welded | K::Combined) {", 2)
cut(CONN, r'    let kind = conn\.kind\.to_ascii_lowercase\(\);\n    if !matches!\(kind\.as_str\(\), "bolted" \| "welded" \| "combined" \| "pinned"\) \{\n(?:[^\n]*\n){7}    \}\n')
sub(CONN, "&alloy, &kind, &g)", "&alloy, conn.kind, &g)")
ENT = CK + "🎯️entity-checks/🦀️.rs"
sub(ENT, 'const DETAIL_TYPES: &[&str] = &["plain", "butt-weld", "fillet-weld", "bolted", "riveted", "other"];\n', '')
cut(ENT, r"    if !DETAIL_TYPES\.contains[^\n]*\n[^\n]*\n    \}\n")
cut(ENT, r"    if !matches!\(st\.sides, [^\n]*\n[^\n]*\n    \}\n")
sub(ENT, "({} detail, category {}", "({:?} detail, category {}")
sub(ENT, "({}-Detail, Kerbfall {}", "({:?}-Detail, Kerbfall {}")
OUT = CK + "🔷️outline-checks/🦀️.rs"
sub(OUT, "r.name == v.ring)", f"r.name == format!(\"{{:?}}\", v.ring))")
sub(OUT, "Ring { name: v.ring.clone(),", "Ring { name: format!(\"{:?}\", v.ring),")
sub(OUT, "parametric {} section", "parametric {:?} section")
# action field validations
cut(EVAL, r'    const KINDS: &\[&str\] = [^\n]*\n    const CATS: &\[&str\] = [^\n]*\n    const SOURCES: &\[&str\] = [^\n]*\n')
cut(EVAL, r"        if !KINDS\.contains\(&a\.kind\.as_str\(\)\) \{\n(?:[^\n]*\n)*?        \}\n        if !CATS\.contains\(&a\.category\.as_str\(\)\) \{\n(?:[^\n]*\n)*?        \}\n        if !SOURCES\.contains\(&a\.source\.as_str\(\)\) \{\n(?:[^\n]*\n)*?        \}\n")
# -- scene
KIT = SC + "🧰️kit/🦀️.rs"
sub(KIT, 'match s.kind.as_str() {\n        "tube" | "chs" => {', f'match s.kind {{\n        {M}::SectionKind::Chs => {{')
sub(KIT, '"box" | "rhs" if tf', f'{M}::SectionKind::Box | {M}::SectionKind::Rhs if tf')
sub(KIT, '"angle" if t > 0.0', f'{M}::SectionKind::Angle if t > 0.0')
sub(KIT, '"channel" if tw', f'{M}::SectionKind::Channel if tw')
sub(KIT, '"extrudedI" | "i" if tw', f'{M}::SectionKind::ExtrudedI if tw')
MEMB = SC + "🏗️members/🦀️.rs"
sub(MEMB, 'fn load_kind(kind: &str) -> LoadKind {\n    match kind {\n        "permanent" => LoadKind::Permanent,\n        "imposed" => LoadKind::Variable,\n        "snow" => LoadKind::Snow,\n        "wind" => LoadKind::Wind,\n        "temperature" => LoadKind::Thermal,\n        "fire" => LoadKind::Accidental,\n        _ => LoadKind::Other,\n    }',
    f'fn load_kind(kind: {M}::ActionKind) -> LoadKind {{\n    match kind {{\n        {M}::ActionKind::Permanent => LoadKind::Permanent,\n        {M}::ActionKind::Imposed => LoadKind::Variable,\n        {M}::ActionKind::Snow => LoadKind::Snow,\n        {M}::ActionKind::Wind => LoadKind::Wind,\n        {M}::ActionKind::Temperature => LoadKind::Thermal,\n        {M}::ActionKind::Fire => LoadKind::Accidental,\n    }}')
sub(MEMB, "load_kind(&action.kind)", "load_kind(action.kind)")
sub(MEMB, 'loc(&format!("{} {} weld on {}, a = {:.1} mm, b_haz = {:.1} mm"', 'loc(&format!("{:?} {:?} weld on {}, a = {:.1} mm, b_haz = {:.1} mm"')
sub(MEMB, '&format!("{}-{}-Naht an {}, a = {:.1} mm, b_haz = {:.1} mm"', '&format!("{:?}-{:?}-Naht an {}, a = {:.1} mm, b_haz = {:.1} mm"')
sub(MEMB, 'loc(&format!("{} {}, {:.2} m, {}", material.designation, section.kind,', 'loc(&format!("{} {:?}, {:.2} m, {}", material.designation, section.kind,')
sub(MEMB, '&format!("{} {}, {:.2} m, {}", material.designation, section.kind,', '&format!("{} {:?}, {:.2} m, {}", material.designation, section.kind,')
sub(SC + "🔗️connections/🦀️.rs", 'loc(&format!("{} connection on {}", conn.kind', 'loc(&format!("{:?} connection on {}", conn.kind')
sub(SC + "🔗️connections/🦀️.rs", '&format!("{}-Anschluss an {}", conn.kind', '&format!("{:?}-Anschluss an {}", conn.kind')
FF = SC + "🔥️fire-and-fatigue/🦀️.rs"
sub(FF, '("Fatigue site {} ({} {}): D = {total:.3}"', '("Fatigue site {} ({:?} {}): D = {total:.3}"') if False else None
sub(FF, 'Fatigue site {} ({} {}): D', 'Fatigue site {} ({:?} {}): D')
sub(FF, 'Ermüdungsstelle {} ({} {}): D', 'Ermüdungsstelle {} ({:?} {}): D')
sub(FF, 'loc(&format!("{} {}: D = {total:.2}"', 'loc(&format!("{:?} {}: D = {total:.2}"')
sub(FF, '&format!("{} {}: D = {total:.2}"', '&format!("{:?} {}: D = {total:.2}"')
sub(FF, "over {:.0} mm ({})\"", "over {:.0} mm ({:?})\"")
sub(FF, "über {:.0} mm ({})\"", "über {:.0} mm ({:?})\"")
WP = SC + "📈️diagrams/🦴️web-panels/🦀️.rs"
sub(WP, "{:.0} × {:.0} mm, {}\", s.id, s.position, s.width * 1e3, s.thickness * 1e3, s.sides)", "{:.0} × {:.0} mm, {:?}\", s.id, s.position, s.width * 1e3, s.thickness * 1e3, s.sides)", 2)
print("edits", n)

# ---- part 2: weld runs, outline ring, test helpers and tests of unrepresentable values
WR = CK + "🧵️weld-run-checks/🦀️.rs"
cut(WR, r"    if !matches!\(run\.process, [^\n]*\n(?:[^\n]*\n){3}    \}\n")
cut(WR, r"    if !matches!\(run\.direction, [^\n]*\n(?:[^\n]*\n){3}    \}\n")
sub(WR, "if let (Some(el), true) = (element, finite && matches!(run.process, semio_s_artifact_norm_en1999_model::WeldProcess::Mig | semio_s_artifact_norm_en1999_model::WeldProcess::Tig)) {", "if let (Some(el), true) = (element, finite) {")
sub(WR, "({} weld{}), HAZ reach", "({:?} weld{}), HAZ reach")
sub(WR, "({}-Naht{}), WEZ-Reichweite", "({:?}-Naht{}), WEZ-Reichweite")
sub(WR, "run.process.clone()", "run.process") if False else None
sub(OUT, "r.name == format!(\"{:?}\", v.ring)", "r.name == ring_name(v.ring)")
sub(OUT, "Ring { name: format!(\"{:?}\", v.ring),", "Ring { name: ring_name(v.ring).to_owned(),")
sub(OUT, "/// 🔷️ The rings of one section in order of first appearance.", f"/// 🏷️ The ring name the outline checks and the scene use.\nfn ring_name(ring: {M}::OutlineRing) -> &'static str {{\n    match ring {{\n        {M}::OutlineRing::Outer => \"outer\",\n        {M}::OutlineRing::Hole => \"hole\",\n    }}\n}}\n\n/// 🔷️ The rings of one section in order of first appearance.")
T = CK.replace("💡️inferences/⚖️checks/", "") + "🧪️tests/⚖️compliance/"
RW = T + "🔬️resistance-worked-examples/🦀️.rs"
cut(RW, r'    let mut bad = En1999Snapshot::compliant_roof_purlin\(\);\n    bad\.sections\[0\]\.elements\[0\]\.weld_process = [^\n]*\n    bad\.shells\[0\]\.fabrication_class = [^\n]*\n    let report = evaluate_structure\(&bad\);\n    assert!\(report\.failing\(\)\.any\(\|c\| c\.id\.starts_with\("en1999\.ref\.element\.process\."\)\)\);\n    assert!\(report\.failing\(\)\.any\(\|c\| c\.id\.starts_with\("en1999\.ref\.shell\.class\."\)\)\);\n')
cut(RW, r'#\[test\]\nfn unknown_connection_kind_fails_instead_of_passing_as_not_applicable\(\) \{\n(?:[^\n]*\n)*?\}\n\n')
sub(RW, "fn a_pinned_connection_emits_the_pin_checks_and_a_known_kind()", "fn a_pinned_connection_emits_the_pin_checks()")
sub(RW, "AluminiumSection { id: kind.into(), kind: kind.into(),", f"AluminiumSection {{ id: kind.into(), kind: match kind {{ \"tube\" | \"chs\" => {M}::SectionKind::Chs, \"rhs\" => {M}::SectionKind::Rhs, \"box\" => {M}::SectionKind::Box, \"angle\" => {M}::SectionKind::Angle, \"channel\" => {M}::SectionKind::Channel, \"free\" => {M}::SectionKind::Free, _ => {M}::SectionKind::ExtrudedI }},")
WRO = T + "🧵️weld-runs-and-outlines/🦀️.rs"
sub(WRO, "ring: ring.into(),", f"ring: if ring == \"hole\" {{ {M}::OutlineRing::Hole }} else {{ {M}::OutlineRing::Outer }},")
cut(WRO, r'    bad\.process = "laser"\.into\(\);\n')
sub(WRO, 'for id in ["member", "process", "direction", "throat"] {', 'for id in ["member", "throat"] {')
sub(WRO, "fn a_run_with_unknown_member_element_process_or_geometry_fails_with_a_remedy()", "fn a_run_with_unknown_member_element_or_geometry_fails_with_a_remedy()")
OW = CK + "🧪️tests/🎯️oracle-wiring/🦀️.rs"
sub(OW, "AluminiumSection { id: String::new(), kind: kind.into(),", f"AluminiumSection {{ id: String::new(), kind: match kind {{ \"tube\" | \"chs\" => {M}::SectionKind::Chs, \"rhs\" => {M}::SectionKind::Rhs, \"box\" => {M}::SectionKind::Box, \"angle\" => {M}::SectionKind::Angle, \"channel\" => {M}::SectionKind::Channel, \"free\" => {M}::SectionKind::Free, _ => {M}::SectionKind::ExtrudedI }},")
W5 = CK + "🧪️tests/🎯️wave5-references/🦀️.rs"
cut(W5, r'    doc\.transverse_stiffeners\[0\]\.sides = "three"\.into\(\);\n    assert!\(evaluate_structure\(&doc\)\.checks\.iter\(\)\.any\(\|c\| c\.id == "en1999\.6\.7\.6\.web-sides\.stiffener-1"\)\);\n')
sub(W5, "sides: sides.into()", f"sides: if sides == \"one\" {{ {M}::StiffenerSides::One }} else {{ {M}::StiffenerSides::Two }}")
print("edits", n)
