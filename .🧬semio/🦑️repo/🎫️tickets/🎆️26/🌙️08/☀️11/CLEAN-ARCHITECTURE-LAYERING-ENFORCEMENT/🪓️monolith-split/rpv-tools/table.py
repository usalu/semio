"""📊 RPV attribution table: a1 (before, all rows) joined with a2 (state when rpv started editing) and the current definition text.
Writes markdown to stdout. usage: table.py <a1-attr.json> <a2-attr.json>"""
import json, re, sys, collections
a1 = json.load(open(sys.argv[1], encoding="utf-8"))
a2 = json.load(open(sys.argv[2], encoding="utf-8"))
S = lambda s: (s or "?").replace("semio_framework_os_renderer_wgpu", "").lstrip("_") or "top"
still = {(m["owner"], m["member"]) for m in a2["members"] if m["owner"]}
rows = [m for m in a1["members"] if m["owner"]]
by_owner = collections.OrderedDict()
for m in sorted(rows, key=lambda m: (-(m["lib"] + m["test"]), m["owner"], m["member"])):
    by_owner.setdefault(m["owner"], []).append(m)
tot = collections.Counter()
for r in a1["rows"]:
    if r["owner"]:
        tot[(r["owner"], r["cfg"])] += 1
out = []
out.append("| owner crate | lib | test | members | users (lib rows / test rows) | fixed by |")
out.append("| --- | ---: | ---: | ---: | --- | --- |")
for o in sorted(by_owner, key=lambda o: -(tot[(o, "lib")] + tot[(o, "test")])):
    ms = by_owner[o]
    users = collections.Counter()
    for m in ms:
        for u, n in m["users"].items():
            users[S(u)] += n
    mine = sum(1 for m in ms if (m["owner"], m["member"]) in still)
    fixed = f"rw2t {len(ms) - mine}, rpv {mine}" if mine and len(ms) - mine else ("rpv" if mine else "rw2t")
    out.append(f"| `{S(o)}` | {tot[(o, 'lib')]} | {tot[(o, 'test')]} | {len(ms)} | " + ", ".join(f"`{u}` {n}" for u, n in users.most_common()) + f" | {fixed} |")
out.append("")
for o in sorted(by_owner, key=lambda o: -(tot[(o, "lib")] + tot[(o, "test")])):
    out.append(f"**`{S(o)}`**")
    grp = collections.OrderedDict()
    for m in by_owner[o]:
        k = m["member"].split(".")[0] if m["kind"] == "field" else "(methods and items)"
        grp.setdefault(k, []).append(m)
    for k, ms in grp.items():
        parts = []
        for m in ms:
            nm = m["member"].split(".", 1)[1] if m["kind"] == "field" else m["member"]
            mark = "*" if (m["owner"], m["member"]) in still else ""
            parts.append(f"`{nm}`{mark} {m['lib']}/{m['test']}")
        out.append(f"* `{k}`: " + ", ".join(parts))
    out.append("")
print("\n".join(out))
