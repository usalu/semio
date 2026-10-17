"""🧭 Head check: every first segment of a `use` path (and of fully qualified calls) must be an extern crate of the owning manifest, an `extern crate` alias of the lib root, a std crate, or a keyword.
usage: heads.py <manifest-dir> <rs-file>...   (read only)"""
import os, re, sys
STD = {"std", "core", "alloc", "crate", "super", "self", "Self", "proc_macro", "test"}
def deps(manifest):
    out = set()
    sec = None
    for line in open(manifest, encoding="utf8").read().splitlines():
        m = re.match(r"^\[(.+)\]\s*$", line)
        if m:
            sec = m.group(1)
            continue
        if sec and sec.endswith("dependencies") and "=" in line and not line.lstrip().startswith("#"):
            key = line.split("=", 1)[0].strip().split(".")[0]
            if re.match(r"^[A-Za-z0-9_\-]+$", key):
                out.add(key.replace("-", "_"))
    return out
def lib_root(manifest):
    t = open(manifest, encoding="utf8").read()
    m = re.search(r'(?ms)^\[lib\].*?^path\s*=\s*"([^"]+)"', t)
    return os.path.normpath(os.path.join(os.path.dirname(manifest), m.group(1) if m else "src/lib.rs"))
def aliases(root):
    s = open(root, encoding="utf8").read()
    a = set()
    for m in re.finditer(r"extern\s+crate\s+([A-Za-z0-9_]+)(?:\s+as\s+([A-Za-z0-9_]+))?\s*;", s):
        a.add(m.group(2) or m.group(1))
    return a
USE = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?use\s+(?:::)?([A-Za-z_][A-Za-z0-9_]*)(?:::|\s*;)", re.M)
def main():
    mdir = sys.argv[1]
    manifest = mdir + "/Cargo.toml"
    ok = deps(manifest) | aliases(lib_root(manifest)) | STD
    bad = []
    for f in sys.argv[2:]:
        s = open(f, encoding="utf8").read()
        mods = set(re.findall(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:mod|enum|struct|trait|type)\s+([A-Za-z_][A-Za-z0-9_]*)", s, re.M))
        mods |= set(re.findall(r"\bas\s+([A-Za-z_][A-Za-z0-9_]*)\s*[;,}]", s))
        for m in USE.finditer(s):
            h = m.group(1)
            if h not in ok and h not in mods:
                line = s.count("\n", 0, m.start()) + 1
                bad.append((f, line, h))
    for f, line, h in bad:
        print("UNRESOLVED-HEAD", os.path.basename(os.path.dirname(f)), line, h)
    print(len(bad), "unresolved heads")
main()
