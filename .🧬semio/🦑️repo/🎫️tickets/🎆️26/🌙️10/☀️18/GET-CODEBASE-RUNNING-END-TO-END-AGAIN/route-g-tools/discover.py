"""Streaming discovery of build-library test files by the process route protocol."""
import json, os, re, sys, resource
root = sys.argv[1]
protocol = json.load(open(sys.argv[2], encoding="utf8"))["definitions"]["Protocol"]["const"]
patterns = [re.compile(p) for p in protocol["importPatterns"]]
for directory, names, files in os.walk(root, followlinks=False):
    names[:] = sorted(n for n in names if n not in protocol["skippedDirectories"] and not os.path.islink(os.path.join(directory, n)))
    rel = os.path.relpath(directory, root).split(os.sep)
    if protocol["testDirectory"] not in rel: continue
    for name in sorted(files):
        p = os.path.join(directory, name)
        if name.endswith(".ts") and not os.path.islink(p):
            t = open(p, encoding="utf8").read()
            if any(x.search(t) for x in patterns): print("/".join(os.path.relpath(p, root).split(os.sep)))
assert resource.getrusage(resource.RUSAGE_SELF).ru_maxrss < 500*1024*1024
