"""Stage 3: EN + DE field metadata (editor field-meta table and schema field-metadata.json) for WeldRun, SectionVertex and the `free` section kind. Idempotent."""
import json, sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from spec import *

FM = ART / "✏️editor/🏷️field-meta/🦀️.rs"
JS = SCHEMA / "🎲️field-metadata.json"
UNIT = {"f64": ("Some(UnitId::Metre)", "m", "length")}
CHOICE = {"process": "WELD_PROCESS", "direction": "WELD_DIRECTION"}


def main():
    t = FM.read_text(encoding="utf8")
    if "weldRuns[]" not in t:
        anchor = '    ("placements[].yaw", m("Placement yaw about Z", "Platzierung Gierwinkel um Z", Some(UnitId::Radian), None)),\n'
        assert anchor in t
        rows = ""
        for e in ENTS:
            for f in e.all_fields:
                unit = "Some(UnitId::Metre)" if f.ty == "f64" else "None"
                choices = f"Some({CHOICE[f.snake]})" if e is WELD_RUN and f.snake in CHOICE else "None"
                rows += f'    ("{e.camel}[].{f.camel}", m("{f.en}", "{f.de}", {unit}, {choices})),\n'
        t = t.replace(anchor, anchor + rows)
        kind = '    choice("chs", "Circular hollow section (CHS)", "Kreis-Hohlprofil (CHS)"),\n];'
        assert kind in t
        t = t.replace(kind, '    choice("chs", "Circular hollow section (CHS)", "Kreis-Hohlprofil (CHS)"),\n    choice("free", "Free outline (polygon with holes)", "Freier Umriss (Polygon mit Löchern)"),\n];', 1)
        FM.write_text(t, encoding="utf8")
    d = json.loads(JS.read_text(encoding="utf8"))
    if not any(f["path"].startswith("weldRuns[]") for f in d["fields"]):
        for e in ENTS:
            for f in e.fields:
                if f.ty == "f64":
                    d["fields"].append({"path": f"{e.camel}[].{f.camel}", "label": {"en": f.en, "de": f.de}, "unit": "m", "kind": "length"})
                elif f.ty == "str" and e is WELD_RUN and f.snake in CHOICE:
                    pass
        d["fields"].append({"path": "weldRuns[].process", "label": {"en": "Welding process", "de": "Schweißverfahren"}, "kind": "string", "choices": ["mig", "tig"]})
        d["fields"].append({"path": "weldRuns[].direction", "label": {"en": "Weld direction", "de": "Schweißrichtung"}, "kind": "string", "choices": ["longitudinal", "transverse"]})
        JS.write_text(json.dumps(d, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
    print("stage 3 done")


main()
