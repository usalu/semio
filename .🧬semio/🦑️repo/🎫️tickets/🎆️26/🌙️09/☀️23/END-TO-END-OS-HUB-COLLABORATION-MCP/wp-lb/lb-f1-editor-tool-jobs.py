#!/usr/bin/env python3
"""🧰️ LB-F1 (session 13): 52 stdio editors outside the shipped component carried their `ArtifactEditor` tool-job items
(`bounded_first_step_tool_proofs!`, `register_tool_job_factories`, `build_tool_job`,
`build_document_store_initialization_job`, `command_id`, `command_from_action`) INSIDE
`impl ArtifactOwnedToolJobFactory for <X>EditorExampleFactory`, so `semio-s-plugin-stdio --features full-app-catalog`
did not compile. This moves those items into the editor's own `impl ArtifactEditor for <X>` right after its
`const DOCUMENT_SCHEMA` (the txt editor's layout); the factory impl keeps `Owner`, `TOOL_IDS`, `DOCUMENT_SCHEMA` and
`PUBLICATION_CONTRACTS`. Every crate whose editor names `semio_framework_job` gains the workspace dependency.
Usage: lb-f1-editor-tool-jobs.py --dry-run | --write"""
import os
import re
import sys

ROOT = "/Users/ueli/Documents/semio"
STDIO = f"{ROOT}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
write = "--write" in sys.argv
if not write and "--dry-run" not in sys.argv:
    sys.exit(__doc__)
KEEP = re.compile(r"^    (type Owner|const TOOL_IDS|const DOCUMENT_SCHEMA|const PUBLICATION_CONTRACTS)\b")
edits, problems, crates = {}, [], set()


def block_end(lines, start):
    """🧱️ Index of the line that closes the brace block opened on `lines[start]` (braces counted outside strings)."""
    depth = 0
    for index in range(start, len(lines)):
        stripped = re.sub(r'"(?:\\.|[^"\\])*"', '""', lines[index])
        depth += stripped.count("{") - stripped.count("}")
        if depth == 0:
            return index
    raise ValueError(f"unclosed block at line {start + 1}")


def item_end(lines, start):
    """🔚️ Index of the last line of the impl item starting at `lines[start]`: its brace block's closing line, or the
    line ending in `;` when the item closes before any brace opens."""
    depth, opened = 0, False
    for index in range(start, len(lines)):
        stripped = re.sub(r'"(?:\\.|[^"\\])*"', '""', lines[index])
        depth += stripped.count("{") - stripped.count("}")
        opened = opened or "{" in stripped
        if (opened and depth == 0) or (not opened and stripped.rstrip().endswith(";")):
            return index
    raise ValueError(f"unclosed item at line {start + 1}")


for folder, dirs, files in os.walk(STDIO):
    dirs[:] = [d for d in dirs if d not in ("target", "node_modules", "dist", "🧪️tests")]
    if "🦀️.rs" not in files:
        continue
    path = os.path.join(folder, "🦀️.rs")
    source = open(path, encoding="utf-8").read()
    lines = source.split("\n")
    heads = [i for i, line in enumerate(lines) if re.match(r"^impl ArtifactOwnedToolJobFactory for \w+ \{$", line)]
    for head in heads:
        end = block_end(lines, head)
        body = lines[head + 1:end]
        if not any("bounded_first_step_tool_proofs!" in line or "fn register_tool_job_factories" in line for line in body):
            continue
        owner = re.search(r"type Owner = EditorApp<(\w+)>;", "\n".join(body))
        if owner is None:
            problems.append(f"{path}: no EditorApp owner in the factory impl")
            continue
        keep, moved, at = [], [], 0
        while at < len(body):
            line = body[at]
            if KEEP.match(line):
                keep.append(line)
                at += 1
            elif line.startswith("    ") and not line.startswith("     "):
                stop = item_end(body, at)
                moved.extend(body[at:stop + 1])
                at = stop + 1
            elif line.strip() == "":
                if moved and moved[-1] != "":
                    moved.append("")
                at += 1
            else:
                problems.append(f"{path}: unexpected line {head + at + 2}: {line[:80]!r}")
                break
        while moved and moved[-1] == "":
            moved.pop()
        editor = owner.group(1)
        editor_head = next((i for i, line in enumerate(lines) if line == f"impl ArtifactEditor for {editor} {{"), None)
        if editor_head is None:
            problems.append(f"{path}: impl ArtifactEditor for {editor} not found")
            continue
        editor_end = block_end(lines, editor_head)
        editor_body = "\n".join(lines[editor_head:editor_end])
        for name in ("register_tool_job_factories", "build_tool_job", "build_document_store_initialization_job", "command_id", "command_from_action"):
            if re.search(rf"\bfn {name}\(", editor_body):
                problems.append(f"{path}: {editor} already defines {name}")
        anchor = next((i for i in range(editor_head, editor_end) if lines[i].startswith("    const DOCUMENT_SCHEMA:")), None)
        if anchor is None:
            problems.append(f"{path}: {editor} has no const DOCUMENT_SCHEMA anchor")
            continue
        lines = lines[:head + 1] + keep + lines[end:anchor + 1] + [""] + moved + lines[anchor + 1:]
        edits[path] = "\n".join(lines)
        crates.add(path.split("/🏅️standards/")[0])
manifests = []
for crate in sorted(crates):
    manifest = f"{crate}/📦️packages/🦀️rust/Cargo.toml"
    text = open(manifest, encoding="utf-8").read()
    if "semio-framework-job" in text:
        continue
    anchor = re.search(r"^semio-framework-plugin = .*$", text, re.M)
    if anchor is None:
        problems.append(f"{manifest}: no semio-framework-plugin dependency line to anchor on")
        continue
    edits[manifest] = text[:anchor.end()] + "\nsemio-framework-job = { workspace = true }" + text[anchor.end():]
    manifests.append(manifest)
print(f"editors={len(edits) - len(manifests)} manifests={len(manifests)} crates={len(crates)} problems={len(problems)} write={write}")
for problem in problems:
    print("PROBLEM", problem.replace(ROOT + "/", ""))
if write and not problems:
    for path, text in edits.items():
        open(path, "w", encoding="utf-8").write(text)
