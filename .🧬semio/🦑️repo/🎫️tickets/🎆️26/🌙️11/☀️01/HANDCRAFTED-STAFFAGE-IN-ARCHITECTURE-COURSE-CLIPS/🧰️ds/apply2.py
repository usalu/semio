import ast, glob, os, sys
sys.path.insert(0, os.path.dirname(__file__))
from scan_py import scan, emoji_lead

BASE = "🎓️teaching/🏛️architecture/"
SPEC = [
    ("⚡️energy/❄️cooling/🎬️clip/👥️internal-gains/🐍️.py", "_office_furniture", "🖥️", ["🪑", "🏢"]),
    ("⚡️energy/🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py", "_mvhr", "🌀", ["♻️", "🔁"]),
    ("⚡️energy/🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py", "_spin", "🔄", ["▶️", "⏩"]),
    ("⚡️energy/🌬️ventilation/🎬️clip/📦️volume/🐍️.py", "_use_cast", "🧍", ["🎭", "🛋️"]),
    ("⚡️energy/🏗️authoring/🎨️visuals/🐍️.py", "fitted_staffage", "📐", ["🧑‍🤝‍🧑", "🪆", "🙋"]),
    ("⚡️energy/🏗️authoring/🎨️visuals/🐍️.py", "thermometer_glyph", "🌡️", ["📈", "🧪", "🩺"]),
    ("⚡️energy/📊️demand/🎬️clip/🏭️primary-energy/🐍️.py", "_boiler", "🔥", ["⛽", "🛢️", "🚿"]),
    ("⚡️energy/📊️demand/🎬️clip/🏷️certificate/🐍️.py", "_heater", "♨️", ["🧱", "🔆", "🛋️"]),
    ("⚡️energy/📊️demand/🎬️clip/🔁️recap/🐍️.py", "_metre", "📏", ["🧮", "📐", "🪜"]),
    ("⚡️energy/🔥️heating/🎬️clip/☀️solar-gains/🐍️.py", "_first_slat_hit", "🎯", ["🔍", "🔎"]),
    ("⚡️energy/🔥️heating/🎬️clip/☀️solar-gains/🐍️.py", "_blind_slats", "🪟", ["🎚️", "🪞", "📚"]),
    ("⚡️energy/🔥️heating/🎬️clip/👥️internal-gains/🐍️.py", "_house_gains", "💡", ["👪", "🏘️"]),
    ("⚡️energy/🔥️heating/🎬️clip/🧮️calculation/🐍️.py", "_plant_house", "🏠", ["🏡", "🏚️"]),
    ("⚡️energy/🔥️heating/🎬️clip/🧱️conduction/🐍️.py", "_ground_line", "📏", ["🟫", "🌍"]),
    ("⚡️energy/🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py", "_radiator", "♨️", ["🔲", "🛋️", "🧱"]),
    ("⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py", "_vacuum", "🧹", ["🌪️", "🧼", "🧽"]),
    ("🎯️optimization/🎬️clip/🎨️visuals/🏠️room/🐍️.py", "pendant", "💡", ["🪔", "🏮"]),
    ("🎯️optimization/🎬️clip/🎨️visuals/🏠️room/🐍️.py", "pendant_lit", "💡", ["🔆", "✨"]),
    ("🏗️bim/🎬️clip/🔗️consistency/🐍️.py", "person", "🧑", ["🚶", "🕴️"]),
    ("📊️excel/🎬️clip/📋️program/🐍️.py", "occupant", "🧍", ["🙋", "🚶"]),
    ("⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🐍️staffage_only.py", "scene_spans", "🌡️", ["🔭", "🧭", "📏"]),
]

def find(root_rel):
    return BASE + root_rel

for rel, fn, old, cands in SPEC:
    p = find(rel)
    raw = open(p, "rb").read().decode("utf-8")
    text = raw.replace("\r\n", "\n")
    tree = ast.parse(text)
    node = next((n for n in ast.walk(tree) if isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)) and n.name == fn), None)
    if node is None:
        print("MISSING", rel, fn); continue
    lead_all = set()
    for n in ast.walk(tree):
        if isinstance(n, (ast.Module, ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            d = ast.get_docstring(n, clean=False)
            if d is not None:
                lead_all.add(emoji_lead(d))
    ln = node.body[0].lineno
    lines = text.split("\n")
    line = lines[ln - 1]
    if emoji_lead(ast.get_docstring(node, clean=False) or "") != old:
        print("SKIP changed", rel, fn); continue
    new = next((c for c in cands if c not in lead_all), None)
    if new is None:
        print("NOCAND", rel, fn); continue
    if old not in line:
        print("NOLINE", rel, fn, line[:60]); continue
    lines[ln - 1] = line.replace(old, new, 1)
    out = "\n".join(lines)
    if "\r\n" in raw:
        out = out.replace("\n", "\r\n")
    ast.parse(out.replace("\r\n", "\n"))
    tmp = p + ".dstmp"
    open(tmp, "wb").write(out.encode("utf-8"))
    os.replace(tmp, p)
    print("OK", rel.split("/")[-2], fn, old, "->", new)
