#!/usr/bin/env python3
"""Drops the `implementations` property from the operation rows of the build catalogue: backend-contract makes the registry the one source of the backends and its law refuses a catalogue row that names one, but the rows `units` (polyglot-units), `bench` (perf-slo) and `service` (hot-cache) are added after it.
usage: catalog-drop-implementations.py <root> [--write]"""
import json, os, re, sys

ROOT = sys.argv[1]
path = os.path.join(ROOT, "\U0001F9F0️framework/\U0001F6CD️products/\U0001F991️repo/\U0001F528️modules/\U0001F4DA️library/\U0001F3D7️build/⚙️operations/\U0001F523️catalog.json")
BENCH = os.path.join(ROOT, "\U0001F9F0\ufe0fframework/\U0001F6CD\ufe0fproducts/\U0001F991\ufe0frepo/\U0001F528\ufe0fmodules/\U0001F4DA\ufe0flibrary/\U0001F3D7\ufe0fbuild/\U0001F4CF\ufe0fincremental/\U0001F3CE\ufe0fbench/\U0001F9EA\ufe0ftests/\U0001F7E6\ufe0f.ts")
TEST_EDITS = (
    ('expect(entry.implementations).toEqual({nx:"neutral",bazel:"neutral"});', "expect(entry.implementations).toBeUndefined();"),
    ('import {tmpdir} from "node:os";', 'import {homedir,tmpdir} from "node:os";'),
    ("tmpdir:()=>tmpdir(),now:()=>performance.now(),", "tmpdir:()=>tmpdir(),home:()=>homedir(),now:()=>performance.now(),"),
    ('toEqual(["preflight","tools setup all","dependencies sync --all"]);', 'toEqual(["preflight","tools setup all","dependencies sync --all","tools setup all"]);'),
)
test_text = open(BENCH, encoding="utf8").read() if os.path.isfile(BENCH) else ""
test_new = test_text
for _old, _new in TEST_EDITS:
    if _old in test_new:
        test_new = test_new.replace(_old, _new)
text = open(path, encoding="utf8").read()
new, count = re.subn(r'\n[ \t]*"implementations": \{[^{}]*\},', "", text)
json.loads(new)
if count == 0 and test_new == test_text:
    print("already applied")
    sys.exit(0)
if "--write" in sys.argv:
    open(path, "w", encoding="utf8").write(new)
    if test_new != test_text:
        open(BENCH, "w", encoding="utf8").write(test_new)
    print("applied: %d `implementations` propert%s dropped" % (count, "y" if count == 1 else "ies"))
else:
    print("pending: %d `implementations` propert%s to drop" % (count, "y" if count == 1 else "ies"))
    sys.exit(1)
