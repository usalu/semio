"""📖 Rewrites compile-time fixture embeds of test files into run-time reads through the companion root helper `crate::fixture_text` (or `crate::fixture_bytes`), keyed by the path relative to the module directory.
usage: fixture_rewrite.py <module-dir-rel> <test-file-rel>...  (applies in place; asserts each file is re-read right before the write)
"""
import os, re, sys
ROOT = "/Users/ueli/Documents/semio"
INC = re.compile(r'include_(str|bytes)!\(\s*"([^"]+)"\s*\)')


def convert(mod_rel, rel):
    path = os.path.join(ROOT, rel)
    with open(path, encoding="utf-8") as f:
        text = f.read()
    d = os.path.dirname(rel)
    n = 0

    def sub(m):
        nonlocal n
        n += 1
        target = os.path.normpath(os.path.join(d, m.group(2)))
        assert target.startswith(mod_rel + "/"), (rel, target)
        assert os.path.exists(os.path.join(ROOT, target)), target
        return f'crate::fixture_{"text" if m.group(1) == "str" else "bytes"}("{target[len(mod_rel) + 1:]}")'

    out = INC.sub(sub, text)
    with open(path, encoding="utf-8") as f:
        assert f.read() == text, "file changed while converting " + rel
    tmp = path + ".ttc-tmp"
    with open(tmp, "w", encoding="utf-8") as f:
        f.write(out)
    os.replace(tmp, path)
    print(f"{n} embeds rewritten in {rel}")


if __name__ == "__main__":
    for r in sys.argv[2:]:
        convert(sys.argv[1], r)
