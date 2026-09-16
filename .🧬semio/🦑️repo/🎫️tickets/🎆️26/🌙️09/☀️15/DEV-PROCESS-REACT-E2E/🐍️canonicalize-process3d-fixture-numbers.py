"""🔣️ Aligns the process3d mutation fixtures with the codec's decode→encode fixed point
(`committed_json_is_canonical`): every number the schema types as f64 is written as a float
(`0.0`, not `0`), by walking each committed file next to its canonical re-encoding (dumped by a
one-off test into 🗑️generated/canonical-fixtures.tsv) and keeping everything else — key order,
formatting — exactly as committed."""
import json, sys
from pathlib import Path

rows = [line.rstrip("\n").split("\t") for line in open(sys.argv[1], encoding="utf-8") if line.startswith("[FIXTURE]\t")]

def align(original, canonical):
    if isinstance(original, dict) and isinstance(canonical, dict):
        return {k: (align(v, canonical[k]) if k in canonical else v) for k, v in original.items()}
    if isinstance(original, list) and isinstance(canonical, list) and len(original) == len(canonical):
        return [align(a, b) for a, b in zip(original, canonical)]
    if isinstance(original, bool) or isinstance(canonical, bool):
        return original
    if isinstance(original, int) and isinstance(canonical, float):
        return float(original)
    return original

changed = 0
for _, path, canonical_text in rows:
    p = Path(path)
    before = p.read_text(encoding="utf-8")
    aligned = align(json.loads(before), json.loads(canonical_text))
    after = json.dumps(aligned, ensure_ascii=False, indent=2) + "\n"
    if after != before:
        p.write_text(after, encoding="utf-8"); changed += 1
print(changed, "fixtures canonicalised")
