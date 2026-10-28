#!/usr/bin/env python3
"""🔧 Normalize Rust target triples for the ticket-owned Zig compiler."""
from pathlib import Path
import os
import sys
root = Path(__file__).resolve().parent / "🗑️generated"
args = [arg.replace("--target=x86_64-unknown-linux-gnu", "--target=x86_64-linux-gnu") for arg in sys.argv[1:]]
os.execv(str(root / "zig-x86_64-linux-0.14.1/zig"), ["zig", "cc", *args])
