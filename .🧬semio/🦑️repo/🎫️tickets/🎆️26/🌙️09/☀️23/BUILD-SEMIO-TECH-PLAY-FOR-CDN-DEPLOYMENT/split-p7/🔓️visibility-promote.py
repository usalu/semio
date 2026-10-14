#!/usr/bin/env python3
"""🔓️ Promotes crate-restricted items that a cut family names across crate boundaries (split lane P7).

The family cut spells every cross-crate reference as `<crate ident>::<path>`, but items the monolith declared
`pub(crate)`, `pub(super)`, `pub(in …)` or private stay restricted inside their new crate. This pass collects every
spelled path into each family crate from the other family crates, their test targets and the consumer files, and gives
the named item and every module segment on the path `pub` visibility in the crate that owns them. Names that resolve
to several restricted definitions are all promoted and listed, so the report shows each choice.

usage: 🔓️visibility-promote.py <cut config> plan|apply
"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
config_path, mode = sys.argv[1:3]
config = json.load(open(config_path, encoding="utf-8"))
plan = json.load(open(os.path.join(os.path.dirname(os.path.abspath(config_path)), "🗺️plan.json"), encoding="utf-8"))
artifact = os.path.join(REPO, config["artifact"])
family = plan["family"]
ident = {name: name.replace("-", "_") for name in family}
lib_files = {name: [os.path.join(artifact, f) for f in info["lib"]] for name, info in family.items()}
owner_of = {f: name for name, files in lib_files.items() for f in files}
readers = {}
for name, info in family.items():
    for f in info["lib"] + info["test"]:
        readers.setdefault(os.path.join(artifact, f), set()).add(name)
for t in config.get("tests", []) + config.get("existing_tests", []):
    readers.setdefault(os.path.join(artifact, t["file"]), set()).add(None)
for rel in plan.get("consumers", []):
    path = rel if os.path.isabs(rel) else os.path.join(REPO, rel)
    if os.path.exists(path):
        readers.setdefault(path, set()).add(None)
IDENT = r"[A-Za-z_][A-Za-z0-9_]*"


def strip(text):
    text = re.sub(r"//[^\n]*", "", text)
    text = re.sub(r"/\*.*?\*/", "", text, flags=re.S)
    return re.sub(r'"(?:\\.|[^"\\])*"', '""', text)


def leaves(text, at):
    """🌿️ Expands the path that starts at `at` into leaf segment lists, following `::{…}` groups."""
    out, segs, i = [], [], at
    while True:
        m = re.compile(r"\s*(" + IDENT + r"|\*)").match(text, i)
        if not m:
            break
        segs.append(m.group(1))
        i = m.end()
        n = re.compile(r"\s*::\s*").match(text, i)
        if not n:
            break
        i = n.end()
        if text.startswith("{", i):
            depth, j = 0, i
            while j < len(text):
                depth += text[j] == "{"
                depth -= text[j] == "}"
                if depth == 0:
                    break
                j += 1
            inner, parts, depth, last = text[i + 1:j], [], 0, 0
            for k, ch in enumerate(inner):
                depth += ch in "{"
                depth -= ch in "}"
                if ch == "," and depth == 0:
                    parts.append(inner[last:k]); last = k + 1
            parts.append(inner[last:])
            for part in parts:
                part = re.sub(r"\s+as\s+" + IDENT + r"\s*$", "", part.strip())
                if part and part != "self":
                    out += [segs + leaf for leaf in leaves(part, 0)]
                elif part == "self":
                    out.append(list(segs))
            return out
    out.append(segs)
    return out


def in_trait_block(text, at):
    """🧱️ Whether the definition at `at` sits in a trait or a trait impl, where visibility qualifiers are refused."""
    indent = len(text[text.rfind("\n", 0, at) + 1:at]) - len(text[text.rfind("\n", 0, at) + 1:at].lstrip())
    lines = text[:text.rfind("\n", 0, at)].split("\n")
    for line in reversed(lines):
        if line.strip() and len(line) - len(line.lstrip()) < indent:
            return re.match(r"\s*(?:unsafe\s+)?(?:impl\b.*\bfor\b|(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?trait\b)", line) is not None
    return False


wanted = {name: {} for name in family}
for path, owners in readers.items():
    text = strip(open(path, encoding="utf-8").read())
    for name, crate_ident in ident.items():
        if name in owners:
            continue
        for m in re.finditer(r"(?<![\w:])" + crate_ident + r"\s*::", text):
            for segs in leaves(text, m.end()):
                segs = [s for s in segs if s != "*"]
                if segs:
                    wanted[name].setdefault(tuple(segs), set()).add(os.path.relpath(path, REPO))
KINDS = r"(?:(?:const|async|unsafe|extern\s+\"C\"|default)\s+)*(?:fn|struct|enum|trait|type|const|static|union|mod)"
changes, report = {}, []
for name, paths in wanted.items():
    names, modules = set(), set()
    for segs in paths:
        names.add(segs[-1])
        modules.update(segs[:-1])
    for f in lib_files[name]:
        raw = open(f, encoding="utf-8").read()
        text = changes.get(f, raw)
        new = text
        for n in sorted(names | modules):
            kinds = KINDS if n in names else r"mod"
            pattern = re.compile(r"(?m)^(?P<indent>[ \t]*)(?P<vis>pub\s*\((?:crate|super|self|in [^)]*)\)\s*|)(?P<head>" + kinds + r"\s+" + re.escape(n) + r"\b)")
            def promote(m):
                if m.group("indent") and in_trait_block(m.string, m.start("head")):
                    return m.group(0)
                report.append(f"{name}: {m.group('vis').strip() or 'private'} {m.group('head').split()[-2]} {n} in {os.path.relpath(f, artifact)}")
                return m.group("indent") + "pub " + m.group("head")
            new = pattern.sub(promote, new)
            use = re.compile(r"(?m)^(?P<indent>[ \t]*)pub\s*\((?:crate|super|self|in [^)]*)\)\s*(?P<head>use\s+[^;]*\b" + re.escape(n) + r"\b[^;]*;)")
            def promote_use(m):
                report.append(f"{name}: restricted use {n} in {os.path.relpath(f, artifact)}")
                return m.group("indent") + "pub " + m.group("head")
            if n in names:
                new = use.sub(promote_use, new)
        if new != text:
            changes[f] = new
for line in sorted(set(report)):
    print(line)
print(f"files={len(changes)} promotions={len(set(report))}")
if mode == "apply":
    for f, text in changes.items():
        open(f, "w", encoding="utf-8").write(text)
