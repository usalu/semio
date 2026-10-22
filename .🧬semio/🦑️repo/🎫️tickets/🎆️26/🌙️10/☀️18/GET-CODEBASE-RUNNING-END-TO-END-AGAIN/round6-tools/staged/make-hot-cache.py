#!/usr/bin/env python3
"""Merged copy of the hot-cache staged set (phases 1-3) on top of the chain: its three catalogue/schema anchors are computed from the tree as it is by then
 - `--json` flag operation list (rcache-server, tool-iface, backend-contract and polyglot-units extend it): `service` is appended to whatever list stands there;
 - the operation-name enum of the catalogue schema ends with the last name that stands there: `service` is appended;
 - backend-contract moved the Nx caller envelope into `tools/nx/envelope`: the two caller edits that wrap its capabilities follow it;
 - the caller schema's `tool` pattern is the open pattern backend-contract installs; the service block is inserted after it."""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/hot-cache-tools/staged", "hot-cache")
root = repo_root()
B = "\U0001F9F0️framework/\U0001F6CD️products/\U0001F991️repo/\U0001F528️modules/\U0001F4DA️library/\U0001F3D7️build"
spec = json.load(open(T + "/edits.json", encoding="utf8"))
rows = spec["edits"] if isinstance(spec, dict) else spec
by_id = {row["id"]: row for row in rows}
catalog = open(os.path.join(root, B, "⚙️operations/\U0001F523️catalog.json"), encoding="utf8").read()
start = catalog.index('"name": "--json",')
match = re.compile(r'"operations": \[([^\]]*)\]').search(catalog, start)
current = match.group(0)
row = by_id["catalog-json-flag"]
row["anchor"], row["text"] = current, current[:-1] + ', "service"]'
schema = open(os.path.join(root, B, "⚙️operations/\U0001F9EC️schema/\U0001F523️.json"), encoding="utf8").read()
tail = re.search(r'"OperationName": \{\s*"enum": \[[^\]]*?("[a-z-]+")\]\n    \},\n    "Operation": \{', schema)
if tail is None:
    raise SystemExit("maker: hot-cache: OperationName enum not found")
last = tail.group(1)
row = by_id["catalog-schema-operation"]
row["anchor"], row["text"] = last + ']\n    },\n    "Operation": {', last + ', "service"]\n    },\n    "Operation": {'
caller = open(os.path.join(root, B, "\U0001F9EC️schema/\U0001F4E5️caller.json"), encoding="utf8").read()
pattern = re.search(r'"pattern": ("\^[^"]+\$")\n    \}\n  \}\n\}', caller)
if pattern is None:
    raise SystemExit("maker: hot-cache: caller tool pattern not found")
row = by_id["service-schema-caller"]
old_pattern = '"pattern\\": \\"^(nx|bazel)$\\"'
assert "^(nx|bazel)$" in row["anchor"] and "^(nx|bazel)$" in row["text"]
row["anchor"] = row["anchor"].replace("^(nx|bazel)$", pattern.group(1).strip('"'))
row["text"] = row["text"].replace("^(nx|bazel)$", pattern.group(1).strip('"'))
envelope = B + "/\U0001F527\ufe0ftools/\U0001F9F6\ufe0fnx/\U0001F4E8\ufe0fenvelope/\U0001F7E6\ufe0f.ts"
text = open(os.path.join(root, envelope), encoding="utf8").read() if os.path.isfile(os.path.join(root, envelope)) else ""
moved = re.search(r"import \{isStoreRepairSelectionV1\} from \"[^\"]+\";\n", text)
if moved:
    for ident in ("service-caller-import", "service-caller-nx-envelope"):
        by_id[ident]["file"] = envelope
    by_id["service-caller-import"]["anchor"] = moved.group(0)
    by_id["service-caller-import"]["text"] = by_id["service-caller-import"]["text"].replace("./\U0001F6F0\ufe0fservice/", "../../../\U0001F4E5\ufe0fcaller/\U0001F6F0\ufe0fservice/")
if moved:
    _caller_file = B + "/\U0001F4E5\ufe0fcaller/\U0001F7E6\ufe0f.ts"
    _caller = open(os.path.join(root, _caller_file), encoding="utf8").read()
    _line = re.search(r"^import \{requireBuildOrchestrationContextV1[^\n]*\n", _caller, re.M)
    if _line is None:
        raise SystemExit("maker: hot-cache: caller import anchor not found")
    _base = dict(by_id["service-caller-import"])
    _base.update({"id": "service-caller-import-caller", "file": _caller_file, "anchor": _line.group(0), "text": 'import {declaredCapabilitiesV1} from "./\U0001F6F0\ufe0fservice/\U0001F7E6\ufe0f.ts";\n'})
    rows.append(_base)
json.dump(spec, open(T + "/edits.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("hot-cache merged copy:", T)
