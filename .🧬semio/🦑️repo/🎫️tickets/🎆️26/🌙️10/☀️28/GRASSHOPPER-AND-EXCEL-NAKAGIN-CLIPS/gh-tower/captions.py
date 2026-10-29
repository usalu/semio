import importlib, os, sys
sys.path.insert(0, "C:/git/semio")
os.environ["SEMIO_TEACHING_LANGUAGE"] = "de"
art = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
m = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🗼️tower.🐍️")
for lang, idx in (("en", 1), ("de", 2)):
    for row in m.Beat3_Tower.NARRATION:
        try:
            art.caption_bar(row[idx])
        except Exception as e:
            print(lang, row[0], len(row[idx]), str(e)[:60])
