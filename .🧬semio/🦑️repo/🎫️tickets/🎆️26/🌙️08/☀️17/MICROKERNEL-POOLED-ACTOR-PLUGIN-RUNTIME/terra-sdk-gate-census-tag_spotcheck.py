#!/usr/bin/env python3
"""🔍 R2 spot check: sample plain (non-async) `fn` declarations and check whether a
`// 🚫️async: E<n>` tag appears within a few lines above, classifying likely E-class by
nearby context (const fn / extern / impl-of-external-trait / fn main) where detectable."""
import sys, os, re, random
sys.path.insert(0, '/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad')
from dyn_census import walk_rust_files

random.seed(42)
files = walk_rust_files()

FN_DECL_RE = re.compile(r'^\s*(?:pub(?:\([^)]*\))?\s+)?(?:const\s+)?(?:unsafe\s+)?(?:extern\s+"[^"]*"\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)')
ASYNC_MARK = re.compile(r'\basync\s+fn\s+')
TAG_RE = re.compile(r'🚫️async:\s*(E\d)')

candidates = []
for area, path in files:
    try:
        raw = open(path, encoding='utf-8', errors='replace').read()
    except Exception:
        continue
    lines = raw.splitlines()
    for i, line in enumerate(lines):
        stripped_line = line.strip()
        if stripped_line.startswith('//'):
            continue
        if 'async fn' in line:
            continue
        m = FN_DECL_RE.match(line)
        if not m:
            continue
        candidates.append((area, path, i+1, line))

print(f"total plain-fn-declaration-line candidates (heuristic): {len(candidates)}")
sample = random.sample(candidates, min(60, len(candidates)))

tagged = 0
untagged = []
for area, path, lineno, line in sample:
    raw = open(path, encoding='utf-8', errors='replace').read()
    lines = raw.splitlines()
    window = lines[max(0, lineno-7):lineno]
    has_tag = any(TAG_RE.search(l) for l in window)
    # crude context classification
    ctx = "\n".join(lines[max(0,lineno-15):lineno])
    is_const = bool(re.search(r'\bconst\s+fn\b', line))
    is_extern = bool(re.search(r'\bextern\s+"', line)) or 'fn main' in line
    is_impl_external = bool(re.search(r'impl(?:<[^>]*>)?\s+(Display|Debug|From|TryFrom|Default|Drop|Iterator|Serialize|Deserialize)\b', ctx))
    if has_tag:
        tagged += 1
    else:
        untagged.append((area, path, lineno, line.strip(), is_const, is_extern, is_impl_external))

print(f"sampled {len(sample)}; tagged nearby: {tagged}; untagged nearby: {len(untagged)}")
print()
print("untagged sample details (path, line, snippet, const?, extern?, impl-external-trait-nearby?):")
for u in untagged:
    print(u)
