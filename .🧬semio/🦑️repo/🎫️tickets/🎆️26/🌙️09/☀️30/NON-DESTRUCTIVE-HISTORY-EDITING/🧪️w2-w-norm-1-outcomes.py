#!/usr/bin/env python3
"""🧪️ W2-W norm-1 follow-up: makes every EN 1991 / EN 1990 leaf descriptor's `outcomeClasses` — and the v2 manifest row that
mirrors it — state what production dispatch reaches: `applied` always, `no-op` where the diff keeps an equality guard
(`mutation.no-op`), `rejected` where it refuses (an index past the end, a missing impact record). The crate's vector law
(`🧪️tests/🔬️fixture`) asserts the same classes from the committed vectors, so the two cannot drift apart.

    python3 🧪️w2-w-norm-1-outcomes.py
"""
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
ARTIFACTS = ROOT / "✏️s/🔌️plugins/📕️norm/🗿️artifacts"


def reached(diff_source):
    """🎯️ The outcome classes one leaf's diff can produce, in vocabulary order."""
    return ["applied"] + (["no-op"] if "mutation.no-op" in diff_source else []) + (["rejected"] if re.search(r"MutationOutcome::(error|fatal)\(", diff_source) else [])


if __name__ == "__main__":
    for artifact in ("🏋️en1991", "⚖️en1990"):
        subset = ARTIFACTS / artifact / "🏅️standards/🔖️1/🪆️subsets/✳️any"
        classes = {}
        for leaf in sorted((subset / "🧬️schema/🧬️mutations").iterdir()):
            descriptor = leaf / "🔣️.json"
            declared = json.loads(descriptor.read_text(encoding="utf-8")) if descriptor.is_file() else {}
            if "semanticKind" not in declared:
                continue
            classes[declared["semanticKind"]] = reached((leaf / "🔺️diff/🦀️.rs").read_text(encoding="utf-8"))
            if declared["outcomeClasses"] != classes[declared["semanticKind"]]:
                declared["outcomeClasses"] = classes[declared["semanticKind"]]
                descriptor.write_text(json.dumps(declared, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
                print(f"{artifact} {declared['semanticKind']}: {classes[declared['semanticKind']]}")
        manifest_path = subset / "🔮️oracles/🔣️.json"
        manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
        for row in manifest["mutationManifests"][0]["mutations"]:
            row["outcomes"] = classes[row["id"]]
        manifest_path.write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
