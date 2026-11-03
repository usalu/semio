#!/usr/bin/env python3
"""One-shot (typed enums v4): hand edits of en1998 call sites that take the typed enums (`&str` helper signatures, Display uses, string checks that became unrepresentable). Every replacement asserts its match count; reruns are no-ops."""
import pathlib
ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any")
M = "semio_s_artifact_norm_en1998_model"
n = 0

def sub(rel, old, new, count=1):
    global n
    p = ROOT / rel
    t = p.read_text(encoding="utf-8")
    if old not in t and new in t:
        return
    c = t.count(old)
    if c == 0:
        print("SKIP (absent):", rel, repr(old[:70]))
        return
    assert c == count, f"{rel}: expected {count} of {old[:90]!r}, found {c}"
    p.write_text(t.replace(old, new), encoding="utf-8")
    n += 1

F = "🧬️schema/📐️formulas/🦀️.rs"
SNAP = "🧬️schema/📸️snapshot/🦀️.rs"
INF = "🧬️schema/💡️inferences/"
SC = INF + "🌍️full-seismic-check/"
SYS = SC + "🧱️system-checks/"

sub(SNAP, "pub enum LimitState {", "pub enum LimitState {")
t = (ROOT / SNAP).read_text(encoding="utf-8")
if "pub const fn letter" not in t:
    t = t.rstrip("\n") + f"""

impl EnGroundType {{
    /// 🔤 Ground type letter A to E of EN 1998-1 Table 3.1.
    pub const fn letter(self) -> char {{
        match self {{
            Self::A => 'A',
            Self::B => 'B',
            Self::C => 'C',
            Self::D => 'D',
            Self::E => 'E',
        }}
    }}
}}

impl LimitState {{
    /// 🔤 Lower-case key `nc`, `sd` or `dl` of the limit state.
    pub const fn token(self) -> &'static str {{
        match self {{
            Self::Nc => "nc",
            Self::Sd => "sd",
            Self::Dl => "dl",
        }}
    }}
}}

impl AnalysisDirection {{
    /// 🔤 Lower-case key `x` or `y` of the plan direction.
    pub const fn token(self) -> &'static str {{
        match self {{
            Self::X => "x",
            Self::Y => "y",
        }}
    }}
}}
"""
    (ROOT / SNAP).write_text(t, encoding="utf-8"); n += 1

sub(F, '''    pub fn psi2_de(category: &str) -> f64 {
        let key = match category.trim().to_ascii_lowercase().as_str() {
            "residential" | "a" => "residential",
            "office" | "b" => "office",
            "congregation" | "c" => "congregation",
            "retail" | "d" => "retail",
            "storage" | "e" => "storage",
            "traffic" | "f" => "traffic_light",
            "vehicles" | "g" => "traffic_heavy",
            "roofs" | "h" => "roof",
            "snow" => "snow_high",
            "wind" => "wind",
            _ => "residential",
        };''', f'''    pub fn psi2_de(category: {M}::ImposedCategory) -> f64 {{
        use {M}::ImposedCategory as C;
        let key = match category {{
            C::A => "residential",
            C::B => "office",
            C::C => "congregation",
            C::D => "retail",
            C::E => "storage",
            C::F => "traffic_light",
            C::G => "traffic_heavy",
            C::H => "roof",
            C::Snow => "snow_high",
            C::Wind => "wind",
        }};''')
sub(F, '''    pub fn phi_table_4_2(category: &str, is_roof: bool, correlated_occupancy: bool) -> f64 {
        match category.trim().to_ascii_lowercase().as_str() {
            "retail" | "d" | "storage" | "e" | "traffic" | "f" | "archives" | "archive" => 1.0,
            _ if is_roof => 1.0,''', f'''    pub fn phi_table_4_2(category: {M}::ImposedCategory, is_roof: bool, correlated_occupancy: bool) -> f64 {{
        use {M}::ImposedCategory as C;
        match category {{
            C::D | C::E | C::F => 1.0,
            _ if is_roof => 1.0,''')
