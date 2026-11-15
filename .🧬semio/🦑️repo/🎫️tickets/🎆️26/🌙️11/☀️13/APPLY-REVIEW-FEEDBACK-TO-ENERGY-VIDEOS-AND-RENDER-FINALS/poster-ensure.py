import importlib
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")
root = Path(__file__).resolve().parents[7]
sys.path.insert(0, str(root))
posters = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🖼️posters.🐍️")
settings = {"language": sys.argv[1], "profile": sys.argv[2], "resolution": None, "fps": None, "audience": "speaker"}
print(posters.directory(settings))
print(posters.ensure(sys.argv[3:], settings, posters.directory(settings), jobs=2, log_output=True))
