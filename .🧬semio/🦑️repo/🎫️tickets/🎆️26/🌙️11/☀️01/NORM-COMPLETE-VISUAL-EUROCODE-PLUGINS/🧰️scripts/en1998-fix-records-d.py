#!/usr/bin/env python3
"""Repairs hyphenated Pascal record names written by one generator run (leaf-kinds D); one-off, kinds derived from the root enum."""
import os, re
ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm"
ANY = ROOT + "/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any"
root = open(ANY + "/🧬️schema/🧬️mutations/🦀️.rs", encoding="utf-8").read()
hy = sorted(set(re.findall(r"\b((?:Change|Insert|Remove)(?:-[a-z0-9]+)+)\b", root)))
if not hy:
    hy = set()
files = []
for base in (ANY, ROOT + "/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1"):
    for d, _, fs in os.walk(base):
        for f in fs:
            if f.endswith((".rs", ".json", ".ts", ".py", ".feature", ".semio", ".graphql", ".proto")): files.append(os.path.join(d, f))
tok = re.compile(r"\b(Change|Insert|Remove)(?:-[a-z0-9]+)+\b")
def pas(h): return "".join(x[:1].upper() + x[1:] for x in h.split("-"))
def sub(m): return pas(m.group(0))
n = 0
for p in files:
    try: s = open(p, encoding="utf-8").read()
    except UnicodeDecodeError: continue
    if not tok.search(s): continue
    t = tok.sub(sub, s)
    open(p, "w", encoding="utf-8").write(t); n += 1
print("fixed files:", n)

kinds = [k for k in re.findall(r'^\s+"((?:change|insert|remove)-[a-z0-9-]+)",$', root, re.M)]
def pascal_k(k): return "".join(x[:1].upper() + x[1:] for x in k.split("-"))
def camel_k(k): p = pascal_k(k); return p[:1].lower() + p[1:]
pats = []
for k in kinds:
    pats.append((re.compile(r'(?<![\w-])parse' + re.escape(k[:1].upper() + k[1:]) + r'(?![\w-])'), "parse" + pascal_k(k)))
    pats.append((re.compile(r'("mutation": |"const": |mutation: |normWireLiteral\()"' + re.escape(k) + '"'), lambda m, k=k: m.group(1) + '"' + camel_k(k) + '"'))
    pats.append((re.compile(r'^(\s+)' + re.escape(k) + r': parse', re.M), lambda m, k=k: m.group(1) + camel_k(k) + ": parse"))
m = 0
for p in files:
    try: s = open(p, encoding="utf-8").read()
    except UnicodeDecodeError: continue
    t = s
    if p.endswith("/🔣️.json") or p.endswith("🟦️.ts"):
        for rx, rep in pats: t = rx.sub(rep, t)
    if t != s:
        open(p, "w", encoding="utf-8").write(t); m += 1
print("camel fixed files:", m)
