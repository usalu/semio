import importlib
import os
import sys

os.environ.setdefault("SEMIO_TEACHING_LANGUAGE", "de")
root = next(p for p in __import__("pathlib").Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(root))
A = "🎓️teaching.🏛️architecture."
E = A + "⚡️energy."
modules = [E + f"🔥️heating.🎬️clip.{c}.🐍️" for c in ("🌬️convection", "👥️internal-gains", "☀️solar-gains", "🧱️conduction", "🧮️calculation", "🧲️fundamentals")]
modules += [E + f"❄️cooling.🎬️clip.{c}.🐍️" for c in ("📐️system-sizing", "💧️transmission-humidity", "👥️internal-gains", "☀️solar-radiation", "🌤️summer-strategy")]
modules += [E + "🧲️physics.🎬️clip.🧲️fundamentals.🐍️", E + "🏗️authoring.🎨️visuals.🐍️"]
modules += [A + f"🎯️optimization.🎬️clip.{c}.🐍️" for c in ("⚖️index", "🏆️pareto", "📉️extrema", "🧪️experiments", "🧬️algorithms")]
modules += [A + "🏗️bim.🎬️clip.🧩️models.🐍️"]
failed = 0
for name in modules:
    try:
        importlib.import_module(name)
        print("ok  ", name)
    except Exception as error:
        failed += 1
        print("FAIL", name, type(error).__name__, error)
sys.exit(1 if failed else 0)
