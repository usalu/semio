"""Splitter: builds child files from line ranges of the ORIGINAL (snapshot) root, rewrites the root with mounts, widens
the visibility of names that cross a new file boundary (name-keyed against masked tokens of every other reachable file).
Usage: split.py <name> plan|apply  (plan writes only to tgm-work/<name>/stage, apply installs stage into the repo)."""
import importlib, json, os, re, shutil, sys, collections
sys.path.insert(0, os.path.dirname(__file__))
from tgmlib import *
from modtree import reachable
import rslex

ITEM_KW = ("fn", "struct", "enum", "trait", "const", "static", "type", "union")
VIS_RX = re.compile(r"^\s*pub(\s*\([^)]*\))?\s")
MEMBER_RX = re.compile(r"^(\s*)(?:(?:default|const|async|unsafe|extern\s+\"[^\"]*\")\s+)*(fn|const|type)\s+([A-Za-z_]\w*)")
FIELD_RX = re.compile(r"^(\s*)([A-Za-z_]\w*)\s*:\s")
POOL = ["🧩", "🧱", "🪢", "🗂", "📚", "🔖", "🪪", "🧭", "🧰", "🪄", "🎛", "🧷", "📌", "🗜", "🧮", "🔬", "🧲", "🪡", "📎", "🧵", "🏺", "🪙", "🔭", "📐", "🧿"]
EMOJI_FILE = F + "/🗑️generated/tgm-emoji-counts.txt"


def folder_emoji(folder):
    return folder[: folder.index("️") + 1] if "️" in folder else folder[0]


def head_line(lines, depths, inlit, start, end):
    i = start
    while i <= end:
        st = lines[i].strip()
        if depths[i] == 0 and not inlit[i] and st and not st.startswith("//") and not st.startswith("#[") and not st.startswith("#!"):
            return i
        if st.startswith("#[") or st.startswith("#!"):
            j = i
            while j + 1 <= end and depths[j + 1] != 0:
                j += 1
            i = j + 1
            continue
        i += 1
    return start


def analyse(text):
    """Return (lines, items) where each item has head index and member/field candidates (line index, kind, name)."""
    lines = text.split("\n")
    depths, inlit = rslex.lex(text)
    items = rslex.outline(text)
    out = []
    for it in items:
        h = head_line(lines, depths, inlit, it["start"], it["end"])
        it = dict(it)
        it["head_idx"] = h
        hs = lines[h]
        it["private"] = not VIS_RX.match(hs) and not hs.lstrip().startswith("pub")
        cands = []
        kind = it["kind"]
        if kind == "impl":
            header = ""
            k = h
            while k <= it["end"]:
                header += lines[k] + " "
                if "{" in lines[k]:
                    break
                k += 1
            header_nogen = header.split("{")[0]
            trait_impl = re.search(r"\bfor\b(?!\s*<)", header_nogen) is not None
            if not trait_impl:
                for li in range(h + 1, it["end"] + 1):
                    if depths[li] == 1 and not inlit[li]:
                        mm = MEMBER_RX.match(lines[li])
                        if mm and not re.match(r"^\s*pub\b", lines[li]):
                            cands.append((li, "method", mm.group(3)))
        elif kind == "struct":
            hdr = lines[h]
            if re.search(r"\bstruct\s+\w+[^;{(]*\(", hdr) and "{" not in hdr:
                if not re.search(r"\(\s*pub\b", hdr) or re.search(r",\s*[A-Za-z_&\[]", hdr.split("(", 1)[1]):
                    cands.append((h, "tuplestruct", it["name"]))
            elif "{" in hdr or (h + 1 <= it["end"] and "{" in lines[h + 1]):
                for li in range(h + 1, it["end"] + 1):
                    if depths[li] == 1 and not inlit[li]:
                        fm = FIELD_RX.match(lines[li])
                        if fm and not re.match(r"^\s*pub\b", lines[li]) and not lines[li].lstrip().startswith(("//", "#")):
                            cands.append((li, "field", fm.group(2)))
        it["cands"] = cands
        out.append(it)
    return lines, depths, inlit, out


