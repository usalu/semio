#!/usr/bin/env python3
# 🔓️ Definition scan: restricted (private / pub(crate) / pub(super)) items, fields and inherent methods of a crate's files that other files of the plugin tree name.
# usage: defscan.py <crate> [--apply] [--exclude-file path ...]
import collections, os, re, subprocess, sys, json
from lib import *
import defs, wp1files
from rs import blank
from crates_wp1 import ROOT

_INDEX = {}


def other_files(owned):
    out = []
    r = subprocess.run(["git", "-c", "core.quotepath=false", "ls-files", "--cached", "--others", "--exclude-standard", "--", "*.rs"], cwd=PLUG, capture_output=True, text=True)
    for rel in r.stdout.split("\n"):
        if not rel or rel.startswith(("🖥️host/", "📦️packages/")):
            continue
        p = os.path.join(PLUG, rel)
        if p in owned or not os.path.exists(p):
            continue
        out.append(p)
    return out


def index_of(path):
    if path in _INDEX:
        return _INDEX[path]
    try:
        clean = blank(read(path))
    except OSError:
        clean = ""
    toks = collections.Counter(re.findall(r"(?<![A-Za-z0-9_$])([A-Za-z_][A-Za-z0-9_]*)", clean))
    dots = collections.Counter(re.findall(r"\.\s*([A-Za-z_][A-Za-z0-9_]*)", clean))
    colons = collections.Counter(re.findall(r"::\s*([A-Za-z_][A-Za-z0-9_]*)", clean))
    fields = collections.Counter(re.findall(r"(?<![A-Za-z0-9_:])([a-z_][A-Za-z0-9_]*)\s*:(?!:)", clean))
    _INDEX[path] = (toks, dots, colons, fields)
    return _INDEX[path]


def scan_crate(crate_files, others, include_private=False):
    owned = set(crate_files)
    results = []
    for f in crate_files:
        text = read(f)
        for d in defs.scan(text):
            if d.vis == "pub" or (d.vis == "" and not include_private):
                continue
            if d.kind == "item" and d.ctx == "mod":
                continue
            results.append((f, d))
    return results


def hits(d, others):
    found = []
    for p in others:
        toks, dots, colons, fields = index_of(p)
        n = 0
        if d.kind == "reexport":
            n = sum(toks.get(x, 0) for x in d.name.split("|"))
        elif d.kind == "item":
            n = toks.get(d.name, 0)
        elif d.kind in ("method", "assoc"):
            n = dots.get(d.name, 0) + colons.get(d.name, 0)
            if n and d.parent and not toks.get(d.parent, 0) and d.kind == "method" and not dots.get(d.name):
                n = 0
        elif d.kind == "field":
            n = dots.get(d.name, 0) + fields.get(d.name, 0)
            if n and d.parent and not toks.get(d.parent, 0):
                n = -n
        elif d.kind == "tfield":
            n = toks.get(d.parent, 0)
        if n:
            found.append((p, n))
    return found


if __name__ == "__main__":
    args = sys.argv[1:]
    crate = args[0]
    apply = "--apply" in args
    root = PLUG + "/" + ROOT[crate] + "/🦀️.rs"
    files = [f for f, _ in wp1files.tree(root)]
    extra = [a for a in args if a.startswith("--also=")]
    for a in extra:
        files += [f for f, _ in wp1files.tree(a.split("=", 1)[1])]
    owned = set(files)
    others = other_files(owned)
    private_for = []
    for a in args:
        if a.startswith("--private-for="):
            private_for += [x if os.path.isabs(x) else PLUG + "/" + x for x in a.split("=", 1)[1].split(",") if x]
    if private_for:
        others = private_for
    scan_files = files
    if private_for:
        scan_files = [f for f in wp1files.tree_split(root)[0]]
    rows = scan_crate(scan_files, others, include_private=bool(private_for))
    flagged = []
    for f, d in rows:
        if d.kind == "item" and d.vis == "" and d.ctx == "mod" and False:
            continue
        h = hits(d, others)
        strong = [x for x in h if x[1] > 0]
        weak = [x for x in h if x[1] < 0]
        if d.kind in ("item", "reexport") and d.vis in ("", "pub(super)") and not strong:
            continue
        if strong or weak:
            flagged.append((f, d, strong, weak))
    if apply:
        byfile = collections.defaultdict(list)
        for f, d, strong, weak in flagged:
            if strong:
                byfile[f].append(d)
        for f, ds in byfile.items():
            text = read(f)
            for d in sorted(ds, key=lambda x: -x.pos):
                text = defs.widen(text, d)
            write(f, text)
        print("widened", sum(len(v) for v in byfile.values()), "definitions in", len(byfile), "files")
    for f, d, strong, weak in flagged:
        print(f"{'STRONG' if strong else 'weak  '} {d.kind:6s} {d.parent or '':28s} {d.name:36s} [{d.vis or 'private'}] {f.replace(PLUG + '/', '')}:{d.line}  hits={len(strong)}/{len(weak)} e.g. {[x[0].replace(PLUG + '/', '')[:40] for x in (strong or weak)[:2]]}")
