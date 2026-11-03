"""en1992 `punching.betaMethod|momentC1|momentC2` (EN 1992-1-1 6.4.3): enum def `PunchingBetaMethod` and three properties of `PunchingSpec` in the snapshot and artifact JSON (text surgery, layout preserved, idempotent); twins and the PunchingSpec struct then come from `schema twins generate`."""
import re
ANY = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/"
DEF = '''  "PunchingBetaMethod": {
   "title": "PunchingBetaMethod",
   "type": "string",
   "enum": ["Simplified", "UnbalancedMoments"],
   "x-semio-ui": {"options": {"Simplified": {"en": "Simplified (fixed β of 6.4.3(6))", "de": "Vereinfacht (festes β nach 6.4.3(6))"}, "UnbalancedMoments": {"en": "From unbalanced moments (6.4.3(3)-(5))", "de": "Aus Stützenmomenten (6.4.3(3)-(5))"}}}
  },
'''
PROPS = """
    "betaMethod": {"$ref": "#/$defs/PunchingBetaMethod", "x-semio-ui": {"label": {"en": "Load increase factor β", "de": "Lasterhöhungsfaktor β"}, "description": {"en": "EN 1992-1-1 6.4.3: the fixed values of 6.4.3(6) (Figure 6.21N) or β from the unbalanced column moments.", "de": "EN 1992-1-1 6.4.3: die festen Werte nach 6.4.3(6) (Bild 6.21N) oder β aus den Stützenmomenten."}}},
    "momentC1": {"type": "number", "x-semio-ui": {"label": {"en": "Unbalanced column moment M_Ed,1 (eccentricity along c₁)", "de": "Stützenmoment M_Ed,1 (Ausmitte entlang c₁)"}, "description": {"en": "Design value; edge and corner columns: positive when the load shifts towards the slab interior. Used with the method from unbalanced moments.", "de": "Bemessungswert; Rand- und Eckstützen: positiv, wenn sich die Last zum Deckeninneren verschiebt. Wirksam bei der Methode aus Stützenmomenten."}, "widget": "stepper", "unit": "N·m", "displayUnit": "kN·m", "displayFactor": 0.001, "precision": 2, "step": 1000}},
    "momentC2": {"type": "number", "x-semio-ui": {"label": {"en": "Unbalanced column moment M_Ed,2 (eccentricity along c₂)", "de": "Stützenmoment M_Ed,2 (Ausmitte entlang c₂)"}, "description": {"en": "Design value; corner columns: positive when the load shifts towards the slab interior. Used with the method from unbalanced moments.", "de": "Bemessungswert; Eckstützen: positiv, wenn sich die Last zum Deckeninneren verschiebt. Wirksam bei der Methode aus Stützenmomenten."}, "widget": "stepper", "unit": "N·m", "displayUnit": "kN·m", "displayFactor": 0.001, "precision": 2, "step": 1000}}"""
for name in ("🔣️.json", "📸️snapshot/🔣️.json"):
    text = open(ANY + name, encoding="utf8").read()
    if '"PunchingBetaMethod"' in text:
        print("present", name)
        continue
    m = re.search(r'("asw": \{"type": "number", "x-semio-ui": \{[^\n]*\}\})\n(   \},\n   "x-semio-formats")', text)
    assert m and text.count('  "PunchingSpec": {\n') == 1
    text = text.replace(m.group(0), m.group(1) + "," + PROPS + "\n" + m.group(2))
    text = text.replace('  "PunchingSpec": {\n', DEF + '  "PunchingSpec": {\n')
    old = '"required": ["columnWidth", "columnDepth", "columnPosition", "asw"],'
    assert text.count(old) == 1
    text = text.replace(old, '"required": ["columnWidth", "columnDepth", "columnPosition", "asw", "betaMethod", "momentC1", "momentC2"],')
    open(ANY + name, "w", encoding="utf8").write(text)
    print("edited", name)
