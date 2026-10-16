"""Guarded repo edits for the otcp slice: exact-anchor replace with count assertions, pre-edit copies, touched list."""
import os, shutil, sys
ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
PRE = F + "/🗑️generated/otcp-work/pre"
TOUCHED = F + "/🗑️generated/otcp-work/touched.txt"


def rel(path):
    return os.path.relpath(path, ROOT) if os.path.isabs(path) else path


def abspath(path):
    return path if os.path.isabs(path) else os.path.join(ROOT, path)


def note(kind, path):
    os.makedirs(os.path.dirname(TOUCHED), exist_ok=True)
    with open(TOUCHED, "a", encoding="utf-8") as f:
        f.write(f"{kind}\t{rel(path)}\n")


def pre_copy(path):
    p = abspath(path)
    dst = os.path.join(PRE, rel(p))
    if os.path.exists(p) and not os.path.exists(dst):
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        shutil.copyfile(p, dst)


def read(path):
    with open(abspath(path), encoding="utf-8") as f:
        return f.read()


def write(path, text):
    with open(abspath(path), "w", encoding="utf-8") as f:
        f.write(text)


def edit(path, old, new, count=1):
    t = read(path)
    assert t.count(old) == count, f"{rel(path)}: anchor count {t.count(old)} != {count}: {old[:80]!r}"
    pre_copy(path)
    write(path, t.replace(old, new))
    note("edit", path)


def create(path, text):
    p = abspath(path)
    if os.path.exists(p):
        assert read(p) == text, f"refusing to overwrite differing {rel(p)}"
        return
    os.makedirs(os.path.dirname(p), exist_ok=True)
    write(p, text)
    note("create", p)


def replace_lines(path, text_old_block, text_new_block):
    edit(path, text_old_block, text_new_block)


def put(path, text):
    """write a file this slice created (companion crate files): overwrite allowed, never for pre-existing foreign files."""
    p = abspath(path)
    if os.path.exists(p):
        assert rel(p) in {l.split("\t")[1].strip() for l in open(TOUCHED, encoding="utf-8") if l.startswith("create")}, f"{rel(p)} was not created by this slice"
    else:
        os.makedirs(os.path.dirname(p), exist_ok=True)
        note("create", p)
    write(p, text)
