#!/usr/bin/env python3
"""One-shot: the hand-written remainder of the en1993 StiffenerShape / WarpingSupport conversion (diff twins, graphql/proto enums, leaf payloads, semantic Rust sites, grips). Every replacement asserts its match count. Usage: python3 -I en1993_manual.py [--dry]"""
import pathlib, re, sys

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any")
S = ROOT / "🧬️schema"
CK = S / "💡️inferences/⚖️checks"
DRY = "--dry" in sys.argv
cache = {}
M = "semio_s_artifact_norm_en1993_model"


def text(p):
    if p not in cache: cache[p] = p.read_text(encoding="utf-8")
    return cache[p]


def sub(p, old, new, count=1):
    t = text(p)
    n = t.count(old)
    assert n == count, f"{p.relative_to(ROOT)}: expected {count} of {old[:80]!r}, found {n}"
    cache[p] = t.replace(old, new)


def resub(p, pattern, new, count=1):
    t = text(p)
    n = len(re.findall(pattern, t))
    assert n == count, f"{p.relative_to(ROOT)}: expected {count} of {pattern[:80]!r}, found {n}"
    cache[p] = re.sub(pattern, new, t)


snap = S / "📸️snapshot/🦀️.rs"
resub(snap, r'/// 🔢️ The `stiffenerShape` values in handle order[^\n]*\npub const STIFFENER_SHAPES: \[&str; 4\] = [^\n]*\n\n/// 🔢️ The `warpingSupport` values in handle order[^\n]*\npub const WARPING_SUPPORTS: \[&str; 4\] = [^\n]*\n',
      '''impl StiffenerShape {
    /// 🔢️ Every outline in handle order: a numeric grip write is an index into this list that the editor resolves to the enum token.
    pub const ALL: [Self; 4] = [Self::None, Self::Flat, Self::Tee, Self::Angle];
}

impl WarpingSupport {
    /// 🔢️ Every end condition in handle order: a numeric grip write is an index into this list that the editor resolves to the enum token.
    pub const ALL: [Self; 4] = [Self::Fork, Self::Fixed, Self::FixedFork, Self::Cantilever];
}
''')

dr = S / "🔺️diff/🦀️.rs"
sub(dr, "warping_support: String", f"warping_support: {M}::WarpingSupport")
sub(dr, "stiffener_shape: String", f"stiffener_shape: {M}::StiffenerShape")

dt = S / "🔺️diff/🟦️.ts"
sub(dt, "warpingSupport: (string) | null", "warpingSupport: (WarpingSupport) | null")
sub(dt, "stiffenerShape: (string) | null", "stiffenerShape: (StiffenerShape) | null")
sub(dt, "warpingSupport: normWireDefault(normWireNullable(normWireString), () => null)", "warpingSupport: normWireDefault(normWireNullable(normWireRef(() => parseWarpingSupport)), () => null)")
sub(dt, "stiffenerShape: normWireDefault(normWireNullable(normWireString), () => null)", "stiffenerShape: normWireDefault(normWireNullable(normWireRef(() => parseStiffenerShape)), () => null)")
resub(dt, r'import \{ type BridgeFatigue,', "import { type StiffenerShape, type WarpingSupport, type BridgeFatigue,")
resub(dt, r'parseTowerLeg \} from "\.\./📸️snapshot/🟦️\.ts";', 'parseTowerLeg, parseStiffenerShape, parseWarpingSupport } from "../📸️snapshot/🟦️.ts";')

dj = S / "🔺️diff/🔣️.json"
SNAP = "https://json.schemas.assets.semio-tech.com/s/norm/en1993/snapshot.json"
sub(dj, '"warpingSupport":{"anyOf":[{"type":"string"},{"type":"null"}', f'"warpingSupport":{{"anyOf":[{{"$ref":"{SNAP}#/$defs/WarpingSupport"}},{{"type":"null"}}')
sub(dj, '"stiffenerShape":{"anyOf":[{"type":"string"},{"type":"null"}', f'"stiffenerShape":{{"anyOf":[{{"$ref":"{SNAP}#/$defs/StiffenerShape"}},{{"type":"null"}}')

