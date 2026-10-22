#!/usr/bin/env python3
"""Writes the scaffold of a spec file: one block per source file, one entry per distinct prose skeleton; German texts are filled in afterwards (`fill.py`)."""
import os, sys, importlib.util
from mig import *

spec = importlib.util.spec_from_file_location("chk", os.path.join(HERE, "staged/files", L, "⚖️laws/🗣️language-catalogs/🧪️tests/🗣️corpus/🐍️checker.py"))
chk = importlib.util.module_from_spec(spec); spec.loader.exec_module(chk)


def prose_sites(text):
    out = []
    for s in sites(text):
        f = s["first"]
        if not is_literal(f):
            continue
        plain = "".join(v if k == "t" else "XX" for k, v in split_template(f))
        if plain.startswith(("[DEBUG] ", "[DATA] ")):
            continue
        if chk.prose(plain):
            out.append(s)
    return out


def scaffold(tree, files, scopes, only_kinds=("console", "error")):
    lines = ["# i18n-core spec: skeleton ¦ names ¦ code ¦ key; `=>` German; optional `en=>` and `fixed=>`", ""]
    for rel in files:
        text = tree.text(rel)
        seen = []
        rows = []
        for s in prose_sites(text):
            if s["kind"] not in only_kinds:
                continue
            sk = skeleton(s["first"])
            if sk in seen:
                continue
            seen.append(sk)
            _, ex = derive(s["first"])
            rows.append((s, sk, [n for n, _ in ex]))
        if not rows:
            continue
        lines.append("== %s ¦ %s" % (rel.replace(L + "/", ""), scopes[rel]))
        for s, sk, names in rows:
            code = guess_code(sk) if s["kind"] == "error" else ""
            lines.append("%s ¦ %s ¦ %s" % (enc(sk), ",".join(names), code))
            lines.append("=> ???")
        lines.append("")
    return "\n".join(lines) + "\n"
