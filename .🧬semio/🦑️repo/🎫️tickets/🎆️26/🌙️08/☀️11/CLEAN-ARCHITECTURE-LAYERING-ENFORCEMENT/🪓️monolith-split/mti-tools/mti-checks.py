import json, os, re, subprocess, sys
R = "/Users/ueli/Documents/semio"
PATH_ATTR = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
INCLUDE = re.compile(r'\b(include_str|include_bytes|include)!\(\s*"([^"]+)"\s*\)')
ENV = re.compile(r'\b(include_str|include_bytes|include)!\(\s*concat!\(\s*env!\("CARGO_MANIFEST_DIR"\)\s*,\s*"([^"]+)"\s*\)\s*\)')
paths = [l.strip() for l in open(sys.argv[1], encoding="utf8") if l.strip()]
fmt_fail, missing, long_paths, bad_json = [], [], [], []
checked = 0
for p in paths:
    full = f"{R}/{p}"
    if not os.path.exists(full):
        missing.append(("file", p)); continue
    if len(p.encode("utf8")) > 240: long_paths.append((len(p.encode("utf8")), p))
    if p.endswith(".rs"):
        r = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--emit", "stdout"], stdin=open(full, "rb"), capture_output=True, cwd=R)
        if r.returncode != 0: fmt_fail.append((p, r.stderr.decode("utf8", "replace")[:200]))
        text = open(full, encoding="utf8").read()
        base = os.path.dirname(full)
        for m in PATH_ATTR.finditer(text):
            checked += 1
            if not os.path.exists(os.path.normpath(os.path.join(base, m.group(1)))): missing.append((p, "path", m.group(1)))
        for m in INCLUDE.finditer(text):
            checked += 1
            if not os.path.exists(os.path.normpath(os.path.join(base, m.group(2)))): missing.append((p, m.group(1), m.group(2)))
    if p.endswith(".json"):
        try: json.load(open(full, encoding="utf8"))
        except Exception as e: bad_json.append((p, str(e)[:100]))
print(json.dumps({"files": len(paths), "rustfmt_failures": len(fmt_fail), "plain_literals_checked": checked, "plain_literals_missing": len(missing), "paths_over_240_bytes": len(long_paths), "json_parse_failures": len(bad_json), "longest_path_bytes": max(len(p.encode("utf8")) for p in paths), "longest_path_chars": max(len(p) for p in paths)}))
for row in fmt_fail: print("RUSTFMT", row)
for row in missing: print("MISSING", row)
for row in long_paths: print("TOO-LONG", row)
for row in bad_json: print("JSON", row)
