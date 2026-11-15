#!/usr/bin/env python3
"""R37 payload: freezes the anchored edits of the digest helper selection into edits.json (run once against the live tree before the edits landed)."""
import json,os,sys
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
import payload_edits
json.dump([list(row) for row in payload_edits.EDITS],open(HERE+"/edits.json","w",encoding="utf8"),ensure_ascii=False)
print("edits",len(payload_edits.EDITS))