sub(F, "pub fn psi_e(category: &str,", f"pub fn psi_e(category: {M}::ImposedCategory,")
sub(F, "psi_e(&v.category,", "psi_e(v.category,", 2)
sub(F, "content_category: &str, filling_ratio", f"content_category: {M}::ImposedCategory, filling_ratio")
sub(F, '''    pub fn gamma_i(importance: &str) -> f64 {
        let class = match importance.trim().to_ascii_uppercase().as_str() {
            "I" | "CC1" => "i",
            "III" | "CC3" => "iii",
            "IV" | "CC4" => "iv",
            _ => "ii",
        };''', f'''    pub fn gamma_i(importance: {M}::ImportanceClass) -> f64 {{
        use {M}::ImportanceClass as I;
        let class = match importance {{
            I::I => "i",
            I::III => "iii",
            I::IV => "iv",
            I::II => "ii",
        }};''')
sub(F, '''    fn class_key(ductility: &str) -> &'static str {
        match ductility.trim().to_ascii_lowercase().as_str() {
            "dch" => "dch",
            "dcm" => "dcm",
            _ => "low",
        }''', f'''    fn class_key(ductility: {M}::DuctilityClass) -> &'static str {{
        match ductility {{
            {M}::DuctilityClass::Dch => "dch",
            {M}::DuctilityClass::Dcm => "dcm",
            {M}::DuctilityClass::Dcl => "low",
        }}''')
sub(F, '''    pub fn q_max_for_system(system_type: &str, material: &str, ductility: &str) -> f64 {
        let st = system_type.trim().to_ascii_lowercase();
        let mat = material.trim().to_ascii_lowercase();
        let dc = ductility.trim().to_ascii_lowercase();
        let class = match dc.as_str() {
            "dch" => "dch",
            "dcm" => "dcm",
            _ => "low",
        };
        let system = match (mat.as_str(), st.as_str()) {
            ("rc" | "concrete", system @ ("frame" | "wall" | "dual")) => format!("rc.{system}"),
            ("steel", "frame") => "steel.frame".to_owned(),
            ("steel", "wall" | "dual") => "steel.wall".to_owned(),
            ("composite", _) => "composite".to_owned(),
            ("timber", _) => "timber".to_owned(),
            ("masonry", _) => "masonry".to_owned(),
            _ => return semio_s_plugin_norm_compliance::ndp_common("ndp.en1998.en1.q-max.default"),
        };''', f'''    pub fn q_max_for_system(system_type: {M}::SystemType, material: {M}::StructuralMaterial, ductility: {M}::DuctilityClass) -> f64 {{
        use {M}::{{StructuralMaterial as Mat, SystemType as Sys}};
        let class = class_key(ductility);
        let system = match (material, system_type) {{
            (Mat::Rc, Sys::Frame) => "rc.frame".to_owned(),
            (Mat::Rc, Sys::Wall) => "rc.wall".to_owned(),
            (Mat::Rc, Sys::Dual) => "rc.dual".to_owned(),
            (Mat::Steel, Sys::Frame) => "steel.frame".to_owned(),
            (Mat::Steel, Sys::Wall | Sys::Dual) => "steel.wall".to_owned(),
            (Mat::Composite, _) => "composite".to_owned(),
            (Mat::Timber, _) => "timber".to_owned(),
            (Mat::Masonry, _) => "masonry".to_owned(),
            _ => return semio_s_plugin_norm_compliance::ndp_common("ndp.en1998.en1.q-max.default"),
        }};''')
for fn in ("rc_column_min_b_m", "rc_omega_wd_min"):
    sub(F, f"pub fn {fn}(ductility: &str)", f"pub fn {fn}(ductility: {M}::DuctilityClass)")
