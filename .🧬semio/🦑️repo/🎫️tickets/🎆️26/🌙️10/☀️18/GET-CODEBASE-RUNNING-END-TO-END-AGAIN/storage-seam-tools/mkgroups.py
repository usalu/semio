#!/usr/bin/env python3
"""mkgroups.py: derives the staged set from the verified work copy. Compares /private/tmp/storage-seam/work against /private/tmp/storage-seam/base (the pristine snapshot the work started from): new files go to staged/files, changed files become anchored EDITS (token-level hunks, expanded inside their own line until the anchor is unique in the BASE file, then checked against the LIVE file), whole-file rewrites go to REWRITES (sha256 of the old bytes). Writes groups/g1.py..g3.py. Nothing under the checkout is modified."""
import difflib, hashlib, json, os, re, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
LIVE = "/Users/ueli/Documents/semio"
BASE, WORK = "/private/tmp/storage-seam/base", "/private/tmp/storage-seam/work"
STAGED = os.path.join(HERE, "staged", "files")
sys.path.insert(0, os.path.abspath(os.path.join(HERE, "..")))
from winpath import place
TOP = "🧰️framework"
REWRITE_SUFFIXES = json.load(open(os.path.join(HERE, "rewrites.json"))) if os.path.exists(os.path.join(HERE, "rewrites.json")) else []
SKIP = ("node_modules", "/art/", ".envwire.ts", ".DS_Store")
TESTS = "🧪️tests"


def files(root):
    out = {}
    for directory, _, names in os.walk(os.path.join(root, TOP)):
        if any(s in directory + "/" for s in SKIP):
            continue
        for name in names:
            if name == ".DS_Store" or name == ".envwire.ts":
                continue
            full = os.path.join(directory, name)
            out[os.path.relpath(full, root)] = full
    return out


def read(path):
    with open(path, "rb") as handle:
        return handle.read()


TOKEN = re.compile(r"\w+|\s+|[^\w\s]")


def unique_in(text, piece):
    return text.count(piece) == 1


def hunks_for(old_text, new_text):
    """List of (old, new) anchored replacements that turn old_text into new_text."""
    a, b = old_text.splitlines(keepends=True), new_text.splitlines(keepends=True)
    out = []
    matcher = difflib.SequenceMatcher(None, a, b, autojunk=False)
    for tag, i1, i2, j1, j2 in matcher.get_opcodes():
        if tag == "equal":
            continue
        old_block, new_block = "".join(a[i1:i2]), "".join(b[j1:j2])
        if tag == "replace" and i2 - i1 == 1 and j2 - j1 == 1 and len(old_block) > 160:
            ta, tb = TOKEN.findall(old_block), TOKEN.findall(new_block)
            sm = difflib.SequenceMatcher(None, ta, tb, autojunk=False)
            ops = [op for op in sm.get_opcodes() if op[0] != "equal"]
            for tag2, k1, k2, l1, l2 in ops:
                pre, post = "".join(ta[:k1]), "".join(ta[k2:])
                core_old, core_new = "".join(ta[k1:k2]), "".join(tb[l1:l2])
                left, right = 0, 0
                while True:
                    o = pre[len(pre) - left:] + core_old + post[:right]
                    if len(o) >= 24 and unique_in(old_text, o):
                        break
                    if left < len(pre) and (left <= right or right >= len(post)):
                        left += 1
                    elif right < len(post):
                        right += 1
                    else:
                        o = None
                        break
                if o is None:
                    out = out + [(old_block, new_block)]
                    break
                n = pre[len(pre) - left:] + core_new + post[:right]
                out.append((o, n))
            else:
                continue
            continue
        if tag == "insert" or tag == "delete" or not unique_in(old_text, old_block):
            lo, hi = i1, i2
            while True:
                ob = "".join(a[lo:hi])
                nb = "".join(a[lo:i1]) + new_block + "".join(a[i2:hi])
                if ob and unique_in(old_text, ob):
                    out.append((ob, nb))
                    break
                if lo > 0:
                    lo -= 1
                if hi < len(a):
                    hi += 1
                if lo == 0 and hi == len(a) and not ob:
                    raise SystemExit("cannot anchor")
            continue
        out.append((old_block, new_block))
    return out


def simulate(old_text, edits):
    text = old_text
    for old, new in edits:
        if text.count(old) != 1:
            return None
        text = text.replace(old, new)
    return text


