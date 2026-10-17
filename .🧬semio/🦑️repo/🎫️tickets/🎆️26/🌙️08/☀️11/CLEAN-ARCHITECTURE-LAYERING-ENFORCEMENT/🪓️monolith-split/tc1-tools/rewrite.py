"""Rewrites `const NAME: &[u8]|&str = include_bytes!|include_str!("REL");` lines of oracle test units into cached run-time readers and renames the uses. usage: rewrite.py <file>..."""
import os, re, shutil, sys, hashlib
ROOT = "/Users/ueli/Documents/semio"
ORIG = os.path.join(os.path.dirname(os.path.abspath(__file__)), "orig")
os.makedirs(ORIG, exist_ok=True)
const_re = re.compile(r'^const (\w+): &(\[u8\]|str) = include_(?:bytes|str)!\("([^"]*)"\);\n', re.M)

def artifact_of(path):
    parts = path.split("🔮️oracles")
    return parts[0].rstrip("/") if False else None

def art_root(path):
    marker = "/🗿️artifacts/"
    i = path.index(marker) + len(marker)
    j = path.index("/", i)
    return path[:j]

for f in sys.argv[1:]:
    path = f if f.startswith("/") else os.path.join(ROOT, f)
    text = open(path, encoding="utf-8").read()
    key = hashlib.sha256(path.encode()).hexdigest()[:16]
    dest = os.path.join(ORIG, key + ".rs")
    if not os.path.exists(dest):
        shutil.copy2(path, dest)
        open(os.path.join(ORIG, "index.tsv"), "a").write(f"{key}\t{path}\n")
    art = art_root(path)
    out = text
    for m in list(const_re.finditer(text)):
        name, kind, rel = m.group(1), m.group(2), m.group(3)
        tgt = os.path.normpath(os.path.join(os.path.dirname(path), rel))
        assert os.path.isfile(tgt), (path, rel)
        art_rel = os.path.relpath(tgt, art)
        lower = name.lower()
        assert not re.search(rf'\bfn {lower}\b', out), (path, lower)
        if kind == "[u8]":
            fn = f'fn {lower}() -> &\'static [u8] {{\n    static CELL: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();\n    CELL.get_or_init(|| crate::runtime_fixture::bytes("{art_rel}")).as_slice()\n}}\n'
        else:
            fn = f'fn {lower}() -> &\'static str {{\n    static CELL: std::sync::OnceLock<String> = std::sync::OnceLock::new();\n    CELL.get_or_init(|| crate::runtime_fixture::text("{art_rel}")).as_str()\n}}\n'
        assert out.count(m.group(0)) == 1
        out = out.replace(m.group(0), "\x00" + name + "\x00")
        out = re.sub(rf'\b{name}\b', f'{lower}()', out)
        out = out.replace("\x00" + f"{lower}()" + "\x00", fn)
    arg_re = re.compile(r'(?<=\()include_(bytes|str)!\("([^"]*)"\)')
    def arg(m):
        tgt = os.path.normpath(os.path.join(os.path.dirname(path), m.group(2)))
        assert os.path.isfile(tgt), (path, m.group(2))
        return f'&crate::runtime_fixture::{"bytes" if m.group(1) == "bytes" else "text"}("{os.path.relpath(tgt, art)}")'
    out = arg_re.sub(arg, out)
    left = [l for l in out.splitlines() if re.search(r'include_(bytes|str)!', l)]
    for l in left: print("  MANUAL", os.path.relpath(path, ROOT)[-60:], "|", l.strip()[:200])
    if out != text:
        open(path, "w", encoding="utf-8").write(out)
        print("rewrote", os.path.relpath(path, ROOT))