def token_sets(files):
    """name -> set(file) for masked word tokens, and method-ish tokens preceded by . or ::"""
    words = {}
    meth = {}
    for f, t in files.items():
        m = mask(t)
        for w in set(re.findall(r"[A-Za-z_][A-Za-z0-9_]*", m)):
            words.setdefault(w, set()).add(f)
        for w in set(re.findall(r"(?:\.|::)\s*([A-Za-z_][A-Za-z0-9_]*)", m)):
            meth.setdefault(w, set()).add(f)
    return words, meth


_MASKS = {}


def masked_of(files, f):
    key = (f, len(files[f]))
    if key not in _MASKS:
        _MASKS[key] = mask(files[f])
    return _MASKS[key]


def load_spec(name):
    return importlib.import_module("specs." + name).SPEC


def build(name, spec):
    t = TARGETS[name]
    snap = snap_dir(name)
    root_rel = t["root"]
    orig = read(os.path.join(snap, root_rel))
    olines = orig.split("\n")
    trailing_nl = orig.endswith("\n")
    if trailing_nl:
        olines = olines[:-1]
    N = len(olines)
    claimed = [None] * (N + 1)
    for ci, ch in enumerate(spec["children"]):
        for a, b in ch["ranges"]:
            assert 1 <= a <= b <= N, (ch["folder"], a, b)
            for k in range(a, b + 1):
                assert claimed[k] is None, f"line {k} claimed twice: {ch['folder']} and {claimed[k]}"
                claimed[k] = ch["folder"]
    base = os.path.dirname(root_rel)
    childtexts = {}
    mounts = {}
    for ch in spec["children"]:
        folder, mod = ch["folder"], ch["mod"]
        kind = ch.get("kind", "lines")
        em = folder_emoji(folder)
        if kind == "lines":
            body = []
            for a, b in ch["ranges"]:
                if body:
                    body.append("")
                body.extend(olines[a - 1:b])
            body = [re.sub(r"\bpub\(super\)", "pub(crate)", l) for l in body]
            hoisted = []
            blines, bdepths, binlit, bitems = analyse("\n".join(body))
            drop = set()
            for bit in bitems:
                if bit["kind"] == "use" and bit["private"]:
                    k0 = bit["head_idx"]
                    while k0 > 0 and blines[k0 - 1].strip().startswith("#["):
                        k0 -= 1
                    hoisted.extend(blines[k0:bit["end"] + 1])
                    drop.update(range(k0, bit["end"] + 1))
            if drop:
                body = [l for i, l in enumerate(body) if i not in drop]
            ch["_hoisted"] = hoisted
            doc = ch["doc"]
            assert doc.startswith(em), (folder, doc[:5])
            used = set()
            for bl in body:
                dm = re.match(r"^\s*(?:///|//!|\*)\s*(\S)", bl)
                if dm:
                    used.add(dm.group(1))
            lead = em.replace("\ufe0f", "")
            if lead in used or lead + "\ufe0f" in used:
                pick = next(c for c in POOL if c not in used and c != lead)
                ch["_doc_swap"] = (lead, pick)
                doc = pick + "\ufe0f" + doc[len(em):]
            head = ["//! " + dl if dl else "//!" for dl in doc.split("\n")]
            text = "\n".join(head + ["", "use super::*;", ""] + body) + "\n"
            mount = [f'#[path = "{folder}/🦀️.rs"]', f"mod {mod};", f"pub use {mod}::*;"]
        else:
            (a, b), = ch["ranges"]
            mh = olines[a - 1]
            mm = re.match(r"^(pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*\{\s*$", mh)
            assert mm and mm.group(2) == mod, (folder, mh)
            assert olines[b - 1] == "}", (folder, olines[b - 1])
            inner = olines[a:b - 1]
            inner = [l[4:] if l.startswith("    ") else l for l in inner]
            if ch.get("hoist_doc"):
                di = [k for k, l in enumerate(inner[:8]) if l.startswith("//!")]
                assert di, folder
                first, last = di[0], di[0]
                while last + 1 < len(inner) and inner[last + 1].startswith("//!"):
                    last += 1
                docl = inner[first:last + 1]
                inner = docl + [""] + inner[:first] + inner[last + 1:]
                while len(inner) > 1 and inner[len(docl) + 1:len(docl) + 2] == [""] and inner[len(docl) + 2:len(docl) + 3] == [""]:
                    del inner[len(docl) + 1]
            if ch.get("doc") and not inner[0].startswith("//!"):
                used = set()
                for bl in inner:
                    dm = re.match(r"^\s*(?:///|//!|\*)\s*(\S)", bl)
                    if dm:
                        used.add(dm.group(1))
                lead = em.replace("\ufe0f", "")
                dd = ch["doc"]
                if lead in used or lead + "\ufe0f" in used:
                    pick = next(c for c in POOL if c not in used and c != lead)
                    ch["_doc_swap"] = (lead, pick)
                    dd = pick + "\ufe0f" + dd[len(em):]
                inner = ["//! " + dd, ""] + inner
            assert inner[0].startswith("//! "), (folder, inner[0][:20])
            if not ch.get("doc"):
                assert inner[0].startswith("//! " + em), (folder, inner[0][:20])
            inner = [re.sub(r'include!\("(?!\.\./|/)', 'include!("../', l) for l in inner]
            inner = [re.sub(r"\bpub\(super\)", "pub(super)", l) for l in inner]
            text = "\n".join(inner) + "\n"
            mount = [f'#[path = "{folder}/🦀️.rs"]', f"{mm.group(1) or ''}mod {mod};"]
        childtexts[folder] = text
        mounts[ch["ranges"][0][0]] = (ch, mount)
    root = []
    k = 1
    while k <= N:
        if claimed[k] is None:
            root.append(olines[k - 1])
            k += 1
            continue
        ch, mount = mounts.get(k, (None, None))
        folder = claimed[k]
        if ch is not None:
            root.extend(ch.get("_hoisted", []))
            root.extend(mount)
        e = k
        while e + 1 <= N and claimed[e + 1] == folder:
            e += 1
        k = e + 1
    rtext = "\n".join(root) + "\n"
    return orig, rtext, childtexts, base


