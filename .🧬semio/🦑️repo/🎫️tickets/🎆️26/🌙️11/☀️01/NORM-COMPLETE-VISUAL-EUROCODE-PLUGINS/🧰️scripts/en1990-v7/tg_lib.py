"""Shared constants of the traffic-group cascade (ticket-local input script)."""
from lib import *
ENTITY, COLL, CAMEL, TABLE = "TrafficGroup", "traffic_groups", "trafficGroups", "en1990_traffic_group"
KEY = "LoadGroup"
KEYS = ["GR1A", "GR1B", "GR2", "GR3", "GR4", "GR5", "GR6"]
LIT = " | ".join('"%s"' % k for k in KEYS)
WIRE = "normWireLiteral(%s)" % ", ".join('"%s"' % k for k in KEYS)
N = "En1990" + ENTITY
M = "semio_s_artifact_norm_en1990_model::" + ENTITY
KM = "semio_s_artifact_norm_en1990_model::" + KEY
SNAP_URL = "https://json.schemas.assets.semio-tech.com/s/norm/en1990/snapshot.json#/$defs/"
def ui(en, de, **extra):
    return {"label": {"en": en, "de": de}, **extra}
OPTIONS = {
    "GR1A": ("gr1a tandem system, uniform load and footway", "gr1a Tandemsystem, Flächenlast und Gehweg"),
    "GR1B": ("gr1b single axle", "gr1b Einzelachse"),
    "GR2": ("gr2 horizontal forces", "gr2 Horizontalkräfte"),
    "GR3": ("gr3 pedestrian loads", "gr3 Fußgängerlasten"),
    "GR4": ("gr4 crowd loading", "gr4 Menschenansammlung"),
    "GR5": ("gr5 special vehicles", "gr5 Sonderfahrzeuge"),
    "GR6": ("gr6 (no normative table row registered, unverified)", "gr6 (keine normative Tabellenzeile registriert, ungeprüft)"),
}
def props(root):
    return {
        "id": {"type": "string"},
        "group": {"type": "string", "enum": KEYS, "x-semio-ui": ui("Traffic load group", "Verkehrslastgruppe", widget="select", options={k: {"en": v[0], "de": v[1]} for k, v in OPTIONS.items()}, description={"en": "Load group of EN 1991-2 Table 4.4a; the groups are mutually exclusive (EN 1990 A2.2.2).", "de": "Lastgruppe nach EN 1991-2 Tabelle 4.4a; die Gruppen schließen sich gegenseitig aus (EN 1990 A2.2.2)."})},
        "variableId": {"type": "string", "x-semio-ui": ui("Load component", "Lastbestandteil", widget="reference", role="value", ref={"kind": "variable"}, description={"en": "The variable action that is a component of this group.", "de": "Die veränderliche Einwirkung, die Bestandteil dieser Gruppe ist."})},
    }
def definition(root):
    p = props(root)
    d = {} if root else {"title": ENTITY}
    d.update({"type": "object", "additionalProperties": False, "required": list(p), "properties": p})
    if not root:
        d["x-semio-formats"] = ["🔣️jsonschema", "🔗️graphql", "🛰️protobuf"]
    return d
