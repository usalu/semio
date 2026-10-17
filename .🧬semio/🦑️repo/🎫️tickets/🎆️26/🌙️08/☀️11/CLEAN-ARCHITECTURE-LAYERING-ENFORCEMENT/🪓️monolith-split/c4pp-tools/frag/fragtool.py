"""✂️ C4PP include-fragment splitter v2 for flat test modules (descendant of c4p39 fragsplit/verify/lossless/incl/install).
New over v2 of c4p39: chunks at brace depth 0 or 1 (an inline `mod x { ... }` body keeps its module identity), region markers that span several chunks are closed and reopened in each fragment (balanced pairs per file), a region opened in the root header is closed in the root, fragments are dedented by their depth, relative include literals get one more `../`.
usage: fragtool.py stage|check|install <tag>... ; check takes `stage` or `live` first: fragtool.py check live <tag>..."""
import collections, hashlib, json, os, re, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
T = os.path.dirname(HERE)
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/rf8-tools")
sys.path.insert(0, HERE)
from rslex import lex, outline, multiline_literal_lines
import tables

REPO = "/Users/ueli/Documents/semio"
STAGE = "/Users/ueli/.cache/c4pp/stage"
DOCABLE = {"fn", "struct", "enum", "trait", "impl", "mod", "const", "static", "type", "union"}
RGN_OPEN = re.compile(r"^\s*//\s?#region\b")
RGN_CLOSE = re.compile(r"^\s*//\s?#endregion\b")
INC_RX = re.compile(r'include(?:_str|_bytes)?!\(\s*"([^"]+)"')
sha = lambda b: hashlib.sha256(b).hexdigest()


def read_lines(path):
    ls = open(path, encoding="utf-8").read().split("\n")
    if ls and ls[-1] == "":
        ls.pop()
    return ls


def back_up(ls, k, indent):
    while k > 1:
        prev = ls[k - 2]
        st = prev.strip()
        if st and not RGN_CLOSE.match(prev) and (st.startswith("///") or st.startswith("#[") or st.startswith("//")):
            k -= 1
        else:
            break
    return k


def regions(ls):
    stack, out = [], []
    for i, l in enumerate(ls, 1):
        if RGN_OPEN.match(l):
            stack.append(len(out))
            out.append(dict(open=i, close=None, text=l.strip()))
        elif RGN_CLOSE.match(l):
            assert stack, ("endregion without region", i)
            out[stack.pop()]["close"] = i
    return out


def closer_text(text):
    return re.sub(r"#region", "#endregion", text, count=1)


