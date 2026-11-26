#!/usr/bin/env python3
"""🔗 Preserve the exact macOS service-test link and its ephemeral objects."""
import json
import os
from pathlib import Path
import shutil
import sys

receipt = Path(os.environ["SEMIO_GEOMETRY_LINK_RECEIPT"])
objects = receipt.parent / "link-objects"
objects.mkdir(exist_ok=True)
args = sys.argv[1:]
for index, argument in enumerate(args):
    if index and args[index - 1] == "-o":
        continue
    source = Path(argument)
    if source.suffix != ".o" or not source.is_file():
        continue
    target = objects / f"{index:03d}-{source.name}"
    shutil.copyfile(source, target)
    args[index] = str(target)
receipt.write_text(json.dumps({"args": [shutil.which("cc"), *args], "cwd": os.getcwd()}))
sys.exit(89)
