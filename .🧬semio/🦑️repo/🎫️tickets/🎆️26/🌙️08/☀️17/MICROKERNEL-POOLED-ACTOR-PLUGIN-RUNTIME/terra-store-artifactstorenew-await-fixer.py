#!/usr/bin/env python3
"""🩹 Root-cause fixer for the `🏪️store/🦀️component.rs` test-module defect: the test-local
`impl ArtifactStore { async fn new(...) -> Self }` wrapper (line ~8971) shadows `super::ArtifactStore`
inside `mod tests`, but ~130 call sites of `ArtifactStore::new(<args>)` there predate that shadow and
never got `.await` appended. This is NOT a name-keyed sweep (R10) — it is keyed on the single fully
qualified constructor call `ArtifactStore::new(`, root-caused by hand (see
📓️terra-alltargets-kernel-report.md), with balanced-paren matching so nested calls/generics inside the
argument list never confuse the span. Only fires where `.await` is not already present immediately
after the call.
"""
import sys

PATH = sys.argv[1] if len(sys.argv) > 1 else None
APPLY = "--apply" in sys.argv
START_LINE = 8958  # 1-indexed; `#[cfg(test)]` — only touch the test module, never production scope

def find_matching_paren(text, open_idx):
    depth = 0
    i = open_idx
    while i < len(text):
        if text[i] == '(':
            depth += 1
        elif text[i] == ')':
            depth -= 1
            if depth == 0:
                return i
        i += 1
    return -1

def main():
    with open(PATH, 'r', encoding='utf-8') as f:
        text = f.read()

    lines = text.split('\n')
    start_offset = sum(len(l) + 1 for l in lines[:START_LINE - 1])

    needle = "ArtifactStore::new("
    edits = []  # (insert_pos, text_to_insert)
    pos = start_offset
    while True:
        idx = text.find(needle, pos)
        if idx == -1:
            break
        open_paren = idx + len("ArtifactStore::new")
        close_paren = find_matching_paren(text, open_paren)
        if close_paren == -1:
            print(f"UNBALANCED at byte {idx}, aborting", file=sys.stderr)
            sys.exit(1)
        after = text[close_paren + 1:close_paren + 7]
        if after != ".await":
            line_no = text.count('\n', 0, idx) + 1
            edits.append((close_paren + 1, line_no))
        pos = close_paren + 1

    print(f"found {len(edits)} call sites missing .await (in test module, from line {START_LINE})")
    for insert_pos, line_no in edits:
        print(f"  INSERT .await at byte {insert_pos} (line {line_no})")

    if APPLY and edits:
        # apply from the end so earlier byte offsets stay valid
        new_text = text
        for insert_pos, _ in sorted(edits, key=lambda e: -e[0]):
            new_text = new_text[:insert_pos] + ".await" + new_text[insert_pos:]
        with open(PATH, 'w', encoding='utf-8') as f:
            f.write(new_text)
        print(f"applied {len(edits)} edits")

if __name__ == "__main__":
    main()
