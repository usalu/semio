#!/usr/bin/env python3
"""One-shot: hand edits of en1992 call sites that take the typed enums (functions with `&str` parameters, Display uses). Every replacement asserts its match count; reruns are no-ops."""
import pathlib
ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any")
M = "semio_s_artifact_norm_en1992_model"
n = 0

def sub(rel, old, new, count=1):
    global n
    p = ROOT / rel
    t = p.read_text(encoding="utf-8")
    if new in t and old not in t:
        return
    c = t.count(old)
    assert c == count, f"{rel}: expected {count} of {old[:80]!r}, found {c}"
    p.write_text(t.replace(old, new), encoding="utf-8")
    n += 1

REIN = "🎬️scene/🧱️reinforcement/🦀️.rs"
sub(REIN, "use super::model::RcMember;", "use super::model::{BarBend, BarPosition, RcMember};")
sub(REIN, 'pub fn is_top(position: &str) -> bool {\n    matches!(position, "top" | "compression")\n}',
    'pub fn is_top(position: BarPosition) -> bool {\n    matches!(position, BarPosition::Top | BarPosition::Compression)\n}\n\n/// 🏷️ The lower-case name of a layer position as the bar labels spell it.\npub fn position_name(position: BarPosition) -> &\'static str {\n    match position {\n        BarPosition::Bottom => "bottom",\n        BarPosition::Top => "top",\n        BarPosition::Tension => "tension",\n        BarPosition::Compression => "compression",\n    }\n}')
sub(REIN, "let top = is_top(&l.position);", "let top = is_top(l.position);")
sub(REIN, 'pub fn bend_mode(name: &str) -> BendMode {\n    match name {\n        "straight" => BendMode::Straight,\n        "hook" => BendMode::Hook,\n        "u-hook" => BendMode::UHook,\n        _ => BendMode::Auto,\n    }\n}',
    'pub fn bend_mode(bend: BarBend) -> BendMode {\n    match bend {\n        BarBend::Auto => BendMode::Auto,\n        BarBend::Straight => BendMode::Straight,\n        BarBend::Hook => BendMode::Hook,\n        BarBend::UHook => BendMode::UHook,\n    }\n}')
sub(REIN, "`auto` keeps the derived rule (a hook only where anchorage fails), `straight` none, `hook` one hook at the end bending into the section, `u-hook` both ends; unknown text is `auto`.", "`Auto` keeps the derived rule (a hook only where anchorage fails), `Straight` none, `Hook` one hook at the end bending into the section, `UHook` both ends.")
MEM = "🎬️scene/🏛️members/🦀️.rs"
sub(MEM, "hook_end, is_top, lap_spans, offset_inward, BendMode,", "hook_end, is_top, lap_spans, offset_inward, position_name, BendMode,")
sub(MEM, "is_top(&spec.position)", "is_top(spec.position)", 2)
sub(MEM, "bend_mode(&b.bend)", "bend_mode(b.bend)")
sub(MEM, "mm(spec.diameter), spec.position", "mm(spec.diameter), position_name(spec.position)", 6)
sub("🎬️scene/🔬️mechanics/🦀️.rs", "!is_top(&l.position)", "!is_top(l.position)")
sub("🎬️scene/🔬️mechanics/🦀️.rs", 'fn gamma(kind: &str) -> f64 {\n    if kind == "permanent" { 1.35 } else { 1.5 }', f'fn gamma(kind: {M}::ActionKind) -> f64 {{\n    if kind == {M}::ActionKind::Permanent {{ 1.35 }} else {{ 1.5 }}')
sub("🎬️scene/🔬️mechanics/🦀️.rs", "gamma(&a.kind)", "gamma(a.kind)", 2)
KIT = "🎬️scene/🧰️kit/🦀️.rs"
sub(KIT, 'pub(crate) fn load_kind(kind: &str) -> LoadKind {\n    match kind {\n        "permanent" => LoadKind::Permanent,\n        "imposed" | "variable" => LoadKind::Variable,\n        "wind" => LoadKind::Wind,\n        "snow" => LoadKind::Snow,\n        "accidental" => LoadKind::Accidental,\n        "seismic" => LoadKind::Seismic,\n        "thermal" => LoadKind::Thermal,\n        "prestress" => LoadKind::Prestress,\n        _ => LoadKind::Other,\n    }\n}',
    f'pub(crate) fn load_kind(kind: {M}::ActionKind) -> LoadKind {{\n    match kind {{\n        {M}::ActionKind::Permanent => LoadKind::Permanent,\n        {M}::ActionKind::Imposed => LoadKind::Variable,\n        {M}::ActionKind::Wind => LoadKind::Wind,\n        {M}::ActionKind::Snow => LoadKind::Snow,\n        {M}::ActionKind::Accidental => LoadKind::Accidental,\n        {M}::ActionKind::Prestress => LoadKind::Prestress,\n    }}\n}}')
