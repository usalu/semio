"""EN 1999 wave 5 entity specs: fatigue categories, web patch loads, shell ring stiffeners. Shares the helper classes with wave 2."""
import pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent.parent / "en1999-wave2"))
from spec import *

ANGLE = {"unit": "rad", "displayUnit": "deg", "displayFactor": 57.29577951308232, "precision": 1, "step": 0.0175}
FORCE = {"unit": "N", "displayUnit": "kN", "displayFactor": 0.001, "precision": 1, "step": 100}

FATIGUE_CATEGORY = Ent(
    "FatigueCategory", "fatigue_categories", "FatigueCategories", "FatigueCategoryKey", "🎯️",
    "Fatigue-critical detail of a member (EN 1999-1-3 §6, Annex J): the site on the member where one detail category governs. The `fatigueDetails` rows with the same member and detail category are its stress-range spectrum blocks; the Δσ_C of the category is derived from `detailCategory`, the slopes `m1` and `m2` override the block slopes when positive.",
    [
        F("member_id", "str", "🏗️", "Member carrying the detail.", "Member", "Bauteil", ref="member"),
        F("element_id", "str", "🪵️", "Plate element of the member's section at the detail (empty = whole member).", "Plate element at the detail", "Blechelement am Detail", ref="element"),
        F("detail_category", "str", "🎛️", "Annex J detail category id (e.g. `71`, `40-weld`).", "Detail category", "Kerbfallklasse"),
        F("detail_type", "str", "🧷️", "Detail type: `plain` | `butt-weld` | `fillet-weld` | `bolted` | `riveted` | `other`.", "Detail type", "Detailtyp"),
        F("location_x", "f64", "📍️", "Location of the detail along the member axis [m].", "Location x along the member", "Lage x am Bauteil", {**MM, "step": 0.01}),
        F("m1", "f64", "🌳️", "First S-N slope m1; 0 = slope of the first stress block.", "S-N slope m1 (0 = from first block)", "Wöhler-Neigung m1 (0 = vom ersten Block)", {"unit": "", "precision": 1, "step": 0.5}),
        F("m2", "f64", "📜️", "Second S-N slope m2; 0 = m1 + 2.", "S-N slope m2 (0 = m1 + 2)", "Wöhler-Neigung m2 (0 = m1 + 2)", {"unit": "", "precision": 1, "step": 0.5}),
    ],
    "Fatigue detail site", "Ermüdungsdetail-Stelle",
)

PATCH_LOAD = Ent(
    "PatchLoad", "patch_loads", "PatchLoads", "PatchLoadKey", "⬇️",
    "Transverse patch load on a web (EN 1999-1-1 §6.7.5 for members, EN 1999-1-4 §6.1.7 for cold-formed sheets): a force introduced through a bearing length into the web of a member or a sheet. Web height, web slope and inner corner radius of 0 are taken from the section.",
    [
        F("subject_id", "str", "🎯️", "Member or cold-formed sheet whose web carries the load.", "Member or sheet", "Bauteil oder Blech", ref="member"),
        F("position", "str", "↔️", "Load position `end` (one-flange, near a support) | `interior`.", "Load position", "Lastlage"),
        F("x", "f64", "📍️", "Position of the load along the member or sheet [m].", "Position x", "Lage x", {**MM, "step": 0.01}),
        F("bearing_length", "f64", "📏️", "Bearing length s_s of the load on the flange [m].", "Bearing length s_s", "Lasteinleitungslänge s_s", {**MM, "step": 0.005}),
        F("web_height", "f64", "↕️", "Web height h_w between flange mid-lines [m]; 0 = from the section.", "Web height h_w (0 = section)", "Steghöhe h_w (0 = Querschnitt)", {**MM, "step": 0.005}),
        F("web_angle", "f64", "📐️", "Web slope φ against the flange [rad]; 0 = 90 degrees.", "Web slope φ (0 = 90°)", "Stegneigung φ (0 = 90°)", ANGLE),
        F("corner_radius", "f64", "⭕️", "Inner corner radius r between web and flange [m].", "Corner radius r", "Eckradius r", {**MM, "step": 0.001}),
        F("load", "f64", "⬇️", "Design transverse force F_Ed on the web [N].", "Transverse force F_Ed", "Querkraft F_Ed", FORCE),
    ],
    "Web patch load", "Stegeinzellast",
)

RING_STIFFENER = Ent(
    "RingStiffener", "ring_stiffeners", "RingStiffeners", "RingStiffenerKey", "⭕️",
    "Ring stiffener of a cylindrical shell (EN 1999-1-5 §5, §8): a flat-bar ring at axial position `x` that divides the shell into bays; its flat-bar depth and thickness give the ring section that is checked against the required stiffness.",
    [
        F("shell_id", "str", "🫙", "Shell the ring belongs to.", "Shell", "Schale", ref="shell"),
        F("x", "f64", "📍️", "Axial position of the ring from the shell base [m].", "Axial position x", "Axiale Lage x", {**MM, "step": 0.01}),
        F("depth", "f64", "↕️", "Radial depth of the flat-bar ring [m].", "Ring depth", "Ringhöhe", {**MM, "step": 0.005}),
        F("thickness", "f64", "🧱️", "Thickness of the flat-bar ring [m].", "Ring thickness", "Ringdicke", {**MM, "step": 0.001}),
        F("internal", "bool", "🔄️", "The ring stands inside the shell (otherwise outside).", "Internal ring", "Innenliegender Ring"),
    ],
    "Ring stiffener", "Ringsteife",
)

NEW = [FATIGUE_CATEGORY, PATCH_LOAD, RING_STIFFENER]
TABLE = {"fatigue_categories": "en1999_fatigue_category", "patch_loads": "en1999_patch_load", "ring_stiffeners": "en1999_ring_stiffener"}
CONST = {"fatigue_categories": "CATEGORY", "patch_loads": "PATCH", "ring_stiffeners": "RING"}
CHOICES = {"detail_type": "DETAIL_TYPES", "position": "PATCH_POSITIONS"}
CHOICE_VALUES = {
    "DETAIL_TYPES": [("plain", "Plain material", "Grundwerkstoff"), ("butt-weld", "Butt weld", "Stumpfnaht"), ("fillet-weld", "Fillet weld", "Kehlnaht"), ("bolted", "Bolted connection", "Schraubverbindung"), ("riveted", "Riveted connection", "Nietverbindung"), ("other", "Other detail", "Sonstiges Detail")],
    "PATCH_POSITIONS": [("end", "End (near support)", "Ende (nahe Auflager)"), ("interior", "Interior", "Innen")],
}


def ent_of(e):
    return e


def reals(e):
    return [f for f in e.fields if f.ty == "f64"]


def col_index(e):
    return {f.snake: 4 + i for i, f in enumerate(e.fields)}


def real_cols(e):
    ix = col_index(e)
    return [ix[f.snake] for f in reals(e)]


def sql_cols(e):
    out = []
    for f in e.fields:
        if f.ty == "str":
            out.append(("%s_reference" % f.snake[:-3]) if f.snake.endswith("_id") else f.snake)
            out[-1] += " TEXT NOT NULL"
        elif f.ty == "f64":
            out.append("%s REAL" % f.snake)
        else:
            out.append("%s INTEGER NOT NULL CHECK(%s IN(0,1))" % (f.snake, f.snake))
    return out


def sql_name(f):
    return (f.snake[:-3] + "_reference") if f.ty == "str" and f.snake.endswith("_id") else f.snake
