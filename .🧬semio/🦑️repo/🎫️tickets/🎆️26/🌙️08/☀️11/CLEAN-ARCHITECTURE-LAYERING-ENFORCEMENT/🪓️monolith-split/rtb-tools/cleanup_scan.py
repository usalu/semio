"""Lists the cfg(test)-gated top-level statements (items, uses, mounts, includes, consts) of the monolith files that only served the moved tests."""
import re, sys
from lib import *
FILES = {
    "shell parent": SH + "/🦀️.rs",
    "engine_canvas parent": EL + "/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs",
    "scenes parent": EL + "/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs",
    "interpreter parent": EL + "/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs",
    "renderer root": R + "/🦀️.rs",
    "hub wp1": R + "/📥️mounts/🌱️wp1-foundation/🦀️.rs",
    "hub wp2": R + "/📥️mounts/🎞️wp2-scenes/🦀️.rs",
    "hub wp3": R + "/📥️mounts/🧵️wp3-frame-core/🦀️.rs",
    "hub wp4": R + "/📥️mounts/🐚️wp4-shell-state/🦀️.rs",
    "hub wp5": R + "/📥️mounts/🎛️wp5-shell-views/🦀️.rs",
    "lib adapter": T + "/📦️packages/🦀️rust/📚️library/🦀️.rs",
    "docsurface": EL + "/🏛️ShellHost/🔀️surface-switch/📄️document/🦀️.rs",
}
GATE = re.compile(r"#\[cfg\((?:test|all\(test[^\n]*|all\([^\n]*\btest\b[^\n]*|any\(test[^\n]*)\)\]")
def statements(text):
    """(start line idx, end line idx, head text) of top-level statements preceded by a cfg(test)-like attribute line."""
    lines = text.split("\n")
    clean = blank(text).split("\n")
    out = []
    i = 0
    depth = 0
    n = len(lines)
    while i < n:
        l = lines[i]
        if depth == 0 and l.startswith("#[cfg(") and GATE.fullmatch(l.strip()) and "feature" not in l:
            j = i
            while j + 1 < n and lines[j + 1].startswith("#["):
                j += 1
            # statement body
            k = j + 1
            d = 0
            seen = False
            while k < n:
                d += clean[k].count("{") - clean[k].count("}")
                seen = seen or "{" in clean[k]
                if (seen and d <= 0) or (not seen and clean[k].rstrip().endswith(";")):
                    break
                k += 1
            out.append((i, k, " ".join(x.strip() for x in lines[i:min(k + 1, i + 3)])[:150]))
            i = k + 1
            continue
        depth += clean[i].count("{") - clean[i].count("}")
        i += 1
    return out
if __name__ == "__main__":
    for name, f in FILES.items():
        t = read(f)
        st = statements(t)
        print(f"== {name}: {len(st)} cfg(test) statements")
        for a, b, h in st:
            print(f"   {a+1}-{b+1} {h}")
