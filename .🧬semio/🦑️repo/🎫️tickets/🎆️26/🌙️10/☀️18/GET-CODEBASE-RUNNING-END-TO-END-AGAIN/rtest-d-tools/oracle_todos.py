#!/usr/bin/env python3
"""Independent Python model of the todos module: the todo line grammars (regular expressions, the JavaScript ones of the
TS oracle restated for Python), the rewrites over real documents, the scan of the committed tree with and without the
filesystem skip rules, comment openers and draft slugs.  Writes (or with --check verifies)
🧪️tests/🧫️fixtures/🔬️unit-vectors.json of the todos module."""
import json
import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
MODULE = ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules/📝️todos"
FIXTURES = MODULE / "🧪️tests/🧫️fixtures"
ORACLE_SCRIPT = str(Path(__file__).resolve().relative_to(ROOT))
TARGET = FIXTURES / "🔬️unit-vectors.json"
SCAN_EXTENSIONS = {".ts", ".js", ".tsx", ".jsx", ".go", ".cs", ".py", ".md", ".json"}
SKIPPED = {"node_modules", "dist", "build"}


def simple_upper(text):
    return "".join(c.upper() if len(c.upper()) == 1 else c for c in text)


def slugify(text):
    spaced = re.sub(r"(?<=[a-z])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])", "-", text)
    return re.sub(r"[^A-Z0-9]+", "-", simple_upper(spaced)).strip("-")


MARKDOWN = re.compile(r"^-\sTODO\s([^:]*?)(?::(.*))?$")
COMMENT = re.compile(r"^(\s*(?:\/\/|#|--)\s*TODO\s+)([^:]*?):(.*)$")


def markdown_items(content, parent):
    found = []
    for line in content.split("\n"):
        stripped = line.strip()
        if not stripped.startswith("- TODO "):
            continue
        rest = stripped[len("- TODO "):]
        name, _, description = rest.partition(":")
        has_colon = ":" in rest
        found.append({"id": slugify(name.strip()), "name": name.strip(), "description": description.strip() if has_colon else "", "parent": parent, "file": (parent + "/" if parent else "") + ".todos.md", "line": 0, "column": 0})
    return found


def comment_parts(line):
    match = COMMENT.match(line)
    if not match:
        return None
    after = line[len(match.group(1)):]
    if after.strip() == "":
        return None
    return [match.group(1), match.group(2).strip(), match.group(3).strip()]


def comments(content, path):
    found = []
    for number, line in enumerate(content.split("\n"), 1):
        parts = comment_parts(line)
        if parts:
            found.append({"id": slugify(parts[1]), "name": parts[1], "description": parts[2], "parent": path, "file": path, "line": number, "column": 1})
    return found


def extension(path):
    name = re.split(r"[/\\]", path)[-1]
    index = name.rfind(".")
    return name[index:] if index > 0 else ""


def replace_markdown(document, old, new, description):
    prefix = f"- TODO {old}:"
    lines = document.split("\n")
    for index, line in enumerate(lines):
        if line.lstrip().startswith(prefix):
            lines[index] = f"- TODO {new}: {description}"
            return {"ok": "\n".join(lines)}
    return {"err": "not-found"}


def remove_markdown(document, name):
    prefix = f"- TODO {name}:"
    return "\n".join(line for line in document.split("\n") if not line.lstrip().startswith(prefix))


def replace_file(document, number, name, description):
    lines = document.split("\n")
    if number <= 0 or number > len(lines):
        return {"err": "no-location"}
    parts = comment_parts(lines[number - 1])
    if not parts:
        return {"err": "not-found"}
    lines[number - 1] = f"{parts[0]}{name}: {description}"
    return {"ok": "\n".join(lines)}


def remove_file(document, number):
    lines = document.split("\n")
    if 0 < number <= len(lines):
        del lines[number - 1]
    return "\n".join(lines)


def opener(path):
    return {".py": "#", ".sh": "#", ".yaml": "#", ".yml": "#", ".sql": "--", ".lua": "--"}.get(extension(path), "//")


def skipped(segment):
    return segment in SKIPPED or (segment.startswith(".") and segment != ".🧬semio")


