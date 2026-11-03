import re, sys
A = "C:/git/semio/🎓️teaching/🏛️architecture/"
sites = [("🎯️optimization/🎬️clip/⚖️index/🐍️.py", 135), ("🎯️optimization/🎬️clip/🎨️visuals/🏠️room/🐍️.py", 587),
         ("🎯️optimization/🎬️clip/📉️extrema/🐍️.py", 440), ("🎯️optimization/🎬️clip/🧪️experiments/🐍️.py", 275),
         ("🎯️optimization/🎬️clip/🧪️experiments/🐍️.py", 904), ("🏗️bim/🎬️clip/📊️information/🐍️.py", 212),
         ("🏗️bim/🎬️clip/🔌️exchange/🐍️.py", 876), ("🏗️bim/🎬️clip/🔗️consistency/🐍️.py", 273),
         ("🏗️bim/🎬️clip/🔗️consistency/🐍️.py", 1034), ("🏗️bim/🎬️clip/🔗️consistency/🐍️.py", 1078),
         ("🏗️bim/🎬️clip/🔗️consistency/🐍️.py", 1105), ("📊️excel/🎬️clip/📋️program/🐍️.py", 376)]
for f, n in sites:
    lines = open(A + f, encoding="utf-8").read().split("\n")
    cls = [(i + 1, l) for i, l in enumerate(lines[:n]) if re.match(r"class |    def ", l)]
    print(f, n, [c[1].strip()[:60] for c in cls[-2:]])
    print("   class:", [l for i, l in [(i + 1, l) for i, l in enumerate(lines[:n])] if l.startswith("class ")][-1][:80])
