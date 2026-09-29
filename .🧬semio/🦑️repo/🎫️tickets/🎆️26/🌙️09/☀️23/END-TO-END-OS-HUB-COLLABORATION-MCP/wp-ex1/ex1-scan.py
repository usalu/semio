#!/usr/bin/env python3
"""📡️ EX1 broad scan: every fallback-shaped expression within 6 lines of an example reference in non-test plugin Rust."""
import os, re, sys, json
ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins"
EX = re.compile(r"PRIMARY_TEXT|examples::|example_snapshot|EXAMPLE|example_id|exampleId|ExampleSource|catalogue_example_document|document_json\(")
FB = re.compile(r"\.unwrap_or_default\(\)|\.unwrap_or_else\(|\.unwrap_or\(|\.or_else\(|\.ok\(\)|::default\(\)\s*\}?$|Emit::default\(\)|=>\s*None|else\s*\{\s*None")
out = []
for dp, dns, fns in os.walk(ROOT):
    if "🧪️tests" in dp or "/target" in dp or "node_modules" in dp: continue
    for fn in fns:
        if not fn.endswith(".rs"): continue
        p = os.path.join(dp, fn)
        lines = open(p, encoding="utf-8").read().split("\n")
        cut = len(lines)
        for i, l in enumerate(lines):
            if re.match(r"\s*#\[cfg\(test\)\]\s*$", l) and i + 1 < len(lines) and re.match(r"\s*(pub\s+)?mod\s+\w+\s*\{", lines[i+1]):
                cut = i; break
        for i in range(cut):
            if FB.search(lines[i]):
                lo, hi = max(0, i-6), min(cut, i+1)
                if any(EX.search(lines[j]) for j in range(lo, hi)):
                    out.append({"file": os.path.relpath(p, ROOT), "line": i+1, "text": lines[i].strip()[:220]})
json.dump(out, open(sys.argv[1], "w"), ensure_ascii=False, indent=1)
print(len(out))
