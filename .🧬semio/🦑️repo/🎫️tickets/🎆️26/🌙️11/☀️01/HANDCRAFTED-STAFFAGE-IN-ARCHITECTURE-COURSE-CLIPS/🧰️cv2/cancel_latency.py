import importlib
import json
import signal
import subprocess
import sys
import threading
import time
from pathlib import Path

S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
T = Path(__file__).resolve().parent.parent
out = T / "🗑️generated/cv2/cancel"
out.mkdir(parents=True, exist_ok=True)


def latency(cast: str, wait_frames: float) -> tuple[float, str]:
    manifest = out / "manifest.json"
    manifest.write_text(json.dumps({"version": 1, "casts": {cast: S._entry(cast)}}, ensure_ascii=False), encoding="utf-8")
    command = [S._repo(), "tools", "exec", "--light", "bun", "--", str(S._EXPORTER), "--manifest", str(manifest), "--ids", cast, "--out", str(out / "out")]
    tree = S._Tree(command)
    seen = threading.Event()
    lines = []

    def pump():
        for line in tree.process.stderr:
            lines.append(line.strip())
            if line.startswith("[cast "):
                seen.set()

    reader = threading.Thread(target=pump, daemon=True)
    reader.start()
    seen.wait(120)
    time.sleep(wait_frames)
    began = time.time()
    tree.process.send_signal(signal.CTRL_BREAK_EVENT)
    try:
        tree.process.wait(timeout=60)
        outcome = f"exit {tree.process.returncode}"
    except subprocess.TimeoutExpired:
        outcome = "still running after 60 s"
    elapsed = time.time() - began
    tree.kill()
    tree.close()
    reader.join(5)
    return elapsed, outcome + " " + (lines[-1] if lines else "")


for cast, hold in (("physics-fundamentals-kettle", 0.0), ("physics-fundamentals-kettle", 0.3), ("physics-fundamentals-vacuum", 0.0), ("ventilation-volume-cook", 0.0)):
    elapsed, outcome = latency(cast, hold)
    print(f"{cast}: CTRL_BREAK to exit {elapsed:.2f} s ({outcome})")