sub("🎬️scene/⚓️anchors/🦀️.rs", "load_kind(&action.kind)", "load_kind(action.kind)", 2)
sub(MEM, "load_kind(&a.kind)", "load_kind(a.kind)", 2)
PUN = "🎬️scene/🕳️punching/🦀️.rs"
sub(PUN, 'match m.punching.as_ref().map_or("interior", |p| p.column_position.as_str()) {\n        "edge" => (l - cw / 2.0, 0.0),\n        "corner" => (l - cw / 2.0, w / 2.0 - cd / 2.0),',
    f'match m.punching.as_ref().map_or({M}::ColumnPosition::Interior, |p| p.column_position) {{\n        {M}::ColumnPosition::Edge => (l - cw / 2.0, 0.0),\n        {M}::ColumnPosition::Corner => (l - cw / 2.0, w / 2.0 - cd / 2.0),')
sub(PUN, "mm(p.column_depth), p.column_position)", "mm(p.column_depth), format!(\"{:?}\", p.column_position).to_lowercase())", 2)
CK = "🧬️schema/💡️inferences/⚖️checks/"
sub(CK + "⚓️anchor-evaluation/🦀️.rs", f"            {M}::ActionSource::External => {{}}\n            _ => {{\n                n += a.point_force.abs() + a.g_k_line.abs() + a.q_k_line.abs();\n            }}\n", f"            {M}::ActionSource::External => {{}}\n")
sub(CK + "⚓️anchor-evaluation/🦀️.rs", f"            {M}::ActionKind::Accidental => accidentals.push((a, eff)),\n            _ => variables.push((a, eff)),", f"            {M}::ActionKind::Accidental => accidentals.push((a, eff)),\n            {M}::ActionKind::Imposed | {M}::ActionKind::Snow | {M}::ActionKind::Wind => variables.push((a, eff)),")
HLP = CK + "🧰️helpers-and-annex-parts/🦀️.rs"
sub(HLP, 'pub fn punching_beta(position: &str, annex: AnnexChoice) -> f64 {\n        ndp(annex, match position { "edge" => "ndp.en1992.beta-edge", "corner" => "ndp.en1992.beta-corner", "wall_end" => "ndp.en1992.beta-wall-end", _ => "ndp.en1992.beta-interior" })',
    f'pub fn punching_beta(position: {M}::ColumnPosition, annex: AnnexChoice) -> f64 {{\n        ndp(annex, match position {{ {M}::ColumnPosition::Edge => "ndp.en1992.beta-edge", {M}::ColumnPosition::Corner => "ndp.en1992.beta-corner", {M}::ColumnPosition::WallEnd => "ndp.en1992.beta-wall-end", {M}::ColumnPosition::Interior => "ndp.en1992.beta-interior" }})')
