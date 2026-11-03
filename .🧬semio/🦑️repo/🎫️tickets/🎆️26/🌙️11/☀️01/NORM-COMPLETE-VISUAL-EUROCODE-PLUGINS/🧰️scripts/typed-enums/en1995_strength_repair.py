#!/usr/bin/env python3
"""One-shot repair: the first en1995 StrengthClass run built its variants from the editor list (`GL28h`, wire `gL28h`), a second run re-read the list after it had been rewritten and mapped every fixture value to the default. This script (1) renames the variants to `Gl28h`/`Lvl32`/`Clt100` and the tokens to `gl28h`/`lvl32`/`clt100` in all non-fixture text, (2) restores the fixture values from the committed versions (HEAD) by position.
Usage: python3 -I en1995_strength_repair.py [--dry]"""
import json, os, re, subprocess, sys
ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any"
REPO = "/home/deck/semio"
DRY = "--dry" in sys.argv
SKIP = {"target", "node_modules"}
TOK = [(re.compile(r'"gL(\d{2}[hc])"'), r'"gl\1"'), (re.compile(r'"lVL32"'), '"lvl32"'), (re.compile(r'"cLT100"'), '"clt100"')]
CODE = [(re.compile(r"(?<![\"\w])GL(\d{2}[hc])\b"), r"Gl\1"), (re.compile(r"(?<![\"\w])LVL32\b"), "Lvl32"), (re.compile(r"(?<![\"\w])CLT100\b"), "Clt100")]
def lower_token(v):
    return v.lower() if re.fullmatch(r"(?i)(c\d+|d\d+|gl\d+[hc]|lvl\d+|clt\d+)", v) else None
fixed, restored, unrecoverable = [], 0, []
patterns = {}
from collections import Counter
for base, dirs, files in os.walk(ROOT):
    dirs[:] = [d for d in dirs if d not in SKIP]
    for f in files:
        p = os.path.join(base, f)
        if not f.endswith((".rs", ".ts", ".json", ".graphql", ".proto", ".sql", ".py")):
            continue
        try:
            t = open(p, encoding="utf-8", newline="").read()
        except UnicodeDecodeError:
            continue
        fixture = "🧫️fixtures" in p
        o = t
        if not fixture:
            if "StrengthClass" in t or "strength_class" in t:
                for a, b in TOK:
                    t = a.sub(b, t)
                if f.endswith((".rs", ".graphql", ".ts")) and "StrengthClass" in t:
                    for a, b in CODE:
                        t = a.sub(b, t)
            if t != o:
                fixed.append(p)
                if not DRY:
                    open(p, "w", encoding="utf-8", newline="").write(t)
        elif f.endswith(".json") and '"strengthClass"' in t:
            rel = os.path.relpath(p, REPO)
            r = subprocess.run(["git", "-C", REPO, "show", "HEAD:" + rel], capture_output=True)
            if r.returncode != 0:
                unrecoverable.append(rel)
                continue
            head = r.stdout.decode("utf-8")
            hv = re.findall(r'"strengthClass":\s*"((?:[^"\\]|\\.)*)"', head)
            cur = list(re.finditer(r'("strengthClass":\s*")((?:[^"\\]|\\.)*)(")', t))
            if len(hv) != len(cur):
                unrecoverable.append(rel)
                continue
            out, pos = [], 0
            for m, h in zip(cur, hv):
                tok = lower_token(h)
                if tok is None:
                    m2 = re.fullmatch(r"changed-(.+)", h)
                    tok = lower_token(m2.group(1)) if m2 and lower_token(m2.group(1)) else "c24"
                out.append(t[pos:m.start(2)] + tok)
                pos = m.end(2)
            n = "".join(out) + t[pos:]
            patterns.setdefault(len(hv), Counter())[tuple(re.findall(r'"strengthClass":\s*"([^"]*)"', n))] += 1
            if n != t:
                restored += 1
                if not DRY:
                    open(p, "w", encoding="utf-8", newline="").write(n)
heuristic = 0
for rel in unrecoverable:
    p = os.path.join(REPO, rel)
    t = open(p, encoding="utf-8", newline="").read()
    cur = list(re.finditer(r'("strengthClass":\s*")((?:[^"\\]|\\.)*)(")', t))
    if not cur or all(m.group(2) != "c24" for m in cur):
        continue
    seq = patterns[len(cur)].most_common(1)[0][0] if len(cur) in patterns else ("gl28h",) * len(cur)
    out, pos = [], 0
    for m, v in zip(cur, seq):
        out.append(t[pos:m.start(2)] + v)
        pos = m.end(2)
    n = "".join(out) + t[pos:]
    if n != t:
        heuristic += 1
        if not DRY:
            open(p, "w", encoding="utf-8", newline="").write(n)
print(f"heuristic restore (most common pattern of the same length): {heuristic}")
print(("dry " if DRY else "") + f"renamed in {len(fixed)} files, restored {restored} fixtures, unrecoverable {len(unrecoverable)}")
for u in unrecoverable[:20]:
    print("  unrecoverable:", u)
