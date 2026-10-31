"""🔁 Reruns every failed preview of the Grasshopper and Excel series once the batch has launched all 18 and a slot is free (at most 6 previews alive).
Usage: python rerun_queue.py   (runs until all 18 mp4 files exist or a target failed three times)
"""
import subprocess
import sys
import time
from pathlib import Path

TICKET = Path(__file__).resolve().parents[1]
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
PY = ROOT / ".🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe"
LOGS = TICKET / "🗑️generated/preview"
SCENES = ROOT / "🎓️teaching/🏛️architecture"
TARGETS = [("🦗️grasshopper", "💊️capsule", "Beat2_Capsule", "Grasshopper_capsule"), ("🦗️grasshopper", "🗼️tower", "Beat3_Tower", "Grasshopper_tower"),
           ("🦗️grasshopper", "🌳️trees", "Beat5_DataTrees", "Grasshopper_trees"), ("🦗️grasshopper", "🫗️casting", "Beat4_Casting", "Grasshopper_casting"),
           ("🦗️grasshopper", "🦴️anatomy", "Beat1_Anatomy", "Grasshopper_anatomy"), ("📊️excel", "🦴️anatomy", "Beat1_Anatomy", "Excel_anatomy"),
           ("📊️excel", "📍️references", "Beat2_References", "Excel_references"), ("📊️excel", "📋️program", "Beat3_Program", "Excel_program"),
           ("📊️excel", "⚡️energy", "Beat4_Energy", "Excel_energy")]
JOBS = [(series, topic, scene, output, lang) for lang in ("en", "de") for series, topic, scene, output in TARGETS]
LIST = ROOT / ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/render/procs.ps1"
tries = {}


def mp4(output, lang):
    return ROOT / ".🧬semio/🎓️teaching/⚡️cache/videos" / lang / "speaker/preview" / f"{output}_speaker_{lang}_480p15.mp4"


def alive():
    text = subprocess.run(["powershell", "-NoProfile", "-File", str(LIST)], capture_output=True, text=True, encoding="utf-8", errors="replace").stdout
    return [line for line in text.splitlines() if " preview.py " in line and "preview_any" not in line]


def fresh(series, topic, output, lang):
    target = mp4(output, lang)
    source = SCENES / series / "🎬️clip" / topic / "🐍️.py"
    return target.exists() and target.stat().st_mtime > source.stat().st_mtime


while True:
    pending = [job for job in JOBS if not fresh(job[0], job[1], job[3], job[4])]
    if not pending:
        print("[DEBUG] all 18 fresh", flush=True)
        break
    if not all((LOGS / f"{job[3]}-{job[4]}.log").exists() for job in JOBS):
        time.sleep(20)
        continue
    lines = alive()
    running = len({line.split()[0] for line in lines}) // 2
    for series, topic, scene, output, lang in pending:
        running_now = [line for line in lines if f" {output} {lang}" in line]
        if running_now or running >= 6:
            continue
        log = LOGS / f"{output}-{lang}.log"
        text = log.read_text(encoding="utf-8", errors="replace") if log.exists() else ""
        if "Traceback" not in text:
            continue
        key = (output, lang)
        tries[key] = tries.get(key, 0) + 1
        if tries[key] > 3:
            continue
        with log.open("w", encoding="utf-8") as handle:
            subprocess.Popen([str(PY), str(TICKET / "render/preview.py"), series, topic, scene, output, lang], cwd=ROOT, stdout=handle, stderr=subprocess.STDOUT,
                             env={**__import__("os").environ, "PYTHONUTF8": "1"})
        print("[DEBUG] rerun", output, lang, "try", tries[key], flush=True)
        running += 1
        lines = alive()
    time.sleep(20)
