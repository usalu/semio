#!/usr/bin/env python3
"""🔓️ Source-level visibility widening after a crate cut, for when no compiler is available to name each E0603.

1. Every name a family crate reaches in another family crate (`<crate ident>::a::b::Name` paths and `use` trees) is
   made `pub` where it is defined at top level in that crate's lib closure (items without visibility, and
   `pub(crate)`/`pub(super)`/`pub(in …)`).
2. With `--members <file>...`, the listed files (whose items were split across crates by item moves) also get `pub`
   on every named struct field and every inherent-impl associated fn/const that has no visibility; trait impls and
   macro bodies are left alone.

usage: 🔓️widen.py <workspace dir> <family package prefix> [--members <file>...] [--write]
"""
import json, os, re, subprocess, sys, importlib.util

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
spec = importlib.util.spec_from_file_location("relocate", os.path.join(HERE, "🧭️relocate.py"))
relocate = importlib.util.module_from_spec(spec)
spec.loader.exec_module(relocate)
args = sys.argv[1:]
write = "--write" in args
members_files = []
if "--members" in args:
    i = args.index("--members") + 1
    while i < len(args) and not args[i].startswith("--"):
        members_files.append(os.path.join(REPO, args[i]))
        i += 1
workspace_dir, prefix = os.path.join(REPO, args[0]), args[1]


def toml_load(path):
    out = subprocess.run(["bun", "-e", "console.log(JSON.stringify(Bun.TOML.parse(await Bun.file(process.argv[1]).text())))", path], capture_output=True, text=True, check=True)
    return json.loads(out.stdout)


def closure(root, mode):
    out = subprocess.run(["bun", os.path.join(HERE, "📏️unit-files.ts"), root, mode, "--files"], capture_output=True, text=True, cwd=HERE)
    return [os.path.join(REPO, line.split("\t")[4]) for line in out.stdout.splitlines() if line.startswith("F\t") and line.split("\t")[3] == "source"]


workspace = toml_load(os.path.join(workspace_dir, "Cargo.toml"))
crates = {}
for member in workspace["workspace"]["members"]:
    manifest = toml_load(os.path.join(workspace_dir, member, "Cargo.toml"))
    name = manifest["package"]["name"]
    if name.startswith(prefix):
        root = os.path.normpath(os.path.join(workspace_dir, member, manifest.get("lib", {}).get("path", "src/lib.rs")))
        crates[name.replace("-", "_")] = {"lib": closure(root, "native"), "all": closure(root, "test")}
idents = sorted(crates, key=len, reverse=True)
wanted = {ident: set() for ident in crates}
ref = re.compile(r"(?<![\w:])(" + "|".join(map(re.escape, idents)) + r")((?:\s*::\s*(?:\w+|\{))+)")
files = sorted({f for info in crates.values() for f in info["all"]})
for path in files:
    text = relocate.strip(open(path, encoding="utf-8").read())
    for m in ref.finditer(text):
        tail = m.group(2)
        if tail.rstrip().endswith("{"):
            j = text.find("{", m.start(2))
            close = relocate.close_of(text, j)
            for segs, _alias, _s in relocate.leaves([], text[j:close + 1]):
                wanted[m.group(1)].update(s for s in segs if s not in ("self", "*"))
            wanted[m.group(1)].update(s for s in re.split(r"\s*::\s*", tail) if s and s != "{")
        else:
            wanted[m.group(1)].update(s for s in re.split(r"\s*::\s*", tail) if s)
ITEM = re.compile(r"(?m)^(?P<indent>[ \t]*)(?P<vis>pub\s*\([^)]*\)\s+)?(?P<head>(?:(?:const|async|unsafe|extern\s+\"[^\"]*\")\s+)*(?:fn|struct|enum|trait|type|const|static|union|mod)\s+(?P<name>\w+))")
edits = {}
for ident, names in wanted.items():
    for path in crates[ident]["lib"]:
        raw = open(path, encoding="utf-8").read()
        text = relocate.strip(raw)
        for m in ITEM.finditer(text):
            if m.group("name") not in names:
                continue
            before = text[:m.start()]
            depth = before.count("{") - before.count("}")
            if depth and not re.search(r"\bmod\s+\w+\s*\{[^{}]*$", before[max(0, len(before) - 3000):]):
                continue
            line_start = m.start("head") if m.group("vis") is None else m.start("vis")
            prev = text[max(0, line_start - 4):line_start]
            if m.group("vis") is None and re.search(r"pub\s+$", text[max(0, line_start - 8):line_start]):
                continue
            edits.setdefault(path, []).append((line_start, m.start("head"), "pub "))
FIELD = re.compile(r"(?m)^(?P<indent>[ \t]+)(?P<vis>pub\s*\([^)]*\)\s+)?(?P<name>[a-z_][a-z0-9_]*)\s*:(?!:)")
for path in members_files:
    raw = open(path, encoding="utf-8").read()
    text = relocate.strip(raw)
    for s in re.finditer(r"(?m)^[ \t]*(?:pub(?:\s*\([^)]*\))?\s+)?struct\s+\w+[^{;(]*\{", text):
        close = relocate.close_of(text, s.end() - 1)
        for f in FIELD.finditer(text, s.end(), close):
            if text[f.start():f.end()].lstrip().startswith("pub "):
                continue
            edits.setdefault(path, []).append((f.start("vis") if f.group("vis") else f.start("name"), f.start("name"), "pub "))
    for s in re.finditer(r"(?m)^impl\b(?:<(?:[^<>]|<[^<>]*>)*>)?\s*([^{]*)\{", text):
        if re.search(r"\bfor\b", s.group(1)):
            continue
        close = relocate.close_of(text, s.end() - 1)
        for f in re.finditer(r"(?m)^(?P<indent>[ \t]+)(?P<vis>pub\s*\([^)]*\)\s+)?(?P<head>(?:(?:const|async|unsafe)\s+)*fn\s+\w+|const\s+[A-Z_][A-Z0-9_]*\s*:)", text[:close]):
            if f.start() < s.end():
                continue
            inner = text[s.end():f.start()]
            if inner.count("{") - inner.count("}") != 0:
                continue
            edits.setdefault(path, []).append((f.start("vis") if f.group("vis") else f.start("head"), f.start("head"), "pub "))
total = 0
for path, items in edits.items():
    raw = open(path, encoding="utf-8").read()
    for a, b, rep in sorted(set(items), reverse=True):
        raw = raw[:a] + rep + raw[b:]
        total += 1
    if write:
        open(path, "w", encoding="utf-8").write(raw)
print(f"widened {total} visibilities in {len(edits)} files")
for path, items in sorted(edits.items()):
    print(f"  {len(set(items)):4d} {os.path.relpath(path, REPO)[-110:]}")
