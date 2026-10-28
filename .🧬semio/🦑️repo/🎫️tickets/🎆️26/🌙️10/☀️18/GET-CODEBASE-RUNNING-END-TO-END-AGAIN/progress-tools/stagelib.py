"""🧱 Builds a flat staged batch (manifest.json + files/ + base/ + stage.py + verify.sh) from edits applied to the CURRENT live bytes.
An edit is `(rel, fn)` with fn(old_text)->new_text (old_text is None for a new file) or `(rel, bytes)`. Anchored replacements go through `sub(text, old, new)` which demands exactly one occurrence."""
import hashlib, json, os, shutil, stat, sys
HERE = os.path.dirname(os.path.abspath(__file__))
TEMPLATE = os.path.join(HERE, "..", "xplat5-tools", "stage.template.py")
def sha(b): return hashlib.sha256(b).hexdigest()
def sub(text, old, new, count=1):
    n = text.count(old)
    if n != count: raise SystemExit(f"anchor found {n}x (want {count}): {old[:90]!r}")
    return text.replace(old, new)
def after(batch, rel):
    """⏭ Pre-transform: the file as it is once staged batch `batch` is landed (live == its base -> its new text; live == its new text -> unchanged)."""
    def run(text):
        d = os.path.join(HERE, "staged", batch); row = json.load(open(os.path.join(d, "manifest.json"), encoding="utf8"))[rel]
        h = sha(text.encode("utf8"))
        if h == row["new"]: return text
        if h != row["base"]: raise SystemExit(f"{rel[-60:]}: live matches neither the base nor the new text of batch {batch}; rebuild {batch} first")
        return open(os.path.join(d, "files", row["file"]), encoding="utf8").read()
    return run
def after_all(batches, rel):
    """⏭ Chains `after` over every batch of the list that touches `rel`, in landing order."""
    def run(text):
        rows = {b: json.load(open(os.path.join(HERE, "staged", b, "manifest.json"), encoding="utf8")).get(rel) for b in batches}
        h = sha(text.encode("utf8"))
        if any(row is not None and row["new"] == h for row in rows.values()): return text
        for batch in batches:
            if rows[batch] is not None: text = after(batch, rel)(text)
        return text
    return run
def build(batch, root, edits, verify_body, doc):
    out = os.path.join(HERE, "staged", batch)
    shutil.rmtree(out, ignore_errors=True)
    os.makedirs(os.path.join(out, "files")); os.makedirs(os.path.join(out, "base"))
    manifest = {}
    for i, ed in enumerate(edits):
        rel, edit = ed[0], ed[1]
        path = os.path.join(root, rel)
        old = open(path, "rb").read() if os.path.exists(path) else None
        if len(ed) > 2 and old is not None: old = ed[2](old.decode("utf8")).encode("utf8")
        new = edit if isinstance(edit, bytes) else edit(old.decode("utf8") if old is not None else None).encode("utf8")
        if old is not None and new == old: continue
        name = f"{i:02d}" + os.path.splitext(rel)[1]
        open(os.path.join(out, "files", name), "wb").write(new)
        if old is not None: open(os.path.join(out, "base", name), "wb").write(old)
        manifest[rel] = {"file": name, "base": sha(old) if old is not None else None, "new": sha(new)}
    json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    shutil.copy(TEMPLATE, os.path.join(out, "stage.py"))
    s = open(os.path.join(out, "stage.py"), encoding="utf8").read().replace("🌌 Staged batch `universe` (lane rustedit): print's Bazel plan becomes reusable although a dependency reads a root-anchored pattern.", doc, 1)
    open(os.path.join(out, "stage.py"), "w", encoding="utf8").write(s)
    open(os.path.join(out, "verify.sh"), "w").write("#!/bin/sh\n# usage: verify.sh <mirror-root>   (applies this batch to the mirror, then runs the laws)\nset -eu\nROOT=\"$1\"; HERE=\"$(cd \"$(dirname \"$0\")\" && pwd)\"\n" + verify_body)
    os.chmod(os.path.join(out, "verify.sh"), 0o755)
    print(batch, len(manifest), "files ->", out)
