"""✅ C5C structural verification of module splits (modsplit.py) against the stage or the live tree.
usage: vmod.py stage|live <tag>...
checks: every child mounted exactly once in the root and present; docstring emoji first, equal to the folder emoji, unique among the children, different from the root emoji, not clashing with an existing sibling folder;
region balance per file (a declared orphan endregion of the original is allowed); no [DEBUG]; path length (repo-relative UTF-8 bytes <= 240, absolute chars <= 256); brace depth 0; rustfmt parses every file;
include literal targets (include!/include_str!/include_bytes!) equal to the original ones as absolute paths and all existing; #[test] fn multiset equal; child module names do not clash with other names of the split scope."""
import collections, importlib, os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/../rf8-tools")
sys.path.insert(0, HERE)
from rslex import lex
from rsitems import items, name_kind
REPO = "/Users/ueli/Documents/semio"
SCR = os.environ.get("C5C_SCR", "/Users/ueli/.cache/c5c")
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
    folders = [c["folder"] for c in sp["children"]]
    inn = sp.get("inner")
    allf = list(folders)
    kids = {f: open(os.path.join(base, f, "🦀️.rs"), encoding="utf-8").read() for f in folders}
    problems = []
    for c in sp["children"]:
        f = c["folder"]
        n = len(re.findall(r'^#\[path = "%s/🦀️\.rs"\]\s*$' % re.escape(f), root, re.M))
        if n != 1:
            problems.append(f"{f}: mounted {n} times")
        if len(re.findall(r"^(?:pub(?:\([^)]*\))? )?mod %s;" % c["mod"], root, re.M)) != 1:
            problems.append(f"{f}: mod {c['mod']} declared != 1 time")
    mm = re.match(r"^//! (\S+)", root)
    root_emoji = mm.group(1).replace("️", "") if mm else None
    seen = collections.Counter()
    for f, t in kids.items():
        first = t.split("\n")[0]
        m = re.match(r"^//! (\S+) ", first)
        emo = f[: f.index("️") + 1]
        if not m or m.group(1) != emo:
            problems.append(f"{f}: docstring emoji {m.group(1) if m else None} != folder emoji {emo}")
        seen[emo] += 1
        if m and root_emoji and m.group(1).replace("️", "") == root_emoji:
            problems.append(f"{f}: docstring emoji equals root emoji")
    for e, k in seen.items():
        if k > 1:
            problems.append(f"duplicate sibling emoji {e}")
    for e in os.listdir(live_dir):
        if e in kids or not os.path.isdir(os.path.join(live_dir, e)) or "️" not in e:
            continue
        if e[: e.index("️") + 1] in seen:
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
    for c in sp["children"]:
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
