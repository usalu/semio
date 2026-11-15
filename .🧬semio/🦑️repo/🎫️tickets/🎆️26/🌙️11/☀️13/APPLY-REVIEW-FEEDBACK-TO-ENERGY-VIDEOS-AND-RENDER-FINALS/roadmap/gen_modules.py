import re
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")
root = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
energy = root / "🎓️teaching/🏛️architecture/⚡️energy"
need = energy / "🌬️ventilation/🎬️clip/❓️need"
speech_template = (need / "🔊️speech/🐍️.py").read_text(encoding="utf-8")
composition_template = (need / "🏗️composition/🐍️.py").read_text(encoding="utf-8")

VIDEOS = {
    "🌬️ventilation": ("Ventilation", "Full_Ventilation_Roadmap.mp4"),
    "🧲️physics": ("Physical Fundamentals", "Full_Physical_Fundamentals_Roadmap.mp4"),
    "🔥️heating": ("Heating", "Full_Heating_Roadmap.mp4"),
    "❄️cooling": ("Cooling", "Full_Cooling_Roadmap.mp4"),
    "📊️demand": ("Energy Balance", "Full_EnergyBalance_Roadmap.mp4"),
}

for family, (title, output) in VIDEOS.items():
    folder = energy / family / "🎬️clip/🗺️roadmap"
    module = f"🎓️teaching.🏛️architecture.⚡️energy.{family}.🎬️clip.🗺️roadmap.🐍️"
    speech = re.sub(r'""".*?\n', f'"""🔊 Subtitle voiceover for the {title} roadmap (Fahrplan) — synth, align, verify.\n', speech_template, count=1)
    speech = re.sub(r'_scene = import_module\("[^"]+"\)', f'_scene = import_module("{module}")', speech, count=1)
    speech = re.sub(r"Beat1_Lueftungsbedarf = _scene\.Beat1_Lueftungsbedarf\nBeat2_Lueftungsweg = _scene\.Beat2_Lueftungsweg\nBeat3_Fahrplan = _scene\.Beat3_Fahrplan\n",
                    "Beat1_Fahrplan = _scene.Beat1_Fahrplan\n", speech, count=1)
    speech = re.sub(r"BEATS = \[\n.*?\n\]\n", "BEATS = [\n    Beat1_Fahrplan,\n]\n", speech, count=1, flags=re.S)
    composition = re.sub(r'""".*?\n', '"""🎬 Render the roadmap beat with optional narration."""\n', composition_template, count=1)
    composition = re.sub(r"SCENES = \[\n.*?\n\]\n", 'SCENES = [\n    (BASE_DIR / "🐍️.py", "Beat1_Fahrplan", SPEECH_DIR / "beat_1_audio.mp3"),\n]\n', composition, count=1, flags=re.S)
    composition = re.sub(r'"Full_Ventilation_Chapter1_Warum_Lueften\.mp4"', f'"{output}"', composition, count=1)
    for sub, text in (("🔊️speech", speech), ("🏗️composition", composition)):
        target = folder / sub / "🐍️.py"
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(text, encoding="utf-8", newline="\n")
        print(target.relative_to(energy))
