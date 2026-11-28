#!/usr/bin/env python3
"""H99 landing edit of the worker's command policy: the environment a build script run may carry (dependency metadata and the links name).
Usage: apply-command-policy.py <command policy json>."""
import json, sys
f = sys.argv[1]; j = json.load(open(f)); e = j["policy"]["environment"] if "policy" in j else None
if e is None:
    raise SystemExit("no policy.environment")
if "CARGO_MANIFEST_LINKS" not in e["allowedNames"]: e["allowedNames"].insert(e["allowedNames"].index("PATH"), "CARGO_MANIFEST_LINKS")
if "DEP_" not in e["allowedPrefixes"]: e["allowedPrefixes"].append("DEP_")
json.dump(j, open(f, "w"), ensure_ascii=False, indent=2); open(f, "a").write("\n")
