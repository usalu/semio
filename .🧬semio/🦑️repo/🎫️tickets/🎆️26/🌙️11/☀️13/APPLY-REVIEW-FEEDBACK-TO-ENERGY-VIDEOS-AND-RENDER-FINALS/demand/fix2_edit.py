import sys
from pathlib import Path

BASE = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\📊️demand\🎬️clip")


def patch(name, pairs):
    p = BASE / name / "🐍️.py"
    s = p.read_text(encoding="utf-8")
    for old, new in pairs:
        if s.count(old) != 1:
            sys.exit(f"MISMATCH {s.count(old)}: {old[:70]!r}")
        s = s.replace(old, new)
    p.write_text(s, encoding="utf-8")


patch("⚖️regulations", [
    ('tr("GEG (since 07/2026 GModG) — Building Energy Act", "GEG (seit 07/2026 GModG) — Gebäudeenergiegesetz")',
     'tr("GEG, since 07/2026 GModG — Building Energy Act", "GEG, seit 07/2026 GModG — Gebäudeenergiegesetz")'),
    ('_din_ref("GEG (seit 7/2026 GModG) § 15 Abs. 1, § 20 Abs. 1")', '_din_ref("GEG, seit 7/2026 GModG: § 15 Abs. 1, § 20 Abs. 1")'),
    ('_din_ref("DIN V 4108-6:2003-06, Anh. D.2 (Heizperiodenbilanz) · D.5")', '_din_ref("DIN V 4108-6:2003-06, Anh. D.2 Heizperiodenbilanz · D.5")'),
])
patch("⚡️demand-power", [
    ('_din_ref("DIN V 4108-6:2003-06, 3.2 (Symbole, Einheiten)")', '_din_ref("DIN V 4108-6:2003-06, 3.2 Symbole, Einheiten")'),
    ('for v in values or range(2, int(y_max_kw) + 1, 2):', 'for v in values or range(2, int(y_max_kw - 0.5) + 1, 2):'),
    ('''math_label(tr(r"\\dot{Q}_{H}\\ \\text{— in the standard } \\Phi_{\\mathrm{HL}}", r"\\dot{Q}_{H}\\ \\text{— in der Norm } \\Phi_{\\mathrm{HL}}")''',
     '''math_label(tr(r"\\dot{Q}_{H}\\;\\text{— in the standard}\\;\\Phi_{\\mathrm{HL}}", r"\\dot{Q}_{H}\\;\\text{— in der Norm}\\;\\Phi_{\\mathrm{HL}}")'''),
    ('''            ("q", r"Q_{h}", ENERGY), (None, r"\\to", P_WHITE), (None, tr(r"\\text{Final energy}", r"\\text{Endenergie}"), P_WHITE),
            (None, r"\\to", P_WHITE), (None, tr(r"\\text{Costs}", r"\\text{Kosten}"), P_WHITE),''',
     '''            ("q", r"Q_{h}", ENERGY), (None, tr(r"\\text{useful energy}", r"\\text{Nutzenergie}"), P_WHITE), (None, r"\\to", P_WHITE),
            (None, r"Q_{E}", P_ORANGE), (None, tr(r"\\text{final energy}", r"\\text{Endenergie}"), P_WHITE),
            (None, r"\\to", P_WHITE), (None, tr(r"\\text{costs}", r"\\text{Kosten}"), P_WHITE),'''),
])
print("ok")
