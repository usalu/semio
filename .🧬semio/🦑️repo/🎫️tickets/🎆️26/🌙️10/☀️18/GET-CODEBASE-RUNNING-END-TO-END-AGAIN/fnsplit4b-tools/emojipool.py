"""🎲 Unused doc emoji picker. usage (import): pick(text, n) -> n emoji (with U+FE0F) that start no `///`/`//!` doc line of text."""
import re
POOL = "🧱🪵🔩⚙🧲🪛🔧🔨🪚🧰🧪🧫🧬🔬🔭📡🛰🧭🗺🗝🔑🧩🎯🎲🧮📐📏📎📌📍🔖🏷🗂🗃🗄📦📥📤📬🔔🔕🔒🔓🛡⚖⛓🪝🪢🧵🧶🪡🎛🎚🎬🎞🎥📷📸🔍🔎💡🔦🕯🪔⏱⏲⏳⌛🔋🔌🧯🚦🚧🏗🏭🏛🏟🏰🌱🌿🍃🌳🌲🪨🌊🔥💧❄🌀🌈☀🌙⭐✨🪐🚀🛸🛠⛏🪓🧹🧺🧴🪣🧼🛁🪒🧷🧾📜📃📄📑📊📈📉📋📒📓📔📕📖📗📘📙📚🔗🧿🪬🎁🎀🎈🎉🎊🏁🚩🏳🏴🏆🥇🎖🏅🔰♻🔱⚜🔶🔷🔸🔹🔺🔻💠🔘🔲🔳▪▫◾◽🟥🟧🟨🟩🟦🟪🟫⬛⬜".replace("️", "")
def used(text):
    return {m.replace("️", "") for m in re.findall(r"(?m)^\s*(?:///|//!|/\*\*)\s*(\S+)", text)}
def pick(text, n, taken=()):
    u = used(text) | {t.replace("️", "") for t in taken}
    out = []
    for ch in POOL:
        if ch not in u and ch not in out:
            out.append(ch)
            if len(out) == n: break
    return [c + "️" for c in out]