for gq in (S / "🔗️.graphql", S / "📸️snapshot/🔗️.graphql"):
    sub(gq, "warpingSupport: String!", "warpingSupport: WarpingSupport!")
    sub(gq, "stiffenerShape: String!", "stiffenerShape: StiffenerShape!")
    resub(gq, r"(?m)^(# [^\n]*\n)", r"\1enum WarpingSupport { Fork Fixed FixedFork Cantilever }\nenum StiffenerShape { None Flat Tee Angle }\n")

PROTO_ENUMS = "enum WarpingSupport { WARPING_SUPPORT_FORK = 0; WARPING_SUPPORT_FIXED = 1; WARPING_SUPPORT_FIXED_FORK = 2; WARPING_SUPPORT_CANTILEVER = 3; }\nenum StiffenerShape { STIFFENER_SHAPE_NONE = 0; STIFFENER_SHAPE_FLAT = 1; STIFFENER_SHAPE_TEE = 2; STIFFENER_SHAPE_ANGLE = 3; }\n"
for pb in (S / "🛰️.proto", S / "📸️snapshot/🛰️.proto"):
    resub(pb, r"string warping_support = (\d+);", r"WarpingSupport warping_support = \1;")
    resub(pb, r"string stiffener_shape = (\d+);", r"StiffenerShape stiffener_shape = \1;")
    t = text(pb)
    i = t.index("\nmessage ") + 1
    cache[pb] = t[:i] + PROTO_ENUMS + t[i:]

MUT = S / "🧬️mutations"
for leaf, enum, field in (("🧱️change-plated-stiffener-shape", "StiffenerShape", "stiffener_shape"), ("📏️change-member-warping-support", "WarpingSupport", "warping_support")):
    L = MUT / leaf
    lr = L / "🦀️.rs"
    sub(lr, f"pub new_{field}: String,", f"pub new_{field}: {M}::{enum},")
    sub(L / "🔺️diff/🦀️.rs", f"Some(payload.new_{field}.clone())", f"Some(payload.new_{field})")
    sub(L / "↩️inverse/🦀️.rs", f"new_{field}: row.{field}.clone()", f"new_{field}: row.{field}")
    camel = "".join(w.capitalize() for w in field.split("_"))
    camel = camel[0].lower() + camel[1:]
    ts = L / "🧬️schema/🟦️.ts"
    sub(ts, f"  new{camel[0].upper() + camel[1:]}: string;", f"  new{camel[0].upper() + camel[1:]}: {enum};")
    sub(ts, f"new{camel[0].upper() + camel[1:]}: normWireRequired(normWireString)", f"new{camel[0].upper() + camel[1:]}: normWireRequired(parse{enum})")
    sub(ts, 'import { normWireObject,', f'import {{ type {enum}, parse{enum} }} from "../../../📸️snapshot/🟦️.ts";\nimport {{ normWireObject,')
    js = L / "🧬️schema/🔣️.json"
    resub(js, r'("new%s": \{\n\s+)"type": "string",' % (camel[0].upper() + camel[1:]), r'\1"$ref": "%s#/$defs/%s",' % (SNAP, enum))
    resub(js, r'\n\s+"widget": "text",(\n\s+"role": "value")', r"\1")
    rs = L / "🦀️.rs"
    if not re.search(r"use %s::" % M, text(rs)):
        pass

