#!/usr/bin/env python3
# records the sha256 of every edited live file as the drift baseline (run when the edit set is final and the targets are as intended)
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from engine import *
hashes = {relative: sha(read(os.path.join(ROOT, relative))) for relative in edited_files()}
json.dump(hashes, open(os.path.join(TOOLS, "base-hashes.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("stamped", len(hashes))
