from spec import *
FM = SUB + "/✏️editor/🏷️field-meta"
p = FM + "/🦀️.rs"; t = open(p, encoding="utf8").read()
if "fatigueSpectra" in t: raise SystemExit("done")
t = t.replace("const VARIABLE_CATEGORIES:", 'const DISTRIBUTIONS: &[NormFieldChoice] = &[choice("normal", "Normal", "Normal"), choice("lognormal", "Lognormal", "Lognormal"), choice("gumbel", "Gumbel", "Gumbel"), choice("weibull", "Weibull", "Weibull")];\nconst TEST_DISTRIBUTIONS: &[NormFieldChoice] = &[choice("normal", "Normal (D.7.2 a)", "Normal (D.7.2 a)"), choice("lognormal", "Lognormal (D.7.2 b)", "Lognormal (D.7.2 b)")];\n\nconst VARIABLE_CATEGORIES:', 1)
t = t.replace('    ("variables[].qk", FieldSpec::quantity("Q_k", "Q_k", UnitId::Newton)),\n', '    ("variables[].qk", FieldSpec::quantity("Q_k", "Q_k", UnitId::Newton)),\n    ("variables[].group", FieldSpec::new("Exclusive group", "Ausschlussgruppe")),\n', 1)
new = '''    ("fatigueSpectra", FieldSpec::new("Fatigue spectrum bins", "Ermüdungskollektiv-Klassen")),
    ("fatigueSpectra[].id", FieldSpec::new("Spectrum bin id", "Kollektiv-Klassen-Id")),
    ("fatigueSpectra[].memberId", FieldSpec::new("Member id", "Bauteil-Id")),
    ("fatigueSpectra[].detailCategory", FieldSpec::quantity("Detail category Δσ_C", "Kerbfall Δσ_C", UnitId::Pascal)),
    ("fatigueSpectra[].stressRange", FieldSpec::quantity("Stress range Δσ", "Spannungsschwingbreite Δσ", UnitId::Pascal)),
    ("fatigueSpectra[].cycles", FieldSpec::quantity("Cycles n", "Lastwechsel n", UnitId::One)),
    ("reliabilityAnalyses", FieldSpec::new("Reliability analyses", "Zuverlässigkeitsanalysen")),
    ("reliabilityAnalyses[].id", FieldSpec::new("Reliability analysis id", "Zuverlässigkeitsanalyse-Id")),
    ("reliabilityAnalyses[].resistanceDistribution", FieldSpec::choice("Resistance distribution", "Verteilung des Widerstands", DISTRIBUTIONS)),
    ("reliabilityAnalyses[].resistanceMean", FieldSpec::quantity("Resistance mean μ_R", "Mittelwert Widerstand μ_R", UnitId::Newton)),
    ("reliabilityAnalyses[].resistanceSd", FieldSpec::quantity("Resistance standard deviation σ_R", "Standardabweichung Widerstand σ_R", UnitId::Newton)),
    ("reliabilityAnalyses[].actionDistribution", FieldSpec::choice("Action distribution", "Verteilung der Einwirkung", DISTRIBUTIONS)),
    ("reliabilityAnalyses[].actionMean", FieldSpec::quantity("Action mean μ_E", "Mittelwert Einwirkung μ_E", UnitId::Newton)),
    ("reliabilityAnalyses[].actionSd", FieldSpec::quantity("Action standard deviation σ_E", "Standardabweichung Einwirkung σ_E", UnitId::Newton)),
    ("testSeries", FieldSpec::new("Test series", "Versuchsreihen")),
    ("testSeries[].id", FieldSpec::new("Test series id", "Versuchsreihen-Id")),
    ("testSeries[].quantity", FieldSpec::new("Tested quantity", "Geprüfte Größe")),
    ("testSeries[].distribution", FieldSpec::choice("Distribution", "Verteilung", TEST_DISTRIBUTIONS)),
    ("testSeries[].sampleSize", FieldSpec::quantity("Sample size n", "Stichprobenumfang n", UnitId::One)),
    ("testSeries[].mean", FieldSpec::new("Sample mean m", "Stichprobenmittel m")),
    ("testSeries[].stdDev", FieldSpec::new("Sample standard deviation s", "Stichprobenstandardabweichung s")),
    ("testSeries[].variationKnown", FieldSpec::quantity("Known coefficient of variation V_X", "Bekannter Variationskoeffizient V_X", UnitId::One)),
    ("testSeries[].conversionFactor", FieldSpec::quantity("Conversion factor η_d", "Umrechnungsfaktor η_d", UnitId::One)),
    ("testSeries[].required", FieldSpec::new("Required design value", "Geforderter Bemessungswert")),
'''
t = t.replace('    ("effects[].influence", FieldSpec::new("Influence coefficient", "Einflussbeiwert")),\n', '    ("effects[].influence", FieldSpec::new("Influence coefficient", "Einflussbeiwert")),\n' + new, 1)
assert "testSeries[].required" in t and "variables[].group" in t
open(p, "w", encoding="utf8").write(t)
p = FM + "/🧪️tests/🔬️unit/🦀️.rs"; t = open(p, encoding="utf8").read()
paths = ['"variables[].group"'] + [f'"{e["coll_camel"]}"' for e in ENTITIES.values()] + [f'"{e["coll_camel"]}[].{camel(f["snake"])}"' for e in ENTITIES.values() for f in e["fields"]]
t = t.replace('        "effects[].influence",\n    ];\n    for path in wildcards', '        "effects[].influence",\n' + "".join(f"        {x},\n" for x in paths) + "    ];\n    for path in wildcards", 1)
assert "testSeries[].required" in t
open(p, "w", encoding="utf8").write(t)
print("ok")
