#!/usr/bin/env python3
"""🔧 async-test-attr — rewrites `#[test]` sitting above an `async fn` to the fully-qualified
`#[semio_framework_async_macros::async_test]`, and patches (or, in --scan, just reports) each
affected crate's `📦️packages/🦀️rust/Cargo.toml` with a path [dev-dependencies] entry on the
macro crate.

Usage:
  async-test-attr.py --scan  <root>...   # report only, JSON to stdout, touches nothing
  async-test-attr.py --apply <root>...   # rewrite .rs attributes + Cargo.toml in place

Idempotent both ways: a second run over the same roots reports/produces zero further changes,
since a rewritten site no longer matches the bare `#[test]` pattern and an already-present
dev-dependency line short-circuits the Cargo.toml insertion.

Scope note (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, packet macros-blockon):
--scan is safe repo-wide. --apply is validated ONLY over this ticket's own ⏳️async/** slice; a
repo-wide --apply is explicitly a LATER packet's job, not this script's to perform unattended.
"""
import argparse
import json
import os
import re
from pathlib import Path

#region 🔖️Config
MACRO_CRATE_DIR = Path("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust")
MACRO_CRATE_NAME = "semio-framework-async-macros"
NEW_ATTR_BASENAME = "semio_framework_async_macros::async_test"
SKIP_PATH_PARTS = ("node_modules", ".git")
SKIP_PATH_PREFIXES = ("🎯️target",)
#endregion 🔖️Config

#region 🔖️Patterns
# 🔎 An "attrs block" (consecutive single-line `#[...]`/`#![...]`/doc-comment lines) immediately
# followed by an `async fn` declaration. Multi-line attributes (rare in this repo, e.g. a
# `#[should_panic(\n  expected = "..."\n)]` split across lines) are NOT matched by this v1 —
# `--scan`'s counts are therefore a conservative lower bound, never an over-count.
ATTR_BLOCK_RE = re.compile(
    r"(?P<attrs>(?:^[ \t]*(?:#!?\[[^\]\n]*\]|///[^\n]*|//![^\n]*)[ \t]*\r?\n)*)"
    r"^(?P<indent>[ \t]*)(?P<vis>(?:pub(?:\([^)]*\))?[ \t]+)?)async[ \t]+fn[ \t]+(?P<name>[A-Za-z_][A-Za-z0-9_]*)",
    re.MULTILINE,
)
TEST_ATTR_LINE_RE = re.compile(r"^([ \t]*)#\[test\][ \t]*$")
DEV_DEPS_HEADER_RE = re.compile(r"^\[dev-dependencies\][ \t]*\r?\n", re.MULTILINE)
#endregion 🔖️Patterns

#region 🔖️Scan
def find_matches(text):
    """🔎 Yields (start, end, attrs_text, fn_name) for every attrs-block immediately preceding an
    `async fn` whose block contains a bare `#[test]` line."""
    for m in ATTR_BLOCK_RE.finditer(text):
        attrs = m.group("attrs")
        if not attrs:
            continue
        lines = attrs.splitlines(keepends=True)
        if any(TEST_ATTR_LINE_RE.match(line.rstrip("\r\n")) for line in lines):
            yield m.start("attrs"), m.end("attrs"), attrs, m.group("name")


def should_skip(path):
    parts = path.parts
    if any(part in SKIP_PATH_PARTS for part in parts):
        return True
    if any(part.startswith(prefix) for part in parts for prefix in SKIP_PATH_PREFIXES):
        return True
    return False


def find_manifest(rs_file):
    """📦 Walks up from an `.rs` file to the nearest `📦️packages/🦀️rust/Cargo.toml`. Two shapes
    both occur in this taxonomy and are checked at EVERY ancestor level, nearest first: the file
    may already live inside the `📦️packages/🦀️rust` tree itself (an ancestor is literally named
    `🦀️rust` under a `📦️packages` parent), or — the common case for an owner `🦀️component.rs` —
    the crate root is a SIBLING subtree one level down from an ancestor (`<owner>/📦️packages/🦀️rust`),
    never an ancestor of the file itself, since `📦️glue.rs` mounts the owner file via `#[path]`."""
    for parent in rs_file.parents:
        if parent.name == "🦀️rust" and parent.parent is not None and parent.parent.name == "📦️packages":
            candidate = parent / "Cargo.toml"
            if candidate.exists():
                return candidate
        sibling_candidate = parent / "📦️packages" / "🦀️rust" / "Cargo.toml"
        if sibling_candidate.exists():
            return sibling_candidate
    return None