def stage(tag):
    tb = tables.TABLES[tag]
    orig_path = f"{T}/orig/{tb['orig']}"
    ls = read_lines(orig_path)
    src = "\n".join(ls) + "\n"
    depths, inlit = lex(src)
    rows = tb["rows"]
    starts, ends = [], []
    for i, r in enumerate(rows):
        item, folder, doc = r[0], r[1], r[2]
        d = r[3] if len(r) > 3 else tb.get("depth", 0)
        s = back_up(ls, item, d)
        starts.append(s)
    assert starts == sorted(starts) and len(set(starts)) == len(starts), (tag, starts)
    for i, r in enumerate(rows):
        explicit = r[4] if len(r) > 4 else None
        if explicit:
            b = explicit
        else:
            b = (starts[i + 1] - 1) if i + 1 < len(rows) else len(ls)
        while not ls[b - 1].strip():
            b -= 1
        ends.append(b)
    regs = regions(ls)
    chunk_of = {}
    for i in range(len(rows)):
        for k in range(starts[i], ends[i] + 1):
            chunk_of[k] = i
    for g in regs:
        oc = chunk_of.get(g["open"])
        cc = chunk_of.get(g["close"]) if g["close"] else None
        if oc is None:
            assert g["close"] is None or cc is None, (tag, "region opened in root closes in a chunk", g)
        else:
            assert g["close"] is not None, (tag, "region in chunk without close", g)
    files, chunks_info = {}, []
    for i, r in enumerate(rows):
        item, folder, doc = r[0], r[1], r[2]
        d = r[3] if len(r) > 3 else tb.get("depth", 0)
        a, b = starts[i], ends[i]
        assert depths[a - 1] == d and not inlit[a - 1], (tag, folder, a, "start depth", depths[a - 1], "wanted", d, ls[a - 1][:60])
        nxt = depths[b] if b < len(depths) else 0
        assert nxt == d, (tag, folder, b, "end depth", nxt, "wanted", d, ls[b - 1][:60])
        assert min(depths[a - 1:b]) >= d, (tag, folder, "chunk dips below depth", d)
        assert not inlit[a - 1] and not (b < len(inlit) and inlit[b]), (tag, folder, "chunk edge inside a multi-line literal")
        assert d == 0 or not any(inlit[a - 1:b]), (tag, folder, "multi-line literal in an indented chunk")
        body = ls[a - 1:b]
        pad = "    " * d
        ded = []
        for l in body:
            if l.strip() == "":
                ded.append("")
            else:
                assert l.startswith(pad), (tag, folder, "line not indented", l[:60])
                ded.append(l[len(pad):])
        reopen = [g for g in regs if chunk_of.get(g["open"]) is not None and chunk_of[g["open"]] < i and g["close"] is not None and chunk_of.get(g["close"], -1) >= i]
        reopen = sorted(reopen, key=lambda g: g["open"])
        st = [(g["text"], g["text"]) for g in reopen]
        pre = [g["text"] for g in reopen]
        for l in ded:
            if RGN_OPEN.match(l):
                st.append((l.strip(), l.strip()))
            elif RGN_CLOSE.match(l):
                st.pop()
        post = [closer_text(t) for t, _ in reversed(st)]
        rebased = 0
        for j, l in enumerate(ded):
            if re.search(r'include(?:_str|_bytes)?!\(\s*"[^"/]', l) or re.search(r'include(?:_str|_bytes)?!\(\s*"\.\./', l):
                ded[j] = re.sub(r'(include(?:_str|_bytes)?!\(\s*)"(?!/)', r'\1"../', l)
                rebased += 1
        while ded and ded[0].strip() == "":
            ded.pop(0)
        while ded and ded[-1].strip() == "":
            ded.pop()
        last = next((l for l in reversed(ded) if l.strip()), "")
        assert not last.strip().startswith("///") and not last.strip().startswith("#["), (tag, folder, "fragment ends with orphan doc or attribute", last)
        its = outline("\n".join(ded) + "\n")
        first = its[0] if its else None
        marker = "///" if first and first["kind"] in DOCABLE and not pre else "//"
        out = [f"{marker} {doc}", ""] + pre + ([""] if pre else []) + ded + post
        files[folder] = "\n".join(out) + "\n"
        chunks_info.append(dict(folder=folder, depth=d, start=a, end=b, lines=len(out), reopen=len(pre), close=len(post), rebased=rebased, marker=marker, first=(first["kind"], first["name"]) if first else None))
    skip = set(chunk_of)
    anchors = {starts[i]: i for i in range(len(rows))}
    root, root_stack = [], []
    for k in range(1, len(ls) + 1):
        if k in anchors:
            i = anchors[k]
            d = rows[i][3] if len(rows[i]) > 3 else tb.get("depth", 0)
            if root and root[-1].strip():
                root.append("")
            root.append("    " * d + f'include!("{rows[i][1]}/🦀️.rs");')
        if k in skip:
            continue
        if root and root[-1].lstrip().startswith("include!(") and ls[k - 1].strip():
            root.append("")
        if RGN_OPEN.match(ls[k - 1]):
            root_stack.append(ls[k - 1].strip())
        elif RGN_CLOSE.match(ls[k - 1]):
            root_stack.pop()
        root.append(ls[k - 1])
    for t in reversed(root_stack):
        root.append(closer_text(t))
    tidy = []
    for l in root:
        if l.strip() == "" and tidy and tidy[-1].strip() == "":
            continue
        if l.strip() == "}" and tidy and tidy[-1].strip() == "":
            tidy.pop()
        tidy.append(l)
    root_text = "\n".join(tidy).rstrip("\n") + "\n"
    base = os.path.dirname(tb["file"])
    out = {tb["file"]: root_text}
    for folder, txt in files.items():
        out[f"{base}/{folder}/🦀️.rs"] = txt
    d0 = f"{STAGE}/{tag}"
    subprocess.run(["rm", "-rf", d0])
    for rel, txt in out.items():
        p = f"{d0}/{rel}"
        os.makedirs(os.path.dirname(p), exist_ok=True)
        open(p, "w", encoding="utf-8").write(txt)
    spec = dict(tag=tag, file=tb["file"], original=orig_path, stage=d0, chunks=chunks_info, root_lines=root_text.count("\n"), root_synth_closers=len(root_stack))
    json.dump(spec, open(f"{T}/specs/{tag}.json", "w", encoding="utf-8"), indent=1, ensure_ascii=False)
    print(tag, "root", root_text.count("\n"), "children", {c["folder"]: c["lines"] for c in chunks_info})
    print("   reopened/closed markers:", sum(c["reopen"] for c in chunks_info), sum(c["close"] for c in chunks_info), "root synthesized closers", len(root_stack), "rebased literals", sum(c["rebased"] for c in chunks_info))


