#!/usr/bin/env python3
"""✅️ Static verification of the plugin-host carve: losslessness, region markers, rustfmt, path resolution, emoji admission, path length, census rows, definition scan."""
import os, sys, re, json, collections, subprocess, tempfile
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hcmod, carve, ws as wsm, plan_host, regs_host
from carve import REPO, Model, inline_descendants
H = plan_host.H


def closure_files(roots):
    """All repo files reachable by mounts from the given crate roots, using the in-memory ws."""
    out = set()
    for ident, root in roots.items():
        r, modmap = hcmod.load_crate(root)
        for m in hcmod.walk(r):
            if not m.inline and not getattr(m, 'missing', False):
                out.add(m.file)
    return out


def norm_lines(text):
    return [l.strip() for l in text.split('\n') if l.strip()]


def lossless(ws, roots):
    """Multiset of non-blank lines: every line of the old closure must live in the new closure (moved, not lost)."""
    base_ws = wsm.WS()
    hcmod.FS = base_ws
    old_files = closure_files({'semio_framework_plugin_host': plan_host.P('📦️packages/🦀️rust/🦀️.rs')})
    hcmod.FS = ws
    new_files = closure_files(roots)
    old = collections.Counter()
    for f in old_files:
        old.update(norm_lines(base_ws.read(f)))
    new = collections.Counter()
    for f in new_files:
        new.update(norm_lines(ws.read(f)))
    lost = old - new
    gained = new - old
    return lost, gained, old_files, new_files


def regions(text):
    names = collections.Counter()
    stack = []
    bad = 0
    for ln in text.split('\n'):
        s = ln.strip()
        if s.startswith('//#region'):
            n = s[len('//#region'):].strip()
            stack.append(n)
            names[n.split(' — ')[0]] += 1
        elif s.startswith('//#endregion'):
            if stack:
                stack.pop()
            else:
                bad += 1
    return names, len(stack) + bad


def region_report(ws, old_files, new_files, base_ws):
    old = collections.Counter()
    for f in old_files:
        n, unb = regions(base_ws.read(f))
        old.update({k: 1 for k in n})
    new = collections.Counter()
    unbalanced = []
    for f in new_files:
        n, unb = regions(ws.read(f))
        new.update({k: 1 for k in n})
        if unb:
            unbalanced.append(f)
    missing = [k for k in old if k not in new]
    return missing, unbalanced


def rustfmt_parse(ws, files):
    bad = []
    for f in sorted(files):
        if not f.endswith('.rs'):
            continue
        p = subprocess.run(['rustfmt', '--edition', '2021', '--emit', 'stdout'], input=ws.read(f), capture_output=True, text=True)
        if p.returncode != 0:
            bad.append((f, p.stderr[:200]))
    return bad


def emoji_ok(name):
    """The taxonomy admission rule: NFC, and the leading emoji is an RGI sequence or an Extended_Pictographic code point followed by U+FE0F."""
    js = 'const n=process.argv[1];const m=n.match(/^(\\P{ASCII}+?)[a-z0-9-]+$/u);const e=m?m[1]:"";console.log(e===e.normalize("NFC")&&(/^\\p{RGI_Emoji}$/v.test(e)||/^\\p{Extended_Pictographic}\\uFE0F$/u.test(e))?"ok":"BAD:"+e)'
    p = subprocess.run(['bun', '-e', js, name], capture_output=True, text=True)
    return p.stdout.strip()


def longest_paths(ws):
    out = []
    for p in ws.changed():
        out.append((len(os.path.relpath(p, REPO).encode('utf-8')), os.path.relpath(p, REPO)))
    return sorted(out, reverse=True)[:3]
