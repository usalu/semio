"""Surgical editor of the BIM editor files for w2-f3-editor: finds every directory by its name suffix (never types an emoji path), replaces anchored text exactly once and keeps the line endings."""
import os
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), *[".."] * 7))


def child(parent, suffix):
    names = [n for n in os.listdir(parent) if n.endswith(suffix)]
    if not names:
        raise SystemExit(f"no {suffix!r} in {parent}")
    return os.path.join(parent, sorted(names)[0])


def subset():
    plugins = child(child(REPO, "s"), "plugins") if False else None
    root = [n for n in os.listdir(REPO) if n.startswith("✏") and n.endswith("s")][0]
    node = os.path.join(REPO, root)
    for suffix in ("plugins", "bim", "artifacts", "model", "standards", "1", "subsets", "any"):
        node = child(node, suffix)
    return node


def at(*suffixes):
    node = subset()
    for suffix in suffixes[:-1]:
        node = child(node, suffix)
    return child(node, suffixes[-1]) if suffixes[-1] != "rs" else os.path.join(node, [n for n in os.listdir(node) if n.endswith(".rs")][0])


def rs(*dirs):
    node = subset()
    for suffix in dirs:
        node = child(node, suffix)
    return os.path.join(node, [n for n in os.listdir(node) if n.endswith(".rs") and n != "mod.rs"][0])


def edit(path, pairs):
    with open(path, encoding="utf8", newline="") as handle:
        text = handle.read()
    crlf = "\r\n" in text
    text = text.replace("\r\n", "\n")
    for old, new in pairs:
        if text.count(old) != 1:
            raise SystemExit(f"{path}: anchor found {text.count(old)} times: {old[:80]!r}")
        text = text.replace(old, new, 1)
    with open(path, "w", encoding="utf8", newline="") as handle:
        handle.write(text.replace("\n", "\r\n") if crlf else text)
    print("edited", os.path.basename(os.path.dirname(path)))
