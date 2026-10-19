#!/usr/bin/env python3
"""repoint.py <map.json> <files...>: rewrite named import/export statements whose specifier ends with <module>.
map: {"module": "🖼️wire-turn/🟦️.ts", "names": {"name": "child-or-empty"}} -> child dir inserted before /🟦️.ts ("" keeps module)."""
import json, re, sys
cfg = json.load(open(sys.argv[1]))
mod = cfg["module"]
base = mod.rsplit("/", 1)[0]
where = cfg["names"]
STMT = re.compile(r'(?P<kw>import|export)(?P<ty>\s+type)?\s*\{(?P<body>[^}]*)\}\s*from\s*"(?P<pre>[^"]*?)' + re.escape(mod) + r'"\s*;', re.S)
for path in sys.argv[2:]:
    s = open(path, encoding="utf-8").read()
    changed = [0]
    def rep(m):
        groups = {}
        for item in [x.strip() for x in m.group("body").split(",") if x.strip()]:
            bare = re.sub(r"^type\s+", "", item)
            name = bare.split(" as ")[0].strip()
            if name not in where:
                raise SystemExit(f"{path}: unmapped {name}")
            groups.setdefault(where[name], []).append(item)
        out = []
        for child in sorted(groups, key=lambda c: (c != "", c)):
            spec = m.group("pre") + (base + "/" + child + "/🟦️.ts" if child else mod)
            ty = m.group("ty") or ""
            alltype = all(i.startswith("type ") for i in groups[child]) and not ty
            items = groups[child]
            if alltype:
                items = [re.sub(r"^type\s+", "", i) for i in items]
                out.append(f'{m.group("kw")} type {{ {", ".join(items)} }} from "{spec}";')
            else:
                out.append(f'{m.group("kw")}{ty} {{ {", ".join(items)} }} from "{spec}";')
        changed[0] += 1
        return "\n".join(out)
    t = STMT.sub(rep, s)
    if t != s:
        open(path, "w", encoding="utf-8").write(t)
        print("rewrote", path[-70:], changed[0])
