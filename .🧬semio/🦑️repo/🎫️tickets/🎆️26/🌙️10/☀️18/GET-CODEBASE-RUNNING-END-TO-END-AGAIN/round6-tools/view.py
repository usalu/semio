#!/usr/bin/env python3
"""Monolith view of the split build script: `in` inlines the code that moved out of 🏗️build/📜️script.ts (⌨️command/*, 🎯️targets) back into script.ts, in the script's own relative-specifier form, so the sets' anchored edits (staged against the monolith) apply unchanged; `out` splits the edited monolith back into the real files (specifiers re-relativised, import lines kept, imports of symbols the edits introduced added). `selftest` proves in+out is the identity on the current tree.
usage: view.py in|out|selftest <root>"""
import json, os, re, sys

B = "\U0001F9F0️framework/\U0001F6CD️products/\U0001F991️repo/\U0001F528️modules/\U0001F4DA️library/\U0001F3D7️build"
SCRIPT = B + "/\U0001F4DC️script.ts"
CMD = B + "/⌨️command"
PIECES = [CMD + "/" + d + "/\U0001F7E6️.ts" for d in ("⚙️runtime", "\U0001F4BE️cache", "\U0001F4E5️context", "\U0001F4E5️entry", "\U0001F4E6️dependencies", "\U0001F9ED️selection", "\U0001FA7A️preflight")] + [B + "/\U0001F3AF️targets/\U0001F7E6️.ts"]
MARK = "//@@r6piece "
END = "//@@r6end@@\n"
LIT = re.compile(r"([\"'])(\u200b?\.{1,2}/[^\"'\n]*)\1")
IMPORT = re.compile(r"^import\b[^;\n]*(?:\n[^;\n]*)*?;?\s*$", re.M)
HEAD = re.compile(r"^(?:import[^\n]*\n|export \{[^\n]*\} from[^\n]*\n)+")


def depth(rel):
    return len(os.path.dirname(rel[len(B) + 1:]).split("/"))


def to_script(spec, rel):
    d = depth(rel)
    if spec.startswith("\u200b"):
        return spec
    own = "./" + os.path.dirname(rel[len(B) + 1:]) + "/"
    if spec.startswith("./"):
        return "./" + os.path.dirname(rel[len(B) + 1:]) + "/" + spec[2:]
    ups = 0
    s = spec
    while s.startswith("../"):
        ups += 1
        s = s[3:]
    return ("./" if ups == d else "../" * (ups - d)) + s if ups >= d else "\u200b" + spec


def to_piece(spec, rel):
    d = depth(rel)
    if spec.startswith("\u200b"):
        return spec[1:]
    own = "./" + os.path.dirname(rel[len(B) + 1:]) + "/"
    if spec.startswith(own):
        return "./" + spec[len(own):]
    if spec.startswith("./"):
        return "../" * d + spec[2:]
    return "../" * (d + spec.count("../")) + spec.replace("../", "")


def conv(text, rel, fn):
    return LIT.sub(lambda m: m.group(1) + fn(m.group(2), rel) + m.group(1), text)


def indent(text):
    return "\n".join(("    " + l if l.strip() else l) for l in text.split("\n"))


def dedent(text):
    return "\n".join((l[4:] if l.startswith("    ") else l) for l in text.split("\n"))


def split_head(text):
    m = HEAD.match(text)
    return (m.group(0), text[m.end():]) if m else ("", text)


def read(root, rel):
    p = os.path.join(root, rel)
    return open(p, encoding="utf8").read() if os.path.isfile(p) else None


def pieces(root):
    found = []
    for base, dirs, files in os.walk(os.path.join(root, CMD)):
        dirs[:] = sorted(d for d in dirs if d != "\U0001F9EA\ufe0ftests")
        for f in files:
            if f == "\U0001F7E6\ufe0f.ts":
                found.append(os.path.relpath(os.path.join(base, f), root))
    first = [p for p in PIECES if p in found]
    return first + sorted(p for p in found if p not in PIECES) + [p for p in PIECES if p.startswith(B + "/\U0001F3AF")]


def view_in(root):
    script = read(root, SCRIPT)
    if MARK in script:
        return
    side, parts = {}, []
    for rel in pieces(root):
        text = read(root, rel)
        if text is None:
            continue
        head, body = split_head(text)
        side[rel] = head
        parts.append(MARK + rel + "\n" + conv(body, rel, to_script) + ("" if body.endswith("\n") else "\n") + END)
    json.dump(side, open(os.path.join(root, B, ".r6view.json"), "w", encoding="utf8"), ensure_ascii=False)
    open(os.path.join(root, SCRIPT), "w", encoding="utf8").write(script + ("" if script.endswith("\n") else "\n") + "".join(parts))


def head_names(head):
    names = {}
    for m in re.finditer(r"import\s*(?:type\s*)?\{([^}]*)\}\s*from\s*([\"'][^\"']+[\"'])", head):
        for n in m.group(1).split(","):
            n = n.strip().split(" as ")[-1].strip()
            if n:
                names[n] = m.group(2)
    for m in re.finditer(r"import\s+(\w+)\s+from\s*([\"'][^\"']+[\"'])", head):
        names[m.group(1)] = m.group(2)
    return names


def view_out(root):
    script = read(root, SCRIPT)
    if MARK not in script:
        return
    side = json.load(open(os.path.join(root, B, ".r6view.json"), encoding="utf8"))
    first = script.index(MARK)
    main, rest = script[:first], script[first:]
    shead, _ = split_head(main)
    known = head_names(main)
    for m in re.finditer(r"^export\s+(?:async\s+)?(?:function\*?|const|let|class)\s+([A-Za-z_$][\w$]*)", main, re.M):
        known.setdefault(m.group(1), '"./' + os.path.basename(SCRIPT) + '"')
    for chunk in rest.split(END):
        if not chunk.strip():
            continue
        rel, body = chunk[len(MARK):].split("\n", 1)
        body = conv(body, rel, to_piece)
        head = side.get(rel, "")
        have = head_names(head)
        add = ""
        for name, spec in known.items():
            if name not in have and re.search(r"(?<![\w$.])" + re.escape(name) + r"(?![\w$])", body) and not re.search(r"(?:function|const|let|class)\s+" + re.escape(name) + r"\b", body):
                spec2 = spec[1:-1]
                if spec2.startswith("."):
                    spec2 = to_piece(spec2, rel)
                add += 'import {%s} from "%s";\n' % (name, spec2) if not spec2.startswith("node:") or True else ""
        new = head + add + body
        if read(root, rel) != new:
            open(os.path.join(root, rel), "w", encoding="utf8").write(new)
    if read(root, SCRIPT) != main:
        open(os.path.join(root, SCRIPT), "w", encoding="utf8").write(main)
    os.remove(os.path.join(root, B, ".r6view.json"))


def selftest(root):
    snap = {rel: read(root, rel) for rel in [SCRIPT] + pieces(root)}
    view_in(root)
    view_out(root)
    bad = [rel[-40:] for rel, t in snap.items() if t != read(root, rel)]
    print("selftest:", "identity" if not bad else "DIFFERS " + str(bad))
    return 0 if not bad else 1


if __name__ == "__main__":
    root = os.path.abspath(sys.argv[2])
    sys.exit({"in": lambda r: view_in(r) or 0, "out": lambda r: view_out(r) or 0, "selftest": selftest}[sys.argv[1]](root))
