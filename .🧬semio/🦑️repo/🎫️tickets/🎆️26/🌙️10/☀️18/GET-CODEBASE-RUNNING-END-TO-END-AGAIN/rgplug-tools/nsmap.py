"""Namespace resolver for `use super::*` children: which root bindings does a text use?"""
import sys, re, os, json, subprocess
sys.path.insert(0, os.path.dirname(__file__))
from rgp import *

RUST_KW = set("as break const continue crate else enum extern false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true type unsafe use where while async await dyn abstract become box do final macro override priv typeof unsized virtual yield try union".split())

def use_statements(src):
    out = []
    for c in top_items(src):
        body = re.sub(r'(?m)^\s*//[^\n]*\n', '', c).lstrip("\n")
        if re.match(r'(?:#\[[^\n]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?use\s', body):
            out.append(c.strip("\n"))
    return out

def free_idents(text):
    """identifiers that may refer to a binding in scope (not field/method/path-tail, not after `::`)."""
    t = re.sub(r'//[^\n]*', '', text)
    t = re.sub(r'"(?:\\.|[^"\\])*"', '""', t)
    t = re.sub(r"'(?:\\.|[^'\\])'", "''", t)
    out = set()
    for m in re.finditer(r'(?<![A-Za-z0-9_:.])([A-Za-z_][A-Za-z0-9_]*)', t):
        w = m.group(1)
        if w in RUST_KW: continue
        out.add(w)
    return out
