"""🗂️ Series registry data shared by the registration generators of the Nakagin clips."""

from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[8]
ARCH = ROOT / "🎓️teaching" / "🏛️architecture"
TICKET = Path(__file__).resolve().parents[1]
CLIP_AUTHORING = ARCH / "⚡️energy" / "🏗️authoring" / "🎬️clip"

SERIES = {
    "grasshopper": {
        "folder": "🦗️grasshopper",
        "label": "Grasshopper",
        "icon": "🦗",
        "videos": [
            ("grasshopper_anatomy", "Beat1_Anatomy", "🦴️anatomy", "Grasshopper_01_Anatomy", "Grasshopper_anatomy", "🦴️",
             {"en": "Anatomy of a Grasshopper Script", "de": "Aufbau eines Grasshopper-Skripts"}),
            ("grasshopper_capsule", "Beat2_Capsule", "💊️capsule", "Grasshopper_02_Capsule", "Grasshopper_capsule", "💊️",
             {"en": "One Parametric Capsule", "de": "Eine parametrische Kapsel"}),
            ("grasshopper_tower", "Beat3_Tower", "🗼️tower", "Grasshopper_03_Tower", "Grasshopper_tower", "🗼️",
             {"en": "From One Capsule to the Tower", "de": "Von der Kapsel zum Turm"}),
            ("grasshopper_casting", "Beat4_Casting", "🫗️casting", "Grasshopper_04_Casting", "Grasshopper_casting", "🫗️",
             {"en": "Casting Data Types", "de": "Datentypen und Casting"}),
            ("grasshopper_trees", "Beat5_DataTrees", "🌳️trees", "Grasshopper_05_Trees", "Grasshopper_trees", "🌳️",
             {"en": "Data Trees on the Tower", "de": "Datenbäume am Turm"}),
        ],
    },
    "excel": {
        "folder": "📊️excel",
        "label": "Excel",
        "icon": "📊",
        "videos": [
            ("excel_anatomy", "Beat1_Anatomy", "🦴️anatomy", "Excel_01_Anatomy", "Excel_anatomy", "🦴️",
             {"en": "The Anatomy of a Workbook", "de": "Aufbau einer Arbeitsmappe"}),
            ("excel_references", "Beat2_References", "📍️references", "Excel_02_References", "Excel_references", "📍️",
             {"en": "Relative, Absolute and Mixed References", "de": "Relative, absolute und gemischte Bezüge"}),
            ("excel_program", "Beat3_Program", "📋️program", "Excel_03_Program", "Excel_program", "📋️",
             {"en": "The Room Program in the Workbook", "de": "Das Raumprogramm in der Arbeitsmappe"}),
            ("excel_energy", "Beat4_Energy", "⚡️energy", "Excel_04_Energy", "Excel_energy", "⚡️",
             {"en": "Heating Demand in the Nakagin Workbook", "de": "Heizwärmebedarf in der Nakagin-Arbeitsmappe"}),
        ],
    },
}
