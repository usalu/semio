#!/usr/bin/env python3
"""🧪️ W2-W norm-1: recodes the state-dependent refusals of EN 1991 / EN 1990 (an index past the collection's end, an impact
record the addressed accidental case does not carry) from `mutation.invariant` (Fatal) to `mutation.target-missing` (Error,
target = the addressed index) — design §11 negative-witness addendum and the store's message-level table.

    python3 🧪️w2-w-norm-1-target-missing.py
"""
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
ARTIFACTS = ROOT / "✏️s/🔌️plugins/📕️norm/🗿️artifacts"
RULES = [
    (re.compile(r'MutationOutcome::fatal\("mutation\.invariant", "(Index out of range\.|Impact variant missing\.)", Vec::<String>::new\(\)\)'), r'MutationOutcome::error("mutation.target-missing", "\1", [payload.index.to_string()])'),
    (re.compile(r'MutationOutcome::fatal\("mutation\.invariant", format!\("(\w+ index out of range)"\), Vec::<String>::new\(\)\)'), r'MutationOutcome::error("mutation.target-missing", "\1", [payload.index.to_string()])'),
]

if __name__ == "__main__":
    for artifact in ("🏋️en1991", "⚖️en1990"):
        for path in sorted((ARTIFACTS / artifact / "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations").glob("*/🔺️diff/🦀️.rs")):
            source = path.read_text(encoding="utf-8")
            recoded = source
            for pattern, replacement in RULES:
                recoded = pattern.sub(replacement, recoded)
            if recoded != source:
                path.write_text(recoded, encoding="utf-8")
                print(path.relative_to(ARTIFACTS))
