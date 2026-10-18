"""✅ C5B structural verification of module splits (modsplit.py) against the stage or the live tree.
usage: vmod.py stage|live <tag>...
checks: every child mounted exactly once in the root and present; docstring emoji first, equal to the folder emoji, unique among the children, different from the root emoji, not clashing with an existing sibling folder;
region balance per file (a declared orphan endregion of the original is allowed); no [DEBUG]; path length (repo-relative UTF-8 bytes <= 240, absolute chars <= 256); brace depth 0; rustfmt parses every file;
include literal targets (include!/include_str!/include_bytes!) equal to the original ones as absolute paths and all existing; #[test] fn multiset equal; child module names do not clash with other names of the split scope."""
import collections, importlib, os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/rf8-tools')
sys.path.insert(0, HERE)
from rslex import lex
from rsitems import items, name_kind
REPO = "/Users/ueli/Documents/semio"
SCR = os.environ.get("C5B_SCR", "/Users/ueli/.cache/c6fin")
mode = sys.argv[1]
TESTRX = re.compile(r"#\[\s*(?:[\w:]*::)?(?:test|async_test|tokio::test)\b[^\]]*\]\s*(?:#\[[^\]]*\]\s*|//[^\n]*\n\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+(\w+)")
RG = re.compile(r"^\s*//\s?#region\b", re.M)
ER = re.compile(r"^\s*//\s?#endregion\b", re.M)
INC = re.compile(r'include(?:_str|_bytes)?!\(\s*"([^"]+)"')
ok_all = True
for tag in sys.argv[2:]:
    sp = importlib.import_module("specs." + tag).SPEC
    rel_root = sp["file"]
    live_dir = os.path.join(REPO, os.path.dirname(rel_root))
    base = os.path.join(SCR, "stage", tag) if mode == "stage" else live_dir
    orig = open(f"{HERE}/orig/{sp['orig']}", encoding="utf-8").read()
    if sp.get("pre"):
        orig = sp["pre"](orig)
    root = open(os.path.join(base, "🦀️.rs"), encoding="utf-8").read()
    inns = sp.get("inners") or ([sp["inner"]] if sp.get("inner") else [])
    outer_children = list(sp["children"]) + [dict(folder=i["folder"], mod=i["mod"]) for i in inns]
    levels = [("", root, outer_children, os.path.dirname(rel_root))]
    folders = [c["folder"] for c in outer_children]
    for inn in inns:
        shell = open(os.path.join(base, inn["folder"], "🦀️.rs"), encoding="utf-8").read()
        levels.append((inn["folder"] + "/", shell, inn["children"], os.path.join(os.path.dirname(rel_root), inn["folder"])))
        folders += [inn["folder"] + "/" + c["folder"] for c in inn["children"]]
    kids = {f: open(os.path.join(base, f, "🦀️.rs"), encoding="utf-8").read() for f in folders}
    problems = []
    for prefix, ptext, chs, pdir in levels:
        for c in chs:
            f = c["folder"]
            n = len(re.findall(r'^#\[path = "%s/🦀️\.rs"\]\s*$' % re.escape(f), ptext, re.M))
            if n != 1:
                problems.append(f"{prefix}{f}: mounted {n} times")
            if len(re.findall(r"^(?:pub(?:\([^)]*\))? )?mod %s;" % c["mod"], ptext, re.M)) != 1:
                problems.append(f"{prefix}{f}: mod {c['mod']} declared != 1 time")
        mm = re.match(r"^//! (\S+)", ptext)
        root_emoji = mm.group(1).replace("\ufe0f", "") if mm else None
        seen = collections.Counter()
        for c in chs:
            f = c["folder"]
            t_ = kids[prefix + f]
            first = t_.split("\n")[0]
            m = re.match(r"^//! (\S+) ", first)
            emo = f[: f.index("\ufe0f") + 1]
            if not m or m.group(1) != emo:
                problems.append(f"{prefix}{f}: docstring emoji {m.group(1) if m else None} != folder emoji {emo}")
            seen[emo] += 1
            if m and root_emoji and m.group(1).replace("\ufe0f", "") == root_emoji:
                problems.append(f"{prefix}{f}: docstring emoji equals parent emoji")
        for e, k in seen.items():
            if k > 1:
                problems.append(f"duplicate sibling emoji {e}")
        adir = os.path.join(REPO, pdir)
        mine = {c["folder"] for c in chs}
        for e in os.listdir(adir) if os.path.isdir(adir) else []:
            if e in mine or not os.path.isdir(os.path.join(adir, e)) or "\ufe0f" not in e:
                continue
            if e[: e.index("\ufe0f") + 1] in seen:
                problems.append(f"sibling emoji clash with existing folder {e}")
    o_orph = len(ER.findall(orig)) - len(RG.findall(orig))
    tot_o = tot_c = 0
    for r, t in [("🦀️.rs", root)] + [(f"{f}/🦀️.rs", kids[f]) for f in folders]:
        p = os.path.join(os.path.dirname(rel_root), r)
        if len(p.encode()) > 240 or len(os.path.join(REPO, p)) > 256:
            problems.append(f"path too long {len(p.encode())} {r}")
        no, nc = len(RG.findall(t)), len(ER.findall(t))
        tot_o += no
        tot_c += nc
        if "[DEBUG]" in t:
            problems.append(f"[DEBUG] in {r}")
        d, _ = lex(t)
        if d[-1] != 0:
            problems.append(f"{r}: brace depth {d[-1]}")
        pr = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--emit", "stdout", "--config", "skip_children=true", os.path.join(base, r)], capture_output=True, text=True)
        if pr.returncode != 0 and re.search(r"^error(?!\[internal\])", pr.stderr, flags=re.M):
            problems.append(f"rustfmt parse failed {r}: {pr.stderr[:140]}")
    if tot_c - tot_o != -o_orph * -1 * 1 and (tot_c - tot_o) != (len(ER.findall(orig)) - len(RG.findall(orig))):
        problems.append(f"region balance {tot_o}/{tot_c} vs original {len(RG.findall(orig))}/{len(ER.findall(orig))}")
    obase = os.path.join(REPO, os.path.dirname(rel_root))
    oinc = collections.Counter(os.path.normpath(os.path.join(obase, m)) for m in INC.findall(orig))
    ninc = collections.Counter()
    for r, t in [("🦀️.rs", root)] + [(f"{f}/🦀️.rs", kids[f]) for f in folders]:
        d = os.path.dirname(os.path.join(live_dir, r))
        for m in INC.findall(t):
            ninc[os.path.normpath(os.path.join(d, m))] += 1
    child_files = {os.path.normpath(os.path.join(live_dir, f, "🦀️.rs")) for f in folders}
    if sp.get("source_concat"):
        got = {p for p in ninc if p in child_files}
        if got != child_files or any(ninc[p] != 1 for p in got):
            problems.append(f"source concat covers {len(got)} of {len(child_files)} children")
        for p in got:
            del ninc[p]
    if oinc != ninc:
        problems.append(f"include targets differ: orig-only {list((oinc - ninc))[:2]} new-only {list((ninc - oinc))[:2]}")
    for p in ninc:
        if not os.path.exists(p):
            problems.append(f"include target missing {p[-90:]}")
    ot = sorted(m.group(1) for m in TESTRX.finditer(orig))
    nt = sorted(m.group(1) for m in TESTRX.finditer("\n".join([root] + list(kids.values()))))
    if ot != nt:
        problems.append("test fn multiset differs")
    names = collections.defaultdict(list)
    for t in [root] + list(kids.values()):
        for it in items(t):
            v, k, n = name_kind(it["toks"])
            if n and k != "impl":
                names[n].append(k)
        for m in re.finditer(r"\buse\b[^;]*;", t, re.S):
            for w in re.findall(r"(?:\bas\s+|::)([A-Za-z_]\w*)\s*(?=[,;}]|$)", m.group(0)):
                names[w].append("use")
    for c in [c for _, _, chs, _ in levels for c in chs]:
        kinds = names.get(c["mod"], [])
        extra = [k for k in kinds if k != "mod"] + [k for k in kinds if k == "mod"][1:]
        if extra:
            problems.append(f"child module {c['mod']} clashes with {kinds}")
    sizes = {f: t.count("\n") for f, t in kids.items()}
    print(f"[{tag}] {mode} root={root.count(chr(10))} children={len(kids)} max-child={max(sizes.values())} tests {len(ot)}->{len(nt)} includes {sum(oinc.values())}->{sum(ninc.values())} {'OK' if not problems else 'PROBLEMS'}")
    for p in problems:
        print("    PROBLEM", p)
        ok_all = False
print("VMOD", "OK" if ok_all else "FAILED", mode)
sys.exit(0 if ok_all else 1)
