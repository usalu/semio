import re
import sys
from pathlib import Path

ENERGY = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy")
IMPORT = '_references = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📚️references.🐍️")\n'

CHIPS = {
    "VDI 2078:2015-06 · Kühllast (Solarstrahlung)": [("vdi-2078", "solar")],
    "DIN 4108-2:2013-02 · Abschn. 8, Tab. 7 (FC)": [("din-4108-2", "fc")],
    "DIN V 18599-2:2016-10 · Interne Wärmequellen": [("din-ts-18599-2", "intern")],
    "DIN 4108-2:2013-02 · Abschn. 8 Sommerlicher Wärmeschutz": [("din-4108-2", "abschn8")],
    "VDI 2078:2015-06 · Kühllast": [("vdi-2078", "kuehllast")],
    "VDI 6022 Blatt 1 · Hygiene (RLT-Anlagen)": [("vdi-6022-1", "hygiene")],
    "DIN EN 16798-1:2022-03 · Anh. B.2, Tab. B.2": [("din-en-16798-1", "kategorien")],
    "VDI 2078:2015-06 · innere Wärmequellen": [("vdi-2078", "innere")],
    "VDI 2078:2015-06 · innere Wärmequellen (Personen)": [("vdi-2078", "personen")],
    "VDI 2078:2015-06 · innere Wärmequellen (Geräte, Beleuchtung)": [("vdi-2078", "geraete")],
    "VDI 2078:2015-06 · Kühllast opake Bauteile": [("vdi-2078", "opak")],
    "VDI 2078:2015-06 · Kühllast Außenluft": [("vdi-2078", "aussenluft")],
    "VDI 2078:2015-06 · Kühllast (Zuluftstrom)": [("vdi-2078", "zuluft")],
    "DIN EN 16798-3": ["din-en-16798-3"],
    "DIN EN 308": ["din-en-308"],
    "DIN 1946-6": ["din-1946-6"],
    "VDI 2087": ["vdi-2087"],
    "Pettenkofer 1858 · Bundesgesundheitsbl. 51 (2008) 1358 · DIN 1946-6": ["pettenkofer-1858", "adhoc-ag-2008"],
    "DIN EN 12831-1": ["din-en-12831-1"],
    "DIN EN 12831-1 · VDI 2078": ["din-en-12831-1", "vdi-2078"],
    "VDI 2078": ["vdi-2078"],
    "DIN V 18599-1 · DIN EN 12831-1": ["din-ts-18599-1", "din-en-12831-1"],
    "DIN EN 12831-1 · DIN 1946-4": ["din-en-12831-1", "din-1946-4"],
    "DIN EN 12831-1 · DIN EN 16798-1": ["din-en-12831-1", "din-en-16798-1"],
    "DIN EN 16798-7": ["din-en-16798-7"],
    "DIN 4108-2": ["din-4108-2"],
    "DIN V 18599-5:2018-09, 6.2 Übergabe · 6.3 Verteilung · 6.4 Speicher": [("din-ts-18599-5", "uebergabe")],
    "DIN V 18599-5:2018-09, 6.5.3 Wärmepumpen · 6.5.4 Heizkessel · VDI 4650 Blatt 1:2024-02": [("din-ts-18599-5", "erzeuger"), ("vdi-4650-1", "jaz")],
    "DIN V 4701-10:2003-08, 4.2.6 Anlagen-Aufwandszahl · DIN V 18599-1:2018-09, 5.5 Endenergie": [("din-ts-18599-5", "aufwandszahl"), ("din-ts-18599-1", "endenergie")],
    "GEG, seit 7/2026 GModG: § 15 Abs. 1, § 20 Abs. 1": [("gmodg", "p15-20")],
    "DIN V 4108-6:2003-06, 5.2 · 6.5 · DIN V 18599-2:2018-09, 5.5.3": [("din-v-4108-6", "heizperiode"), ("din-ts-18599-2", "nutzenergie")],
    "DIN V 4108-6:2003-06, Anh. A · D.5 · DWD TRY 2017": [("din-v-4108-6", "klima"), "dwd-try-2017"],
    "DIN V 4108-6:2003-06, Anh. D.2 Heizperiodenbilanz · D.5": [("din-v-4108-6", "heizperiode")],
    "DIN V 4108-6:2003-06, 3.2 Symbole, Einheiten": [("din-v-4108-6", "symbole")],
    "DIN EN 12831-1:2017-09, Abschn. 6 · DIN/TS 12831-1:2020-04": [("din-en-12831-1", "abschn6"), "din-ts-12831-1"],
    "DIN EN 12831-1:2017-09, Abschn. 6 · DIN V 4108-6:2003-06, D.2": [("din-en-12831-1", "abschn6"), ("din-v-4108-6", "bilanz")],
    "DIN EN 12831-1:2017-09, Abschn. 6 · VDI 4645:2023-04": [("din-en-12831-1", "abschn6"), "vdi-4645"],
    "DIN V 18599-1:2018-09, 3.1 Begriffe · 5.5 Endenergie · 5.6 Primärenergie": [("din-ts-18599-1", "energiearten")],
    "GEG (seit 7/2026 GModG) § 20 Abs. 1, § 22 Abs. 1, Anlage 4 · DIN V 18599-1:2018-09, 5.6": [("gmodg", "p20-22"), ("din-ts-18599-1", "primaerenergie")],
    "GEG (seit 7/2026 GModG) § 15 Abs. 1, § 16, Anlage 1 · § 71 entfallen": [("gmodg", "p71")],
    "GModG 2026 (vormals GEG) · §§ 79–82": [("gmodg", "p79-82")],
    "GModG 2026 (vormals GEG) · § 86 Abs. 2 · Anlage 10": [("gmodg", "p86")],
    "GModG 2026 (vormals GEG) · § 84 · § 87 Abs. 1": [("gmodg", "p84-87")],
    "DIN V 4108-6:2003-06, 3.2 · 5.5": ["!din-v-4108-6"],
    "DIN V 4108-6:2003-06, 6.1 · 6.2": [("din-v-4108-6", "verluste")],
    "DIN V 4108-6:2003-06, 6.3 · 6.4 · 6.5": [("din-v-4108-6", "gewinne")],
    "DIN V 4108-6:2003-06, 6.5 · DIN V 18599-2:2018-09, 5.5.3": [("din-v-4108-6", "ausnutzung"), ("din-ts-18599-2", "nutzenergie")],
    "DIN V 18599-10:2018-09 · Abschn. 5, Tab. 4": [("din-ts-18599-10", "nutzungsprofile")],
    "Sunikka-Blank & Galvin 2012, Build. Res. Inf. 40(3)": ["sunikka-galvin-2012"],
    "GModG 2026 (vormals GEG) § 60c Abs. 1 · VOB/C DIN 18380:2019-09, Abschn. 3.5": [("gmodg", "p60c"), ("din-18380", "abgleich")],
    "DIN V 18599-1:2018-09, Abschn. 5.6 · GModG 2026 Anlage 4, 10": [("din-ts-18599-1", "primaerenergie"), ("gmodg", "p22")],
    "DIN V 4108-6, 6.4": [("din-v-4108-6", "solar")],
    "DIN V 4108-6, 6.4 · DIN EN 410": [("din-v-4108-6", "solar"), ("din-en-410", "g")],
    "DIN V 4108-6, Anh. A": [("din-v-4108-6", "solar")],
    "DIN V 4108-6, Anh. B.4": [("din-v-4108-6", "solar")],
    "DIN EN 12831-1:2017-09": ["din-en-12831-1"],
    "DIN V 4108-6, 6.3 · DIN V 18599-10": [("din-v-4108-6", "intern"), ("din-ts-18599-10", "intern")],
    "DIN 4108-2:2013-02": ["din-4108-2"],
    "DIN EN ISO 6946:2018-03": ["din-en-iso-6946"],
    "DIN EN ISO 13789:2017-10": [("din-en-iso-13789", "transmission")],
    "DIN 4108 Bbl. 2:2019-06 · DIN V 18599-2": ["din-4108-beiblatt-2", "din-ts-18599-2"],
    "DIN EN ISO 6946:2018-03, Tab. 7": [("din-en-iso-6946", "rse")],
}
SPECIAL = {
    ("❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py", "DIN V 18599-2:2016-10 · Interne Wärmequellen"):
        '_din_ref(_references.cite("!din-v-4108-6") + "\\n" + _references.cite(("din-ts-18599-2", "intern")))',
}


def expression(parts):
    items = ", ".join(repr(p).replace("'", '"') for p in parts)
    return f"_references.cite({items})"


def process(path: Path) -> int:
    lines = path.read_bytes().decode("utf-8").splitlines(keepends=True)
    relative = path.relative_to(ENERGY).as_posix()
    changed = 0
    for index, line in enumerate(lines):
        match = re.search(r'_din_ref\("([^"]*)"\)', line)
        if not match or match.group(1) not in CHIPS:
            if match:
                print("UNMAPPED", relative, index + 1, match.group(1))
            continue
        text = match.group(1)
        replacement = SPECIAL.get((relative, text)) or f"_din_ref({expression(CHIPS[text])})"
        lines[index] = line[:match.start()] + replacement + line[match.end():]
        changed += 1
    if changed:
        text = "".join(lines)
        if "_references = importlib" not in text:
            text = re.sub(r"^(tr = _manim_module\.tr\n)", lambda m: m.group(1) + IMPORT, text, count=1, flags=re.M)
        path.write_bytes(text.encode("utf-8"))
    return changed


total = 0
for chips_file in sys.argv[1:]:
    total += process(ENERGY / chips_file)
print("changed", total)
