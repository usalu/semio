#!/usr/bin/env python3
"""Raises the budget of the repository cache root in the cache ledger policy from 20 GiB to 24 GiB: the declared area budgets of the live ledger already fill 19.75 GiB, remote-cache (3 GiB) and the workers' home of remote-execution (512 MiB instead of 64 MiB) add 3.44 GiB.
usage: ledger-root-budget.py <root> [--write]"""
import os, sys

ROOT = sys.argv[1]
path = os.path.join(ROOT, "\U0001F9F0️framework/\U0001F6CD️products/\U0001F991️repo/\U0001F528️modules/\U0001F4DA️library/⚡️caching/\U0001F4BE️storage/\U0001F9FE️ledger/\U0001F523️policy.json")
text = open(path, encoding="utf8").read()
old, new = '"maximumBytes": 21474836480,', '"maximumBytes": 25769803776,'
if new in text and old not in text:
    print("already applied")
    sys.exit(0)
assert text.count(old) == 1, "root budget anchor"
if "--write" in sys.argv:
    open(path, "w", encoding="utf8").write(text.replace(old, new))
    print("applied: repository cache root budget 24 GiB")
else:
    print("pending: repository cache root budget 24 GiB")
    sys.exit(1)