sub(F, "pub fn rc_rho_limits(role: &str, ductility: &str)", f"pub fn rc_rho_limits(role: &str, ductility: {M}::DuctilityClass)")
sub(F, '''    pub fn drift_theta(limit_class: &str) -> f64 {
        let class = match limit_class.trim().to_ascii_lowercase().as_str() {
            "brittle" => "brittle",
            "isolated" => "isolated",
            _ => "ductile",
        };''', f'''    pub fn drift_theta(limit_class: {M}::DriftLimitClass) -> f64 {{
        let class = match limit_class {{
            {M}::DriftLimitClass::Brittle => "brittle",
            {M}::DriftLimitClass::Isolated => "isolated",
            {M}::DriftLimitClass::Ductile => "ductile",
        }};''')
sub(F, '''    pub fn fundamental_period_s(method: &str, ct: f64, t1_given_s: f64, total_height_m: f64, masses_kg: &[f64], stiffness_x: &[f64], stiffness_y: &[f64]) -> (f64, bool) {
        match method.trim().to_ascii_lowercase().as_str() {
            "given" => (t1_given_s, t1_given_s > 0.0),
            "rayleigh" => {''', f'''    pub fn fundamental_period_s(method: {M}::T1Method, ct: f64, t1_given_s: f64, total_height_m: f64, masses_kg: &[f64], stiffness_x: &[f64], stiffness_y: &[f64]) -> (f64, bool) {{
        match method {{
            {M}::T1Method::Given => (t1_given_s, t1_given_s > 0.0),
            {M}::T1Method::Rayleigh => {{''')
sub(F, "            _ => (t1_from_ct(ct, total_height_m), true),\n        }\n    }\n\n    pub fn lambda_factor", "            {M}::T1Method::Ct => (t1_from_ct(ct, total_height_m), true),\n        }\n    }\n\n    pub fn lambda_factor".replace("{M}", M))
sub(F, '''    pub fn confidence_factor(kl: &str) -> f64 {
        match kl.trim().to_ascii_lowercase().as_str() {
            "kl2" => 1.20,
            "kl3" => 1.00,
            _ => 1.35,
        }''', f'''    pub fn confidence_factor(kl: {M}::KnowledgeLevel) -> f64 {{
        match kl {{
            {M}::KnowledgeLevel::Kl1 => 1.35,
            {M}::KnowledgeLevel::Kl2 => 1.20,
            {M}::KnowledgeLevel::Kl3 => 1.00,
        }}''')
sub(F, '''        base * match importance.trim().to_ascii_uppercase().as_str() {
            "III" | "IV" => ''', f'''        base * match importance {{
            {M}::ImportanceClass::III | {M}::ImportanceClass::IV => ''')
sub(F, "            _ => 1.0,\n        }\n    }\n\n    /// 🧾️ Registry ids that the simple masonry", "            _ => 1.0,\n        }\n    }\n\n    /// 🧾️ Registry ids that the simple masonry")
sub(F, "pub fn masonry_wall_ratio_limit(a_gr: f64, importance: &str)", f"pub fn masonry_wall_ratio_limit(a_gr: f64, importance: {M}::ImportanceClass)")
sub(F, ".or_else(|| site.en_ground_type.chars().next().map(|c| c.to_ascii_uppercase()))", ".or_else(|| Some(site.en_ground_type.letter()))")

