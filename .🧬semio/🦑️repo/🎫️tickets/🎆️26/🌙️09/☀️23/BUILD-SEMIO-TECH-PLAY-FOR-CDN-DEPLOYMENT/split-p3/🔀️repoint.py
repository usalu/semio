#!/usr/bin/env python3
"""🔀️ Re-points every path to moved items of one `#[path]`-tree crate after the items were moved by hand.

Each mapping names an old absolute item or module path and its new absolute path inside the same crate. Every
`crate::`/`super::`/`self::` path (and every root-relative path in the crate root) that resolves — re-exports
followed — to an old path or below it is rewritten to `crate::<new path>`; `use` groups are split leaf by leaf.
External consumers naming `<crate ident>::<old path>` are rewritten to `<crate ident>::<new path>`. A path whose
rewrite would import an item into the module that defines it is dropped (`use` leaves only).

usage: 🔀️repoint.py <spec.json> plan|apply
spec: {"artifact": dir, "monolith": crate ident, "map": {"old::path": "new::path"}, "consumers": [roots]}
"""
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ex = {"__name__": "extract", "__file__": os.path.join(HERE, "🪚️extract.py")}
exec(compile(open(os.path.join(HERE, "🪚️extract.py"), encoding="utf-8").read(), "extract", "exec"), ex)
REPO, strip, leaves, segs_of, inline_spans, apply_edits, Crate = ex["REPO"], ex["strip"], ex["leaves"], ex["segs_of"], ex["inline_spans"], ex["apply_edits"], ex["Crate"]
IDENT, SEP = ex["IDENT"], ex["SEP"]


def main():
    spec = json.load(open(sys.argv[1], encoding="utf-8"))
    mode = sys.argv[2]
    model = ex["Model"].__new__(ex["Model"])
    model.spec = spec
    model.artifact = os.path.join(REPO, spec["artifact"])
    model.crate = Crate(os.path.join(model.artifact, "🦀️.rs"))
    model.raw = {f: open(f, encoding="utf-8").read() for f in model.crate.files}
    model.reexports = {}
    model.build_reexports()
    mapping = [(tuple(segs_of(a)), tuple(segs_of(b.lstrip(":"))) if not b.startswith("::") else ("::",) + tuple(segs_of(b[2:]))) for a, b in spec["map"].items()]

    def ext(new):
        if new and new[0] == "::":
            return "::".join(x for x in new[1:] if x != "component")
        return "crate::" + "::".join(x for x in new if x != "component")

    def mapped(target):
        hit = max((m for m in mapping if tuple(target[:len(m[0])]) == m[0]), key=lambda m: len(m[0]), default=None)
        return None if hit is None else hit[1] + tuple(target[len(hit[0]):])

    crate = model.crate
    writes = {}
    for f in sorted(crate.files):
        raw = model.raw[f]
        text = strip(raw)
        spans = inline_spans(text)
        here = tuple(x for x in crate.files[f] if x != "component")
        edits, covered = [], []

        def resolve(segs, offset):
            base = list(crate.files[f]) + [n for a, b, n in spans if a <= offset < b]
            target = model.absolute(segs, base)
            if target is None:
                return None, None
            return model.canonical(target), base

        heads_alt = "|".join(["crate", "super", "self"] + [re.escape(h) for h in ex["child_heads"](crate, f, spans)])
        use_re = re.compile(r"(?m)^(?P<attrs>(?:[ \t]*#\[[^\n]*\]\n)*)(?P<indent>[ \t]*)(?P<vis>pub(?:\s*\([^)]*\))?\s+)?use\s+(?P<body>(?:" + heads_alt + r")\b[^;]*);")
        for m in use_re.finditer(text):
            rows, changed = [], False
            for segs, alias in leaves([], m.group("body")):
                glob = bool(segs) and segs[-1] == "*"
                path = [s for s in segs if s != "*"]
                selfleaf = bool(path) and path[-1] == "self"
                if selfleaf:
                    path = path[:-1]
                target, base = resolve(path, m.start("body"))
                new = mapped(target) if target is not None else None
                if new is not None:
                    changed = True
                rows.append((path, glob, alias, new))
            if not changed:
                covered.append((m.start(), m.end()))
                continue
            lines = []
            for path, glob, alias, new in rows:
                if new is not None and new[0] != "::" and not glob and tuple(x for x in new[:-1] if x != "component") == here and not alias:
                    continue
                shown = (ext(new)) if new is not None else "::".join(path)
                tail = "::*" if glob else (f" as {alias}" if alias else "")
                lines.append(f"{m.group('attrs')}{m.group('indent')}{m.group('vis') or ''}use {shown}{tail};")
            edits.append((m.start(), m.end(), "\n".join(lines)))
            covered.append((m.start(), m.end()))
        for m in re.finditer(r"(?<![\w:$.])(?P<path>(?:" + heads_alt + r")(?:" + SEP + r"(?:" + IDENT + r"|super))+)", text):
            if any(a <= m.start() < b for a, b in covered):
                continue
            target, base = resolve(segs_of(m.group("path")), m.start())
            new = mapped(target) if target is not None else None
            if new is not None:
                edits.append((m.start(), m.end(), ext(new)))
        if f == crate.root:
            heads = sorted({mp[0] for mp in crate.mods if mp})
            for m in re.finditer(r"(?<![\w:$])(?P<path>(?:" + "|".join(map(re.escape, heads)) + r")(?:" + SEP + IDENT + r")+)", text):
                if any(a <= m.start() < b for a, b in covered) or any(a <= m.start() < b for a, b, _ in spans):
                    continue
                new = mapped(model.canonical(segs_of(m.group("path"))))
                if new is not None:
                    edits.append((m.start(), m.end(), ext(new) if new[0] == "::" else "::".join(x for x in new if x != "component")))
        if edits:
            writes[f] = apply_edits(raw, edits)
    ident = spec["monolith"]
    for root in spec.get("consumers", []):
        for dirpath, dirnames, filenames in os.walk(os.path.join(REPO, root)):
            dirnames[:] = [d for d in dirnames if d not in ("node_modules", "target", "🗑️generated") and not d.startswith(".")]
            for name in filenames:
                path = os.path.normpath(os.path.join(dirpath, name))
                if not name.endswith(".rs") or path in crate.files:
                    continue
                raw = open(path, encoding="utf-8").read()
                if ident not in raw:
                    continue
                text = strip(raw)
                edits = []
                for m in re.finditer(r"(?<![\w:])" + re.escape(ident) + r"(?P<rest>(?:" + SEP + IDENT + r")+)", text):
                    new = mapped(model.canonical(segs_of(m.group("rest"))))
                    if new is not None:
                        edits.append((m.start(), m.end(), ext(new) if new[0] == "::" else ident + "::" + "::".join(x for x in new if x != "component")))
                if re.search(r"use\s+" + re.escape(ident) + r"\s*::\s*\{", text):
                    print(f"CHECK group use in consumer {os.path.relpath(path, REPO)}")
                if edits:
                    writes[path] = apply_edits(raw, edits)
    for path in sorted(writes):
        print(f"rewrite {os.path.relpath(path, REPO)}")
    if mode == "apply":
        for path, text in writes.items():
            open(path, "w", encoding="utf-8").write(text)
        print(f"applied {len(writes)}")


main()
