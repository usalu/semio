import json, os, re, sys, glob

ROOT = r"C:\git\semio"
CAT = os.path.join(ROOT, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog")
VIS = os.path.join(ROOT, "🎓️teaching", "🏛️architecture", "⚡️energy", "🏗️authoring", "🎨️visuals", "🧍️staffage", "🔣️.json")
TEXT_KEYS = {"title", "name", "summary", "description", "label", "notes", "parts", "poses", "clips", "text", "caption", "heading"}

files = sys.argv[1:]
problems = []
de_words = {}
strings_seen = []


def is_locale(d):
    return isinstance(d, dict) and ("en" in d or "de" in d)


def walk(node, path, f):
    if is_locale(node):
        keys = set(node.keys())
        if keys - {"en", "de"}:
            pass
        if "en" not in node or "de" not in node:
            problems.append(f"{f}: {path}: locale dict missing {'de' if 'en' in node else 'en'} keys={sorted(keys)}")
        for k in ("en", "de"):
            if k in node and (not isinstance(node[k], str) or not node[k].strip()):
                problems.append(f"{f}: {path}.{k}: empty or non-string")
        if "de" in node and isinstance(node["de"], str):
            for w in re.findall(r"[A-Za-zÄÖÜäöüß\-]+", node["de"]):
                if re.search(r"(ae|oe|ue)", w, re.I):
                    de_words.setdefault(w, set()).add(f)
        if "en" in node and isinstance(node["en"], str):
            strings_seen.append((f, path, node["en"], node.get("de", "")))
        return
    if isinstance(node, dict):
        for k, v in node.items():
            walk(v, f"{path}.{k}", f)
    elif isinstance(node, list):
        for i, v in enumerate(node):
            walk(v, f"{path}[{i}]", f)
    elif isinstance(node, str):
        last = path.split(".")[-1].split("[")[0]
        if last in TEXT_KEYS:
            problems.append(f"{f}: {path}: plain string in text key (no locale): {node[:60]!r}")


for f in files:
    with open(f, encoding="utf-8") as fh:
        data = json.load(fh)
    walk(data, "$", os.path.relpath(f, CAT) if f.startswith(CAT) else os.path.relpath(f, ROOT))

print(f"files={len(files)} locale_pairs={len(strings_seen)}")
print("== problems")
for p in problems:
    print(p)
print(f"problems={len(problems)}")
print("== de words containing ae/oe/ue (review)")
for w in sorted(de_words, key=str.lower):
    print(w, sorted(de_words[w])[:2])
