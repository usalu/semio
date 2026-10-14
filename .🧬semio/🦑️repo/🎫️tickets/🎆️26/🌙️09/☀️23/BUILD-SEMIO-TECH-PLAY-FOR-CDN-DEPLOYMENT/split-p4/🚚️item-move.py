#!/usr/bin/env python3
"""🚚️ Moves top-level items between files of one `#[path]`-tree crate, keeping every reference resolvable (lane P4).

Selected items (by name, `re:` regex, or `region:` marker path) take their `impl` blocks along. Inside the moved text every
relative path becomes absolute (`crate::…` from the source module); bare names of items left behind are imported from the
source module, and the source's `use` lines the moved text needs are copied (made absolute). The source imports what it
still names from the target. Every path anywhere in the crate (and in consumer files) that resolved to a moved item is
retargeted to the target module. A missing target file is created with `doc` and mounted with `mount` (a declaration
appended to `mount_in`).

usage: 🚚️item-move.py <spec.json> [--apply]
spec: {"root": crate root, "crate": ident, "consumers": [dirs],
       "moves": [{"from": file, "to": file, "to_module": "a::b", "select": [name | "re:…" | "region:…"],
                  "doc": "//! …" (new file), "mount_in": file, "mount": "#[path = …] pub mod x;"}]}
"""
import collections, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
_cut = {"__file__": os.path.join(HERE, "✂️owner-cut.py")}
exec(open(os.path.join(HERE, "✂️owner-cut.py"), encoding="utf-8").read().split("\nif __name__")[0], _cut)
Old, strip, use_leaves, inline_mods, apply_edits = _cut["Old"], _cut["strip"], _cut["use_leaves"], _cut["inline_mods"], _cut["apply_edits"]
USE_STMT, INLINE_PATH, IDENT, SEP = _cut["USE_STMT"], _cut["INLINE_PATH"], _cut["IDENT"], _cut["SEP"]
_split = {"__file__": os.path.join(HERE, "🔀️item-split.py")}
exec(compile(open(os.path.join(HERE, "🔀️item-split.py"), encoding="utf-8").read().split("\nif __name__")[0], "item-split", "exec"), _split)
parse = _split["parse"]


def selected(items, select, exclude=()):
    names, regexes, regions, exact = set(), [], [], []
    for sel in select:
        if sel.startswith("re:"):
            regexes.append(re.compile(sel[3:]))
        elif sel.startswith("region:"):
            regions.append(tuple(sel[7:].split("/")))
        elif sel.startswith("regionx:"):
            exact.append(tuple(sel[8:].split("/")))
        else:
            names.add(sel)
    chosen = set()
    for it in items:
        if it.kind in ("use",):
            continue
        if it.kind not in ("impl", "macro", "?") and (it.name in names or any(r.fullmatch(it.name) for r in regexes)):
            chosen.add(it.i)
        if it.kind in ("macro", "?", "impl") and it.name in names:
            chosen.add(it.i)
        if regions and any(it.region[:len(r)] == r for r in regions) and it.kind != "use":
            chosen.add(it.i)
        if exact and any(it.region == r for r in exact) and it.kind != "use":
            chosen.add(it.i)
    types = {items[i].name for i in chosen}
    for it in items:
        if it.kind == "impl" and (it.target in types):
            chosen.add(it.i)
        if it.kind == "macro" and set(re.findall(r"\b[A-Za-z_][A-Za-z0-9_]*\b", it.stripped)) & types and it.i - 1 in chosen:
            chosen.add(it.i)
    drop = [re.compile(x) for x in exclude]
    for it in items:
        code = re.sub(r"^(\s*(#!?\[[^\]]*\]|///[^\n]*|//[^\n]*)\s*)*", "", it.text)
        header = code.split("{", 1)[0].strip()
        if any(r.search(header) for r in drop):
            chosen.discard(it.i)
    return chosen


def absolutize(text, module, old, file_module_offsets=None):
    """🧭️ Every relative path in `text` (stripped twin used for scanning) → `crate::…` from `module`."""
    s = strip(text)
    edits = []
    covered = []
    spans = inline_mods(s)

    def mod_at(off):
        mod = list(module)
        for a, b, name, _ in sorted(spans, key=lambda sp: sp[0]):
            if a <= off < b:
                mod.append(name)
        return mod

    for m in USE_STMT.finditer(s):
        body = m.group("body")
        if not re.match(r"(super|self)\b", body.strip()):
            continue
        parts = []
        for segs, alias, glob in use_leaves([], body):
            if segs and segs[0] in ("super", "self"):
                absolute = old.absolute(segs[:-1] if segs[-1] == "self" else segs, mod_at(m.start()))
                path = "crate::" + "::".join(absolute) if absolute is not None else "::".join(segs)
            else:
                path = "::".join(segs)
            parts.append(path + ("::*" if glob else "") + (f" as {alias}" if alias else ""))
        tree = parts[0] if len(parts) == 1 else "{" + ", ".join(parts) + "}"
        edits.append((m.start("body"), m.end("body"), tree))
        covered.append((m.start(), m.end()))
    for m in INLINE_PATH.finditer(s):
        if any(a <= m.start() < b for a, b in covered):
            continue
        segs = [x for x in re.split(SEP, m.group("path")) if x]
        if segs[0] not in ("super", "self"):
            continue
        absolute = old.absolute(segs, mod_at(m.start()))
        if absolute is not None:
            edits.append((m.start(), m.end(), "crate::" + "::".join(absolute)))
    return apply_edits(text, edits)


