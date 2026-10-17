"""Test-name multiset: names.py <out> <file...>; names are `fn` identifiers following a test attribute (#[test], #[tokio::test], #[semio_framework_async_macros::async_test], #[...::test...])."""
import re, sys, collections
out = sys.argv[1]; files = sys.argv[2:]
rx = re.compile(r'#\[(?:[A-Za-z_][A-Za-z0-9_:]*::)?(?:async_)?test(?:\([^\]]*\))?\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)')
cnt = collections.Counter()
for f in files:
    s = open(f, encoding="utf-8").read()
    for m in rx.finditer(s): cnt[m.group(1)] += 1
with open(out, "w") as o:
    for k in sorted(cnt): o.write(f"{cnt[k]}\t{k}\n")
print(sum(cnt.values()), "tests,", len(cnt), "distinct ->", out)