ER = MUT / "🧭️edit-rules/🦀️.rs"
sub(ER, "use semio_s_artifact_norm_en1993_model::{rolled_heb_catalogue, En1993Snapshot, STIFFENER_SHAPES, WARPING_SUPPORTS};", "use semio_s_artifact_norm_en1993_model::{rolled_heb_catalogue, En1993Snapshot, StiffenerShape, WarpingSupport};")
sub(ER, "a numeric write to `platedPanels[…].stiffenerShape` or `members[…].warpingSupport` is an index into the choice list.", "a numeric write to `platedPanels[…].stiffenerShape` or `members[…].warpingSupport` is an index into the variants of the enum ([`StiffenerShape::ALL`], [`WarpingSupport::ALL`]) and is raised as the enum token.")
sub(ER, "let choices: Option<&[&str]> = if", "let choices: Option<Vec<String>> = if")
sub(ER, "Some(&STIFFENER_SHAPES)", 'Some(StiffenerShape::ALL.iter().map(|v| format!("{v:?}")).collect())')
sub(ER, "Some(&WARPING_SUPPORTS)", 'Some(WarpingSupport::ALL.iter().map(|v| format!("{v:?}")).collect())')
sub(ER, "let value = choices[(index.as_f64().round().max(0.0) as usize).min(choices.len() - 1)];", "let value = &choices[(index.as_f64().round().max(0.0) as usize).min(choices.len() - 1)];")
sub(ER, "DslValue::String(value.to_string()) });\n        }\n    }\n    EDIT_RULES", "DslValue::String(value.clone()) });\n        }\n    }\n    EDIT_RULES")

H = ROOT / "🎬️scene-members/🕹️handles/🦀️.rs"
sub(H, "is an index into [`model::STIFFENER_SHAPES`] or [`model::WARPING_SUPPORTS`] that the editor resolves to the typed string value.", "is an index into [`model::StiffenerShape::ALL`] or [`model::WarpingSupport::ALL`] that the editor resolves to the enum token.")
sub(H, "model::WARPING_SUPPORTS.len()", "model::WarpingSupport::ALL.len()")
sub(H, "model::STIFFENER_SHAPES.len()", "model::StiffenerShape::ALL.len()")
sub(H, "!p.stiffener_shape.is_empty() && p.stiffener_depth > 0.0", f"p.stiffener_shape != {M}::StiffenerShape::None && p.stiffener_depth > 0.0")

PS = ROOT / "🎬️scene-members/🧱️plates-and-shells/🦀️.rs"
sub(PS, '''    if !p.stiffener_shape.is_empty() {
        en += &format!(", transverse stiffener {}", p.stiffener_shape);
        de += &format!(", Quersteife {}", p.stiffener_shape);
    }''', f'''    let (shape_en, shape_de) = match p.stiffener_shape {{
        {M}::StiffenerShape::None => ("", ""),
        {M}::StiffenerShape::Flat => ("flat bar", "Flachstahl"),
        {M}::StiffenerShape::Tee => ("tee", "T-Profil"),
        {M}::StiffenerShape::Angle => ("angle", "Winkel"),
    }};
    if !shape_en.is_empty() {{
        en += &format!(", transverse stiffener {{shape_en}}");
        de += &format!(", Quersteife {{shape_de}}");
    }}''')

PD = ROOT / "🎬️scene-members/🔬️plate-details/🦀️.rs"
sub(PD, f'''            {M}::StiffenerShape::Tee => ("tee", "T-Profil"),
            {M}::StiffenerShape::Angle => ("angle", "Winkel"),
            _ => ("flat bar", "Flachstahl"),''', f'''            {M}::StiffenerShape::Tee => ("tee", "T-Profil"),
            {M}::StiffenerShape::Angle => ("angle", "Winkel"),
            {M}::StiffenerShape::Flat | {M}::StiffenerShape::None => ("flat bar", "Flachstahl"),''')

TV = ROOT / "🎬️scene-members/🌀️torsion-views/🦀️.rs"
sub(TV, "ends_of(&member.warping_support)", "ends_of(member.warping_support)")
sub(TV, 'format!("B_max = {:.2} kNm² ({} supports)", peak / 1e3, member.warping_support)', 'format!("B_max = {:.2} kNm² ({:?} supports)", peak / 1e3, member.warping_support)')
sub(TV, 'format!("B_max = {:.2} kNm² (Lagerung {})", peak / 1e3, member.warping_support)', 'format!("B_max = {:.2} kNm² (Lagerung {:?})", peak / 1e3, member.warping_support)')

