"""Which external crate names do the given Rust files name? usage: extcrates.py <cargo.toml of P> <file>..."""
import re, sys, os
sys.path.insert(0, os.path.dirname(__file__)); sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
from stubgen import mask


def deps_of(manifest_text):
    """{rust ident: (dep key, section, line)}"""
    out = {}
    sec = None
    for line in manifest_text.splitlines():
        m = re.match(r"^\[(.+)\]\s*$", line)
        if m:
            sec = m.group(1)
            continue
        if sec and (sec.endswith("dependencies")) and "=" in line and not line.lstrip().startswith("#"):
            key = line.split("=", 1)[0].strip()
            if not re.match(r"^[A-Za-z0-9_\-]+$", key):
                continue
            pkg = re.search(r'package\s*=\s*"([^"]+)"', line)
            ident = key.replace("-", "_")
            out[ident] = (key, sec, line.strip())
    return out


def used(files, idents):
    hit = {}
    for f in files:
        t = mask(open(f, encoding="utf-8").read())
        for i in idents:
            if re.search(r"(?<![\w:])(?:::)?" + re.escape(i) + r"\s*::", t) or re.search(r"\buse\s+" + re.escape(i) + r"\b", t) or re.search(r"(?<![\w:])" + re.escape(i) + r"\s*!", t):
                hit.setdefault(i, set()).add(os.path.basename(os.path.dirname(f)))
    return hit


if __name__ == "__main__":
    mt = open(sys.argv[1], encoding="utf-8").read()
    d = deps_of(mt)
    h = used(sys.argv[2:], list(d))
    for i in sorted(h):
        print(i, "|", d[i][1], "|", d[i][2][:140])
