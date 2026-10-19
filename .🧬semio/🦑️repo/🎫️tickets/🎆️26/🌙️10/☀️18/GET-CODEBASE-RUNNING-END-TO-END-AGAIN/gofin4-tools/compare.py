#!/usr/bin/env python3
"""usage: compare.py <base-test.log> <final-test.log> -- per-test status diff"""
import re, sys
def parse(p):
    out = {}
    for line in open(p, errors='replace'):
        m = re.match(r'^\s*--- (PASS|FAIL|SKIP): (\S+)', line)
        if m: out[m.group(2)] = m.group(1)
    return out
a, b = parse(sys.argv[1]), parse(sys.argv[2])
print(f"base: {len(a)} tests ({sum(v=='PASS' for v in a.values())} pass, {sum(v=='FAIL' for v in a.values())} fail, {sum(v=='SKIP' for v in a.values())} skip)")
print(f"final: {len(b)} tests ({sum(v=='PASS' for v in b.values())} pass, {sum(v=='FAIL' for v in b.values())} fail, {sum(v=='SKIP' for v in b.values())} skip)")
print("removed:", sorted(set(a) - set(b)))
print("added:", sorted(set(b) - set(a)))
print("changed:", sorted((k, a[k], b[k]) for k in set(a) & set(b) if a[k] != b[k]))
