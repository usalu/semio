import importlib, sys
from pathlib import Path
sys.path.insert(0, "C:/git/semio")
m = importlib.import_module("🎓️teaching.🏛️architecture.📊️excel.🎬️clip.⚡️energy.🐍️")
for k, en, de in m.Beat4_Energy.NARRATION:
    print(k, len(en), len(de))
