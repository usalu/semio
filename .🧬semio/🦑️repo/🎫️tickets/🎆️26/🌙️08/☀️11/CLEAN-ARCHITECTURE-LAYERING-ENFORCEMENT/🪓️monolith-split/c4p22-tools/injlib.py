"""🧩 C4P22 glob emulation helpers shared by posrun.py and posdiff.py: which parent `use` items a staged child receives in the sandbox view, and which child lines they occupy."""
import os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/../rf8-tools")
from rslex import outline

GLOB = "use super::*;"


def parent_uses(root_text):
    ls = root_text.split("\n")
    mods = set(re.findall(r"^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;", root_text, re.M))
    out = []
    for it in outline(root_text):
        if it["kind"] == "use":
            block = [l for l in ls[it["start"]:it["end"] + 1] if not l.lstrip().startswith("//")]
            head = " ".join(block).strip()
            if re.match(r"^(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?use\s+(?:self|super|crate)::", head):
                continue
            if any(re.match(rf"^(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?use\s+{m}::", head) for m in mods):
                continue
            out.extend(block)
    return out


def inject(overlay, rel, data):
    """(new bytes, (first line, last line) of the injected block, 1-based) or (data, None)."""
    text = data.decode("utf-8")
    lines = text.split("\n")
    if GLOB not in lines:
        return data, None
    root_rel = os.path.dirname(os.path.dirname(rel)) + "/🦀️.rs"
    root_path = os.path.join(overlay, root_rel)
    if not os.path.isfile(root_path):
        return data, None
    uses = parent_uses(open(root_path, encoding="utf-8").read())
    at = lines.index(GLOB) + 1
    return "\n".join(lines[:at] + uses + lines[at:]).encode("utf-8"), (at + 1, at + len(uses))


def ranges(overlay):
    """{repo-relative child path: (first, last)} for every staged child with the glob line."""
    out = {}
    for dp, dn, fn in os.walk(overlay):
        for f in fn:
            if f == "🦀️.rs":
                p = os.path.join(dp, f)
                rel = os.path.relpath(p, overlay)
                data = open(p, "rb").read()
                if b"use super::*;" in data:
                    _, r = inject(overlay, rel, data)
                    if r:
                        out[rel] = r
    return out
