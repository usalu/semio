#!/usr/bin/env python3
"""🧪️ S4-BUMP: drops the host coordinator's description argument from its test call sites (CHANNEL_VERSION 21).

`AppCommand::TransactionPrepare.label` left the frame and `transaction_prepare`/the testkit transaction helpers lost their
label parameter; every anchor below must match exactly as often as stated.
"""
import pathlib
import re
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
PLUGIN = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"


def sub(path: pathlib.Path, pattern: str, new: str, count: int) -> None:
    text = path.read_text()
    result, found = re.subn(pattern, new, text)
    if found != count:
        sys.exit(f"{path.name}: pattern matched {found}x (expected {count}): {pattern!r}")
    path.write_text(result)


coordinator = PLUGIN / "🖥️host/🧪️tests/🔬️host-transaction-coordinator/🦀️.rs"
sub(coordinator, r'\n( +)"[^"\n]*"\.into\(\),\n(\1(?:foreign|vec!\[[^\n]*\]),\n)', r"\n\2", 6)
print("host coordinator tests re-sealed")