def idempotent(edits, final_text):
    """Every edit must read as applied (old gone, new present) in the final text, whatever later edits touch."""
    for old, new in edits:
        if old in new:
            if final_text.count(new) != 1:
                return False
        elif final_text.count(old) != 0 or final_text.count(new) < 1:
            return False
    return True


def line_edits(old_text, new_text):
    a, b = old_text.splitlines(keepends=True), new_text.splitlines(keepends=True)
    edits = []
    for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes():
        if tag == "equal":
            continue
        lo, hi = i1, i2
        while True:
            ob = "".join(a[lo:hi])
            nb = "".join(a[lo:i1]) + "".join(b[j1:j2]) + "".join(a[i2:hi])
            if ob and old_text.count(ob) == 1:
                edits.append((ob, nb))
                break
            lo, hi = max(0, lo - 1), min(len(a), hi + 1)
    return edits


def edits_for(rel):
    old_text, new_text = read(os.path.join(BASE, rel)).decode("utf-8"), read(os.path.join(WORK, rel)).decode("utf-8")
    edits = hunks_for(old_text, new_text)
    if simulate(old_text, edits) != new_text or not idempotent(edits, new_text):
        edits = line_edits(old_text, new_text)
        if simulate(old_text, edits) != new_text or not idempotent(edits, new_text):
            raise SystemExit("cannot derive anchored idempotent edits for " + rel)
    return edits


def group_of(rel, new):
    if TESTS in rel:
        return "g3"
    if new:
        return "g1"
    return "g2"


def main():
    base, work = files(BASE), files(WORK)
    newf = sorted(p for p in work if p not in base)
    changed = sorted(p for p in work if p in base and read(base[p]) != read(work[p]))
    removed = sorted(p for p in base if p not in work)
    if removed:
        raise SystemExit("removed files are not supported: %s" % removed[:3])
    if os.path.exists(STAGED):
        shutil.rmtree(STAGED)
    groups = {"g1": {"edits": [], "new": [], "rewrites": []}, "g2": {"edits": [], "new": [], "rewrites": []}, "g3": {"edits": [], "new": [], "rewrites": []}}
    for rel in newf:
        place(STAGED, rel, open(work[rel], "rb").read())
        groups[group_of(rel, True)]["new"].append(rel)
    for rel in changed:
        g = group_of(rel, False)
        if any(rel.endswith(s) for s in REWRITE_SUFFIXES):
            place(STAGED, rel, open(work[rel], "rb").read())
            groups[g]["rewrites"].append((rel, hashlib.sha256(read(base[rel])).hexdigest()))
            continue
        for old, new in edits_for(rel):
            groups[g]["edits"].append((rel, old, new))
    titles = {
        "g1": "registry, selection policy, backend adapters, geometry, hdiutil and codesign ports (new files)",
        "g2": "rewire: platform facade and neutral contract dispatch through the registry; directory partition runs on macOS; hdiutil and codesign literals replaced by the ports",
        "g3": "tests: selection table with Python oracle, ownership and literal laws, per-backend conformance suite, dispatch and directory-host updates",
    }
    first = next(iter(groups["g1"]["new"]), None)
    for name, data in groups.items():
        lines = ["NAME = %r" % name, "TITLE = %r" % titles[name], "REQUIRES = %r" % ([] if name == "g1" else [first])]
        lines.append("EDITS = [")
        for rel, old, new in data["edits"]:
            lines.append("    (%r, %r, %r)," % (rel, old, new))
        lines.append("]")
        lines.append("NEW = %r" % data["new"])
        lines.append("REWRITES = %r" % data["rewrites"])
        with open(os.path.join(HERE, "groups", name + ".py"), "w", encoding="utf-8") as handle:
            handle.write("\n".join(lines) + "\n")
        print(name, "edits", len(data["edits"]), "new", len(data["new"]), "rewrites", len(data["rewrites"]))
    live_issues = 0
    for name, data in groups.items():
        for rel, old, new in data["edits"]:
            lp = os.path.join(LIVE, rel)
            t = read(lp).decode("utf-8") if os.path.exists(lp) else ""
            if t.count(old) != 1 and t.count(new) < 1:
                live_issues += 1
                print("LIVE DRIFT", rel, repr(old[:70]))
    print("live drift:", live_issues)


if __name__ == "__main__":
    main()
