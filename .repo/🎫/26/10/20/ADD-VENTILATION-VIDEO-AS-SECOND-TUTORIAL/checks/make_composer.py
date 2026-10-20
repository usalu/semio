import re, pathlib
src = pathlib.Path("Cooling/full_cooling_video.py").read_text()

head, _, rest = src.partition("#region Section Modules")
_, _, after_playlist = rest.partition("#region Host Binding")

header = head.replace(
    '"""🎬 Full Cooling load series — Parts 1–6 in curriculum order.',
    '"""🎬 Full Ventilation series — Chapters 1–6 in curriculum order (video 2 of 5).',
)
header = (header.replace("_COOLING_ROOT", "_VENTILATION_ROOT")
                .replace('SERIES_INTRO_SCENE = "Demo_Intro_Kuehllast"', 'SERIES_INTRO_SCENE = "Demo_Intro_Lueftung"'))

modules = '''#region Section Modules
_p1 = _load_module("ventilation_full_p1", _VENTILATION_ROOT / "1_warum_lueften" / "scene_1.py")
_p2 = _load_module("ventilation_full_p2", _VENTILATION_ROOT / "2_luftmenge" / "scene_2.py")
_p3 = _load_module("ventilation_full_p3", _VENTILATION_ROOT / "3_waermestrom_luft" / "scene_3.py")
_p4 = _load_module("ventilation_full_p4", _VENTILATION_ROOT / "4_natuerliche_lueftung" / "scene_4.py")
_p5 = _load_module("ventilation_full_p5", _VENTILATION_ROOT / "5_mechanische_lueftung" / "scene_5.py")
_p6 = _load_module("ventilation_full_p6", _VENTILATION_ROOT / "6_uebergabe" / "scene_6.py")
#endregion


#region Playlist
VENTILATION_PLAYLIST: list[tuple[str, list[type[Scene]]]] = [
    (
        "Kapitel 1 · Warum lüften?",
        [_p1.Beat1_Lueftungsbedarf, _p1.Beat2_Lueftungsweg, _p1.Beat3_Fahrplan],
    ),
    (
        "Kapitel 2 · Wie viel Luft?",
        [_p2.Beat1_Innenvolumen, _p2.Beat2_Luftwechselrate, _p2.Beat3_Volumenstrom],
    ),
    (
        "Kapitel 3 · Was die Luft trägt",
        [_p3.Beat1_Luftwaerme, _p3.Beat2_SensibleWaerme, _p3.Beat3_LatenteLast, _p3.Beat4_WinterUndSommer],
    ),
    (
        "Kapitel 4 · Natürliche Lüftung",
        [
            _p4.Beat1_Fensterregeln,
            _p4.Beat2_StossOderKipp,
            _p4.Beat3_Querlueftung,
            _p4.Beat4_Auftrieb,
            _p4.Beat5_Nachtlueftung,
            _p4.Beat6_GrenzenDerFreienLueftung,
        ],
    ),
    (
        "Kapitel 5 · Mechanische Lüftung und Wärmerückgewinnung",
        [_p5.Beat1_Grundtypen, _p5.Beat2_Waermerueckgewinnung, _p5.Beat3_Anlagenvergleich, _p5.Beat4_Kanalquerschnitt],
    ),
    (
        "Kapitel 6 · Übergabe an Heizung und Kühlung",
        [_p6.Beat1_Zusammenfassung, _p6.Beat2_UebergabeHeizungKuehlung],
    ),
]
#endregion


'''

body = "#region Host Binding" + after_playlist
sections = '''#region Section Scenes
class _VentilationSection(Scene):
    """📚 One curriculum section — keeps Manim partial counts manageable."""

    section_beats: list[type[Scene]] = []

    def construct(self):
        for scene_cls in self.section_beats:
            _bind_scene_attrs(self, scene_cls)
            scene_cls.construct(self)
            self.clear()


class Ventilation_01_WarumLueften(_VentilationSection):
    """1️⃣ Kapitel 1 — Warum lüften?"""

    section_beats = VENTILATION_PLAYLIST[0][1]


class Ventilation_02_Luftmenge(_VentilationSection):
    """2️⃣ Kapitel 2 — Wie viel Luft?"""

    section_beats = VENTILATION_PLAYLIST[1][1]


class Ventilation_03_WaermestromLuft(_VentilationSection):
    """3️⃣ Kapitel 3 — Was die Luft trägt."""

    section_beats = VENTILATION_PLAYLIST[2][1]


class Ventilation_04_NatuerlicheLueftung(_VentilationSection):
    """4️⃣ Kapitel 4 — Natürliche Lüftung."""

    section_beats = VENTILATION_PLAYLIST[3][1]


class Ventilation_05_MechanischeLueftung(_VentilationSection):
    """5️⃣ Kapitel 5 — Mechanische Lüftung und Wärmerückgewinnung."""

    section_beats = VENTILATION_PLAYLIST[4][1]


class Ventilation_06_Uebergabe(_VentilationSection):
    """6️⃣ Kapitel 6 — Übergabe an Heizung und Kühlung."""

    section_beats = VENTILATION_PLAYLIST[5][1]


SECTION_SCENES: list[type[Scene]] = [
    Ventilation_01_WarumLueften,
    Ventilation_02_Luftmenge,
    Ventilation_03_WaermestromLuft,
    Ventilation_04_NatuerlicheLueftung,
    Ventilation_05_MechanischeLueftung,
    Ventilation_06_Uebergabe,
]
#endregion
'''
a, _, tail = body.partition("#region Section Scenes")
_, _, after_sections = tail.partition("#region Full Series Scene (fragile mega-combine)")
body = a + sections + "\n\n#region Full Series Scene (fragile mega-combine)" + after_sections

out = header + modules + body
out = (out.replace("_COOLING_ROOT", "_VENTILATION_ROOT")
          .replace("COOLING_PLAYLIST", "VENTILATION_PLAYLIST")
          .replace("FullCoolingDemandVideo", "FullVentilationVideo")
          .replace("full_cooling_video", "full_ventilation_video")
          .replace("compose_full_cooling_video", "compose_full_ventilation_video")
          .replace("Full_Cooling_Demand_NoAudio_", "Full_Ventilation_NoAudio_")
          .replace("Full_Cooling_Demand_", "Full_Ventilation_")
          .replace("_CoolingSection", "_VentilationSection")
          .replace("Entire Cooling series", "Entire Ventilation series")
          .replace("❄️ Entire", "🌬️ Entire")
          .replace("Render Cooling intro", "Render Ventilation intro"))
out = out.replace("compose_full_cooling_video", "compose_full_ventilation_video")
pathlib.Path("Ventilation/full_ventilation_video.py").write_text(out)
