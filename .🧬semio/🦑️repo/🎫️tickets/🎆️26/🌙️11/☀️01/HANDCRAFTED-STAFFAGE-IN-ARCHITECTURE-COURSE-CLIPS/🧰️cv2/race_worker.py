import importlib
import sys
import time

S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
ids = sys.argv[1].split(",")
start = time.time()
try:
    folders = S.ensure_casts(ids)
    print("OK", {cast: (folder / "🔣️.json").is_file() and len(list(folder.glob("*.svg"))) for cast, folder in folders.items()}, round(time.time() - start, 1), "s")
except BaseException as error:
    print("ERROR", repr(error)[:600])
    raise SystemExit(1)
