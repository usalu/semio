#!/usr/bin/env python3
"""📇 S5-NESTED P1 follow-up: the two fixture helpers sit BEFORE `new_test_child`'s own docstring, not inside it.

Usage: python3 🧪️s5-nested-p1-doc-order.py check|land
The P1 landing inserted `declare_test_child` / `register_test_child` between `new_test_child`'s docstring and its signature, so that
docstring read as theirs. Test-only, no behaviour: the block moves above the docstring. Counted anchors, fails closed.
"""
import pathlib
import sys

REPO = pathlib.Path(__file__).resolve().parents[7]
TARGET = REPO / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"
BLOCK_START = "    /// 📇️ The APP's half of a composition: declares `child_id` in the parent's one owned-child slot beside every member\n"
BLOCK_END = "    async fn new_test_child(id: &str) -> Result<TestMembers, store::VcsError> {\n"
DOC_START = "    /// 🧪️ A live child `ArtifactStore<TestSnapshot, TestMutation>`, wrapped as `TestMembers` —\n"


def main() -> None:
    mode = sys.argv[1] if len(sys.argv) > 1 else "check"
    text = TARGET.read_text(encoding="utf-8")
    for anchor in (BLOCK_START, BLOCK_END, DOC_START):
        if text.count(anchor) != 1:
            raise SystemExit(f"anchor drift: {text.count(anchor)} × {anchor[:80]!r}")
    doc, start, end = text.index(DOC_START), text.index(BLOCK_START), text.index(BLOCK_END)
    if not doc < start < end:
        print("already in order: 0 files to write")
        return
    block = text[start:end]
    result = text[:doc] + block + text[doc:start] + text[end:]
    assert len(result) == len(text)
    print(f"moves {block.count(chr(10))} lines above the docstring of new_test_child")
    if mode == "land":
        TARGET.write_text(result, encoding="utf-8")
        print(f"landed {TARGET.name}")


if __name__ == "__main__":
    main()