def widen(name, spec, rtext, childtexts, base, report):
    t = TARGETS[name]
    snap = snap_dir(name)
    others = {}
    reach = reachable(os.path.join(snap, t["lib"]))
    root_abs = os.path.join(snap, t["root"])
    for f in reach:
        if f == root_abs:
            continue
        others[os.path.relpath(f, snap)] = read(f)
    files = dict(others)
    files[t["root"]] = rtext
    for folder, ct in childtexts.items():
        files[f"{base}/{folder}/🦀️.rs"] = ct
    words, meth = token_sets(files)
    newc = {}
    for ch in spec["children"]:
        folder = ch["folder"]
        if ch.get("kind", "lines") != "lines":
            newc[folder] = childtexts[folder]
            continue
        me = f"{base}/{folder}/🦀️.rs"
        lines, depths, inlit, items = analyse(childtexts[folder])
        edits = []
        for it in items:
            kind = it["kind"]
            if kind in ITEM_KW and it["private"] and it["name"]:
                nm = it["name"]
                users = sorted(f for f in words.get(nm, ()) if f != me)
                if users:
                    edits.append((it["head_idx"], "item", nm, users))
            if kind == "use" and not it["vis"] and it["start"] > 5:
                report["uses"].append((folder, lines[it["head_idx"]].strip()))
            if kind == "mod" and it["name"] and it["private"]:
                nm = it["name"]
                users = sorted(f for f in words.get(nm, ()) if f != me)
                if users:
                    report["privmods"].append((folder, nm, users))
            for (li, ck, nm) in it["cands"]:
                if ck == "method":
                    users = sorted(f for f in meth.get(nm, ()) if f != me)
                elif ck == "field":
                    sname = it["name"]
                    users = []
                    for f in sorted(words.get(nm, ())):
                        if f == me:
                            continue
                        mt = masked_of(files, f)
                        if re.search(r"\." + re.escape(nm) + r"\b(?!\s*\()", mt) or re.search(r"\b(?:" + re.escape(sname) + r"|Self)\b[^;{}]*\{[^{}]*\b" + re.escape(nm) + r"\b", mt):
                            users.append(f)
                else:
                    users = sorted(f for f in words.get(it["name"], ()) if f != me)
                if users:
                    edits.append((li, ck, nm, users))
        for li, ck, nm, users in edits:
            ln = lines[li]
            if ck == "tuplestruct":
                report["tuple"].append((folder, nm))
                continue
            if ln.lstrip().startswith("#["):
                report["attrline"].append((folder, nm))
                continue
            ind = len(ln) - len(ln.lstrip())
            lines[li] = ln[:ind] + "pub(crate) " + ln[ind:]
            report["widened"].append((folder, ck, nm, [os.path.basename(os.path.dirname(u)) for u in users[:4]]))
        newc[folder] = "\n".join(lines)
    return newc


