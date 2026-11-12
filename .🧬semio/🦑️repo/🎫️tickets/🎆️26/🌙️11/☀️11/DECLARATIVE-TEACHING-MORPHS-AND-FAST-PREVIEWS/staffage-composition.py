"""Path composition of the first frame of cached casts: group ids (shape-*, hatch, outline, transport...) and bytes outside paths."""
import collections, importlib, re, sys
S = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🧍️staffage.🐍️")
for name in sys.argv[1:]:
    entry = S._manifest()["casts"][name]
    folder = S._cache_root() / S.cast_key(entry) / name
    index = S._index(folder)
    text = (folder / index["frames"][0]).read_text(encoding="utf-8")
    ids = collections.Counter(re.sub(r"[-:][a-z0-9]+$", "", g)[:24] for g in re.findall(r'<g id="([^"]+)"', text))
    meta = text.find("</metadata>") - text.find("<metadata>")
    print(name, "bytes", len(text), "metadata", meta, "paths", text.count("<path"), "groups", dict(ids.most_common(8)))