sub(INF + "🌊️geotechnical-details/🦀️.rs", "document.site.en_ground_type.chars().next().map(|c| c.to_ascii_uppercase())", "Some(document.site.en_ground_type.letter())")
sub(SC + "🦀️.rs", "document.site.en_ground_type.chars().next().unwrap_or('?').to_ascii_uppercase()", "document.site.en_ground_type.letter()")
sub(SC + "🦀️.rs", "combo == expected_combo || document.site.en_ground_type.is_empty()", "combo == expected_combo")
sub(SC + "🦀️.rs", "part_1::gamma_i(&document.site.importance_class)", "part_1::gamma_i(document.site.importance_class)")
BD = SC + "🧩️building-details/🦀️.rs"
sub(BD, "document.site.en_ground_type.chars().next().unwrap_or('?').to_ascii_uppercase()", "document.site.en_ground_type.letter()")
sub(BD, " || (!matches!(annex, AnnexChoice::De) && document.site.en_ground_type.is_empty())", "")
sub(BD, f'matches!(x.system_type.to_ascii_lowercase().as_str(), "frame" | "dual") && !x.ductility_class == {M}::DuctilityClass::Dcl', f'matches!(x.system_type, {M}::SystemType::Frame | {M}::SystemType::Dual) && x.ductility_class != {M}::DuctilityClass::Dcl', 2)
sub(INF + "🧰️seismic-helpers/🦀️.rs", 'let type1 = !document.site.en_spectrum_type.to_ascii_lowercase().contains("2");', f"let type1 = document.site.en_spectrum_type == {M}::EnSpectrumType::Type1;")
sub("🎬️scene/🧮️response/🦀️.rs", "spectrum_type1: !document.site.en_spectrum_type.to_ascii_lowercase().contains('2')", f"spectrum_type1: document.site.en_spectrum_type == {M}::EnSpectrumType::Type1")
sub("🎬️scene/🧮️response/🦀️.rs", "part_1::gamma_i(&document.site.importance_class)", "part_1::gamma_i(document.site.importance_class)")
sub("🎬️scene/🧮️response/🦀️.rs", "fundamental_period_s(&building.t1_method,", "fundamental_period_s(building.t1_method,")
sub("🎬️scene/🧮️response/🦀️.rs", "&system.system_type, &system.material, &system.ductility_class", "system.system_type, system.material, system.ductility_class")
sub("🎬️scene/🌋️capacity/🦀️.rs", "&system.system_type, &system.material, &system.ductility_class", "system.system_type, system.material, system.ductility_class")
sub("🎬️scene/🌋️capacity/🧪️tests/🦀️.rs", "&system.system_type, &system.material, &system.ductility_class", "system.system_type, system.material, system.ductility_class")
sub("🎬️scene/🌋️capacity/🦀️.rs", "confidence_factor(&a.knowledge_level)", "confidence_factor(a.knowledge_level)")
sub("🎬️scene/🌋️capacity/🦀️.rs", "let state = a.limit_state.to_ascii_lowercase();", "let state = a.limit_state.token();")
sub("🎬️scene/🌋️capacity/🦀️.rs", "a.limit_state.to_uppercase()", "a.limit_state.token().to_uppercase()", 2)
sub("🎬️scene/🏢️buildings/🦀️.rs", "part_1::masonry_wall_ratio_limit(site.a_gr, &input.document.site.importance_class)", "part_1::masonry_wall_ratio_limit(site.a_gr, input.document.site.importance_class)")
sub("🎬️scene/🏢️buildings/🦀️.rs", '''    match building.systems.first().map(|s| s.material.to_ascii_lowercase()).as_deref() {
        Some("steel" | "composite") => MaterialRole::Steel,
        Some("timber") => MaterialRole::Timber,
        Some("masonry") => MaterialRole::Masonry,''', f'''    match building.systems.first().map(|s| s.material) {{
        Some({M}::StructuralMaterial::Steel | {M}::StructuralMaterial::Composite) => MaterialRole::Steel,
        Some({M}::StructuralMaterial::Timber) => MaterialRole::Timber,
        Some({M}::StructuralMaterial::Masonry) => MaterialRole::Masonry,''')
