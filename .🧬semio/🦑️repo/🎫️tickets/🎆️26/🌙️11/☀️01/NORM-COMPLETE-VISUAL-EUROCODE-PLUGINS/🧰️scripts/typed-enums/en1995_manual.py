#!/usr/bin/env python3
"""One-shot: hand edits for the en1995 conversions (string-keyed helpers take the typed enums, Display uses, sqlite document row, tests of unrepresentable values). Each replacement asserts its match count."""
import pathlib, re
ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any")
M = "semio_s_artifact_norm_en1995_model"
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

S = "🧬️schema/⚖️timber/"
PSI = S + "🌲️material-and-annex-factors/🦀️.rs"
sub(PSI, '''pub fn psi_factors(factors: &semio_s_plugin_norm_compliance::ActionFactors, kind: &str, category: &str) -> (f64, f64, f64) {
    let key = match kind.trim().to_ascii_lowercase().as_str() {
        "permanent" | "accidental" => return semio_s_plugin_norm_compliance::action_factors::PsiRow::UNITY.as_tuple(),
        k @ ("snow" | "snow_high" | "wind") => k.to_string(),
        "imposed" => category.trim().to_ascii_uppercase(),
        _ => "office".to_string(),
    };
    factors.psi_known(&key)''', f'''pub fn psi_factors(factors: &semio_s_plugin_norm_compliance::ActionFactors, kind: {M}::ActionKind, category: {M}::ImposedCategory) -> (f64, f64, f64) {{
    use {M}::{{ActionKind as K, ImposedCategory as C}};
    let key = match kind {{
        K::Permanent | K::Accidental => return semio_s_plugin_norm_compliance::action_factors::PsiRow::UNITY.as_tuple(),
        K::Snow => "snow",
        K::Wind => "wind",
        K::Imposed => match category {{
            C::NotImposed => "office",
            C::A => "A",
            C::B => "B",
            C::C => "C",
            C::D => "D",
            C::E => "E",
            C::F => "F",
            C::G => "G",
            C::H => "H",
        }},
    }};
    factors.psi_known(key)''')
sub(PSI, '''pub fn with_uls_equation(mut self, equation: &str) -> Self {
        self.uls_ab = equation == "6.10ab";''', f'''pub fn with_uls_equation(mut self, equation: {M}::UlsEquation) -> Self {{
        self.uls_ab = equation == {M}::UlsEquation::Eq610ab;''')
sub(PSI, "`6.10ab` enables", "`Eq610ab` enables")
LC = S + "🧮️load-combinations/🦀️.rs"
sub(LC, '''pub(super) fn action_kind_family(kind: &str) -> &'static str {
    match kind.trim().to_ascii_lowercase().as_str() {
        "permanent" | "g" | "dead" => "permanent",
        "accidental" | "a" => "accidental",
        "snow" => "snow",
        "wind" => "wind",
        _ => "imposed",
    }''', f'''pub(super) fn action_kind_family(kind: {M}::ActionKind) -> &'static str {{
    use {M}::ActionKind as K;
    match kind {{
        K::Permanent => "permanent",
        K::Accidental => "accidental",
        K::Snow => "snow",
        K::Wind => "wind",
        K::Imposed => "imposed",
    }}''')