TESTRX = re.compile(r"#\[\s*(?:[\w:]*::)?(?:test|async_test|tokio::test)\b[^\]]*\]\s*(?:#\[[^\]]*\]\s*|//[^\n]*\n\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+(\w+)")
DOCRX = re.compile(r"^//[!/]? (\S+)")


def check(mode, tag):
    spec = json.load(open(f"{T}/specs/{tag}.json", encoding="utf-8"))
    base = REPO if mode == "live" else spec["stage"]
    root_rel = spec["file"]
    fdir = os.path.dirname(root_rel)
    files = [root_rel] + [f"{fdir}/{c['folder']}/🦀️.rs" for c in spec["chunks"]]
    problems, texts = [], {}
    for rel in files:
        p = os.path.join(base, rel)
        if not os.path.exists(p):
            problems.append(f"missing {rel}")
        else:
            texts[rel] = open(p, encoding="utf-8").read()
    if problems:
        print(tag, "PROBLEMS", problems)
        return False
    orig = open(spec["original"], encoding="utf-8").read()
    kids = {c["folder"]: texts[f"{fdir}/{c['folder']}/🦀️.rs"] for c in spec["chunks"]}
    def derebase(l):
        return re.sub(r'(include(?:_str|_bytes)?!\(\s*)"\.\./', r'\1"', l, count=1) if INC_RX.search(l) else l
    strip_nb = lambda ls: [l.strip() for l in ls if l.strip()]
    spliced = []
    for line in texts[root_rel].split("\n"):
        m = re.fullmatch(r'include!\("([^"]+)/🦀️\.rs"\);', line.strip())
        if m:
            body = kids[m.group(1)].split("\n")
            assert re.match(r"^//[/]? \S", body[0]), (m.group(1), body[0][:50])
            spliced.extend(derebase(l) for l in body[1:])
        else:
            spliced.append(line)
    orig_seq = strip_nb(orig.split("\n"))
    new_seq = strip_nb(spliced)
    nm = lambda seq: [l for l in seq if not (RGN_OPEN.match(l) or RGN_CLOSE.match(l))]
    seq_equal = nm(orig_seq) == nm(new_seq)
    if not seq_equal:
        problems.append("code sequence differs from the snapshot")
        for a, b in zip(nm(orig_seq), nm(new_seq)):
            if a != b:
                problems.append(f"first difference: {a[:80]!r} vs {b[:80]!r}")
                break
    mo = collections.Counter(l for l in orig_seq if RGN_OPEN.match(l) or RGN_CLOSE.match(l))
    mn = collections.Counter(l for l in new_seq if RGN_OPEN.match(l) or RGN_CLOSE.match(l))
    marker_extra = mn - mo
    marker_gone = mo - mn
    allowed_texts = {l for l in mo} | {closer_text(l) for l in mo if RGN_OPEN.match(l)}
    for l in marker_extra:
        if l not in allowed_texts:
            problems.append(f"unexplained marker {l[:80]}")
    if marker_gone:
        problems.append(f"markers lost {dict(marker_gone)}")
    for rel, t in texts.items():
        no = len(re.findall(r"^\s*//\s?#region\b", t, flags=re.M))
        nc = len(re.findall(r"^\s*//\s?#endregion\b", t, flags=re.M))
        if no != nc:
            problems.append(f"{rel.split('/')[-2]}: region markers {no}/{nc}")
        depths, inlit = lex(t)
        if depths[-1] != 0:
            problems.append(f"{rel.split('/')[-2]}: brace depth {depths[-1]} at end")
        if len(rel.encode("utf-8")) > 240:
            problems.append(f"path too long {len(rel.encode('utf-8'))}")
    seen = collections.Counter()
    for c in spec["chunks"]:
        first = kids[c["folder"]].split("\n")[0]
        m = DOCRX.match(first)
        if not m:
            problems.append(f"{c['folder']}: first line is no doc")
            continue
        e = m.group(1)
        if c["folder"].split("️")[0] + "️" != c["folder"][: c["folder"].index("️") + 1]:
            problems.append(f"{c['folder']}: folder emoji without U+FE0F")
        if e.replace("️", "") != c["folder"].split("️")[0]:
            problems.append(f"{c['folder']}: docstring emoji {e} != folder emoji")
        seen[e] += 1
        if "[DEBUG]" in kids[c["folder"]]:
            problems.append(f"{c['folder']}: DEBUG log")
    for e, k in seen.items():
        if k > 1:
            problems.append(f"duplicate sibling emoji {e}")
    for rel, t in texts.items():
        r = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--emit", "stdout", "--config", "skip_children=true", os.path.join(base, rel)], capture_output=True, text=True)
        if r.returncode != 0 and "error" in r.stderr.lower():
            problems.append(f"rustfmt parse failed {rel.split('/')[-2]}: {r.stderr[:160]}")
    ot = sorted(m.group(1) for m in TESTRX.finditer(orig))
    nt = sorted(m.group(1) for m in TESTRX.finditer("\n".join(texts.values())))
    if ot != nt:
        problems.append(f"test fn multiset differs {len(ot)} vs {len(nt)}")
    obase = REPO + "/" + fdir
    ofile = REPO + "/" + root_rel
    orig_targets = collections.Counter(os.path.normpath(os.path.join(obase, m)) for m in INC_RX.findall(orig))
    new_targets = collections.Counter()
    mounts = {f"{c['folder']}/🦀️.rs" for c in spec["chunks"]}
    for rel, t in texts.items():
        d = os.path.dirname(os.path.join(REPO, rel))
        for m in INC_RX.findall(t):
            if rel == root_rel and m in mounts:
                continue
            new_targets[os.path.normpath(os.path.join(d, m))] += 1
    if orig_targets != new_targets:
        problems.append("include literal targets differ")
    miss = [p for p in new_targets if not os.path.exists(p)]
    miss_orig = [p for p in orig_targets if not os.path.exists(p)]
    if len(miss) != len(miss_orig):
        problems.append(f"unresolved include targets {len(miss)} (orig {len(miss_orig)})")
    for c in spec["chunks"]:
        if texts[root_rel].count(f'include!("{c["folder"]}/🦀️.rs");') != 1:
            problems.append(f"{c['folder']}: mounted {texts[root_rel].count(chr(105) + 'nclude!(' + chr(34) + c['folder'] + '/🦀️.rs' + chr(34) + ');')} times")
    sizes = {rel.split("/")[-2]: t.count("\n") for rel, t in texts.items() if rel != root_rel}
    print(f"[{tag}] {mode} root={texts[root_rel].count(chr(10))} max-child={max(sizes.values())} children={len(sizes)} orig={orig.count(chr(10))} seq-equal={seq_equal} test-fns {len(ot)}->{len(nt)} include-literals {sum(orig_targets.values())}->{sum(new_targets.values())} (unresolved {len(miss)} orig {len(miss_orig)}) marker-extra={sum(marker_extra.values())}")
    for p in problems:
        print("    PROBLEM", p)
    return not problems


