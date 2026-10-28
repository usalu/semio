#!/usr/bin/env python3
"""keys3-g: independent oracle for the Tailwind scan narrowing. For every utility rule that only the base scan emits (css-diff.py output), searches the files of the NARROWED scan literally (no Tailwind code) and reports a file that still contains the class token; also lists which excluded files carry it. Streams file by file. usage: dropped-check.py dropped.txt base-files.txt final-files.txt"""
import re, sys, resource
dropped, base_files, final_files = sys.argv[1:4]
tokens = {}
for line in open(dropped, encoding="utf8"):
    if not line.startswith("ONLY-A @layer utilities >> "): continue
    selector = line[len("ONLY-A @layer utilities >> "):].strip()
    if " >> " in selector: selector = selector.split(" >> ")[0]
    m = re.match(r"\.((?:\\.|[^\s:\[,>+~])+)", selector)
    if not m: continue
    css = m.group(1)
    token = re.sub(r"\\(.)", r"\1", css)
    tokens[token] = css
def search(files):
    hits = {}
    pattern = re.compile(r"(?<![A-Za-z0-9_\-])(" + "|".join(re.escape(t) for t in sorted(tokens, key=len, reverse=True)) + r")(?![A-Za-z0-9_\-])")
    for path in files:
        try: text = open(path, encoding="utf8", errors="ignore").read()
        except OSError: continue
        for m in pattern.finditer(text): hits.setdefault(m.group(1), set()).add(path)
        assert resource.getrusage(resource.RUSAGE_SELF).ru_maxrss < 600 * 1024 * 1024
    return hits
final = [l.strip() for l in open(final_files, encoding="utf8") if l.strip()]
excluded = sorted(set(l.strip() for l in open(base_files, encoding="utf8") if l.strip()) - set(final))
inside = search(final); outside = search(excluded)
# a token inside a kept file is a literal occurrence that the Tailwind scan did not turn into this exact rule (substring of a longer value or a different variant)
print(f"dropped utility rules {len(tokens)}; kept-file literal hits {len(inside)}; excluded-file hits {len(outside)}; without any literal hit in excluded files {len([t for t in tokens if t not in outside])}")
for t in sorted(inside): print("KEPT-HIT", t, sorted(inside[t])[:2])
for t in sorted(tokens):
    if t not in outside: print("NO-EXCLUDED-HIT", t)
