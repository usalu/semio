#!/usr/bin/env python3
# 🔌 Wires every emitted 🧪️tests case into 🧱️block's 📦️glue.rs, right after that mutation's own
# `pub mod inverse;` line, at the same indentation.
import io, os, sys, re

REPO = "/Users/ueli/Documents/semio"
GLUE = os.path.join(REPO, "✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs")
ARTIFACTS = {"block5d": "🖐️5d", "block3d": "🧊️3d", "block2d": "◻2d"}

def main(artifact):
    art = ARTIFACTS[artifact]
    prefix = f"../../🗿️artifacts/{art}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
    root = os.path.join(REPO, f"✏️s/🔌️plugins/🧱️block/🗿️artifacts/{art}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations")
    lines = io.open(GLUE, encoding="utf-8").read().split("\n")
    added = 0
    for leaf in sorted(os.listdir(root)):
        tests = os.path.join(root, leaf, "🧪️tests")
        if not os.path.isdir(tests):
            continue
        inverse_path = f'#[path = "{prefix}/{leaf}/↩️inverse/🦀️component.rs"]'
        idx = next((i for i, l in enumerate(lines) if l.strip() == inverse_path), None)
        if idx is None:
            raise SystemExit(f"no glue anchor for {leaf}")
        indent = lines[idx][: len(lines[idx]) - len(lines[idx].lstrip())]
        assert lines[idx + 1].strip() == "pub mod inverse;", lines[idx + 1]
        insert_at = idx + 2
        for case in sorted(os.listdir(tests)):
            if not os.path.isdir(os.path.join(tests, case)):
                continue
            mod = "tests_" + re.sub(r"[^0-9a-zA-Z]+", "_", case)
            if any(l.strip() == f"mod {mod};" for l in lines[idx : idx + 12]):
                continue
            block = [
                f"{indent}#[cfg(test)]",
                f'{indent}#[path = "{prefix}/{leaf}/🧪️tests/{case}/🦀️component.rs"]',
                f"{indent}mod {mod};",
            ]
            lines[insert_at:insert_at] = block
            insert_at += 3
            added += 3
    io.open(GLUE, "w", encoding="utf-8").write("\n".join(lines))
    print(f"{artifact}: inserted {added // 3} test module(s)")

main(sys.argv[1])
