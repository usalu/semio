#!/usr/bin/env python3
"""🧱️ Persistent-local statics owned by a crate but declared in the reactor root block (`component_persistent_local!`, WP4 section 10): the owner declares them `pub static`, the root re-imports them."""
import re
from lib import *
import flipcore as F

ROOT = PLUG + "/⚛️reactor/🦀️.rs"
OPEN = "crate::component_persistent_local! {"


def block(text):
    i = text.index(OPEN)
    j, depth = text.index("{", i), 0
    k = j
    while True:
        depth += text[k] == "{"
        depth -= text[k] == "}"
        if depth == 0:
            break
        k += 1
    return i, j + 1, k


def entries(body):
    out, pending = [], []
    for line in body.split("\n"):
        if not line.strip() and not pending:
            continue
        pending.append(line)
        m = re.search(r"\bstatic\s+(\w+)\s*:", line)
        if m and line.rstrip().endswith(";"):
            out.append((m.group(1), pending))
            pending = []
    return out


def owned_block(names, uses="", rewrite=()):
    """text appended to the owner root: its imports and the statics block (empty when the root no longer declares them)"""
    text = read(ROOT)
    if OPEN not in text:
        return ""
    i, a, b = block(text)
    lines = []
    for name, ls in entries(text[a:b]):
        if name in names:
            lines += [re.sub(r"\bstatic\b", "pub static", l, count=1) if re.search(r"\bstatic\s+\w+\s*:", l) else l for l in ls]
    body = "\n".join(lines)
    for a, b in rewrite:
        body = body.replace(a, b)
    return "\n" + uses + "\nsemio_framework_plugin_abi::component_persistent_local! {\n" + body + "\n}\n"


def leave_root(owned):
    """edit of the reactor root: the statics leave the block and come back through one `pub(crate) use` per owner crate"""
    def fn(text):
        for lib, names in owned.items():
            i, a, b = block(text)
            keep = []
            for name, ls in entries(text[a:b]):
                if name not in names:
                    keep += ls
            text = text[:a] + "\n" + "\n".join(keep) + "\n" + text[b:]
        i, a, b = block(text)
        end = b + 1
        uses = "".join("\npub(crate) use " + lib + "::{" + ", ".join(sorted(names)) + "};" for lib, names in owned.items())
        return text[:end] + uses + text[end:]
    return fn


def import_owned(names):
    """`use crate::{..}` of the owned statics a file of the owner crate names"""
    def fn(text):
        used = sorted(n for n in names if re.search(r"(?<![A-Za-z0-9_])" + n + r"(?![A-Za-z0-9_])", re.sub(r"//[^\n]*", "", text)))
        if not used:
            return text
        pos = F.header_pos(text)
        return text[:pos] + ("use crate::" + used[0] if len(used) == 1 else "use crate::{" + ", ".join(used) + "}") + ";\n" + text[pos:]
    return fn


def widen_types(types):
    """the types behind a `pub static` become `pub` (`#[doc(hidden)]`)"""
    def fn(text):
        for t in types:
            text = re.sub(r"((?:#\[[^\n]*\]\n)*)pub\((?:super|crate)\) struct " + t + r"\b", lambda m: m.group(1) + "#[doc(hidden)]\npub struct " + t, text)
        return text
    return fn