sub("🎬️scene/🏢️buildings/🦀️.rs", "a.limit_state.to_uppercase()", "a.limit_state.token().to_uppercase()", 2)
sub("🎬️scene/🏢️buildings/🦀️.rs", "a.knowledge_level.to_uppercase()", 'format!("{:?}", a.knowledge_level).to_uppercase()', 2)
sub("🎬️scene/📈️diagrams/🦀️.rs", "drift_theta(&b.drift_limit_class)", "drift_theta(b.drift_limit_class)")
sub("🎬️scene/🌉️ground-and-bridges/🦀️.rs", '"Bearing {} ({}) of {}"', '"Bearing {} ({:?}) of {}"')
sub("🎬️scene/🌉️ground-and-bridges/🦀️.rs", '"Lager {} ({}) von {}"', '"Lager {} ({:?}) von {}"')
sub("🎬️scene/🌉️ground-and-bridges/🦀️.rs", '"Bearing {}, {}, k =', '"Bearing {}, {:?}, k =')
sub("🎬️scene/🧱️horizontal-elements/🦀️.rs", '''fn material_role(material: &str) -> MaterialRole {
    match material.to_ascii_lowercase().as_str() {
        "timber" => MaterialRole::Timber,
        "steel" | "steel-deck" => MaterialRole::Steel,
        _ => MaterialRole::Concrete,
    }''', f'''fn material_role(material: {M}::DiaphragmMaterial) -> MaterialRole {{
    match material {{
        {M}::DiaphragmMaterial::Timber => MaterialRole::Timber,
        {M}::DiaphragmMaterial::SteelDeck => MaterialRole::Steel,
        {M}::DiaphragmMaterial::Concrete => MaterialRole::Concrete,
    }}''')
sub("🎬️scene/🧱️horizontal-elements/🦀️.rs", "material_role(&d.material)", "material_role(d.material)")
sub("🎬️scene/🧱️horizontal-elements/🦀️.rs", '"{} diaphragm, t =', '"{:?} diaphragm, t =')
sub("🎬️scene-structures/🏷️variables/🦀️.rs", "category: row.category.clone(),", 'category: format!("{:?}", row.category),')
sub("🎬️scene-structures/🦀️.rs", "&silo.content_category", "silo.content_category")
sub(INF + "⚖️part-evaluators/🦀️.rs", "&silo.content_category", "silo.content_category")
sub(INF + "⚖️part-evaluators/🦀️.rs", "fundamental_period_s(&building.t1_method,", "fundamental_period_s(building.t1_method,")
sub(INF + "⚖️part-evaluators/🦀️.rs", "confidence_factor(&a.knowledge_level)", "confidence_factor(a.knowledge_level)")
sub(INF + "⚖️part-evaluators/🦀️.rs", "let ls = a.limit_state.to_ascii_lowercase();", "let ls = a.limit_state.token();")
sub(INF + "⚖️part-evaluators/🦀️.rs", 'let ls_ok = matches!(ls.as_str(), "nc" | "near_collapse" | "near-collapse" | "sd" | "significant_damage" | "significant-damage" | "dl" | "damage_limitation" | "damage-limitation");\n        let mut ls_c', 'let ls_c')
sub(INF + "⚖️part-evaluators/🦀️.rs", ".status(if ls_ok { semio_s_plugin_norm_compliance::CheckStatus::Pass } else { semio_s_plugin_norm_compliance::CheckStatus::Fail })", ".status(semio_s_plugin_norm_compliance::CheckStatus::Pass)")
sub(INF + "⚖️part-evaluators/🦀️.rs", '''        if !ls_ok {
            ls_c = ls_c.remedy(Remedy::one_of(SubjectRef::new(&a.id, format!("{path}.limitState"), lc("Limit state", "Grenzzustand")), vec!["sd".into(), "nc".into(), "dl".into()], lc("Set limitState to nc, sd, or dl (Table 2.1).", "limitState auf nc, sd oder dl setzen (Tab. 2.1).")));
        }
''', "")
sub(INF + "⚖️part-evaluators/🦀️.rs", "a.limit_state),", "a.limit_state.token()),", 3)
PD = INF + "🪨️part-details/🦀️.rs"
sub(PD, "let ls = a.limit_state.to_ascii_lowercase();", "let ls = a.limit_state.token();", 2)
sub(PD, "confidence_factor(&a.knowledge_level)", "confidence_factor(a.knowledge_level)")
sub(PD, "a.chord_rotation_demand_rad, a.limit_state, capacity.yield_rad", "a.chord_rotation_demand_rad, a.limit_state.token(), capacity.yield_rad", 2)
RB = SC + "🏢️building-regularity-and-base-shear-checks/🦀️.rs"
sub(RB, "fundamental_period_s(&building.t1_method,", "fundamental_period_s(building.t1_method,")
sub(RB, 'building.t1_method.to_ascii_lowercase() == "given"', f"building.t1_method == {M}::T1Method::Given")
sub(RB, "building.t1_method, building.t1_given_s", 'format!("{:?}", building.t1_method), building.t1_given_s', 4)
sub(RB, '''let types: Vec<String> = building.systems.iter().map(|s| s.system_type.to_ascii_lowercase()).collect();
        let has_frame = types.iter().any(|t| t == "frame" || t == "dual");
        let has_wall = types.iter().any(|t| t == "wall" || t == "dual" || t.contains("wall"));''', f'''let has_frame = building.systems.iter().any(|s| matches!(s.system_type, {M}::SystemType::Frame | {M}::SystemType::Dual));
        let has_wall = building.systems.iter().any(|s| matches!(s.system_type, {M}::SystemType::Wall | {M}::SystemType::Dual));''')
