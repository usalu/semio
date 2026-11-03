"""EN 1999 wave 2 transverse web stiffener spec: one id-keyed collection `transverseStiffeners[]` on top of the wave-5 pipeline. Honours EN1999_ANY (scratch-copy redirect) through the wave-2 spec."""
import pathlib, sys
HERE = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / "en1999-wave2"))
from spec import *

TRANSVERSE_STIFFENER = Ent(
    "TransverseStiffener", "transverse_stiffeners", "TransverseStiffeners", "TransverseStiffenerKey", "🦴️",
    "Transverse web stiffener of a plate girder (EN 1999-1-1 §6.7.6, §6.7.7; spec A-28): a flat bar welded across the web of the girder member at `position` along its axis, on one or on both sides of the web. The stiffener with an effective web strip (15·ε·t_w per side) must keep the required stiffness and carry the shear-induced axial force as a strut; the spacing of the stiffeners of one girder gives the panel widths a.",
    [
        F("girder_id", "str", "🏗️", "Plate girder member the stiffener stands on.", "Plate girder", "Blechträger", ref="member"),
        F("position", "f64", "📍️", "Position of the stiffener along the girder axis from its start [m].", "Position along the girder", "Lage entlang des Trägers", {**MM, "step": 0.01}),
        F("width", "f64", "↔️", "Outstand width b_st of the flat bar, measured from the web face [m].", "Stiffener width b_st", "Steifenbreite b_st", {**MM, "step": 0.005}),
        F("thickness", "f64", "🧱️", "Thickness t_st of the flat bar [m].", "Stiffener thickness t_st", "Steifendicke t_st", {**MM, "step": 0.001}),
        F("sides", "str", "🪞️", "Arrangement `one` (single-sided) | `two` (double-sided, symmetric about the web).", "Sides of the web", "Stegseiten"),
    ],
    "Transverse stiffener", "Quersteife",
)

NEW = [TRANSVERSE_STIFFENER]
TABLE = {"transverse_stiffeners": "en1999_transverse_stiffener"}
CONST = {"transverse_stiffeners": "STIFFENER"}
CHOICES = {("TransverseStiffener", "sides"): "STIFFENER_SIDES"}
CHOICE_VALUES = {"STIFFENER_SIDES": [("one", "One-sided", "Einseitig"), ("two", "Two-sided", "Zweiseitig")]}
PREV = {"struct": "RingStiffener", "field": "ring_stiffeners", "camel": "ringStiffeners", "lane": "RingStiffeners", "const": "RING"}

SAMPLE = {"transverseStiffeners": [{"id": "stiffener-1", "girderId": "purlin-1", "position": 1.5, "width": 0.08, "thickness": 0.008, "sides": "two"}]}


def choice_of(e, f):
    return CHOICES.get((e.struct, f.snake))


def reals(e):
    return [f for f in e.fields if f.ty == "f64"]


def col_index(e):
    return {f.snake: 4 + i for i, f in enumerate(e.fields)}


def real_cols(e):
    ix = col_index(e)
    return [ix[f.snake] for f in reals(e)]


def sql_name(f):
    return (f.snake[:-3] + "_reference") if f.ty == "str" and f.snake.endswith("_id") else f.snake


def clone_prev(text, e):
    return text.replace(PREV["lane"], e.lane).replace(PREV["struct"], e.struct).replace(PREV["field"], e.field).replace(PREV["camel"], e.camel)
