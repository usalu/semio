"""en1997 `piles[].concreteClass`: enum def + property in the snapshot and artifact JSON, patch property in the diff JSON (idempotent); twins and the Pile struct then come from `schema twins generate`."""
import os, sys
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/field-adder")
import schema_twins as st

ANY = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any"
CLASSES = [("notGiven", "Not given (explicit modulus or none)", "Nicht angegeben (expliziter Modul oder keiner)"), ("c1215", "C12/15", "C12/15"), ("c1620", "C16/20", "C16/20"), ("c2025", "C20/25", "C20/25"), ("c2530", "C25/30", "C25/30"), ("c3037", "C30/37", "C30/37"), ("c3545", "C35/45", "C35/45"), ("c4050", "C40/50", "C40/50"), ("c4555", "C45/55", "C45/55"), ("c5060", "C50/60", "C50/60")]
DEF = {"title": "PileConcreteClass", "type": "string", "enum": [c[0] for c in CLASSES], "x-semio-ui": {"options": {c[0]: {"en": c[1], "de": c[2]} for c in CLASSES}}}
UI = {"label": {"en": "Concrete class of the shaft", "de": "Betonfestigkeitsklasse des Schafts"}, "description": {"en": "EN 1992-1-1 Table 3.1 class; E_cm of the elastic shortening comes from the shared registry unless an explicit modulus E_c is given.", "de": "Klasse nach EN 1992-1-1 Tabelle 3.1; E_cm der elastischen Stauchung stammt aus dem gemeinsamen Register, sofern kein expliziter Modul E_c angegeben ist."}}
DOC = "🪵 Concrete strength class of the pile shaft (EN 1992-1-1 Table 3.1) for the elastic shortening of the resistance-settlement curve (PILE-09): E_cm from the shared rows `ndp.en1992.t3-1.ecm.*`; `notGiven` leaves the explicit modulus `elasticModulus` as the only source, a modulus above 0 overrides the class."


def snapshot(path, ui):
    def defs(data):
        d = st.defs_of(data)
        if "PileConcreteClass" in d:
            return False
        ordered = st.insert_after(dict(d), "PileConcreteClass", DEF, "Pile")
        d.clear()
        d.update(ordered)
    st.edit_schema(path, defs)
    st.add_field(path, "concreteClass", {"ref": "PileConcreteClass"}, after="hammerEfficiency", container="Pile", ui=ui, doc=DOC)


snapshot(ANY + "/🧬️schema/📸️snapshot/🔣️.json", UI)
snapshot(ANY + "/🧬️schema/🔣️.json", None)


def diff(data):
    node = data["$defs"]["En1997PilesPatch"]
    if "concreteClass" in node["properties"]:
        return False
    ref = "https://json.schemas.assets.semio-tech.com/s/norm/en1997/snapshot.json#/$defs/Pile/properties/concreteClass"
    node["properties"] = st.insert_after(node["properties"], "concreteClass", {"anyOf": [{"$ref": ref}, {"type": "null"}]}, "hammerEfficiency")
    req = node["required"]
    req.insert(req.index("hammerEfficiency") + 1, "concreteClass")


st.edit_schema(ANY + "/🧬️schema/🔺️diff/🔣️.json", diff)
