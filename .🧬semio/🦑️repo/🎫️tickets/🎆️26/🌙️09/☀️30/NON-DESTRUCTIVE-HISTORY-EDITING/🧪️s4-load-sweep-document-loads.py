#!/usr/bin/env python3
"""🛬️ S4-LOAD W2A-6 wave 1: moves every `✏️s` test call of the synchronous `PluginApp::load_document_pack/text` onto the
stepped law helper `semio_framework_plugin::artifact_app_laws::load_document[_text]` (admit → poll → acknowledge,
`📓️api-stepped-document-load.md` §4).

Idempotent and region-safe: only `<receiver>.load_document_pack(` / `<receiver>.load_document_text(` call expressions in
`.rs` files under `✏️s/` are rewritten; nothing else in a file changes. A receiver that is a `&mut` parameter of the
enclosing function is passed as is (implicit reborrow + deref coercion), any other receiver as `&mut <receiver>`.

Wave 1b: `consume_media` answers `MediaConsumption` (a whole document becomes a stepped archive load), so a test that matches
its result as `Ok(()) =>` matches `Ok(_) =>`, which compiles against both signatures.

Usage: `python3 🧪️s4-load-sweep-document-loads.py [--check]` from anywhere; `--check` lists the pending rewrites and exits 1
when any remain.
"""

import pathlib
import re
import subprocess
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
SCOPE = ROOT / "✏️s"
CALL = re.compile(r"(?<![\w:.])(?P<recv>[A-Za-z_][A-Za-z0-9_]*(?:\.[0-9]+)?)\.load_document_(?P<kind>pack|text)\(")
LITERAL = re.compile(r'"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])\'|//[^\n]*')
FN_HEAD = re.compile(r"\bfn\s+\w+[^;{}]*$|\|[^|;{}]*\|\s*(?:->[^{};]*)?$")
HELPER = "semio_framework_plugin::artifact_app_laws::load_document"
CONSUMED = re.compile(r"(match [^\n]*consume_media\([^\n]*\{[ \t]*\n(?:[^\n]*\n){0,2}?[ \t]*)Ok\(\(\)\)([ \t]*=>)")


def candidate_files():
    listed = subprocess.run(["rg", "-l", "-g", "*.rs", r"\.load_document_(pack|text)\(|consume_media\(", str(SCOPE)], capture_output=True, text=True)
    return sorted(pathlib.Path(line) for line in listed.stdout.splitlines() if line)


def receiver_is_mut_param(lines, index, column, receiver):
    if "." in receiver:
        return False
    before = LITERAL.sub(lambda match: " " * len(match.group(0)), "".join(lines[:index]) + lines[index][:column])
    depth = 0
    for position in range(len(before) - 1, -1, -1):
        char = before[position]
        if char == "}":
            depth += 1
        elif char == "{":
            if depth:
                depth -= 1
                continue
            header = before[max(0, position - 600):position]
            if FN_HEAD.search(header):
                parameter = re.search(rf"\b{re.escape(receiver)}\s*:\s*([^,)|]*)", header.rsplit("fn ", 1)[-1] if "fn " in header else header)
                if parameter:
                    return parameter.group(1).lstrip().startswith("&mut")
                if re.search(rf"\blet\s+(?:mut\s+)?{re.escape(receiver)}\b", before[position:]):
                    return False
    return False


def rewrite(path):
    lines = path.read_text(encoding="utf-8").splitlines(keepends=True)
    changes = []
    for index, line in enumerate(lines):
        if ".load_document_" not in line:
            continue

        def replace(match):
            receiver = match.group("recv")
            suffix = "_text" if match.group("kind") == "text" else ""
            argument = receiver if receiver_is_mut_param(lines, index, match.start(), receiver) else f"&mut {receiver}"
            return f"{HELPER}{suffix}({argument}, "

        updated = CALL.sub(replace, line)
        if updated != line:
            changes.append((index + 1, line.strip(), updated.strip()))
            lines[index] = updated
    text = "".join(lines)
    for match in CONSUMED.finditer(text):
        changes.append((text.count("\n", 0, match.end(1)) + 1, "Ok(()) =>", "Ok(_) =>"))
    return CONSUMED.sub(r"\1Ok(_)\2", text), changes


def main():
    check = "--check" in sys.argv
    pending = 0
    for path in candidate_files():
        text, changes = rewrite(path)
        if not changes:
            continue
        pending += len(changes)
        print(f"{path.relative_to(ROOT)}: {len(changes)}")
        for number, before, after in changes:
            print(f"  {number}: {before[:140]}\n  {' ' * len(str(number))}→ {after[:140]}")
        if not check:
            path.write_text(text, encoding="utf-8")
    print(f"{'pending' if check else 'rewritten'}: {pending}")
    return 1 if check and pending else 0


if __name__ == "__main__":
    sys.exit(main())
