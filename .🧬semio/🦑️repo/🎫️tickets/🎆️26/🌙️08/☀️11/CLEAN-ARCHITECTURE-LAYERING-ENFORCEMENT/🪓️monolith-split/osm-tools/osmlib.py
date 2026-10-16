"""Shared helpers of the osm (OS-SERVICES-MCP-SPLITS) tools."""
import collections, hashlib, json, os, re, shutil, sys

ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
W = F + "/🗑️generated/osm-work"
M = "🧰️framework/🛍️products/💻️os/🔨️modules"
SKIP = {"node_modules", "target", "🗑️generated", ".git", "dist"}

TARGETS = {
    "services": dict(file=f"{M}/🛎️services/🦀️.rs", tree=f"{M}/🛎️services", lib=f"{M}/🛎️services/📦️packages/🦀️rust/🦀️.rs", crate="semio_framework_os_services", cfgs=["lib", "test"]),
    "transport": dict(file=f"{M}/🌉️mcp/🚚️transport/🦀️.rs", tree=f"{M}/🌉️mcp", lib=f"{M}/🌉️mcp/📮️http-transport/🦀️.rs", crate="semio_framework_os_mcp_http_transport", cfgs=["lib", "test"]),
    "remote": dict(file=f"{M}/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs", tree=f"{M}/🌉️mcp", lib=f"{M}/🌉️mcp/🌍️hub-remote/🦀️.rs", crate="semio_framework_os_mcp_hub_remote", cfgs=["lib", "test"]),
    "bridge": dict(file=f"{M}/🌉️mcp/🧵️bridge/🦀️.rs", tree=f"{M}/🌉️mcp", lib=f"{M}/🌉️mcp/🧶️bridge-codec/🦀️.rs", crate="semio_framework_os_mcp_bridge_codec", cfgs=["lib", "test"]),
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


def multiset(ls):
    return collections.Counter(l.strip() for l in ls if l.strip() != "")


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
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            shutil.copyfile(p, dst)
            n += 1
    return n
