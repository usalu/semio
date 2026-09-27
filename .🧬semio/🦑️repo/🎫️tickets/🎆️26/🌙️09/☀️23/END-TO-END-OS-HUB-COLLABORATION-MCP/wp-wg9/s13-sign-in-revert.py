#!/usr/bin/env python3
"""↩️ WG9: reverses WG7's `s12-hub-sign-in-off-interaction.py` exactly (every hunk of the patch swapped, anchors asserted once), for a
landing window that closes before the renderer's native check can run. Dry run by default; `--apply` writes."""
import difflib
import importlib.util
import sys
from pathlib import Path

spec = importlib.util.spec_from_file_location("signin", "/Users/ueli/Documents/semio/.tmp-ticket/wp-wg7/s12-hub-sign-in-off-interaction.py")
patch = importlib.util.module_from_spec(spec)
spec.loader.exec_module(patch)
names = [name for name in dir(patch) if name.endswith("_EDITS")]
print("edit lists:", names)
