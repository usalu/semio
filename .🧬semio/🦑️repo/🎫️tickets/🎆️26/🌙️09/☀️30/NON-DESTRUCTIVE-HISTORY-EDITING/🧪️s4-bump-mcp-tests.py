#!/usr/bin/env python3
"""🧪️ S4-BUMP: drops the hand-written agent transaction label from the MCP gateway test call sites (design §20.6/§20.7).

The gateway's `AppCommand::TransactionPrepare` and the shell-channel `transactionPrepare` payload lost `label` together with the
guest frame field; every anchor below must match exactly as often as stated.
"""
import pathlib
import re
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
MCP = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp"


def sub(path: pathlib.Path, pattern: str, new: str, count: int) -> None:
    text = path.read_text()
    result, found = re.subn(pattern, new, text)
    if found != count:
        sys.exit(f"{path.name}: pattern matched {found}x (expected {count}): {pattern!r}")
    path.write_text(result)


sub(MCP / "🐚️channel/🧪️tests/🔬️quick/🦀️.rs", r'(children: Vec::new\(\) \}, )label: "append paragraph"\.to_string\(\), ', r"\1", 1)
sub(MCP / "🔀️dispatch/🧪️tests/🔬️quick/🦀️.rs", r'(ops: PreparedOps::default\(\), )label: "external"\.into\(\), ', r"\1", 1)
long = MCP / "🏠️workspace/🧪️tests/🔬️long/🦀️.rs"
sub(long, r'\n +label: "(?:w3 probe|wr4 two-phase probe)"\.to_string\(\),', "", 2)
sub(long, r"(children: ops\.children \}, )label: txn\.clone\(\), ", r"\1", 1)
print("mcp tests re-sealed")
