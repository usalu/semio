import sys
def patch(p, pairs):
    s = open(p, encoding='utf-8', newline='').read()
    for old, new in pairs:
        assert s.count(old) == 1, (p, old, s.count(old))
        s = s.replace(old, new)
    open(p + ".new", 'w', encoding='utf-8', newline='').write(s)
E = sys.argv[1]
patch(E + "/🧪️tests/🔬️unit/🦀️.rs", [
("for body in [outliner_panel::BODY_KEY, properties_panel::BODY_KEY, library_panel::BODY_KEY] {", "for body in [outliner_panel::BODY_KEY, properties_panel::BODY_KEY, library_panel::BODY_KEY, diagnostics_panel::BODY_KEY] {"),
])
patch(E + "/🧪️tests/⌨️completeness/🦀️.rs", [
('''        "bim.place.unsupported",
    ] {''', '''        "bim.place.unsupported",
        "bim.diagnostic.target-missing",
        "bim.classification.target-missing",
        "bim.classification.invalid",
        "bim.classification.missing",
    ] {'''),
('''(Locale::En, ["Plan", "3D", "Section", "Schedule", "Outliner", "Properties", "Library"]), (Locale::De, ["Grundriss", "3D", "Schnitt", "Mengen", "Struktur", "Eigenschaften", "Bibliothek"])] {''', '''(Locale::En, ["Plan", "3D", "Section", "Schedule", "Outliner", "Properties", "Library", "Diagnostics"]), (Locale::De, ["Grundriss", "3D", "Schnitt", "Mengen", "Struktur", "Eigenschaften", "Bibliothek", "Diagnose"])] {'''),
('''outliner_panel::BODY_KEY, properties_panel::BODY_KEY, library_panel::BODY_KEY].into_iter().zip(names) {''', '''outliner_panel::BODY_KEY, properties_panel::BODY_KEY, library_panel::BODY_KEY, diagnostics_panel::BODY_KEY].into_iter().zip(names) {'''),
])
