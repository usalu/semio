from spec import *
import glob, re
use_line = None
for p in glob.glob(SUB + "/📚️examples/*/🦀️.rs"):
    t = open(p, encoding="utf8").read()
    if "fatigue_spectra" in t: continue
    t = re.sub(r'(VariableAction \{ id: "[^"]+"\.into\(\), category: "[^"]+"\.into\(\), qk: [0-9_.]+) \}', r'\1, group: String::new() }', t)
    assert t.count("group: String::new()") >= 2, p
    spectra = "vec![]"
    if "fatigue-compliant" in p:
        spectra = 'vec![\n            FatigueSpectrum { id: "FS-1".into(), member_id: "beam-B1".into(), detail_category: 71.0e6, stress_range: 60.0e6, cycles: 2.0e5 },\n            FatigueSpectrum { id: "FS-2".into(), member_id: "beam-B1".into(), detail_category: 71.0e6, stress_range: 30.0e6, cycles: 1.0e6 },\n        ]'
    if "fatigue-failing" in p:
        spectra = 'vec![FatigueSpectrum { id: "FS-1".into(), member_id: "beam-B1".into(), detail_category: 71.0e6, stress_range: 80.0e6, cycles: 2.0e6 }]'
    t = re.sub(r"(        effects: vec!\[.*?\n        \],\n)", lambda m: m.group(1) + f"        fatigue_spectra: {spectra},\n        reliability_analyses: vec![],\n        test_series: vec![],\n", t, count=1, flags=re.S)
    assert "reliability_analyses" in t, p
    if "FatigueSpectrum" in t:
        t = re.sub(r"use semio_s_artifact_norm_en1990_model::\{([^}]*)\};", lambda m: "use semio_s_artifact_norm_en1990_model::{" + m.group(1).rstrip() + ", FatigueSpectrum};", t, count=1)
    open(p, "w", encoding="utf8").write(t)
    print(p[-60:], "FatigueSpectrum" in t)
# dsl rows
for name, rows in (("fatigue-compliant", ["  FS-1 beam-B1 71000000 60000000 200000", "  FS-2 beam-B1 71000000 30000000 1000000"]), ("fatigue-failing", ["  FS-1 beam-B1 71000000 80000000 2000000"])):
    p = SUB + f"/🖼️assets/🏢️{name}/🏢️{name}/🗣️.dsl.semio"; t = open(p, encoding="utf8").read()
    hdr = "fatigue-spectra [id:TEXT member-id:TEXT detail-category:NUM stress-range:NUM cycles:NUM] {\n}\n"
    assert hdr in t
    t = t.replace(hdr, hdr[:-3] + "\n".join(rows) + "\n}\n")
    open(p, "w", encoding="utf8").write(t)
