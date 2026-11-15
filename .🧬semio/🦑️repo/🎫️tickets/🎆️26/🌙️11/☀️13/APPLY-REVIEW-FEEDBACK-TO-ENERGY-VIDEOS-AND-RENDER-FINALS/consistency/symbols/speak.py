"""🗣️ Probe how the lexicon speaks given TeX (one expression per line in the file argument)."""
import importlib
import sys

sys.path.insert(0, r"C:\git\semio")
LEX = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔊️speech.🗣️lexicon.🐍️")
sys.stdout.reconfigure(encoding="utf-8")
with open(sys.argv[1], encoding="utf-8") as h:
    for tex in [x for x in h.read().split("\n") if x]:
        for lang in ("de", "en"):
            try:
                print(lang, tex, "=>", LEX.spoken_math(tex, lang))
            except Exception as e:
                print(lang, tex, "!!", e)