DB = SYS + "🧭️direction-and-base-shear/🦀️.rs"
sub(DB, "s.direction.to_ascii_lowercase() == system.direction.to_ascii_lowercase()", "s.direction == system.direction")
sub(DB, 'let known = matches!(system.direction.to_ascii_lowercase().as_str(), "x" | "y");', "let known = true;")
sub(DB, "system.direction, known, dup", "system.direction.token(), known, dup", 2)
sub(DB, "system.direction, f_b,", "system.direction.token(), f_b,", 2)
sub(DB, "&system.system_type, &system.material, &system.ductility_class", "system.system_type, system.material, system.ductility_class")
sub(DB, "for {}/{}/{}\"", "for {:?}/{:?}/{:?}\"")
sub(DB, "für {}/{}/{}\"", "für {:?}/{:?}/{:?}\"")
sub(SYS + "🦀️.rs", "system.direction.to_ascii_lowercase(), f_b", "system.direction.token().to_string(), f_b")
DM = SYS + "🔗️ductility-and-masonry/🦀️.rs"
sub(DM, "let gov_dc = &system.ductility_class;", "let gov_dc = system.ductility_class;")
sub(DM, '''        let mat = member.material.to_ascii_lowercase();
        if mat == "rc" || mat == "concrete" {''', f'''        let mat = member.material;
        if mat == {M}::StructuralMaterial::Rc {{''')
sub(DM, 'if mat == "steel" {', f"if mat == {M}::StructuralMaterial::Steel {{")
sub(DM, "masonry_wall_ratio_limit(a_gr, &document.site.importance_class)", "masonry_wall_ratio_limit(a_gr, document.site.importance_class)")
sub(SYS + "🏬️storey-forces-and-drift/🦀️.rs", "drift_theta(&building.drift_limit_class)", "drift_theta(building.drift_limit_class)")
sub(SYS + "🪧️horizontal-elements/🦀️.rs", "(direction", "(direction", 0) if False else None
sub("🧬️schema/🧱️wall-plan/🦀️.rs", "pub material: String,", f"pub material: {M}::StructuralMaterial,")
sub("🧬️schema/🧱️wall-plan/🦀️.rs", "material: m.material.to_ascii_lowercase(),", "material: m.material,")
sub("🧬️schema/🧪️tests/⚖️compliance/🦀️.rs", "confidence_factor(&a.knowledge_level)", "confidence_factor(a.knowledge_level)")
print("manual edits", n)
