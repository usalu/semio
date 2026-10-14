#!/usr/bin/env python3
"""📂️ Turns compile-time `include_str!` data embeds of test sources into runtime reads relative to the owning crate's manifest.

A test that embeds its committed fixtures (or the source text it inspects) makes every byte of that data part of its
compilation unit. Reading it at runtime instead keeps the unit to authored code. For each file:

- `const NAME: &str = include_str!("rel");` becomes a `static NAME: LazyLock<String>` read at first use, and every other
  use of `NAME` in the file becomes `NAME.as_str()`;
- `let name = include_str!("rel");` binds `&str` borrowed from the read `String`, whose temporary the `let` extends;
- every other `include_str!("rel")` expression becomes `std::fs::read_to_string(…).expect(…).as_str()`, a temporary that
  lives to the end of its statement.

Only embeds whose extension is in `--ext` (default json, rs) are rewritten; grammar, proto and other compiler-owned inputs
stay embedded. The runtime path is `concat!(env!("CARGO_MANIFEST_DIR"), "/<path relative to the manifest dir>")`.

usage: 📂️runtime-fixtures.py <manifest dir> [--ext json,rs] [--apply] files...
"""
import os, re, sys

args = sys.argv[1:]
apply = "--apply" in args
args = [a for a in args if a != "--apply"]
exts = ("json", "rs")
if "--ext" in args:
    i = args.index("--ext")
    exts = tuple(args[i + 1].split(","))
    del args[i:i + 2]
manifest_dir, files = os.path.abspath(args[0]), [os.path.abspath(f) for f in args[1:]]

CONST = re.compile(r'(?m)^(?P<indent>[ \t]*)(?P<vis>pub(?:\([^)]*\))? )?const (?P<name>[A-Z][A-Z0-9_]*): &(?:\'static )?str = include_str!\("(?P<rel>[^"]+)"\);')
INLINE = re.compile(r'(?P<let>let\s+(?:mut\s+)?\w+(?:\s*:\s*&(?:\'static\s+)?str)?\s*=\s*)?include_str!\("(?P<rel>[^"]+)"\)(?P<end>\s*;)?')


def runtime(path, rel):
    target = os.path.normpath(os.path.join(os.path.dirname(path), rel))
    if not os.path.exists(target):
        raise SystemExit(f"missing embed {target} from {path}")
    return 'concat!(env!("CARGO_MANIFEST_DIR"), "/' + os.path.relpath(target, manifest_dir) + '")'


def wanted(rel):
    return rel.rsplit(".", 1)[-1] in exts


total = 0
for path in files:
    text = open(path, encoding="utf-8").read()
    names = []

    def const(m):
        if not wanted(m["rel"]):
            return m[0]
        names.append(m["name"])
        return f'{m["indent"]}{m["vis"] or ""}static {m["name"]}: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| std::fs::read_to_string({runtime(path, m["rel"])}).expect("committed test data reads"));'

    text = CONST.sub(const, text)
    for name in names:
        decl = re.compile(r'static ' + name + r': std::sync::LazyLock')
        out, last = [], 0
        for m in re.finditer(r'(?<![\w.:"])' + name + r'(?![\w])', text):
            line_start = text.rfind("\n", 0, m.start()) + 1
            if decl.search(text, line_start, m.end() + 30) and text.startswith("static " + name, m.start() - 7):
                continue
            if text.startswith(".as_str()", m.end()):
                continue
            out.append(text[last:m.end()] + ".as_str()")
            last = m.end()
        text = "".join(out) + text[last:]
    count = len(names)

    def inline(m):
        global count
        if not wanted(m["rel"]):
            return m[0]
        count += 1
        read = f'std::fs::read_to_string({runtime(path, m["rel"])}).expect("committed test data reads")'
        if m["let"] and m["end"]:
            return re.sub(r'(\w+)(?:\s*:\s*&(?:\'static\s+)?str)?(\s*=\s*)$', r'\1: &str\2&', m["let"]) + read + m["end"]
        return (m["let"] or "") + read + ".as_str()" + (m["end"] or "")

    text = INLINE.sub(inline, text)
    total += count
    print(f"{count:4d} {os.path.relpath(path)}")
    if apply and count:
        open(path, "w", encoding="utf-8").write(text)
print(f"total {total}")