def retarget_text(text, old, retarget):
    """🎯️ Rewrites every absolute `crate::` path in `text` that names a moved item to its target module."""
    s = strip(text)
    edits, covered = [], []

    def new_path(segs):
        loc = old.locate(segs[1:])
        if loc.item is not None and (loc.module, loc.item) in retarget:
            return "crate::" + "::".join(list(retarget[(loc.module, loc.item)]) + [loc.item] + loc.rest)
        return None

    for m in USE_STMT.finditer(s):
        body = m.group("body")
        if not body.strip().startswith("crate"):
            continue
        covered.append((m.start(), m.end()))
        parts, changed = [], False
        for segs, alias, glob in use_leaves([], body):
            new = new_path(segs) if segs and not glob else None
            changed |= bool(new)
            parts.append((new or "::".join(segs)) + ("::*" if glob else "") + (f" as {alias}" if alias else ""))
        if changed:
            edits.append((m.start("body"), m.end("body"), parts[0] if len(parts) == 1 else "{" + ", ".join(parts) + "}"))
    for m in INLINE_PATH.finditer(s):
        if any(a <= m.start() < b for a, b in covered):
            continue
        segs = [x for x in re.split(SEP, m.group("path")) if x]
        if segs[0] != "crate":
            continue
        new = new_path(segs)
        if new:
            edits.append((m.start(), m.end(), new))
    return apply_edits(text, edits)


