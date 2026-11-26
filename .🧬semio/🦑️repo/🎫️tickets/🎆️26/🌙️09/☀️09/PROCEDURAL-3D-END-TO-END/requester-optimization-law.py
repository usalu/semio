#!/usr/bin/env python3
"""🧪 Law: the final procedural requester crate must be compiled optimized; cargo omits `-C opt-level` only for opt-level 0."""
import importlib.util
import json
from pathlib import Path
import sys

ticket = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("rebuild", ticket / "rebuild-procedural-requester.py")
rebuild = importlib.util.module_from_spec(spec)
spec.loader.exec_module(rebuild)
captured = ticket / "🗑️generated/bridge-current"
unoptimized = [json.loads((captured / lane / "rustc.json").read_text()) for lane in ("b4", "boolean-link")]
policy = ["rustc", "--crate-name", "semio_hub_procedural_component", "-C", "opt-level=s", "-C", "metadata=1"]
inline = ["rustc", "--crate-name", "semio_hub_procedural_component", "-Copt-level=3"]
zero = ["rustc", "--crate-name", "semio_hub_procedural_component", "-C", "opt-level=0"]
checks = [rebuild.requester_is_unoptimized(args) for args in unoptimized] + [rebuild.requester_is_unoptimized(zero), rebuild.requester_is_unoptimized(policy), rebuild.requester_is_unoptimized(inline)]
print("[DEBUG] unoptimized verdicts", checks)
assert checks == [True, True, True, False, False], checks
print("[DEBUG] law holds")