WT = CK / "🧮️clauses/🌀️warping-torsion/🦀️.rs"
resub(WT, r'/// 🌀 End conditions of a `warpingSupport` value[^\n]*\npub fn ends_of\(support: &str\) -> \(End, End\) \{\n    match support \{\n[^}]*\n    \}\n\}\n\n', "")
CF = CK / "🧮️clauses/🦀️.rs"
sub(CF, "\n#[cfg(test)]\n#[path = \"🧪️tests/🔬️unit/🦀️.rs\"]", f'''
/// 🌀 End conditions of a `warpingSupport` value: fork, both ends restrained, restrained at the start with a fork end, or cantilever (start restrained, free end).
pub fn ends_of(support: {M}::WarpingSupport) -> (warping_torsion::End, warping_torsion::End) {{
    use warping_torsion::End;
    match support {{
        {M}::WarpingSupport::Fixed => (End::Fixed, End::Fixed),
        {M}::WarpingSupport::FixedFork => (End::Fixed, End::Fork),
        {M}::WarpingSupport::Cantilever => (End::Fixed, End::Free),
        {M}::WarpingSupport::Fork => (End::Fork, End::Fork),
    }}
}}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]''')
sub(ROOT / "🎬️scene-members/🌀️torsion-views/🦀️.rs", "use checks::clauses::warping_torsion::{ends_of, warping_torsion, TorsionProfile};", "use checks::clauses::ends_of;\nuse checks::clauses::warping_torsion::{warping_torsion, TorsionProfile};")

AR = CK / "🧱️member-checks/🧮️action-resistance-checks/🦀️.rs"
sub(AR, '''fn warping_support_en(support: &str) -> &'static str {
    match support {
        "fixed" => "warping-restrained",
        "fixed-fork" => "one warping-restrained and one fork",
        "cantilever" => "cantilever (restrained and free)",
        _ => "fork",
    }''', f'''fn warping_support_en(support: {M}::WarpingSupport) -> &'static str {{
    match support {{
        {M}::WarpingSupport::Fixed => "warping-restrained",
        {M}::WarpingSupport::FixedFork => "one warping-restrained and one fork",
        {M}::WarpingSupport::Cantilever => "cantilever (restrained and free)",
        {M}::WarpingSupport::Fork => "fork",
    }}''')
sub(AR, '''fn warping_support_de(support: &str) -> &'static str {
    match support {
        "fixed" => "beidseitig wölbbehinderten Enden",
        "fixed-fork" => "einem wölbbehinderten und einem Gabelende",
        "cantilever" => "Kragarmlagerung (eingespannt und frei)",
        _ => "Gabellagerung",
    }''', f'''fn warping_support_de(support: {M}::WarpingSupport) -> &'static str {{
    match support {{
        {M}::WarpingSupport::Fixed => "beidseitig wölbbehinderten Enden",
        {M}::WarpingSupport::FixedFork => "einem wölbbehinderten und einem Gabelende",
        {M}::WarpingSupport::Cantilever => "Kragarmlagerung (eingespannt und frei)",
        {M}::WarpingSupport::Fork => "Gabellagerung",
    }}''')
sub(AR, "crate::clauses::warping_torsion::ends_of(&member.warping_support)", "crate::clauses::ends_of(member.warping_support)")
resub(AR, r"warping_support_(en|de)\(&member\.warping_support\)", r"warping_support_\1(member.warping_support)", 2)

SP = CK / "🔗️special-element-checks/🔭️special-parts-checks/🦀️.rs"
sub(SP, "k.iw * 1e18, p.stiffener_shape),", "k.iw * 1e18, format!(\"{:?}\", p.stiffener_shape)),", 2)

if not DRY:
    for p, t in cache.items(): p.write_text(t, encoding="utf-8")
print(("dry " if DRY else "") + f"ok: {len(cache)} files")
