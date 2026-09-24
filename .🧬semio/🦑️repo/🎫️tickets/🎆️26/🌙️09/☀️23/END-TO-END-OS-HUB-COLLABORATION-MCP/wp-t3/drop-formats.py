"""WP-T3 helper: withdraw stdio format declarations from one artifact, schema-first and everywhere at once.

usage: python3 drop-formats.py <artifact-dir> [--import-only] <fmt> [<fmt> ...]
For each format it deletes the io leaves (export and import, or import only), their module
declarations in the crate root, the io registry's DEP/EXPORT dialects, compose functions, entries and
kind lists, and the artifact root's composer capability rows and ArtifactKindSpec kind lists.
"""
import os
import re
import shutil
import sys

sys.path.insert(0, os.path.dirname(__file__))

EXPORT = "📤️export/🧵️serializers/🗿️artifacts"
IMPORT = "📥️import/🧩️deserializers/🗿️artifacts"


def remove_block(text: str, leaf: str) -> str:
    lines = text.split("\n")
    target = f'#[path = "{leaf}/🦀️.rs"]'
    hits = [i for i, line in enumerate(lines) if line.strip() == target]
    if len(hits) != 1:
        raise SystemExit(f"expected one declaration of {leaf}, found {len(hits)}")
    at = hits[0]
    start, end = at - 6, at + 5
    indent = lines[start + 1][: len(lines[start + 1]) - len(lines[start + 1].lstrip())]
    ok = lines[start].strip() == '#[path = "."]' and lines[start + 1].strip().startswith("pub mod ") and lines[end] == indent + "}"
    if not ok or not lines[at - 1].strip().startswith("pub mod "):
        raise SystemExit(f"unexpected block shape around {leaf}")
    return "\n".join(lines[:start] + lines[end + 1 :])


def brace_end(text: str, start: int) -> int:
    depth, i, opened = 0, start, False
    while True:
        if text[i] == "{":
            depth += 1
            opened = True
        elif text[i] == "}":
            depth -= 1
            if opened and depth == 0:
                return i
        i += 1


def main() -> None:
    args = sys.argv[1:]
    artifact = args.pop(0)
    import_only = "--import-only" in args
    formats = [a for a in args if not a.startswith("--")]
    root = os.path.join(artifact, "🦀️.rs")
    subsets = [os.path.join(dp) for dp, dn, _ in os.walk(os.path.join(artifact, "🏅️standards")) if dp.endswith("🚪️io") and "🧪️tests" not in dp]
    root_text = open(root, encoding="utf8").read()
    for io_dir in subsets:
        rel_io = os.path.relpath(io_dir, artifact)
        io_path = os.path.join(io_dir, "🦀️.rs")
        io_text = open(io_path, encoding="utf8").read()
        for fmt in formats:
            for tree in ([IMPORT] if import_only else [EXPORT, IMPORT]):
                base = os.path.join(io_dir, tree)
                if not os.path.isdir(base):
                    continue
                for name in os.listdir(base):
                    if not re.fullmatch(rf"[^a-z0-9]*{fmt}", name):
                        continue
                    for version in os.listdir(os.path.join(base, name)):
                        for profile in os.listdir(os.path.join(base, name, version)):
                            leaf = os.path.join(rel_io, tree, name, version, profile)
                            root_text = remove_block(root_text, leaf)
                            shutil.rmtree(os.path.join(artifact, leaf))
                            print(f"removed {leaf}")
                    shutil.rmtree(os.path.join(base, name))
            upper = fmt.upper()
            for const in re.findall(rf"const (DEP_[A-Z0-9_]+): Dialect = Dialect {{ artifact_kind: \"s\.stdio\.{fmt}\"", io_text):
                io_text = re.sub(rf"^\s*const {const}: Dialect = .*\n", "", io_text, flags=re.M)
                io_text = re.sub(rf", {const}\b", "", io_text)
                at = io_text.find(f"if source.dialect == {const} {{")
                if at != -1:
                    line_start = io_text.rfind("\n", 0, at) + 1
                    io_text = io_text[:line_start] + io_text[io_text.find("\n", brace_end(io_text, at)) + 1 :]
            if not import_only:
                io_text = re.sub(rf"^\s*const EXPORT_{upper}_DIALECT: Dialect = .*\n", "", io_text, flags=re.M)
                at = io_text.find(f"    fn compose_export_{fmt}(")
                if at != -1:
                    io_text = io_text[:at] + io_text[io_text.find("\n", brace_end(io_text, at)) + 1 :]
                io_text = re.sub(rf"^\s*ComposerEntry {{ writes: EXPORT_{upper}_DIALECT, .*\n", "", io_text, flags=re.M)
            for tree_word in (["import"] if import_only else ["export", "import"]):
                io_text = re.sub(rf"^\s*(?:de)?serializer_entry::<[^\n]*\b{tree_word}::{fmt}::[^\n]*\n", "", io_text, flags=re.M)
            for direction in (["import"] if import_only else ["import", "export"]):
                io_text = re.sub(rf"(pub fn {direction}_stdio_kinds\(\) -> &'static \[&'static str\] \{{\n    &\[)([^\]]*)(\])", lambda m: m.group(1) + ", ".join(k for k in [x.strip() for x in m.group(2).split(",")] if k and k != f'"stdio.{fmt}"') + m.group(3), io_text)
        open(io_path, "w", encoding="utf8").write(io_text)
    for fmt in formats:
        if not import_only:
            root_text = re.sub(rf'^\s*\("[^"]+", "composer", "s\.stdio\.{fmt}@[^"]*", .*\),\n', "", root_text, flags=re.M)
            while True:
                m = re.search(rf'\n\s*\.capability\(\s*\n\s*ArtifactCapability::new\(ArtifactIdentity::parse\("[^"]*composer\.{fmt}"\)\?', root_text)
                if not m:
                    break
                end = root_text.find(")?\n", brace_paren_end(root_text, m.start() + root_text[m.start():].find(".capability(") + len(".capability")))
                root_text = root_text[: m.start()] + root_text[end + 2 :]
        for direction in (["import"] if import_only else ["import", "export"]):
            root_text = re.sub(rf'({direction}_stdio_kinds: vec!\[[^\]]*?)"stdio\.{fmt}"\.into\(\)(, )?', lambda m: m.group(1), root_text)
            root_text = re.sub(rf"({direction}_stdio_kinds: vec!\[[^\]]*?), \]", r"\1]", root_text)
    open(root, "w", encoding="utf8").write(root_text)


def brace_paren_end(text: str, open_at: int) -> int:
    depth, i = 0, open_at
    while True:
        if text[i] == "(":
            depth += 1
        elif text[i] == ")":
            depth -= 1
            if depth == 0:
                return i
        i += 1


main()
