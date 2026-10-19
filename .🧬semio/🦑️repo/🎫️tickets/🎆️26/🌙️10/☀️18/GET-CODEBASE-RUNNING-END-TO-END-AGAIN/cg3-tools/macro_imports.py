import sys, os, re
sys.path.insert(0, os.path.dirname(__file__))
import macro_sites as m
PREFIX = "crate::standards::v1::subsets::any::schema::mutations"
apply = "--apply" in sys.argv
ROOT = m.E + "/🦀️.rs"
root = open(ROOT, encoding="utf8").read()
fixed = root.replace(f"{PREFIX}::$module::$leaf", "$module::$leaf")
print("root macro occurrences replaced:", root.count(f"{PREFIX}::$module::$leaf"))
if apply and fixed != root: open(ROOT, "w", encoding="utf8").write(fixed)
ITEM = re.compile(r"^(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:fn|struct|enum|const|static|type|macro_rules!|impl|mod|trait|#\[)")
for dirpath, _, names in os.walk(m.E):
    for n in names:
        if n != "🦀️.rs": continue
        p = os.path.join(dirpath, n)
        text = open(p, encoding="utf8").read()
        mods = sorted({s[1] for s in m.sites(text)})
        if not mods: continue
        have = set()
        for stmt in re.finditer(r"^(?:pub\s+)?use\s[^;]*;", text, re.M):
            for mod in mods:
                if re.search(rf"(?:::|[{{,]\s*){mod}\s*(?:[,}};]|as\s)", stmt.group(0)): have.add(mod)
        local = {mod for mod in mods if re.search(rf"^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+{mod}\b", text, re.M)}
        add = [mod for mod in mods if mod not in have and mod not in local]
        print(p[len(m.E):], "sites", len(mods), "add", len(add), "have", sorted(have), "local", sorted(local))
        if not add: continue
        line = f"use {PREFIX}::{add[0]};" if len(add) == 1 else f"use {PREFIX}::{{{', '.join(add)}}};"
        lines = text.split("\n")
        first_item = next((i for i, l in enumerate(lines) if ITEM.match(l) and not l.startswith("#![")), len(lines))
        last_use = -1; i = 0
        while i < first_item:
            l = lines[i]
            if re.match(r"^(?:pub\s+)?use\s", l):
                j = i
                while not lines[j].rstrip().endswith(";"): j += 1
                last_use = j; i = j
            i += 1
        if last_use < 0:
            last_use = max((i for i, l in enumerate(lines[:first_item]) if l.startswith("//!")), default=-1)
            lines[last_use + 1:last_use + 1] = ["", line] if last_use >= 0 else [line]
        else:
            lines.insert(last_use + 1, line)
        if apply: open(p, "w", encoding="utf8").write("\n".join(lines))
