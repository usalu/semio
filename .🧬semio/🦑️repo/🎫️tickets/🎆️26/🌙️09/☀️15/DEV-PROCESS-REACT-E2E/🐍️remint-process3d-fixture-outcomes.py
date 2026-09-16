"""🧬️ Re-mints the committed after-snapshots and diffs of the process3d mutation fixtures from the
crate's own `apply_mutation` / `Mutation::diff` (dumped by a one-off test into
🗑️generated/applied-fixtures.tsv) — the content-addressed child ids (`steps-flow-<hash>`, …) hash the
codec's JSON, so a codec change re-mints them. Key order and formatting of every field that still
agrees are kept as committed; only differing scalars are replaced and only differing structure is
taken from the re-minted value."""
import json, sys
from pathlib import Path

rows = [line.rstrip("\n").split("\t") for line in open(sys.argv[1], encoding="utf-8") if line.startswith("[")]

def align(original, minted):
    if isinstance(original, dict) and isinstance(minted, dict):
        if set(original) != set(minted):
            return minted
        return {k: align(original[k], minted[k]) for k in original}
    if isinstance(original, list) and isinstance(minted, list):
        if len(original) != len(minted):
            return minted
        return [align(a, b) for a, b in zip(original, minted)]
    if isinstance(original, bool) or isinstance(minted, bool):
        return minted
    if isinstance(original, (int, float)) and isinstance(minted, (int, float)):
        return float(minted) if isinstance(minted, float) else minted
    return minted

changed = 0
for _, path, minted_text in rows:
    p = Path(path)
    before = p.read_text(encoding="utf-8") if p.exists() else "{}\n"
    aligned = align(json.loads(before), json.loads(minted_text))
    after = json.dumps(aligned, ensure_ascii=False, indent=2) + "\n"
    if after != before:
        p.write_text(after, encoding="utf-8"); changed += 1
print(changed, "fixtures re-minted")
