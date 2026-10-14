#!/usr/bin/env python3
"""🧪️ Turns `#[cfg(test)]`-mounted test modules of a lib crate into `[[test]]` integration targets of a host crate.

A unit test module sees its parent through `use super::*` (every item and every import of the parent, private or not)
and the lib crate through `crate::`. An integration target links the lib as a dependency instead, so its compilation unit
holds only the test code. For each target the spec names, this tool:

- removes the `#[cfg(test)] #[path = "…"] mod …;` mounts of the moved files from their parent file;
- makes the target root the parent's stand-in: the lib crate's alias header, `use <lib>::<parent module>::*;`, and a
  copy of every `use` of the parent (cfg(test) ones included, `crate::`/`self::` made `<lib>::`), so `use super::*` and
  `crate::` keep resolving; the root is either a new file or one of the moved files (`root_is` = its path);
- widens to `pub` every item, inherent method and module of the lib closure that the moved code names, and every struct
  field it reads as `.field`, that the lib keeps private (`pub(crate)`/`pub(super)` included);
- applies the spec's literal rewrites to the moved files.

usage: 🧪️integration-target.py <spec.json> plan|apply

spec: {"lib": "<crate ident>", "lib_root": "<repo-rel crate root>", "parent": "<repo-rel parent file>",
       "parent_module": "a::b" | "", "rewrites": [[old, new], ...],
       "targets": [{"name", "root": "<repo-rel root path>", "doc": "<emoji> <text>",
                    "mounts": [{"file": "<repo-rel moved file>", "module": "<name>", "pub": bool, "parent": "<repo-rel
                    file holding its mount, default spec parent>"}]}]}
"""
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
census = {}
CENSUS = os.path.join(HERE, "🕸️module-graph.py")
exec(compile(open(CENSUS, encoding="utf-8").read().rsplit("\nmain()", 1)[0], CENSUS, "exec"), census)
strip, Crate = census["strip"], census["Crate"]

spec, mode = json.load(open(sys.argv[1], encoding="utf-8")), sys.argv[2]
absolute = lambda p: os.path.normpath(os.path.join(REPO, p))
lib, lib_root, parent = spec["lib"], absolute(spec["lib_root"]), absolute(spec["parent"])
parent_module = spec.get("parent_module", "")
texts, writes = {}, {}


def read(path):
    if path not in texts:
        texts[path] = writes.get(path) or open(path, encoding="utf-8").read()
    return writes.get(path, texts[path])


def write(path, text):
    writes[path] = text


def mount_regex(relpath):
    return re.compile(r'(?m)^[ \t]*#\[cfg\(test\)\]\n[ \t]*#\[path = "' + re.escape(relpath) + r'"\]\n[ \t]*(?:pub(?:\([^)]*\))? )?mod \w+;\n')


def statements(text, keep):
    """Top-level `use` statements (with leading cfg attrs) of a file, as source text."""
    s, out, depth, i = strip(text), [], 0, 0
    for m in re.finditer(r'[{}]|(?m)^((?:#\[[^\]]*\]\s*)*)(pub(?:\([^)]*\))?\s+)?use\b', s):
        if m.group(0) == "{":
            depth += 1
        elif m.group(0) == "}":
            depth -= 1
        elif depth == 0 and m.start() >= i:
            end = s.index(";", m.end()) + 1
            if keep(m.group(2)):
                out.append(text[m.start():end])
            i = end
    return out


lib_crate = Crate(lib_root)
lib_files = [f for f in lib_crate.files if f not in lib_crate.test_files and "🧪️tests" not in f]
moved = [absolute(m["file"]) for t in spec["targets"] for m in t["mounts"]]
tokens, fields, methods = set(), set(), set()
for path in moved:
    s = strip(read(path))
    tokens |= set(re.findall(r'\b[A-Za-z_][A-Za-z0-9_]*\b', s))
    fields |= set(re.findall(r'\.\s*([a-z_][a-z0-9_]*)\b(?!\s*[(!:])', s))
    methods |= set(re.findall(r'(?:\.|::)\s*([a-z_][a-z0-9_]*)\s*(?:::\s*<[^>]*>\s*)?\(', s))