sub(LC, "action_kind_family(&a.kind)", "action_kind_family(a.kind)", 3)
sub(LC, "psi_factors(&fac, &m.actions[v].kind, &m.actions[v].category)", "psi_factors(&fac, m.actions[v].kind, m.actions[v].category)", 7)
sub(LC, 'psi_factors(&fac, &c.actions[v].kind, "")', f"psi_factors(&fac, c.actions[v].kind, {M}::ImposedCategory::NotImposed)")
sub(LC, 'psi_factors(&fac, "imposed", "B")', f"psi_factors(&fac, {M}::ActionKind::Imposed, {M}::ImposedCategory::B)")
sub(LC, "parse_load_duration(&a.load_duration)", "load_duration_of(a.load_duration)", 2)
sub(LC, "parse_load_duration(&m.actions[i].load_duration)", "load_duration_of(m.actions[i].load_duration)")
sub(LC, "parse_load_duration(&c.actions[i].load_duration)", "load_duration_of(c.actions[i].load_duration)")
KER = "🧬️schema/⚖️kernel/🦀️.rs"
sub(KER, '''/// 🔤 Duration class from its spelling; unknown spellings read as medium.''', f'''/// 🔁 The kernel duration class of the persisted load-duration class.
pub fn load_duration_of(class: {M}::LoadDurationClass) -> LoadDuration {{
    match class {{
        {M}::LoadDurationClass::Permanent => LoadDuration::Permanent,
        {M}::LoadDurationClass::Long => LoadDuration::Long,
        {M}::LoadDurationClass::Medium => LoadDuration::Medium,
        {M}::LoadDurationClass::Short => LoadDuration::Short,
        {M}::LoadDurationClass::Instantaneous => LoadDuration::Instantaneous,
    }}
}}

/// 🔤 Duration class from its spelling (catalogue inputs); unknown spellings read as medium.''')
sub("🎬️scene/📈️diagrams/🦀️.rs", "t::parse_load_duration(&a.load_duration)", "t::load_duration_of(a.load_duration)")
STR = S + "🏗️structure-and-members/🦀️.rs"
sub(STR, "action_kind_family(&a.kind) == \"permanent\"", "action_kind_family(a.kind) == \"permanent\"")
sub(STR, "with_uls_equation(&document.uls_equation)", "with_uls_equation(document.uls_equation)")
sub(S + "🧱️entities/🦀️.rs", "with_uls_equation(&document.uls_equation)", "with_uls_equation(document.uls_equation)")
sub("🧬️schema/🧪️tests/⚖️compliance/🦀️.rs", 'with_uls_equation("6.10ab")', f"with_uls_equation({M}::UlsEquation::Eq610ab)")
# fastener type
BR = S + "🌉️bridge-and-connections/🦀️.rs"
sub(BR, '''    let ft = c.fastener_type.to_ascii_lowercase();
    let kind = if ft.contains("nail") { "nail" } else if ft.contains("screw") { "screw" } else if ft.contains("bolt") { "bolt" } else { "dowel" };''', f'''    use {M}::FastenerType as F;
    let kind = match c.fastener_type {{ F::Nail => "nail", F::Screw => "screw", F::Bolt => "bolt", F::Dowel => "dowel" }};''')
sub(S + "🔩️fasteners-and-johansen/🦀️.rs", '''    let ft = c.fastener_type.to_ascii_lowercase();
    let kind = if ft.contains("nail") || ft.contains("screw") { "nail" } else if ft.contains("bolt") { "bolt" } else { "dowel" };''', f'''    use {M}::FastenerType as F;
    let kind = match c.fastener_type {{ F::Nail | F::Screw => "nail", F::Bolt => "bolt", F::Dowel => "dowel" }};''')
ENT = S + "🧱️entities/🦀️.rs"
sub(ENT, "let kind = c.fastener_type.as_str();", "let kind = fastener_name(c.fastener_type);")
sub(ENT, '("kind", c.fastener_type.as_str())', '("kind", fastener_name(c.fastener_type))')
sub(ENT, "/// 🧱️ The lamella class the kernel evaluates a panel with: the strength class of the panel.", f"/// 🔩 The lower-case name of a fastener type as the kernel tables spell it.\nfn fastener_name(t: {M}::FastenerType) -> &'static str {{\n    match t {{ {M}::FastenerType::Nail => \"nail\", {M}::FastenerType::Screw => \"screw\", {M}::FastenerType::Bolt => \"bolt\", {M}::FastenerType::Dowel => \"dowel\" }}\n}}\n\n/// 🧱️ The lamella class the kernel evaluates a panel with: the strength class of the panel.")
FACTS = "🎬️scene/🧮️facts/🦀️.rs"
sub(FACTS, '''pub fn family_of(fastener_type: &str) -> &'static str {
    let ft = fastener_type.to_ascii_lowercase();
    if ft.contains("nail") { "nail" } else if ft.contains("screw") { "screw" } else if ft.contains("bolt") { "bolt" } else { "dowel" }''', f'''pub fn family_of(fastener_type: {M}::FastenerType) -> &'static str {{
    use {M}::FastenerType as F;
    match fastener_type {{ F::Nail => "nail", F::Screw => "screw", F::Bolt => "bolt", F::Dowel => "dowel" }}''')
