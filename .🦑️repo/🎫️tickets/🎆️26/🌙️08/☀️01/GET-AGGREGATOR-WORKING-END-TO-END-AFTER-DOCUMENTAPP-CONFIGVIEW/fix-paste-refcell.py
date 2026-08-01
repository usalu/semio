from pathlib import Path
import re

CFG = (
    "_cfg: &semio_framework_plugin::ConfigView<"
    + "'_, semio_framework_plugin::NoConfig>"
)

base = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugin")
puzzle = [x for x in base.iterdir() if "puzzle" in x.name][0]


def fix_paste(text: str):
    m = re.search(r"fn paste_operations\s*\(", text)
    if not m:
        return text, False
    i = m.end() - 1
    depth = 0
    j = i
    while j < len(text):
        if text[j] == "(":
            depth += 1
        elif text[j] == ")":
            depth -= 1
            if depth == 0:
                j += 1
                break
        j += 1
    params = text[i + 1 : j - 1]
    new = re.sub(r"^(\s*)&mut\s+self", r"\1&self", params)
    if "DocumentView" in new and "ConfigView" not in new:
        mm = re.search(r"DocumentView\s*<", new)
        angle = new.find("<", mm.start())
        depth = 0
        k = angle
        while k < len(new):
            if new[k] == "<":
                depth += 1
            elif new[k] == ">":
                depth -= 1
                if depth == 0:
                    k += 1
                    break
            k += 1
        comma = new.find(",", k)
        if comma < 0:
            new = new[:k].rstrip() + ", " + CFG + new[k:]
        else:
            new = new[: comma + 1] + " " + CFG + "," + new[comma + 1 :]
    if new == params:
        return text, False
    return text[: i + 1] + new + text[j - 1 :], True


for label in ("2d", "5d"):
    app = [x for x in (puzzle / "🎛️app").iterdir() if label in x.name][0]
    path = app / "🔨️module" / "🖱️ui" / "⚡️implementation" / "🦀️rust" / "📦️lib.rs"
    t = path.read_text()
    if "use std::cell::RefCell" not in t:
        if "use std::collections::" in t:
            t = t.replace("use std::collections::", "use std::cell::RefCell;\nuse std::collections::", 1)
        else:
            t = "use std::cell::RefCell;\n" + t
        print(label, "added RefCell import")
    t, changed = fix_paste(t)
    print(label, "paste", changed)
    path.write_text(t)
