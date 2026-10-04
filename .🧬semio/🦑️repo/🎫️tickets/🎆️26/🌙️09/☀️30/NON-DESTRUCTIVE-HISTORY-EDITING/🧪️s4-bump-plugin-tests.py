#!/usr/bin/env python3
"""🧪️ S4-BUMP: drops the host-written transaction label from the plugin runtime test call sites (CHANNEL_VERSION 21).

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


fixtures = PLUGIN / "🧪️tests/🧬️mutation-fixtures-transaction/🦀️.rs"
sub(fixtures, r'(\}\.into\(\)\]), "peer-write"', r"\1", 3)
sub(fixtures, r'(\.expect\("encode"\)\], &\[\], )"(?:peer-write|first|second)", ', r"\1", 4)

contract = PLUGIN / "🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"
sub(contract, r'(&\[parent_op\], &wire, )"agent composite", ', r"\1", 1)
sub(contract, r'(&\[\], &(?:only_child|stray), )"", ', r"\1", 2)
sub(contract, r"the way `LoadDocument` \+ `LoadChildren` would\.", "the way the host's document load + `LoadChildren` would.", 1)

dispatch = PLUGIN / "🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs"
sub(dispatch, r"(prepared_ops: Vec::new\(\), )label: String::new\(\), (origin: Vec::new\(\))", r"\1\2", 1)
print("plugin tests re-sealed")
