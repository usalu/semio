"""📥 Replace include_str!/include_bytes! in test files by run-time reads rooted at the product directory: `crate::text("<path>")`, `crate::bytes("<path>")`.
usage: embedrt.py <base-dir> <file>...   (path = normalized join of the test file's directory and the macro argument, made relative to <base-dir>; asserts the target exists)"""
import os, re, sys
base = sys.argv[1]
total = 0
for f in sys.argv[2:]:
    s = open(f, encoding="utf-8").read()
    d = os.path.dirname(os.path.abspath(f))

    def rel(arg):
        p = os.path.normpath(os.path.join(d, arg))
        assert os.path.exists(p), (f, arg)
        return os.path.relpath(p, base)

    PFX = os.environ.get("EMBED_PREFIX", "crate::")
    keep = []

    def protect(m):
        keep.append(m.group(0))
        return "\x00%d\x00" % (len(keep) - 1)

    s = re.sub(r'(?m)^[ \t]*(?:pub(?:\([^)]*\))?\s+)?(?:const|static)\s[^\n]*include_(?:str|bytes)![^\n]*$', protect, s)
    s2 = re.sub(r'include_bytes!\("([^"]+)"\)\.as_slice\(\)', lambda m: PFX + 'bytes("%s")' % rel(m.group(1)), s)
    s2 = re.sub(r'include_bytes!\("([^"]+)"\)', lambda m: PFX + 'bytes("%s")' % rel(m.group(1)), s2)
    s2 = re.sub(r'include_str!\("([^"]+)"\)', lambda m: PFX + 'text("%s")' % rel(m.group(1)), s2)
    s2 = re.sub("\x00(\\d+)\x00", lambda m: keep[int(m.group(1))], s2)
    left = [l for l in s2.split("\n") if "include_" in l and not re.match(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:const|static)\s", l)]
    assert not left, (f, left[:2])
    n = len(re.findall(r"include_(?:str|bytes)!", s)) - len(keep)
    total += n
    if n:
        open(f, "w", encoding="utf-8").write(s2)
    print(f"{os.path.basename(os.path.dirname(f))}: {n}")
print("total", total)