for target in spec["targets"]:
    root = absolute(target["root"])
    header = [line for line in read(lib_root).split("\n") if re.match(r"^extern crate \w+ as \w+;$", line)]
    imports = []
    for statement in statements(read(parent), lambda vis: vis is None):
        body = re.sub(r'#\[cfg\(test\)\]\s*', "", statement)
        body = re.sub(r'\bcrate::', lib + "::", body)
        body = re.sub(r'\bself::', lib + ("::" + parent_module if parent_module else "") + "::", body)
        if re.search(r'\bsuper::', body):
            raise SystemExit(f"parent import needs manual resolution: {body}")
        if not re.match(r'use\s+[\w:]+::\{\s*\};', body):
            imports.append(body)
    glob = f"use {lib}{'::' + parent_module if parent_module else ''}::*;"
    mounts = []
    for m in target["mounts"]:
        file = absolute(m["file"])
        holder = absolute(m.get("parent", spec["parent"]))
        relpath = os.path.relpath(file, os.path.dirname(holder))
        if not m.get("new"):
            pattern = mount_regex(relpath)
            holder_text = read(holder)
            if not pattern.search(holder_text):
                raise SystemExit(f"no cfg(test) mount of {relpath} in {holder}")
            write(holder, pattern.sub("", holder_text, count=1))
        if file != root:
            mounts += [f'#[path = "{os.path.relpath(file, os.path.dirname(root))}"]', f"{'pub ' if m.get('pub') else ''}mod {m['module']};"]
    lines = [f"//! {target['doc']}", "", "#![allow(clippy::result_large_err)]", "", *header, "", glob, *imports, "", *mounts]
    if os.path.exists(root):
        own = read(root)
        own = re.sub(r'(?m)^use super::\*;\n', "", own)
        doc = re.match(r'((?://![^\n]*\n)+)\n?', own)
        rest = own[doc.end():] if doc else own
        prelude = "\n".join(lines[2:])
        write(root, (doc.group(1) if doc else f"//! {target['doc']}\n") + "\n" + prelude + "\n" + rest)
    else:
        write(root, "\n".join(lines) + "\n")

for path in moved:
    text = read(path)
    for old, new in spec.get("rewrites", []):
        text = text.replace(old, new)
    write(path, text)

ITEM = re.compile(r'(?m)^(?P<indent>[ \t]*)(?P<attrs>(?:#\[[^\n]*\]\s*\n[ \t]*)*)(?P<vis>pub\((?:crate|super|in [^)]+)\)\s+|pub\s+)?(?P<quals>(?:async |const |unsafe |extern "C" )*)(?P<kind>fn|struct|enum|static|const|type|trait|mod|union)\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)')
FIELD = re.compile(r'(?m)^(?P<indent>[ \t]+)(?P<vis>pub\((?:crate|super|in [^)]+)\)\s+|pub\s+)?(?P<name>[a-z_][a-z0-9_]*)\s*:(?!:)')
widened = []
for path in lib_files:
    if path in moved:
        continue
    text = read(path)
    s = strip(text)
    edits = []
    events = [(m.start(), m.group(0)) for m in re.finditer(r'[{}]', s)]
    def context(pos):
        ctx = []
        for p, ch in events:
            if p >= pos:
                break
            if ch == "{":
                head = s[max(s.rfind(";", 0, p), s.rfind("}", 0, p), s.rfind("{", 0, p)) + 1:p]
                h = re.sub(r'#\[[^\]]*\]', " ", head).strip()
                ctx.append((h, p))
            else:
                if ctx:
                    ctx.pop()
        return ctx
    for m in ITEM.finditer(s):
        name, kind = m["name"], m["kind"]
        if m["vis"] and m["vis"].strip() == "pub":
            continue
        ctx = context(m.start("kind"))
        heads = [h for h, _ in ctx]
        inner = heads[-1] if heads else None
        if inner is None or re.match(r'^(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+$', inner):
            wanted = name in tokens
        elif re.match(r'^(?:unsafe\s+)?impl\b', inner) and not re.search(r'\bfor\b', re.sub(r'<[^<>]*(?:<[^<>]*>[^<>]*)*>', "", inner)):
            wanted = kind in ("fn", "const", "type") and (name in methods or name in tokens)
        else:
            continue
        if not wanted or kind == "fn" and name == "main":
            continue
        at = m.start("vis") if m["vis"] else m.start("quals")
        edits.append((at, m.end("vis") if m["vis"] else at, "pub "))
        widened.append(f"{os.path.relpath(path, REPO)}: {kind} {name}")
    for m in FIELD.finditer(s):
        if m["vis"] and m["vis"].strip() == "pub":
            continue
        if m["name"] not in fields:
            continue
        ctx = context(m.start())
        if not ctx:
            continue
        inner = ctx[-1][0]
        if not re.match(r'^(?:pub(?:\([^)]*\))?\s+)?(?:struct|union)\b', inner):
            continue
        at = m.start("vis") if m["vis"] else m.start("name")
        edits.append((at, m.end("vis") if m["vis"] else at, "pub "))
        widened.append(f"{os.path.relpath(path, REPO)}: field {m['name']}")
    for start, end, new in sorted(set(edits), reverse=True):
        text = text[:start] + new + text[end:]
    if edits:
        write(path, text)

print("\n".join(widened))
print(f"widened {len(widened)}; writes {len(writes)}")
for path in writes:
    print("  " + os.path.relpath(path, REPO))
if mode == "apply":
    for path, text in writes.items():
        os.makedirs(os.path.dirname(path), exist_ok=True)
        open(path, "w", encoding="utf-8").write(text)
