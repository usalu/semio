#!/usr/bin/env python3
"""🌐️ Carries a module's private imports into the test targets that glob it from outside (split lane P7).

Inside the monolith a test mounted below a module saw that module's private `use` items through `use super::*`. As a
`[[test]]` target the same glob reads `<crate>::<module>::*` and only reaches public items, so every name the test took
from the parent's imports goes missing. This pass resolves each family glob in a test target's files to the module's
source file, and inserts that module's private top-level imports (with `crate::` spelled as the owning crate) right
after the glob, skipping leaves the test already imports by name.

usage: 🧪️glob-imports.py <cut config> plan|apply
"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
config_path, mode = sys.argv[1:3]
config = json.load(open(config_path, encoding="utf-8"))
plan = json.load(open(os.path.join(os.path.dirname(os.path.abspath(config_path)), "🗺️plan.json"), encoding="utf-8"))["family"]
artifact = os.path.join(REPO, config["artifact"])
roots = {name.replace("-", "_"): os.path.join(artifact, info["root"] if "root" in info else info["lib"][0]) for name, info in plan.items()}


def body_of(text, at):
    i = text.index("{", at)
    depth, j = 0, i
    while True:
        depth += text[j] == "{"
        depth -= text[j] == "}"
        if depth == 0:
            return text[i + 1:j]
        j += 1


def top_level(text):
    out, depth, start = [], 0, 0
    for k, ch in enumerate(text):
        if ch == "{":
            if depth == 0:
                out.append(text[start:k])
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                start = k + 1
    out.append(text[start:])
    return "\n".join(out)


MOD = re.compile(r'(?:#\[path\s*=\s*"(?P<path>[^"]+)"\]\s*)?(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+(?P<name>\w+)\s*(?P<end>[;{])')


def resolve(file, text, directory, segs):
    """🧭️ Returns the source file and text of the module `segs` below the module held in `text`."""
    if not segs:
        comp = re.search(r'#\[path\s*=\s*"([^"]+)"\]\s*mod\s+component\s*;', top_level(text))
        if comp and re.search(r"pub use component::\*;", text):
            path = os.path.normpath(os.path.join(directory, comp.group(1)))
            return path, open(path, encoding="utf-8").read()
        return file, text
    for m in MOD.finditer(text):
        if m.group("name") != segs[0] or text[:m.start()].count("{") - text[:m.start()].count("}") != 0:
            continue
        if m.group("end") == ";":
            path = os.path.normpath(os.path.join(directory, m.group("path") or segs[0] + ".rs"))
            if not os.path.exists(path):
                return None
            return resolve(path, open(path, encoding="utf-8").read(), os.path.dirname(path), segs[1:])
        inner = body_of(text, m.end() - 1)
        sub = os.path.normpath(os.path.join(directory, m.group("path") or segs[0]))
        return resolve(file, inner, sub, segs[1:])
    for g in re.finditer(r"pub use (\w+)::\*;", top_level(text)):
        if g.group(1) == segs[0]:
            continue
        found = resolve(file, text, directory, [g.group(1)] + segs)
        if found:
            return found
    return None


def leaves(use):
    body = re.sub(r"\s+", " ", use)
    return set(re.findall(r"(?:as\s+(\w+)|(\w+))\s*(?=[,}]|;|$)", body.split(" use ", 1)[-1] if " use " in body else body)) and {a or b for a, b in re.findall(r"(?:as\s+(\w+)|(\w+))\s*(?=[,};])", body)}


def mounted(root):
    seen, stack = [], [root]
    while stack:
        path = os.path.normpath(stack.pop())
        if path in seen or not os.path.exists(path) or "🧰️framework" in path:
            continue
        seen.append(path)
        for m in re.finditer(r'#\[path\s*=\s*"([^"]+\.rs)"\]', open(path, encoding="utf-8").read()):
            stack.append(os.path.join(os.path.dirname(path), m.group(1)))
    return seen


USE = re.compile(r"(?m)^(?P<attrs>(?:#\[[^\n]*\]\s*)*)use\s+(?P<body>[^;]+);")
files = sorted({f for t in config.get("tests", []) + config.get("existing_tests", []) for f in mounted(os.path.join(artifact, t["file"]))})
changes, report = {}, []
for f in files:
    text = open(f, encoding="utf-8").read()
    new = text
    for g in re.finditer(r"(?m)^(?P<indent>[ \t]*)use\s+(?P<crate>semio_\w+?)(?P<segs>(?:::\w+)*)::\*;\n", text):
        crate = g.group("crate")
        if crate not in roots:
            continue
        segs = [s for s in g.group("segs").split("::") if s]
        root = roots[crate]
        found = resolve(root, open(root, encoding="utf-8").read(), os.path.dirname(root), segs)
        if not found:
            report.append(f"UNRESOLVED {os.path.relpath(f, artifact)}: {crate}::{'::'.join(segs)}::*")
            continue
        module_file, module_text = found
        have = {a or b for a, b in re.findall(r"(?:as\s+(\w+)|(\w+))\s*(?=[,};])", " ".join(m.group("body") for m in USE.finditer(new)))}
        carried = []
        clean = re.sub(r'"(?:\\.|[^"\\])*"', lambda s: '"' + "x" * (len(s.group(0)) - 2) + '"', re.sub(r"//[^\n]*", lambda s: " " * len(s.group(0)), module_text))
        depths, depth = [], 0
        for ch in clean:
            depths.append(depth)
            depth += ch == "{"
            depth -= ch == "}"
        for u in (u for u in USE.finditer(module_text) if depths[u.start()] == 0 and not clean[u.start():u.start() + 2] == "  "):
            line = u.group(0)
            if re.match(r"(?:#\[[^\n]*\]\s*)*use", line) is None:
                continue
            body = u.group("body").strip()
            if body.startswith(("super::", "self::")) or "*" in body and body.endswith("*") and body.startswith("crate::"):
                continue
            body = re.sub(r"(?<![\w:])crate::", crate + "::", body)
            names = {a or b for a, b in re.findall(r"(?:as\s+(\w+)|(\w+))\s*(?=[,}]|$)", body)}
            if names and names <= have:
                continue
            attrs = re.sub(r"#\[cfg\(test\)\]\s*", "", u.group("attrs"))
            carried.append(g.group("indent") + attrs.replace("\n", "\n" + g.group("indent")) + f"use {body};")
            have |= names
        if carried:
            new = new.replace(g.group(0), g.group(0) + "\n".join(carried) + "\n", 1)
            report.append(f"{os.path.relpath(f, artifact)}: +{len(carried)} from {os.path.relpath(module_file, artifact)}")
    if new != text:
        changes[f] = new
for line in report:
    print(line)
print(f"files={len(changes)}")
if mode == "apply":
    for f, t in changes.items():
        open(f, "w", encoding="utf-8").write(t)
