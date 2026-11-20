"""✍️ Round 3 hedgehog: writes the corrected root members (materials, rig, anchors, representations, poses) of 🗑️generated/r3-hedgehog/members.json into the catalog asset file (re-read right before, every other member untouched) and splices the source volumes with volumes-insert.py.

Usage: python3 r3-hedgehog-apply.py   (only after r3-validate.ts --file on the candidate says VALID)
"""
import importlib.util
import json
import subprocess

spec = importlib.util.spec_from_file_location("io", "r3-hedgehog-io.py")
io = importlib.util.module_from_spec(spec)
spec.loader.exec_module(io)
members = json.load(open("🗑️generated/r3-hedgehog/members.json", encoding="utf-8"))
io.put(members)
print("members written:", ", ".join(members))
print(subprocess.run(["python3", "volumes-insert.py", "volumes/animals/hedgehog.json", io.FILE], capture_output=True, text=True).stdout.strip()[-300:])
