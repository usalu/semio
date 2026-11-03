import os, re

B = "🎓️teaching/🏛️architecture/🧍️staffage/"
VS = "️"
M = B + "🔨️modules/"
SPEC = [
    (M + "🌡️thermal/⏳️transient/🟦️.ts", "relax", ["📉", "🪫"]),
    (M + "🌡️thermal/⏳️transient/🟦️.ts", "historyPlan", ["🗓", "📜"]),
    (M + "🌡️thermal/🟦️.ts", "solveThermal", ["♨", "🔥"]),
    (M + "🌡️thermal/🟦️.ts", "operativeTemperatureC", ["🥵", "🛋"]),
    (M + "🌡️thermal/🟦️.ts", "uValue", ["🪟", "🔲"]),
    (M + "🌡️thermal/🧱️balance/🟦️.ts", "regionSetup", ["🧩", "🪛"]),
    (M + "🌡️thermal/🧱️balance/🟦️.ts", "ambientOf", ["🌤", "🌦"]),
    (M + "🌡️thermal/🧱️balance/🟦️.ts", "exchange", ["🔁", "🔄"]),
    (M + "🌡️thermal/🧱️balance/🟦️.ts", "outflow", ["📤", "🚪"]),
    (M + "🌡️thermal/🧱️balance/🟦️.ts", "solarOf", ["☀", "🌞"]),
    (M + "🌡️thermal/🧱️balance/🟦️.ts", "envOf", ["🌍", "🌐"]),
    (M + "🌡️thermal/🧱️balance/🟦️.ts", "stomataOf", ["🌿", "🍃"]),
    (M + "🌡️thermal/🧱️balance/🟦️.ts", "inflowOf", ["📥", "🛬"]),
    (M + "🌡️thermal/🧱️balance/🟦️.ts", "solveSteady", ["🎯", "🏁"]),
    (M + "🌡️thermal/🧱️balance/🟦️.ts", "residualWM2", ["⚖", "🧾"]),
    (M + "🌡️thermal/🧱️balance/🟦️.ts", "restoringConductance", ["🪢", "🔗"]),
    (M + "🌡️thermal/🧱️balance/🟦️.ts", "radiationSplit", ["🔆", "💡"]),
    (M + "🌡️thermal/🧱️balance/🟦️.ts", "emittedOf", ["🔥", "♨"]),
    (M + "🎞️animation/🟦️.ts", "frameAt", ["🖼", "🎴"]),
    (M + "🎞️animation/🟦️.ts", "clipOf", ["🎥", "🎦"]),
    (M + "🎞️animation/🟦️.ts", "contactsAt", ["👣", "🥾"]),
    (M + "🎞️animation/🟦️.ts", "sampleClip", ["📽", "🎦"]),
    (M + "🎨️depiction/🌡️thermal-view/🟦️.ts", "thermalView", ["🟥", "🌈"]),
    (M + "🎨️depiction/💨️transport-view/🟦️.ts", "transportView", ["🌬", "🌀"]),
    (M + "🎨️depiction/🖍️palette/🟦️.ts", "legendOf", ["🗝", "📖"]),
    (M + "🎨️depiction/🟦️.ts", "depict", ["🖌", "✏"]),
    (M + "💨️transport/🔥️sources/🟦️.ts", "emissionsAt", ["🫧", "🌫"]),
    (M + "💨️transport/🔥️sources/🟦️.ts", "emitterStates", ["🚦", "🔌"]),
    (M + "💨️transport/🟦️.ts", "solveTransport", ["🚰", "🌬"]),
    (M + "📤️export/🖼️svg/🟦️.ts", "writeSvg", ["✍", "📝"]),
    (M + "🖨️representation/📐️measure/🟦️.ts", "squaredDistanceTransform", ["🔢", "🗺"]),
    (M + "🖨️representation/📐️measure/🟦️.ts", "whiteSliver", ["🥖", "🧵"]),
    (M + "🖨️representation/🟦️.ts", "resolveRepresentation", ["🧾", "✅"]),
    (M + "🖨️representation/🪞️copies/🟦️.ts", "compareRepresentations", ["🔬", "🆚"]),
    (M + "🧮️evaluation/🟦️.ts", "evaluate", ["▶", "🧠"]),
    (B + "🧪️tests/🧰️support/🧰️.ts", "toolBytes", ["🔩", "🛠"]),
]

def norm(e):
    return e.replace(VS, "")

def lead(s):
    out = ""
    for ch in s.strip():
        if ord(ch) > 0x2000 and not ch.isalnum():
            out += ch
        else:
            break
    return out

for rel, name, cands in SPEC:
    raw = open(rel, "rb").read().decode("utf-8")
    text = raw.replace("\r\n", "\n")
    decl = re.search(r"^(export\s+)?(async\s+)?(function|const)\s+" + re.escape(name) + r"\b", text, re.M)
    if not decl:
        print("NODECL", rel.split("/")[-2], name); continue
    start = text.rfind("/**", 0, decl.start())
    end = text.find("*/", start)
    if start < 0 or text[end + 2:decl.start()].strip():
        print("NODOC", rel.split("/")[-2], name); continue
    block = text[start:end]
    m = re.search(r"/\*\*\s*(?:\n\s*\*\s*)?", block)
    pre = block[:m.end()]
    rest = block[m.end():]
    old = lead(rest)
    used = set()
    for dm in re.finditer(r"/\*\*(.*?)\*/", text, re.S):
        f = next((re.sub(r"^\s*\*\s?", "", l) for l in dm.group(1).splitlines() if re.sub(r"^\s*\*?\s*", "", l).strip()), "")
        used.add(norm(lead(f)))
    if not old:
        print("NOLEAD", rel.split("/")[-2], name); continue
    cnt = sum(1 for x in [norm(old)] if x in used)
    new = next((c for c in cands if c not in used), None)
    if new is None:
        print("NOCAND", name); continue
    newblock = pre + new + VS + rest[len(old):]
    out = text[:start] + newblock + text[end:]
    if "\r\n" in raw:
        out = out.replace("\n", "\r\n")
    tmp = rel + ".dstmp"
    open(tmp, "wb").write(out.encode("utf-8"))
    os.replace(tmp, rel)
    print("OK", rel.split("/")[-2], name, old, "->", new)
