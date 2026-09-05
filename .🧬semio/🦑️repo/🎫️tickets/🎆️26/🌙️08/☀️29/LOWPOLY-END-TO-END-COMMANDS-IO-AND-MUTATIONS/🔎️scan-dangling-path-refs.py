import os, re, sys, json
ROOT = "/Users/ueli/Documents/semio"
SKIP = {"node_modules", ".git", "storybook-static", "temp", "test-results", ".🦑️repo"}
inc_re  = re.compile(r'include_str!\s*\(\s*"([^"]+)"\s*\)')
path_re = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
roots = sys.argv[1:] or ["."]
bad = []
scanned = 0
for r in roots:
    for dirpath, dirnames, filenames in os.walk(os.path.join(ROOT, r)):
        dirnames[:] = [d for d in dirnames if d not in SKIP and not d.startswith("target")]
        for fn in filenames:
            if not fn.endswith(".rs"):
                continue
            fp = os.path.join(dirpath, fn)
            try:
                src = open(fp, encoding="utf-8").read()
            except Exception:
                continue
            scanned += 1
            for rx, kind in ((inc_re, "include_str"), (path_re, "path_attr")):
                for m in rx.finditer(src):
                    rel = m.group(1)
                    target = os.path.normpath(os.path.join(dirpath, rel))
                    if not os.path.exists(target):
                        line = src[:m.start()].count("\n") + 1
                        bad.append({"file": os.path.relpath(fp, ROOT), "line": line,
                                    "kind": kind, "ref": rel,
                                    "resolved": os.path.relpath(target, ROOT)})
print(json.dumps({"scanned": scanned, "dangling": len(bad), "items": bad}, ensure_ascii=False, indent=1))
