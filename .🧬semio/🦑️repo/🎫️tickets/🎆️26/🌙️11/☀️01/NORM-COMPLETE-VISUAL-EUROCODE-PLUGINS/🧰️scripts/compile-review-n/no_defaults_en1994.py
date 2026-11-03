import os, re, sys
APPLY = "--apply" in sys.argv
ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994"
ANY = ROOT + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
MODEL = ANY + "/🧬️schema/📸️snapshot/🦀️.rs"
ROWS = "SteelSection|ProfiledSheeting|HeadedStuds|CharacteristicAction|SlabAction|ColumnAction|CompositeBeam|CompositeColumn|CompositeSlab|Placement"
DE = "semio_s_plugin_norm_compliance::AnnexChoice::De"
changed = []

def write(path, text, old):
    if text != old:
        changed.append(path)
        if APPLY:
            open(path, "w", encoding="utf8").write(text)

s = open(MODEL, encoding="utf8").read(); o = s
s = s.replace("impl Default for En1994Snapshot {\n    fn default() -> Self {", "impl En1994Snapshot {\n    /// 🌱 The showcase building document of the given annex; the annex is a normative choice and never defaulted.\n    pub fn example(annex: AnnexChoice) -> Self {", 1)
s = s.replace("            annex: AnnexChoice::De,\n            reliability_class: ReliabilityClass::RC2,", "            annex,\n            reliability_class: ReliabilityClass::RC2,", 1)
s = s.replace("let mut s = Self::default();", f"let mut s = Self::example(AnnexChoice::De);")
for name in ROWS.split("|"):
    s = re.sub(r"#\[derive\(Clone, Debug, Default, (PartialEq,[^\n]*)\)\]\n((?:#\[[^\n]*\]\n)*pub struct " + name + r" )", r"#[derive(Clone, Debug, \1)]\n\2", s)
write(MODEL, s, o)

repl = [
    (r"model::CharacteristicAction \{ id: \"P-1\"\.into\(\), kind: \"imposed\"\.into\(\), category: \"B\"\.into\(\), stage: \"composite\"\.into\(\), f_k_n: 50\.0e3, x_m: 2\.0, \.\.Default::default\(\) \}",
     'model::CharacteristicAction { x_m: 2.0, ..model::CharacteristicAction::point_force("P-1", "composite", "imposed", "B", 50.0e3) }'),
    (r"model::CharacteristicAction \{ x_m: 1\.0e3, \.\.Default::default\(\) \}", 'model::CharacteristicAction { x_m: 1.0e3, ..model::CharacteristicAction::point_force("", "", "", "", 0.0) }'),
    (r"model::CharacteristicAction::default\(\)", 'model::CharacteristicAction::point_force("", "", "", "", 0.0)'),
    (r"CharacteristicAction \{ id: \"P\"\.into\(\), kind: kind\.into\(\), category: \"B\"\.into\(\), stage: \"composite\"\.into\(\), f_k_n: 100e3, x_m, \.\.Default::default\(\) \}",
     'CharacteristicAction { x_m, ..CharacteristicAction::point_force("P", "composite", kind, "B", 100e3) }'),
    (r"semio_s_artifact_norm_en1994_model::CharacteristicAction \{ id: \"P\"\.into\(\), kind: \"imposed\"\.into\(\), category: \"B\"\.into\(\), stage: \"composite\"\.into\(\), f_k_n: 100e3, x_m: 2\.0, \.\.Default::default\(\) \}",
     'semio_s_artifact_norm_en1994_model::CharacteristicAction { x_m: 2.0, ..semio_s_artifact_norm_en1994_model::CharacteristicAction::point_force("P", "composite", "imposed", "B", 100e3) }'),
    (r"semio_s_artifact_norm_en1994_model::CharacteristicAction \{ id: \"P1\"\.into\(\), kind: \"imposed\"\.into\(\), category: \"B\"\.into\(\), stage: \"composite\"\.into\(\), f_k_n: 50e3, \.\.Default::default\(\) \}",
     'semio_s_artifact_norm_en1994_model::CharacteristicAction::point_force("P1", "composite", "imposed", "B", 50e3)'),
    (r"CharacteristicAction \{ kind: kind\.into\(\), stage: \"composite\"\.into\(\), q_area_pa: 1\.0, \.\.Default::default\(\) \}",
     'CharacteristicAction { q_area_pa: 1.0, ..CharacteristicAction::point_force("", "composite", kind, "", 0.0) }'),
]
call = re.compile(r"(?<![\w])((?:\w+::)*En1994Snapshot)::default\(\)")
for d, _, fs in os.walk(ROOT):
    for f in fs:
        if not f.endswith(".rs"): continue
        p = os.path.join(d, f)
        if p == MODEL: continue
        t = open(p, encoding="utf8").read(); o = t
        for a, b in repl: t = re.sub(a, b, t)
        t = t.replace("#[derive(Clone, Debug, Default)]\n    pub struct En1994BuilderConstruction", "#[derive(Clone, Debug)]\n    pub struct En1994BuilderConstruction")
        t = t.replace('variant = "en1994", scene =', 'variant = "en1994", seed = semio_s_artifact_norm_en1994_model::En1994Snapshot::example(' + DE + '), scene =')
        t = call.sub(lambda m: f"{m.group(1)}::example({DE})", t)
        write(p, t, o)
print(("applied " if APPLY else "dry-run ") + str(len(changed)) + " files")
for c in changed: print(" ", c[len(ROOT):])