#endregion 🔖️Scan

#region 🔖️Rewrite
def rewrite_attrs(attrs):
    out_lines = []
    for line in attrs.splitlines(keepends=True):
        body = line.rstrip("\r\n")
        trailing = line[len(body):]
        match = TEST_ATTR_LINE_RE.match(body)
        if match:
            indent = match.group(1)
            out_lines.append(f"{indent}#[{NEW_ATTR_BASENAME}]{trailing}")
        else:
            out_lines.append(line)
    return "".join(out_lines)


def relative_macro_path(manifest):
    rel = os.path.relpath(MACRO_CRATE_DIR, manifest.parent)
    return Path(rel).as_posix()


def cargo_toml_has_dep(text):
    return re.search(rf"^[ \t]*{re.escape(MACRO_CRATE_NAME)}[ \t]*=", text, re.MULTILINE) is not None


def insert_dev_dependency(text, rel_path):
    """📌 Idempotent, path-correct [dev-dependencies] insertion — adds the section if absent."""
    if cargo_toml_has_dep(text):
        return text
    dep_line = f'{MACRO_CRATE_NAME} = {{ path = "{rel_path}" }}\n'
    if DEV_DEPS_HEADER_RE.search(text):
        return DEV_DEPS_HEADER_RE.sub(lambda m: m.group(0) + dep_line, text, count=1)
    sep = "" if text.endswith("\n") else "\n"
    return text + f"{sep}\n[dev-dependencies]\n{dep_line}"


def process_file(rs_file, apply_changes):
    text = rs_file.read_text(encoding="utf-8")
    matches = list(find_matches(text))
    if not matches:
        return 0, None
    manifest = find_manifest(rs_file)
    if apply_changes:
        pieces = []
        last = 0
        for start, end, attrs, _name in matches:
            pieces.append(text[last:start])
            pieces.append(rewrite_attrs(attrs))
            last = end
        pieces.append(text[last:])
        new_text = "".join(pieces)
        if new_text != text:
            rs_file.write_text(new_text, encoding="utf-8")
    return len(matches), manifest
#endregion 🔖️Rewrite

#region 🔖️Main
def main():
    parser = argparse.ArgumentParser(description="Rewrite `#[test]` above `async fn` to `#[semio_framework_async_macros::async_test]`.")
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--scan", action="store_true", help="report only, JSON to stdout")
    mode.add_argument("--apply", action="store_true", help="rewrite files in place")
    parser.add_argument("roots", nargs="+", help="one or more directories to walk")
    args = parser.parse_args()

    per_root = {}
    manifests_needed = {}
    total_sites = 0
    total_files = 0

    for root_str in args.roots:
        root = Path(root_str).resolve()
        count_sites = 0
        count_files = 0
        for rs_file in root.rglob("*.rs"):
            if should_skip(rs_file):
                continue
            n, manifest = process_file(rs_file, apply_changes=args.apply)
            if n:
                count_sites += n
                count_files += 1
                if manifest:
                    manifests_needed.setdefault(str(manifest), set()).add(str(rs_file))
        per_root[str(root)] = {"files_with_sites": count_files, "sites": count_sites}
        total_sites += count_sites
        total_files += count_files

    manifests_touched = []
    for manifest_str in sorted(manifests_needed):
        manifest = Path(manifest_str)
        rel_path = relative_macro_path(manifest)
        if args.apply:
            text = manifest.read_text(encoding="utf-8")
            new_text = insert_dev_dependency(text, rel_path)
            if new_text != text:
                manifest.write_text(new_text, encoding="utf-8")
                manifests_touched.append(manifest_str)

    report = {
        "mode": "apply" if args.apply else "scan",
        "roots": per_root,
        "total_files_with_sites": total_files,
        "total_sites": total_sites,
        "manifests_needing_dev_dependency": sorted(manifests_needed.keys()),
        "manifests_touched": manifests_touched,
    }
    print(json.dumps(report, indent=2, ensure_ascii=False))


if __name__ == "__main__":
    main()
#endregion 🔖️Main
