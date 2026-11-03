import importlib
import importlib.util
import io
import os
import sys
import traceback

ROOT = r"C:\git\semio"
sys.path.insert(0, ROOT)
ARCH = os.path.join(ROOT, "🎓️teaching", "🏛️architecture")
CLIPS = [
    os.path.join("⚡️energy", "❄️cooling", "🎬️clip", "☀️solar-radiation", "🐍️.py"),
    os.path.join("⚡️energy", "❄️cooling", "🎬️clip", "🌤️summer-strategy", "🐍️.py"),
    os.path.join("⚡️energy", "🌬️ventilation", "🎬️clip", "🪟️natural", "🐍️.py"),
    os.path.join("⚡️energy", "🌬️ventilation", "🎬️clip", "📦️volume", "🐍️.py"),
    os.path.join("⚡️energy", "🔥️heating", "🎬️clip", "☀️solar-gains", "🐍️.py"),
    os.path.join("⚡️energy", "🧲️physics", "🎬️clip", "🧲️fundamentals", "🐍️.py"),
    os.path.join("⚡️energy", "🏗️authoring", "🎬️clip", "🪟️presentation", "🐍️.py"),
]


def main():
    failures = 0
    for rel in CLIPS:
        path = os.path.join(ARCH, rel)
        name = "hs_probe_" + str(abs(hash(rel)))
        spec = importlib.util.spec_from_file_location(name, path)
        module = importlib.util.module_from_spec(spec)
        try:
            spec.loader.exec_module(module)
            print("IMPORT OK", rel)
        except BaseException:
            failures += 1
            print("IMPORT FAIL", rel)
            print(traceback.format_exc()[-600:])
    print("failures", failures)
    return failures


if __name__ == "__main__":
    sys.exit(main())
