import importlib, json, os
from pathlib import Path

os.environ.setdefault("SEMIO_TEACHING_LANGUAGE", "en")
B = "🎓️teaching.🏛️architecture.⚡️energy."
shared = importlib.import_module(B + "🏗️authoring.🎨️visuals.🐍️")
from manim import VGroup

path = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🏗️authoring\🎨️visuals\🧪️tests\🔣️integration.json")
text = path.read_text(encoding="utf-8")
cases = json.loads(text)["captions"]
bad = []
for case in cases:
    module = importlib.import_module("🎓️teaching." + case["script"].removesuffix(".py").replace("/", "."))
    row = next((r for r in getattr(module, case["scene"]).NARRATION if r[0] == case["key"]), None)
    tag = (case["scene"] + ":" + case["key"]).encode("ascii", "backslashreplace").decode()
    if row is None:
        bad.append(("missing", tag))
        continue
    for index, language in ((1, "en"), (2, "de")):
        if row[index] == case[language]:
            continue
        try:
            label = shared.caption_bar(row[index])[1]
            lines = len(label) if isinstance(label, VGroup) else 1
        except ValueError:
            bad.append(("too long", tag, language))
            continue
        if lines > case["maximum_lines"]:
            bad.append(("lines", tag, language, lines))
            continue
        old, new = json.dumps(case[language], ensure_ascii=False), json.dumps(row[index], ensure_ascii=False)
        assert text.count(old) == 1, tag
        text = text.replace(old, new)
temporary = path.with_name(path.name + ".tmp")
temporary.write_text(text, encoding="utf-8", newline="")
os.replace(temporary, path)
print("bad", bad)