sub(HLP, "pub fn required_for(kind: MemberKind, support: SupportCondition, rating: FireRating, _column_method: &str, slab_system: &str) -> (f64, f64) {", f"pub fn required_for(kind: MemberKind, support: SupportCondition, rating: FireRating, slab_system: {M}::SlabFireSystem) -> (f64, f64) {{")
sub(HLP, 'if slab_system.eq_ignore_ascii_case("two-way") {', f"if slab_system == {M}::SlabFireSystem::TwoWay {{")
CP = CK + "📋️member-evaluation/🕳️control-perimeter/🦀️.rs"
sub(CP, 'pub(super) fn control_perimeter(position: &str,', f'pub(super) fn control_perimeter(position: {M}::ColumnPosition,')
sub(CP, '        "edge" => (span - c1 / 2.0, 0.0),\n        "corner" => (span - c1 / 2.0, span / 2.0 - c2 / 2.0),\n        _ => (span / 2.0, 0.0),', f'        {M}::ColumnPosition::Edge => (span - c1 / 2.0, 0.0),\n        {M}::ColumnPosition::Corner => (span - c1 / 2.0, span / 2.0 - c2 / 2.0),\n        _ => (span / 2.0, 0.0),')
sub(CP, '        "edge" => clip(&ring, |q| span - q.0),\n        "corner" => clip(&clip(&ring, |q| span - q.0), |q| span / 2.0 - q.1),', f'        {M}::ColumnPosition::Edge => clip(&ring, |q| span - q.0),\n        {M}::ColumnPosition::Corner => clip(&clip(&ring, |q| span - q.0), |q| span / 2.0 - q.1),')
sub(CP, 'pub(super) fn local_factor(position: &str, beta: f64, lever: f64) -> f64 {\n    if position == "edge" || position == "corner" {', f'pub(super) fn local_factor(position: {M}::ColumnPosition, beta: f64, lever: f64) -> f64 {{\n    if matches!(position, {M}::ColumnPosition::Edge | {M}::ColumnPosition::Corner) {{')
RES = CK + "📋️member-evaluation/🏗️resistance-and-serviceability-checks/🦀️.rs"
sub(RES, "punching_beta(&p.column_position, annex)", "punching_beta(p.column_position, annex)")
sub(RES, "control_perimeter(&p.column_position,", "control_perimeter(p.column_position,")
sub(RES, "local_factor(&p.column_position,", "local_factor(p.column_position,")
sub(CK + "📋️member-evaluation/🧷️entity-cases/🦀️.rs", '.with_s("position", &spec.column_position)', f'.with_s("position", match spec.column_position {{ {M}::ColumnPosition::Interior => "interior", {M}::ColumnPosition::Edge => "edge", {M}::ColumnPosition::Corner => "corner", {M}::ColumnPosition::WallEnd => "wall_end" }})')
FIRE = CK + "📋️member-evaluation/🔥️fire-anchorage-bridge-and-liquid-checks/🦀️.rs"
sub(FIRE, "fire.rating, &fire.column_method, &fire.slab_system);", "fire.rating, fire.slab_system);")
sub(FIRE, "; method {}; slab {})\"", "; slab {:?})\"")
sub(FIRE, "; Verfahren {}; Platte {})\"", "; Platte {:?})\"")
sub(FIRE, "fire.rating, fire.column_method, fire.slab_system)", "fire.rating, fire.slab_system)", 2)
sub(CK + "📋️member-evaluation/📐️cover-and-constitutive-checks/🦀️.rs", "cement={}, ", "cement={:?}, ")
sub(CK + "📋️member-evaluation/📐️cover-and-constitutive-checks/🦀️.rs", "Zement={}, ", "Zement={:?}, ")
sub("🎬️scene/🧪️tests/🪝️detailing/🦀️.rs", 'assert_eq!([bend_mode("auto"), bend_mode("straight"), bend_mode("hook"), bend_mode("u-hook"), bend_mode("other")], [BendMode::Auto, BendMode::Straight, BendMode::Hook, BendMode::UHook, BendMode::Auto]);',
    'assert_eq!([bend_mode(model::BarBend::Auto), bend_mode(model::BarBend::Straight), bend_mode(model::BarBend::Hook), bend_mode(model::BarBend::UHook)], [BendMode::Auto, BendMode::Straight, BendMode::Hook, BendMode::UHook]);')
print("edits", n)
