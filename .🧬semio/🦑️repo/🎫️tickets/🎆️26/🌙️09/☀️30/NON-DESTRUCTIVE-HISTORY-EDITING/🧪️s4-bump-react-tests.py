#!/usr/bin/env python3
"""🧪️ S4-BUMP: drops the coordinator proposal description and the prepare label from the React plugin-runtime tests.

`TransactionProposal.description` (host-side coordinator input) and `TransactionPrepareRequest.label` left with the guest frame
field `AppCommand::TransactionPrepare.label` (CHANNEL_VERSION 21); every anchor must match exactly as often as stated.
"""
import pathlib
import re
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
TEST = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx"


def sub(text: str, pattern: str, new: str, count: int) -> str:
    result, found = re.subn(pattern, new, text)
    if found != count:
        sys.exit(f"pattern matched {found}x (expected {count}): {pattern!r}")
    return result


text = TEST.read_text()
text = sub(text, r'(localOps: [^\n]*,\n)( +)description: "[^"\n]*",\n', r"\1", 11)
text = sub(text, r'(prepared_ops: \[\], )label: "", (origin: \[\])', r"\1\2", 1)
TEST.write_text(text)
print("react plugin-runtime tests re-sealed")
