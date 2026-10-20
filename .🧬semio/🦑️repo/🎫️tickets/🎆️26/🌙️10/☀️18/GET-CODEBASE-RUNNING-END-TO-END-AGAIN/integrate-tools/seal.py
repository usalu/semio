#!/usr/bin/env python3
# 🧷 Seals a hand resolution for the token conflict <key-set> (merge.py output dir /private/tmp/ig/merged/_conflicts): choices.json = list with "ours" | "theirs" | literal text per conflict block.
# usage: seal.py <key-set> <choices.json> [expected-digest-prefix]
import hashlib, json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
key, choices = sys.argv[1], json.load(open(sys.argv[2], encoding="utf8"))
token = open(f"{os.environ.get('MERGED', '/private/tmp/ig/merged')}/_conflicts/{key}.token", "rb").read()
digest = hashlib.sha256(token).hexdigest()
if len(sys.argv) > 3 and not digest.startswith(sys.argv[3]):
    raise SystemExit(f"conflict digest {digest[:12]} differs from the expected {sys.argv[3][:12]}: re-read the blocks")
os.makedirs(f"{HERE}/resolutions", exist_ok=True)
json.dump({"tokenConflictSha256": hashlib.sha256(token).hexdigest(), "choices": choices}, open(f"{HERE}/resolutions/{key}.blocks.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("sealed", key, len(choices), "blocks")
