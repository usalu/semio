"""Gives every duplicate docstring emoji of the changed .ts/.tsx files an unused, fitting one: candidates by keyword of the first words, then a pool; keeps the file's style (emoji + U+FE0F)."""
import re, subprocess
root = "/Users/ueli/Documents/semio"
out = subprocess.run(["git", "-C", root, "-c", "core.quotePath=false", "status", "--short", "-uall", "--", "🎓️teaching/🏛️architecture/🧍️staffage", "🧰️framework/🔨️modules/🧊️3d"], capture_output=True, text=True).stdout
files = [l[3:].strip().strip('"') for l in out.splitlines() if l[3:].strip().strip('"').endswith((".ts", ".tsx")) and "node_modules" not in l and " -> " not in l]
KEYS = [
 (("error", "refus", "throw", "fail"), "🚨🛑⛔🚧"), (("progress", "cancel", "abort", "signal"), "⏳⏹🛑🔔"), (("test", "oracle", "vector"), "🧪🔬🧫⚗"),
 (("angle", "rotat", "turn", "twist"), "🔄🧭🌀🎡"), (("scale", "denominator", "millimetre", "metre"), "📏📐🔭🪜"), (("colour", "color", "tone", "paint", "style"), "🎨🖌🖍🌈"),
 (("view", "viewpoint", "camera", "projection"), "🔭👁📷🎥"), (("file", "folder", "path", "directory", "name"), "🗂📁📂🗃"), (("name", "label", "title", "caption", "text", "language"), "🏷🔤📝💬"),
 (("matrix", "frame", "transform", "basis"), "🧮🧱🗺📊"), (("solid", "mesh", "triangle", "tessell", "volume"), "🧊🔺🧱🪨"), (("export", "write", "encode", "serialis"), "📤📦💾🖨"),
 (("read", "load", "import", "parse"), "📥📖🔎🧲"), (("time", "clock", "second", "duration"), "⏱🕰⌚📆"), (("heat", "thermal", "temperature", "warm"), "🌡🔥♨🧯"),
 (("force", "load", "moment", "structur"), "🏗🏹⚖🪝"), (("bone", "rig", "skeleton", "joint"), "🦴🦿🤸🕸"), (("type", "schema", "contract"), "🧬📜📋🧾"),
 (("cache", "memo", "remember"), "🧠💭🗄🧷"), (("check", "valid", "verify"), "✅🔍🩺🛡"), (("event", "click", "pointer", "key"), "🖱⌨🎹🕹"), (("page", "panel", "button", "dialog"), "🪟🔘🧭🎚"),
]
POOL = list("🌟💡🔧🔩🪛🧰🔑🗝🔒🔓📌📍🧷🧵🪡🎯🏁🚩🏳🎪🎠🎢🎡🧩🎲🃏🀄🎴🧿🪬🪄🔮🛠⚒⛏🪓🔨🧲🪝🧯🔦🕯💎📎🖇📏🧹🧺🪣🧴🧼🪥🪒🔗⛓🧬🦠🍀🌿🌱🌳🍃🌊🌀🌈☀⭐✨⚡💧🔆🌙")
def fe(e): return e + "️" if ord(e) <= 0xFFFF else e + "️"
pat = re.compile(r"^(\s*)/\*\*\s*((?:[^\x00- ][️‍]?)+)", re.M)
changed = 0
for f in files:
    path = f"{root}/{f}"
    try: text = open(path, encoding="utf8").read()
    except FileNotFoundError: continue
    ms = list(pat.finditer(text))
    keyof = lambda tok: re.sub("[️‍]", "", tok)
    used = set(); dups = []
    for m in ms:
        k = keyof(m.group(2))
        if k in used: dups.append(m)
        else: used.add(k)
    if not dups: continue
    edits = []
    for m in dups:
        words = text[m.end(): m.end() + 140].lower()
        choice = None
        for keys, cands in KEYS:
            if any(w in words for w in keys):
                for c in cands:
                    if c not in used: choice = c; break
            if choice: break
        if choice is None:
            for c in POOL:
                if c not in used: choice = c; break
        used.add(choice)
        edits.append((m.start(2), m.end(2), fe(choice)))
        changed += 1
    for s, e, new in sorted(edits, reverse=True): text = text[:s] + new + text[e:]
    open(path, "w", encoding="utf8").write(text)
print("replaced", changed)
