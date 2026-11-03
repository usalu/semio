"""v13 step 0: adds the `StiffnessBasis` enum and the two basis properties to the en1998 schema JSONs (schema, snapshot, diff); idempotent; run with python -I."""
import json

ANY = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema"
DEF = '"StiffnessBasis":{"title":"StiffnessBasis","type":"string","enum":["simplified","declared"],"x-semio-ui":{"options":{"simplified":{"en":"Simplified (cracked-stiffness factor of the registry, EN 1998-1 4.3.1(6))","de":"Vereinfacht (Faktor der gerissenen Steifigkeit aus dem Register, EN 1998-1 4.3.1(6))"},"declared":{"en":"Declared (effective share from a more accurate analysis)","de":"Angegeben (wirksamer Anteil aus genauerer Berechnung)"}}},"x-semio-formats":["🔣️jsonschema","🟦️typescript","🔗️graphql"]},\n    '
BUILDING = '"storeyStiffnessBasis":{"$ref":"#/$defs/StiffnessBasis","x-semio-ui":{"label":{"en":"Stiffness basis of the entered gross storey stiffnesses (EN 1998-1 4.3.1(6))","de":"Steifigkeitsbasis der eingegebenen Brutto-Geschosssteifigkeiten (EN 1998-1 4.3.1(6))"},"description":{"en":"Simplified applies the cracked-stiffness factor of the registry, declared applies the effective share below.","de":"Vereinfacht wendet den Faktor der gerissenen Steifigkeit aus dem Register an, angegeben den wirksamen Anteil darunter."}}},'
MEMBER = '"effectiveStiffnessBasis":{"$ref":"#/$defs/StiffnessBasis","x-semio-ui":{"label":{"en":"Stiffness basis of the gross wall stiffness (EN 1998-1 4.3.1(6))","de":"Steifigkeitsbasis der Brutto-Wandsteifigkeit (EN 1998-1 4.3.1(6))"},"description":{"en":"Simplified applies the cracked-stiffness factor of the registry, declared applies the effective share below.","de":"Vereinfacht wendet den Faktor der gerissenen Steifigkeit aus dem Register an, angegeben den wirksamen Anteil darunter."}}},'
RATIO_B = ('"storeyStiffnessRatio":{"type":"number","x-semio-ui":{"label":{"en":"Effective share of the entered gross storey stiffnesses (EN 1998-1 4.3.1(6)): 0 = simplified', '"storeyStiffnessRatio":{"type":"number","x-semio-ui":{"label":{"en":"Declared effective share of the entered gross storey stiffnesses (EN 1998-1 4.3.1(6)), used only when the basis is declared; 0 < ratio <= 1 (1 = entered values are already effective)","de":"Angegebener wirksamer Anteil der eingegebenen Brutto-Geschosssteifigkeiten (EN 1998-1 4.3.1(6)), nur bei Basis angegeben verwendet; 0 < Anteil <= 1 (1 = eingegebene Werte sind bereits wirksam)"},"widget":"stepper","precision":3,"step":0.001}}')
RATIO_M = ('"effectiveStiffnessRatio":{"type":"number","x-semio-ui":{"label":{"en":"Effective share of the gross wall stiffness (EN 1998-1 4.3.1(6)): 0 = simplified', '"effectiveStiffnessRatio":{"type":"number","x-semio-ui":{"label":{"en":"Declared effective share of the gross wall stiffness (EN 1998-1 4.3.1(6)), used only when the basis is declared; 0 < ratio <= 1 (1 = uncracked)","de":"Angegebener wirksamer Anteil der Brutto-Wandsteifigkeit (EN 1998-1 4.3.1(6)), nur bei Basis angegeben verwendet; 0 < Anteil <= 1 (1 = ungerissen)"},"widget":"stepper","precision":3,"step":0.001}}')


def cut_ratio(t, key, head):
    i = t.index(head)
    j = t.index('"step":0.001}}', i) + len('"step":0.001}}')
    return i, j


def full(path):
    t = open(path, encoding="utf-8").read()
    if '"StiffnessBasis"' in t:
        return
    assert t.count('"MemberKind":{') == 1
    t = t.replace('"MemberKind":{', DEF + '"MemberKind":{', 1)
    for key, basis, (head, new) in (("storeyStiffnessRatio", BUILDING, RATIO_B), ("effectiveStiffnessRatio", MEMBER, RATIO_M)):
        i, j = cut_ratio(t, key, head)
        t = t[:i] + basis + new + t[j:]
        old_req = ',"%s"],"properties"' % key
        assert t.count(old_req) == 1, key
        t = t.replace(old_req, ',"%s","%s"],"properties"' % (key.replace("Ratio", "Basis"), key))
    json.loads(t)
    open(path, "w", encoding="utf-8").write(t)


def diff(path):
    t = open(path, encoding="utf-8").read()
    if "effectiveStiffnessBasis" in t:
        return
    for key in ("storeyStiffnessRatio", "effectiveStiffnessRatio"):
        a = '"%s":{"anyOf":[{"type":"number"},{"type":"null"}]}' % key
        assert t.count(a) == 1, key
        b = '"%s":{"anyOf":[{"type":"string","enum":["simplified","declared"]},{"type":"null"}]},' % key.replace("Ratio", "Basis")
        t = t.replace(a, b + a)
    json.loads(t)
    open(path, "w", encoding="utf-8").write(t)


full(ANY + "/🔣️.json")
full(ANY + "/📸️snapshot/🔣️.json")
diff(ANY + "/🔺️diff/🔣️.json")
