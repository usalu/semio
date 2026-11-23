"""🗺️ Ventilation roadmap — the Fahrplan that opens the Lüftung video, ahead of Chapter 1 (Warum lüften?).

Six cards light up in order and the last one hands the ventilation load over to Heizung and Kühltechnik.
"""

import importlib
import sys
from pathlib import Path

_WORKSPACE_ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
if str(_WORKSPACE_ROOT) not in sys.path:
    sys.path.insert(0, str(_WORKSPACE_ROOT))

from manim import *

_roadmap = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🗺️roadmap.🐍️")
RoadmapScene = _roadmap.RoadmapScene
_visuals = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
load_vo_timing = _visuals.load_vo_timing

_VO_TIMING = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.📁️output.🐍️").speech_root(Path(__file__).resolve().parent) / "timing.json"
if _VO_TIMING.is_file():
    load_vo_timing(_VO_TIMING)


#region Beat1 – Roadmap of the video
class Beat1_Fahrplan(RoadmapScene):
    """🗺️ Roadmap of the video as six cards that light up in order, ending on the hand-over chips for Heizung and Kühltechnik."""

    NARRATION = [
        ("intro",
         "This video follows six steps.",
         "Dieses Video folgt sechs Schritten."),
        ("bedarf",
         "First, why we ventilate: carbon dioxide, moisture and odours.",
         "Zuerst, warum wir lüften: Kohlendioxid, Feuchte und Gerüche."),
        ("menge",
         "Then, how much air: volume, air change rate and volume flow.",
         "Dann die Luftmenge: Volumen, Luftwechselrate und Volumenstrom."),
        ("waerme",
         r"Next, the heat flow $\Phi$ that the air carries: temperature difference, heat capacity and humidity.",
         r"Danach der Wärmestrom $\Phi$, den die Luft trägt: Temperaturdifferenz, Wärmekapazität und Feuchte."),
        ("natuerlich",
         "Then natural ventilation through windows.",
         "Es folgt die natürliche Lüftung durch Fenster."),
        ("mechanisch",
         "And mechanical ventilation with heat recovery.",
         "Dann die mechanische Lüftung mit Wärmerückgewinnung."),
        ("uebergabe",
         r"Last, the hand-over: heating takes the winter value of $\dot{Q}_{\mathrm{V}}$, cooling the summer value.",
         r"Zuletzt die Übergabe: Die Heizung übernimmt den Winterwert von $\dot{Q}_{\mathrm{V}}$, die Kühltechnik den Sommerwert."),
    ]

    HEADING = ("The roadmap of this video", "Der Fahrplan des Videos")
    STEPS = [
        {"key": "bedarf", "name": ("Why ventilate", "Warum lüften"), "symbols": r"\mathrm{CO_2}", "pictogram": "person", "color": "PASTEL_PURPLE"},
        {"key": "menge", "name": ("Air volume", "Luftmenge"), "symbols": r"V,\;n,\;\dot{V}", "pictogram": "air-room", "color": "P_CYAN"},
        {"key": "waerme", "name": ("Heat flow", "Wärmestrom"), "symbols": r"\Phi", "pictogram": "thermometer", "color": "P_YELLOW"},
        {"key": "natuerlich", "name": ("natural", "natürlich"), "symbols": r"A_{\mathrm{eff}}", "pictogram": "window", "color": "P_GREEN"},
        {"key": "mechanisch", "name": ("mechanical", "mechanisch"), "symbols": r"\eta_{\mathrm{WRG}}", "pictogram": "mvhr", "color": "P_PRIMARY"},
        {"key": "uebergabe", "name": ("Handover", "Übergabe"), "symbols": r"\dot{Q}_{\mathrm{V}}", "pictogram": "house", "color": "P_ORANGE"},
    ]
    BRACKET = {"last": 4, "label": ("applies in winter and summer alike", "gilt im Winter wie im Sommer")}
    HANDOVERS = [
        {"text": (r"\to\;\text{Heating:}\;\dot{Q}_{\mathrm{V}}", r"\to\;\text{Heizung:}\;\dot{Q}_{\mathrm{V}}"), "color": "P_ORANGE"},
        {"text": (r"\to\;\text{Cooling:}\;\dot{Q}_{\mathrm{V}}", r"\to\;\text{Kühlung:}\;\dot{Q}_{\mathrm{V}}"), "color": "P_CYAN"},
    ]
#endregion
