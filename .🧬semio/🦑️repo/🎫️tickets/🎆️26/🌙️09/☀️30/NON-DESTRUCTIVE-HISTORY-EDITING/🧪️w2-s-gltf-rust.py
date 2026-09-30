#!/usr/bin/env python3
"""🦀️ W2-S glTF Rust cleanup: a `MutationLeaf` is the phase enum the aggregate wraps, never its payload or rejection struct;
the unmounted per-case leaf tests (superseded by the mounted `🧪️fixture-corpus` law over the same corpus) are removed.

Run from the repository root: `.venv/bin/python <ticket>/🧪️w2-s-gltf-rust.py [--write]`.
"""
import re
import shutil
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
LEAVES = ROOT / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations"
WRITE = "--write" in sys.argv
SPURIOUS = re.compile(r"(#\[derive\([^)]*?), dsl::MutationLeaf\)\]\n#\[mutation_leaf\(contract = ::protocol\)\]\n((?:#\[[^\n]*\]\n)*pub struct (\w+))")

stripped = []
for source in sorted(LEAVES.glob("*/*/🦀️.rs")):
    text = source.read_text()
    if "Apply(" not in text:
        continue
    updated = SPURIOUS.sub(lambda match: f"{match.group(1)})]\n{match.group(2)}", text)
    if updated != text:
        stripped += [f"{source.parent.relative_to(LEAVES)} {match[2]}" for match in SPURIOUS.findall(text)]
        if WRITE:
            source.write_text(updated)

dead = []
for case in sorted(LEAVES.glob("*/*/🧪️tests/*")):
    if case.name in ("🔬️direct-leaf", "🔬️unit") or not case.is_dir():
        continue
    if f'"🧪️tests/{case.name}/🦀️.rs"' not in (case.parent.parent / "🦀️.rs").read_text():
        dead.append(case)
        if WRITE:
            shutil.rmtree(case)

print("\n".join(f"strip MutationLeaf {line}" for line in stripped))
print(f"{len(stripped)} spurious MutationLeaf derive(s); {len(dead)} unmounted per-case test dir(s){'' if WRITE else ' (dry run)'}")
