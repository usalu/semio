from pathlib import Path
import re

def app_lib(label):
    base = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugin")
    puzzle = [x for x in base.iterdir() if "puzzle" in x.name][0]
    app = [x for x in (puzzle / "🎛️app").iterdir() if label in x.name][0]
    return app / "🔨️module" / "🖱️ui" / "⚡️implementation" / "🦀️rust" / "📦️lib.rs"

# --- 5d ---
p5 = app_lib("5d")
t = p5.read_text()
if "use std::cell::RefCell;\n" not in t[:900]:
    t = t.replace(
        "use std::collections::{BTreeMap, HashMap, HashSet};",
        "use std::cell::RefCell;\nuse std::collections::{BTreeMap, HashMap, HashSet};",
        1,
    )
    print("5d: added import")

cfg_snip = "_cfg: &semio_framework_plugin::ConfigView<" + "'_, semio_framework_plugin::NoConfig>, "
needle = "fn paste_operations(&self, doc: &DocumentView<'_, Puzzle5dPlayProjection>, " + cfg_snip
if needle in t:
    t = t.replace(needle, "fn paste_operations(&self, doc: &DocumentView<'_, Puzzle5dPlayProjection>, ", 1)
    print("5d: removed paste ConfigView")
else:
    # broader
    m = re.search(r"fn paste_operations\([^)]*\)", t)
    print("5d paste now:", m.group(0)[:220] if m else "MISSING")
p5.write_text(t)

# --- 2d host borrows ---
p2 = app_lib("2d")
t = p2.read_text()
fixes = 0
out = []
for line in t.splitlines(True):
    orig = line
    if "&self.host" in line:
        # only replace occurrences not already followed by .borrow
        def repl(m):
            rest = line[m.end() : m.end() + 8]
            if rest.startswith(".borrow"):
                return m.group(0)
            return "&self.host.borrow()"
        line = re.sub(r"&self\.host", repl, line)
        if line != orig:
            fixes += 1
            print("2d:", orig.strip()[:140])
    out.append(line)
p2.write_text("".join(out))
print("2d host fixes", fixes)

# leftover host without borrow (excluding field defs)
for i, l in enumerate(p2.read_text().splitlines(), 1):
    if "self.host" not in l:
        continue
    if "borrow" in l or "RefCell" in l or "host:" in l:
        continue
    if "struct " in l:
        continue
    print(f"2d leftover {i}:{l[:140]}")