def entries(files, skip):
    dirs = {""}
    for item in files:
        parts = item["path"].split("/")
        for i in range(1, len(parts)):
            dirs.add("/".join(parts[:i]))
    rows = [(d, True) for d in dirs] + [(item["path"], False) for item in files]
    rows.sort(key=lambda r: (r[0], not r[1]))
    if skip:
        def kept(row):
            segments = [seg for seg in row[0].split("/") if seg]
            directories = segments if row[1] else segments[:-1]
            return not any(skipped(seg) for seg in directories)
        rows = [row for row in rows if kept(row)]
    return rows


def scan(files, skip):
    contents = {item["path"]: item["content"] for item in files}
    found = []
    for path, is_dir in entries(files, skip):
        if is_dir:
            markdown = (path + "/" if path else "") + ".todos.md"
            if markdown in contents:
                found += markdown_items(contents[markdown], path)
            continue
        if path.endswith(".todos.md"):
            continue
        if extension(path) in SCAN_EXTENSIONS:
            found += comments(contents[path], path)
    return found


def build():
    vectors = json.loads((FIXTURES / "📝️line-vectors.json").read_text(encoding="utf8"))
    tree = json.loads((FIXTURES / "🔍️scan-tree.json").read_text(encoding="utf8"))
    parent = vectors["markdownPath"].rsplit("/", 1)[0]
    markdown, source = vectors["markdown"], vectors["source"]
    extra_lines = [
        "// TODO", "// TODO ", "// TODO : empty name", "//TODO Tight: no space before TODO", "//\tTODO Tab: tab after the opener", "\t# TODO Tab Indent: x", "/// TODO Triple: the third slash is part of the name",
        "// todo Lower: the keyword is upper case", "x // TODO Trailing: not at the start", "// TODO A: b: c: d", "# TODO Hash:", "-- TODO Dash: ünï cödé", "// TODO   Spaces   Inside  :  padded  ", "/* TODO Block: no */", "TODO Bare: no opener", "  --TODO Dash2: ok",
    ]
    return {
        "schema": "semio.repo.todos.unit-vectors/1",
        "$comment": "🔬️ Expected answers for the cargo unit tests, computed by an independent Python model (the JavaScript grammars of the TS oracle restated as Python regular expressions, plus string methods) over the real line vectors and the committed scan tree. `scan.memory` follows no skip rule, `scan.filesystem` skips node_modules, dist, build and dotted directories. Regenerate with the model; never edit by hand.",
        "oracle": {"script": ORACLE_SCRIPT, "tool": "python3", "libraries": ["re", "json"], "model": "todo line grammars, rewrites, scan, comment openers, slugs"},
        "markdownParse": markdown_items(markdown, parent),
        "sourceParse": comments(source, vectors["sourcePath"]),
        "markdownRewrites": [dict(r, result=replace_markdown(markdown, r["oldName"], r["newName"], r["newDescription"]), removed=remove_markdown(markdown, r["oldName"])) for r in vectors["markdownRewrites"]],
        "sourceRewrites": [dict(r, result=replace_file(source, r["line"], r["newName"], r["newDescription"]), removed=remove_file(source, r["line"])) for r in vectors["sourceRewrites"]],
        "commentLines": [{"line": line, "parts": comment_parts(line)} for line in source.split("\n") + extra_lines],
        "openers": [{"path": p, "opener": opener(p)} for p in vectors["openerPaths"]],
        "scan": {"memory": scan(tree["files"], False), "filesystem": scan(tree["files"], True)},
        "slugs": [{"title": t, "slug": slugify(t)} for t in ["Sketch Of The Tree", "sketch of the tree", "///", "", "  Spaces  ", "ÜberCool Draft", "XMLParser 2", "a", "Ünï"]],
    }


def main():
    text = json.dumps(build(), ensure_ascii=False, indent=1) + "\n"
    if "--check" in sys.argv:
        same = TARGET.exists() and TARGET.read_text(encoding="utf8") == text
        print("SAME" if same else "DIFFERENT")
        sys.exit(0 if same else 1)
    TARGET.write_text(text, encoding="utf8")
    print("written", TARGET, len(text))


main()
