#!/usr/bin/env python3
"""🩹️ S3-STDIO: moves every generic stdio snapshot editor from whole-snapshot `SetSnapshot` emission to the path-scoped
`patch-snapshot` leaf (design §20.3), keeping `SetSnapshot` only as the `ReplaceSource` (whole-source replacement) branch.

Usage (repo root): python3 T/🧪️s3-stdio-convert-editors.py [--apply]

Rewrites `editing::snapshot_edit_set_snapshot(event, snapshot, |snapshot| E::SetSnapshot(P::SetSnapshot { .. }))` into
`editing::snapshot_edit_patch(event, snapshot, |patch| E::PatchSnapshot(Q::PatchSnapshot { patch }), |snapshot| E::SetSnapshot(..))`
and imports the sibling `patch_snapshot` module next to the `set_snapshot` import. Idempotent; prints MANUAL for any site it
cannot resolve.
"""
import os, re, subprocess, sys

ROOT = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
TEXT = ("📝️md/", "🌐️html/", "🔤️txt/")
CALL = re.compile(r"snapshot_edit_set_snapshot\(event, snapshot, \|snapshot\| (\w+)::SetSnapshot\(((?:[\w:]*::)?)SetSnapshot \{ ([^}]*) \}\)\)")


def import_patch_module(text, module_path):
    """Adds `patch_snapshot` beside the file's import of `set_snapshot` (plain, aliased or grouped)."""
    if re.search(r"^use [^;]*\bpatch_snapshot\b", text, re.M):
        return text, True
    grouped = re.search(r"^(use [^;{]*\{)([^;]*?)\bset_snapshot( as snapshot_edit_set_snapshot)?\b", text, re.M)
    if grouped:
        start = grouped.start(2)
        return text[:start] + "patch_snapshot, " + text[start:], True
    single = re.search(r"^use ([^;{]*)::set_snapshot( as snapshot_edit_set_snapshot)?;\n", text, re.M)
    if single:
        return text[:single.end()] + f"use {single.group(1)}::patch_snapshot;\n" + text[single.end():], True
    return text, False


def main():
    apply = "--apply" in sys.argv
    files = subprocess.run(["git", "grep", "-l", "-z", "snapshot_edit_set_snapshot(event, snapshot,", "--", ROOT], capture_output=True).stdout.decode().split("\0")
    for path in sorted(file for file in files if file and not any(text in file for text in TEXT)):
        text = open(path, encoding="utf-8").read()
        call = CALL.search(text)
        if call is None:
            print(f"MANUAL call shape: {path}")
            continue
        enum, prefix, body = call.groups()
        if prefix == "":
            print(f"MANUAL bare SetSnapshot import: {path}")
            continue
        if prefix.startswith("crate::") or prefix.startswith("super::"):
            patch_prefix = prefix[: -len("set_snapshot::")] + "patch_snapshot::"
            imported = True
        else:
            patch_prefix = "patch_snapshot::"
            text, imported = import_patch_module(text, prefix)
        if not imported:
            print(f"MANUAL patch_snapshot import: {path}")
            continue
        call = CALL.search(text)
        replacement = (f"snapshot_edit_patch(event, snapshot, |patch| {enum}::PatchSnapshot({patch_prefix}PatchSnapshot {{ patch }}), "
                       f"|snapshot| {enum}::SetSnapshot({prefix}SetSnapshot {{ {body} }}))")
        text = text[:call.start()] + replacement + text[call.end():]
        if apply:
            with open(path, "w", encoding="utf-8") as handle:
                handle.write(text)
        print(("converted " if apply else "would convert ") + path)


if __name__ == "__main__":
    main()