def main():
    name, mode = sys.argv[1], sys.argv[2]
    spec = load_spec(name)
    t = TARGETS[name]
    orig, rtext, childtexts, base = build(name, spec)
    report = dict(widened=[], uses=[], privmods=[], tuple=[], attrline=[])
    newc = widen(name, spec, rtext, childtexts, base, report)
    stage = f"{W}/{name}/stage"
    if os.path.exists(stage):
        shutil.rmtree(stage)
    under(stage, W)
    write(f"{stage}/{t['root']}", rtext)
    for folder, ct in newc.items():
        write(f"{stage}/{base}/{folder}/🦀️.rs", ct)
    for ch in spec["children"]:
        if ch.get("_doc_swap"):
            print("   header emoji swapped", ch["folder"], ch["_doc_swap"])
    print(f"root: {len(rtext.splitlines())} lines (was {len(orig.splitlines())})")
    for ch in spec["children"]:
        print(f"  {ch['folder']:28} {len(newc[ch['folder']].splitlines()):5} lines")
    for k, v in report.items():
        print(f"{k}: {len(v)}")
        if k != "widened":
            for x in v:
                print("   ", x)
    for w in report["widened"]:
        print("   widen", w)
    write(f"{F}/🗑️generated/tgm-{name}-plan.json", json.dumps(report, ensure_ascii=False, indent=1))
    if mode == "restore":
        for rel in [f"{base}/{f}/🦀️.rs" for f in newc]:
            dst = os.path.join(ROOT, rel)
            if os.path.exists(dst):
                assert read(dst) == read(os.path.join(stage, rel)) or "--force" in sys.argv, f"{dst} differs from my stage; refusing"
                os.remove(dst)
                try:
                    os.rmdir(os.path.dirname(dst))
                except OSError:
                    print("kept non-empty", os.path.dirname(dst))
        write(os.path.join(ROOT, t["root"]), read(os.path.join(snap_dir(name), t["root"])))
        print("RESTORED")
    if mode == "apply":
        for rel in [t["root"]] + [f"{base}/{f}/🦀️.rs" for f in newc]:
            src = os.path.join(stage, rel)
            dst = os.path.join(ROOT, rel)
            if rel == t["root"]:
                cur = read(dst)
                assert sha(cur) == json.load(open(f"{W}/{name}/snap.json"))["sha"], "root changed since snapshot; refusing"
            else:
                assert not os.path.exists(dst), f"new child exists: {dst}"
            write(dst, read(src))
        print("APPLIED")


if __name__ == "__main__":
    main()
