from pathlib import Path
p = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🏗️authoring\📏️running-example\🐍️.py")
s = p.read_text(encoding="utf-8")
a = s.index("def winter_day_label")
b = s.index("ENVELOPE = [")
new = '''def _tex_degrees(value: float) -> str:
    return rf"{value:g}".replace("-", "{-}") + r"\,^{\circ}\mathrm{C}"


def winter_day_label(language: str) -> str:
    """📅 TeX of the example winter day that every Δθ = 20 K of the course refers to."""
    day = {"de": "Beispiel-Wintertag", "en": "Example winter day"}[language]
    return rf"\text{{{day}}}\;\theta_{{e}} = {_tex_degrees(THETA_E_WINTER_C)}"


def design_day_label(language: str) -> str:
    """📅 TeX of the design day of the standard heating load (θ_e = −12 °C, Δθ = 32 K), named after its registry entry."""
    references = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📚️references.🐍️")
    day = {"de": "Auslegungstag", "en": "Design day"}[language]
    note = {"de": "Norm-Außentemperatur", "en": "standard outdoor temperature"}[language]
    return rf"\text{{{day}}}\;\theta_{{e}} = {_tex_degrees(THETA_E_DESIGN_C)}\;\text{{({note}, {references.ref(DESIGN_REFERENCE_ID, language=language)})}}"


'''
s = s[:a] + new + s[b:]
s = s.replace("import json\nimport numpy as np", "import importlib\nimport json\nimport numpy as np", 1)
p.write_text(s, encoding="utf-8")
