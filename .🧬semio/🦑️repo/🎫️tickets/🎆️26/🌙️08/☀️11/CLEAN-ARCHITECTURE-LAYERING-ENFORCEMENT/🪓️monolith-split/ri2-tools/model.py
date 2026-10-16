#!/usr/bin/env python3
# 🧭️ Model of the Shell module tree for RENDERER-INTEGRATION-2: the parent mounts, the staging modules by crate, the residue (old chunk) modules, the name index and the crate ranks.
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + "/w1")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *
from rs import blank, uses, expand_use
import items as _items

SH = E + "/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu"
PARENT = SH + "/🦀️.rs"
STG = SH + "/📥️staging"
FROZEN = json.load(open(GEN + "/rwp0-crates.json"))
RANK = {k: v["rank"] for k, v in FROZEN.items()}

TIER1 = ["shell-async", "shell-chrome-state", "shell-surfaces", "shell-plugins", "shell-history", "shell-prefs", "shell-directory", "shell-sync", "shell-docs", "shell-doc-open", "shell-windows", "shell-dock", "shell-input", "shell-commands", "shell-state"]
WP5 = ["shell-session", "shell-window-actions", "shell-actions", "shell-pointer", "shell-view-paint", "shell-view-chrome", "shell-view-overlays", "shell-view-panels"]
SHELL_ORDER = ["shell-vocab"] + TIER1 + WP5


def strip_emoji(name):
    return re.sub(r"^[^A-Za-z0-9]+", "", name)


def staging_crate(folder):
    base = strip_emoji(folder)
    cand = "shell-" + base
    return cand if cand in RANK else None


MOUNT_RE = re.compile(r"((?:#\[[^\]\n]*\]\s*)*)(?:(pub(?:\([^)]*\))?)\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;")


def parent_mounts(path=PARENT):
    text = read(path)
    clean = blank(text)
    here = os.path.dirname(path)
    mounts = []
    reexp = {}
    for m in re.finditer(r"(?m)^[ \t]*((?:#\[[^\]\n]*\][ \t]*\n[ \t]*)*)(pub(?:\([^)]*\))?)[ \t]+use[ \t]+([A-Za-z_][A-Za-z0-9_]*)::\*;", clean):
        reexp.setdefault(m.group(3), []).append((text[m.start(1):m.end(1)].strip(), m.group(2)))
    for m in MOUNT_RE.finditer(clean):
        attrs = text[m.start(1):m.end(1)]
        pm = re.search(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]', attrs)
        name = m.group(3)
        if not pm:
            continue
        file = os.path.normpath(os.path.join(here, pm.group(1)))
        cfg = [a.strip() for a in re.findall(r"#\[\s*cfg[^\n]*\]", attrs)]
        rec = dict(name=name, file=file, cfg=cfg, vis=m.group(2) or "", reexport=[r[1] for r in reexp.get(name, [])], reexport_cfg=[r[0] for r in reexp.get(name, [])])
        rel = os.path.relpath(file, STG)
        if not rel.startswith(".."):
            parts = rel.split(os.sep)
            crate = staging_crate(parts[0])
            rec.update(kind="staging", crate=crate, slug=parts[1] if len(parts) > 2 else "")
        elif file.startswith(SH + "/"):
            rec.update(kind="chunk", crate=None, slug=os.path.relpath(file, SH).split(os.sep)[0])
        else:
            rec.update(kind="other", crate=None, slug="")
        mounts.append(rec)
    return mounts


def defined(file):
    text = read(file)
    out = []
    for it in _items.table(text):
        if it["kind"] in ("impl", "use"):
            continue
        out.append(it)
    return out


def tokens(text):
    clean = blank(text)
    out = set()
    for m in re.finditer(r"(?<![A-Za-z0-9_$])([A-Za-z_][A-Za-z0-9_]*)", clean):
        s, e = m.start(), m.end()
        k = s - 1
        while k >= 0 and clean[k] in " \t\n":
            k -= 1
        if k >= 0 and clean[k] == "." and not (k >= 1 and clean[k - 1] == "."):
            continue
        j = e
        while j < len(clean) and clean[j] in " \t":
            j += 1
        if j < len(clean) and clean[j] == ":" and clean[j:j + 2] != "::":
            continue
        if m.group(1) != "_":
            out.add(m.group(1))
    for s in re.finditer(r'"((?:[^"\\\n]|\\.)*)"', text):
        for f in re.finditer(r"\{([A-Za-z_][A-Za-z0-9_]*)(?::[^{}]*)?\}", s.group(1)):
            out.add(f.group(1))
    return out


if __name__ == "__main__":
    ms = parent_mounts()
    from collections import Counter
    print(len(ms), Counter(m["kind"] for m in ms))
    by = Counter(m["crate"] for m in ms if m["kind"] == "staging")
    print(by)
    for m in ms:
        if m["kind"] != "staging":
            print(m["kind"], m["name"], m["slug"], m["vis"], m["reexport"], m["cfg"])


def original_mounts():
    import json
    return json.load(open(os.path.dirname(os.path.abspath(__file__)) + "/mounts0.json"))