def install(tag):
    spec = json.load(open(f"{T}/specs/{tag}.json", encoding="utf-8"))
    live = f"{REPO}/{spec['file']}"
    problems = []
    if sha(open(live, "rb").read()) != sha(open(spec["original"], "rb").read()):
        problems.append("live file differs from the snapshot (a peer edited it)")
    base = os.path.dirname(spec["file"])
    plan = []
    for c in spec["chunks"]:
        rel = f"{base}/{c['folder']}/🦀️.rs"
        if os.path.exists(f"{REPO}/{rel}"):
            problems.append(f"{rel}: already exists")
        if not os.path.isfile(f"{spec['stage']}/{rel}"):
            problems.append(f"{rel}: not staged")
        plan.append(rel)
    if not os.path.isfile(f"{spec['stage']}/{spec['file']}"):
        problems.append("root not staged")
    print(tag, "children", len(plan), "problems", len(problems))
    for p in problems:
        print("  PROBLEM", p)
    if problems:
        return False
    for rel in plan + [spec["file"]]:
        dst = f"{REPO}/{rel}"
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        tmp = dst + ".c4pp-tmp"
        with open(tmp, "wb") as f:
            f.write(open(f"{spec['stage']}/{rel}", "rb").read())
        os.replace(tmp, dst)
    inst = json.load(open(f"{T}/installed.json")) if os.path.exists(f"{T}/installed.json") else {}
    inst[tag] = dict(created=plan, rewritten=[spec["file"]])
    json.dump(inst, open(f"{T}/installed.json", "w", encoding="utf-8"), indent=1, ensure_ascii=False)
    print("installed", tag, len(plan) + 1, "files")
    return True


if __name__ == "__main__":
    cmd = sys.argv[1]
    if cmd == "stage":
        for t in sys.argv[2:]:
            stage(t)
    elif cmd == "check":
        mode = sys.argv[2]
        ok = True
        for t in sys.argv[3:]:
            ok = check(mode, t) and ok
        print("CHECK", "OK" if ok else "FAILED", mode)
        sys.exit(0 if ok else 1)
    elif cmd == "install":
        ok = True
        for t in sys.argv[2:]:
            ok = install(t) and ok
        sys.exit(0 if ok else 1)
