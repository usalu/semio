#!/usr/bin/env python3
"""🌉️ W3-STDIO-CASES item 6: retires every stdio artifact's per-aggregate test-reachability bridge
(`decode_<x>_mutation_payload[_json]`, `inverse_<x>_mutation`) in favour of the shared stdio contract bridges
(`mutation_from_payload_json`, `mutation_payload_json`, `apply_mutation_checked`, `mutation_inverse`, `MutationRefusal`),
which every artifact crate root re-exports. `🧿️semio` is excluded: its bridges have production consumers (gis inferences,
intra-semio composition). Idempotent. Usage: `python3 🧪️w3-stdio-bridges.py [--dry-run]`."""

import re
import subprocess
import sys

ROOT = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
EXCLUDE = ("🧿️semio",)
REEXPORT = "pub use semio_s_artifact_stdio_contract::{apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, MutationRefusal};"
STALE_REEXPORTS = (
    "pub use protocol::json::{from_json_str, to_json_string};\n",
    "pub use protocol::{DslValue, Mutation};\n",
    "pub use semio_s_artifact_stdio_contract::{apply_mutation_checked, mutation_from_payload_json, mutation_inverse, part21};\n",
)
ANCHOR = "extern crate semio_framework_value_derive as value_derive;\n"
DRY = "--dry-run" in sys.argv


def git_lines(*args: str) -> list[str]:
    return subprocess.run(["git", *args], capture_output=True, text=True, check=False).stdout.splitlines()


def write(path: str, text: str, original: str) -> None:
    if text != original:
        print(f"[w3-stdio] {'would edit' if DRY else 'edit'} {path}")
        if not DRY:
            open(path, "w", encoding="utf-8").write(text)


def bridges() -> dict[str, set[str]]:
    found: dict[str, set[str]] = {}
    for line in git_lines("grep", "-nE", r"pub fn (decode_[a-z0-9_]+_mutation_payload(_json)?|inverse_[a-z0-9_]+_mutation)\(", "--", ROOT):
        path, _, rest = line.split(":", 2)
        if any(part in path for part in EXCLUDE):
            continue
        found.setdefault(re.search(r"pub fn ([a-z0-9_]+)", rest).group(1), set()).add(path)
    return found


def drop_item(text: str, name: str) -> str:
    lines = text.split("\n")
    start = next((i for i, line in enumerate(lines) if re.match(rf"\s*pub fn {name}\(", line)), None)
    if start is None:
        return text
    head = start
    while head > 0 and re.match(r"\s*(///|// 🚫️async|#\[)", lines[head - 1]):
        head -= 1
    depth, end, opened = 0, start, False
    for index in range(start, len(lines)):
        depth += lines[index].count("{") - lines[index].count("}")
        opened = opened or "{" in lines[index]
        if opened and depth == 0:
            end = index
            break
    if head > 0 and lines[head - 1].strip() == "" and end + 1 < len(lines) and lines[end + 1].strip() == "":
        end += 1
    return "\n".join(lines[:head] + lines[end + 1:])


def drop_empty_reachability(text: str) -> str:
    return re.sub(r"\n?//#region 🚪️Reachability\n(\s*\n)*//#endregion 🚪️Reachability\n", "\n", text)


def drop_from_use_lists(text: str, names: set[str]) -> str:
    def braces(match: re.Match) -> str:
        kept = [item.strip() for item in match.group(2).split(",") if item.strip() and item.strip() not in names]
        if not kept:
            return ""
        return f"{match.group(1)}{{{', '.join(kept)}}};\n" if len(kept) > 1 else f"{match.group(1)}{kept[0]};\n"
    return re.sub(r"(pub use [A-Za-z0-9_:]+::)\{([^{}]*)\};\n", braces, text)


def main() -> None:
    found = bridges()
    names = set(found)
    for name, paths in sorted(found.items()):
        for path in sorted(paths):
            original = open(path, encoding="utf-8").read()
            write(path, drop_empty_reachability(drop_item(original, name)), original)
    for path in git_lines("grep", "-lE", "pub use [A-Za-z0-9_:]+::\\{[^}]*(decode_[a-z0-9_]+_mutation_payload|inverse_[a-z0-9_]+_mutation)", "--", ROOT):
        if any(part in path for part in EXCLUDE):
            continue
        original = open(path, encoding="utf-8").read()
        write(path, drop_from_use_lists(original, names), original)
    for path in git_lines("ls-files", f"{ROOT}/*/🦀️.rs"):
        if path.count("/") != ROOT.count("/") + 2 or any(part in path for part in EXCLUDE):
            continue
        original = open(path, encoding="utf-8").read()
        text = original
        for stale in STALE_REEXPORTS:
            text = text.replace(stale, "")
        if REEXPORT not in text:
            if "pub use semio_s_artifact_stdio_contract::{apply_mutation_checked, mutation_from_payload_json, mutation_inverse, part21};" in original:
                text = text.replace(ANCHOR, f"{ANCHOR}\n{REEXPORT}\npub use semio_s_artifact_stdio_contract::part21;\n", 1)
            else:
                text = text.replace(ANCHOR, f"{ANCHOR}\n{REEXPORT}\n", 1)
        write(path, text, original)


if __name__ == "__main__":
    main()