def main():
    spec = json.load(open(sys.argv[1], encoding="utf-8"))
    write = "--apply" in sys.argv
    old = Old(spec["root"])
    texts = {f: old.raw[f] for f in old.crate.files}
    created = {}
    retarget = {}
    plans = []
    for move in spec["moves"]:
        src = os.path.normpath(os.path.join(REPO, move["from"]))
        dst = os.path.normpath(os.path.join(REPO, move["to"]))
        src_module = old.norm(old.crate.files[src])
        dst_module = tuple(x for x in move["to_module"].split("::") if x)
        _, items = parse(src)
        chosen = selected(items, move["select"], move.get("exclude", []))
        moved_names = {items[i].name for i in chosen if items[i].kind not in ("impl", "macro", "?")}
        for name in moved_names:
            retarget[(src_module, name)] = dst_module
        plans.append((move, src, dst, src_module, dst_module, items, chosen, moved_names))
        print(f"move {len(chosen)} items ({sum(items[i].text.count(chr(10)) + 1 for i in chosen)} lines) {os.path.relpath(src, REPO)} -> {os.path.relpath(dst, REPO)}: {sorted(moved_names)[:12]}{'…' if len(moved_names) > 12 else ''}")
    edits = collections.defaultdict(list)
    for f in old.crate.files:
        s = old.text[f]
        spans = inline_mods(s)
        mod_base = tuple(old.crate.files[f])
        root_mods = old.root_modules()

        def mod_at(off):
            mod = list(mod_base)
            for a, b, name, _ in sorted(spans, key=lambda sp: sp[0]):
                if a <= off < b:
                    mod.append(name)
            return mod

        def target_of(segs, off):
            if segs[0] in ("crate", "super", "self"):
                absolute = old.absolute(segs, mod_at(off))
            elif f == old.crate.root and segs[0] in root_mods:
                absolute = list(segs)
            else:
                return None
            if absolute is None:
                return None
            loc = old.locate(absolute)
            if loc.item is not None and (loc.module, loc.item) in retarget:
                return "crate::" + "::".join(list(retarget[(loc.module, loc.item)]) + [loc.item] + loc.rest)
            return None

        covered = []
        for m in USE_STMT.finditer(s):
            body = m.group("body")
            head = re.match(r"(?:::\s*)?(" + IDENT + r")", body).group(1)
            if head not in ("crate", "super", "self") and not (f == old.crate.root and head in root_mods):
                continue
            covered.append((m.start(), m.end()))
            leaves = use_leaves([], body)
            parts, changed = [], False
            for segs, alias, glob in leaves:
                new = target_of(segs, m.start()) if segs and not glob else None
                if new:
                    changed = True
                    parts.append(new + (f" as {alias}" if alias else ""))
                else:
                    parts.append("::".join(segs) + ("::*" if glob else "") + (f" as {alias}" if alias else ""))
            if changed:
                edits[f].append((m.start("body"), m.end("body"), parts[0] if len(parts) == 1 else "{" + ", ".join(parts) + "}"))
        for m in INLINE_PATH.finditer(s):
            if any(a <= m.start() < b for a, b in covered):
                continue
            segs = [x for x in re.split(SEP, m.group("path")) if x]
            new = target_of(segs, m.start())
            if new:
                edits[f].append((m.start(), m.end(), new))
    for move, src, dst, src_module, dst_module, items, chosen, moved_names in plans:
        src_text = texts[src]
        removed = []
        moved_text = []
        for it in items:
            if it.i in chosen:
                removed.append((it.start, it.end, ""))
                moved_text.append(retarget_text(absolutize(it.text, old.crate.files[src], old), old, retarget))
        body = "\n\n".join(t.strip("\n") for t in moved_text)
        moved_idents = set(re.findall(r"\b[A-Za-z_][A-Za-z0-9_]*\b", strip(body)))
        left = {it.name for it in items if it.i not in chosen and it.kind not in ("impl", "use", "macro", "?")}
        need_from_src = sorted(left & moved_idents)
        uses = []
        for it in items:
            if it.kind != "use":
                continue
            stripped = strip(it.text)
            leaves = []
            m = USE_STMT.search(stripped)
            if not m:
                continue
            for segs, alias, glob in use_leaves([], m.group("body")):
                name = alias or (segs[-1] if segs else None)
                if glob or (name and name in moved_idents):
                    leaves.append((segs, alias, glob))
            if not leaves:
                continue
            head = re.match(r"\s*((?:#\[[^\n]*\]\s*)*)(?:pub(?:\s*\([^)]*\))?\s+)?use\b", it.text)
            attrs = head.group(1) if head else ""
            parts = []
            for segs, alias, glob in leaves:
                if segs and segs[0] in ("super", "self"):
                    absolute = old.absolute(segs, list(old.crate.files[src]))
                    path = "crate::" + "::".join(absolute) if absolute else "::".join(segs)
                else:
                    path = "::".join(segs)
                parts.append(path + ("::*" if glob else "") + (f" as {alias}" if alias else ""))
            uses.append(f"{attrs}use {parts[0] if len(parts) == 1 else '{' + ', '.join(parts) + '}'};")
        if need_from_src:
            uses.append(f"use crate::{'::'.join(src_module)}{'::' if src_module else ''}{{{', '.join(need_from_src)}}};" if len(need_from_src) > 1 else f"use crate::{'::'.join(src_module)}{'::' if src_module else ''}{need_from_src[0]};")
        remaining = apply_edits(src_text, [e for e in removed])
        still = set(re.findall(r"\b[A-Za-z_][A-Za-z0-9_]*\b", strip(remaining)))
        back = sorted(moved_names & still)
        src_edits = [e for e in edits[src] if not any(a <= e[0] < b for a, b, _ in removed)] + removed
        if back:
            imp = f"use crate::{'::'.join(dst_module)}::{{{', '.join(back)}}};\n" if len(back) > 1 else f"use crate::{'::'.join(dst_module)}::{back[0]};\n"
            first_use = re.search(r"(?m)^use ", src_text)
            at = first_use.start() if first_use else 0
            src_edits.append((at, at, imp))
        edits[src] = src_edits
        block = "\n".join(dict.fromkeys(uses)) + "\n\n" + body + "\n"
        if os.path.exists(dst) or dst in created:
            created.setdefault(dst, [])
            created[dst].append(block)
        else:
            created[dst] = [move.get("doc", "//! moved") + "\n\n" + block]
            if move.get("mount_in"):
                mount_in = os.path.normpath(os.path.join(REPO, move["mount_in"]))
                edits[mount_in].append((len(texts[mount_in]), len(texts[mount_in]), "\n" + move["mount"] + "\n"))
        print(f"   imports into target: {len(uses)} use lines; source re-imports {back[:10]}")
    for root in spec.get("consumers", []):
        pass
    if not write:
        return
    for f, es in edits.items():
        dedup, last = [], -1
        for e in sorted(es):
            if e[0] < last:
                continue
            dedup.append(e); last = e[1] if e[1] > e[0] else last
        texts[f] = apply_edits(texts[f], dedup)
    for dst, blocks in created.items():
        base = texts.get(dst) if dst in texts else (open(dst, encoding="utf-8").read() if os.path.exists(dst) else "")
        text = base.rstrip("\n") + ("\n\n" if base else "") + "\n\n".join(b.strip("\n") for b in blocks) + "\n"
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        texts[dst] = text
    for f, text in texts.items():
        if not os.path.exists(f) or open(f, encoding="utf-8").read() != text:
            open(f, "w", encoding="utf-8").write(text)
            print("wrote", os.path.relpath(f, REPO))


if __name__ == "__main__":
    main()
