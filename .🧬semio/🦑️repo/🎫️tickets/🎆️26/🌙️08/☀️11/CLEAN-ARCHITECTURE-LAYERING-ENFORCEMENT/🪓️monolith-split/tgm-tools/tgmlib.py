"""Shared helpers of the tgm (TOOL-GRAPH-MACHINE-SPLITS) tools: paths, targets, snapshots, mask, module-tree walk."""
import hashlib, os, re, shutil, sys

ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
W = F + "/🗑️generated/tgm-work"
M = "🧰️framework/🔨️modules"
SKIP = {"node_modules", "target", "🗑️generated", ".git", "dist"}

TARGETS = {
    "toolrun": dict(
        tree=f"{M}/⏯️tool-run", root=f"{M}/⏯️tool-run/🦀️.rs",
        lib=f"{M}/⏯️tool-run/📦️packages/🦀️rust/🦀️.rs", crate="semio_framework_tool_run",
        cfgs=["lib", "test", "wasm", "p2"], features=[]),
    "toolmachine": dict(
        tree=f"{M}/🛠️tool-machine", root=f"{M}/🛠️tool-machine/🦀️.rs",
        lib=f"{M}/🛠️tool-machine/📦️packages/🦀️rust/🦀️.rs", crate="semio_framework_tool_machine",
        cfgs=["lib", "test", "wasm", "p2"], features=[]),
    "machine": dict(
        tree=f"{M}/🔄️machine", root=f"{M}/🔄️machine/🦀️.rs",
        lib=f"{M}/🔄️machine/📦️packages/🦀️rust/🦀️.rs", crate="semio_framework_machine",
        cfgs=["lib", "test", "wasm", "p2"], features=[]),
    "dsl": dict(
        tree=f"{M}/🕸️graph/🗣️dsl", root=f"{M}/🕸️graph/🗣️dsl/🦀️.rs",
        lib=f"{M}/🕸️graph/🗣️dsl/🦀️.rs", crate="semio_framework_graph_dsl",
        cfgs=["lib", "test", "wasm", "p2"], features=[]),
    "engine": dict(
        tree=f"{M}/🕸️graph/⚙️engine", root=f"{M}/🕸️graph/⚙️engine/🦀️.rs",
        lib=f"{M}/🕸️graph/⚙️engine/🦀️.rs", crate="semio_framework_graph_engine",
        cfgs=["lib", "test", "wasm", "p2"], features=[]),
    "layoutrun": dict(
        tree=f"{M}/🕸️graph/⏯️layout-run", root=f"{M}/🕸️graph/⏯️layout-run/🦀️.rs",
        lib=f"{M}/🕸️graph/⏯️layout-run/📦️packages/🦀️rust/🦀️.rs", crate="semio_framework_graph_layout_run",
        cfgs=["lib", "test", "wasm", "p2"], features=["testing"]),
}


def read(path):
    with open(path, encoding="utf-8") as f:
        return f.read()


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(text)


def sha(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def lines_of(text):
    ls = text.split("\n")
    if ls and ls[-1] == "":
        ls.pop()
    return ls


def snap_dir(name):
    return f"{W}/{name}/snap"


def under(path, base):
    rp, rb = os.path.realpath(path), os.path.realpath(base)
    assert rp == rb or rp.startswith(rb + os.sep), f"path {rp} outside {rb}"


def copy_tree(src_rel, dst_root):
    n = 0
    base = os.path.join(ROOT, src_rel)
    for dp, dn, fn in os.walk(base):
        dn[:] = [d for d in dn if d not in SKIP]
        for f in fn:
            p = os.path.join(dp, f)
            if os.path.islink(p):
                continue
            if not f.endswith(".rs") and os.path.getsize(p) > 300_000:
                continue
            rel = os.path.relpath(p, ROOT)
            dst = os.path.join(dst_root, rel)
            under(dst, dst_root)
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            shutil.copyfile(p, dst)
            n += 1
    return n


def mask(text):
    out = list(text)
    n = len(text)
    i = 0
    while i < n:
        c = text[i]
        if c == "/" and text.startswith("//", i):
            j = text.find("\n", i)
            j = n if j < 0 else j
            for k in range(i, j):
                out[k] = " "
            i = j
            continue
        if c == "/" and text.startswith("/*", i):
            d, j = 1, i + 2
            while j < n and d:
                if text.startswith("/*", j):
                    d += 1; j += 2
                elif text.startswith("*/", j):
                    d -= 1; j += 2
                else:
                    j += 1
            for k in range(i, j):
                if out[k] != "\n":
                    out[k] = " "
            i = j
            continue
        m = re.match(r'b?r(#*)"', text[i:i + 40]) if c in "br" else None
        if m and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == "_")):
            close = '"' + m.group(1)
            j = text.find(close, i + m.end())
            j = n if j < 0 else j + len(close)
            for k in range(i, j):
                if out[k] != "\n":
                    out[k] = " "
            i = j
            continue
        if c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            j = min(n, j + 1)
            for k in range(i, j):
                if out[k] != "\n":
                    out[k] = " "
            i = j
            continue
        if c == "'":
            m2 = re.match(r"'(\\u\{[0-9a-fA-F_]+\}|\\x[0-9a-fA-F]{2}|\\.|[^\\'\n])'", text[i:i + 16])
            if m2:
                for k in range(i, i + m2.end()):
                    out[k] = " "
                i += m2.end()
                continue
        i += 1
    return "".join(out)
