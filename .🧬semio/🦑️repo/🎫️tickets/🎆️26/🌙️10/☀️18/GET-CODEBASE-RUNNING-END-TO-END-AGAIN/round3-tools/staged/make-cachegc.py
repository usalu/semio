#!/usr/bin/env python3
"""Derives the merged copy of cache-gc from the CURRENT original: the catalogue edit of the `--json` flag (index 1 of CATALOG) anchored on the exact operation list `["graph", "show", "incremental", "capacity"]`,
which conc-harness (`concurrency`) and other sets extend; the merged copy appends `cache` to whatever list the flag has (idempotent) and keeps every other pair as authored."""
import os, re, shutil
HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(os.path.dirname(HERE))
SOURCE, TARGET = C + "/cache-gc-tools", HERE + "/cache-gc"
shutil.rmtree(TARGET, ignore_errors=True)
shutil.copytree(SOURCE, TARGET, ignore=shutil.ignore_patterns("__pycache__", "backup"))
path = TARGET + "/patches.py"
text = open(path, encoding="utf8").read()
old = '"catalog": lambda text: apply(text, CATALOG, "catalog"),'
assert text.count(old) == 1
helper = '''
import re as _re


def _json_flag(text):
    pattern = _re.compile(r'("name": "--json",\\n\\s+"value": "none",\\n\\s+"operations": \\[)([^\\]]*)(\\])')
    match = pattern.search(text)
    if match is None:
        raise Drift("catalog: --json flag not found")
    if '"cache"' not in match.group(2):
        text = text[:match.start(2)] + match.group(2) + ', "cache"' + text[match.end(2):]
    if '"name": "--area"' not in text:
        tail = CATALOG[1][1][CATALOG[1][1].index('    {\\n      "name": "--area"'):]
        end = text.index("\\n    },\\n", match.start()) + len("\\n    },\\n")
        text = text[:end] + tail + text[end:]
    return text


CATALOG_WITHOUT_JSON = [pair for index, pair in enumerate(CATALOG) if index != 1]
'''
marker = "\nEDITORS" if "\nEDITORS" in text else None
new = '"catalog": lambda text: apply(_json_flag(text), CATALOG_WITHOUT_JSON, "catalog"),'
text = text.replace(old, new)
i = text.index("\n    \"catalog\"")
j = text.rfind("\n\n", 0, i)
text = text[:j] + "\n" + helper + text[j:]
open(path, "w", encoding="utf8").write(text)
print("merged cache-gc: --json flag edit made list-independent")