sub(FACTS, "family_of(&connection.fastener_type)", "family_of(connection.fastener_type)")
sub(FACTS, '''let permanent = |k: &str| matches!(k.trim().to_ascii_lowercase().as_str(), "permanent" | "g" | "dead");''', f'''let permanent = |k: {M}::ActionKind| k == {M}::ActionKind::Permanent;''')
sub(FACTS, "filter(|a| permanent(&a.kind))", "filter(|a| permanent(a.kind))")
sub("🎬️scene/🪵️members/🦀️.rs", '''fn load_kind(kind: &str) -> LoadKind {
    match kind.trim().to_ascii_lowercase().as_str() {
        "permanent" | "g" | "dead" => LoadKind::Permanent,
        "snow" => LoadKind::Snow,
        "wind" => LoadKind::Wind,
        "accidental" => LoadKind::Accidental,
        _ => LoadKind::Variable,
    }''', f'''fn load_kind(kind: {M}::ActionKind) -> LoadKind {{
    match kind {{
        {M}::ActionKind::Permanent => LoadKind::Permanent,
        {M}::ActionKind::Snow => LoadKind::Snow,
        {M}::ActionKind::Wind => LoadKind::Wind,
        {M}::ActionKind::Accidental => LoadKind::Accidental,
        {M}::ActionKind::Imposed => LoadKind::Variable,
    }}''')
sub("🎬️scene/🪵️members/🦀️.rs", "load_kind(&action.kind)", "load_kind(action.kind)")
sub("🎬️scene/🪵️members/🦀️.rs", "member.strength_class, member.b_m", "member.strength_class.id(), member.b_m", 2)
sub("🎬️scene/🪵️members/🦀️.rs", 'format!("{} {}", member.label_en, member.strength_class)', 'format!("{} {}", member.label_en, member.strength_class.id())')
sub("🎬️scene/🪵️members/🦀️.rs", 'format!("{} {}", member.label_de, member.strength_class)', 'format!("{} {}", member.label_de, member.strength_class.id())')
CON = "🎬️scene/🔩️connections/🦀️.rs"
sub(CON, "g.per_row, c.fastener_type, c.diameter_m", "g.per_row, facts::family_of(c.fastener_type), c.diameter_m", 2)
# sqlite document row
IO = "🚪️io/🪶️sqlite/📸️snapshot/"
sub(IO + "🦀️.rs", "Cell::Text(&self.uls_equation)", "Cell::Text(uls_equation_text(self.uls_equation))")
sub(IO + "🦀️.rs", '''let uls_equation=reconstruct_text(control,document.text(2)?)?;if uls_equation!="6.10"&&uls_equation!="6.10ab"{return Err(invalid("EN1995 unsupported ULS equation"));}''', "let uls_equation=read_uls_equation(document.text(2)?)?;")
sub(IO + "🦀️.rs", "fn strength_class_text(", f'''fn uls_equation_text(a:UlsEquation)->&'static str{{match a{{UlsEquation::Eq610=>"eq610",UlsEquation::Eq610ab=>"eq610ab"}}}}
fn read_uls_equation(a:&str)->Result<UlsEquation, ValueError>{{match a{{"eq610"=>Ok(UlsEquation::Eq610),"eq610ab"=>Ok(UlsEquation::Eq610ab),_=>Err(invalid("EN1995 unsupported ULS equation"))}}}}
fn strength_class_text(''')
sub(IO + "🦀️.rs", "ActionKind, LoadDurationClass, FastenerType, StrengthClass, ImposedCategory}", "ActionKind, LoadDurationClass, FastenerType, StrengthClass, ImposedCategory, UlsEquation}")
sub(IO + "🗄️.sql", "CHECK (uls_equation IN ('6.10','6.10ab'))", "CHECK (uls_equation IN ('eq610','eq610ab'))")
sub(IO + "🟦️.ts", 'function ulsEquation(value:string):string{if(value!=="6.10"&&value!=="6.10ab")throw Error("EN1995 unsupported ULS equation");return value}', 'function ulsEquation(value:string):UlsEquation{if(value!=="eq610"&&value!=="eq610ab")throw Error("EN1995 unsupported ULS equation");return value}')
print("edits", n)
