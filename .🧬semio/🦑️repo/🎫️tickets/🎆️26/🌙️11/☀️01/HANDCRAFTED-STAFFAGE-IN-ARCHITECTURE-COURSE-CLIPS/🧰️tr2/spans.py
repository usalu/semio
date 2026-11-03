import sys
sys.path.insert(0, "C:/git/semio")
import importlib

m = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧪️tests.🐍️staffage_only")
manifest = m.manifest_casts()
for path in m.energy_clip_sources():
    for spans in m.scene_spans(path.read_text(encoding="utf-8"), manifest):
        if len(spans) > 1:
            print(m.clip_key(path), spans)
