#!/usr/bin/env python3
# ➕️ Copies dependency lines of the top crate manifest into a crate manifest (same section, path specs shifted one directory deeper). usage: adddep.py <crate-suffix> <dep-ident>...
import sys, os, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + "/w1")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *

def top_lines(ident):
    text = read(T + "/📦️packages/🦀️rust/Cargo.toml")
    section, out = None, []
    for line in text.split("\n"):
        m = re.match(r"^\[(.+)\]\s*$", line)
        if m:
            section = m.group(1)
            continue
        k = re.match(r"^([A-Za-z0-9_-]+)\s*=", line)
        if k and k.group(1).replace("-", "_") == ident:
            out.append((section, line))
    return out

def shift(line):
    return re.sub(r'path = "((?:\.\./)+)', lambda m: 'path = "../' + m.group(1), line)

def add(suffix, ident, table):
    man = T + "/" + table[suffix] + "/📦️packages/🦀️rust/Cargo.toml"
    text = read(man)
    for section, line in top_lines(ident):
        if section in ("dev-dependencies",) or (section or "").endswith("dev-dependencies"):
            continue
        line = shift(line)
        hdr = f"[{section}]"
        key = line.split("=")[0].strip()
        if re.search(r"(?m)^" + re.escape(key) + r"\s*=", text[text.find(hdr):] if hdr in text else ""):
            continue
        if hdr in text:
            i = text.index(hdr) + len(hdr)
            j = text.find("\n[", i)
            j = len(text) if j < 0 else j
            body = text[i:j].rstrip("\n")
            lines = body.split("\n")
            lines.append(line)
            head = [l for l in lines if l.strip() == ""]
            items = sorted([l for l in lines if l.strip()], key=lambda l: l.split("=")[0].strip())
            text = text[:i] + "\n" + "\n".join(items) + "\n" + text[j:]
        else:
            k = text.find("\n[dev-dependencies]")
            block = f"\n{hdr}\n{line}\n"
            text = text[:k] + block + text[k:] if k >= 0 else text.rstrip("\n") + "\n" + block
    write(man, text)

if __name__ == "__main__":
    table = crate_table()
    suffix = sys.argv[1]
    for ident in sys.argv[2:]:
        add(suffix, ident, table)
