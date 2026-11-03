"""EN 1999 wave 2 entity specs (weld runs, section outline vertices) shared by the generators in this folder."""
import json, os, pathlib, re

ART = pathlib.Path(os.environ.get("EN1999_ANY", "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any"))
SCHEMA = ART / "🧬️schema"
SNAP = SCHEMA / "📸️snapshot"
DIFF = SCHEMA / "🔺️diff"
SQL = ART / "🚪️io/🪶️sqlite/📸️snapshot"


class F:
    def __init__(self, snake, ty, emoji, doc, en, de, ui=None, ref=None, desc=None):
        self.snake, self.ty, self.emoji, self.doc, self.en, self.de, self.ui, self.ref, self.desc = snake, ty, emoji, doc, en, de, ui or {}, ref, desc
        parts = snake.split("_")
        self.camel = parts[0] + "".join(p.capitalize() for p in parts[1:])

    @property
    def rust(self):
        return {"str": "String", "f64": "f64", "bool": "bool"}[self.ty]

    @property
    def ts(self):
        return {"str": "string", "f64": "number", "bool": "boolean"}[self.ty]

    @property
    def wire(self):
        return {"str": "normWireString", "f64": "normWireNumber", "bool": "normWireBoolean"}[self.ty]

    @property
    def gql(self):
        return {"str": "String", "f64": "Float", "bool": "Boolean"}[self.ty]

    @property
    def proto(self):
        return {"str": "string", "f64": "double", "bool": "bool"}[self.ty]

    @property
    def json(self):
        return {"str": "string", "f64": "number", "bool": "boolean"}[self.ty]

    def schema_ui(self):
        ui = {}
        if self.ty == "str" and self.ref:
            ui = {"widget": "reference", "label": {"en": self.en, "de": self.de}, "ref": {"kind": self.ref}}
        elif self.ty == "str":
            ui = {"widget": "text", "label": {"en": self.en, "de": self.de}}
        elif self.ty == "bool":
            ui = {"label": {"en": self.en, "de": self.de}, "widget": "toggle"}
        else:
            ui = {"label": {"en": self.en, "de": self.de}, "widget": "stepper", **self.ui}
        if self.desc:
            ui["description"] = {"en": self.desc[0], "de": self.desc[1]}
        return ui


MM = {"unit": "m", "displayUnit": "mm", "displayFactor": 1000, "precision": 1}


class Ent:
    def __init__(self, struct, field, lane, key, emoji, doc, fields, id_en, id_de):
        self.struct, self.field, self.lane, self.key, self.emoji, self.doc, self.fields, self.id_en, self.id_de = struct, field, lane, key, emoji, doc, fields, id_en, id_de
        parts = field.split("_")
        self.camel = parts[0] + "".join(p.capitalize() for p in parts[1:])

    @property
    def all_fields(self):
        return [F("id", "str", "🆔️", "", self.id_en, self.id_de)] + self.fields


WELD_RUN = Ent(
    "WeldRun", "weld_runs", "WeldRuns", "WeldRunKey", "🧵️",
    "Explicit weld run along a member (EN 1999-1-1 §6.1.6, §6.3.3.3, §8.6): a longitudinal run lies between `start` and `start + length` along the member axis, a transverse run sits at `start` (a localized weld, `length` is ignored). It overrides the weld flags of the named plate element in the member's checks and drives the localized-weld factor ω_x.",
    [
        F("member_id", "str", "🏗️", "Member the run lies on.", "Welded member", "Geschweißtes Bauteil", ref="member"),
        F("element_id", "str", "🧱️", "Plate element of the member's section carrying the weld; empty = no element (only a transverse run affects the member).", "Welded plate element", "Geschweißtes Blechelement"),
        F("start", "f64", "📍️", "Start (longitudinal run) or position (transverse run) x along the member axis [m].", "Run start / position x_s", "Nahtanfang / Lage x_s", {**MM, "step": 0.01}),
        F("length", "f64", "📏️", "Run length along the member axis [m]; ignored for a transverse run.", "Run length", "Nahtlänge", {**MM, "step": 0.01}),
        F("offset", "f64", "🔀️", "Weld position across the plate element from its root [m] (0 = at the junction).", "Weld position across the element", "Nahtlage quer zum Element", {**MM, "step": 0.001}),
        F("throat", "f64", "🔥️", "Fillet weld throat thickness a [m].", "Weld throat a", "Kehlnahtdicke a", {**MM, "step": 0.0005}),
        F("process", "str", "⚙️", "Welding process `mig` | `tig` (HAZ extent factor).", "Welding process", "Schweißverfahren"),
        F("t_junction", "bool", "🔱️", "The run sits in a T-junction (HAZ extent × 1.33).", "T-junction weld", "Schweißnaht im T-Stoß"),
        F("direction", "str", "🧭️", "`longitudinal` (HAZ strips beside the line) | `transverse` (HAZ across the whole element width, localized weld on the member).", "Weld direction", "Schweißrichtung"),
        F("haz_extent", "f64", "♨️", "HAZ extent b_haz per side [m]; 0 = table of EN 1999-1-1 §6.1.6.3 by thickness, process and junction.", "HAZ extent b_haz override (0 = table)", "WEZ-Breite b_haz abweichend (0 = Tabelle)", {**MM, "step": 0.005}),
    ],
    "Welded run", "Schweißnaht",
)

SECTION_VERTEX = Ent(
    "SectionVertex", "outline_vertices", "OutlineVertices", "SectionVertexKey", "🔷️",
    "Vertex of the free section outline (EN 1999-1-1 §6.1): the vertices of a section in list order form its closed rings; ring `outer` is the outer boundary, every other ring name is a hole. A section of kind `free` takes its gross properties from the polygon.",
    [
        F("section_id", "str", "📏️", "Section the vertex belongs to.", "Section", "Querschnitt", ref="section"),
        F("ring", "str", "⭕️", "Ring name: `outer` for the outer boundary, any other name for a hole.", "Ring (outer or hole name)", "Ring (outer oder Lochname)"),
        F("element_id", "str", "🪵️", "Plate element carried by the edge from this vertex to the next vertex of the ring (empty = none).", "Plate element of the edge", "Blechelement der Kante", ref="element"),
        F("y", "f64", "↗️", "Horizontal coordinate y of the vertex in the section plane [m].", "Vertex y", "Knoten y", {**MM, "step": 0.001}),
        F("z", "f64", "🔝️", "Vertical coordinate z of the vertex in the section plane [m].", "Vertex z", "Knoten z", {**MM, "step": 0.001}),
    ],
    "Outline vertex", "Umrissknoten",
)

ENTS = [WELD_RUN, SECTION_VERTEX]
